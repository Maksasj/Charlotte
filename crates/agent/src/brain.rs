//! The brain: the one owner of the vault in a process. Every conversation
//! (CLI, trainer teachers, evaluation) holds a [`SharedBrain`] and changes
//! knowledge only while holding its lock, so there is a single in-memory copy
//! of the vault and a single writer of its files and log. Model calls never
//! happen while the lock is held.
//!
//! The brain also keeps one evaluated model per state of the vault: answering,
//! curiosity and the agent's other checks share it, and it is rebuilt only
//! when the vault changes or time crosses a fact's start or end.

use crate::curiosity::Curiosity;
use crate::events::EventBus;
use sla_core::{Fact, GroundAtom, KnowledgeBase, Timestamp};
use sla_learner::Learner;
use sla_reasoner::{evaluate, facts_for, quarantine, EvalError, FactInput, Model, RuleInput};
use std::collections::{BTreeMap, HashSet};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::SystemTime;

pub type SharedBrain = Arc<Mutex<Brain>>;

/// Lock a shared brain, recovering from a panic in another holder.
pub fn lock(b: &SharedBrain) -> MutexGuard<'_, Brain> {
    b.lock().unwrap_or_else(|e| e.into_inner())
}

/// Most past moments evaluated for one history question.
pub const PAST_MOMENTS: usize = 64;

struct NowCache {
    generation: u64,
    /// The model holds for every time in `[lo, hi)`.
    lo: i64,
    hi: i64,
    model: Arc<Model>,
}

pub struct Brain {
    pub kb: KnowledgeBase,
    pub learner: Learner,
    pub curiosity: Curiosity,
    pub events: Arc<EventBus>,
    /// Rules set aside because they break evaluation: id -> reason.
    pub quarantined: BTreeMap<String, String>,
    /// Full evaluations so far (tests check the per-message count).
    pub evaluations: u64,
    /// Milliseconds spent in those evaluations, and in learner updates.
    pub eval_ms: f64,
    pub learn_ms: f64,
    now_cache: Option<NowCache>,
    past_cache: Option<(u64, i64, String, Arc<Vec<Arc<Model>>>)>,
    names: (u64, Arc<HashSet<String>>),
    vault_stamp: u64,
}

/// Fingerprint of all vault files (mtimes + sizes) to notice hand edits.
fn vault_fingerprint(root: &Path) -> u64 {
    let mut h: u64 = 1469598103934665603;
    for dir in ["facts", "rules", "core", "lexicon", "procedures", "tasks"] {
        let Ok(rd) = std::fs::read_dir(root.join(dir)) else { continue };
        let mut entries: Vec<_> = rd.flatten().collect();
        entries.sort_by_key(|e| e.path());
        for e in entries {
            if let Ok(m) = e.metadata() {
                let t = m.modified().ok().and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok()).map_or(0, |d| d.as_nanos() as u64);
                for x in [t, m.len(), e.path().to_string_lossy().len() as u64] {
                    h = (h ^ x).wrapping_mul(1099511628211);
                }
            }
        }
    }
    h
}

impl Brain {
    /// Open (or create) a vault.
    pub fn open(root: impl Into<PathBuf>) -> io::Result<Self> {
        let root = root.into();
        let kb = KnowledgeBase::open(&root)?;
        let mut b = Brain {
            kb,
            learner: Learner::default(),
            curiosity: Curiosity::default(),
            events: Arc::new(EventBus::default()),
            quarantined: BTreeMap::new(),
            evaluations: 0,
            eval_ms: 0.0,
            learn_ms: 0.0,
            now_cache: None,
            past_cache: None,
            names: (u64::MAX, Arc::new(HashSet::new())),
            vault_stamp: 0,
        };
        b.vault_stamp = vault_fingerprint(&b.kb.vault.root);
        // Questions asked in earlier sessions are not asked again.
        if let Ok(log) = b.kb.vault.read_log() {
            for (_, action, subject, _) in log {
                if action == "ask" || action == "proactive" {
                    b.curiosity.asked.insert(subject);
                }
            }
        }
        Ok(b)
    }

    pub fn shared(root: impl Into<PathBuf>) -> io::Result<SharedBrain> {
        Ok(Arc::new(Mutex::new(Brain::open(root)?)))
    }

    /// Reload if the vault files changed on disk since our last save (hand edits).
    pub fn reload_if_edited(&mut self) -> io::Result<bool> {
        let stamp = vault_fingerprint(&self.kb.vault.root);
        if stamp != self.vault_stamp {
            self.kb.reload()?;
            self.vault_stamp = vault_fingerprint(&self.kb.vault.root);
            return Ok(true);
        }
        Ok(false)
    }

    pub fn save(&mut self) -> io::Result<()> {
        self.kb.save()?;
        // Our own writes changed the files: remember the new fingerprint.
        self.vault_stamp = vault_fingerprint(&self.kb.vault.root);
        Ok(())
    }

    /// The rules the reasoner runs, with any that break evaluation set aside.
    fn runnable_rules(&mut self, include_hypotheses: bool, now: Timestamp) -> Vec<RuleInput> {
        let rules: Vec<RuleInput> = self.kb.rules(include_hypotheses).iter().map(RuleInput::from).collect();
        if sla_reasoner::check(&rules).is_ok() {
            return rules;
        }
        match quarantine(&rules) {
            Ok((kept, bad)) => {
                for (r, e) in bad {
                    if !self.quarantined.contains_key(&r.id) {
                        self.kb.log_change(now, "quarantine", &format!("{}: {}", r.id, r.rule), &e.to_string());
                        self.quarantined.insert(r.id.clone(), e.to_string());
                    }
                }
                kept
            }
            Err(e) => {
                // Core itself is broken (a bad hand edit): nothing else can be trusted.
                if !self.quarantined.contains_key("core") {
                    self.kb.log_change(now, "error", "core rules", &e.to_string());
                    self.quarantined.insert("core".into(), e.to_string());
                }
                Vec::new()
            }
        }
    }

    fn eval(&mut self, facts: &[FactInput], rules: &[RuleInput]) -> Result<Model, EvalError> {
        self.evaluations += 1;
        let t = std::time::Instant::now();
        let out = evaluate(facts, rules);
        self.eval_ms += t.elapsed().as_secs_f64() * 1000.0;
        out
    }

    /// The model at time `at`, shared until the vault changes or `at` crosses
    /// a fact boundary.
    pub fn model_at(&mut self, at: Timestamp) -> Arc<Model> {
        let generation = self.kb.vault.generation;
        if let Some(c) = &self.now_cache {
            if c.generation == generation && c.lo <= at.0 && at.0 < c.hi {
                return Arc::clone(&c.model);
            }
        }
        let facts = self.kb.reasoning_facts();
        let (mut lo, mut hi) = (i64::MIN, i64::MAX);
        for f in &facts {
            for b in [f.from, f.until].into_iter().flatten() {
                if b.0 <= at.0 {
                    lo = lo.max(b.0);
                } else {
                    hi = hi.min(b.0);
                }
            }
        }
        let rules = self.runnable_rules(false, at);
        let inputs = facts_for(&facts, Some(at));
        let model = Arc::new(self.eval(&inputs, &rules).unwrap_or_default());
        // Quarantine logging may have bumped nothing; key on the generation we evaluated.
        self.now_cache = Some(NowCache { generation: self.kb.vault.generation, lo, hi, model: Arc::clone(&model) });
        model
    }

    /// The model at `at` with extra facts (a conversation's situation) on top.
    /// Not cached: the extra facts belong to one conversation.
    pub fn model_with(&mut self, at: Timestamp, extra: &[GroundAtom]) -> Arc<Model> {
        let rules = self.runnable_rules(false, at);
        let mut inputs = facts_for(&self.kb.reasoning_facts(), Some(at));
        inputs.extend(extra.iter().cloned().map(FactInput::new));
        Arc::new(self.eval(&inputs, &rules).unwrap_or_default())
    }

    /// Models of the past for questions about `pred`, one per moment that
    /// matters (see [`sla_reasoner::past_moments`]), newest first. Only the
    /// facts and rules `pred` depends on take part, so the past of one
    /// predicate costs little. Facts are only joined within one moment, so a
    /// history answer never mixes facts that were not true together.
    pub fn past_models(&mut self, pred: &str, now: Timestamp) -> Arc<Vec<Arc<Model>>> {
        let generation = self.kb.vault.generation;
        let rules = self.runnable_rules(false, now);
        let deps = sla_reasoner::depends_on(&rules, &[pred]);
        // Predicates with the same dependencies share one past.
        let mut key: Vec<&str> = deps.iter().map(String::as_str).collect();
        key.sort_unstable();
        let key = key.join(",");
        if let Some((g, t, k, m)) = &self.past_cache {
            if *g == generation && *t == now.0 && *k == key {
                return Arc::clone(m);
            }
        }
        let rules: Vec<RuleInput> = rules.into_iter().filter(|r| deps.contains(r.rule.head.pred.as_str())).collect();
        let facts: Vec<Fact> = self.kb.reasoning_facts().into_iter().filter(|f| deps.contains(f.atom.pred.as_str())).collect();
        let moments = sla_reasoner::past_moments(&facts, &rules, now, PAST_MOMENTS);
        let mut models = Vec::new();
        for t in moments {
            let inputs = facts_for(&facts, Some(t));
            if let Ok(m) = self.eval(&inputs, &rules) {
                models.push(Arc::new(m));
            }
        }
        let models = Arc::new(models);
        self.past_cache = Some((self.kb.vault.generation, now.0, key, Arc::clone(&models)));
        models
    }

    /// Was `g` true at some moment in the past?
    pub fn was_true(&mut self, g: &GroundAtom, now: Timestamp) -> bool {
        self.past_models(&g.pred, now).iter().any(|m| m.contains(g))
    }

    /// Atoms matching `pattern` at some moment in the past (deduplicated, sorted).
    pub fn past_matches(&mut self, pattern: &sla_core::Atom, now: Timestamp) -> Vec<GroundAtom> {
        let mut out: Vec<GroundAtom> = self.past_models(&pattern.pred, now).iter().flat_map(|m| m.query_atom(pattern)).collect();
        out.sort();
        out.dedup();
        out
    }

    /// Every past moment, all predicates (for inspection; costs a full
    /// evaluation per moment).
    pub fn all_past_models(&mut self, now: Timestamp) -> Vec<Arc<Model>> {
        let facts: Vec<Fact> = self.kb.reasoning_facts();
        let rules = self.runnable_rules(false, now);
        let moments = sla_reasoner::past_moments(&facts, &rules, now, PAST_MOMENTS);
        let mut out = Vec::new();
        for t in moments {
            let inputs = facts_for(&facts, Some(t));
            if let Ok(m) = self.eval(&inputs, &rules) {
                out.push(Arc::new(m));
            }
        }
        out
    }

    /// The learner's view of the world now: entity features from the model
    /// built with evidence rules only (no rule the learner guessed). When no
    /// guessed rule is confirmed that is the agent's own model, so it is shared.
    pub fn learner_features(&mut self, now: Timestamp) -> std::collections::BTreeMap<sla_core::Value, std::collections::BTreeSet<sla_learner::Feature>> {
        if !Learner::has_confirmed_guesses(&self.kb) {
            let m = self.model_at(now);
            return self.learner.features_from(&m);
        }
        let rules = Learner::evidence_rules(&self.kb);
        let inputs = facts_for(&self.kb.reasoning_facts(), Some(now));
        match self.eval(&inputs, &rules) {
            Ok(m) => self.learner.features_from(&m),
            Err(_) => Default::default(),
        }
    }

    /// Entities written as names (people, places), recomputed only when the vault changes.
    pub fn names(&mut self) -> Arc<HashSet<String>> {
        let generation = self.kb.vault.generation;
        if self.names.0 == generation {
            return Arc::clone(&self.names.1);
        }
        let facts = self.kb.all_facts();
        let mut names: HashSet<String> = HashSet::new();
        for f in &facts {
            let a = &f.atom;
            let first = a.args.first().and_then(|v| v.as_sym()).map(String::from);
            let second = a.args.get(1).and_then(|v| v.as_sym()).map(String::from);
            match a.base_pred() {
                "likes" | "feels" | "wants" | "person" | "place" | "proper_name" => names.extend(first),
                "knows" => names.extend(first.into_iter().chain(second)),
                "is_a" if second.as_deref() == Some("person") => names.extend(first),
                "located_in" => names.extend(second),
                _ => {}
            }
        }
        for f in &facts {
            // Things in general knowledge are common nouns, not names ("Rain is wet" starts with a capital).
            if f.source.starts_with("teacher") {
                for v in &f.atom.args {
                    if let Some(x) = v.as_sym() {
                        names.remove(x);
                    }
                }
            }
            // Categories are not names ("Fruit is a kind of food" starts with a capital).
            if f.atom.base_pred() == "is_a" {
                if let Some(c) = f.atom.args.get(1).and_then(|v| v.as_sym()) {
                    names.remove(c);
                }
            }
        }
        self.names = (generation, Arc::new(names));
        Arc::clone(&self.names.1)
    }

    /// Install a scenario's procedures and task schemas (`<dir>/procedures/*.md`,
    /// `<dir>/tasks/*.md`) into the vault. Returns how many files.
    pub fn install(&mut self, dir: &Path) -> io::Result<usize> {
        let mut n = 0;
        for (sub, kind) in [("procedures", sla_core::ProcedureKind::Procedure), ("tasks", sla_core::ProcedureKind::Task)] {
            let Ok(rd) = std::fs::read_dir(dir.join(sub)) else { continue };
            for e in rd.flatten() {
                let p = e.path();
                if p.extension().and_then(|x| x.to_str()) != Some("md") {
                    continue;
                }
                let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string();
                let pf = sla_core::ProcedureFile::parse(&std::fs::read_to_string(&p)?, &stem);
                self.kb.vault.put_procedure(kind, pf);
                n += 1;
            }
        }
        self.save()?;
        Ok(n)
    }

    /// Is there a stored fact with exactly this atom (current or not)?
    pub fn has_stored(&self, atom: &GroundAtom) -> bool {
        let stem = match atom.args.first() {
            Some(sla_core::Value::Sym(s)) => sla_core::vault::file_stem(s),
            Some(sla_core::Value::Int(_)) | Some(sla_core::Value::Dec(_)) => "_numbers".into(),
            None => "_global".into(),
        };
        self.kb.vault.facts.get(&stem).map_or(false, |ff| ff.doc.facts().any(|f| f.atom == *atom))
    }
}
