//! Milestone 8 acceptance: a dedicated test set of corrections
//! (`data/synthetic/corrections/cases.md`) changes the right rules and
//! nothing else; demonstrations become effect rules.

use sla_core::{parse_ground, parse_rule, Fact, GroundAtom, KnowledgeBase, RuleFile, RuleMeta, RuleStatus, Timestamp, Vault};
use sla_learner::corrections::{correct_derived, learn_from_demo, Change, Demonstration};
use sla_learner::Learner;
use sla_reasoner::evaluate_kb;
use std::collections::BTreeMap;

#[derive(Default, Debug)]
struct Case {
    name: String,
    facts: Vec<String>,
    rules: Vec<(String, String, bool)>,
    wrong: Vec<String>,
    changed: Vec<String>,
    expect_rules: Vec<(String, String)>,
    also_false: Vec<String>,
    keeps: Vec<String>,
    stored: Vec<String>,
}

fn load_cases() -> Vec<Case> {
    let path = format!("{}/../../data/synthetic/corrections/cases.md", env!("CARGO_MANIFEST_DIR"));
    let src = std::fs::read_to_string(path).unwrap();
    let mut cases = Vec::new();
    let mut cur: Option<Case> = None;
    for line in src.lines() {
        if let Some(name) = line.strip_prefix("## ") {
            if let Some(c) = cur.take() {
                cases.push(c);
            }
            cur = Some(Case { name: name.into(), ..Default::default() });
            continue;
        }
        let Some(c) = cur.as_mut() else { continue };
        let Some((key, val)) = line.split_once(": ") else { continue };
        let val = val.trim();
        match key {
            "facts" => c.facts.extend(val.split(';').map(|s| s.trim().to_string())),
            "rules" => {
                let (id, rule) = val.split_once(": ").unwrap();
                let learner = id.ends_with("[learner]");
                c.rules.push((id.trim_end_matches("[learner]").trim().into(), rule.into(), learner));
            }
            "wrong" => c.wrong.push(val.into()),
            "changed" => {
                if val != "none" {
                    c.changed = val.split(',').map(|s| s.trim().to_string()).collect();
                }
            }
            "rule" => {
                let (id, rule) = val.split_once(": ").unwrap();
                c.expect_rules.push((id.into(), rule.into()));
            }
            "also_false" => c.also_false.extend(val.split(';').map(|s| s.trim().to_string())),
            "keeps" => c.keeps.push(val.into()),
            "stored" => c.stored.push(val.into()),
            _ => {}
        }
    }
    cases.extend(cur);
    cases
}

fn build(c: &Case, t: Timestamp) -> KnowledgeBase {
    let mut v = Vault::empty(std::env::temp_dir().join("sla_m8_unused"));
    for (s, rf) in sla_core::default_core() {
        v.core.insert(s, rf);
    }
    let mut kb = KnowledgeBase::new(v);
    kb.logging = false;
    for f in &c.facts {
        kb.assert_fact(Fact::new(parse_ground(f).unwrap(), "teacher"), t, "setup");
    }
    for (id, rule, learner) in &c.rules {
        let meta = RuleMeta {
            id: id.clone(),
            status: if *learner { RuleStatus::Hypothesis } else { RuleStatus::Confirmed },
            conf: if *learner { 0.4 } else { 0.9 },
            evidence: 3,
            counter: 0,
            module: "general".into(),
            source: if *learner { "learner".into() } else { "teacher".into() },
        };
        kb.vault.put_rule_file(RuleFile::new(&meta, "test rule", &[parse_rule(rule).unwrap()]));
    }
    kb
}

fn rule_texts(kb: &KnowledgeBase) -> BTreeMap<String, String> {
    kb.vault.rules.iter().map(|(k, rf)| (k.clone(), rf.render())).collect()
}

fn believed(kb: &KnowledgeBase, t: Timestamp) -> Vec<GroundAtom> {
    // Hypotheses count too: a learned rule is part of what the agent may answer from.
    evaluate_kb(kb, Some(t), true).unwrap().all_facts()
}

#[test]
fn correction_set() {
    let cases = load_cases();
    assert!(cases.len() >= 10, "only {} cases", cases.len());
    let t0 = Timestamp::from_ymd(2026, 11, 1);
    let mut failures = Vec::new();
    for c in &cases {
        let mut kb = build(c, t0);
        // Hypothesis rules answer too in this test: include them by confirming status for evaluation.
        let before_rules = rule_texts(&kb);
        let before = believed(&kb, t0);
        let mut t = t0;
        for w in &c.wrong {
            t = t.plus_days(1);
            let wrong = parse_ground(w).unwrap();
            // Learner hypotheses only answer when included; evaluate with them like the agent's teacher sees them.
            let out = correct_derived(&mut kb, &wrong, t, "teacher");
            if matches!(out.change, Change::GivenFact) {
                kb.close_matching(&wrong.to_atom(), t, "teacher said it is wrong");
            }
            if matches!(out.change, Change::NotDerived) && kb.rules(true).iter().any(|r| r.status == RuleStatus::Hypothesis) {
                // The wrong conclusion came from a hypothesis: promote for evaluation and retry.
                for rf in kb.vault.rules.values_mut() {
                    let mut m = rf.meta();
                    if m.status == RuleStatus::Hypothesis {
                        m.status = RuleStatus::Confirmed;
                        rf.set_meta(&m);
                    }
                }
                let out2 = correct_derived(&mut kb, &wrong, t, "teacher");
                assert!(!matches!(out2.change, Change::NotDerived), "{}: {w} not derived", c.name);
            }
        }
        let after_rules = rule_texts(&kb);
        let after = believed(&kb, t);
        // 1. Exactly the expected rule files changed.
        let changed: Vec<String> = after_rules.iter().filter(|(k, v)| before_rules.get(*k) != Some(v)).map(|(k, _)| k.clone()).collect();
        let mut want = c.changed.clone();
        want.sort();
        if changed != want {
            failures.push(format!("{}: changed {changed:?}, want {want:?}", c.name));
            continue;
        }
        // 2. Changed rules have the expected text.
        for (id, text) in &c.expect_rules {
            let got = kb.vault.rules[id].rules()[0].to_string();
            if &got != text {
                failures.push(format!("{}: rule {id} = {got}, want {text}", c.name));
            }
        }
        // 3. Wrong conclusions are gone (as positive atoms).
        for w in c.wrong.iter().chain(&c.also_false).filter(|w| !c.keeps.contains(w)) {
            let g = parse_ground(w).unwrap();
            if after.contains(&g) {
                failures.push(format!("{}: still believes {w}", c.name));
            }
        }
        for st in &c.stored {
            let g = parse_ground(st).unwrap();
            if !kb.all_facts().iter().any(|f| f.atom == g && f.valid_at(t)) {
                failures.push(format!("{}: {st} not stored", c.name));
            }
        }
        // 4. Nothing else was lost.
        for b in &before {
            let s = b.to_string();
            if !after.contains(b) && !c.wrong.contains(&s) && !c.also_false.contains(&s) {
                failures.push(format!("{}: lost unrelated belief {s}", c.name));
            }
        }
    }
    assert!(failures.is_empty(), "{} failures:\n{}", failures.len(), failures.join("\n"));
}

fn g(s: &str) -> GroundAtom {
    parse_ground(s).unwrap()
}

#[test]
fn demonstrations_become_rules() {
    let mut v = Vault::empty(std::env::temp_dir().join("sla_m8_demo"));
    for (s, rf) in sla_core::default_core() {
        v.core.insert(s, rf);
    }
    let mut kb = KnowledgeBase::new(v);
    kb.logging = false;
    let l = Learner::default();
    let t = Timestamp::from_ymd(2026, 11, 1);
    let demo = |cup: &str, colour: &str| Demonstration {
        action: g(&format!("fill(maksim, {cup})")),
        before: vec![g(&format!("is_a({cup}, cup)")), g(&format!("property({cup}, colour, {colour})")), g(&format!("property({cup}, quality, empty)"))],
        after: vec![g(&format!("is_a({cup}, cup)")), g(&format!("property({cup}, colour, {colour})")), g(&format!("property({cup}, quality, full)"))],
    };
    let first = learn_from_demo(&mut kb, &l, &demo("cup1", "red"), t);
    assert_eq!(first.len(), 2, "one positive and one negative effect: {first:?}");
    let full = &kb.vault.rules["demo_fill_property"];
    assert_eq!(full.rules()[0].to_string(), "property(B, quality, full) :- fill(A, B), is_a(B, cup), property(B, colour, red).");
    assert_eq!(full.meta().status, RuleStatus::Hypothesis);
    // A second demonstration with a blue cup drops the incidental colour.
    learn_from_demo(&mut kb, &l, &demo("cup2", "blue"), t);
    let full = &kb.vault.rules["demo_fill_property"];
    assert_eq!(full.rules()[0].to_string(), "property(B, quality, full) :- fill(A, B), is_a(B, cup).");
    assert_eq!(full.meta().evidence, 2);
    let neg = &kb.vault.rules["demo_fill_not_property"];
    assert_eq!(neg.rules()[0].to_string(), "-property(B, quality, empty) :- fill(A, B), is_a(B, cup).");
    // The learned rule predicts the effect for a new cup.
    for f in ["is_a(cup3, cup)", "fill(anna, cup3)"] {
        kb.assert_fact(Fact::new(g(f), "user"), t, "test");
    }
    let m = evaluate_kb(&kb, Some(t), true).unwrap();
    assert!(m.contains(&g("property(cup3, quality, full)")));
    assert!(!m.contains(&g("property(cup1, colour, blue)")));
}

/// Design 2.0 (milestone 12): a refinement that would make the program
/// invalid leaves the rule, its counter and the vault exactly as they were,
/// apart from the exception fact, and says so.
#[test]
fn refused_refinement_leaves_no_partial_change() {
    let mut v = Vault::empty("unused");
    for (s, rf) in sla_core::default_core() {
        v.core.insert(s, rf);
    }
    let mut kb = KnowledgeBase::new(v);
    kb.logging = false;
    let t = Timestamp::from_ymd(2026, 1, 1);
    for f in ["is_a(tweety, bird)", "is_a(robin, bird)", "is_a(pingu, bird)", "cold(pingu)", "is_a(pingo, bird)", "cold(pingo)"] {
        kb.assert_fact(Fact::new(parse_ground(f).unwrap(), "test"), t, "setup");
    }
    let meta = |id: &str| RuleMeta { id: id.into(), status: RuleStatus::Confirmed, conf: 0.9, evidence: 0, counter: 0, module: "general".into(), source: "user".into() };
    kb.vault.put_rule_file(RuleFile::new(&meta("flies_rule"), "Birds fly.", &[parse_rule("flies(X) :- is_a(X, bird).").unwrap()]));
    // `brag` depends on `flies`: adding `not brag(X)` (shared by both cold
    // counterexamples) to the flies rule would put negation inside recursion.
    kb.vault.put_rule_file(RuleFile::new(&meta("brag_rule"), "Cold fliers brag.", &[parse_rule("brag(X) :- cold(X).").unwrap(), parse_rule("brag(X) :- flies(X), loud(X).").unwrap()]));
    // The first counterexample is only an exception (gradual corrections).
    let first = correct_derived(&mut kb, &parse_ground("flies(pingo)").unwrap(), t.plus_days(1), "user");
    assert_eq!(first.change, Change::Exception, "{first:?}");
    let before_rules = kb.vault.rules.get("flies_rule").unwrap().render();
    let now = t.plus_days(2);
    let out = correct_derived(&mut kb, &parse_ground("flies(pingu)").unwrap(), now, "user");
    assert_eq!(out.change, Change::RefinementRefused, "{out:?}");
    assert!(out.new_rule.is_none());
    assert_eq!(kb.vault.rules.get("flies_rule").unwrap().render(), before_rules, "rule text and counter unchanged");
    let negs: Vec<String> = kb.all_facts().iter().filter(|f| f.atom.pred == "-flies").map(|f| f.atom.to_string()).collect();
    assert_eq!(negs, vec!["-flies(pingo)", "-flies(pingu)"], "one exception fact per counterexample");
    assert!(evaluate_kb(&kb, Some(now), true).is_ok(), "the program stays valid");
}
