//! The hygiene pass (Design 2.0, "Knowledge hygiene"), run offline between
//! sessions.
//!
//! - **Duplicate entities** (`knive`/`knife`, `sunglass`/`sunglasses`,
//!   `ice_cream`/`icecream`) are linked with `same(a, b)` and queued as merge
//!   proposals (`vault/queue/merges.jsonl`). Nothing is rewritten in place, so
//!   every merge can be undone by closing the `same` fact.
//! - **Numbers to watch** are counted: predicates used once, facts not linked
//!   to any category or task, open review items.

use crate::Brain;
use serde::Serialize;
use sla_core::{Fact, GroundAtom, Timestamp, Value};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Default, Serialize)]
pub struct HygieneReport {
    pub entities: usize,
    /// Duplicate pairs not linked by `same` before the pass.
    pub duplicates_before: Vec<(String, String)>,
    /// Pairs linked by this pass.
    pub linked: usize,
    /// Duplicate pairs still unlinked after the pass (should be 0).
    pub duplicates_after: usize,
    pub predicates: usize,
    pub predicates_used_once: usize,
    /// Entities with no category (`is_a`) and no task link.
    pub unlinked_entities: usize,
}

/// A spelling key: singular, without separators.
fn key(s: &str) -> String {
    let w: String = s.chars().filter(|c| c.is_ascii_alphanumeric()).collect();
    let w = w.to_lowercase();
    if w.len() > 4 {
        if let Some(r) = w.strip_suffix("ies") {
            return format!("{r}y");
        }
        for e in ["ches", "shes", "sses", "xes", "ves"] {
            if w.ends_with(e) {
                // knives -> knif(e): compare with "knife" -> "knif"
                return if e == "ves" { format!("{}f", &w[..w.len() - 3]) } else { w[..w.len() - 2].to_string() };
            }
        }
        if w.ends_with('s') && !w.ends_with("ss") {
            return w[..w.len() - 1].to_string();
        }
    }
    // knife / knive -> knif
    if let Some(r) = w.strip_suffix("fe").or_else(|| w.strip_suffix("ve")) {
        return format!("{r}f");
    }
    w
}

fn entities(facts: &[Fact]) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for f in facts {
        if matches!(f.atom.base_pred(), "same" | "proper_name" | "takes_time" | "time_available") {
            continue;
        }
        for v in &f.atom.args {
            if let Value::Sym(s) = v {
                out.insert(s.to_string());
            }
        }
    }
    out
}

fn duplicate_pairs(facts: &[Fact]) -> Vec<(String, String)> {
    let linked: BTreeSet<(String, String)> = facts
        .iter()
        .filter(|f| f.atom.pred == "same")
        .filter_map(|f| Some((f.atom.args.first()?.as_sym()?.to_string(), f.atom.args.get(1)?.as_sym()?.to_string())))
        .flat_map(|(a, b)| [(a.clone(), b.clone()), (b, a)])
        .collect();
    let mut by_key: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for e in entities(facts) {
        by_key.entry(key(&e)).or_default().push(e);
    }
    let mut out = Vec::new();
    for group in by_key.values().filter(|g| g.len() > 1) {
        // Link each to the first (shortest, then alphabetical) name.
        let mut g = group.clone();
        g.sort_by_key(|s| (s.len(), s.clone()));
        for other in &g[1..] {
            if !linked.contains(&(g[0].clone(), other.clone())) {
                out.push((g[0].clone(), other.clone()));
            }
        }
    }
    out
}

/// Run the pass: link duplicates, count what to watch.
pub fn run(b: &mut Brain, now: Timestamp) -> HygieneReport {
    let facts: Vec<Fact> = b.kb.all_facts().into_iter().filter(|f| f.valid_at(now)).collect();
    let mut rep = HygieneReport { entities: entities(&facts).len(), duplicates_before: duplicate_pairs(&facts), ..Default::default() };
    let mut merges = Vec::new();
    for (a, other) in &rep.duplicates_before {
        let g = GroundAtom::new("same", vec![Value::sym(a.as_str()), Value::sym(other.as_str())]);
        b.kb.assert_fact(Fact::new(g, "hygiene"), now, "same spelling up to plural and separators");
        merges.push(serde_json::json!({"kind": "merge", "keep": a, "other": other, "status": "pending", "reason": "same spelling up to plural and separators"}));
        rep.linked += 1;
    }
    if !merges.is_empty() {
        let p = b.kb.vault.root.join("queue").join("merges.jsonl");
        let _ = std::fs::create_dir_all(p.parent().unwrap());
        let old = std::fs::read_to_string(&p).unwrap_or_default();
        let add: String = merges.iter().map(|m| format!("{m}\n")).collect();
        let _ = std::fs::write(&p, old + &add);
        b.kb.log_change(now, "hygiene", &format!("{} duplicate pairs linked", rep.linked), "hygiene pass");
    }
    let after: Vec<Fact> = b.kb.all_facts().into_iter().filter(|f| f.valid_at(now)).collect();
    rep.duplicates_after = duplicate_pairs(&after).len();
    let mut uses: BTreeMap<String, usize> = BTreeMap::new();
    for f in &after {
        *uses.entry(f.atom.base_pred().to_string()).or_default() += 1;
    }
    rep.predicates = uses.len();
    rep.predicates_used_once = uses.values().filter(|n| **n == 1).count();
    let linked_entities: BTreeSet<String> = after
        .iter()
        .filter(|f| matches!(f.atom.base_pred(), "is_a" | "needs" | "step" | "task" | "object_of" | "uses" | "same"))
        .flat_map(|f| f.atom.args.iter().filter_map(|v| v.as_sym().map(String::from)).collect::<Vec<_>>())
        .collect();
    rep.unlinked_entities = entities(&after).iter().filter(|e| !linked_entities.contains(*e)).count();
    let _ = b.save();
    rep
}

#[cfg(test)]
mod tests {
    use super::key;

    #[test]
    fn spelling_keys() {
        assert_eq!(key("knive"), key("knife"));
        assert_eq!(key("knives"), key("knife"));
        assert_eq!(key("sunglass"), key("sunglasses"));
        assert_eq!(key("ice_cream"), key("icecream"));
        assert_eq!(key("cherry"), key("cherries"));
        assert_eq!(key("box"), key("boxes"));
        assert_ne!(key("glass"), key("gas"));
        assert_ne!(key("tea"), key("team"));
    }
}
