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
    /// A task someone asked for that the agent could not do.
    UnknownTask { task: String, failures: usize },
    /// A known task with something missing: "steps", "needs" or "duration".
    TaskGap { task: String, missing: String, failures: usize },
    /// A message the agent did not understand.
    Unclear { text: String, failures: usize },
    ResolveContradiction(GroundAtom, GroundAtom),
    ConfirmHypothesis { id: String, rule: sla_core::Rule },
    FillGap(Atom),
}

impl Goal {
    /// Stable key so the same question is not asked twice.
    pub fn key(&self) -> String {
        match self {
            Goal::UnknownTask { task, .. } => format!("task:{task}"),
            Goal::TaskGap { task, missing, .. } => format!("gap:{task}:{missing}"),
            Goal::Unclear { text, .. } => format!("unclear:{text}"),
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
        let mut out = task_goals(kb, m);
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
        // Named things the user talks about (subjects of facts) form a category too:
        // if most named subjects have a job, the one without is a gap.
        let subjects: BTreeSet<Value> = m
            .all_facts()
            .into_iter()
            .filter(|g| !matches!(g.base_pred(), "proper_name" | "thing" | "same" | "is_a" | "person" | "place"))
            .filter_map(|g| g.args.first().cloned())
            .collect();
        for g in m.facts_of("proper_name") {
            if subjects.contains(&g.args[0]) {
                cats.entry(Value::sym("named things")).or_default().insert(g.args[0].clone());
            }
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

const NOT_UNDERSTOOD: &str = "Sorry, I didn't understand that.";

fn ledger(kb: &KnowledgeBase) -> std::path::PathBuf {
    kb.vault.root.join("queue").join("failures.jsonl")
}

/// Keep a failed turn: a task the agent could not do, a duration it did not
/// know, or a message it did not understand.
pub fn record_failure(kb: &KnowledgeBase, message: &str, reply: &str, source: &str) {
    let task_after = |prefix: &str, suffix: &str| -> Option<String> {
        let rest = reply.split_once(prefix)?.1;
        let t = rest.split_once(suffix)?.0;
        Some(t.trim().replace(' ', "_"))
    };
    let (kind, key) = if let Some(t) = task_after("I don't know how to ", " yet") {
        ("unknown_task", t)
    } else if let Some(t) = task_after("I don't know how long ", " takes") {
        ("unknown_duration", t)
    } else if reply.trim() == NOT_UNDERSTOOD {
        ("not_understood", message.trim().to_string())
    } else {
        return;
    };
    let p = ledger(kb);
    let _ = std::fs::create_dir_all(p.parent().unwrap());
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&p) {
        let _ = writeln!(f, "{}", serde_json::json!({"kind": kind, "key": key, "message": message, "source": source}));
    }
}

/// Goals from failed turns and from gaps in known procedures, scored by how
/// many failed turns an answer would fix.
pub fn task_goals(kb: &KnowledgeBase, m: &Model) -> Vec<ScoredGoal> {
    let mut counts: std::collections::BTreeMap<(String, String), usize> = Default::default();
    for l in std::fs::read_to_string(ledger(kb)).unwrap_or_default().lines() {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(l) {
            if let (Some(k), Some(key)) = (v["kind"].as_str(), v["key"].as_str()) {
                *counts.entry((k.to_string(), key.to_string())).or_default() += 1;
            }
        }
    }
    let known = |t: &str| kb.vault.procedures.contains_key(t) || kb.vault.tasks.contains_key(t);
    let score = |failures: usize, base: f64| (base + 0.1 * failures as f64).min(0.95);
    let mut out = Vec::new();
    for ((kind, key), n) in &counts {
        match kind.as_str() {
            "unknown_task" if !known(key) => out.push(ScoredGoal { goal: Goal::UnknownTask { task: key.clone(), failures: *n }, score: score(*n, 0.5) }),
            "unknown_duration" if known(key) => out.push(ScoredGoal { goal: Goal::TaskGap { task: key.clone(), missing: "duration".into(), failures: *n }, score: score(*n, 0.45) }),
            "not_understood" => out.push(ScoredGoal { goal: Goal::Unclear { text: key.clone(), failures: *n }, score: score(*n, 0.35) }),
            _ => {}
        }
    }
    // Gaps in what a procedure says.
    for (t, pf) in &kb.vault.procedures {
        let facts = pf.facts();
        let has = |p: &str| facts.iter().any(|f| f.atom.pred == p);
        let total_time = has("takes_time") || m.facts_of("total_minutes").iter().any(|a| a.args.first().and_then(|v| v.as_sym()) == Some(t.as_str()));
        let failures = counts.iter().filter(|((_, k), _)| k == t).map(|(_, n)| *n).sum::<usize>();
        for (missing, gap) in [("steps", !has("step")), ("needs", !has("needs")), ("duration", !total_time)] {
            if gap {
                out.push(ScoredGoal { goal: Goal::TaskGap { task: t.clone(), missing: missing.into(), failures }, score: score(failures, 0.4) });
            }
        }
    }
    out
}

/// HTN-style decomposition: goal -> method -> primitive task (one question).
pub fn plan(goal: &Goal) -> (Act, Pending) {
    match goal {
        Goal::UnknownTask { task, .. } => {
            let q = format!("How do I {}? What do I need, and what is the first step?", task.replace('_', " "));
            (Act::Note(q.clone()), Pending::Open(q))
        }
        Goal::TaskGap { task, missing, .. } => {
            let t = task.replace('_', " ");
            let q = match missing.as_str() {
                "needs" => format!("What do I need to {t}?"),
                "duration" => format!("How long does it take to {t}?"),
                _ => format!("What are the steps to {t}?"),
            };
            (Act::Note(q.clone()), Pending::Open(q))
        }
        Goal::Unclear { text, .. } => {
            let q = format!("Earlier I did not understand \"{text}\". Could you say it in simpler words?");
            (Act::Note(q.clone()), Pending::Open(q))
        }
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
