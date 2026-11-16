//! Drives and goals: the curiosity drive.
//!
//! Curiosity scores what the agent does not know well:
//! - **contradictions**: `p` and `-p` both believed (score 1.0);
//! - **weak hypotheses**: learned rules not yet confirmed (0.5 to 0.9, more
//!   evidence first, since those are closest to being useful);
//! - **gaps**: a known thing lacks a property most of its peers have, e.g.
//!   everyone else has a job (0.3 to 0.5).
//!
//! The drive level rises with open goals and falls when the agent learns
//! (rewarding learning progress, not raw novelty). A tiny HTN-style planner
//! turns the chosen goal into a primitive task: one question to ask, plus what
//! answer it is waiting for.

use crate::Pending;
use sla_core::{Atom, GroundAtom, KnowledgeBase, RuleStatus, Term, Value};
use sla_nlg::Act;
use sla_reasoner::Model;
use std::collections::{BTreeMap, BTreeSet, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub enum Goal {
    ResolveContradiction(GroundAtom, GroundAtom),
    ConfirmHypothesis { id: String, rule: sla_core::Rule },
    FillGap(Atom),
}

impl Goal {
    /// Stable key so the same question is not asked twice.
    pub fn key(&self) -> String {
        match self {
            Goal::ResolveContradiction(a, _) => format!("contradiction:{a}"),
            Goal::ConfirmHypothesis { rule, .. } => format!("hypothesis:{rule}"),
            Goal::FillGap(a) => format!("gap:{a}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScoredGoal {
    pub goal: Goal,
    pub score: f64,
}

/// Predicates worth filling when peers have them; `property` gaps are per property name.
const GAP_PREDS: &[&str] = &["likes", "located_in", "property", "work_at"];

#[derive(Debug, Clone)]
pub struct Curiosity {
    /// Drive level in [0, 1].
    pub level: f64,
    /// Ask when the best goal scores at least this and the level is high enough.
    pub ask_threshold: f64,
    /// Questions per day added to replies while the user is chatting.
    pub chat_cap: usize,
    /// Proactive messages (agent starts the conversation) per day.
    pub proactive_cap: usize,
    /// Goal keys already asked (also restored from the log).
    pub asked: HashSet<String>,
}

impl Default for Curiosity {
    fn default() -> Self {
        Curiosity { level: 0.0, ask_threshold: 0.3, chat_cap: 8, proactive_cap: 3, asked: HashSet::new() }
    }
}

/// Is `t` the variable W?
fn is_w(t: &Term) -> bool {
    matches!(t, Term::Var(v) if v == "W")
}

impl Curiosity {
    /// All goals, best first, excluding ones already asked.
    pub fn goals(&self, kb: &KnowledgeBase, m: &Model) -> Vec<ScoredGoal> {
        let mut out = Vec::new();
        for (a, b) in m.contradictions() {
            out.push(ScoredGoal { goal: Goal::ResolveContradiction(a, b), score: 1.0 });
        }
        for (stem, rf) in &kb.vault.rules {
            let meta = rf.meta();
            if meta.status != RuleStatus::Hypothesis {
                continue;
            }
            if let Some(rule) = rf.rules().into_iter().next() {
                let score = 0.5 + 0.4 * (meta.evidence as f64 / 5.0).min(1.0);
                out.push(ScoredGoal { goal: Goal::ConfirmHypothesis { id: stem.clone(), rule }, score });
            }
        }
        out.extend(self.gaps(m));
        out.retain(|g| !self.asked.contains(&g.goal.key()));
        out.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap().then_with(|| a.goal.key().cmp(&b.goal.key())));
        out
    }

    /// Unknown properties of known things: members of a category that lack a
    /// predicate most other members have.
    pub fn gaps(&self, m: &Model) -> Vec<ScoredGoal> {
        // category -> members
        let mut cats: BTreeMap<Value, BTreeSet<Value>> = BTreeMap::new();
        for g in m.facts_of("is_a") {
            cats.entry(g.args[1].clone()).or_default().insert(g.args[0].clone());
        }
        for g in m.facts_of("person") {
            cats.entry(Value::sym("person")).or_default().insert(g.args[0].clone());
        }
        // (entity, gap key) present
        let mut has: BTreeSet<(Value, String)> = BTreeSet::new();
        for p in GAP_PREDS {
            for g in m.facts_of(p) {
                let key = if *p == "property" { format!("property:{}", g.args[1]) } else { p.to_string() };
                has.insert((g.args[0].clone(), key));
            }
        }
        let keys: BTreeSet<String> = has.iter().map(|(_, k)| k.clone()).collect();
        let mut out = Vec::new();
        let mut seen = BTreeSet::new();
        for members in cats.values() {
            if members.len() < 3 {
                continue;
            }
            for k in &keys {
                let holders = members.iter().filter(|e| has.contains(&((*e).clone(), k.clone()))).count();
                // Most peers (at least two, and more than half) have it.
                if holders < 2 || holders * 2 <= members.len() {
                    continue;
                }
                for e in members {
                    if has.contains(&(e.clone(), k.clone())) || !seen.insert((e.clone(), k.clone())) {
                        continue;
                    }
                    let pattern = match k.strip_prefix("property:") {
                        Some(p) => Atom::new("property", vec![Term::Val(e.clone()), Term::sym(p), Term::var("W")]),
                        None => Atom::new(k, vec![Term::Val(e.clone()), Term::var("W")]),
                    };
                    let frac = holders as f64 / members.len() as f64;
                    out.push(ScoredGoal { goal: Goal::FillGap(pattern), score: 0.3 + 0.2 * frac });
                }
            }
        }
        out
    }

    /// Update the drive from the current goals: rises with what is open.
    pub fn update(&mut self, goals: &[ScoredGoal]) {
        let top = goals.first().map_or(0.0, |g| g.score);
        self.level = (self.level * 0.7 + 0.3 * top + 0.05 * goals.len().min(4) as f64).min(1.0);
    }

    /// Learning progress lowers the drive (the reward).
    pub fn reward(&mut self, amount: f64) {
        self.level = (self.level - amount).max(0.0);
    }

    pub fn wants_to_ask(&self, goals: &[ScoredGoal]) -> bool {
        goals.first().map_or(false, |g| g.score >= self.ask_threshold) && self.level >= self.ask_threshold
    }
}

/// HTN-style decomposition: goal -> method -> primitive task (one question).
pub fn plan(goal: &Goal) -> (Act, Pending) {
    match goal {
        Goal::ResolveContradiction(a, b) => (Act::AskContradiction(a.clone(), b.clone()), Pending::Contradiction(a.clone(), b.clone())),
        Goal::ConfirmHypothesis { id, rule } => (Act::AskRule(rule.clone()), Pending::Rule(id.clone())),
        Goal::FillGap(p) => {
            debug_assert!(p.args.iter().any(is_w));
            (Act::AskWh(p.clone()), Pending::Wh(p.clone()))
        }
    }
}

/// Who answers the agent's questions. In the first version a Claude teacher
/// sub-agent (see the eval crate); tests use scripted teachers.
pub trait Teacher {
    fn answer(&mut self, question: &str) -> String;
}

/// A scripted teacher: answers by matching a substring of the question.
pub struct ScriptedTeacher {
    pub answers: Vec<(String, String)>,
    pub default: String,
}

impl Teacher for ScriptedTeacher {
    fn answer(&mut self, question: &str) -> String {
        self.answers
            .iter()
            .find(|(k, _)| question.to_lowercase().contains(&k.to_lowercase()))
            .map(|(_, a)| a.clone())
            .unwrap_or_else(|| self.default.clone())
    }
}
