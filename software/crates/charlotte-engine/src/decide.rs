//! Layer 3, first implementation: pick the pattern for the kind of request and run its rule list.

use crate::view::{features, selectors, view};
use charlotte_core::*;
use std::collections::BTreeMap;

pub struct RuleDecide;

pub fn lit_holds(lit: &str, feats: &BTreeMap<String, bool>) -> bool {
    match lit.strip_prefix("not ") {
        Some(f) => !feats.get(f.trim()).copied().unwrap_or(false),
        None => feats.get(lit.trim()).copied().unwrap_or(false),
    }
}
/// The first rule whose conditions all hold, as (index from 1, its class). Falls through to `otherwise`.
pub fn run_rules<'a>(rules: &'a [Rule], otherwise: &'a str, feats: &BTreeMap<String, bool>) -> (usize, &'a str) {
    for (i, r) in rules.iter().enumerate() {
        if r.when.iter().all(|l| lit_holds(l, feats)) {
            return (i + 1, &r.act);
        }
    }
    (rules.len() + 1, otherwise)
}
/// "ask(p_first_unbound)" with the selectors of the moment becomes the act `ask packaging`.
pub fn class_to_act(class: &str, sels: &BTreeMap<String, String>) -> Option<Act> {
    let (name, sel) = match class.split_once('(') {
        Some((n, rest)) => (n, Some(rest.trim_end_matches(')'))),
        None => (class, None),
    };
    let op = Op::parse(name.trim())?;
    match sel {
        None => Some(Act::new(op)),
        Some(s) => sels.get(s.trim()).map(|arg| Act::with(op, arg.clone())),
    }
}

impl Decide for RuleDecide {
    fn decide(&self, frame: &Frame, mem: &dyn Memory) -> Decision {
        let v = view(frame, mem);
        let mut d = Decision {
            kind: v.kind,
            pattern: None,
            atom: v.prim.map(|a| a.id.clone()),
            alternative: v.alt.map(|a| a.id.clone()),
            features: Vec::new(),
            rule: None,
            act: Act::new(Op::TeachMe),
            note: None,
        };
        match v.kind {
            ReqKind::Move => {
                d.act = Act::with(Op::Say, v.mv.map(|m| m.id.clone()).unwrap_or_default());
                return d;
            }
            ReqKind::Unknown => {
                d.note = Some(format!("nothing in the vault answers '{}'", frame.intent));
                return d;
            }
            _ => {}
        }
        let feats = features(frame, &v);
        d.features = feats.iter().filter(|(_, b)| **b).map(|(k, _)| k.clone()).collect();
        let Some(p) = mem.pattern(v.kind) else {
            d.note = Some(format!("no trusted pattern for {} requests", v.kind.name()));
            return d;
        };
        d.pattern = Some(p.id.clone());
        let (index, class) = run_rules(&p.rules, &p.otherwise, &feats);
        let when = p.rules.get(index - 1).map(|r| r.when.clone()).unwrap_or_else(|| vec!["otherwise".to_string()]);
        d.rule = Some(RuleTrace { index, when, act: class.to_string() });
        match class_to_act(class, &selectors(frame, &v)) {
            Some(act) => d.act = act,
            None => {
                d.note = Some(format!("rule {index} chose '{class}', which names nothing in this state"));
                d.act = Act::new(Op::Escalate);
            }
        }
        d
    }
}
