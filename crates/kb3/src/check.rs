//! The fact-and-relation check (schema section 6): the measure of record.
//!
//! The answer's procedure is compared with the gold procedure (base procedure with the
//! variant's difference applied). Steps are aligned by (action, object) in order, with
//! ground-truth aliases. No wording is scored.

use crate::fact::{unquote, Fact};
use crate::procedure::{Procedure, Step};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

/// Symbol equivalence from ground-truth `alias` facts: two symbols are the same if equal, or if
/// one's alias text, joined with underscores, is the other, or they share an alias text.
#[derive(Debug, Clone, Default)]
pub struct Aliases {
    forms: BTreeMap<String, BTreeSet<String>>,
}

fn join(text: &str) -> String {
    unquote(text)
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(|w| w.to_ascii_lowercase())
        .collect::<Vec<_>>()
        .join("_")
}

impl Aliases {
    pub fn from_facts(facts: &[Fact]) -> Aliases {
        let mut forms: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for f in facts.iter().filter(|f| f.pred == "alias") {
            forms.entry(f.arg(0).to_string()).or_default().insert(join(f.arg(1)));
        }
        Aliases { forms }
    }

    pub fn same(&self, a: &str, b: &str) -> bool {
        if a == b {
            return true;
        }
        let fa = self.forms.get(a);
        let fb = self.forms.get(b);
        fa.map(|s| s.contains(b)).unwrap_or(false)
            || fb.map(|s| s.contains(a)).unwrap_or(false)
            || matches!((fa, fb), (Some(x), Some(y)) if !x.is_disjoint(y))
    }

    fn same_opt(&self, a: &Option<String>, b: &Option<String>) -> bool {
        match (a, b) {
            (None, None) => true,
            (Some(x), Some(y)) => self.same(x, y),
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CheckResult {
    pub core_total: usize,
    pub core_found: usize,
    pub all_total: usize,
    pub all_found: usize,
    /// facts the answer states, and how many of them match gold
    pub answer_facts: usize,
    pub answer_matched: usize,
    pub extra_steps: usize,
    pub extra_needs: usize,
    pub wrong: Vec<String>,
    pub missing: Vec<String>,
    pub strict_pass: bool,
}

impl CheckResult {
    pub fn recall(&self) -> f64 {
        self.all_found as f64 / self.all_total.max(1) as f64
    }
    pub fn core_recall(&self) -> f64 {
        self.core_found as f64 / self.core_total.max(1) as f64
    }
    pub fn precision(&self) -> f64 {
        self.answer_matched as f64 / self.answer_facts.max(1) as f64
    }
}

/// Graded facts of one step, as comparable keys (excluding action and object).
fn step_details(s: &Step) -> Vec<(String, String)> {
    let mut v = Vec::new();
    for i in s.uses.keys() {
        v.push(("uses".into(), i.clone()));
    }
    for (p, i) in &s.places {
        v.push((p.clone(), i.clone()));
    }
    if let Some(u) = &s.until {
        v.push(("until".into(), u.clone()));
    }
    if let Some((lo, hi, u)) = &s.time {
        v.push(("time".into(), format!("{lo} {hi} {u}")));
    }
    for (a, val, u) in &s.settings {
        v.push((format!("setting {a}"), format!("{val} {u}")));
    }
    for (i, q, u) in &s.amounts {
        v.push((format!("amount {i}"), format!("{q} {u}")));
    }
    for (p, val) in &s.states {
        v.push((p.clone(), val.clone()));
    }
    for (a, i) in &s.avoid {
        v.push((format!("avoid {a}"), i.clone()));
    }
    v
}

/// Slots that take one value: a different value is a wrong claim, not just a miss.
fn functional(key: &str) -> bool {
    key == "time" || key == "until" || key.starts_with("setting ") || key.starts_with("amount ")
}

/// Compare an answer with the gold procedure. `lacks` are items the situation says the user
/// does not have: using one is a wrong claim.
pub fn check(answer: &Procedure, gold: &Procedure, aliases: &Aliases, lacks: &[String]) -> CheckResult {
    let mut r = CheckResult::default();
    // align steps in order by (action, object)
    let mut map: BTreeMap<usize, usize> = BTreeMap::new(); // answer index -> gold index
    let mut used = BTreeSet::new();
    for (ai, a) in answer.steps.iter().enumerate() {
        if let Some(gi) = gold
            .steps
            .iter()
            .enumerate()
            .position(|(gi, g)| !used.contains(&gi) && aliases.same(&a.action, &g.action) && aliases.same_opt(&a.object, &g.object))
        {
            used.insert(gi);
            map.insert(ai, gi);
        }
    }
    let gold_pos: BTreeMap<&str, usize> = gold.steps.iter().enumerate().map(|(k, s)| (s.id.as_str(), k)).collect();
    let rev: BTreeMap<usize, usize> = map.iter().map(|(a, g)| (*g, *a)).collect();

    // core: needs
    for n in &gold.needs {
        r.core_total += 1;
        r.all_total += 1;
        if answer.needs.iter().any(|a| aliases.same(a, n)) {
            r.core_found += 1;
            r.all_found += 1;
        } else {
            r.missing.push(format!("need {n}"));
        }
    }
    r.extra_needs = answer
        .needs
        .iter()
        .filter(|a| !gold.needs.iter().chain(gold.optional_needs.iter()).any(|n| aliases.same(a, n)))
        .count();
    // core: steps
    for (gi, g) in gold.steps.iter().enumerate() {
        r.core_total += 1;
        r.all_total += 1;
        if rev.contains_key(&gi) {
            r.core_found += 1;
            r.all_found += 1;
        } else {
            r.missing.push(format!("step {} {}", g.action, g.object.as_deref().unwrap_or("-")));
        }
    }
    r.extra_steps = answer.steps.len() - map.len();
    // core: order. Gold before(a, b) holds if both are aligned and a comes first in the answer.
    for (a, b) in &gold.before {
        let (Some(&ga), Some(&gb)) = (gold_pos.get(a.as_str()), gold_pos.get(b.as_str())) else { continue };
        r.core_total += 1;
        r.all_total += 1;
        match (rev.get(&ga), rev.get(&gb)) {
            (Some(x), Some(y)) if x < y => {
                r.core_found += 1;
                r.all_found += 1;
            }
            (Some(x), Some(y)) if x > y => r.wrong.push(format!("order: {a} after {b}")),
            _ => r.missing.push(format!("before {a} {b}")),
        }
    }
    // graded: step details
    for (gi, g) in gold.steps.iter().enumerate() {
        let a = rev.get(&gi).map(|ai| &answer.steps[*ai]);
        let ad = a.map(step_details).unwrap_or_default();
        for (k, v) in step_details(g) {
            r.all_total += 1;
            if ad.iter().any(|(ak, av)| *ak == k && (av == &v || aliases.same(av, &v))) {
                r.all_found += 1;
            } else if functional(&k) && ad.iter().any(|(ak, _)| *ak == k) {
                r.wrong.push(format!("{} {k}: answer differs from {v}", g.id));
            } else {
                r.missing.push(format!("{} {k} {v}", g.id));
            }
        }
    }
    // precision: answer facts that match a gold fact
    let gold_details: BTreeMap<usize, Vec<(String, String)>> = gold.steps.iter().enumerate().map(|(k, s)| (k, step_details(s))).collect();
    r.answer_facts += answer.needs.len() + answer.steps.len();
    r.answer_matched += answer.needs.len() - r.extra_needs + map.len();
    for (ai, s) in answer.steps.iter().enumerate() {
        for (k, v) in step_details(s) {
            r.answer_facts += 1;
            if let Some(gi) = map.get(&ai) {
                if gold_details[gi].iter().any(|(gk, gv)| *gk == k && (gv == &v || aliases.same(gv, &v))) {
                    r.answer_matched += 1;
                }
            }
        }
    }
    // wrong: using an item the user lacks
    for l in lacks {
        let in_needs = answer.needs.iter().any(|n| aliases.same(n, l));
        let in_steps = answer.steps.iter().any(|s| s.uses.keys().any(|i| aliases.same(i, l)) || s.object.as_deref().map(|o| aliases.same(o, l)).unwrap_or(false));
        let gold_keeps = gold.needs.iter().any(|n| aliases.same(n, l))
            || gold.steps.iter().any(|s| s.uses.keys().any(|i| aliases.same(i, l)) || s.object.as_deref().map(|o| aliases.same(o, l)).unwrap_or(false));
        if (in_needs || in_steps) && !gold_keeps {
            r.wrong.push(format!("uses lacked item {l}"));
        }
    }
    r.strict_pass = r.core_found == r.core_total && r.wrong.is_empty() && r.extra_steps == 0 && r.extra_needs == 0;
    r
}
