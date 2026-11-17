//! Learning from corrections and demonstrations.
//!
//! **Corrections.** When a derived conclusion is wrong, the rule that produced
//! it is refined:
//! - *condition*: if the wrong case has a feature that none of the rule's other
//!   (correct) cases share, add `not feature(X)` to the rule body
//!   (`fly(X) :- is_a(X, bird)` + Pingu the penguin -> `..., not is_a(X, penguin)`);
//! - *exception*: otherwise add `not -head` and store the negative fact, so the
//!   rule stops firing for this case only.
//! Core rules are read-only: only the negative fact is stored. A wrong *given*
//! fact is not a rule problem; the caller closes it.
//!
//! **Demonstrations.** A before/after state trace around an action becomes
//! effect rules: added facts become heads, removed facts become negated heads,
//! the action plus unchanged facts about its participants become the body,
//! with the action's constants generalized to variables. Several
//! demonstrations of the same action keep only the preconditions they share.

use crate::{rule_features, Learner};
use sla_core::{Atom, Fact, GroundAtom, KnowledgeBase, Literal, Rule, RuleFile, RuleMeta, RuleStatus, Term, Timestamp, Value};
use sla_reasoner::{evaluate_kb, Model, ProofStep, RuleInput};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq)]
pub enum Change {
    /// A condition `not ...` was added to the rule.
    Condition(Literal),
    /// The exception `not -head` was added (and the negative fact stored).
    Exception,
    /// The rule is core (read-only); only the negative fact was stored.
    CoreRuleUnchanged,
    /// Refining the rule would make the program invalid; the rule is unchanged
    /// and only the negative fact was stored.
    RefinementRefused,
    /// The wrong atom is a stored fact; no rule involved.
    GivenFact,
    /// The atom is not believed at all.
    NotDerived,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CorrectionOutcome {
    /// Rule file stem that changed (if any).
    pub rule: Option<String>,
    pub change: Change,
    pub new_rule: Option<Rule>,
}

/// Features (pred, rest args) of an entity in a model.
fn features_of(m: &Model, e: &Value) -> BTreeSet<(String, Vec<Value>)> {
    m.all_facts()
        .into_iter()
        .filter(|g| g.args.first() == Some(e) && !matches!(g.base_pred(), "proper_name" | "thing" | "same"))
        .map(|g| (g.pred, g.args[1..].to_vec()))
        .collect()
}

/// Fix the rule that derived `wrong`. Stores `-wrong` as a fact (source
/// `source`) in every case except `GivenFact` and `NotDerived`.
pub fn correct_derived(kb: &mut KnowledgeBase, wrong: &GroundAtom, now: Timestamp, source: &str) -> CorrectionOutcome {
    let none = |change| CorrectionOutcome { rule: None, change, new_rule: None };
    let Ok(m) = evaluate_kb(kb, Some(now), false) else { return none(Change::NotDerived) };
    let Some(proof) = m.proof(wrong) else { return none(Change::NotDerived) };
    let (rule_id, rule) = match &proof.step {
        ProofStep::Given { .. } => return none(Change::GivenFact),
        ProofStep::Rule { id, rule, .. } => (id.clone(), rule.clone()),
    };
    let store_negation = |kb: &mut KnowledgeBase| {
        kb.assert_fact(Fact::new(wrong.complement(), source), now, &format!("{source} said {wrong} is wrong"));
    };
    if rule_id.starts_with("core/") {
        store_negation(kb);
        kb.log_change(now, "correction", &wrong.to_string(), &format!("core rule {rule_id} is read-only; stored the exception fact"));
        return none(Change::CoreRuleUnchanged);
    }
    let stem = rule_id.split('#').next().unwrap_or("").to_string();

    // The case variable: the head's first argument.
    let case_var = match rule.head.args.first() {
        Some(Term::Var(v)) => v.clone(),
        _ => String::new(),
    };
    let entity = wrong.args.first().cloned();

    // Other (correct) cases of the same rule: same head predicate, derived by this rule.
    let positives: Vec<Value> = m
        .facts_of(&wrong.pred)
        .into_iter()
        .filter(|g| g != wrong && g.args[1..] == wrong.args[1..])
        .filter(|g| matches!(m.proof(g).map(|p| p.step), Some(ProofStep::Rule { id, .. }) if id == rule_id))
        .filter_map(|g| g.args.first().cloned())
        .collect();

    // Gradual (Design 2.0): the first counterexample is stored as an
    // exception only. A rule changes once a second counterexample shares a
    // feature with an earlier one that the rule's correct cases lack.
    let neg_pred = sla_core::term::complement_pred(&wrong.pred);
    let earlier: Vec<Value> = kb
        .all_facts()
        .into_iter()
        .filter(|f| f.atom.pred == neg_pred && f.valid_at(now) && f.atom.args.len() == wrong.args.len() && f.atom.args[1..] == wrong.args[1..])
        .filter_map(|f| f.atom.args.first().cloned())
        .filter(|v| Some(v) != entity.as_ref())
        .collect();
    let mut condition: Option<Literal> = None;
    if let (Some(e), false, false, false) = (&entity, case_var.is_empty(), positives.is_empty(), earlier.is_empty()) {
        let mut mine = features_of(&m, e);
        // Keep only what at least one earlier counterexample shares.
        let shared: BTreeSet<(String, Vec<Value>)> = earlier.iter().flat_map(|x| features_of(&m, x)).collect();
        mine.retain(|f| shared.contains(f));
        // A feature held by half or more of the rule's other cases does not
        // set the counterexamples apart. (Uncorrected cases may still be
        // wrong, so a few other cases with the feature do not rule it out.)
        let per_case: Vec<BTreeSet<(String, Vec<Value>)>> = positives.iter().map(|p| features_of(&m, p)).collect();
        let theirs: BTreeSet<(String, Vec<Value>)> =
            per_case.iter().flatten().filter(|f| per_case.iter().filter(|c| c.contains(*f)).count() * 2 >= per_case.len()).cloned().collect();
        // Count how common each distinguishing feature is: prefer categories, then rare ones.
        let mut candidates: Vec<&(String, Vec<Value>)> = mine
            .iter()
            .filter(|f| !theirs.contains(*f))
            .filter(|(p, _)| p.trim_start_matches('-') != wrong.base_pred() && !p.starts_with('-'))
            .collect();
        candidates.sort_by_key(|(p, rest)| (p != "is_a", rest.len() != 1, p.clone(), rest.clone()));
        if let Some((p, rest)) = candidates.first() {
            let mut args = vec![Term::Var(case_var.clone())];
            args.extend(rest.iter().cloned().map(Term::Val));
            condition = Some(Literal::Neg(Atom { pred: p.clone(), args }));
        }
    }
    let (literal, change) = match condition {
        Some(c) => (c.clone(), Change::Condition(c)),
        None => {
            let neg_head = Atom { pred: sla_core::term::complement_pred(&rule.head.pred), args: rule.head.args.clone() };
            (Literal::Neg(neg_head), Change::Exception)
        }
    };
    if !kb.vault.rules.contains_key(&stem) {
        return none(Change::NotDerived);
    }
    let mut new_rule = rule.clone();
    if !new_rule.body.contains(&literal) {
        new_rule.body.push(literal);
    }
    // All or nothing: the negative fact, the refined rule and the raised
    // counter are kept together, or none of them is.
    let refined = kb.transaction(|kb| -> Result<(), ()> {
        store_negation(kb);
        let rf = kb.vault.rules.get_mut(&stem).ok_or(())?;
        let old = rule.clone();
        let nr = new_rule.clone();
        rf.doc.update_items(|item| match item {
            sla_core::doc::Item::Rule(r) if *r == old => {
                *r = nr.clone();
                true
            }
            _ => false,
        });
        let mut meta = rf.meta();
        meta.counter += 1;
        rf.set_meta(&meta);
        kb.vault.mark_rule_dirty(&stem);
        // A refinement must keep the program valid.
        let rules: Vec<RuleInput> = kb.rules(true).iter().map(RuleInput::from).collect();
        sla_reasoner::check(&rules).map_err(|_| ())?;
        let what = match &change {
            Change::Condition(c) => format!("added condition {c}"),
            _ => "added exception for explicit negations".into(),
        };
        kb.log_change(now, "refine_rule", &format!("{stem}: {new_rule}"), &format!("{source} said {wrong} is wrong; {what}"));
        Ok(())
    });
    if refined.is_err() {
        // The refined rule would break the program: keep the rule as it was
        // and store only the exception fact, and say so.
        store_negation(kb);
        kb.log_change(now, "correction", &wrong.to_string(), &format!("refining {stem} would make the program invalid; stored the exception fact only"));
        return none(Change::RefinementRefused);
    }
    CorrectionOutcome { rule: Some(stem), change, new_rule: Some(new_rule) }
}

/// A demonstration: the state before, an action, and the state after.
#[derive(Debug, Clone, PartialEq)]
pub struct Demonstration {
    pub action: GroundAtom,
    pub before: Vec<GroundAtom>,
    pub after: Vec<GroundAtom>,
}

/// Turn a demonstration into (or merge it with) effect rules. Returns the
/// rule file stems created or updated.
pub fn learn_from_demo(kb: &mut KnowledgeBase, learner: &Learner, demo: &Demonstration, now: Timestamp) -> Vec<String> {
    // Constants of the action become variables A, B, C...
    let mut var_of: BTreeMap<Value, String> = BTreeMap::new();
    for (i, v) in demo.action.args.iter().enumerate() {
        var_of.entry(v.clone()).or_insert_with(|| ((b'A' + i as u8) as char).to_string());
    }
    let gen = |g: &GroundAtom| -> Atom {
        Atom { pred: g.pred.clone(), args: g.args.iter().map(|v| var_of.get(v).map_or(Term::Val(v.clone()), |n| Term::Var(n.clone()))).collect() }
    };
    let involves = |g: &GroundAtom| g.args.first().map_or(false, |v| var_of.contains_key(v));
    let before: BTreeSet<&GroundAtom> = demo.before.iter().collect();
    let after: BTreeSet<&GroundAtom> = demo.after.iter().collect();
    // Preconditions: facts about the participants that the action did not change.
    let pre: BTreeSet<Atom> = before.intersection(&after).filter(|g| involves(g)).map(|g| gen(g)).collect();
    let mut effects: Vec<Atom> = after.difference(&before).filter(|g| involves(g)).map(|g| gen(g)).collect();
    for removed in before.difference(&after).filter(|g| involves(g)) {
        let mut a = gen(removed);
        a.pred = sla_core::term::complement_pred(&a.pred);
        effects.push(a);
    }
    let action = gen(&demo.action);
    let mut touched = Vec::new();
    for head in effects {
        let id = format!("demo_{}_{}", demo.action.pred, head.pred.replace('-', "not_"));
        let stem = sla_core::vault::file_stem(&id);
        // Existing demo rule for the same action and effect: keep shared preconditions.
        let existing = kb.vault.rules.get(&stem).and_then(|rf| rf.rules().into_iter().find(|r| r.head == head));
        let (body_pre, evidence) = match &existing {
            Some(r) => {
                let old: BTreeSet<Atom> = r.body.iter().skip(1).filter_map(|l| if let Literal::Pos(a) = l { Some(a.clone()) } else { None }).collect();
                (old.intersection(&pre).cloned().collect::<Vec<_>>(), kb.vault.rules[&stem].meta().evidence + 1)
            }
            None => (pre.iter().cloned().collect(), 1),
        };
        let mut body = vec![Literal::Pos(action.clone())];
        body.extend(body_pre.into_iter().map(Literal::Pos));
        // Head variables must appear in the body (safety): skip effects about new things.
        let body_vars: BTreeSet<String> = body.iter().filter_map(|l| if let Literal::Pos(a) = l { Some(a.vars().map(String::from).collect::<Vec<_>>()) } else { None }).flatten().collect();
        if head.vars().any(|v| !body_vars.contains(v)) {
            continue;
        }
        let rule = Rule { head: head.clone(), body };
        let conf = learner.conf(evidence, 0);
        let meta = RuleMeta {
            id: id.clone(),
            status: if conf >= learner.promote_at { RuleStatus::Confirmed } else { RuleStatus::Hypothesis },
            conf,
            evidence,
            counter: 0,
            module: crate::module_for(&head.pred).into(),
            source: "demonstration".into(),
        };
        let note = format!("Learned from {evidence} demonstration(s) of {}.", demo.action.pred);
        kb.vault.put_rule_file(RuleFile::new(&meta, &note, &[rule.clone()]));
        kb.log_change(now, if existing.is_some() { "update_demo_rule" } else { "demo_rule" }, &format!("{id}: {rule}"), &note);
        touched.push(stem);
    }
    touched
}

/// Is this a rule shape the counting learner can rescore? (Used to keep refined rules scored.)
pub fn countable(r: &Rule) -> bool {
    rule_features(r).is_some()
}
