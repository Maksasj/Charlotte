//! Milestone 31 acceptance: the logic §2.4 cases reproduce; the core passes the engine's safety
//! and stratification check; each Bk switches off by id; stand-in, demands, gaps.

use reason4::{core_rules, encode, holds_atom, parse_atom, run};
use sla_reasoner::{FactInput, Model};

fn kb(lines: &[&str]) -> Vec<FactInput> {
    lines
        .iter()
        .map(|l| {
            let (neg, a) = match l.strip_prefix("not ") { Some(r) => (true, r), None => (false, *l) };
            FactInput { atom: encode(&parse_atom(a).unwrap(), neg), conf: 1.0, source: "test".into() }
        })
        .collect()
}

fn holds(m: &Model, a: &str) -> bool {
    m.contains(&holds_atom(&parse_atom(a).unwrap(), false))
}
fn holdsn(m: &Model, a: &str) -> bool {
    m.contains(&holds_atom(&parse_atom(a).unwrap(), true))
}
fn has(m: &Model, a: &str) -> bool {
    m.contains(&parse_atom(a).unwrap())
}

const BIRDS: &[&str] = &[
    "is_a(bird, animal)", "is_a(penguin, bird)", "is_a(tweety, penguin)", "is_a(tweety, bird)",
    "is_a(superpenguin, penguin)", "is_a(pingu, superpenguin)",
    "can_do(bird, fly)", "not can_do(penguin, fly)", "can_do(superpenguin, fly)",
    "is_a(nixon, quaker)", "is_a(nixon, republican)",
    "has_property(quaker, pacifist)", "not has_property(republican, pacifist)",
];

#[test]
fn core_passes_safety_and_stratification() {
    let all: Vec<_> = core_rules().into_values().flatten().collect();
    sla_reasoner::check(&all).unwrap();
    let ids: Vec<String> = core_rules().keys().cloned().collect();
    assert_eq!(ids, ["B1", "B2", "B3", "B5", "B6", "B7", "B8", "B9"]);
}

#[test]
fn penguin_with_a_redundant_link() {
    // tweety is_a penguin and (redundantly) is_a bird: the more specific penguin wins
    let m = run(&kb(BIRDS), &[], &[]).unwrap();
    assert!(holdsn(&m, "can_do(tweety, fly)"));
    assert!(!holds(&m, "can_do(tweety, fly)"));
}

#[test]
fn exception_to_an_exception() {
    let m = run(&kb(BIRDS), &[], &[]).unwrap();
    assert!(holds(&m, "can_do(pingu, fly)"));
    assert!(!holdsn(&m, "can_do(pingu, fly)"));
}

#[test]
fn diamond_blocks_both_and_asks() {
    let m = run(&kb(BIRDS), &[], &[]).unwrap();
    assert!(!holds(&m, "has_property(nixon, pacifist)"));
    assert!(!holdsn(&m, "has_property(nixon, pacifist)"));
    assert!(has(&m, "ask_conflict(nixon, has_property, pacifist)"));
}

const BAKING: &[&str] = &[
    "is_a(loaf_tin, baking_tin)", "is_a(muffin_tray, baking_tin)", "is_a(baking_tin, tin)", "is_a(plastic_tub, tin)",
    "can_do_on(baking_tin, hold, batter)", "can_do_on(plastic_tub, hold, batter)",
    "can_go_in(tin, oven)", "not can_go_in(plastic_tub, oven)",
    "role(bake_s4, loaf_tin, hold)", "uses(bake_s4, loaf_tin)", "demands(bake_s4, hold, can_go_in, oven)",
    "lacks(user, loaf_tin)",
];

#[test]
fn muffin_tray_stands_in_for_the_loaf_tin() {
    let mut f = BAKING.to_vec();
    f.push("has(user, muffin_tray)");
    let m = run(&kb(&f), &[], &[]).unwrap();
    assert!(holds(&m, "can_do_on(muffin_tray, hold, batter)"));
    assert!(has(&m, "stand_in(bake_s4, loaf_tin, muffin_tray)"));
    assert!(!has(&m, "gap(bake_s4, loaf_tin)"));
}

#[test]
fn the_plastic_tub_decoy_fails_the_demand() {
    let mut f = BAKING.to_vec();
    f.push("has(user, plastic_tub)");
    let m = run(&kb(&f), &[], &[]).unwrap();
    assert!(!has(&m, "stand_in(bake_s4, loaf_tin, plastic_tub)"));
    assert!(has(&m, "unfit_known(bake_s4, plastic_tub)"));
    assert!(has(&m, "gap(bake_s4, loaf_tin)"));
}

#[test]
fn an_unknown_demand_asks_instead_of_swapping() {
    let mut f: Vec<&str> = BAKING.iter().copied().filter(|l| *l != "can_go_in(tin, oven)").collect();
    f.push("has(user, muffin_tray)");
    let m = run(&kb(&f), &[], &[]).unwrap();
    assert!(!has(&m, "stand_in(bake_s4, loaf_tin, muffin_tray)"));
    assert!(has(&m, "ask_fit(bake_s4, muffin_tray)"));
}

#[test]
fn each_base_rule_switches_off_by_id() {
    let mut f = BAKING.to_vec();
    f.push("has(user, muffin_tray)");
    let m = run(&kb(&f), &["B7"], &[]).unwrap();
    assert!(!has(&m, "stand_in(bake_s4, loaf_tin, muffin_tray)"));
    let m = run(&kb(&f), &["B1"], &[]).unwrap();
    assert!(!holds(&m, "can_do_on(muffin_tray, hold, batter)"), "without B1 there is no isa, so nothing inherits");
}

#[test]
fn proofs_show_their_premises_and_hops() {
    let mut f = BAKING.to_vec();
    f.push("has(user, muffin_tray)");
    let m = run(&kb(&f), &[], &[]).unwrap();
    let r = reason4::proof::record(&m, &parse_atom("stand_in(bake_s4, loaf_tin, muffin_tray)").unwrap()).unwrap();
    assert_eq!(r["kind"], "deduced");
    let leaves: Vec<String> = r["leaves"].as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_string()).collect();
    assert!(leaves.iter().any(|l| l.contains("is_a(muffin_tray, baking_tin)")), "{leaves:?}");
    assert!(r["hops"].as_u64().unwrap() >= 2);
}
