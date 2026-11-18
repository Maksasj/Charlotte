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

/// A new word enters as pending and cannot be spoken until it is approved.
#[test]
fn new_word_is_pending_until_approved() {
    let dir = temp("word");
    let vault = dir.join("vault");
    let brain = Brain::shared(&vault).unwrap();
    let gold: &[(&str, &[&str])] = &[
        ("Moss grows on rocks.", &["new_pred: grows_on(moss, rock) | X grows on the surface of Y"]),
        ("What do you know about moss?", &["recall: moss"]),
    ];
    let mut a = Agent::with_brain(Arc::clone(&brain), "maksim");
    a.ask_in_chat = false;
    a.set_parser(Box::new(oracle(gold)));
    let r = talk(&mut a, gold[0].0);
    assert!(r.contains("grows on") && r.contains("teacher"), "{r}");
    {
        let b = sla_agent::brain::lock(&brain);
        let q = sla_agent::queue::pending(&b);
        assert_eq!(q.len(), 1);
        assert_eq!(q[0].facts, vec!["grows_on(moss, rock)".to_string()]);
        assert!(b.kb.all_facts().iter().all(|f| f.atom.pred != "grows_on"), "pending facts are not believed");
    }
    let r = talk(&mut a, gold[1].0);
    assert!(!r.contains("grows") && !r.contains("rock"), "a pending word is not spoken: {r}");
    // A reviewer approves it with a sentence frame.
    {
        let mut b = sla_agent::brain::lock(&brain);
        let now = sla_core::Timestamp::now();
        assert_eq!(sla_agent::queue::approve(&mut b, "grows_on", "{1} grows on {2}", now).unwrap(), 1);
        assert!(sla_agent::queue::pending(&b).is_empty());
    }
    // After a restart the word is known and spoken with its frame.
    drop(a);
    drop(brain);
    let brain = Brain::shared(&vault).unwrap();
    let mut a = Agent::with_brain(Arc::clone(&brain), "maksim");
    a.ask_in_chat = false;
    a.set_parser(Box::new(oracle(gold)));
    let r = talk(&mut a, gold[1].0);
    assert!(r.contains("moss grows on rock") || r.contains("Moss grows on rock"), "{r}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// A teacher who knows which patterns are true and answers rule questions.
struct TruthTeacher {
    true_rules: Vec<&'static str>,
    asked: usize,
}

impl sla_agent::curiosity::Teacher for TruthTeacher {
    fn answer(&mut self, q: &str) -> String {
        self.asked += 1;
        println!("Q: {q}");
        if !q.contains("noticed a pattern") {
            return "I don't know.".into();
        }
        if self.true_rules.iter().any(|t| q.contains(t)) { "Yes.".into() } else { "No.".into() }
    }
}

/// In a scripted teacher run, 80% or more of guessed rules end confirmed or rejected.
#[test]
fn guessed_rules_end_confirmed_or_rejected() {
    let dir = temp("guesses");
    let mut a = Agent::open(dir.join("vault"), "teacher").unwrap();
    a.ask_in_chat = false;
    a.clock = Some(sla_core::Timestamp::from_ymd_hms(2026, 11, 17, 9, 0, 0));
    let people = [("Anna", "British", "tea"), ("Bob", "British", "tea"), ("Carl", "British", "tea"), ("Dora", "British", "tea"),
        ("Emil", "French", "coffee"), ("Fay", "French", "coffee"), ("Gus", "French", "coffee"), ("Hana", "French", "coffee")];
    for (n, nat, d) in people {
        talk(&mut a, &format!("{n} is {nat}."));
        talk(&mut a, &format!("{n} likes {d}."));
    }
    for (n, pet) in [("Ivo", "dog"), ("Jan", "dog"), ("Kim", "dog")] {
        talk(&mut a, &format!("{n} has a {pet}."));
        talk(&mut a, &format!("{n} likes walks."));
    }
    let mut teacher = TruthTeacher { true_rules: vec!["British then X likes tea", "French then X likes coffee", "has dog then X likes walk"], asked: 0 };
    for _ in 0..40 {
        if a.ask_teacher(&mut teacher).is_none() {
            break;
        }
    }
    let b = a.brain();
    let learned: Vec<sla_core::RuleMeta> = b.kb.vault.rules.values().map(|rf| rf.meta()).filter(|m| m.source == "learner").collect();
    for rf in b.kb.vault.rules.values().filter(|rf| rf.meta().source == "learner") {
        println!("{} [{}] {}", rf.meta().id, rf.meta().status.as_str(), rf.rules().first().map(|r| r.to_string()).unwrap_or_default());
    }
    // The true patterns are confirmed.
    for head in ["likes(X, tea)", "likes(X, coffee)", "likes(X, walk)"] {
        let ok = b.kb.vault.rules.values().any(|rf| rf.meta().status == sla_core::RuleStatus::Confirmed && rf.rules().iter().any(|r| r.head.to_string().replace(['A', 'B'], "X") == head));
        assert!(ok, "{head} not confirmed");
    }
    drop(b);
    let settled = learned.iter().filter(|m| m.status != sla_core::RuleStatus::Hypothesis).count();
    println!("guessed {} rules, {} settled, teacher asked {} questions", learned.len(), settled, teacher.asked);
    assert!(!learned.is_empty(), "the learner guessed no rules");
    assert!(settled * 10 >= learned.len() * 8, "only {settled} of {} guessed rules settled", learned.len());
    let _ = std::fs::remove_dir_all(&dir);
}

/// The tentative layer: a guessed rule answers as a guess, and the answer to
/// the guess settles the rule.
#[test]
fn guessed_rule_answers_as_a_guess() {
    let dir = temp("tentative");
    let mut a = Agent::open(dir.join("vault"), "teacher").unwrap();
    a.ask_in_chat = false;
    a.clock = Some(sla_core::Timestamp::from_ymd_hms(2026, 11, 17, 9, 0, 0));
    for n in ["Anna", "Bob", "Carl", "Dora"] {
        talk(&mut a, &format!("{n} is British."));
        talk(&mut a, &format!("{n} likes tea."));
    }
    talk(&mut a, "Emil is French.");
    talk(&mut a, "Emil likes coffee.");
    talk(&mut a, "Eve is British.");
    let r = talk(&mut a, "Does Eve like tea?");
    assert!(r.contains("I'd guess so") && r.contains("Eve likes tea") && r.contains("Is that right?"), "{r}");
    let r = talk(&mut a, "Yes.");
    assert!(r.contains("treat that as a rule"), "{r}");
    let r = talk(&mut a, "Does Eve like tea?");
    assert!(r.starts_with("Yes") || r.contains("That's right"), "{r}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// Steps taught by number without an order are ordered by their numbers; a
/// differently worded task name lands in the same procedure.
#[test]
fn numbered_steps_are_ordered_and_names_merge() {
    let dir = temp("order");
    let brain = Brain::shared(dir.join("vault")).unwrap();
    let gold: &[(&str, &[&str])] = &[
        ("To boil an egg you need an egg and a pot.", &["tell: task(boil_egg); needs(boil_egg, egg); needs(boil_egg, pot)"]),
        ("Step 1 of boiling an egg: boil water in the pot.", &["tell: step(boil_egg, boil_egg_s1); action(boil_egg_s1, boil); object_of(boil_egg_s1, water)"]),
        ("Step 2 of boiling an egg: add the egg.", &["tell: step(cook_egg, cook_egg_s2); action(cook_egg_s2, add); object_of(cook_egg_s2, egg)"]),
        ("Step 3 of boiling an egg: cool the egg.", &["tell: step(boil_egg, boil_egg_s3); action(boil_egg_s3, cool); object_of(boil_egg_s3, egg)"]),
        ("How do I cook an egg?", &["ask_how: cook_egg"]),
    ];
    let mut t = Agent::with_brain(Arc::clone(&brain), "teacher");
    t.ask_in_chat = false;
    t.set_parser(Box::new(oracle(gold)));
    for (m, _) in &gold[..4] {
        talk(&mut t, m);
    }
    {
        let b = sla_agent::brain::lock(&brain);
        assert!(!b.kb.vault.procedures.contains_key("cook_egg"), "cook_egg is boil_egg");
        let facts: Vec<String> = b.kb.vault.procedures["boil_egg"].facts().iter().map(|f| f.atom.to_string()).collect();
        assert!(facts.contains(&"before(boil_egg_s1, boil_egg_s2)".to_string()), "{facts:?}");
        assert!(facts.contains(&"before(boil_egg_s2, boil_egg_s3)".to_string()), "{facts:?}");
    }
    let mut a = Agent::with_brain(Arc::clone(&brain), "maksim");
    a.ask_in_chat = false;
    a.set_parser(Box::new(oracle(gold)));
    let r = talk(&mut a, gold[4].0);
    let (x, y, z) = (r.find("Boil").unwrap_or(usize::MAX), r.find("Add").unwrap_or(usize::MAX), r.find("Cool").unwrap_or(usize::MAX));
    assert!(x < y && y < z && z != usize::MAX, "{r}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// Transfer: the reply takes the user's situation into account (too little
/// time, a missing item) instead of ignoring it.
#[test]
fn reply_notes_time_and_missing_items() {
    let dir = temp("constraints");
    let brain = Brain::shared(dir.join("vault")).unwrap();
    let mut t = Agent::with_brain(Arc::clone(&brain), "teacher");
    t.ask_in_chat = false;
    t.set_parser(Box::new(oracle(TEACH)));
    for (m, _) in TEACH {
        talk(&mut t, m);
    }
    let gold: &[(&str, &[&str])] = &[(
        "How do I make garlic tomato pasta? I have no garlic and only 10 minutes.",
        &["ask_how: garlic_tomato_pasta", "inform: -available(garlic); time_available(10, minute)"],
    )];
    let mut a = Agent::with_brain(Arc::clone(&brain), "maksim");
    a.ask_in_chat = false;
    a.set_parser(Box::new(oracle(gold)));
    let r = talk(&mut a, gold[0].0);
    assert!(r.contains("more than the 10 minutes you have"), "{r}");
    assert!(r.contains("You don't have garlic"), "{r}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// A request in quite different words finds the task by what it involves,
/// and the reply says what is needed.
#[test]
fn request_finds_task_by_what_it_involves() {
    let dir = temp("involves");
    let brain = Brain::shared(dir.join("vault")).unwrap();
    let gold: &[(&str, &[&str])] = &[
        ("To fix a flat tire you need a patch kit, a tube and a pump.", &["tell: task(fix_tire); needs(fix_tire, patch_kit); needs(fix_tire, tube); needs(fix_tire, pump)"]),
        ("Step 1 of fixing a flat tire: take out the tube.", &["tell: step(fix_tire, fix_tire_s1); action(fix_tire_s1, remove); object_of(fix_tire_s1, tube)"]),
        ("Step 2 of fixing a flat tire: patch the hole.", &["tell: step(fix_tire, fix_tire_s2); action(fix_tire_s2, patch); object_of(fix_tire_s2, hole)"]),
        ("Can you walk me through patching the tube?", &["ask_how: patch_tube"]),
    ];
    let mut t = Agent::with_brain(Arc::clone(&brain), "teacher");
    t.ask_in_chat = false;
    t.set_parser(Box::new(oracle(gold)));
    for (m, _) in &gold[..3] {
        talk(&mut t, m);
    }
    let mut a = Agent::with_brain(Arc::clone(&brain), "maksim");
    a.ask_in_chat = false;
    a.set_parser(Box::new(oracle(gold)));
    let r = talk(&mut a, gold[3].0);
    assert!(r.contains("Here is how to fix a tire") && r.contains("You need a patch kit, a pump and a tube"), "{r}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// "How long ... and what are the steps?" gets the steps and the time.
#[test]
fn duration_question_that_asks_for_steps_gets_them() {
    let dir = temp("duration_steps");
    let brain = Brain::shared(dir.join("vault")).unwrap();
    let mut t = Agent::with_brain(Arc::clone(&brain), "teacher");
    t.ask_in_chat = false;
    t.set_parser(Box::new(oracle(TEACH)));
    for (m, _) in TEACH {
        talk(&mut t, m);
    }
    let gold: &[(&str, &[&str])] = &[
        ("How long does garlic tomato pasta take, and what are the steps?", &["ask_duration: garlic_tomato_pasta"]),
        ("How long does garlic tomato pasta take?", &["ask_duration: garlic_tomato_pasta"]),
    ];
    let mut a = Agent::with_brain(Arc::clone(&brain), "maksim");
    a.ask_in_chat = false;
    a.set_parser(Box::new(oracle(gold)));
    let r = talk(&mut a, gold[0].0);
    assert!(r.contains("Boil the pasta") && r.contains("15 to 20 minutes"), "{r}");
    let r = talk(&mut a, gold[1].0);
    assert!(!r.contains("Boil the pasta") && r.contains("15 to 20 minutes"), "{r}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// A taught substitute shows up when the user lacks the item.
#[test]
fn known_substitute_is_offered() {
    let dir = temp("substitute");
    let brain = Brain::shared(dir.join("vault")).unwrap();
    let mut teach: Vec<(&str, &[&str])> = TEACH.to_vec();
    teach.push(("A shallot can replace garlic.", &["tell: substitute(garlic, shallot)"]));
    let mut t = Agent::with_brain(Arc::clone(&brain), "teacher");
    t.ask_in_chat = false;
    t.set_parser(Box::new(oracle(&teach)));
    for (m, _) in &teach {
        talk(&mut t, m);
    }
    let gold: &[(&str, &[&str])] = &[("How do I make garlic tomato pasta without garlic?", &["ask_how: garlic_tomato_pasta", "inform: -available(garlic)"])];
    let mut a = Agent::with_brain(Arc::clone(&brain), "maksim");
    a.ask_in_chat = false;
    a.set_parser(Box::new(oracle(gold)));
    let r = talk(&mut a, gold[0].0);
    assert!(r.contains("You can use a shallot instead of the garlic"), "{r}");
    assert!(!r.contains("I don't know what can replace"), "{r}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// Transfer: a taught substitute is offered when the user has it instead.
#[test]
fn reply_offers_a_substitute_the_user_has() {
    let dir = temp("substitute_have");
    let brain = Brain::shared(dir.join("vault")).unwrap();
    let mut t = Agent::with_brain(Arc::clone(&brain), "teacher");
    t.ask_in_chat = false;
    let mut teach: Vec<(&str, &[&str])> = TEACH.to_vec();
    teach.push(("Shallots can replace garlic.", &["tell: substitute(garlic, shallot)"]));
    t.set_parser(Box::new(oracle(&teach)));
    for (m, _) in &teach {
        talk(&mut t, m);
    }
    let gold: &[(&str, &[&str])] = &[(
        "How do I make garlic tomato pasta? I have pasta, a shallot, olive oil and cherry tomatoes.",
        &["ask_how: garlic_tomato_pasta", "inform: available(pasta); available(shallot); available(olive_oil); available(cherry_tomato)"],
    )];
    let mut a = Agent::with_brain(Arc::clone(&brain), "maksim");
    a.ask_in_chat = false;
    a.set_parser(Box::new(oracle(gold)));
    let r = talk(&mut a, gold[0].0);
    assert!(r.contains("You can use a shallot instead of the garlic"), "{r}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// Transfer: a lacking item with a known substitute is swapped into the steps.
#[test]
fn steps_use_the_substitute_for_a_lacking_item() {
    let dir = temp("substitute_swap");
    let brain = Brain::shared(dir.join("vault")).unwrap();
    let mut t = Agent::with_brain(Arc::clone(&brain), "teacher");
    t.ask_in_chat = false;
    let mut teach: Vec<(&str, &[&str])> = TEACH.to_vec();
    teach.push(("Shallots can replace garlic.", &["tell: substitute(garlic, shallot)"]));
    t.set_parser(Box::new(oracle(&teach)));
    for (m, _) in &teach {
        talk(&mut t, m);
    }
    let gold: &[(&str, &[&str])] = &[("How do I make garlic tomato pasta? I have no garlic.", &["ask_how: garlic_tomato_pasta", "inform: -available(garlic)"])];
    let mut a = Agent::with_brain(Arc::clone(&brain), "maksim");
    a.ask_in_chat = false;
    a.set_parser(Box::new(oracle(gold)));
    let r = talk(&mut a, gold[0].0);
    assert!(r.contains("fry the shallot") && r.contains("how to make garlic tomato pasta"), "{r}");
    assert!(r.contains("You can use a shallot instead of the garlic"), "{r}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// Transfer by analogy: no substitute was taught, but another procedure boils
/// water in a pot; the agent asks what the user has and adapts the steps.
#[test]
fn transfer_by_analogy_after_asking() {
    let dir = temp("analogy");
    let brain = Brain::shared(dir.join("vault")).unwrap();
    let teach: &[(&str, &[&str])] = &[
        ("To make tea you need a kettle, a mug, a tea bag and water.", &["tell: task(make_tea); needs(make_tea, kettle); needs(make_tea, mug); needs(make_tea, tea_bag); needs(make_tea, water)"]),
        ("Step 1 of making tea: boil the water in the kettle.", &["tell: step(make_tea, make_tea_s1); action(make_tea_s1, boil); object_of(make_tea_s1, water); uses(make_tea_s1, kettle)"]),
        ("Step 2 of making tea: pour the water into the mug.", &["tell: step(make_tea, make_tea_s2); action(make_tea_s2, pour); object_of(make_tea_s2, water); uses(make_tea_s2, mug)"]),
        ("To boil an egg you need an egg, water and a pot.", &["tell: task(boil_egg); needs(boil_egg, egg); needs(boil_egg, water); needs(boil_egg, pot)"]),
        ("Step 1 of boiling an egg: boil the water in the pot.", &["tell: step(boil_egg, boil_egg_s1); action(boil_egg_s1, boil); object_of(boil_egg_s1, water); uses(boil_egg_s1, pot)"]),
    ];
    let mut t = Agent::with_brain(Arc::clone(&brain), "teacher");
    t.ask_in_chat = false;
    t.set_parser(Box::new(oracle(teach)));
    for (m, _) in teach {
        talk(&mut t, m);
    }
    let gold: &[(&str, &[&str])] = &[
        ("My kettle broke. How do I make tea?", &["inform: -available(kettle)", "ask_how: make_tea"]),
        ("I have a small pot, a mug, tea bags and water.", &["inform: available(pot); available(mug); available(tea_bag); available(water)"]),
    ];
    let mut a = Agent::with_brain(Arc::clone(&brain), "maksim");
    a.ask_in_chat = false;
    a.set_parser(Box::new(oracle(gold)));
    let r = talk(&mut a, gold[0].0);
    assert!(r.trim_end().ends_with('?') && r.contains("what do you have"), "asks: {r}");
    let r = talk(&mut a, gold[1].0);
    assert!(r.contains("Boil the water in the pot") && r.contains("probably use a pot instead of the kettle") && r.contains("boil an egg"), "{r}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// Asked what could do the job, the user names a replacement: the agent uses it.
#[test]
fn adopts_the_replacement_the_user_names() {
    let dir = temp("user_proposal");
    let brain = Brain::shared(dir.join("vault")).unwrap();
    let teach: &[(&str, &[&str])] = &[
        ("To water a plant you need a watering can and water.", &["tell: task(water_plant); needs(water_plant, watering_can); needs(water_plant, water)"]),
        ("Step 1 of watering a plant: fill the watering can with water.", &["tell: step(water_plant, water_plant_s1); action(water_plant_s1, fill); object_of(water_plant_s1, watering_can); uses(water_plant_s1, water)"]),
        ("Step 2 of watering a plant: pour the water with the watering can.", &["tell: step(water_plant, water_plant_s2); action(water_plant_s2, pour); object_of(water_plant_s2, water); uses(water_plant_s2, watering_can)"]),
    ];
    let mut t = Agent::with_brain(Arc::clone(&brain), "teacher");
    t.ask_in_chat = false;
    t.set_parser(Box::new(oracle(teach)));
    for (m, _) in teach {
        talk(&mut t, m);
    }
    let gold: &[(&str, &[&str])] = &[
        ("I don't own a watering can. How do I water my plant?", &["inform: -available(watering_can)", "ask_how: water_plant"]),
        ("I have a kitchen jug and tap water.", &["inform: available(jug); available(water)"]),
    ];
    let mut a = Agent::with_brain(Arc::clone(&brain), "maksim");
    a.ask_in_chat = false;
    a.set_parser(Box::new(oracle(gold)));
    let r = talk(&mut a, gold[0].0);
    assert!(r.contains("what do you have"), "{r}");
    let r = talk(&mut a, gold[1].0);
    assert!(r.contains("Since you have a jug, use it instead of the watering can") && r.contains("Fill the jug"), "{r}");
    let _ = std::fs::remove_dir_all(&dir);
}
