//! A procedure view over schema v3 facts, and applying difference facts (schema 3.6).

use crate::fact::Fact;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

/// Per-step relation predicates with one value argument: `pred(S, X)`.
pub const STATE_PREDS: &[&str] = &["purpose", "achieves", "requires_state", "condition", "warning", "check", "with_part", "repeat"];
/// Destination and source predicates: `pred(S, I)`.
pub const PLACE_PREDS: &[&str] = &["into", "onto", "from"];

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Step {
    pub id: String,
    pub action: String,
    pub object: Option<String>,
    /// item -> role
    pub uses: BTreeMap<String, Option<String>>,
    /// (into|onto|from, item)
    pub places: BTreeSet<(String, String)>,
    pub until: Option<String>,
    pub time: Option<(String, String, String)>,
    pub settings: BTreeSet<(String, String, String)>,
    pub amounts: BTreeSet<(String, String, String)>,
    /// (purpose|achieves|..., value)
    pub states: BTreeSet<(String, String)>,
    pub avoid: BTreeSet<(String, String)>,
    pub optional: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Procedure {
    pub task: String,
    pub needs: BTreeSet<String>,
    pub optional_needs: BTreeSet<String>,
    pub time: Option<(String, String, String)>,
    /// steps in procedure order
    pub steps: Vec<Step>,
    pub before: BTreeSet<(String, String)>,
    /// (step, anchor): step runs while anchor runs
    pub overlaps: BTreeSet<(String, String)>,
}

fn step_num(id: &str) -> u32 {
    id.rsplit("_s").next().and_then(|n| n.parse().ok()).unwrap_or(u32::MAX)
}

impl Procedure {
    pub fn from_facts(task: &str, facts: &[Fact]) -> Procedure {
        let mut p = Procedure { task: task.to_string(), ..Default::default() };
        let mut steps: BTreeMap<String, Step> = BTreeMap::new();
        for f in facts.iter().filter(|f| f.pred == "step" && f.arg(0) == task) {
            steps.insert(f.arg(1).to_string(), Step { id: f.arg(1).to_string(), ..Default::default() });
        }
        for f in facts {
            p.absorb(f, &mut steps);
        }
        p.set_steps(steps);
        p
    }

    fn absorb(&mut self, f: &Fact, steps: &mut BTreeMap<String, Step>) {
        let a = |i| f.arg(i).to_string();
        match f.pred.as_str() {
            "needs" if f.arg(0) == self.task => {
                self.needs.insert(a(1));
            }
            "optional_need" if f.arg(0) == self.task => {
                self.optional_needs.insert(a(1));
            }
            "takes_time" if f.arg(0) == self.task => self.time = Some((a(1), a(2), a(3))),
            "before" => {
                self.before.insert((a(0), a(1)));
            }
            "overlaps" => {
                self.overlaps.insert((a(0), a(1)));
            }
            _ => {
                let Some(s) = steps.get_mut(f.arg(0)) else { return };
                match f.pred.as_str() {
                    "action" => s.action = a(1),
                    "object_of" => s.object = Some(a(1)),
                    "uses" => {
                        s.uses.entry(a(1)).or_insert(None);
                    }
                    "role" => {
                        s.uses.insert(a(1), Some(a(2)));
                    }
                    "until" => s.until = Some(a(1)),
                    "takes_time" => s.time = Some((a(1), a(2), a(3))),
                    "setting" => {
                        s.settings.insert((a(1), a(2), a(3)));
                    }
                    "amount" => {
                        s.amounts.insert((a(1), a(2), a(3)));
                    }
                    "avoid" => {
                        s.avoid.insert((a(1), a(2)));
                    }
                    "optional" => s.optional = true,
                    p if PLACE_PREDS.contains(&p) => {
                        s.places.insert((p.to_string(), a(1)));
                    }
                    p if STATE_PREDS.contains(&p) => {
                        s.states.insert((p.to_string(), a(1)));
                    }
                    _ => {}
                }
            }
        }
    }

    /// Order steps: a step comes after every step it must follow (`before`) and after the step
    /// it overlaps; ties by step number.
    fn set_steps(&mut self, steps: BTreeMap<String, Step>) {
        let ids: Vec<String> = steps.keys().cloned().collect();
        let mut preds: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
        for (a, b) in &self.before {
            if steps.contains_key(a) && steps.contains_key(b) {
                preds.entry(b.as_str()).or_default().insert(a.as_str());
            }
        }
        for (s, anchor) in &self.overlaps {
            if steps.contains_key(s) && steps.contains_key(anchor) {
                preds.entry(s.as_str()).or_default().insert(anchor.as_str());
            }
        }
        let mut done: Vec<String> = Vec::new();
        let mut left: Vec<&String> = ids.iter().collect();
        left.sort_by_key(|id| step_num(id));
        while !left.is_empty() {
            let pick = left
                .iter()
                .position(|id| preds.get(id.as_str()).map(|ps| ps.iter().all(|p| done.iter().any(|d| d == p))).unwrap_or(true))
                .unwrap_or(0); // a cycle: take the lowest-numbered step
            done.push(left.remove(pick).clone());
        }
        let mut steps = steps;
        self.steps = done.into_iter().filter_map(|id| steps.remove(&id)).collect();
    }

    pub fn step(&self, id: &str) -> Option<&Step> {
        self.steps.iter().find(|s| s.id == id)
    }

    /// Back to facts (the inverse of `from_facts`, up to order).
    pub fn to_facts(&self) -> Vec<Fact> {
        let t = self.task.as_str();
        let mut out = vec![Fact::new("task", &[t])];
        for n in &self.needs {
            out.push(Fact::new("needs", &[t, n]));
        }
        for n in &self.optional_needs {
            out.push(Fact::new("optional_need", &[t, n]));
        }
        if let Some((lo, hi, u)) = &self.time {
            out.push(Fact::new("takes_time", &[t, lo, hi, u]));
        }
        for s in &self.steps {
            let id = s.id.as_str();
            out.push(Fact::new("step", &[t, id]));
            out.push(Fact::new("action", &[id, &s.action]));
            if let Some(o) = &s.object {
                out.push(Fact::new("object_of", &[id, o]));
            }
            for (i, r) in &s.uses {
                out.push(Fact::new("uses", &[id, i]));
                if let Some(r) = r {
                    out.push(Fact::new("role", &[id, i, r]));
                }
            }
            for (p, i) in &s.places {
                out.push(Fact::new(p, &[id, i]));
            }
            if let Some(u) = &s.until {
                out.push(Fact::new("until", &[id, u]));
            }
            if let Some((lo, hi, u)) = &s.time {
                out.push(Fact::new("takes_time", &[id, lo, hi, u]));
            }
            for (a, v, u) in &s.settings {
                out.push(Fact::new("setting", &[id, a, v, u]));
            }
            for (i, q, u) in &s.amounts {
                out.push(Fact::new("amount", &[id, i, q, u]));
            }
            for (p, v) in &s.states {
                out.push(Fact::new(p, &[id, v]));
            }
            for (a, i) in &s.avoid {
                out.push(Fact::new("avoid", &[id, a, i]));
            }
            if s.optional {
                out.push(Fact::new("optional", &[id]));
            }
        }
        for (a, b) in &self.before {
            out.push(Fact::new("before", &[a, b]));
        }
        for (a, b) in &self.overlaps {
            out.push(Fact::new("overlaps", &[a, b]));
        }
        out
    }

    /// Apply difference facts (swap, drop, add_step, change, extra_need, no_need) and the step
    /// facts of new steps. `cannot` leaves the procedure as it is (the answer must say so).
    pub fn apply(&self, diff: &[Fact]) -> Procedure {
        let mut facts = self.to_facts();
        let new_steps: BTreeSet<String> = diff.iter().filter(|f| f.pred == "add_step").map(|f| f.arg(0).to_string()).collect();
        for d in diff {
            let a = |i| d.arg(i).to_string();
            match d.pred.as_str() {
                "swap" => {
                    let (s, old, new) = (a(0), a(1), a(2));
                    for f in facts.iter_mut() {
                        let on_step = f.arg(0) == s;
                        if on_step && matches!(f.pred.as_str(), "uses" | "role" | "object_of") && f.arg(1) == old {
                            f.args[1] = new.clone();
                        }
                        if on_step && PLACE_PREDS.contains(&f.pred.as_str()) && f.arg(1) == old {
                            f.args[1] = new.clone();
                        }
                    }
                    let still_used = facts.iter().any(|f| matches!(f.pred.as_str(), "uses" | "object_of") && f.arg(1) == old);
                    if !still_used {
                        facts.retain(|f| !(f.pred == "needs" && f.arg(1) == old));
                    }
                    if !facts.iter().any(|f| f.pred == "needs" && f.arg(1) == new) {
                        facts.push(Fact::new("needs", &[&self.task, &new]));
                    }
                }
                "drop" => {
                    let s = a(0);
                    // keep the order through the dropped step: a -> s -> b becomes a -> b
                    let ins: Vec<String> = self.before.iter().filter(|(_, b)| *b == s).map(|(x, _)| x.clone()).collect();
                    let outs: Vec<String> = self.before.iter().filter(|(x, _)| *x == s).map(|(_, b)| b.clone()).collect();
                    facts.retain(|f| f.arg(0) != s && !(f.pred == "step" && f.arg(1) == s) && !(matches!(f.pred.as_str(), "before" | "overlaps") && f.arg(1) == s));
                    for x in &ins {
                        for y in &outs {
                            facts.push(Fact::new("before", &[x, y]));
                        }
                    }
                }
                "change" => {
                    let (s, pred, new) = (a(0), a(1), a(3));
                    let parts: Vec<&str> = new.split('_').collect();
                    for f in facts.iter_mut().filter(|f| f.arg(0) == s && f.pred == pred) {
                        match pred.as_str() {
                            "takes_time" if parts.len() == 2 => {
                                f.args[1] = parts[0].to_string();
                                f.args[2] = parts[1].to_string();
                            }
                            "takes_time" | "setting" | "amount" => {}
                            _ if f.args.len() == 2 => f.args[1] = new.clone(),
                            _ => {}
                        }
                    }
                    if !facts.iter().any(|f| f.arg(0) == s && f.pred == pred) && arity_two(&pred) {
                        facts.push(Fact::new(&pred, &[&s, &new]));
                    }
                }
                "extra_need" => facts.push(Fact::new("needs", &[&self.task, &a(0)])),
                "no_need" => facts.retain(|f| !(f.pred == "needs" && f.arg(1) == a(0))),
                "add_step" | "cannot" => {}
                "step" | "before" | "overlaps" => facts.push(d.clone()),
                _ if new_steps.contains(d.arg(0)) => facts.push(d.clone()),
                _ => {}
            }
        }
        // place a new step after its anchor when no order fact says otherwise
        for d in diff.iter().filter(|d| d.pred == "add_step") {
            let (s, after) = (d.arg(0), d.arg(1));
            if !facts.iter().any(|f| f.pred == "step" && f.arg(1) == s) {
                facts.push(Fact::new("step", &[&self.task, s]));
            }
            let ordered = facts.iter().any(|f| matches!(f.pred.as_str(), "before" | "overlaps") && (f.arg(0) == s || f.arg(1) == s));
            if after != "start" && !ordered {
                facts.push(Fact::new("before", &[after, s]));
            }
        }
        facts.sort();
        facts.dedup();
        Procedure::from_facts(&self.task, &facts)
    }
}

fn arity_two(pred: &str) -> bool {
    crate::fact::arity(pred) == Some(2)
}
