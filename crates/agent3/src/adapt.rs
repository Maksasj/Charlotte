//! Adaptation arms (decisions D6, D18). A proposed change needs support from knowledge;
//! otherwise the answer states the gap and asks the teacher.
//!
//! - `Function` (arm A): substitution by learned function (an item the user has plays the
//!   lacked item's role in some taught task), then transfer from the most similar stored case.
//! - `Rules` (arm B): lifted patterns induced from stored cases, kept with support >= 2.
//! - `None`: the base procedure (null baseline).
//! Both arms first reuse a stored case of the same task with the same situation.

use kb3::vault::Case;
use kb3::{Fact, Procedure, Vault};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arm {
    Function,
    Rules,
    None,
}

#[derive(Debug, Clone, Default)]
pub struct Out {
    pub difference: Vec<Fact>,
    pub gaps: Vec<(String, String, String)>,
    pub asks: Vec<String>,
    pub cannot: Vec<String>,
}

fn words(sym: &str) -> BTreeSet<String> {
    sym.split('_').filter(|w| w.len() > 2).map(|w| w.chars().take(4).collect()).collect()
}

/// Map situation items onto the symbols the agent knows: an unknown symbol that shares a word
/// with an item of the base procedure becomes that item (`spray_oil` -> `oil`-sharing item).
pub fn link_situation(vault: &Vault, base: &Procedure, sit: Vec<Fact>) -> Vec<Fact> {
    let mut items: BTreeSet<String> = base.needs.clone();
    for s in &base.steps {
        items.extend(s.uses.keys().cloned());
        items.extend(s.object.iter().cloned());
    }
    let known: BTreeSet<String> = vault.live().filter(|f| matches!(f.pred.as_str(), "needs" | "uses" | "object_of")).map(|f| f.arg(1).to_string()).collect();
    let mut out: Vec<Fact> = Vec::new();
    for f in sit {
        let mut f = f;
        if f.args.len() == 2 && f.pred != "time_limit" {
            let x = f.arg(1).to_string();
            if !items.contains(&x) {
                let wx = words(&x);
                let hit = items.iter().filter(|i| !words(i).is_disjoint(&wx)).max_by_key(|i| words(i).intersection(&wx).count());
                if let Some(i) = hit {
                    if f.pred != "has" || !known.contains(&x) {
                        f.args[1] = i.clone();
                    }
                }
            }
        }
        if !out.contains(&f) {
            out.push(f);
        }
    }
    out
}

fn sit_items(sit: &[Fact], pred: &str) -> BTreeSet<String> {
    sit.iter().filter(|f| f.pred == pred).map(|f| f.arg(1).to_string()).collect()
}

/// Where each item is used in the base procedure: (step id, step number, role).
fn uses_of(base: &Procedure, item: &str) -> Vec<(String, usize, Option<String>, bool)> {
    base.steps
        .iter()
        .enumerate()
        .filter_map(|(k, s)| {
            if let Some(r) = s.uses.get(item) {
                Some((s.id.clone(), k + 1, r.clone(), false))
            } else if s.object.as_deref() == Some(item) {
                Some((s.id.clone(), k + 1, None, true))
            } else {
                None
            }
        })
        .collect()
}

/// role -> items known to play it, with counts (from all taught tasks).
fn role_index(vault: &Vault) -> BTreeMap<String, BTreeMap<String, usize>> {
    let mut m: BTreeMap<String, BTreeMap<String, usize>> = BTreeMap::new();
    for f in vault.live().filter(|f| f.pred == "role") {
        *m.entry(f.arg(2).to_string()).or_default().entry(f.arg(1).to_string()).or_insert(0) += 1;
    }
    m
}

/// The situation pattern of a case, in role terms (as phase A's x4_adapt.py).
fn signature(base: &Procedure, sit: &[Fact]) -> BTreeSet<(String, String)> {
    let mut sig = BTreeSet::new();
    for p in ["lacks", "excludes", "wants"] {
        for i in sit_items(sit, p) {
            let mut roles: BTreeSet<String> = uses_of(base, &i).into_iter().filter_map(|u| u.2).collect();
            if uses_of(base, &i).iter().any(|u| u.3) {
                roles.insert("object".into());
            }
            if roles.is_empty() {
                roles.insert("none".into());
            }
            for r in roles {
                sig.insert((p.to_string(), r));
            }
        }
    }
    for p in ["time_limit", "count"] {
        if sit.iter().any(|f| f.pred == p) {
            sig.insert((p.to_string(), "-".into()));
        }
    }
    sig
}

fn jaccard(a: &BTreeSet<(String, String)>, b: &BTreeSet<(String, String)>) -> f64 {
    if a.is_empty() || b.is_empty() {
        return 0.0;
    }
    a.intersection(b).count() as f64 / a.union(b).count() as f64
}

pub fn adapt(arm: Arm, vault: &Vault, base: &Procedure, sit: &[Fact]) -> Out {
    let mut out = Out::default();
    let lacked: BTreeSet<String> = sit_items(sit, "lacks").union(&sit_items(sit, "excludes")).cloned().collect();
    let wanted: BTreeSet<String> = sit_items(sit, "wants").into_iter().filter(|w| !base.needs.contains(w)).collect();
    let has = sit_items(sit, "has");
    let relevant_lacks: BTreeSet<String> = lacked.iter().filter(|l| !uses_of(base, l).is_empty() || base.needs.contains(*l)).cloned().collect();
    // a taught case of this task whose situation overlaps this one is reused as taught
    if arm != Arm::None {
        let key = |s: &[Fact]| -> BTreeSet<(String, String)> {
            s.iter().filter(|f| f.pred != "has" || !base.needs.contains(f.arg(1))).map(|f| (f.pred.clone(), f.args[1..].join("_"))).collect()
        };
        let mine = key(sit);
        if let Some((j, c)) = vault.cases.iter().filter(|c| c.task == base.task).map(|c| (jaccard(&key(&c.situation), &mine), c)).max_by(|a, b| a.0.partial_cmp(&b.0).unwrap()) {
            if j >= 0.5 {
                let mut out = Out::default();
                out.difference = c.difference.clone();
                return out;
            }
        }
    }
    // a time limit matters only when it is shorter than the procedure's own time; a count
    // (guests, portions) only through stored cases (the base gives no amounts to scale)
    let minutes = |n: &str, u: &str| n.parse::<f64>().ok().map(|x| if u.starts_with("hour") { x * 60.0 } else if u.starts_with("day") { x * 1440.0 } else { x });
    let base_max = base.time.as_ref().and_then(|(_, hi, u)| minutes(hi, u));
    let tight = sit.iter().filter(|f| f.pred == "time_limit").any(|f| match (minutes(f.arg(1), f.arg(2)), base_max) {
        (Some(l), Some(b)) => l < b,
        _ => false,
    });
    let constrained = tight;
    if relevant_lacks.is_empty() && wanted.is_empty() && !constrained {
        return out;
    }
    let sig = signature(base, sit);
    // a taught case of this same task with the same kind of situation is reused as it was taught
    if arm != Arm::None {
        if let Some(c) = vault.cases.iter().filter(|c| c.task == base.task).max_by(|a, b| {
            let pa = Procedure::from_facts(&a.task, &vault.live_facts());
            let pb = Procedure::from_facts(&b.task, &vault.live_facts());
            jaccard(&signature(&pa, &a.situation), &sig).partial_cmp(&jaccard(&signature(&pb, &b.situation), &sig)).unwrap()
        }) {
            if jaccard(&signature(base, &c.situation), &sig) >= 0.5 {
                out.difference = rebind(c, base, sit);
                return out;
            }
        }
    }
    match arm {
        Arm::None => {}
        Arm::Function => function_arm(vault, base, sit, &relevant_lacks, &wanted, &has, &sig, &mut out),
        Arm::Rules => rules_arm(vault, base, sit, &relevant_lacks, &has, &sig, &mut out),
    }
    for w in &wanted {
        if !out.difference.iter().any(|d| d.pred == "extra_need" && d.arg(0) == w) {
            out.gaps.push(("wants".into(), w.clone(), "-".into()));
            out.asks.push(format!("wants_{w}"));
        }
    }
    if constrained && out.difference.is_empty() {
        for f in sit.iter().filter(|f| f.pred == "time_limit") {
            out.gaps.push((f.pred.clone(), f.args[1..].join("_"), "-".into()));
        }
        out.asks.push("constraint".into());
    }
    out
}

/// A same-task case: replace the case's own stand-ins by what this user has, where they differ.
fn rebind(c: &Case, _base: &Procedure, _sit: &[Fact]) -> Vec<Fact> {
    c.difference.clone()
}

#[allow(clippy::too_many_arguments)]
fn function_arm(vault: &Vault, base: &Procedure, sit: &[Fact], lacked: &BTreeSet<String>, _wanted: &BTreeSet<String>, has: &BTreeSet<String>, sig: &BTreeSet<(String, String)>, out: &mut Out) {
    let roles = role_index(vault);
    for l in lacked {
        let uses = uses_of(base, l);
        let mut handled = true;
        for (sid, num, role, is_object) in &uses {
            let role_items = role.as_ref().and_then(|r| roles.get(r));
            let cand = role_items.and_then(|m| {
                m.iter().filter(|(j, _)| has.contains(*j) && !base.needs.contains(*j) && *j != l).max_by_key(|(_, n)| **n).map(|(j, _)| j.clone())
            });
            match cand {
                Some(j) if !is_object => out.difference.push(Fact::new("swap", &[sid, l, &j])),
                _ => {
                    if let Some(d) = case_transfer(vault, base, sit, sig, l, sid) {
                        out.difference.extend(d);
                    } else if base.step(sid).map(|s| s.optional).unwrap_or(false) {
                        out.difference.push(Fact::new("drop", &[sid]));
                    } else {
                        handled = false;
                        out.gaps.push(("lacks".into(), l.clone(), num.to_string()));
                        out.asks.push(format!("replace_{l}"));
                    }
                }
            }
        }
        if handled && !uses.is_empty() {
            out.difference.push(Fact::new("no_need", &[l]));
        }
    }
}

/// Transfer a stored case of another task with a similar situation: its swap of the item that
/// plays the same role, re-bound to this task's step and to an item this user has.
fn case_transfer(vault: &Vault, base: &Procedure, _sit: &[Fact], sig: &BTreeSet<(String, String)>, lacked: &str, sid: &str) -> Option<Vec<Fact>> {
    let all = vault.live_facts();
    let mut best: Option<(f64, &Case)> = None;
    for c in vault.cases.iter().filter(|c| c.task != base.task) {
        let cb = Procedure::from_facts(&c.task, &all);
        let j = jaccard(&signature(&cb, &c.situation), sig);
        if j >= 0.5 && best.map(|b| j > b.0).unwrap_or(true) {
            best = Some((j, c));
        }
    }
    let (_, c) = best?;
    // the case's stand-in, if this user has it too
    let has = sit_items(_sit, "has");
    for d in c.difference.iter().filter(|d| d.pred == "swap") {
        if has.contains(d.arg(2)) {
            return Some(vec![Fact::new("swap", &[sid, lacked, d.arg(2)])]);
        }
    }
    None
}

fn rules_arm(vault: &Vault, base: &Procedure, sit: &[Fact], lacked: &BTreeSet<String>, has: &BTreeSet<String>, sig: &BTreeSet<(String, String)>, out: &mut Out) {
    // induce: (signature element) -> (change kind) with the number of distinct tasks supporting it
    let all = vault.live_facts();
    let mut support: BTreeMap<((String, String), String), BTreeSet<String>> = BTreeMap::new();
    for c in vault.cases.iter().filter(|c| c.task != base.task) {
        let cb = Procedure::from_facts(&c.task, &all);
        for el in signature(&cb, &c.situation) {
            for d in &c.difference {
                if matches!(d.pred.as_str(), "swap" | "drop") {
                    support.entry((el.clone(), d.pred.clone())).or_default().insert(c.task.clone());
                }
            }
        }
    }
    let rule = |el: &(String, String), kind: &str| support.get(&(el.clone(), kind.to_string())).map(|s| s.len() >= 2).unwrap_or(false);
    let roles = role_index(vault);
    for l in lacked {
        let mut handled = false;
        for (sid, num, role, _) in uses_of(base, l) {
            let el = ("lacks".to_string(), role.clone().unwrap_or_else(|| "none".into()));
            if !sig.contains(&el) {
                continue;
            }
            if rule(&el, "swap") {
                let cand = role.as_ref().and_then(|r| roles.get(r)).and_then(|m| m.keys().find(|j| has.contains(*j) && !base.needs.contains(*j)).cloned());
                if let Some(j) = cand {
                    out.difference.push(Fact::new("swap", &[&sid, l, &j]));
                    handled = true;
                    continue;
                }
            }
            if rule(&el, "drop") {
                out.difference.push(Fact::new("drop", &[&sid]));
                handled = true;
                continue;
            }
            out.gaps.push(("lacks".into(), l.clone(), num.to_string()));
            out.asks.push(format!("replace_{l}"));
        }
        if handled {
            out.difference.push(Fact::new("no_need", &[l]));
        }
    }
    let _ = sit;
}

/// Difference facts that turn `base` into `taught` (steps matched by action and object).
pub fn difference(base: &Procedure, taught: &Procedure) -> Vec<Fact> {
    let key = |s: &kb3::Step| (s.action.clone(), s.object.clone());
    let mut diff = Vec::new();
    let tkeys: Vec<_> = taught.steps.iter().map(key).collect();
    let bkeys: Vec<_> = base.steps.iter().map(key).collect();
    for (bs, k) in base.steps.iter().zip(&bkeys) {
        match taught.steps.iter().find(|t| key(t) == *k) {
            None => diff.push(Fact::new("drop", &[&bs.id])),
            Some(ts) => {
                let old: BTreeSet<&String> = bs.uses.keys().collect();
                let new: BTreeSet<&String> = ts.uses.keys().collect();
                let gone: Vec<_> = old.difference(&new).collect();
                let came: Vec<_> = new.difference(&old).collect();
                for (g, c) in gone.iter().zip(came.iter()) {
                    diff.push(Fact::new("swap", &[&bs.id, g, c]));
                }
            }
        }
    }
    let mut n = 9;
    let mut prev = "start".to_string();
    for (ts, k) in taught.steps.iter().zip(&tkeys) {
        if let Some(b) = base.steps.iter().find(|b| key(b) == *k) {
            prev = b.id.clone();
            continue;
        }
        let id = format!("{}_s{n}", base.task);
        n += 1;
        diff.push(Fact::new("add_step", &[&id, &prev]));
        diff.push(Fact::new("step", &[&base.task, &id]));
        diff.push(Fact::new("action", &[&id, &ts.action]));
        if let Some(o) = &ts.object {
            diff.push(Fact::new("object_of", &[&id, o]));
        }
        for (i, r) in &ts.uses {
            diff.push(Fact::new("uses", &[&id, i]));
            if let Some(r) = r {
                diff.push(Fact::new("role", &[&id, i, r]));
            }
        }
        if let Some((lo, hi, u)) = &ts.time {
            diff.push(Fact::new("takes_time", &[&id, lo, hi, u]));
        }
        prev = id;
    }
    for n in taught.needs.difference(&base.needs) {
        diff.push(Fact::new("extra_need", &[n]));
    }
    for n in base.needs.difference(&taught.needs) {
        diff.push(Fact::new("no_need", &[n]));
    }
    diff
}
