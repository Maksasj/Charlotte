//! The offline consolidation pass (Design 2.0, "Two learning times"), run
//! between sessions. During a turn the agent does only cheap work; here it
//! does the heavy work over the whole vault:
//!
//! - the learner's full rule search (`Learner::update`), not just around what
//!   one turn talked about;
//! - the hygiene pass (duplicates linked and queued as merges);
//! - steps shared by several procedures (the same action on the same object),
//!   reported as candidates for a shared sub-task;
//! - old forms migrated: "B can replace A" stored as the verb `replace(B, A)`
//!   (or, with a plural subject, the rule `replace(X, A) :- is_a(X, B)`) by
//!   builds before that was fixed becomes `substitute(A, B)`.

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
    /// `replace` facts and rules migrated to `substitute`.
    pub substitutes_migrated: usize,
    pub ms: f64,
}

/// Migrate `replace(B, A)` facts and `replace(X, A) :- is_a(X, B)` rules to
/// `substitute(A, B)` facts. Returns how many were migrated.
pub fn migrate_substitutes(b: &mut Brain, now: Timestamp) -> usize {
    use sla_core::{Fact, GroundAtom, Literal, Term, Value};
    let mut n = 0;
    let olds: Vec<Fact> = b.kb.all_facts().into_iter().filter(|f| f.is_open() && f.atom.pred == "replace" && f.atom.args.len() == 2).collect();
    for f in olds {
        let sub = GroundAtom::new("substitute", vec![f.atom.args[1].clone(), f.atom.args[0].clone()]);
        if let Some(id) = &f.id {
            b.kb.close_by_id(id, now, "migrated to substitute");
        }
        b.kb.assert_fact(Fact::new(sub, &f.source), now, "migrated from replace");
        n += 1;
    }
    let stems: Vec<String> = b.kb.vault.rules.keys().cloned().collect();
    for stem in stems {
        let rules = b.kb.vault.rules[&stem].rules();
        let [r] = rules.as_slice() else { continue };
        if r.head.pred != "replace" || r.head.args.len() != 2 {
            continue;
        }
        let [Literal::Pos(body)] = r.body.as_slice() else { continue };
        if body.pred != "is_a" || body.args.len() != 2 || body.args[0] != r.head.args[0] {
            continue;
        }
        let (Term::Val(a @ Value::Sym(_)), Term::Val(bv @ Value::Sym(_))) = (&r.head.args[1], &body.args[1]) else { continue };
        let source = b.kb.vault.rules[&stem].meta().source;
        b.kb.vault.remove_rule_file(&stem);
        b.kb.log_change(now, "remove_rule", &stem, "migrated to substitute");
        b.kb.assert_fact(Fact::new(GroundAtom::new("substitute", vec![a.clone(), bv.clone()]), &source), now, "migrated from a replace rule");
        n += 1;
    }
    n
}

pub fn run(b: &mut Brain, now: Timestamp) -> ConsolidationReport {
    let t = std::time::Instant::now();
    let substitutes_migrated = migrate_substitutes(b, now);
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
    ConsolidationReport { rules_proposed: lr.proposed.len(), rules_changed: lr.changed.len(), hygiene, shared_steps, substitutes_migrated, ms: t.elapsed().as_secs_f64() * 1000.0 }
}
