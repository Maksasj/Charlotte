//! Charlotte 4.0 reasoning core (Design 4.0 sections 4.3-4.4, 7.4): the base rules B1-B9 as
//! data on `sla_reasoner`, the encoding of taught relations into the reified form the
//! exception program reads, evaluation with any base rule switched off, and proofs.

pub mod proof;

use sla_core::term::{GroundAtom, Value};
use sla_reasoner::{evaluate, FactInput, Model, RuleInput};
use std::collections::BTreeMap;

pub const CORE: &str = include_str!("../core/base.dl");

/// Relations that inherit down `is_a` (Design 4.0 table 4.1), binary and ternary.
pub const INHERITABLE: &[&str] = &["has_part", "made_of", "has_property", "can_do", "used_for", "can_go_in"];
pub const INHERITABLE3: &[&str] = &["can_do_on"];

/// The base rules by section id (B1 ... B9), each rule with id `core/Bk/n`.
pub fn core_rules() -> BTreeMap<String, Vec<RuleInput>> {
    let mut out: BTreeMap<String, Vec<RuleInput>> = BTreeMap::new();
    let mut sec = String::new();
    let mut buf = String::new();
    let flush = |sec: &str, buf: &mut String, out: &mut BTreeMap<String, Vec<RuleInput>>| {
        if sec.is_empty() || buf.trim().is_empty() {
            buf.clear();
            return;
        }
        let (_, rules) = sla_reasoner::parse_program(buf).expect("core rules parse");
        let v = out.entry(sec.to_string()).or_default();
        for (k, mut r) in rules.into_iter().enumerate() {
            r.id = format!("core/{sec}/{}", k + 1);
            v.push(r);
        }
        buf.clear();
    };
    for line in CORE.lines() {
        if let Some(rest) = line.strip_prefix("%% ") {
            flush(&sec, &mut buf, &mut out);
            sec = rest.split_whitespace().next().unwrap_or("").to_string();
        } else {
            buf.push_str(line);
            buf.push('\n');
        }
    }
    flush(&sec, &mut buf, &mut out);
    out
}

fn sym(s: &str) -> Value {
    Value::Sym(s.to_string())
}

/// A taught surface fact (or its negation) in the form the core program reads.
pub fn encode(atom: &GroundAtom, negated: bool) -> GroundAtom {
    let p = atom.pred.as_str();
    if INHERITABLE.contains(&p) && atom.args.len() == 2 {
        let mut args = vec![atom.args[0].clone(), sym(p), atom.args[1].clone()];
        return GroundAtom::new(if negated { "neg" } else { "pos" }, std::mem::take(&mut args));
    }
    if INHERITABLE3.contains(&p) && atom.args.len() == 3 {
        let args = vec![atom.args[0].clone(), sym(p), atom.args[1].clone(), atom.args[2].clone()];
        return GroundAtom::new(if negated { "neg3" } else { "pos3" }, args);
    }
    if negated {
        return GroundAtom::new(&format!("not_{p}"), atom.args.clone());
    }
    atom.clone()
}

/// Facts the core needs besides the taught ones: the inheritable markers.
pub fn markers() -> Vec<GroundAtom> {
    INHERITABLE.iter().map(|r| GroundAtom::new("inheritable", vec![sym(r)])).collect()
}

/// The internal atom that says a surface claim holds (or does not hold, when `negated`).
pub fn holds_atom(atom: &GroundAtom, negated: bool) -> GroundAtom {
    let p = atom.pred.as_str();
    if INHERITABLE.contains(&p) && atom.args.len() == 2 {
        return GroundAtom::new(if negated { "holdsn" } else { "holds" }, vec![atom.args[0].clone(), sym(p), atom.args[1].clone()]);
    }
    if INHERITABLE3.contains(&p) && atom.args.len() == 3 {
        return GroundAtom::new(if negated { "holdsn3" } else { "holds3" }, vec![atom.args[0].clone(), sym(p), atom.args[1].clone(), atom.args[2].clone()]);
    }
    match (p, negated) {
        ("is_a", false) => GroundAtom::new("isa", atom.args.clone()),
        ("effect", false) => GroundAtom::new("holds_effect", atom.args.clone()),
        ("precondition", false) => GroundAtom::new("holds_pre", atom.args.clone()),
        ("has_part", false) => GroundAtom::new("part", atom.args.clone()),
        _ => encode(atom, negated),
    }
}

/// Back from an internal `holds`/`holdsn` atom to the surface relation.
pub fn surface(a: &GroundAtom) -> Option<(GroundAtom, bool)> {
    let name = |v: &Value| if let Value::Sym(s) = v { s.clone() } else { v.to_string() };
    match a.pred.as_str() {
        "holds" | "holdsn" | "pos" | "neg" if a.args.len() == 3 => {
            Some((GroundAtom::new(&name(&a.args[1]), vec![a.args[0].clone(), a.args[2].clone()]), a.pred == "holdsn" || a.pred == "neg"))
        }
        "holds3" | "holdsn3" | "pos3" | "neg3" if a.args.len() == 4 => Some((
            GroundAtom::new(&name(&a.args[1]), vec![a.args[0].clone(), a.args[2].clone(), a.args[3].clone()]),
            a.pred.ends_with('n') || a.pred == "neg3",
        )),
        "isa" => Some((GroundAtom::new("is_a", a.args.clone()), false)),
        _ => None,
    }
}

/// Evaluate taught facts (already encoded) with the core, some base rules switched off,
/// plus extra (learned) rules.
pub fn run(facts: &[FactInput], off: &[&str], extra: &[RuleInput]) -> Result<Model, String> {
    let mut rules: Vec<RuleInput> = Vec::new();
    for (sec, rs) in core_rules() {
        if !off.contains(&sec.as_str()) {
            rules.extend(rs);
        }
    }
    rules.extend(extra.iter().cloned());
    let mut all: Vec<FactInput> = facts.to_vec();
    for m in markers() {
        all.push(FactInput { atom: m, conf: 1.0, source: "core".into() });
    }
    evaluate(&all, &rules).map_err(|e| e.to_string())
}

/// Parse a ground atom written `pred(a, b)`.
pub fn parse_atom(s: &str) -> Result<GroundAtom, String> {
    let s = s.trim().trim_end_matches('.');
    let r = sla_core::parse_rule(&format!("{s}.")).map_err(|e| format!("{s}: {e}"))?;
    r.head.to_ground().ok_or_else(|| format!("{s}: not ground"))
}
