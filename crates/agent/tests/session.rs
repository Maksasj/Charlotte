//! Milestone 6 acceptance: a scripted conversation teaches 10 facts and 2
//! rules, and a later session recalls them after restart.

use sla_agent::Agent;
use sla_core::Timestamp;
use std::fs;

fn temp_vault(name: &str) -> std::path::PathBuf {
    let d = std::env::temp_dir().join(format!("sla_m6_{name}_{}", std::process::id()));
    let _ = fs::remove_dir_all(&d);
    d
}

fn talk(agent: &mut Agent, msg: &str) -> String {
    let r = agent.respond(msg).text;
    println!("> {msg}\n{r}");
    r
}

#[test]
fn teach_then_recall_after_restart() {
    let root = temp_vault("session");
    let day1 = Timestamp::from_ymd_hms(2026, 11, 16, 9, 0, 0);
    {
        let mut a = Agent::open(&root, "maksim").unwrap();
        a.clock = Some(day1);
        assert!(talk(&mut a, "Hello!").contains('!'));
        // 10 facts.
        for msg in [
            "I like coffee.",
            "Anna likes tea.",
            "Anna lives in Berlin.",
            "Berlin is in Germany.",
            "Rex is a dog.",
            "Dogs are animals.",
            "The car is red.",
            "Bob works at Acme.",
            "Anna's job is teacher.",
            "I am tired.",
        ] {
            let r = talk(&mut a, msg);
            // An acknowledgement that does not echo the user (Design 2.0).
            assert!(["Okay", "Got it", "Noted", "Thanks, noted", "Good to know", "I'll keep", "All right"].iter().any(|l| r.starts_with(l)) && !r.contains('?'), "{msg} -> {r}");
        }
        // 2 rules.
        let r = talk(&mut a, "If someone likes tea then they want tea.");
        assert!(r.contains("rule"), "{r}");
        let r = talk(&mut a, "If something is in Berlin then it is in Germany.");
        assert!(r.contains("rule"), "{r}");
        assert_eq!(a.brain().kb.all_facts().iter().filter(|f| f.atom.pred != "proper_name").count(), 10);
        assert_eq!(a.dump_rules().len(), 2);
    }

    // Restart: a new agent on the same vault, a day later.
    let mut b = Agent::open(&root, "maksim").unwrap();
    b.clock = Some(day1.plus_days(1));
    let checks = [
        ("Do I like coffee?", "Yes"),
        ("What does Anna like?", "Anna likes tea"),
        ("Where does Anna live?", "Anna lives in Berlin"),
        ("Is Rex an animal?", "Rex is an animal"),
        ("What colour is the car?", "ar is red"),
        ("Who works at Acme?", "Bob works at Acme"),
        ("What is Anna's job?", "Anna's job is teacher"),
        ("Am I tired?", "you are tired"),
        ("Is Berlin in Germany?", "Yes"),
        ("Is Rex a dog?", "Yes"),
        // Rule 1: derived want.
        ("Does Anna want tea?", "Anna wants tea"),
        // Rule 2: derived location.
        ("Is Anna in Germany?", "Anna lives in Germany"),
    ];
    for (q, want) in checks {
        let r = talk(&mut b, q);
        assert!(r.contains(want), "{q} -> {r} (want {want:?})");
    }
    // Why answers come from the proof and mention the rule.
    let r = talk(&mut b, "Why?");
    assert!(r.contains("because") && r.contains("Berlin"), "{r}");
    // Recall.
    let r = talk(&mut b, "What do you know about Anna?");
    assert!(r.contains("likes tea") && r.contains("Berlin"), "{r}");
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn contradiction_and_history() {
    let root = temp_vault("history");
    let t = Timestamp::from_ymd_hms(2026, 11, 16, 9, 0, 0);
    let mut a = Agent::open(&root, "maksim").unwrap();
    a.clock = Some(t);
    talk(&mut a, "I like coffee.");
    a.clock = Some(t.plus_days(30));
    let r = talk(&mut a, "I don't like coffee anymore.");
    assert!(r.contains("not anymore") || r.contains("do not like coffee"), "{r}");
    a.clock = Some(t.plus_days(31));
    let r = talk(&mut a, "Do I like coffee?");
    assert!(r.starts_with("No"), "{r}");
    let r = talk(&mut a, "Did I like coffee?");
    assert!(r.contains("used to like coffee"), "{r}");
    // Correction of a wrong statement. `is_a` has many values (a dog is also
    // an animal), so only an explicit negation removes the old one (2.0).
    talk(&mut a, "Rex is a cat.");
    let r = talk(&mut a, "No, Rex is a dog.");
    assert!(r.contains("Rex is a dog"), "{r}");
    talk(&mut a, "Rex is not a cat.");
    let r = talk(&mut a, "Is Rex a cat?");
    assert!(r.starts_with("No"), "{r}");
    // Design 2.0: "No, Anna likes coffee" no longer deletes "Anna likes tea"...
    talk(&mut a, "Anna likes tea.");
    talk(&mut a, "What does Anna like?");
    talk(&mut a, "No, Anna likes coffee.");
    let r = talk(&mut a, "Does Anna like tea?");
    assert!(r.contains("Anna likes tea") && !r.starts_with("No"), "{r}");
    let r = talk(&mut a, "Does Anna like coffee?");
    assert!(r.contains("Anna likes coffee") && !r.starts_with("No"), "{r}");
    // ...but a single-valued relation keeps one value.
    talk(&mut a, "Anna lives in Paris.");
    talk(&mut a, "Where does Anna live?");
    talk(&mut a, "No, Anna lives in Rome.");
    let r = talk(&mut a, "Does Anna live in Paris?");
    assert!(r.starts_with("No") || r.contains("used to"), "{r}");
    // Unknown things stay unknown.
    let r = talk(&mut a, "Does Bob like jazz?");
    assert!(r.contains("know") || r.contains("sure") || r.contains("idea"), "{r}");
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn hand_edits_are_reloaded() {
    let root = temp_vault("handedit");
    let mut a = Agent::open(&root, "maksim").unwrap();
    talk(&mut a, "Anna likes tea.");
    let path = root.join("facts").join("anna.md");
    let mut s = fs::read_to_string(&path).unwrap();
    s.push_str("\n```datalog\nlikes(anna, jazz). @conf 1 @source hand\n```\n");
    // Make sure the mtime/size changes are visible.
    std::thread::sleep(std::time::Duration::from_millis(20));
    fs::write(&path, s).unwrap();
    let r = talk(&mut a, "What does Anna like?");
    assert!(r.contains("jazz"), "{r}");
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn correction_refines_rule_in_conversation() {
    let root = temp_vault("correct");
    let mut a = Agent::open(&root, "maksim").unwrap();
    a.clock = Some(Timestamp::from_ymd_hms(2026, 11, 16, 9, 0, 0));
    for m in ["If something is a bird then it flies.", "Tweety is a bird.", "Robin is a bird.", "Pingu is a bird.", "Pingu is a penguin.", "Pinga is a bird.", "Pinga is a penguin."] {
        talk(&mut a, m);
    }
    assert!(talk(&mut a, "Does Pingu fly?").contains("Pingu flies"));
    // One counterexample: an exception only (gradual corrections).
    let r = talk(&mut a, "No, Pingu doesn't fly.");
    assert!(!r.contains("changed my rule") && r.contains("exception"), "{r}");
    assert!(talk(&mut a, "Does Pingu fly?").starts_with("No"));
    // A second with a shared feature changes the rule.
    assert!(talk(&mut a, "Does Pinga fly?").contains("Pinga flies"));
    let r = talk(&mut a, "No, Pinga doesn't fly.");
    assert!(r.contains("changed my rule") && r.contains("penguin"), "{r}");
    assert!(talk(&mut a, "Does Pinga fly?").starts_with("No"));
    assert!(talk(&mut a, "Does Tweety fly?").contains("Tweety flies"));
    let _ = fs::remove_dir_all(&root);
}
