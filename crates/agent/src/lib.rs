//! The agent: message -> parser -> knowledge base update or query -> reply -> log.
//!
//! [`Agent::respond`] handles one user message. The CLI in `main.rs` wraps it
//! in a read-eval-print loop; tests and the evaluation harness call it directly.

use sla_core::kb::matches;
use sla_core::{Atom, AssertOutcome, Fact, GroundAtom, KnowledgeBase, Literal, Rule, RuleFile, RuleMeta, RuleStatus, Term, Timestamp, Value};
use sla_learner::corrections::{self, Change, Demonstration};
use sla_learner::{LearnReport, Learner};
use sla_nlg::{Act, Realizer, Reply, Truth};
use sla_parser::{Context, Intent, Parser, PatternParser};
use sla_reasoner::{evaluate_kb, Model};
use std::collections::HashSet;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub mod router;

/// Something the agent asked and is waiting for an answer to.
#[derive(Debug, Clone, PartialEq)]
pub enum Pending {
    /// "Does Bob like coffee?" -> yes stores it, no stores the negation.
    Fact(GroundAtom),
    /// "What does Bob like?" -> the next statement is the answer.
    Wh(Atom),
    /// "Is it true that ...?" about a hypothesis rule (by rule file id).
    Rule(String),
    /// "Which one is true?" about two contradicting facts.
    Contradiction(GroundAtom, GroundAtom),
}

pub struct Agent {
    pub kb: KnowledgeBase,
    parser: Box<dyn Parser>,
    pub ctx: Context,
    pub realizer: Realizer,
    /// The atom the last answer was about (for "why?" and "that's wrong").
    pub last_answer: Option<GroundAtom>,
    pub pending: Option<Pending>,
    /// Fixed clock for tests and simulations; `None` uses the system clock.
    pub clock: Option<Timestamp>,
    /// Source label for facts from this conversation partner.
    pub source: String,
    pub learner: Learner,
    /// What the learner did after the last message.
    pub last_learn: LearnReport,
    vault_stamp: u64,
}

/// Fingerprint of all vault files (mtimes + sizes) to notice hand edits.
fn vault_fingerprint(root: &Path) -> u64 {
    let mut h: u64 = 1469598103934665603;
    for dir in ["facts", "rules", "core"] {
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

impl Agent {
    /// Open (or create) a vault and start an agent talking to `speaker`.
    pub fn open(root: impl Into<PathBuf>, speaker: &str) -> io::Result<Self> {
        let root = root.into();
        let kb = KnowledgeBase::open(&root)?;
        let mut a = Agent {
            kb,
            parser: Box::new(PatternParser),
            ctx: Context { speaker: speaker.to_string() },
            realizer: Realizer::new(speaker),
            last_answer: None,
            pending: None,
            clock: None,
            source: "user".into(),
            learner: Learner::default(),
            last_learn: LearnReport::default(),
            vault_stamp: 0,
        };
        a.vault_stamp = vault_fingerprint(&a.kb.vault.root);
        a.refresh_names();
        Ok(a)
    }

    /// Swap the parser (for example an LLM translator).
    pub fn set_parser(&mut self, p: Box<dyn Parser>) {
        self.parser = p;
    }

    pub fn now(&self) -> Timestamp {
        self.clock.unwrap_or_else(Timestamp::now)
    }

    /// Reload if the vault files changed on disk since our last save.
    pub fn reload_if_edited(&mut self) -> io::Result<bool> {
        let stamp = vault_fingerprint(&self.kb.vault.root);
        if stamp != self.vault_stamp {
            self.kb.reload()?;
            self.vault_stamp = vault_fingerprint(&self.kb.vault.root);
            self.refresh_names();
            return Ok(true);
        }
        Ok(false)
    }

    pub fn save(&mut self) -> io::Result<()> {
        self.kb.save()?;
        self.vault_stamp = vault_fingerprint(&self.kb.vault.root);
        Ok(())
    }

    /// The model at the current time (`history`: over all time).
    pub fn model(&self, history: bool) -> Model {
        let at = if history { None } else { Some(self.now()) };
        match evaluate_kb(&self.kb, at, false) {
            Ok(m) => m,
            Err(e) => {
                // A bad rule must not take the agent down: evaluate without learned rules.
                self.kb.log_change(self.now(), "error", "evaluate", &e.to_string());
                let facts = sla_reasoner::facts_for(&self.kb.all_facts(), at);
                sla_reasoner::evaluate(&facts, &[]).unwrap_or_default()
            }
        }
    }

    /// Capitalized words inside a sentence are proper names ("in Berlin",
    /// "Is Rex a dog?"). They are stored as `proper_name(x)` facts so replies
    /// can capitalize them after a restart too.
    fn learn_names(&mut self, text: &str) {
        let now = self.now();
        for name in proper_names(text) {
            let atom = GroundAtom::new("proper_name", vec![Value::sym(name.as_str())]);
            if !self.kb.all_facts().iter().any(|f| f.atom == atom) {
                self.kb.vault.add_fact(Fact::new(atom, "parser").with_from(now));
            }
        }
    }

    /// Entities written as names: people and places.
    fn refresh_names(&mut self) {
        let mut names: HashSet<String> = HashSet::new();
        names.insert(self.ctx.speaker.clone());
        for f in self.kb.all_facts() {
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
        self.realizer.names = names;
    }

    /// Handle one user message and return the reply.
    pub fn respond(&mut self, text: &str) -> Reply {
        let _ = self.reload_if_edited();
        self.learn_names(text);
        self.refresh_names();
        let intent = self.parser.parse(text, &self.ctx);
        let reply = self.handle(intent);
        self.refresh_names();
        let _ = self.save();
        self.kb.log_change(self.now(), "turn", text, &reply.text);
        reply
    }

    fn say(&self, act: Act) -> Reply {
        self.realizer.realize(&act)
    }

    fn handle(&mut self, intent: Intent) -> Reply {
        let pending = self.pending.take();
        match intent {
            Intent::Tell { facts, rules, past } => {
                // An answer to "What does Bob like?" arrives as a statement.
                self.tell(facts, rules, past)
            }
            Intent::Ask { query, history } => self.ask(query, history),
            Intent::Recall(e) => {
                let m = self.model(false);
                let mut facts: Vec<GroundAtom> = m
                    .all_facts()
                    .into_iter()
                    .filter(|a| a.args.first().and_then(|v| v.as_sym()) == Some(e.as_str()))
                    .filter(|a| !matches!(a.base_pred(), "thing" | "same" | "proper_name"))
                    .collect();
                facts.truncate(15);
                self.record_uses(&m, &facts);
                self.say(Act::Recall { entity: e, facts })
            }
            Intent::Why(target) => {
                let atom = match target.and_then(|a| a.to_ground()) {
                    Some(a) => Some(a),
                    None => self.last_answer.clone(),
                };
                match atom {
                    None => self.say(Act::Note("Why what? Ask me something first.".into())),
                    Some(a) => {
                        let m = self.model(false);
                        match m.proof(&a) {
                            Some(p) => {
                                self.record_uses(&m, &p.leaves());
                                self.last_answer = Some(a);
                                self.say(Act::Explain(p))
                            }
                            None => self.say(Act::ExplainUnknown(a)),
                        }
                    }
                }
            }
            Intent::Correct(atoms) => self.correct(atoms),
            Intent::Yes | Intent::No => self.answer_pending(pending, matches!(intent, Intent::Yes)),
            Intent::Greet => self.say(Act::Greet),
            Intent::Bye => self.say(Act::Bye),
            Intent::Unknown => self.say(Act::NotUnderstood),
        }
    }

    fn tell(&mut self, facts: Vec<Atom>, rules: Vec<Rule>, past: bool) -> Reply {
        let now = self.now();
        let mut added = Vec::new();
        let mut known = Vec::new();
        let mut closed = Vec::new();
        for a in facts {
            let Some(g) = a.to_ground() else { continue };
            let mut fact = Fact::new(g.clone(), &self.source);
            if past {
                // "I used to like tea": true before, not now.
                fact.until = Some(now);
                fact.from = None;
                let reason = format!("{} said it used to be true", self.source);
                self.kb.vault.add_fact(fact);
                self.kb.log_change(now, "add", &g.to_string(), &reason);
                added.push(g);
                continue;
            }
            match self.kb.assert_fact(fact, now, &format!("{} said so", self.source)) {
                AssertOutcome::Added { closed: c } => {
                    added.push(g);
                    closed.extend(c.into_iter().map(|f| f.atom));
                }
                AssertOutcome::Confirmed { .. } => known.push(g),
            }
        }
        let mut new_rules = Vec::new();
        for r in rules {
            match self.add_rule(r.clone(), RuleStatus::Confirmed, 0.9, &self.source.clone(), "stated by the user") {
                Ok(true) => new_rules.push(r),
                Ok(false) => {}
                Err(e) => return self.say(Act::Note(format!("I can't use that rule: {e}."))),
            }
        }
        if let Some(a) = added.first().or(known.first()) {
            self.last_answer = Some(a.clone());
        }
        // New facts may support, contradict or suggest learned rules.
        if !added.is_empty() {
            self.last_learn = self.learner.update(&mut self.kb, now);
        }
        self.say(Act::Acknowledge { added, closed, known, rules: new_rules, past })
    }

    /// Store a rule in its own file. Returns false if it already exists.
    /// The rule is checked by evaluating; an unsafe or unstratifiable rule is refused.
    pub fn add_rule(&mut self, rule: Rule, status: RuleStatus, conf: f64, source: &str, note: &str) -> Result<bool, String> {
        if self.kb.find_rule(&rule).is_some() {
            return Ok(false);
        }
        let module = router::module_for(&rule.head.pred);
        let base = format!("{}_{}", rule.head.pred.trim_start_matches('-').replace('-', "not_"), if rule.head.pred.starts_with('-') { "neg" } else { "rule" });
        let mut id = base.clone();
        let mut n = 2;
        while self.kb.vault.rules.contains_key(&sla_core::vault::file_stem(&id)) {
            id = format!("{base}_{n}");
            n += 1;
        }
        let meta = RuleMeta { id: id.clone(), status, conf, evidence: 0, counter: 0, module: module.into(), source: source.into() };
        self.kb.vault.put_rule_file(RuleFile::new(&meta, note, &[rule.clone()]));
        if let Err(e) = evaluate_kb(&self.kb, Some(self.now()), true) {
            self.kb.vault.remove_rule_file(&id);
            return Err(e.to_string());
        }
        self.kb.log_change(self.now(), "add_rule", &format!("{id}: {rule}"), note);
        Ok(true)
    }

    fn ask(&mut self, query: Vec<Literal>, history: bool) -> Reply {
        let now_model = self.model(false);
        // Single ground atom: yes/no.
        if let [Literal::Pos(a)] = query.as_slice() {
            if let Some(g) = a.to_ground() {
                self.last_answer = Some(g.clone());
                let truth = if now_model.contains(&g) {
                    Truth::Yes
                } else if now_model.contains(&g.complement()) && !history {
                    Truth::No
                } else if self.model(true).contains(&g) {
                    Truth::WasTrue
                } else if now_model.contains(&g.complement()) {
                    Truth::No
                } else {
                    Truth::Unknown
                };
                if truth == Truth::Yes {
                    if let Some(p) = now_model.proof(&g) {
                        self.record_uses(&now_model, &p.leaves());
                    }
                }
                return self.say(Act::YesNo { atom: g, truth });
            }
            // One atom with W: wh-question.
            let mut found = now_model.query_atom(a);
            let mut past = false;
            if found.is_empty() || history {
                let hist = self.model(true).query_atom(a);
                if history || !hist.is_empty() {
                    past = found.is_empty() && !hist.is_empty();
                    if history {
                        // Past answers: those no longer true now.
                        let old: Vec<GroundAtom> = hist.iter().filter(|h| !found.contains(h)).cloned().collect();
                        if !old.is_empty() {
                            found = old;
                            past = true;
                        }
                    } else {
                        found = hist;
                    }
                }
            }
            found.sort();
            found.truncate(12);
            self.last_answer = found.first().cloned();
            self.record_uses(&now_model, &found);
            return self.say(Act::Wh { pattern: a.clone(), found, past });
        }
        // Conjunctive query: answer with the matching first atoms.
        match now_model.query(&query) {
            Ok(answers) => {
                let found: Vec<GroundAtom> = answers.iter().filter_map(|a| a.premises.first().cloned()).collect();
                let pattern = match query.first() {
                    Some(Literal::Pos(a)) => a.clone(),
                    _ => Atom::new("unknown", vec![]),
                };
                self.last_answer = found.first().cloned();
                self.say(Act::Wh { pattern, found, past: false })
            }
            Err(_) => self.say(Act::NotUnderstood),
        }
    }

    fn record_uses(&mut self, m: &Model, atoms: &[GroundAtom]) {
        let mut leaves = Vec::new();
        for a in atoms {
            if let Some(p) = m.proof(a) {
                leaves.extend(p.leaves());
            }
        }
        let now = self.now();
        self.kb.record_use(&leaves, now);
    }

    /// "No, X" / "That's wrong".
    fn correct(&mut self, atoms: Vec<Atom>) -> Reply {
        let now = self.now();
        let mut retracted = Vec::new();
        let mut added = Vec::new();
        let mut rules = Vec::new();
        let last = self.last_answer.clone();
        if atoms.is_empty() {
            // "That's wrong": the last answer is false.
            let Some(a) = last else {
                return self.say(Act::Note("What is wrong? I haven't said anything yet.".into()));
            };
            if let Some(r) = self.make_false(&a, now, &mut retracted) {
                rules.push(r);
            }
            return self.say(Act::Corrected { added, retracted, rules });
        }
        let model = self.model(false);
        for a in atoms {
            let Some(g) = a.to_ground() else { continue };
            let mut wrong = Vec::new();
            // The opposite of the correction is believed: it is wrong.
            if model.contains(&g.complement()) {
                wrong.push(g.complement());
            }
            // The last answer was about the same subject and relation with another value.
            if let Some(l) = &last {
                if l.pred == g.pred && l.args.first() == g.args.first() && l != &g && model.contains(l) {
                    wrong.push(l.clone());
                }
            }
            for w in wrong {
                if let Some(r) = self.make_false(&w, now, &mut retracted) {
                    rules.push(r);
                }
            }
            let out = self.kb.assert_fact(Fact::new(g.clone(), &self.source), now, "correction");
            if let AssertOutcome::Added { closed } = out {
                retracted.extend(closed.into_iter().map(|f| f.atom));
            }
            added.push(g);
        }
        retracted.sort();
        retracted.dedup();
        self.last_answer = added.first().cloned();
        self.say(Act::Corrected { added, retracted, rules })
    }

    /// Make `a` false from now on. A stored fact is closed (and its negation
    /// recorded); a derived one has its rule refined by the learner. Returns
    /// the refined rule, if any.
    fn make_false(&mut self, a: &GroundAtom, now: Timestamp, retracted: &mut Vec<GroundAtom>) -> Option<Rule> {
        let stored = self.kb.all_facts().iter().any(|f| f.atom == *a && f.valid_at(now));
        if stored {
            self.retract(a, now, retracted);
            return None;
        }
        let out = corrections::correct_derived(&mut self.kb, a, now, &self.source);
        if !matches!(out.change, Change::NotDerived) {
            retracted.push(a.clone());
        }
        out.new_rule
    }

    /// Learn effect rules from a before/after demonstration.
    pub fn demonstrate(&mut self, demo: &Demonstration) -> Vec<String> {
        let now = self.now();
        for f in &demo.after {
            self.kb.assert_fact(Fact::new(f.clone(), &self.source), now, "demonstration");
        }
        self.kb.assert_fact(Fact::new(demo.action.clone(), &self.source), now, "demonstration");
        let learner = self.learner.clone();
        let out = corrections::learn_from_demo(&mut self.kb, &learner, demo, now);
        let _ = self.save();
        out
    }

    /// Make `a` false from now on: close it if stored, and record the negation.
    fn retract(&mut self, a: &GroundAtom, now: Timestamp, retracted: &mut Vec<GroundAtom>) {
        let pattern = a.to_atom();
        let closed = self.kb.close_matching(&pattern, now, "user said it is wrong");
        retracted.push(a.clone());
        retracted.extend(closed.into_iter().map(|f| f.atom));
        self.kb.assert_fact(Fact::new(a.complement(), &self.source), now, "user said it is wrong");
    }

    fn answer_pending(&mut self, pending: Option<Pending>, yes: bool) -> Reply {
        let now = self.now();
        match pending {
            Some(Pending::Fact(a)) => {
                let g = if yes { a } else { a.complement() };
                self.kb.assert_fact(Fact::new(g.clone(), &self.source), now, "answer to my question");
                self.last_answer = Some(g.clone());
                self.say(Act::Acknowledge { added: vec![g], closed: vec![], known: vec![], rules: vec![], past: false })
            }
            Some(Pending::Rule(id)) => {
                let stem = sla_core::vault::file_stem(&id);
                if let Some(rf) = self.kb.vault.rules.get_mut(&stem) {
                    let mut m = rf.meta();
                    m.status = if yes { RuleStatus::Confirmed } else { RuleStatus::Rejected };
                    if yes {
                        m.conf = m.conf.max(0.9);
                    }
                    rf.set_meta(&m);
                    self.kb.vault.mark_rule_dirty(&stem);
                    let action = if yes { "confirm_rule" } else { "reject_rule" };
                    self.kb.log_change(now, action, &id, "user answered my question");
                }
                self.say(Act::Note(if yes { "Thanks, I'll treat that as a rule now.".into() } else { "Okay, I'll drop that idea.".into() }))
            }
            Some(Pending::Contradiction(a, b)) => {
                // "yes" keeps the first, "no" the second.
                let keep = if yes { a } else { b };
                self.kb.assert_fact(Fact::new(keep.clone(), &self.source), now, "resolved a contradiction");
                self.say(Act::Acknowledge { added: vec![keep], closed: vec![], known: vec![], rules: vec![], past: false })
            }
            Some(Pending::Wh(_)) | None => self.say(Act::Note(if yes { "Good.".into() } else { "Okay.".into() })),
        }
    }

    /// Facts currently believed, rendered as Datalog lines (for the CLI).
    pub fn dump_facts(&self) -> Vec<String> {
        let now = self.now();
        let mut v: Vec<String> = self.kb.all_facts().iter().filter(|f| f.valid_at(now)).map(|f| f.to_line()).collect();
        v.sort();
        v
    }

    pub fn dump_rules(&self) -> Vec<String> {
        self.kb.rules(true).iter().filter(|r| r.module != "core").map(|r| format!("{} [{} {}] {}", r.id, r.status.as_str(), sla_core::term::fmt_conf(r.conf), r.rule)).collect()
    }

    /// True if `pattern` matches a fact valid now.
    pub fn believes(&self, pattern: &str) -> bool {
        let Ok(p) = sla_core::parse_atom(pattern) else { return false };
        self.model(false).all_facts().iter().any(|g| matches(&p, g))
    }
}

/// Convenience for building a constant term.
pub fn sym(s: &str) -> Term {
    Term::Val(Value::sym(s))
}

/// Capitalized words that are not the first word of a sentence, joined when
/// adjacent ("New York" -> new_york). "I" and its contractions are skipped.
pub fn proper_names(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current: Vec<String> = Vec::new();
    let mut sentence_start = true;
    let flush = |current: &mut Vec<String>, out: &mut Vec<String>| {
        if !current.is_empty() {
            let n = current.join("_");
            if !out.contains(&n) {
                out.push(n);
            }
            current.clear();
        }
    };
    for raw in text.split_whitespace() {
        let ends_sentence = raw.ends_with(['.', '!', '?']);
        let breaks = ends_sentence || raw.ends_with([',', ';', ':']);
        let w = raw.trim_matches(|c: char| !c.is_alphanumeric() && c != '\'');
        let w = w.strip_suffix("'s").unwrap_or(w).trim_matches('\'');
        let is_i = w == "I" || w.starts_with("I'");
        let cap = w.chars().next().map_or(false, |c| c.is_uppercase()) && w.chars().skip(1).all(|c| c.is_lowercase() || c == '-');
        if cap && !is_i && !sentence_start && w.len() > 1 {
            current.push(w.to_lowercase());
        } else {
            flush(&mut current, &mut out);
        }
        if breaks {
            flush(&mut current, &mut out);
        }
        sentence_start = ends_sentence;
    }
    flush(&mut current, &mut out);
    out
}

#[cfg(test)]
mod tests {
    #[test]
    fn names() {
        assert_eq!(super::proper_names("Anna lives in New York. Is Rex a dog? I'm Maksim"), vec!["new_york", "rex", "maksim"]);
        assert!(super::proper_names("Hello there.").is_empty());
    }
}
