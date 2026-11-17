//! Milestone 9 acceptance: in a scripted session the agent asks about a
//! stated contradiction and about an unconfirmed hypothesis.

use sla_agent::curiosity::ScriptedTeacher;
use sla_agent::{Agent, Pending};
use sla_core::{RuleStatus, Timestamp};
use std::fs;

fn temp_vault(name: &str) -> std::path::PathBuf {
    let d = std::env::temp_dir().join(format!("sla_m9_{name}_{}", std::process::id()));
    let _ = fs::remove_dir_all(&d);
    d
}

fn talk(agent: &mut Agent, msg: &str) -> String {
    let r = agent.respond(msg).text;
    println!("> {msg}\n{r}");
    r
}

#[test]
fn asks_about_contradiction_then_hypothesis() {
    let root = temp_vault("session");
    let mut a = Agent::open(&root, "maksim").unwrap();
    a.clock = Some(Timestamp::from_ymd_hms(2026, 11, 16, 9, 0, 0));

    // A stated contradiction: the rule says Pingu flies, the user says it doesn't.
    talk(&mut a, "If something is a bird then it flies.");
    talk(&mut a, "Tweety is a bird.");
    talk(&mut a, "Pingu is a bird.");
    let r = talk(&mut a, "Pingu doesn't fly.");
    assert!(r.contains("conflicting information") && r.contains("Pingu flies") && r.contains("Pingu does not fly"), "{r}");
    assert!(matches!(a.pending, Some(Pending::Contradiction(..))));
    // The answer resolves it: one counterexample is an exception only (gradual
    // corrections), and the contradiction is gone.
    let r = talk(&mut a, "Pingu does not fly.");
    assert!(r.contains("exception to my rule"), "{r}");
    assert!(a.model(false).contradictions().is_empty());
    assert!(talk(&mut a, "Does Tweety fly?").contains("Tweety flies"));

    // A pattern the learner notices: British people like tea.
    let mut asked_rule = None;
    for m in [
        "Anna is British.", "Anna likes tea.", "Bob is British.", "Bob likes tea.", "Carl is British.", "Carl likes tea.",
        "Dan is French.", "Dan likes coffee.",
    ] {
        let r = talk(&mut a, m);
        // The agent may ask while chatting; answer it right away.
        if r.contains("noticed a pattern") && asked_rule.is_none() {
            asked_rule = Some(r.clone());
            talk(&mut a, "Yes.");
        }
    }
    // Otherwise the question goes to the teacher, who confirms it.
    let mut teacher = ScriptedTeacher { answers: vec![("pattern".into(), "yes".into())], default: "I don't know.".into() };
    for _ in 0..5 {
        if asked_rule.is_some() {
            break;
        }
        let Some((q, ans, reply)) = a.ask_teacher(&mut teacher) else { break };
        println!("agent asks: {q}\nteacher: {ans}\nagent: {}", reply.text);
        if q.contains("pattern") {
            asked_rule = Some(q);
        }
    }
    let q = asked_rule.expect("asked about the hypothesis");
    assert!(q.to_lowercase().contains("british") && q.contains("tea"), "{q}");
    let confirmed = a.brain().kb.vault.rules.values().filter(|rf| rf.meta().source == "learner" && rf.meta().status == RuleStatus::Confirmed).count();
    assert!(confirmed >= 1, "teacher's yes confirmed the rule");
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn gaps_and_caps() {
    let root = temp_vault("gaps");
    let mut a = Agent::open(&root, "maksim").unwrap();
    a.ask_in_chat = false;
    a.clock = Some(Timestamp::from_ymd_hms(2026, 11, 16, 9, 0, 0));
    for m in ["Anna is a person.", "Bob is a person.", "Carl is a person.", "Dan is a person.", "Anna's job is teacher.", "Bob's job is cook.", "Carl's job is pilot."] {
        talk(&mut a, m);
    }
    let m = a.model(false);
    let goals = {
        let b = a.brain();
        b.curiosity.goals(&b.kb, &m)
    };
    assert!(goals.iter().any(|g| g.goal.key() == "gap:property(dan, job, W)"), "{goals:?}");
    let mut teacher = ScriptedTeacher { answers: vec![("Dan's job".into(), "Dan's job is doctor.".into())], default: "I don't know.".into() };
    let (q, _, reply) = a.ask_teacher(&mut teacher).unwrap();
    assert_eq!(q, "What is Dan's job?");
    assert!(reply.text.contains("Dan's job is doctor"), "{}", reply.text);
    // The gap is filled and not asked again.
    let m = a.model(false);
    let goals = {
        let b = a.brain();
        b.curiosity.goals(&b.kb, &m)
    };
    assert!(!goals.iter().any(|g| g.goal.key().starts_with("gap:property(dan, job")));

    // Proactive messages are capped per day.
    // Three newcomers lack jobs and hobbies most people have: six gaps.
    for m in ["Anna likes tea.", "Bob likes jazz.", "Carl likes chess.", "Dan likes golf.", "Eve is a person.", "Fay is a person.", "Gus is a person."] {
        talk(&mut a, m);
    }
    let mut proactive = 0;
    for _ in 0..10 {
        if a.proactive().is_some() {
            proactive += 1;
            a.pending = None;
        }
    }
    assert!(proactive >= 1 && proactive <= 3, "proactive {proactive}");
    // Restart: asked questions are remembered.
    let asked_before = a.brain().curiosity.asked.clone();
    drop(a);
    let b = Agent::open(&root, "maksim").unwrap();
    assert!(asked_before.iter().all(|k| b.brain().curiosity.asked.contains(k)));
    let _ = fs::remove_dir_all(&root);
}

/// Milestone 18: failed turns become questions for a teacher, ranked by how
/// many failures an answer fixes; they are never asked in a chat.
#[test]
fn failures_become_teacher_questions() {
    let root = temp_vault("failures");
    let mut a = Agent::open(&root, "maksim").unwrap();
    a.clock = Some(Timestamp::from_ymd_hms(2026, 11, 17, 9, 0, 0));
    a.ask_in_chat = true;
    let gold: &[(&str, &[&str])] = &[("How do I fix a tire?", &["ask_how: fix_tire"]), ("Can you help me fix my tire?", &["request: fix_tire"])];
    let mut o = sla_parser::oracle::OracleParser::default();
    for (t, acts) in gold {
        o.insert(t, &acts.iter().map(|x| x.to_string()).collect::<Vec<_>>()).unwrap();
    }
    a.set_parser(Box::new(o));
    for (m, _) in gold {
        let r = talk(&mut a, m);
        assert!(!r.contains("What do I need, and what is the first step"), "not asked in chat: {r}");
    }
    let mut teacher = ScriptedTeacher { answers: vec![], default: "I don't know.".into() };
    let (q, _, _) = a.ask_teacher(&mut teacher).expect("a question for the teacher");
    assert!(q.contains("How do I fix tire") && q.contains("first step"), "{q}");
    let _ = fs::remove_dir_all(&root);
}
