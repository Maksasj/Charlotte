//! Milestone 22 acceptance: gold answers score 100%, negative controls fail, lines round-trip.

use kb3::{check, gold, lacked, lines, Aliases, Fact, Procedure, Scenario};


fn corpus() -> Vec<Scenario> {
    let root = kb3::repo_root().join("data").join("curriculum3");
    let mut all = Vec::new();
    for d in ["train", "dev", "seed", "heldout_domain"] {
        all.extend(Scenario::load_dir(&root.join(d)).unwrap());
    }
    all
}

fn answer_of(p: &Procedure) -> Procedure {
    lines::parse(&lines::print(p, &[], &[], &[])).procedure
}

#[test]
fn every_fact_is_in_the_schema() {
    for sc in corpus() {
        for f in sc.facts.iter().chain(sc.aliases.iter()) {
            f.valid().unwrap_or_else(|e| panic!("{}: {e}", sc.task));
        }
        for v in &sc.variants {
            for f in v.situation.iter().chain(v.difference.iter()) {
                f.valid().unwrap_or_else(|e| panic!("{}: {e}", sc.task));
            }
        }
    }
}

#[test]
fn gold_answers_pass_for_every_base_and_variant() {
    let mut n = 0;
    for sc in corpus() {
        let al = Aliases::from_facts(&sc.aliases);
        for k in std::iter::once(None).chain((0..sc.variants.len()).map(Some)) {
            let g = gold(&sc, k);
            let lacks = k.map(|k| lacked(&sc.variants[k])).unwrap_or_default();
            let r = check(&answer_of(&g), &g, &al, &lacks);
            assert!(r.strict_pass, "{} {:?}: {:?}", sc.task, k, r);
            assert_eq!(r.all_found, r.all_total, "{} {:?}", sc.task, k);
            assert!((r.precision() - 1.0).abs() < 1e-9, "{} {:?}", sc.task, k);
            n += 1;
        }
    }
    assert!(n > 150, "only {n} cases");
}

#[test]
fn lines_round_trip() {
    for sc in corpus() {
        let g = gold(&sc, None);
        let again = answer_of(&g);
        assert_eq!(lines::print(&g, &[], &[], &[]), lines::print(&again, &[], &[], &[]), "{}", sc.task);
    }
}

fn potato() -> Scenario {
    Scenario::load(&kb3::repo_root().join("data/curriculum3/dev/bake_potato.md")).unwrap()
}

#[test]
fn a_missing_step_fails() {
    let sc = potato();
    let g = gold(&sc, None);
    let mut a = answer_of(&g);
    a.steps.remove(1);
    let r = check(&a, &g, &Aliases::from_facts(&sc.aliases), &[]);
    assert!(!r.strict_pass);
}

#[test]
fn an_extra_step_fails() {
    let sc = potato();
    let g = gold(&sc, None);
    let mut a = answer_of(&g);
    let mut extra = a.steps[0].clone();
    extra.action = "dance".into();
    extra.id = "bake_potato_s99".into();
    a.steps.push(extra);
    assert!(!check(&a, &g, &Aliases::from_facts(&sc.aliases), &[]).strict_pass);
}

#[test]
fn a_wrong_order_fails() {
    let sc = potato();
    let g = gold(&sc, None);
    let mut a = answer_of(&g);
    let n = a.steps.len();
    a.steps.swap(0, n - 1);
    let r = check(&a, &g, &Aliases::from_facts(&sc.aliases), &[]);
    assert!(!r.strict_pass);
    assert!(!r.wrong.is_empty());
}

#[test]
fn a_fact_dump_fails() {
    // every step of every task: core recall is complete, but the dump must not pass
    let sc = potato();
    let g = gold(&sc, None);
    let mut a = answer_of(&g);
    for other in corpus().iter().take(10) {
        for s in gold(other, None).steps {
            a.steps.push(s);
        }
        a.needs.extend(gold(other, None).needs);
    }
    let r = check(&a, &g, &Aliases::from_facts(&sc.aliases), &[]);
    assert_eq!(r.core_found, r.core_total);
    assert!(!r.strict_pass);
    assert!(r.precision() < 0.5);
}

#[test]
fn using_a_lacked_item_is_wrong() {
    // gold without the fork (its step dropped); an answer that still uses the fork is wrong
    let sc = potato();
    let base = gold(&sc, None);
    let g = base.apply(&[Fact::new("drop", &["bake_potato_s3"]), Fact::new("no_need", &["fork"])]);
    let a = answer_of(&base);
    let r = check(&a, &g, &Aliases::from_facts(&sc.aliases), &["fork".into()]);
    assert!(r.wrong.iter().any(|w| w.contains("fork")));
}

#[test]
fn a_wrong_time_is_a_wrong_claim() {
    let sc = potato();
    let g = gold(&sc, None);
    let mut a = answer_of(&g);
    a.steps[0].time = Some(("99".into(), "99".into(), "minute".into()));
    let r = check(&a, &g, &Aliases::from_facts(&sc.aliases), &[]);
    assert!(!r.strict_pass);
}

#[test]
fn aliases_accept_the_same_thing() {
    let al = Aliases::from_facts(&[Fact::new("alias", &["baking_tray", "\"baking sheet\""])]);
    assert!(al.same("baking_tray", "baking_sheet"));
    assert!(!al.same("baking_tray", "oven"));
}

#[test]
fn applying_the_potato_variant_adds_the_microwave_step() {
    let sc = potato();
    let g = gold(&sc, Some(0));
    assert!(g.needs.contains("microwave"));
    assert!(g.steps.iter().any(|s| s.action == "microwave"));
    let mw = g.steps.iter().position(|s| s.action == "microwave").unwrap();
    let bake = g.steps.iter().position(|s| s.action == "bake").unwrap();
    assert!(mw < bake);
}
