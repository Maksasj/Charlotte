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
            assert!(r.starts_with("Okay") || r.starts_with("Got it") || r.starts_with("Noted"), "{msg} -> {r}");
        }
        // 2 rules.
        let r = talk(&mut a, "If someone likes tea then they want tea.");
        assert!(r.contains("rule"), "{r}");
        let r = talk(&mut a, "If something is in Berlin then it is in Germany.");
        assert!(r.contains("rule"), "{r}");
        assert_eq!(a.kb.all_facts().iter().filter(|f| f.atom.pred != "proper_name").count(), 10);
        assert_eq!(a.dump_rules().len(), 2);
    }

    // Restart: a new agent on the same vault, a day later.
    let mut b = Agent::open(&root, "maksim").unwrap();
    b.clock = Some(day1.plus_days(1));
    let checks = [
        ("Do I like coffee?", "Yes"),
        ("What does Anna like?", "Anna likes tea"),
        ("Where does Anna live?", "Anna is in Berlin"),
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
        ("Is Anna in Germany?", "Anna is in Germany"),
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
    // Correction of a wrong statement.
    talk(&mut a, "Rex is a cat.");
    let r = talk(&mut a, "No, Rex is a dog.");
    assert!(r.contains("Rex is a dog"), "{r}");
    let r = talk(&mut a, "Is Rex a cat?");
    assert!(r.starts_with("No"), "{r}");
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
