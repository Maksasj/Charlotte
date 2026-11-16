//! Milestone 4 acceptance: the synthetic dev set (200 sentences written by a
//! separate agent from docs/parser_spec.md) parses with >= 90% correct intent
//! and slots (exact canonical match).

use sla_parser::{parse, Context};

pub fn load(path: &str) -> Vec<(String, String)> {
    let full = format!("{}/../../{path}", env!("CARGO_MANIFEST_DIR"));
    let src = std::fs::read_to_string(&full).unwrap_or_else(|e| panic!("{full}: {e}"));
    src.lines()
        .filter_map(|l| l.strip_prefix("- "))
        .filter_map(|l| l.rsplit_once(" => "))
        .map(|(s, e)| (s.trim().to_string(), e.trim().to_string()))
        .collect()
}

#[test]
fn dev_set_accuracy() {
    let cases = load("data/synthetic/parser/dev.md");
    assert_eq!(cases.len(), 200);
    let ctx = Context::default();
    let mut ok = 0;
    let mut intent_ok = 0;
    let mut fails = Vec::new();
    for (s, want) in &cases {
        let got = parse(s, &ctx).canonical();
        if got.split(':').next() == want.split(':').next() {
            intent_ok += 1;
        }
        if &got == want {
            ok += 1;
        } else {
            fails.push(format!("{s}\n    got  {got}\n    want {want}"));
        }
    }
    let acc = ok as f64 / cases.len() as f64;
    println!("intent accuracy {}/{}; exact {}/{} = {:.1}%", intent_ok, cases.len(), ok, cases.len(), acc * 100.0);
    for f in &fails {
        println!("{f}");
    }
    assert!(acc >= 0.9, "exact accuracy {:.1}% < 90%", acc * 100.0);
}

/// Held-out set: never used to tune patterns. Prints only the score (no
/// failing sentences) so it stays a fair check for overfitting.
#[test]
fn held_out_score() {
    let cases = load("data/synthetic/parser/held_out.md");
    let ctx = Context::default();
    let ok = cases.iter().filter(|(s, want)| &parse(s, &ctx).canonical() == want).count();
    let intents = cases
        .iter()
        .filter(|(s, want)| parse(s, &ctx).canonical().split(':').next() == want.split(':').next())
        .count();
    println!("held-out: intent {}/{}, exact {}/{}", intents, cases.len(), ok, cases.len());
    assert!(ok as f64 / cases.len() as f64 >= 0.75, "held-out exact accuracy below 75%: {ok}/{}", cases.len());
}
