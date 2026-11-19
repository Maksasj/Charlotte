//! Milestone 31 acceptance for the conversation path (Design 4.0 section 6).

use agent4::{learn, Agent, Teacher};
use std::collections::BTreeMap;

struct Oracle {
    answers: BTreeMap<String, String>,
    asked: Vec<String>,
}

impl Teacher for Oracle {
    fn answer(&mut self, q: &str) -> String {
        self.asked.push(q.to_string());
        self.answers.get(q).cloned().unwrap_or_else(|| "unknown".into())
    }
}

fn oracle(pairs: &[(&str, &str)]) -> Oracle {
    Oracle { answers: pairs.iter().map(|(a, b)| (a.to_string(), b.to_string())).collect(), asked: vec![] }
}

fn teach(a: &mut Agent, t: &mut Oracle, lines: &str) -> Vec<String> {
    let mut out = Vec::new();
    for l in lines.lines() {
        out.extend(a.hear(l, "fixture", t));
    }
    out
}

#[test]
fn derived_consequences_are_shown_when_taught() {
    let mut a = Agent::new("t");
    let mut t = oracle(&[]);
    let out = teach(&mut a, &mut t, "topic baking/tins.\nnew(loaf_tin, item).\nnew(baking_tin, item).\nis_a(loaf_tin, baking_tin).\ncan_do_on(baking_tin, hold, batter).");
    assert!(out.iter().any(|l| l.contains("derived: can_do_on(loaf_tin, hold, batter)")), "{out:?}");
}

#[test]
fn sort_errors_are_rejected_and_not_stored() {
    let mut a = Agent::new("t");
    let mut t = oracle(&[]);
    teach(&mut a, &mut t, "new(oven, place).\nnew(metal, material).\nmade_of(oven, metal).");
    let out = teach(&mut a, &mut t, "made_of(metal, oven).");
    assert!(out[0].starts_with("reject:"), "{out:?}");
    assert_eq!(a.stats.rejected, 1);
}

#[test]
fn an_inherited_contrary_asks_for_an_exception() {
    let mut a = Agent::new("t");
    let mut t = oracle(&[("conflict: not can_go_in(plastic_tub, oven) vs inherited contrary. exception?", "exception")]);
    teach(&mut a, &mut t, "new(oven, place).\nis_a(plastic_tub, tin).\ncan_go_in(tin, oven).");
    let out = teach(&mut a, &mut t, "not can_go_in(plastic_tub, oven).");
    assert!(t.asked.iter().any(|q| q.contains("exception?")), "{:?}", t.asked);
    assert!(out.iter().any(|l| l.contains("exception stored")), "{out:?}");
    let r = a.respond("query: can_go_in(plastic_tub, oven).");
    assert!(r.claims.is_empty());
    let r = a.respond("query: not can_go_in(plastic_tub, oven).");
    assert_eq!(r.claims.len(), 1);
}

#[test]
fn naming_drift_is_asked_and_canonicalised() {
    let mut a = Agent::new("t");
    let mut t = oracle(&[("same_as(egg_whisk, whisk)? or is_a(egg_whisk, whisk)?", "same_as")]);
    teach(&mut a, &mut t, "new(whisk, item).\nnew(beat, function).\ncan_do(whisk, beat).\nnew(egg_whisk, item).\nused_for(egg_whisk, mix).");
    assert!(t.asked.iter().any(|q| q.starts_with("same_as(egg_whisk, whisk)")), "{:?}", t.asked);
    let r = a.respond("query: used_for(whisk, mix).");
    assert_eq!(r.claims.len(), 1, "{r:?}");
}

#[test]
fn a_request_gets_a_stand_in_with_its_proof() {
    let mut a = Agent::new("t");
    let mut t = oracle(&[]);
    teach(&mut a, &mut t, "new(oven, place).\nis_a(loaf_tin, baking_tin).\nis_a(muffin_tray, baking_tin).\ncan_do_on(baking_tin, hold, batter).\ncan_go_in(baking_tin, oven).\nnew(bake_bread, task).\nstep(bake_bread, bake_bread_s4).\nuses(bake_bread_s4, loaf_tin).\nrole(bake_bread_s4, loaf_tin, hold).\ndemands(bake_bread_s4, hold, can_go_in, oven).");
    let r = a.respond("request(bake_bread). lacks(user, loaf_tin). has(user, muffin_tray).");
    assert_eq!(r.claims.len(), 1, "{r:?}");
    assert_eq!(r.claims[0].0, "swap(bake_bread_s4, loaf_tin, muffin_tray)");
    assert_eq!(r.claims[0].1["kind"], "deduced");
    let r = a.respond("request(bake_bread). lacks(user, loaf_tin).");
    assert!(r.gaps.iter().any(|g| g.contains("gap(bake_bread_s4, loaf_tin)")), "{r:?}");
}

const PLASTIC: &str = "new(oven, place).\nnew(plastic, material).\nnew(glass, material).\nis_a(tub, container).\nis_a(jug, container).\nis_a(box, container).\nis_a(bowl, dish).\nmade_of(tub, plastic).\nmade_of(jug, plastic).\nmade_of(bowl, plastic).\nmade_of(box, plastic).\nnot can_go_in(tub, oven).\nnot can_go_in(jug, oven).\nnot can_go_in(bowl, oven).";

#[test]
fn a_rule_is_induced_confirmed_and_used_as_a_hypothesis() {
    let mut a = Agent::new("t");
    let mut t = oracle(&[("ask: not can_go_in(box, oven)?", "yes")]);
    teach(&mut a, &mut t, PLASTIC);
    teach(&mut a, &mut t, "new(spoon, item).\nis_a(spoon, cutlery).\nmade_of(spoon, plastic).");
    let out = a.end_batch(&mut t);
    assert!(out.iter().any(|l| l.starts_with("hypothesis: not can_go_in(X, oven) :- made_of(X, plastic)")), "{out:?}");
    let r = a.respond("query: not can_go_in(spoon, oven).");
    assert_eq!(r.claims.len(), 1, "{r:?}");
    assert_eq!(r.claims[0].1["kind"], "hypothesis");
}

#[test]
fn below_the_support_threshold_nothing_is_induced() {
    let mut a = Agent::new("t");
    let mut t = oracle(&[]);
    teach(&mut a, &mut t, "new(oven, place).\nnew(plastic, material).\nis_a(tub, container).\nis_a(jug, kitchenware).\nis_a(box, container).\nmade_of(tub, plastic).\nmade_of(jug, plastic).\nmade_of(box, plastic).\nnot can_go_in(tub, oven).\nnot can_go_in(jug, oven).");
    let out = a.end_batch(&mut t);
    assert!(!out.iter().any(|l| l.contains("can_go_in(X, oven)")), "{out:?}");
}

#[test]
fn a_counterexample_withdraws_and_revises_the_rule() {
    let mut a = Agent::new("t");
    let mut t = oracle(&[]);
    teach(&mut a, &mut t, PLASTIC);
    teach(&mut a, &mut t, "is_a(spoon, cutlery).\nmade_of(spoon, plastic).");
    a.end_batch(&mut t);
    assert!(a.learned.iter().any(|r| r.status == "hypothesis"));
    // the teacher says the silicone-free spoon can go in the oven after all
    let out = teach(&mut a, &mut t, "can_go_in(spoon, oven).");
    assert!(out.iter().any(|l| l.starts_with("withdrawn:")), "{out:?}");
    assert!(out.iter().any(|l| l.starts_with("specialised:") || l.starts_with("retracted:")), "{out:?}");
    let _ = learn::MIN_SUPPORT.load(std::sync::atomic::Ordering::SeqCst);
}
