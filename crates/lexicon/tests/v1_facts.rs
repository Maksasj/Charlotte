//! Design 2.0, milestone 13 acceptance: the registry's type checks reject 20
//! ill-typed facts taken from the v1 vault, and accept 20 well-typed ones
//! from the same vault.

use sla_lexicon::Registry;

fn section(src: &str, title: &str) -> Vec<String> {
    let mut on = false;
    let mut out = Vec::new();
    for l in src.lines() {
        if let Some(t) = l.strip_prefix("## ") {
            on = t.starts_with(title);
            continue;
        }
        if on {
            if let Some(f) = l.strip_prefix("- ") {
                out.push(f.trim().to_string());
            }
        }
    }
    out
}

fn data() -> String {
    std::fs::read_to_string(format!("{}/../../data/synthetic/v1_types.md", env!("CARGO_MANIFEST_DIR"))).unwrap()
}

#[test]
fn rejects_twenty_ill_typed_v1_facts() {
    let r = Registry::builtin();
    let bad = section(&data(), "Ill-typed");
    assert_eq!(bad.len(), 20);
    for f in &bad {
        let g = sla_core::parse_ground(f).unwrap_or_else(|e| panic!("{f}: {e}"));
        match r.check(&g) {
            Ok(()) => panic!("accepted an ill-typed fact: {f}"),
            Err(e) => println!("{f}\n  rejected: {e}"),
        }
    }
}

#[test]
fn accepts_twenty_well_typed_v1_facts() {
    let r = Registry::builtin();
    let good = section(&data(), "Well-typed");
    assert_eq!(good.len(), 20);
    for f in &good {
        let g = sla_core::parse_ground(f).unwrap();
        assert_eq!(r.check(&g), Ok(()), "{f}");
    }
}
