//! Design 2.0, milestone 15 acceptance: the dialogue manager.

use sla_agent::{Agent, Brain};
use sla_parser::oracle::OracleParser;
use std::path::PathBuf;
use std::sync::Arc;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn temp(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("sla_m15_{name}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    d
}

fn oracle(gold: &[(&str, &[&str])]) -> OracleParser {
    let mut o = OracleParser::default();
    for (text, acts) in gold {
        o.insert(text, &acts.iter().map(|a| a.to_string()).collect::<Vec<_>>()).unwrap();
    }
    o
}

fn talk(a: &mut Agent, m: &str) -> String {
    let r = a.respond(m).text;
    let t = a.last_trace.as_ref().unwrap();
    println!("> {m}\n{r}\n   moves: {:?}", t.moves);
    r
}

const DINNER: &[(&str, &[&str])] = &[
    ("Hi! Can you help me plan dinner tonight?", &["greet:", "request: plan_dinner"]),
    (
        "I have pasta, garlic, olive oil, and some cherry tomatoes. Maybe 20 minutes.",
        &["inform: available(pasta); available(garlic); available(olive_oil); available(cherry_tomato)", "inform: time_available(20, minute)"],
    ),
    ("Sounds great, thanks!", &["accept:", "social: thanks"]),
];

/// The design's dinner dialogue, with the recipe in the vault and gold acts.
#[test]
fn dinner_dialogue_with_oracle() {
    let dir = temp("dinner");
    let brain = Brain::shared(&dir).unwrap();
    assert_eq!(sla_agent::brain::lock(&brain).install(&root().join("data/scenarios/dinner")).unwrap(), 3);
    let mut a = Agent::with_brain(Arc::clone(&brain), "maksim");
    a.set_parser(Box::new(oracle(DINNER)));
    a.ask_in_chat = false;

    let r1 = talk(&mut a, DINNER[0].0);
    assert!(r1.starts_with("Sure!"), "{r1}");
    assert!(r1.contains("What ingredients do you have") && r1.contains("how much time do you want to spend cooking?"), "both slots in one turn: {r1}");
    assert!(!r1.contains("Hello"), "the greeting reply gives way to content");
    let proof = a.last_trace.as_ref().unwrap().move_proof.clone().expect("the move has a proof");
    assert!(proof.contains("dm_move(ask_slots") && proof.contains("missing_slot(plan_meal, ingredients)"), "{proof}");

    let r2 = talk(&mut a, DINNER[1].0);
    assert!(r2.contains("You could make garlic tomato pasta"), "{r2}");
    assert!(r2.contains("Boil the pasta") && r2.contains("Mix the pasta"), "the steps: {r2}");
    assert!(r2.contains("15 to 20 minutes"), "the total time: {r2}");
    assert!(!r2.contains("tomato soup"), "soup is not feasible with these ingredients");
    assert!(!r2.contains("You have pasta"), "no echo of what the user said");

    let r3 = talk(&mut a, DINNER[2].0);
    assert!(r3.contains("Enjoy your dinner!"), "{r3}");
    assert!(r3.contains("welcome") || r3.contains("Glad") || r3.contains("Any time"), "{r3}");
    assert!(a.frame.is_none(), "the frame is closed");
    let b = sla_agent::brain::lock(&brain);
    assert_eq!(b.kb.vault.procedures["garlic_tomato_pasta"].meta().successes, 1, "the outcome is logged as evidence");
    assert!(b.kb.all_facts().iter().all(|f| f.atom.pred != "available" && f.atom.pred != "time_available"), "the situation stays in the session");
    drop(b);
    let _ = std::fs::remove_dir_all(&dir);
}

/// Not enough time, or a missing ingredient: the closest option and what it lacks.
#[test]
fn no_feasible_option_names_what_is_missing() {
    let dir = temp("closest");
    let brain = Brain::shared(&dir).unwrap();
    sla_agent::brain::lock(&brain).install(&root().join("data/scenarios/dinner")).unwrap();
    let mut a = Agent::with_brain(Arc::clone(&brain), "maksim");
    a.ask_in_chat = false;
    a.set_parser(Box::new(oracle(&[
        ("Can you help me make dinner?", &["request: make_dinner"]),
        ("I have pasta and olive oil, and an hour.", &["inform: available(pasta); available(olive_oil); time_available(1, hour)"]),
    ])));
    talk(&mut a, "Can you help me make dinner?");
    let r = talk(&mut a, "I have pasta and olive oil, and an hour.");
    assert!(r.contains("closest") && r.contains("garlic") && r.contains("cherry tomato"), "{r}");
    let _ = std::fs::remove_dir_all(&dir);
}

const TIRE: &str = "---\nid: change_tire\nstatus: confirmed\nsource: test\nuses: 0\nsuccesses: 0\n---\n# Change a tire\n\n```datalog\ntask(change_tire).\nneeds(change_tire, spare_tire).\nstep(change_tire, ct_s1).\naction(ct_s1, loosen).\nobject_of(ct_s1, nut).\nuses(ct_s1, wrench).\nstep(change_tire, ct_s2).\naction(ct_s2, lift).\nobject_of(ct_s2, car).\nuses(ct_s2, jack).\nstep(change_tire, ct_s3).\naction(ct_s3, swap).\nobject_of(ct_s3, tire).\nstep(change_tire, ct_s4).\naction(ct_s4, tighten).\nobject_of(ct_s4, nut).\nbefore(ct_s1, ct_s2).\nbefore(ct_s2, ct_s3).\nbefore(ct_s3, ct_s4).\n```\n";

fn tire_agent(name: &str) -> (PathBuf, Agent) {
    let dir = temp(name);
    let scen = dir.join("scenario");
    std::fs::create_dir_all(scen.join("procedures")).unwrap();
    std::fs::write(scen.join("procedures").join("change_tire.md"), TIRE).unwrap();
    let brain = Brain::shared(dir.join("vault")).unwrap();
    sla_agent::brain::lock(&brain).install(&scen).unwrap();
    let mut a = Agent::with_brain(brain, "maksim");
    a.ask_in_chat = false;
    a.set_parser(Box::new(oracle(&[
        ("How do I change a tire?", &["ask_how: change_tire"]),
        ("Done.", &["done: last"]),
        ("Nice weather today.", &["tell: property(weather, state, nice)"]),
        ("Anna likes tea.", &["tell: likes(anna, tea)"]),
        ("Ok, what's next?", &["ask_next:"]),
        ("Why do I need a jack?", &["why: needs(change_tire, jack)"]),
    ])));
    (dir, a)
}

/// After two side remarks, "what's next?" resumes at the right step.
#[test]
fn resumes_after_side_remarks() {
    let (dir, mut a) = tire_agent("resume");
    let r = talk(&mut a, "How do I change a tire?");
    assert!(r.contains("Loosen the nuts") && r.contains("Lift the cars") || r.contains("Lift the car"), "{r}");
    let r = talk(&mut a, "Done.");
    assert!(r.contains("Next: Lift the car"), "{r}");
    talk(&mut a, "Nice weather today.");
    talk(&mut a, "Anna likes tea.");
    let r = talk(&mut a, "Ok, what's next?");
    assert!(r.contains("Lift the car"), "still on the step not yet done: {r}");
    let r = talk(&mut a, "Done.");
    assert!(r.contains("Swap the tire"), "{r}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// "Why do I need a jack?" is answered from a proof: a step uses it.
#[test]
fn why_from_a_proof() {
    let (dir, mut a) = tire_agent("why");
    talk(&mut a, "How do I change a tire?");
    let r = talk(&mut a, "Why do I need a jack?");
    assert!(r.contains("because") && r.contains("jack"), "{r}");
    assert!(r.contains("lift") || r.contains("Lift") || r.contains("ct_s2") || r.contains("step"), "the proof names the step: {r}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// One conversation's situation never reaches another.
#[test]
fn no_facts_from_another_session() {
    let dir = temp("isolation");
    let brain = Brain::shared(&dir).unwrap();
    sla_agent::brain::lock(&brain).install(&root().join("data/scenarios/dinner")).unwrap();
    let gold = oracle(DINNER);
    let mut a = Agent::with_brain(Arc::clone(&brain), "maksim");
    a.ask_in_chat = false;
    a.set_parser(Box::new(gold.clone()));
    talk(&mut a, DINNER[0].0);
    talk(&mut a, DINNER[1].0);
    let mut b = Agent::with_brain(Arc::clone(&brain), "friend");
    b.ask_in_chat = false;
    b.set_parser(Box::new(gold));
    let r = talk(&mut b, DINNER[0].0);
    assert!(r.contains("What ingredients do you have"), "the second conversation knows nothing of the first: {r}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// The same dinner dialogue through the real translator (Haiku). Spends a few
/// model calls, so it runs only on request:
/// `cargo test -p sla_agent --test dialogue -- --ignored dinner_dialogue_live`
#[test]
#[ignore]
fn dinner_dialogue_live() {
    let dir = temp("dinner_live");
    let brain = Brain::shared(&dir).unwrap();
    sla_agent::brain::lock(&brain).install(&root().join("data/scenarios/dinner")).unwrap();
    let mut a = Agent::with_brain(Arc::clone(&brain), "maksim");
    a.enable_llm("haiku");
    a.ask_in_chat = false;
    let r1 = talk(&mut a, DINNER[0].0);
    assert!(r1.contains("What ingredients do you have") && r1.contains("how much time"), "{r1}");
    let r2 = talk(&mut a, DINNER[1].0);
    assert!(r2.contains("garlic tomato pasta") && r2.contains("Boil the pasta"), "{r2}");
    let r3 = talk(&mut a, DINNER[2].0);
    assert!(r3.contains("Enjoy your dinner!"), "{r3}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// The open question survives a side remark, and the agent returns to it.
#[test]
fn returns_to_the_open_question() {
    let dir = temp("question_stack");
    let brain = Brain::shared(&dir).unwrap();
    sla_agent::brain::lock(&brain).install(&root().join("data/scenarios/dinner")).unwrap();
    let mut a = Agent::with_brain(Arc::clone(&brain), "maksim");
    a.ask_in_chat = false;
    a.set_parser(Box::new(oracle(&[
        ("Can you help me plan dinner?", &["request: plan_dinner"]),
        ("By the way, Anna likes tea.", &["tell: likes(anna, tea)"]),
        ("I have pasta, garlic, olive oil and cherry tomatoes.", &["inform: available(pasta); available(garlic); available(olive_oil); available(cherry_tomato)"]),
        ("About 20 minutes.", &["inform: time_available(20, minute)"]),
    ])));
    talk(&mut a, "Can you help me plan dinner?");
    let r = talk(&mut a, "By the way, Anna likes tea.");
    assert!(r.contains("Back to your plan") && r.contains("what ingredients") && !r.contains("Anna likes tea"), "acknowledged without echo, then back to the plan: {r}");
    let r = talk(&mut a, "I have pasta, garlic, olive oil and cherry tomatoes.");
    assert!(r.to_lowercase().contains("how much time") && !r.contains("ingredients"), "only the slot still open: {r}");
    let r = talk(&mut a, "About 20 minutes.");
    assert!(r.contains("garlic tomato pasta"), "{r}");
    let _ = std::fs::remove_dir_all(&dir);
}
