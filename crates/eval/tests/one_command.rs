//! Milestone 10 acceptance: one command runs the full evaluation and appends
//! a row. (This test runs the fixed parts offline; the real run with Claude
//! teachers and Haiku judges is `cargo run --release -p sla_eval`.)

use sla_eval::{run, Check, Dialogue, EvalConfig, EvalReport};

#[test]
fn run_appends_a_row() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..");
    let mut cfg = EvalConfig::new(&root);
    cfg.use_claude = false;
    let dir = std::env::temp_dir().join(format!("sla_eval_test_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    cfg.history = dir.join("history.csv");
    cfg.notes = "offline test".into();
    let rep1 = run(&cfg).unwrap();
    let rep2 = run(&cfg).unwrap();
    let csv = std::fs::read_to_string(&cfg.history).unwrap();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 3, "header + two rows:\n{csv}");
    assert_eq!(lines[0], EvalReport::HEADER);
    assert_eq!(lines[1].split(',').count(), EvalReport::HEADER.split(',').count());
    assert!(rep1.parser_dev > 0.9 && rep2.parser_dev == rep1.parser_dev, "fixed tests are repeatable");
    assert!(dir.join("last_report.md").exists());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn dialogue_format() {
    let d = Dialogue::parse(
        "# Dialogue: tea\ndomain: food\n\n## script\n- Anna likes tea.\n- Does Anna like tea? => yes\n- What does Anna like? => contains: tea\n- Does Bob like tea? => unknown\n\n## expect facts\n- likes(anna, tea)\n\n## expect rules\n- wants(A, tea) :- likes(A, tea).\n",
        "x",
    );
    assert_eq!(d.name, "tea");
    assert_eq!(d.script.len(), 4);
    assert_eq!(d.script[1].1, Some(Check::Yes));
    assert_eq!(d.facts.len(), 1);
    assert_eq!(d.rules.len(), 1);
    assert!(d.errors.is_empty());
    let r = sla_eval::run_dialogue(&d, None, 0);
    assert_eq!(r.checks, (3, 3), "{:?}", r.failures);
    assert_eq!(r.facts, (1, 1));
    assert_eq!(r.rules, (0, 1), "the rule was never taught");
}

#[test]
fn checks() {
    assert!(Check::Yes.passes("Yes, Anna likes tea."));
    assert!(Check::Yes.passes("That's right: Anna likes tea."));
    assert!(!Check::Yes.passes("No, Anna does not like tea."));
    assert!(Check::Unknown.passes("I'm not sure whether Bob likes tea."));
    assert!(Check::parse("contains: Tea; anna").unwrap().passes("Anna likes tea."));
    assert!(Check::parse("not: coffee").unwrap().passes("Anna likes tea."));
}
