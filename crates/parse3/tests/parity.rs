//! Milestone 23 acceptance: Rust predictions equal the Python trainer's on every dev line.

use parse3::{tokenize, Models};
use std::collections::BTreeSet;

#[derive(serde::Deserialize)]
struct Row {
    toks: Vec<String>,
    known: Vec<String>,
    tags: Vec<String>,
}

#[test]
fn rust_tags_equal_python_tags_on_dev() {
    let root = kb3::repo_root();
    let m = Models::load(&Models::default_path()).expect("run tools/design3/train_parser.py first");
    let text = std::fs::read_to_string(root.join("data/training/parser3/parity.jsonl")).unwrap();
    let mut n = 0;
    for line in text.lines() {
        let r: Row = serde_json::from_str(line).unwrap();
        let known: BTreeSet<String> = r.known.into_iter().collect();
        let (tags, _) = m.teach.tag(&r.toks, &known);
        assert_eq!(tags, r.tags, "{:?}", r.toks);
        n += 1;
    }
    assert!(n >= 100);
}

#[test]
fn tokenizer_matches_python() {
    assert_eq!(tokenize("Step 1 of baking a potato: heat the oven to 200 degrees."), vec![
        "step", "1", "of", "baking", "a", "potato", ":", "heat", "the", "oven", "to", "200", "degrees", "."
    ]);
    assert_eq!(tokenize("I don't have a whisk!"), vec!["i", "don't", "have", "a", "whisk", "!"]);
}

#[test]
fn a_teach_line_becomes_step_facts() {
    let m = Models::load(&Models::default_path()).unwrap();
    let known = m.known_words("To bake a potato you need a potato, olive oil, salt, a fork, a baking tray and an oven.");
    let p = m.parse_teach("bake_potato", "While step 1 runs, step 3 of baking a potato: prick the potato all over with the fork.", &known);
    let s: Vec<String> = p.facts.iter().map(|f| f.to_string()).collect();
    assert!(s.contains(&"action(bake_potato_s3, prick).".to_string()), "{s:?}");
    assert!(s.contains(&"overlaps(bake_potato_s3, bake_potato_s1).".to_string()), "{s:?}");
}
