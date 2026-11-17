//! Learner, first version: pattern counting.
//!
//! Every entity (first argument of a fact) has *features*: its facts with the
//! entity replaced by a variable, e.g. `during(X, morning)`, `drink(X, coffee)`.
//! When feature B appears together with feature A on enough entities, the
//! learner proposes the hypothesis rule `B :- A` with low confidence. Each
//! update recounts support (entities with A and B) and counterexamples
//! (entities with A but not B), recomputes confidence, promotes the rule to
//! confirmed past a threshold and rejects it when counterexamples dominate.
//! Every change is written to the rule file and the audit log with its reason.

pub mod corrections;

use sla_core::{Atom, GroundAtom, KnowledgeBase, Literal, Rule, RuleFile, RuleMeta, RuleStatus, Term, Timestamp, Value};
use sla_reasoner::{evaluate, facts_for, RuleInput};
use std::collections::{BTreeMap, BTreeSet};

/// A fact with its subject replaced by a variable.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Feature {
    pub pred: String,
    pub rest: Vec<Value>,
}

impl Feature {
    pub fn atom(&self, var: &str) -> Atom {
        let mut args = vec![Term::var(var)];
        args.extend(self.rest.iter().cloned().map(Term::Val));
        Atom { pred: self.pred.clone(), args }
    }
}

/// Predicates that describe the bookkeeping or structure of things, not
/// behaviour worth predicting. They may appear in rule bodies only.
const NO_HEAD: &[&str] = &["event", "agent_of", "during", "before", "thing", "proper_name", "same", "person", "place", "is_a", "core_predicate", "functional"];
/// Never used as features at all.
const IGNORE: &[&str] = &["proper_name", "thing", "core_predicate", "functional"];

#[derive(Debug, Clone)]
pub struct Learner {
    /// Supporting entities needed before a hypothesis is proposed.
    pub min_support: u64,
    /// Fraction of A-entities that must also have B to propose.
    pub min_ratio: f64,
    /// Prior weight: conf = evidence / (evidence + counter + k).
    pub k: f64,
    /// Promote to confirmed at or above this confidence.
    pub promote_at: f64,
}

impl Default for Learner {
    fn default() -> Self {
        Learner { min_support: 3, min_ratio: 0.75, k: 5.0, promote_at: 0.8 }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LearnReport {
    /// New hypothesis rule ids.
    pub proposed: Vec<String>,
    /// (id, old conf, new conf) for rules whose confidence changed.
    pub changed: Vec<(String, f64, f64)>,
    pub promoted: Vec<String>,
    pub rejected: Vec<String>,
}

/// Round to 3 decimals so files and comparisons stay stable.
fn r3(x: f64) -> f64 {
    (x * 1000.0).round() / 1000.0
}

impl Learner {
    pub fn conf(&self, evidence: u64, counter: u64) -> f64 {
        r3(evidence as f64 / (evidence as f64 + counter as f64 + self.k))
    }

    /// The rules whose conclusions count as evidence: core and confirmed rules
    /// that the learner did not guess itself (a guessed rule must not support itself).
    pub fn evidence_rules(kb: &KnowledgeBase) -> Vec<RuleInput> {
        kb.rules(false)
            .iter()
            .filter(|r| {
                let stem = r.id.split('#').next().unwrap_or("");
                kb.vault.rules.get(stem).map_or(true, |rf| rf.meta().source != "learner")
            })
            .map(RuleInput::from)
            .collect()
    }

    /// Does the vault have confirmed rules the learner guessed? If not, the
    /// agent's own model is also the evidence model and can be shared.
    pub fn has_confirmed_guesses(kb: &KnowledgeBase) -> bool {
        kb.vault.rules.values().any(|rf| {
            let m = rf.meta();
            m.source == "learner" && m.status == RuleStatus::Confirmed
        })
    }

    /// Features of every entity in a model.
    pub fn features_from(&self, m: &sla_reasoner::Model) -> BTreeMap<Value, BTreeSet<Feature>> {
        let mut out: BTreeMap<Value, BTreeSet<Feature>> = BTreeMap::new();
        for g in m.all_facts() {
            if g.args.is_empty() || IGNORE.contains(&g.base_pred()) || g.args.len() > 3 {
                continue;
            }
            out.entry(g.args[0].clone()).or_default().insert(Feature { pred: g.pred.clone(), rest: g.args[1..].to_vec() });
        }
        out
    }

    /// Features of every entity in the current model (learned hypotheses excluded).
    pub fn features(&self, kb: &KnowledgeBase, now: Timestamp) -> BTreeMap<Value, BTreeSet<Feature>> {
        let rules = Self::evidence_rules(kb);
        let Ok(m) = evaluate(&facts_for(&kb.all_facts(), Some(now)), &rules) else { return BTreeMap::new() };
        self.features_from(&m)
    }

    /// Count (support, counterexamples) for `head :- body` with one shared variable.
    pub fn count(features: &BTreeMap<Value, BTreeSet<Feature>>, body: &[Feature], negs: &[Feature], head: &Feature) -> (u64, u64) {
        let mut support = 0;
        let mut counter = 0;
        for fs in features.values() {
            if body.iter().all(|b| fs.contains(b)) && !negs.iter().any(|n| fs.contains(n)) {
                let neg = Feature { pred: sla_core::term::complement_pred(&head.pred), rest: head.rest.clone() };
                if fs.contains(head) {
                    support += 1;
                } else if fs.contains(&neg) || !is_open_world(head) {
                    counter += 1;
                }
            }
        }
        (support, counter)
    }

    /// The full pass (the offline consolidation): recount everything, propose
    /// new hypotheses from every pattern, update and promote/reject old ones.
    pub fn update(&self, kb: &mut KnowledgeBase, now: Timestamp) -> LearnReport {
        let features = self.features(kb, now);
        self.update_with(kb, now, &features, None)
    }

    /// The cheap pass for one turn: re-score every hypothesis, and propose new
    /// ones only around the entities in `focus` (the ones the turn told
    /// something about): patterns elsewhere did not change.
    pub fn update_turn(&self, kb: &mut KnowledgeBase, now: Timestamp, features: &BTreeMap<Value, BTreeSet<Feature>>, focus: &BTreeSet<Value>) -> LearnReport {
        self.update_with(kb, now, features, Some(focus))
    }

    fn update_with(&self, kb: &mut KnowledgeBase, now: Timestamp, features: &BTreeMap<Value, BTreeSet<Feature>>, focus: Option<&BTreeSet<Value>>) -> LearnReport {
        let mut report = LearnReport::default();
        // Stated (stored) features, to tell discoveries from consequences of known rules.
        let mut stated: BTreeMap<Value, BTreeSet<Feature>> = BTreeMap::new();
        for f in kb.all_facts() {
            if f.valid_at(now) && !f.atom.args.is_empty() {
                stated.entry(f.atom.args[0].clone()).or_default().insert(Feature { pred: f.atom.pred.clone(), rest: f.atom.args[1..].to_vec() });
            }
        }

        // 1. Re-score existing learner hypotheses.
        let ids: Vec<String> = kb.vault.rules.keys().cloned().collect();
        for stem in ids {
            let rf = &kb.vault.rules[&stem];
            let meta = rf.meta();
            if meta.source != "learner" || meta.status == RuleStatus::Rejected {
                continue;
            }
            let Some((body, negs, head)) = rf.rules().first().and_then(rule_features) else { continue };
            let (support, counter) = Self::count(features, &body, &negs, &head);
            let conf = self.conf(support, counter);
            let mut m = meta.clone();
            m.evidence = support;
            m.counter = counter;
            m.conf = conf;
            if m.status == RuleStatus::Hypothesis {
                if conf >= self.promote_at {
                    m.status = RuleStatus::Confirmed;
                    report.promoted.push(meta.id.clone());
                    kb.log_change(now, "promote_rule", &meta.id, &format!("confidence {conf} reached {} ({support} cases, {counter} counterexamples)", self.promote_at));
                } else if counter >= 2 * support && counter >= self.min_support {
                    m.status = RuleStatus::Rejected;
                    report.rejected.push(meta.id.clone());
                    kb.log_change(now, "reject_rule", &meta.id, &format!("{counter} counterexamples against {support} cases"));
                }
            } else if m.status == RuleStatus::Confirmed && counter > support {
                // A confirmed learned rule that stopped fitting goes back to being a hypothesis.
                m.status = RuleStatus::Hypothesis;
                kb.log_change(now, "demote_rule", &meta.id, &format!("{counter} counterexamples against {support} cases"));
            }
            if m != meta {
                if (m.conf - meta.conf).abs() > 1e-9 {
                    report.changed.push((meta.id.clone(), meta.conf, m.conf));
                    kb.log_change(now, "rule_conf", &meta.id, &format!("{} -> {} ({support} cases, {counter} counterexamples)", meta.conf, m.conf));
                }
                let rf = kb.vault.rules.get_mut(&stem).unwrap();
                rf.set_meta(&m);
                kb.vault.mark_rule_dirty(&stem);
            }
        }

        // 2. Propose new hypotheses: best single-feature body per head feature.
        // An inverted index (feature -> entities) keeps counting cheap.
        let mut ents_of: BTreeMap<&Feature, Vec<&Value>> = BTreeMap::new();
        for (e, fs) in features {
            for f in fs {
                ents_of.entry(f).or_default().push(e);
            }
        }
        let count_of = |f: &Feature| ents_of.get(f).map_or(0, |v| v.len() as u64);
        // Heads worth looking at: all of them in the full pass; in a turn, the
        // features of the focus entities and everything that co-occurs with them.
        let mut heads: BTreeSet<&Feature> = BTreeSet::new();
        match focus {
            None => heads.extend(ents_of.keys().copied()),
            Some(focus) => {
                for e in focus {
                    for f in features.get(e).into_iter().flatten() {
                        heads.insert(f);
                        for e2 in ents_of.get(f).into_iter().flatten() {
                            heads.extend(features.get(*e2).into_iter().flatten());
                        }
                    }
                }
            }
        }
        let mut best: BTreeMap<Feature, (Feature, u64, u64, f64)> = BTreeMap::new();
        for head in heads {
            let hc = count_of(head);
            if hc < self.min_support || NO_HEAD.contains(&head.pred.trim_start_matches('-')) {
                continue;
            }
            // Bodies that co-occur with the head at least once.
            let mut bodies: BTreeSet<&Feature> = BTreeSet::new();
            for e in ents_of.get(head).into_iter().flatten() {
                bodies.extend(features.get(*e).into_iter().flatten());
            }
            let neg = Feature { pred: sla_core::term::complement_pred(&head.pred), rest: head.rest.clone() };
            for body in bodies {
                let bc = count_of(body);
                if body == head || bc < self.min_support || body.pred == head.pred {
                    continue;
                }
                let (mut support, mut counter, mut stated_support) = (0u64, 0u64, 0u64);
                for e in ents_of.get(body).into_iter().flatten() {
                    let fs = &features[*e];
                    if fs.contains(head) {
                        support += 1;
                        // A pattern is a discovery only if enough cases state the head themselves;
                        // otherwise existing rules already explain it (e.g. inherited parts).
                        if stated.get(*e).map_or(false, |st| st.contains(head)) {
                            stated_support += 1;
                        }
                    } else if fs.contains(&neg) || !is_open_world(head) {
                        counter += 1;
                    }
                }
                if support < self.min_support || stated_support < self.min_support {
                    continue;
                }
                let ratio = support as f64 / bc as f64;
                if ratio < self.min_ratio {
                    continue;
                }
                let better = match best.get(head) {
                    None => true,
                    Some((b, _, _, r)) => {
                        // Higher ratio; then the more specific body; then bodies with constants.
                        (ratio, -(bc as i64), body.rest.len(), !NO_HEAD.contains(&body.pred.as_str()) as u8)
                            > (*r, -(count_of(b) as i64), b.rest.len(), !NO_HEAD.contains(&b.pred.as_str()) as u8)
                    }
                };
                if better {
                    best.insert(head.clone(), (body.clone(), support, counter, ratio));
                }
            }
        }
        for (head, (body, support, counter, _)) in best {
            let rule = Rule { head: head.atom("X"), body: vec![Literal::Pos(body.atom("X"))] };
            if kb.find_rule(&rule).is_some() {
                continue;
            }
            // Already implied for every entity by a stored rule? Then it adds nothing.
            let base = format!("hyp_{}", head.pred.trim_start_matches('-'));
            let mut id = base.clone();
            let mut n = 2;
            while kb.vault.rules.contains_key(&sla_core::vault::file_stem(&id)) {
                id = format!("{base}_{n}");
                n += 1;
            }
            let conf = self.conf(support, counter);
            let meta = RuleMeta {
                id: id.clone(),
                status: RuleStatus::Hypothesis,
                conf,
                evidence: support,
                counter,
                module: module_for(&head.pred).into(),
                source: "learner".into(),
            };
            let note = format!("Guessed from {support} cases where both were true ({counter} counterexamples).");
            kb.vault.put_rule_file(RuleFile::new(&meta, &note, &[rule.clone()]));
            kb.log_change(now, "propose_rule", &format!("{id}: {rule}"), &note);
            report.proposed.push(id);
        }
        report
    }
}

/// Closed world for observed episodes: if an entity has the body but the head is
/// missing, that counts against the rule. Predicates listed here are open-world
/// (missing means unknown, only an explicit negation counts).
fn is_open_world(head: &Feature) -> bool {
    matches!(head.pred.trim_start_matches('-'), "knows" | "wants" | "has")
}

/// `head(X, c..) :- body(X, c..), ..., not cond(X, c..)` -> (body, negated, head)
/// features, if the rule has that shape.
pub fn rule_features(r: &Rule) -> Option<(Vec<Feature>, Vec<Feature>, Feature)> {
    let feat = |a: &Atom| -> Option<Feature> {
        match a.args.first()? {
            Term::Var(v) if v == "X" => {}
            _ => return None,
        }
        let mut rest = Vec::new();
        for t in &a.args[1..] {
            match t {
                Term::Val(v) => rest.push(v.clone()),
                Term::Var(_) => return None,
            }
        }
        Some(Feature { pred: a.pred.clone(), rest })
    };
    let head = feat(&r.head)?;
    let mut body = Vec::new();
    let mut negs = Vec::new();
    for l in &r.body {
        match l {
            Literal::Pos(a) => body.push(feat(a)?),
            Literal::Neg(a) => negs.push(feat(a)?),
            Literal::Cmp(..) | Literal::Arith(..) | Literal::Agg { .. } => return None,
        }
    }
    Some((body, negs, head))
}

/// Same routing as the agent's module router (kept here to avoid a dependency cycle).
pub fn module_for(pred: &str) -> &'static str {
    match pred.trim_start_matches('-') {
        "likes" | "knows" | "wants" | "feels" => "people",
        "located_in" => "places",
        "has" | "part_of" | "property" => "things",
        p if p.starts_with("drink") || p.starts_with("eat") || p.starts_with("cook") => "food",
        p if p.starts_with("work") => "work",
        _ => "general",
    }
}

/// Helper for tests and simulations: ground atom from text.
pub fn g(s: &str) -> GroundAtom {
    sla_core::parse_ground(s).expect("valid ground atom")
}
