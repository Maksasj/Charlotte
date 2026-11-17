//! The offline consolidation pass (Design 2.0, "Two learning times"), run
//! between sessions. During a turn the agent does only cheap work; here it
//! does the heavy work over the whole vault:
//!
//! - the learner's full rule search (`Learner::update`), not just around what
//!   one turn talked about;
//! - the hygiene pass (duplicates linked and queued as merges);
//! - steps shared by several procedures (the same action on the same object),
//!   reported as candidates for a shared sub-task.

use crate::Brain;
use serde::Serialize;
use sla_core::Timestamp;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Default)]
pub struct ConsolidationReport {
    pub rules_proposed: usize,
    pub rules_changed: usize,
    pub hygiene: crate::hygiene::HygieneReport,
    /// (action, object) shared by two or more procedures, with them.
    pub shared_steps: Vec<(String, String, Vec<String>)>,
    pub ms: f64,
}

pub fn run(b: &mut Brain, now: Timestamp) -> ConsolidationReport {
    let t = std::time::Instant::now();
    let learner = b.learner.clone();
    let lr = learner.update(&mut b.kb, now);
    let hygiene = crate::hygiene::run(b, now);
    let mut by_step: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
    for (task, pf) in &b.kb.vault.procedures {
        let facts: Vec<_> = pf.facts().iter().map(|f| f.atom.clone()).collect();
        let sym = |g: &sla_core::GroundAtom, i: usize| g.args.get(i).and_then(|v| v.as_sym()).unwrap_or("").to_string();
        for st in facts.iter().filter(|g| g.pred == "step").map(|g| sym(g, 1)) {
            let action = facts.iter().find(|g| g.pred == "action" && sym(g, 0) == st).map(|g| sym(g, 1));
            let object = facts.iter().find(|g| g.pred == "object_of" && sym(g, 0) == st).map(|g| sym(g, 1));
            if let (Some(a), Some(o)) = (action, object) {
                let v = by_step.entry((a, o)).or_default();
                if !v.contains(task) {
                    v.push(task.clone());
                }
            }
        }
    }
    let shared_steps = by_step.into_iter().filter(|(_, v)| v.len() >= 2).map(|((a, o), v)| (a, o, v)).collect();
    b.kb.log_change(now, "consolidate", &format!("{} rules proposed", lr.proposed.len()), "offline consolidation pass");
    let _ = b.save();
    ConsolidationReport { rules_proposed: lr.proposed.len(), rules_changed: lr.changed.len(), hygiene, shared_steps, ms: t.elapsed().as_secs_f64() * 1000.0 }
}
