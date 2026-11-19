//! Milestone 33 acceptance: replay is byte-identical to the live run; a migration renaming one
//! relation rebuilds the same answers; nothing outside the agent can write knowledge; the deny
//! filter catches planted leaks at P1-P5 and passes clean text.

use agent4::deny::Deny;
use agent4::kb;
use eval4::{deliver, Fixture, Opts};

fn tmp(name: &str) -> std::path::PathBuf {
    let d = std::env::temp_dir().join(format!("kb4_{name}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    d
}

fn fixture() -> Option<Fixture> {
    let dir = eval4::root().join("data/spike4");
    dir.join("bench/questions.jsonl").exists().then(|| Fixture::load(&dir).unwrap())
}

fn opts() -> Opts {
    Opts { learn: true, naming: true, prefix: 1.0, min_support: 3, ..Default::default() }
}

#[test]
fn replay_is_byte_identical_to_the_live_run() {
    let Some(fx) = fixture() else { return };
    let root = tmp("replay");
    let live = deliver(&fx, &Opts { session: Some("s0001".into()), ..opts() }).agent;
    kb::write_raw(&root, &live, "fixture").unwrap();
    kb::write_build(&root.join("builds/live"), &live, live.raw.len()).unwrap();
    let rows = kb::read_lines(&root.join("raw/sessions/s0001.jsonl"));
    let (again, unanswered) = kb::replay(&rows, "s0001", &Default::default(), &[], true, true);
    kb::write_build(&root.join("builds/replay"), &again, rows.len()).unwrap();
    for f in ["facts.dl", "rules.dl", "vocab.jsonl", "provenance.jsonl", "BUILD.json"] {
        let a = std::fs::read(root.join("builds/live").join(f)).unwrap();
        let b = std::fs::read(root.join("builds/replay").join(f)).unwrap();
        assert!(a == b, "{f} differs between live and replay");
    }
    assert!(unanswered.is_empty() || unanswered.len() * 20 <= live.questions.len(), "under-answered replay");
}

#[test]
fn a_migration_renaming_a_relation_rebuilds_the_same_answers() {
    let Some(fx) = fixture() else { return };
    let mut live = deliver(&fx, &Opts { session: Some("s0002".into()), ..opts() }).agent;
    let rows: Vec<serde_json::Value> = kb::raw_records(&live, "fixture")
        .into_iter()
        .map(|mut r| {
            // the old raw wrote `designed_for` where schema v4 says `used_for`
            let t = r["text"].as_str().unwrap_or("").to_string();
            if let Some(rest) = t.strip_prefix("used_for(") {
                r["text"] = serde_json::json!(format!("designed_for({rest}"));
            }
            r
        })
        .collect();
    let renames: std::collections::BTreeMap<String, String> = [("designed_for".to_string(), "used_for".to_string())].into();
    let (mut migrated, _) = kb::replay(&rows, "s0002", &renames, &[], true, true);
    for q in &fx.questions {
        let a: Vec<String> = live.respond(&q.query).claims.into_iter().map(|c| c.0).collect();
        let b: Vec<String> = migrated.respond(&q.query).claims.into_iter().map(|c| c.0).collect();
        assert_eq!(a, b, "{}", q.id);
    }
}

#[test]
fn nothing_outside_the_agent_writes_knowledge() {
    // the only writers of facts are the agent's own entry point and learner (no-injection, P2)
    let root = eval4::root().join("crates");
    for c in ["eval4", "reason4"] {
        for e in std::fs::read_dir(root.join(c).join("src")).unwrap().flatten() {
            let src = std::fs::read_to_string(e.path()).unwrap();
            for bad in [".facts.push(", ".facts.insert(", ".facts =", ".facts.extend(", ".learned.push("] {
                assert!(!src.contains(bad), "{} writes knowledge directly ({bad})", e.path().display());
            }
        }
    }
}

#[test]
fn the_deny_filter_catches_planted_leaks_at_every_point() {
    let deny = Deny::new(["can_go_in(muffin_tray, oven)".to_string(), "stand_in(bake_s4, loaf_tin, muffin_tray)".to_string()]);
    // P1: a teacher prompt that mentions a denied atom
    assert!(!deny.hits("Teach baking tins. Known: can_go_in(muffin_tray, oven).").is_empty());
    assert!(deny.hits("Teach baking tins. Known: is_a(muffin_tray, baking_tin).").is_empty());
    // P2: a teacher reply; the denied statement is withheld, the rest delivered
    let (keep, withheld) = deny.filter_reply("is_a(muffin_tray, baking_tin).\ncan_go_in(muffin_tray, oven).\n");
    assert_eq!(keep.len(), 1);
    assert_eq!(withheld.len(), 1);
    // P3: an agent question about a denied atom is dropped
    assert!(!deny.hits("ask: can_go_in(muffin_tray, oven)?").is_empty());
    // P4: an archive statement
    assert!(!deny.hits("not can_go_in(muffin_tray, oven).").is_empty());
    // P5: the audit of delivered facts
    let delivered = ["is_a(muffin_tray, baking_tin)", "can_go_in(muffin_tray, oven)"];
    assert_eq!(delivered.iter().filter(|d| !deny.hits(d).is_empty()).count(), 1);
}
