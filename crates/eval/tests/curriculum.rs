//! Milestone 18: the curriculum runner's encounters, on the oracle parser and
//! a symbolic judge (no model calls).

use sla_agent::{Agent, Brain, SharedBrain};
use sla_eval::curriculum::{self, CheckJudge, CurriculumConfig, Scenario, Turn};
use sla_parser::oracle::OracleParser;
use std::sync::Arc;

const SCENARIO: &str = r#"---
id: make_tea
stage: procedures
task: make_tea
---
# Make tea

## requests
- How do I make a cup of tea?
- Could you walk me through making tea?
- What are the steps for tea?

## situation
- I have tea bags and a kettle.

## ground truth
```datalog
task(make_tea).
needs(make_tea, tea_bag).
needs(make_tea, water).
takes_time(make_tea, 5, 7, minute).
step(make_tea, make_tea_s1).
action(make_tea_s1, boil).
object_of(make_tea_s1, water).
step(make_tea, make_tea_s2).
action(make_tea_s2, pour).
object_of(make_tea_s2, water).
before(make_tea_s1, make_tea_s2).
```

## teach
- To make tea you need a tea bag and water.
- Making tea takes 5 to 7 minutes.
- Step 1 of making tea: boil the water.
- Step 2 of making tea: pour the water over the tea bag.

## checklist
- gives the steps in order: boil, pour

## variant
- change: no kettle
- request: How do I make tea without a kettle?
- situation: I only have a pot.
- expect: heats the water in a pot
"#;

fn gold() -> Vec<(&'static str, Vec<&'static str>)> {
    vec![
        ("How do I make a cup of tea?", vec!["ask_how: make_tea"]),
        ("Could you walk me through making tea?", vec!["ask_how: make_tea"]),
        ("What are the steps for tea?", vec!["ask_how: make_tea"]),
        ("How do I make tea without a kettle?", vec!["ask_how: make_tea"]),
        ("I have tea bags and a kettle.", vec!["inform: available(tea_bag); available(kettle)"]),
        ("I only have a pot.", vec!["inform: available(pot)"]),
        ("Thanks!", vec!["social: thanks"]),
        ("To make tea you need a tea bag and water.", vec!["tell: task(make_tea); needs(make_tea, tea_bag); needs(make_tea, water)"]),
        ("Making tea takes 5 to 7 minutes.", vec!["tell: takes_time(make_tea, 5, 7, minute)"]),
        ("Step 1 of making tea: boil the water.", vec!["tell: step(make_tea, make_tea_s1); action(make_tea_s1, boil); object_of(make_tea_s1, water)"]),
        ("Step 2 of making tea: pour the water over the tea bag.", vec!["tell: step(make_tea, make_tea_s2); action(make_tea_s2, pour); object_of(make_tea_s2, water); before(make_tea_s1, make_tea_s2)"]),
    ]
}

fn oracle() -> OracleParser {
    let mut o = OracleParser::default();
    for (t, acts) in gold() {
        o.insert(t, &acts.iter().map(|a| a.to_string()).collect::<Vec<_>>()).unwrap();
    }
    o
}

/// Passes when a reply gives boil before pour.
struct StepsJudge;
impl CheckJudge for StepsJudge {
    fn passes(&self, _s: &Scenario, _v: bool, turns: &[Turn]) -> (bool, String) {
        let ok = turns.iter().any(|t| {
            let a = t.agent.to_lowercase();
            matches!((a.find("boil"), a.find("pour")), (Some(x), Some(y)) if x < y)
        });
        (ok, String::new())
    }
}

#[test]
fn encounters_teach_repeat_transfer_retention() {
    let s = curriculum::parse_scenario(SCENARIO);
    assert_eq!(s.requests.len(), 3);
    assert_eq!(s.truth.len(), 11);
    assert_eq!(s.teach.len(), 4);
    assert!(s.variant.is_some());
    let dir = std::env::temp_dir().join(format!("sla_m18_curr_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let brain: SharedBrain = Brain::shared(&dir).unwrap();
    let mut cfg = CurriculumConfig {
        user_agent: Box::new(|b, sp| {
            let mut a = Agent::with_brain(Arc::clone(b), sp);
            a.ask_in_chat = false;
            a.set_parser(Box::new(oracle()));
            a
        }),
        teacher_agent: Box::new(|b| {
            let mut a = Agent::with_brain(Arc::clone(b), "teacher");
            a.source = "teacher:curriculum".into();
            a.ask_in_chat = false;
            a.set_parser(Box::new(oracle()));
            a
        }),
        judge: Box::new(StepsJudge),
        labeller: None,
        max_turns: 4,
        teacher_model: None,
    };
    let r = curriculum::run(&mut cfg, &brain, &[s], 0);
    println!("{}", curriculum::markdown(&r));
    assert_eq!(r.teach_first_try, 0, "nothing known before teaching");
    assert_eq!(r.taught_learned, 1, "the retry after the teach blocks passes");
    assert_eq!((r.repeat_passed, r.repeat_total), (1, 1));
    assert_eq!((r.retention_passed, r.retention_total), (1, 1));
    assert_eq!(r.transfer_total, 1);
    assert!(r.contradicting.is_empty(), "{:?}", r.contradicting);
    assert_eq!(r.teacher_meta_requests, 0);
    assert_eq!(r.hygiene.as_ref().unwrap().duplicates_after, 0);
    // Teach blocks are teacher knowledge.
    let b = sla_agent::brain::lock(&brain);
    let pf = &b.kb.vault.procedures["make_tea"];
    assert!(pf.facts().iter().filter(|f| f.atom.pred == "step").all(|f| f.source.starts_with("teacher")));
    drop(b);
    let _ = std::fs::remove_dir_all(&dir);
}

fn root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

/// The 30 seed procedures parse, enter as hypotheses and type-check.
#[test]
fn seed_procedures_are_well_typed_hypotheses() {
    let reg = sla_lexicon::Registry::builtin();
    let dir = root().join("data/curriculum/seed");
    let mut n = 0;
    let mut problems = Vec::new();
    for e in std::fs::read_dir(&dir).unwrap().flatten() {
        let p = e.path();
        let stem = p.file_stem().unwrap().to_str().unwrap().to_string();
        let pf = sla_core::ProcedureFile::parse(&std::fs::read_to_string(&p).unwrap(), &stem);
        assert_eq!(pf.meta().status, sla_core::RuleStatus::Hypothesis, "{stem}");
        let facts = pf.facts();
        assert!(facts.iter().filter(|f| f.atom.pred == "step").count() >= 3, "{stem}: fewer than 3 steps");
        for f in facts {
            if let Err(e) = reg.check(&f.atom) {
                problems.push(format!("{stem}: {} ({e:?})", f.atom));
            }
        }
        n += 1;
    }
    assert_eq!(n, 30);
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// The 20 held-out scenarios parse, their ground truth type-checks, and 15
/// have a transfer variant.
#[test]
fn held_out_scenarios_are_complete() {
    let reg = sla_lexicon::Registry::builtin();
    let all = curriculum::load_scenarios(&root().join("data/curriculum/scenarios"));
    assert_eq!(all.len(), 20);
    assert_eq!(all.iter().filter(|s| s.variant.is_some()).count(), 15);
    let mut problems = Vec::new();
    for s in &all {
        if s.requests.len() != 3 || s.teach.is_empty() || s.checklist.is_empty() || s.truth.len() < 5 {
            problems.push(format!("{}: incomplete", s.id));
        }
        for g in &s.truth {
            if let Err(e) = reg.check(g) {
                problems.push(format!("{}: {g} ({e:?})", s.id));
            }
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// Protocol rule 3: after teaching, a step that was not stored is sent again,
/// and nothing that was stored right.
#[test]
fn teacher_resends_only_what_was_lost() {
    let s = curriculum::parse_scenario(SCENARIO);
    let dir = std::env::temp_dir().join(format!("sla_m18_fix_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let brain: SharedBrain = Brain::shared(&dir).unwrap();
    let mut o = OracleParser::default();
    for (t, acts) in gold() {
        // Step 2 is lost: the translator gives nothing for it.
        if t.starts_with("Step 2") {
            o.insert(t, &["unknown:".to_string()]).unwrap();
        } else {
            o.insert(t, &acts.iter().map(|a| a.to_string()).collect::<Vec<_>>()).unwrap();
        }
    }
    let mut t = Agent::with_brain(Arc::clone(&brain), "teacher");
    t.ask_in_chat = false;
    t.set_parser(Box::new(o));
    for line in &s.teach {
        t.respond(line);
    }
    let fixes = curriculum::fix_lines(&brain, &s, &t.session_id);
    assert_eq!(fixes, vec!["Step 2 of making tea: pour the water over the tea bag.".to_string()]);
    let _ = std::fs::remove_dir_all(&dir);
}

/// The training scenarios parse and type-check, and never repeat a held-out
/// scenario or a seed procedure.
#[test]
fn training_scenarios_are_complete_and_apart() {
    let reg = sla_lexicon::Registry::builtin();
    let train = curriculum::load_scenarios(&root().join("data/curriculum/training"));
    assert!(train.len() >= 50, "{}", train.len());
    let held: Vec<String> = curriculum::load_scenarios(&root().join("data/curriculum/scenarios")).into_iter().map(|s| s.task).collect();
    let seeds: Vec<String> = std::fs::read_dir(root().join("data/curriculum/seed")).unwrap().flatten().map(|e| e.path().file_stem().unwrap().to_string_lossy().to_string()).collect();
    let mut problems = Vec::new();
    for s in &train {
        if held.contains(&s.task) || seeds.contains(&s.task) {
            problems.push(format!("{}: also a held-out scenario or seed", s.id));
        }
        if s.requests.len() != 3 || s.teach.is_empty() || s.checklist.is_empty() || s.truth.len() < 5 {
            problems.push(format!("{}: incomplete", s.id));
        }
        for g in &s.truth {
            if let Err(e) = reg.check(g) {
                problems.push(format!("{}: {g} ({e:?})", s.id));
            }
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("
"));
}

/// The ground-truth teacher answers the agent's questions with the right
/// teach lines, and says it does not know about anything else.
#[test]
fn ground_truth_teacher_answers_from_the_scenario() {
    use sla_agent::curiosity::Teacher;
    let s = curriculum::parse_scenario(SCENARIO);
    let mut t = curriculum::GroundTruthTeacher { scenarios: vec![s], answered: 0 };
    assert_eq!(t.answer("What do I need to make tea?"), "To make tea you need a tea bag and water.");
    assert_eq!(t.answer("How long does it take to make tea?"), "Making tea takes 5 to 7 minutes.");
    assert!(t.answer("How do I make tea? What do I need, and what is the first step?").contains("Step 1 of making tea"));
    assert_eq!(t.answer("How do I fix a bike?"), "I don't know.");
    assert_eq!(t.answered, 3);
}

/// The in-role teacher's variant lines are kept only in the fixed frames.
#[test]
fn variant_teacher_lines_keep_to_the_frames() {
    assert!(curriculum::in_teach_frame("A pot can replace a kettle."));
    assert!(curriculum::in_teach_frame("Step 1 of making tea: heat the water in a pot for 4 minutes."));
    assert!(!curriculum::in_teach_frame("Sure! Here is what you should know about tea."));
    assert!(!curriculum::in_teach_frame("Please say it back to me as a list."));
    let s = curriculum::parse_scenario(SCENARIO);
    let v = s.variant.clone().unwrap();
    sla_parser::claude::set_thread_mock(std::sync::Arc::new(|_role: &str, _prompt: &str| {
        Ok("Here you go:
- A pot can replace a kettle.
Step 1 of making tea: heat the water in the pot.
Thanks for asking!".to_string())
    }));
    let lines = curriculum::variant_teach_lines("haiku", &s, &v);
    sla_parser::claude::clear_thread_mock();
    assert_eq!(lines, vec!["A pot can replace a kettle.".to_string(), "Step 1 of making tea: heat the water in the pot.".to_string()]);
}
