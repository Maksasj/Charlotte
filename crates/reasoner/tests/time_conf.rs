//! Milestone 3 acceptance: facts carry start/end times, queries default to
//! "now" and can ask about the past, contradictions close the old fact, and
//! derived confidence is the minimum of the inputs.

use sla_core::{parse_atom, parse_ground, parse_rule, Fact, KnowledgeBase, RuleFile, RuleMeta, RuleStatus, Timestamp};
use sla_reasoner::evaluate_kb;
use std::fs;

fn temp_vault(name: &str) -> std::path::PathBuf {
    let d = std::env::temp_dir().join(format!("sla_m3_{name}_{}", std::process::id()));
    let _ = fs::remove_dir_all(&d);
    d
}

#[test]
fn dont_drink_coffee_anymore() {
    let root = temp_vault("coffee");
    let t1 = Timestamp::from_ymd(2026, 3, 1);
    let t2 = Timestamp::from_ymd(2026, 11, 16);
    let now = t2.plus_days(1);
    {
        let mut kb = KnowledgeBase::open(&root).unwrap();
        kb.assert_fact(Fact::new(parse_ground("likes(maksim, coffee)").unwrap(), "user").with_conf(0.9), t1, "user said: I like coffee");
        kb.save().unwrap();
        // "I don't drink coffee anymore" -> the parser (milestone 4) produces this:
        kb.assert_fact(Fact::new(parse_ground("-likes(maksim, coffee)").unwrap(), "user"), t2, "user said: I don't drink coffee anymore");
        kb.save().unwrap();
    }
    // Restart: everything comes back from the vault files.
    let kb = KnowledgeBase::open(&root).unwrap();
    let m_now = evaluate_kb(&kb, Some(now), false).unwrap();
    assert!(m_now.ask("likes(maksim, coffee)").unwrap().is_empty(), "current: no longer likes coffee");
    assert_eq!(m_now.ask("-likes(maksim, coffee)").unwrap().len(), 1);

    let m_past = evaluate_kb(&kb, Some(t1.plus_days(10)), false).unwrap();
    assert_eq!(m_past.ask("likes(maksim, coffee)").unwrap().len(), 1, "past: liked coffee");

    let hist = kb.history(&parse_atom("likes(maksim, coffee)").unwrap());
    assert_eq!(hist.len(), 1, "history keeps the closed fact");
    assert_eq!(hist[0].from, Some(t1));
    assert_eq!(hist[0].until, Some(t2));
    assert_eq!(hist[0].conf, 0.9);

    // The file is readable and shows both lines.
    let text = fs::read_to_string(root.join("facts").join("maksim.md")).unwrap();
    assert!(text.contains("likes(maksim, coffee). @from 2026-03-01 @until 2026-11-16 @conf 0.9 @source user"), "{text}");
    assert!(text.contains("-likes(maksim, coffee). @from 2026-11-16 @conf 1 @source user"), "{text}");

    // The audit log recorded the close with its reason.
    let log = kb.vault.read_log().unwrap();
    assert!(log.iter().any(|(_, a, s, _)| a == "close" && s == "likes(maksim, coffee)"));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn derived_confidence_and_time() {
    let root = temp_vault("derived");
    let t1 = Timestamp::from_ymd(2026, 1, 1);
    let t2 = Timestamp::from_ymd(2026, 6, 1);
    let mut kb = KnowledgeBase::open(&root).unwrap();
    kb.logging = false;
    kb.assert_fact(Fact::new(parse_ground("located_in(maksim, berlin)").unwrap(), "user").with_conf(0.7), t1, "told");
    kb.assert_fact(Fact::new(parse_ground("located_in(berlin_office, berlin)").unwrap(), "user"), t1, "told");
    kb.assert_fact(Fact::new(parse_ground("is_a(berlin, city)").unwrap(), "user"), t1, "told");
    let meta = RuleMeta {
        id: "lives_in_city".into(),
        status: RuleStatus::Confirmed,
        conf: 0.8,
        evidence: 2,
        counter: 0,
        module: "places".into(),
        source: "user".into(),
    };
    kb.vault.put_rule_file(RuleFile::new(&meta, "Someone located in a city lives in that city.", &[
        parse_rule("lives_in_city(X, C) :- located_in(X, C), is_a(C, city).").unwrap(),
    ]));
    // located_in is functional on its first argument: moving closes the old place.
    kb.assert_fact(Fact::new(parse_ground("located_in(maksim, paris)").unwrap(), "user"), t2, "moved");
    kb.assert_fact(Fact::new(parse_ground("is_a(paris, city)").unwrap(), "user"), t2, "told");
    kb.save().unwrap();

    let before = evaluate_kb(&kb, Some(t1.plus_days(1)), false).unwrap();
    let g = parse_ground("lives_in_city(maksim, berlin)").unwrap();
    assert_eq!(before.conf(&g), Some(0.7), "min(0.7 fact, 0.8 rule, 1.0)");
    let proof = before.proof(&g).unwrap().render();
    assert!(proof.contains("by rule lives_in_city#0"), "{proof}");

    let after = evaluate_kb(&kb, Some(t2.plus_days(1)), false).unwrap();
    assert!(!after.contains(&g));
    assert_eq!(after.conf(&parse_ground("lives_in_city(maksim, paris)").unwrap()), Some(0.8));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn core_rules_apply() {
    let root = temp_vault("core");
    let t = Timestamp::from_ymd(2026, 1, 1);
    let mut kb = KnowledgeBase::open(&root).unwrap();
    kb.logging = false;
    for s in ["is_a(rex, dog)", "is_a(dog, animal)", "has(animal, heart)", "is_a(anna, person)"] {
        kb.assert_fact(Fact::new(parse_ground(s).unwrap(), "user"), t, "told");
    }
    let m = evaluate_kb(&kb, Some(t), false).unwrap();
    assert_eq!(m.ask("is_a(rex, animal)").unwrap().len(), 1);
    assert_eq!(m.ask("has(rex, heart)").unwrap().len(), 1);
    assert_eq!(m.ask("thing(anna)").unwrap().len(), 1);
    // Core files exist on disk and are readable.
    assert!(root.join("core").join("ontology.md").exists());
    let _ = fs::remove_dir_all(&root);
}
