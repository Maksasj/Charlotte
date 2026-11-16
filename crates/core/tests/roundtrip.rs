//! Milestone 1 acceptance: parse, modify and write back 100 generated files
//! with no diffs except the intended ones.

use sla_core::doc::Item;
use sla_core::term::{Fact, GroundAtom, Value};
use sla_core::vault::{RuleFile, RuleMeta, RuleStatus, Vault};
use sla_core::{parse_rule, Timestamp};
use std::fs;
use std::path::PathBuf;

/// Small deterministic generator (no external crates).
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }
    fn pick<'a>(&mut self, xs: &'a [&'a str]) -> &'a str {
        xs[(self.next() as usize) % xs.len()]
    }
}

const ENTITIES: &[&str] = &[
    "maksim", "anna", "bob", "paris", "kitchen", "hammer", "coffee", "office", "rex", "berlin",
    "laptop", "garden", "meeting", "concert", "tomato", "bicycle", "library", "lisa", "omar", "river",
];
const PREDS: &[&str] = &["likes", "has", "is_a", "part_of", "located_in", "wants", "knows", "feels"];
const OBJECTS: &[&str] = &["coffee", "tea", "dog", "city", "tool", "red", "happy", "work", "music", "Big Apple"];

fn temp_dir(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("sla_{name}_{}", std::process::id()));
    let _ = fs::remove_dir_all(&d);
    fs::create_dir_all(&d).unwrap();
    d
}

fn changed_lines(a: &str, b: &str) -> Vec<(String, String)> {
    assert_eq!(a.lines().count(), b.lines().count(), "line count changed");
    a.lines().zip(b.lines()).filter(|(x, y)| x != y).map(|(x, y)| (x.to_string(), y.to_string())).collect()
}

#[test]
fn hundred_files_roundtrip() {
    let root = temp_dir("roundtrip");
    let mut rng = Lcg(42);
    let base = Timestamp::from_ymd(2026, 1, 1);

    // 70 fact files (one per entity, with variety) + 30 rule files.
    let mut vault = Vault::empty(&root);
    for i in 0..70 {
        let ent = format!("{}{}", rng.pick(ENTITIES), i);
        let n = 1 + rng.next() % 8;
        for j in 0..n {
            let obj = rng.pick(OBJECTS);
            let mut f = Fact::new(GroundAtom::new(rng.pick(PREDS), vec![Value::sym(&ent), Value::sym(obj)]), rng.pick(&["user", "teacher", "synthetic data"]));
            f.from = Some(base.plus_secs((rng.next() % 10_000_000) as i64));
            if j % 3 == 0 {
                f.until = Some(f.from.unwrap().plus_days(1 + (rng.next() % 30) as i64));
            }
            f.conf = (rng.next() % 1000) as f64 / 1000.0;
            if j % 4 == 0 {
                f.uses = rng.next() % 20;
                f.last_used = Some(base.plus_days(40));
            }
            vault.add_fact(f);
        }
    }
    for i in 0..30 {
        let p = rng.pick(PREDS);
        let q = rng.pick(PREDS);
        let meta = RuleMeta {
            id: format!("rule_{i}"),
            status: if i % 2 == 0 { RuleStatus::Hypothesis } else { RuleStatus::Confirmed },
            conf: (rng.next() % 1000) as f64 / 1000.0,
            evidence: rng.next() % 9,
            counter: rng.next() % 3,
            module: rng.pick(&["food", "people", "places", "general"]).to_string(),
            source: "learner".into(),
        };
        let rules = vec![
            parse_rule(&format!("{p}(X, Y) :- {q}(X, Z), {p}(Z, Y).")).unwrap(),
            parse_rule(&format!("{q}(X, Y) :- {p}(Y, X), not -{q}(X, Y), X != Y.")).unwrap(),
        ];
        vault.put_rule_file(RuleFile::new(&meta, &format!("Generated rule group {i}."), &rules));
    }
    vault.save_all().unwrap();

    let count = |d: &str| fs::read_dir(root.join(d)).unwrap().count();
    assert_eq!(count("facts") + count("rules"), 100);

    // Snapshot, reload, check identity.
    let snapshot: Vec<(PathBuf, String)> = ["facts", "rules"]
        .iter()
        .flat_map(|d| fs::read_dir(root.join(d)).unwrap().map(|e| e.unwrap().path()))
        .map(|p| {
            let s = fs::read_to_string(&p).unwrap();
            (p, s)
        })
        .collect();
    let mut v2 = Vault::load(&root).unwrap();
    assert!(v2.errors().is_empty(), "{:?}", v2.errors());
    for (p, s) in &snapshot {
        let stem = p.file_stem().unwrap().to_str().unwrap();
        let rendered = if p.parent().unwrap().ends_with("facts") {
            v2.facts[stem].render()
        } else {
            v2.rules[stem].render()
        };
        assert_eq!(&rendered, s, "unmodified render differs for {p:?}");
    }

    // Modify exactly one item per file: first fact's conf, rule meta's evidence.
    let fact_stems: Vec<String> = v2.facts.keys().cloned().collect();
    for stem in &fact_stems {
        let mut done = false;
        v2.facts.get_mut(stem).unwrap().doc.update_items(|i| match i {
            Item::Fact(f) if !done => {
                done = true;
                f.conf = if f.conf > 0.5 { 0.25 } else { 0.75 };
                true
            }
            _ => false,
        });
        v2.mark_fact_dirty(stem);
    }
    let rule_stems: Vec<String> = v2.rules.keys().cloned().collect();
    for stem in &rule_stems {
        let rf = v2.rules.get_mut(stem).unwrap();
        let mut m = rf.meta();
        m.evidence += 1;
        rf.set_meta(&m);
        v2.mark_rule_dirty(stem);
    }
    v2.save().unwrap();

    for (p, before) in &snapshot {
        let after = fs::read_to_string(p).unwrap();
        let diff = changed_lines(before, &after);
        assert_eq!(diff.len(), 1, "{p:?}: {diff:?}");
        let (old, new) = &diff[0];
        if p.parent().unwrap().ends_with("facts") {
            assert!(old.contains("@conf") && new.contains("@conf"), "{old} -> {new}");
            let a = sla_core::parse_fact_line(old).unwrap();
            let b = sla_core::parse_fact_line(new).unwrap();
            assert_eq!(a.atom, b.atom);
            assert_eq!(a.from, b.from);
            assert_eq!(a.until, b.until);
            assert_ne!(a.conf, b.conf);
        } else {
            assert!(old.starts_with("evidence: ") && new.starts_with("evidence: "), "{old} -> {new}");
        }
    }

    // Hand edit survives: append a comment and a fact by hand, reload, re-save untouched.
    let hand = root.join("facts").join(format!("{}.md", fact_stems[0]));
    let mut s = fs::read_to_string(&hand).unwrap();
    s.push_str("\nHand-written note.\n\n```datalog\n% added by hand\nlikes(x, y).   @conf 1 @source hand\n```\n");
    fs::write(&hand, &s).unwrap();
    let mut v3 = Vault::load(&root).unwrap();
    assert!(v3.facts[&fact_stems[0]].doc.facts().any(|f| f.source == "hand"));
    v3.mark_fact_dirty(&fact_stems[0]);
    v3.save().unwrap();
    assert_eq!(fs::read_to_string(&hand).unwrap(), s);

    let _ = fs::remove_dir_all(&root);
}
