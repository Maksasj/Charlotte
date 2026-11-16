//! Milestone 7 acceptance: after 5 synthetic mornings with coffee, a
//! hypothesis rule appears; after a counterexample its confidence drops.

use sla_core::{parse_rule, Fact, KnowledgeBase, RuleStatus, Timestamp, Vault};
use sla_learner::{g, Learner};

fn kb() -> KnowledgeBase {
    let mut v = Vault::empty(std::env::temp_dir().join("sla_m7_unused"));
    for (s, rf) in sla_core::default_core() {
        v.core.insert(s, rf);
    }
    let mut kb = KnowledgeBase::new(v);
    kb.logging = false;
    kb
}

fn episode(kb: &mut KnowledgeBase, id: &str, when: &str, drink: &str, t: Timestamp) {
    for s in [format!("event({id})"), format!("during({id}, {when})"), format!("agent_of({id}, maksim)"), format!("drink({id}, {drink})")] {
        kb.assert_fact(Fact::new(g(&s), "synthetic"), t, "episode");
    }
}

fn coffee_rule(kb: &KnowledgeBase) -> Option<sla_core::RuleMeta> {
    let r = parse_rule("drink(X, coffee) :- during(X, morning).").unwrap();
    kb.find_rule(&r).map(|stem| kb.vault.rules[&stem].meta())
}

#[test]
fn five_mornings_then_counterexample() {
    let mut kb = kb();
    let l = Learner::default();
    let day = |d: i64| Timestamp::from_ymd_hms(2026, 11, 1, 8, 0, 0).plus_days(d);
    // Evenings with tea make "event" a poor predictor of coffee.
    for d in 0..3 {
        episode(&mut kb, &format!("evening{d}"), "evening", "tea", day(d).plus_secs(12 * 3600));
    }
    let mut first_seen = None;
    for d in 0..5 {
        episode(&mut kb, &format!("morning{d}"), "morning", "coffee", day(d));
        let rep = l.update(&mut kb, day(d));
        if first_seen.is_none() && coffee_rule(&kb).is_some() {
            first_seen = Some(d + 1);
            assert!(rep.proposed.iter().any(|id| id.starts_with("hyp_drink")), "{rep:?}");
        }
    }
    let after5 = coffee_rule(&kb).expect("hypothesis after 5 mornings");
    assert_eq!(after5.status, RuleStatus::Hypothesis);
    assert_eq!(after5.evidence, 5);
    assert_eq!(after5.counter, 0);
    assert!(after5.conf < 0.6, "starts with low confidence: {}", after5.conf);
    assert!(first_seen.unwrap() >= 3, "needs at least min_support cases");

    // Confidence rises with another supporting case...
    episode(&mut kb, "morning5", "morning", "coffee", day(5));
    l.update(&mut kb, day(5));
    let after6 = coffee_rule(&kb).unwrap();
    assert!(after6.conf > after5.conf);

    // ...and drops after a counterexample: a morning with tea.
    episode(&mut kb, "morning6", "morning", "tea", day(6));
    let rep = l.update(&mut kb, day(6));
    let after_counter = coffee_rule(&kb).unwrap();
    assert_eq!(after_counter.counter, 1);
    assert!(after_counter.conf < after6.conf, "{} !< {}", after_counter.conf, after6.conf);
    assert!(rep.changed.iter().any(|(id, old, new)| id == &after_counter.id && new < old));
    // The evening-tea pattern was also noticed.
    assert!(kb.find_rule(&parse_rule("drink(X, tea) :- during(X, evening).").unwrap()).is_some());
    // No rule predicts bookkeeping predicates.
    assert!(kb.vault.rules.values().all(|rf| !rf.rules()[0].head.pred.starts_with("during")));
}

#[test]
fn promotion_past_threshold() {
    let mut kb = kb();
    let l = Learner::default();
    let t0 = Timestamp::from_ymd_hms(2026, 11, 1, 8, 0, 0);
    for d in 0..3 {
        episode(&mut kb, &format!("evening{d}"), "evening", "tea", t0.plus_days(d).plus_secs(43200));
    }
    let mut promoted_at = None;
    for d in 0..30 {
        episode(&mut kb, &format!("m{d}"), "morning", "coffee", t0.plus_days(d));
        let rep = l.update(&mut kb, t0.plus_days(d));
        if !rep.promoted.is_empty() && promoted_at.is_none() {
            promoted_at = Some(d + 1);
        }
    }
    let m = coffee_rule(&kb).unwrap();
    assert_eq!(m.status, RuleStatus::Confirmed, "{m:?}");
    // conf = n/(n+5) >= 0.8 needs n >= 20.
    assert_eq!(promoted_at, Some(20));
    // A promoted rule does not count its own conclusions: a new morning without
    // any drink is still a counterexample.
    for s in ["event(m99)", "during(m99, morning)"] {
        kb.assert_fact(Fact::new(g(s), "synthetic"), t0.plus_days(40), "episode");
    }
    l.update(&mut kb, t0.plus_days(40));
    assert_eq!(coffee_rule(&kb).unwrap().counter, 1);
}

#[test]
fn rejected_when_counterexamples_dominate() {
    let mut kb = kb();
    let l = Learner::default();
    let t0 = Timestamp::from_ymd_hms(2026, 11, 1, 8, 0, 0);
    // Contrast: evenings with tea, so "morning" is what predicts coffee.
    for d in 0..3 {
        episode(&mut kb, &format!("e{d}"), "evening", "tea", t0.plus_days(d).plus_secs(43200));
    }
    for d in 0..3 {
        episode(&mut kb, &format!("m{d}"), "morning", "coffee", t0.plus_days(d));
    }
    l.update(&mut kb, t0.plus_days(3));
    assert!(coffee_rule(&kb).is_some());
    for d in 3..10 {
        episode(&mut kb, &format!("m{d}"), "morning", "juice", t0.plus_days(d));
        l.update(&mut kb, t0.plus_days(d));
    }
    assert_eq!(coffee_rule(&kb).unwrap().status, RuleStatus::Rejected);
}
