//! Relation-line answers (decision D5, schema section 5).
//!
//! One line is one fact: `predicate arg arg ...`, with step numbers local to the answer.
//! A step's action and object are written `step N do ACTION OBJECT`, so an action named like a
//! line keyword (`check`) cannot be misread.
//! These keywords are the answer's formal language, not English text.

use crate::procedure::{Procedure, Step, PLACE_PREDS, STATE_PREDS};
use std::collections::BTreeMap;

/// A parsed answer: the procedure it states, plus gap and question lines.
#[derive(Debug, Clone, Default)]
pub struct Answer {
    pub procedure: Procedure,
    /// (kind, item, step number or "-"), e.g. ("lacks", "whisk", "2")
    pub gaps: Vec<(String, String, String)>,
    pub asks: Vec<String>,
    pub cannot: Vec<String>,
}

fn sid(task: &str, n: usize) -> String {
    format!("{task}_s{n}")
}

/// Print a procedure as relation lines; steps are numbered 1..n in procedure order.
pub fn print(p: &Procedure, gaps: &[(String, String, String)], asks: &[String], cannot: &[String]) -> String {
    let mut out = vec![format!("task {}", p.task)];
    for n in &p.needs {
        out.push(format!("need {n}"));
    }
    for n in &p.optional_needs {
        out.push(format!("optional_need {n}"));
    }
    if let Some((lo, hi, u)) = &p.time {
        out.push(format!("time {lo} {hi} {u}"));
    }
    let num: BTreeMap<&str, usize> = p.steps.iter().enumerate().map(|(k, s)| (s.id.as_str(), k + 1)).collect();
    for (k, s) in p.steps.iter().enumerate() {
        let n = k + 1;
        out.push(format!("step {n} do {} {}", s.action, s.object.as_deref().unwrap_or("-")));
        for (i, r) in &s.uses {
            out.push(format!("step {n} uses {i} {}", r.as_deref().unwrap_or("-")));
        }
        for (pl, i) in &s.places {
            out.push(format!("step {n} {pl} {i}"));
        }
        if let Some(u) = &s.until {
            out.push(format!("step {n} until {u}"));
        }
        if let Some((lo, hi, u)) = &s.time {
            out.push(format!("step {n} time {lo} {hi} {u}"));
        }
        for (a, v, u) in &s.settings {
            out.push(format!("step {n} setting {a} {v} {u}"));
        }
        for (i, q, u) in &s.amounts {
            out.push(format!("step {n} amount {i} {q} {u}"));
        }
        for (pr, v) in &s.states {
            out.push(format!("step {n} {pr} {v}"));
        }
        for (a, i) in &s.avoid {
            out.push(format!("step {n} avoid {a} {i}"));
        }
        if s.optional {
            out.push(format!("step {n} optional"));
        }
    }
    for (a, b) in &p.overlaps {
        if let (Some(x), Some(y)) = (num.get(a.as_str()), num.get(b.as_str())) {
            out.push(format!("step {x} overlaps {y}"));
        }
    }
    for (a, b) in &p.before {
        if let (Some(x), Some(y)) = (num.get(a.as_str()), num.get(b.as_str())) {
            out.push(format!("before {x} {y}"));
        }
    }
    for (k, i, s) in gaps {
        out.push(format!("gap {k} {i} {s}"));
    }
    for c in cannot {
        out.push(format!("cannot {c}"));
    }
    for a in asks {
        out.push(format!("ask {a}"));
    }
    out.join("\n")
}

/// Parse relation lines. Unknown lines are ignored (they are never scored).
pub fn parse(text: &str) -> Answer {
    let mut ans = Answer::default();
    let mut steps: BTreeMap<usize, Step> = BTreeMap::new();
    let mut task = String::new();
    for line in text.lines() {
        let w: Vec<&str> = line.split_whitespace().collect();
        match w.as_slice() {
            ["task", t] => task = t.to_string(),
            ["need", i] => {
                ans.procedure.needs.insert(i.to_string());
            }
            ["optional_need", i] => {
                ans.procedure.optional_needs.insert(i.to_string());
            }
            ["time", lo, hi, u] => ans.procedure.time = Some((lo.to_string(), hi.to_string(), u.to_string())),
            ["before", a, b] => {
                if let (Ok(a), Ok(b)) = (a.parse::<usize>(), b.parse::<usize>()) {
                    ans.procedure.before.insert((a.to_string(), b.to_string()));
                }
            }
            ["gap", k, i, s] => ans.gaps.push((k.to_string(), i.to_string(), s.to_string())),
            ["cannot", c] => ans.cannot.push(c.to_string()),
            ["ask", a] => ans.asks.push(a.to_string()),
            ["step", n, rest @ ..] => {
                let Ok(n) = n.parse::<usize>() else { continue };
                let s = steps.entry(n).or_default();
                match rest {
                    ["uses", i, r] => {
                        s.uses.insert(i.to_string(), if *r == "-" { None } else { Some(r.to_string()) });
                    }
                    ["uses", i] => {
                        s.uses.insert(i.to_string(), None);
                    }
                    ["until", u] => s.until = Some(u.to_string()),
                    ["time", lo, hi, u] => s.time = Some((lo.to_string(), hi.to_string(), u.to_string())),
                    ["setting", a, v, u] => {
                        s.settings.insert((a.to_string(), v.to_string(), u.to_string()));
                    }
                    ["amount", i, q, u] => {
                        s.amounts.insert((i.to_string(), q.to_string(), u.to_string()));
                    }
                    ["avoid", a, i] => {
                        s.avoid.insert((a.to_string(), i.to_string()));
                    }
                    ["optional"] => s.optional = true,
                    ["overlaps", m] => {
                        ans.procedure.overlaps.insert((n.to_string(), m.to_string()));
                    }
                    [p, v] if PLACE_PREDS.contains(p) => {
                        s.places.insert((p.to_string(), v.to_string()));
                    }
                    [p, v] if STATE_PREDS.contains(p) => {
                        s.states.insert((p.to_string(), v.to_string()));
                    }
                    ["do", action, object] => {
                        s.action = action.to_string();
                        s.object = if *object == "-" { None } else { Some(object.to_string()) };
                    }
                    ["do", action] => s.action = action.to_string(),
                    _ => {}
                }
            }
            _ => {}
        }
    }
    ans.procedure.task = task.clone();
    // step numbers become local ids `<task>_s<n>`, in numeric order
    let rename = |n: &str| n.parse::<usize>().map(|n| sid(&task, n)).unwrap_or_else(|_| n.to_string());
    ans.procedure.before = ans.procedure.before.iter().map(|(a, b)| (rename(a), rename(b))).collect();
    ans.procedure.overlaps = ans.procedure.overlaps.iter().map(|(a, b)| (rename(a), rename(b))).collect();
    ans.procedure.steps = steps
        .into_iter()
        .map(|(n, mut s)| {
            s.id = sid(&task, n);
            s
        })
        .collect();
    ans
}
