//! Proof records (Design 4.0 section 7.4): the engine's proof tree as JSON, with hop counts.

use serde_json::{json, Value as J};
use sla_reasoner::{Model, Proof, ProofStep};
use sla_core::term::GroundAtom;

/// Rule-application hops: inheritance down n is_a links counts n (the isa chain), each
/// B5/B6 role bridge/B7/B8 and learned rule counts 1; helper clauses count 0.
pub fn hops(p: &Proof) -> usize {
    let own = match &p.step {
        ProofStep::Given { .. } => 0,
        ProofStep::Rule { id, .. } => {
            let counted = id.starts_with("core/B1/") || id.starts_with("learn/")
                || matches!(p.atom.pred.as_str(), "stand_in" | "replace_step" | "part")
                || (p.atom.pred == "can" && id.starts_with("core/B6/2"));
            usize::from(counted)
        }
    };
    own + p.children.iter().map(hops).sum::<usize>()
}

pub fn to_json(p: &Proof) -> J {
    match &p.step {
        ProofStep::Given { source } => json!({"atom": p.atom.to_string(), "given": source}),
        ProofStep::Rule { id, absent, .. } => json!({
            "atom": p.atom.to_string(), "rule": id, "conf": p.conf,
            "premises": p.children.iter().map(to_json).collect::<Vec<_>>(),
            "checked_absent": absent,
        }),
    }
}

/// The proof record for a claim, if the model holds it.
pub fn record(m: &Model, claim: &GroundAtom) -> Option<J> {
    let p = m.proof(claim)?;
    let kind = if p.rules_used().iter().any(|r| r.starts_with("learn/")) { "hypothesis" } else if matches!(p.step, ProofStep::Given { .. }) { "given" } else { "deduced" };
    Some(json!({"claim": claim.to_string(), "kind": kind, "conf": p.conf, "hops": hops(&p),
        "rules_used": p.rules_used(), "leaves": p.leaves().iter().map(|l| l.to_string()).collect::<Vec<_>>(), "tree": to_json(&p)}))
}
