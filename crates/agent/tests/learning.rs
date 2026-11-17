//! Design 2.0, milestone 17 acceptance: learning.

use sla_agent::{Agent, Brain};
use sla_parser::oracle::OracleParser;
use std::path::PathBuf;
use std::sync::Arc;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn temp(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("sla_m17_{name}_{}", std::process::id()));
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
    println!("> {m}\n{r}");
    r
}

/// The teaching session: the recipe, told over several messages.
pub const TEACH: &[(&str, &[&str])] = &[
    ("Garlic tomato pasta is a good quick dinner.", &["tell: task(garlic_tomato_pasta); solves(garlic_tomato_pasta, plan_dinner)"]),
    (
        "To make it you need pasta, garlic, olive oil and cherry tomatoes.",
        &["tell: needs(garlic_tomato_pasta, pasta); needs(garlic_tomato_pasta, garlic); needs(garlic_tomato_pasta, olive_oil); needs(garlic_tomato_pasta, cherry_tomato)"],
    ),
    ("It takes 15 to 20 minutes.", &["tell: takes_time(garlic_tomato_pasta, 15, 20, minute)"]),
    (
        "First boil the pasta for 10 to 12 minutes.",
        &["tell: step(garlic_tomato_pasta, garlic_tomato_pasta_s1); action(garlic_tomato_pasta_s1, boil); object_of(garlic_tomato_pasta_s1, pasta); takes_time(garlic_tomato_pasta_s1, 10, 12, minute)"],
    ),
    (
        "While it cooks, fry the garlic in olive oil for a minute.",
        &["tell: step(garlic_tomato_pasta, garlic_tomato_pasta_s2); action(garlic_tomato_pasta_s2, fry); object_of(garlic_tomato_pasta_s2, garlic); uses(garlic_tomato_pasta_s2, olive_oil); takes_time(garlic_tomato_pasta_s2, 1, 1, minute); overlaps(garlic_tomato_pasta_s2, garlic_tomato_pasta_s1)"],
    ),
    (
        "Then add the cherry tomatoes until they are soft.",
        &["tell: step(garlic_tomato_pasta, garlic_tomato_pasta_s3); action(garlic_tomato_pasta_s3, add); object_of(garlic_tomato_pasta_s3, cherry_tomato); until(garlic_tomato_pasta_s3, soft); before(garlic_tomato_pasta_s2, garlic_tomato_pasta_s3)"],
    ),
    (
        "Finally mix it all with the pasta.",
        &["tell: step(garlic_tomato_pasta, garlic_tomato_pasta_s4); action(garlic_tomato_pasta_s4, mix); object_of(garlic_tomato_pasta_s4, pasta); before(garlic_tomato_pasta_s3, garlic_tomato_pasta_s4); before(garlic_tomato_pasta_s1, garlic_tomato_pasta_s4)"],
    ),
];

/// The dinner dialogue later, in new words, with no teaching.
pub const ASK: &[(&str, &[&str])] = &[
    ("Hey! What should I cook tonight?", &["greet:", "request: plan_dinner"]),
    (
        "I've got pasta, garlic, some olive oil and cherry tomatoes, and maybe 20 min.",
        &["inform: available(pasta); available(garlic); available(olive_oil); available(cherry_tomato); time_available(20, minute)"],
    ),
    ("Perfect, thank you!", &["accept:", "social: thanks"]),
];

/// A vault with only the meal task schema (slots and questions): no recipe.
fn schema_only(dir: &std::path::Path) -> PathBuf {
    let scen = dir.join("scenario");
    std::fs::create_dir_all(scen.join("tasks")).unwrap();
    std::fs::copy(root().join("data/scenarios/dinner/tasks/plan_meal.md"), scen.join("tasks/plan_meal.md")).unwrap();
    scen
}

#[test]
fn recipe_taught_once_is_used_later_in_new_words() {
    let dir = temp("taught");
    let scen = schema_only(&dir);
    let vault = dir.join("vault");
    // Session 1: a teacher teaches the recipe.
    {
        let brain = Brain::shared(&vault).unwrap();
        sla_agent::brain::lock(&brain).install(&scen).unwrap();
        let mut t = Agent::with_brain(Arc::clone(&brain), "teacher");
        t.source = "teacher:test".into();
        t.ask_in_chat = false;
        t.set_parser(Box::new(oracle(TEACH)));
        for (m, _) in TEACH {
            talk(&mut t, m);
        }
        let b = sla_agent::brain::lock(&brain);
        let pf = b.kb.vault.procedures.get("garlic_tomato_pasta").expect("a procedure file was learned");
        assert_eq!(pf.meta().status, sla_core::RuleStatus::Hypothesis);
        assert_eq!(pf.facts().iter().filter(|f| f.atom.pred == "step").count(), 4);
        assert!(b.kb.all_facts().iter().all(|f| !matches!(f.atom.pred.as_str(), "step" | "needs" | "action")), "no loose recipe facts");
    }
    // Restart, and an unrelated conversation.
    let brain = Brain::shared(&vault).unwrap();
    {
        let mut other = Agent::with_brain(Arc::clone(&brain), "friend");
        other.ask_in_chat = false;
        talk(&mut other, "Anna likes tea.");
        talk(&mut other, "Rex is a dog.");
    }
    // The dinner dialogue in new words, no teaching.
    let mut a = Agent::with_brain(Arc::clone(&brain), "maksim");
    a.ask_in_chat = false;
    a.set_parser(Box::new(oracle(ASK)));
    let r1 = talk(&mut a, ASK[0].0);
    assert!(r1.contains("What ingredients do you have"), "{r1}");
    let r2 = talk(&mut a, ASK[1].0);
    assert!(r2.contains("garlic tomato pasta") && r2.contains("I was taught this once"), "{r2}");
    assert!(r2.contains("Boil the pasta") && r2.contains("While you boil the pasta, fry the garlic") && r2.contains("until they are soft"), "{r2}");
    assert!(r2.contains("15 to 20 minutes"), "{r2}");
    let r3 = talk(&mut a, ASK[2].0);
    assert!(r3.contains("Enjoy your dinner!"), "{r3}");
    // A successful use confirms the procedure.
    let b = sla_agent::brain::lock(&brain);
    assert_eq!(b.kb.vault.procedures["garlic_tomato_pasta"].meta().status, sla_core::RuleStatus::Confirmed);
    drop(b);
    let _ = std::fs::remove_dir_all(&dir);
}
