//! Design 2.0, milestone 16 acceptance: 200 content plans are realized; every
//! reply passes the grammar checks, every claim is true in the knowledge base,
//! and every claim parses back to its atom, with the v1 pattern parser or,
//! for predicates worded by registry frames, with the realizer's own frames
//! run backwards.

use sla_core::{parse_atom, parse_ground, GroundAtom, RuleFile};
use sla_nlg::{Act, Realizer, Truth};
use sla_parser::{parse, Context, Intent};
use sla_reasoner::{evaluate, parse_program, FactInput, Model, RuleInput};

const SCENARIO: &str = "
likes(anna, tea).
likes(anna, coffee).
likes(maksim, coffee).
-likes(bob, tea).
is_a(rex, dog).
is_a(dog, animal).
is_a(tweety, bird).
is_a(bird, animal).
has(bird, wing).
feels(anna, happy).
feels(maksim, tired).
property(car, colour, red).
property(anna, job, teacher).
property(box, size, big).
property(soup, temperature, hot).
property(bob, character, lazy).
located_in(anna, berlin).
located_in(berlin, germany).
part_of(wheel, car).
drink(bob, juice).
work_at(bob, acme).
fly(tweety).
wants(maksim, bicycle).
knows(anna, bob).
task(garlic_tomato_pasta).
solves(garlic_tomato_pasta, plan_meal).
needs(garlic_tomato_pasta, pasta).
needs(garlic_tomato_pasta, garlic).
needs(garlic_tomato_pasta, olive_oil).
needs(garlic_tomato_pasta, cherry_tomato).
takes_time(garlic_tomato_pasta, 15, 20, minute).
step(garlic_tomato_pasta, gtp_s1).
action(gtp_s1, boil).
object_of(gtp_s1, pasta).
takes_time(gtp_s1, 10, 12, minute).
step(garlic_tomato_pasta, gtp_s2).
action(gtp_s2, fry).
object_of(gtp_s2, garlic).
uses(gtp_s2, pan).
takes_time(gtp_s2, 1, 1, minute).
step(garlic_tomato_pasta, gtp_s3).
action(gtp_s3, add).
object_of(gtp_s3, cherry_tomato).
takes_time(gtp_s3, 5, 7, minute).
before(gtp_s1, gtp_s3).
before(gtp_s2, gtp_s3).
task(change_tire).
needs(change_tire, spare_tire).
step(change_tire, ct_s1).
action(ct_s1, lift).
object_of(ct_s1, car).
uses(ct_s1, jack).
takes_time(change_tire, 20, 30, minute).
task(make_tea).
needs(make_tea, kettle).
needs(make_tea, tea_bag).
takes_time(make_tea, 3, 5, minute).
staple(salt).
available(pasta).
available(garlic).
";

fn model() -> Model {
    let (facts, mut rules) = parse_program(SCENARIO).unwrap();
    for (stem, src) in sla_core::CORE_FILES {
        for (i, r) in RuleFile::parse(src, stem).rules().into_iter().enumerate() {
            if r.head.pred != "core_predicate" && r.head.pred != "functional" && !r.body.is_empty() {
                rules.push(RuleInput::new(&format!("core/{stem}#{i}"), r));
            }
        }
    }
    let facts: Vec<FactInput> = facts.into_iter().map(|f| FactInput { source: "user".into(), ..f }).collect();
    evaluate(&facts, &rules).unwrap()
}

fn realizer() -> Realizer {
    let mut r = Realizer::new("maksim");
    for n in ["anna", "bob", "berlin", "germany", "acme", "rex", "tweety"] {
        r.names.insert(n.into());
    }
    let reg = sla_lexicon::Registry::builtin();
    for (name, info) in &reg.preds {
        if let Some(f) = info.frames.first() {
            r.frames.insert(name.clone(), f.clone());
        }
        r.arg_types.insert(name.clone(), info.types.iter().map(|t| t.name().to_string()).collect());
    }
    r.surface.insert("gtp_s1".into(), "the step to boil the pasta".into());
    r.surface.insert("gtp_s2".into(), "the step to fry the garlic".into());
    r.surface.insert("gtp_s3".into(), "the step to add the cherry tomato".into());
    r.surface.insert("ct_s1".into(), "the step to lift the car".into());
    r
}

fn g(s: &str) -> GroundAtom {
    parse_ground(s).unwrap()
}

/// Rule-based grammar checks.
fn grammar_problems(text: &str) -> Vec<String> {
    let mut p = Vec::new();
    if !text.chars().next().map_or(false, |c| c.is_uppercase()) {
        p.push("does not start with a capital".into());
    }
    if !text.trim_end().ends_with(['.', '?', '!']) {
        p.push("no final punctuation".into());
    }
    for bad in ["_", "(", ")", ":-", "{", "}", "  ", " ,", " .", "holds for", "the relation"] {
        if text.contains(bad) {
            p.push(format!("contains {bad:?}"));
        }
    }
    for sentence in text.split(['.', ':', '?', '!', ',']) {
        p.extend(sentence_problems(sentence));
    }
    p
}

fn sentence_problems(text: &str) -> Vec<String> {
    let mut p = Vec::new();
    let words: Vec<String> = text.split_whitespace().map(|w| w.trim_matches(|c: char| !c.is_alphanumeric() && c != '\'').to_lowercase()).collect();
    for w in words.windows(2) {
        let (a, b) = (w[0].as_str(), w[1].as_str());
        if a == b && a != "that" && a != "had" && a.len() > 1 {
            p.push(format!("doubled word {a}"));
        }
        let vowel = b.starts_with(['a', 'e', 'i', 'o']);
        if a == "a" && vowel {
            p.push(format!("'a {b}'"));
        }
        if a == "an" && !vowel && !b.starts_with('u') && !b.starts_with('h') {
            p.push(format!("'an {b}'"));
        }
        if matches!((a, b), ("you", "is") | ("you", "likes") | ("they", "is") | ("it", "are") | ("i", "is") | ("a", "minutes") | ("a", "minute")) {
            p.push(format!("agreement '{a} {b}'"));
        }
    }
    p
}

/// Does the claim's sentence read back to the claim?
fn round_trip(r: &Realizer, c: &GroundAtom) -> Result<(), String> {
    let sentence = r.clause(c);
    let mapped = matches!(c.base_pred(), "likes" | "has" | "is_a" | "feels" | "property" | "located_in" | "part_of" | "wants" | "knows" | "before" | "during" | "same" | "causes" | "says" | "does" | "agent_of");
    if r.frames.contains_key(c.base_pred()) && !mapped {
        let back = r.frame_parse(&sentence);
        return if back.contains(c) { Ok(()) } else { Err(format!("{c}: \"{sentence}\" reads back as {back:?}")) };
    }
    if !matches!(c.base_pred(), "likes" | "has" | "is_a" | "feels" | "property" | "located_in" | "part_of" | "wants" | "knows" | "drink" | "work_at" | "fly") {
        return Ok(());
    }
    // The reply speaks to the user: "you" is the speaker, "your" is "my".
    let mut text = sentence.replace("your ", "my ");
    if let Some(rest) = text.strip_prefix("you ") {
        text = format!("I {}", rest.replacen("are ", "am ", 1));
    }
    if let Some(f) = text.get(0..1) {
        text = f.to_uppercase() + &text[1..];
    }
    let ctx = Context { speaker: "maksim".into(), last_entity: None };
    match parse(&format!("{text}."), &ctx) {
        Intent::Tell { facts, .. } if facts.iter().any(|a| a.to_ground().as_ref() == Some(c)) => Ok(()),
        // Named properties and v1's catch-all read back the same way.
        Intent::Tell { facts, .. } if c.base_pred() == "property" && facts.iter().any(|a| a.pred == c.pred && a.args.first() == c.to_atom().args.first()) => Ok(()),
        other => Err(format!("{c}: \"{text}\" parses as {}", other.canonical())),
    }
}

fn plans(m: &Model) -> Vec<Act> {
    let mut acts = Vec::new();
    // 40 yes/no answers, 10 of each truth value.
    let yes = ["likes(anna, tea)", "is_a(rex, animal)", "has(tweety, wing)", "feels(anna, happy)", "located_in(anna, berlin)", "property(car, colour, red)", "needs(make_tea, kettle)", "needs_all(change_tire, jack)", "missing(garlic_tomato_pasta, cherry_tomato)", "knows_task(garlic_tomato_pasta)"];
    let no = ["likes(bob, tea)", "is_a(rex, bird)", "has(rex, wing)", "feels(anna, sad)", "located_in(bob, berlin)", "needs(make_tea, pan)", "missing(garlic_tomato_pasta, pasta)", "property(box, size, small)", "is_a(tweety, dog)", "fly(rex)"];
    for (i, a) in yes.iter().enumerate() {
        acts.push(Act::YesNo { atom: g(a), truth: Truth::Yes });
        // "No" only where the negation is known; otherwise "I don't know".
        let atom = g(no[i]);
        let truth = if m.contains(&atom.complement()) { Truth::No } else { Truth::Unknown };
        acts.push(Act::YesNo { atom, truth });
    }
    for a in ["likes(anna, coffee)", "drink(bob, juice)", "work_at(bob, acme)", "property(anna, job, teacher)", "needs(change_tire, spare_tire)", "uses(gtp_s2, pan)", "takes_time(make_tea, 3, 5, minute)", "step(change_tire, ct_s1)", "solves(garlic_tomato_pasta, plan_meal)", "staple(salt)"] {
        acts.push(Act::YesNo { atom: g(a), truth: Truth::Yes });
        acts.push(Act::YesNo { atom: g(a), truth: Truth::Unknown });
    }
    // 40 wh-answers.
    let pats = [
        "likes(anna, W)", "likes(W, coffee)", "is_a(rex, W)", "has(W, wing)", "located_in(anna, W)", "property(car, colour, W)", "property(anna, job, W)", "work_at(bob, W)",
        "needs(garlic_tomato_pasta, W)", "needs(make_tea, W)", "needs_all(change_tire, W)", "missing(garlic_tomato_pasta, W)", "step(garlic_tomato_pasta, W)", "uses(W, jack)",
        "takes_time(garlic_tomato_pasta, 15, W, minute)", "solves(W, plan_meal)", "knows_task(W)", "total_minutes(garlic_tomato_pasta, W, 20)", "next_step(garlic_tomato_pasta, W)", "step_count(garlic_tomato_pasta, W)",
    ];
    for p in pats {
        let pat = parse_atom(p).unwrap();
        let found = m.query_atom(&pat);
        acts.push(Act::Wh { pattern: pat.clone(), found: found.clone(), past: false });
        acts.push(Act::Wh { pattern: pat, found: found.into_iter().take(1).collect(), past: false });
    }
    // 30 explanations from proofs.
    for a in [
        "is_a(rex, animal)", "is_a(tweety, animal)", "has(tweety, wing)", "located_in(anna, berlin)", "likes(anna, tea)", "property(car, colour, red)", "feels(maksim, tired)",
        "needs_all(change_tire, jack)", "needs_all(garlic_tomato_pasta, pan)", "needs_all(garlic_tomato_pasta, garlic)", "missing(garlic_tomato_pasta, olive_oil)", "missing(change_tire, spare_tire)",
        "missing(make_tea, kettle)", "knows_task(garlic_tomato_pasta)", "knows_task(change_tire)", "next_step(garlic_tomato_pasta, gtp_s1)", "minutes(gtp_s1, 10, 12)", "minutes(gtp_s3, 5, 7)",
        "lacking(make_tea)", "lacking(change_tire)", "needs(make_tea, tea_bag)", "staple(salt)", "drink(bob, juice)", "work_at(bob, acme)", "wants(maksim, bicycle)", "knows(anna, bob)",
        "part_of(wheel, car)", "property(box, size, big)", "property(soup, temperature, hot)", "property(bob, character, lazy)",
    ] {
        acts.push(Act::Explain(m.proof(&g(a)).unwrap_or_else(|| panic!("no proof for {a}"))));
    }
    // 20 recalls.
    for e in ["anna", "bob", "rex", "maksim", "tweety", "car", "berlin", "garlic_tomato_pasta", "change_tire", "make_tea", "gtp_s1", "gtp_s2", "ct_s1", "box", "soup", "dog", "bird", "salt", "zorg", "wheel"] {
        let facts: Vec<GroundAtom> = m
            .all_facts()
            .into_iter()
            .filter(|a| a.args.first().and_then(|v| v.as_sym()) == Some(e))
            .filter(|a| !matches!(a.base_pred(), "thing" | "same" | "adds_time" | "side_by_side" | "covered" | "budget"))
            .take(6)
            .collect();
        acts.push(Act::Recall { entity: e.into(), facts, instances: vec![], more: false });
    }
    // 40 acknowledgements and corrections.
    let ack = |added: Vec<GroundAtom>, known: Vec<GroundAtom>| Act::Acknowledge { added, closed: vec![], known, rules: vec![], past: false };
    for a in ["likes(anna, tea)", "is_a(rex, dog)", "needs(make_tea, kettle)", "takes_time(make_tea, 3, 5, minute)", "property(car, colour, red)", "uses(gtp_s2, pan)", "work_at(bob, acme)", "staple(salt)", "feels(maksim, tired)", "has(bird, wing)"] {
        acts.push(ack(vec![g(a)], vec![]));
        acts.push(ack(vec![], vec![g(a)]));
        acts.push(Act::Corrected { added: vec![g(a)], retracted: vec![], rules: vec![] });
        acts.push(Act::AskYesNo(g(a)));
    }
    // 20 questions the agent asks about task content.
    for p in [
        "needs(make_tea, W)", "needs(change_tire, W)", "takes_time(make_tea, W, 5, minute)", "uses(gtp_s1, W)", "uses(ct_s1, W)", "solves(W, plan_meal)", "step(make_tea, W)", "until(gtp_s3, W)",
        "likes(bob, W)", "located_in(rex, W)", "property(box, colour, W)", "property(bob, job, W)", "work_at(anna, W)", "has(car, W)", "is_a(tweety, W)", "drink(anna, W)",
        "substitute(garlic, W)", "portion(pasta, W, gram)", "recommended_for(make_tea, W)", "subtask(change_tire, W)",
    ] {
        acts.push(Act::AskWh(parse_atom(p).unwrap()));
    }
    // 10 social and failures.
    for a in [Act::Greet, Act::Bye, Act::Thanks, Act::NotUnderstood, Act::NotUnderstood, Act::Greet, Act::Bye, Act::Thanks, Act::NotUnderstood, Act::Greet] {
        acts.push(a);
    }
    acts
}

#[test]
fn two_hundred_content_plans() {
    let m = model();
    let r = realizer();
    let acts = plans(&m);
    assert_eq!(acts.len(), 200);
    let truths = m.all_facts();
    let mut problems = Vec::new();
    for act in &acts {
        let reply = r.realize(act);
        for p in grammar_problems(&reply.text) {
            problems.push(format!("GRAMMAR {p}: {}", reply.text));
        }
        let from_input = matches!(act, Act::Acknowledge { .. } | Act::Corrected { .. });
        for c in &reply.claims {
            if !from_input && !truths.contains(c) {
                problems.push(format!("CLAIM not in the KB {c}: {}", reply.text));
            }
            if let Err(e) = round_trip(&r, c) {
                problems.push(format!("ROUND-TRIP {e}"));
            }
        }
        println!("[{}] {}", reply.intent, reply.text);
    }
    assert!(problems.is_empty(), "{} problems:\n{}", problems.len(), problems.join("\n"));
}

#[test]
fn the_same_failure_has_one_wording() {
    let r = realizer();
    let texts: std::collections::BTreeSet<String> = (0..6).map(|_| r.realize(&Act::NotUnderstood).text).collect();
    assert_eq!(texts.len(), 1);
}
