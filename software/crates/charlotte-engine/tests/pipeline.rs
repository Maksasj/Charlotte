//! The pass/fail checks from the build order in software/docs/design.md.
//! They run on scratch copies of the vault, trained from simulated conversations, so they do not
//! depend on what the committed vault happens to have learned.

use charlotte_engine::learn::{Corpus, LearnOptions};
use charlotte_engine::sim::{paraphrases_from_yaml, simulate, SimOptions};
use charlotte_engine::{default_layers, Engine};
use charlotte_vault::Vault;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}
fn copy(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for e in std::fs::read_dir(from).unwrap().map(|e| e.unwrap().path()) {
        let dest = to.join(e.file_name().unwrap());
        if e.is_dir() {
            copy(&e, &dest)
        } else {
            std::fs::copy(&e, &dest).unwrap();
        }
    }
}
/// A copy of the vault with everything learned removed.
fn untrained(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("charlotte-test-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    copy(&repo().join("vault"), &dir);
    let _ = std::fs::remove_dir_all(dir.join("patterns"));
    let _ = std::fs::remove_dir_all(dir.join("candidates"));
    let _ = std::fs::remove_dir_all(dir.join("chats"));
    dir
}
fn corpus(vault: &Path, n: usize) -> Corpus {
    let v = Vault::open(vault).unwrap();
    let para = paraphrases_from_yaml(&std::fs::read_to_string(repo().join("data/paraphrases.yaml")).unwrap()).unwrap();
    let convs = simulate(&v, &default_layers(&v), &para, &SimOptions { n, per_shape: true, seed: 11, corrections: 0.3 });
    let jsonl: String = convs.iter().map(|c| serde_json::to_string(c).unwrap() + "\n").collect();
    Corpus::parse("test", &jsonl)
}
/// An engine trained once on 400 simulated conversations of each shape.
fn trained() -> &'static Engine {
    static E: OnceLock<Engine> = OnceLock::new();
    E.get_or_init(|| {
        let dir = untrained("trained");
        let engine = Engine::open(&dir).unwrap();
        let report = engine.learn(&corpus(&dir, 400), &LearnOptions::default(), &|_| {}).unwrap();
        assert!(report.held_out.after >= 0.98, "held-out conversations reproduced: {}", report.held_out.after);
        engine
    })
}
fn last(engine: &Engine, messages: &[&str]) -> String {
    let mut c = engine.conversation();
    messages.iter().map(|m| c.say(m).text).last().unwrap()
}

#[test]
fn step1_writing_the_vault_back_changes_nothing() {
    let v = Vault::open(repo().join("vault")).unwrap();
    assert!(v.lint().is_empty(), "{:?}", v.lint());
    for n in v.notes.values() {
        assert_eq!(std::fs::read_to_string(v.root.join(&n.path)).unwrap(), n.render(), "{}", n.path);
    }
}

#[test]
fn step4_without_patterns_it_declines_and_learning_restores_them() {
    let dir = untrained("restore");
    let engine = Engine::open(&dir).unwrap();
    let reply = engine.conversation().say("I want a refund for order 7741, it arrived 5 days ago and is still sealed");
    assert_eq!(reply.text, "I don't know how to help with that yet.");
    assert!(reply.trace.notes.iter().any(|n| n.contains("no trusted pattern")));

    engine.learn(&corpus(&dir, 400), &LearnOptions::default(), &|_| {}).unwrap();
    for p in ["check-a-rule", "walk-through-steps", "look-up-a-record"] {
        assert!(dir.join("patterns").join(format!("{p}.md")).is_file(), "{p} was not promoted");
    }
    assert!(last(&engine, &["I want a refund for order 7741, it arrived 5 days ago and is still sealed"]).starts_with("Refund approved for 7741"));
}

#[test]
fn step2_one_pattern_answers_six_domains() {
    let e = trained();
    for (q, a) in [
        ("I want a refund for order 7741, it arrived 5 days ago and is still sealed", "Refund approved for 7741"),
        ("my laptop is faulty, bought it 6 months ago, never dropped, device 204", "Replacement approved for 204"),
        ("reimburse my expense 77 of $120, receipt attached", "Reimbursement of $120 approved for 77"),
        ("leave request 9 with 20 days notice, coverage arranged", "Leave approved for 9"),
        ("enable advanced features on account 3, we are on pro and verified", "Advanced features are now enabled on 3"),
        ("upgrade account 3, 4 seats", "Upgrade to Pro is live on 3"),
    ] {
        let got = last(e, &[q]);
        assert!(got.starts_with(a), "{q}\n  got: {got}");
    }
}

#[test]
fn step2_the_v5_defects_are_gone() {
    let e = trained();
    // D1: each request keeps its own facts.
    let got = last(e, &["cancel my subscription 12 bought 5 days ago, also where is my order 5150?"]);
    assert!(got.contains("Cancellation approved for 12") && got.contains("5150 is in transit"), "{got}");
    // D2: "never dropped" is not a drop.
    assert!(last(e, &["my laptop stopped working, bought it 6 months ago, never dropped, device 204"]).starts_with("Replacement approved"));
    // D3: a late order is refused by the order rule, not asked about a subscription.
    assert!(last(e, &["refund order 7741, it arrived 45 days ago, still sealed"]).contains("days is 45"));
    // D4: an alternative is an offer chosen by the pattern.
    let reply = e.conversation().say("refund order 7741, arrived 5 days ago, I already opened it");
    assert!(reply.text.contains("store credit"), "{}", reply.text);
    assert_eq!(reply.acts[0].op, charlotte_core::Op::Offer);
}

#[test]
fn step2_conversations_over_several_turns() {
    let e = trained();
    // Questions, then a correction that overturns the decision.
    let mut c = e.conversation();
    assert_eq!(c.say("I want a refund for order 7741").text, "How many days ago was that?");
    assert_eq!(c.say("5 days ago").text, "Is it still in the original packaging, unopened?");
    assert!(c.say("yes").text.starts_with("Refund approved"));
    assert!(c.say("actually, it arrived 45 days ago").text.contains("can't approve"));
    // A fact from a record decides a rule.
    assert!(last(e, &["order 7741 arrived late, can I get compensation?"]).contains("10% credit"));
    assert!(last(e, &["order 5150 arrived late, can I get compensation?"]).contains("can't approve"));
    // A procedure with one failed step.
    let mut c = e.conversation();
    assert!(c.say("I forgot my password").text.starts_with("Step 1 of 4"));
    assert!(c.say("done").text.starts_with("Step 2 of 4"));
    assert!(c.say("didn't work").text.starts_with("Let's try that step once more"));
    assert!(c.say("didn't work").text.contains("teammate"));
}

#[test]
fn plain_chats_are_remembered_and_reused() {
    let dir = untrained("chats");
    let engine = Engine::open(&dir).unwrap();
    let before = engine.conversation().say("any plans for the weekend?").text;
    assert_eq!(before, "I don't know how to help with that yet.");

    let line = |id: &str, topic: &str, turns: &[(&str, &str)]| {
        let t: Vec<String> = turns.iter().map(|(r, x)| format!(r#"{{"role":"{r}","text":"{x}"}}"#)).collect();
        format!(r#"{{"id":"{id}","domain":"{topic}","turns":[{}]}}"#, t.join(","))
    };
    let jsonl = [
        line("w1", "weekend plans", &[("user", "Hey! Any plans for the weekend?"), ("agent", "Not really, I might just stay in and read."), ("user", "That sounds relaxing. Which book?"), ("agent", "A detective novel a friend lent me.")]),
        line("c1", "coffee and tea", &[("user", "Do you prefer coffee or tea in the morning?"), ("agent", "Coffee, always. Tea is for the afternoon.")]),
    ]
    .join("\n");
    let report = engine.learn(&Corpus::parse("test-chats", &jsonl), &LearnOptions::default(), &|_| {}).unwrap();
    assert!(report.candidates.iter().any(|c| c.kind == "chats" && c.promoted), "{report}");
    assert!(dir.join("chats/chats-weekend-plans.md").is_file());

    // A similar message gets the remembered reply, and the trace says where it came from.
    let reply = engine.conversation().say("any plans for the weekend?");
    assert_eq!(reply.text, "Not really, I might just stay in and read.");
    assert!(reply.trace.notes.iter().any(|n| n.contains("chats-weekend-plans")), "{:?}", reply.trace.notes);
    assert_eq!(engine.conversation().say("coffee or tea in the morning?").text, "Coffee, always. Tea is for the afternoon.");
    // Something it never saw is still declined, and a request the vault knows is not hijacked.
    assert_eq!(engine.conversation().say("what is the capital of Peru?").text, "I don't know how to help with that yet.");
    // Teaching the same chats again adds nothing.
    engine.learn(&Corpus::parse("test-chats", &jsonl), &LearnOptions::default(), &|_| {}).unwrap();
    let note = std::fs::read_to_string(dir.join("chats/chats-weekend-plans.md")).unwrap();
    assert_eq!(note.matches("Entry ").count(), 1, "{note}");
}
