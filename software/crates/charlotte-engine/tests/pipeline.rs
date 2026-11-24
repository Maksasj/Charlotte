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
    corpus_with(vault, n, false)
}
fn corpus_with(vault: &Path, n: usize, calls: bool) -> Corpus {
    let v = Vault::open(vault).unwrap();
    let para = paraphrases_from_yaml(&std::fs::read_to_string(repo().join("data/paraphrases.yaml")).unwrap()).unwrap();
    let convs = simulate(&v, &default_layers(&v), &para, &SimOptions { n, per_shape: true, seed: 11, corrections: 0.3, calls });
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
    // One shared word is not enough to reuse a reply.
    assert_eq!(engine.conversation().say("weekend").text, "I don't know how to help with that yet.");
    // Teaching the same chats again adds nothing.
    engine.learn(&Corpus::parse("test-chats", &jsonl), &LearnOptions::default(), &|_| {}).unwrap();
    let note = std::fs::read_to_string(dir.join("chats/chats-weekend-plans.md")).unwrap();
    assert_eq!(note.matches("Entry ").count(), 1, "{note}");
}

#[test]
fn seed_and_grow_finds_wordings_that_draw_the_same_replies() {
    let dir = untrained("grow");
    let vault = Vault::open(&dir).unwrap();
    let conv = |a: &str, b: &str| format!(r#"{{"turns":[{{"role":"user","text":"{a}"}},{{"role":"agent","text":"{b}"}}]}}"#);
    let mut lines = Vec::new();
    let thanks = ["thanks a lot", "thank you so much", "thanks again", "thank you very much"];
    let welcome = ["You're welcome!", "You're welcome, anytime.", "My pleasure."];
    let chatter = ["What a nice day", "Did you see the match", "The bus was late again", "I like that song"];
    let other = ["Yes it is.", "No, I missed it.", "Typical.", "Which one?", "Me too.", "Really?", "Oh dear."];
    for i in 0..400 {
        lines.push(conv(thanks[i % 4], welcome[i % 3]));
        // A wording of the same act, and a topic that draws all kinds of replies.
        if i % 4 == 0 || i % 7 == 0 {
            lines.push(conv(if i % 2 == 0 { "much obliged to you" } else { "much obliged my friend" }, welcome[(i + 1) % 3]));
        }
        for k in 0..10 {
            lines.push(conv(chatter[(i + k) % 4], other[(i * 3 + i / 7 + k) % 7]));
            lines.push(conv("what is on the menu today", other[(i * 5 + i / 3 + k) % 7]));
        }
    }
    let corpus = Corpus::parse("grow-test", &lines.join("\n"));
    let ex = charlotte_engine::grow::exchanges(&corpus);
    let growth = charlotte_engine::grow::grow(&vault, &ex, &Default::default(), Some("thanks"));
    let g = &growth[0];
    assert!(g.skipped.is_none(), "{g:?}");
    let found: Vec<&str> = g.grown.iter().map(|x| x.phrase.as_str()).collect();
    assert!(found.iter().any(|p| p.contains("obliged")), "{found:?} {g:?}");
    assert!(!found.iter().any(|p| p.contains("menu") || p.contains("day") || p.contains("match")), "{found:?}");
    // Nothing is promoted: the result is a candidate note.
    let note = charlotte_engine::grow::candidate_note(g, "grow-test").unwrap();
    assert!(note.path.starts_with("candidates/"));
}

#[test]
fn exchange_types_find_a_new_act_and_ignore_a_topic() {
    use charlotte_engine::exchange_types::{candidate_notes, induce, TypeOptions};
    use charlotte_engine::grow::Exchange;
    let dir = untrained("types");
    let vault = Vault::open(&dir).unwrap();
    let names = ["Ann", "Bob", "Carla", "Dev", "Eva", "Finn", "Gus", "Hana", "Ivo", "Jun"];
    let intro = ["my name is", "i am called", "they call me"];
    let nice = ["Nice to meet you", "Pleased to meet you", "Lovely to meet you"];
    let filler = ["it rained", "the bus", "a song", "lunch", "my cat", "the match", "a book", "the shop", "this film", "that road", "our trip", "his car"];
    let verbs = ["I saw", "Did you hear", "We talked about", "Remember", "Funny how", "Strange that"];
    let mut ex = Vec::new();
    let mut seed = 12345u64;
    let mut pick = |n: usize| {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((seed >> 33) as usize) % n
    };
    for i in 0..1500 {
        let mut add = |conv: usize, m: String, r: String| ex.push(Exchange { conv, message: m, reply: r, user: true });
        if i % 12 == 0 {
            let name = names[pick(10)];
            add(i * 3, format!("{} {}", intro[pick(3)], name), format!("{}, {}!", nice[pick(3)], name));
        }
        // A topic: every reply says "football", each says something else about it.
        if i % 10 == 0 {
            add(i * 3 + 1, format!("tell me about {} please", filler[pick(12)]), format!("{} {} and football {}", verbs[pick(6)], filler[pick(12)], filler[pick(12)]));
        }
        add(i * 3 + 2, format!("{} {}", filler[pick(12)], filler[pick(12)]), format!("{} {} {}", verbs[pick(6)], filler[pick(12)], filler[pick(12)]));
    }
    let types = induce(&vault, &ex, &TypeOptions { min_group: 20, ..Default::default() });
    let new: Vec<_> = types.iter().filter(|t| t.known.is_none() && t.skipped.is_none()).collect();
    assert!(new.iter().any(|t| t.skeletons.iter().any(|s| s.phrase.contains("name") || s.phrase.contains("called") || s.phrase.contains("call me"))), "{types:#?}");
    assert!(!types.iter().any(|t| t.skipped.is_none() && t.skeletons.iter().any(|s| s.phrase.contains("football") || s.phrase.contains("tell me"))), "{types:#?}");
    let t = new[0];
    assert!(t.echo.is_some(), "{t:?}");
    let notes = candidate_notes(t, "test");
    assert_eq!(notes.len(), 2);
    assert!(notes.iter().all(|n| n.path.starts_with("candidates/") && n.front["status"] == "candidate"));
}

#[test]
fn a_request_that_needs_a_record_calls_a_lookup_subframe() {
    let dir = untrained("calls");
    let engine = Engine::open(&dir).unwrap();
    let report = engine.learn(&corpus_with(&dir, 300, true), &LearnOptions::default(), &|_| {}).unwrap();
    assert!(report.held_out.after >= 0.95, "{}", report.held_out.after);
    let mut c = engine.conversation();
    let reply = c.say("my order 8813 arrived late, can I get compensation?");
    assert!(reply.trace.requests.iter().flat_map(|r| &r.steps).any(|s| s.act.op == charlotte_core::Op::Call));
    assert!(reply.text.contains("delay"), "{}", reply.text);
}

fn subject(r: &charlotte_core::Reply) -> String {
    match r.trace.requests[0].frame.as_ref().and_then(|f| f.facts.get("subject")) {
        Some(charlotte_core::Value::Str(s)) => s.clone(),
        v => format!("{v:?}"),
    }
}
#[test]
fn a_reference_resolves_to_a_subject_from_earlier_in_the_conversation() {
    let e = trained();
    let mut c = e.conversation();
    c.say("what is the status of order 8813?");
    c.say("and what about order 4410?");
    // "that order" is the one in focus; "the other one" is the one before it.
    let same = c.say("is that order still on time? what is its status");
    assert_eq!(subject(&same), "4410");
    let other = c.say("what is the status of the other order");
    assert_eq!(subject(&other), "8813");
    // With nothing to point at, it asks instead of guessing.
    let fresh = e.conversation().say("what is the status of that order").text;
    assert!(fresh.to_lowercase().contains("which"), "{fresh}");
}

fn chat(lines: &[&str]) -> String {
    let turns: Vec<_> = lines.iter().map(|t| serde_json::json!({ "role": "user", "text": t })).collect();
    serde_json::json!({ "turns": turns }).to_string() + "\n"
}

#[test]
fn split_connectors_are_found_in_chats_and_promoted_to_the_lexicon() {
    let dir = untrained("connectors");
    let mut jsonl = String::new();
    for i in 0..40 {
        jsonl += &chat(&[&format!("I want a refund for order {}, on top of that cancel my subscription", 1000 + i)]);
        jsonl += &chat(&["I want a refund and my money back", "please cancel it and also thanks"]);
    }
    let mut vault = Vault::open(&dir).unwrap();
    let found = charlotte_engine::connectors::induce(&vault, &Corpus::parse("test", &jsonl), &Default::default());
    let phrases: Vec<&str> = found.iter().map(|c| c.phrase.as_str()).collect();
    assert!(phrases.contains(&"on top of that"), "{phrases:?}");
    // "and" is everywhere, not between requests; "also" is already a connector.
    assert!(!phrases.contains(&"and") && !phrases.contains(&"also"), "{phrases:?}");
    let note = charlotte_engine::connectors::candidate_note(&found, "test").unwrap();
    vault.put(note).unwrap();
    let engine = Engine::with_layers(vault, Box::new(default_layers));
    engine.promote("phrases-connectors").unwrap();
    assert!(engine.vault(|v| charlotte_core::Memory::lexicon(v).connectors.iter().any(|c| c == "on top of that")));
}

#[test]
fn a_user_contradicting_themselves_is_asked_which_value_is_right() {
    let e = trained();
    let mut c = e.conversation();
    let r = c.say("I want a refund for order 7741, it arrived 5 days ago, no wait, 40 days ago");
    assert!(r.text.contains("two different values for days (5 and 40)"), "{}", r.text);
    // Settling it removes the conflict; nothing was granted or denied on a guess.
    let r = c.say("3 days");
    assert!(!r.text.contains("two different values"), "{}", r.text);
    assert_eq!(r.trace.requests[0].frame.as_ref().unwrap().facts.get("days").map(|v| v.text()), Some("3".into()));
}

#[test]
fn atoms_that_contradict_each_other_are_reported() {
    let dir = untrained("atoms");
    let mut v = Vault::open(&dir).unwrap();
    assert!(v.atom_conflicts().is_empty(), "{:?}", v.atom_conflicts());
    let atom = |name: &str, result: &str, cond: &str| {
        let text = format!("---\nkind: atom\nintent: '[[refund]]'\nobject: order\npriority: 9\nresult: {result}\nconditions:\n{cond}\ntrust: 0.8\n---\nok\n");
        charlotte_vault::Note::parse(&format!("atoms/{name}.md"), &text).unwrap()
    };
    let cond = |op: &str, n: u32| format!("- slot: '[[days]]'\n  op: '{op}'\n  value: {n}\n");
    v.put(atom("x-early", "money_back", &cond("<=", 30))).unwrap();
    v.put(atom("x-mid", "store_credit", &cond("<=", 60))).unwrap();
    v.put(atom("x-late", "nothing", &cond(">", 90))).unwrap();
    v.put(atom("x-never", "nothing", &(cond(">=", 10) + &cond("<=", 5)))).unwrap();
    let found = v.atom_conflicts();
    assert!(found.iter().any(|f| f.contains("x-early") && f.contains("x-mid")), "{found:?}");
    assert!(found.iter().any(|f| f.contains("x-never") && f.contains("never apply")), "{found:?}");
    // Disjoint conditions do not conflict.
    assert!(!found.iter().any(|f| f.contains("x-early") && f.contains("x-late")), "{found:?}");
}

#[test]
fn a_taught_reply_is_a_candidate_until_promoted() {
    let dir = untrained("teach");
    let engine = Engine::open(&dir).unwrap();
    let mut c = engine.conversation();
    let first = c.say("do you ship to norway");
    assert_eq!(first.text, "I don't know how to help with that yet.");
    let item = c.teach("Yes, we ship to Norway.").unwrap();
    assert_eq!(item.status, "candidate");
    assert_eq!(engine.conversation().say("do you ship to norway").text, first.text, "a candidate is not used");
    engine.promote(&item.id).unwrap();
    assert_eq!(engine.conversation().say("do you ship to norway").text, "Yes, we ship to Norway.");
    assert!(engine.conversation().teach("anything").is_err(), "nothing said yet");
}
