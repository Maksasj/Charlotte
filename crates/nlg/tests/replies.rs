//! Milestone 5 acceptance: 100 replies over a scenario knowledge base are
//! grammatical (rule-based checks) and never state anything not in the KB:
//! every claim is true in the model, and every claim's sentence parses back
//! (with our own parser) to exactly that atom.

use sla_core::{parse_atom, parse_ground, parse_rule, GroundAtom};
use sla_nlg::{Act, Realizer, Truth};
use sla_parser::{parse, Context, Intent};
use sla_reasoner::{evaluate, parse_program, FactInput, Model};

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
located_in(anna, berlin).
located_in(berlin, germany).
part_of(wheel, car).
drink(bob, juice).
work_at(bob, acme).
fly(tweety).
wants(maksim, bicycle).
knows(anna, bob).
is_a(X, Z) :- is_a(X, Y), is_a(Y, Z).
has(X, Y) :- is_a(X, C), has(C, Y).
located_in(X, Z) :- located_in(X, Y), located_in(Y, Z).
wants(X, D) :- likes(X, D).
";

fn model() -> Model {
    let (facts, rules) = parse_program(SCENARIO).unwrap();
    let facts: Vec<FactInput> = facts.into_iter().map(|f| FactInput { source: "user".into(), ..f }).collect();
    evaluate(&facts, &rules).unwrap()
}

fn realizer() -> Realizer {
    let mut r = Realizer::new("maksim");
    for n in ["anna", "bob", "berlin", "germany", "acme"] {
        r.names.insert(n.into());
    }
    r
}

/// Rule-based grammar checks. Returns the problems found.
pub fn grammar_problems(text: &str) -> Vec<String> {
    let mut p = Vec::new();
    if !text.chars().next().map_or(false, |c| c.is_uppercase()) {
        p.push("does not start with a capital".into());
    }
    if !text.ends_with(['.', '?', '!']) {
        p.push("no final punctuation".into());
    }
    for bad in ["_", "(", ")", ":-", "\"", "\u{1}", "\u{2}", "  ", " ,", " ."] {
        if text.contains(bad) {
            p.push(format!("contains {bad:?}"));
        }
    }
    for sentence in text.split(['.', ':', '?', '!']) {
        p.extend(sentence_problems(sentence));
    }
    p
}

fn sentence_problems(text: &str) -> Vec<String> {
    let mut p = Vec::new();
    let words: Vec<String> = text
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric() && c != '\'').to_lowercase())
        .collect();
    let bad_pairs = [
        ("you", "is"), ("you", "likes"), ("you", "has"), ("you", "does"), ("you", "wants"), ("they", "is"), ("they", "likes"),
        ("they", "has"), ("they", "does"), ("it", "are"), ("it", "like"), ("it", "have"), ("i", "is"), ("does", "likes"),
        ("not", "likes"), ("to", "likes"), ("do", "likes"),
    ];
    for w in words.windows(2) {
        let (a, b) = (w[0].as_str(), w[1].as_str());
        if a == b && a != "that" {
            p.push(format!("doubled word {a}"));
        }
        let vowel = b.starts_with(['a', 'e', 'i', 'o']);
        if a == "a" && vowel {
            p.push(format!("'a {b}'"));
        }
        if a == "an" && !vowel && !b.starts_with('u') && !b.starts_with('h') {
            p.push(format!("'an {b}'"));
        }
        if bad_pairs.contains(&(a, b)) {
            p.push(format!("agreement '{a} {b}'"));
        }
    }
    p
}

/// Predicates the parser can read back.
fn round_trippable(a: &GroundAtom) -> bool {
    matches!(
        a.base_pred(),
        "likes" | "has" | "is_a" | "feels" | "property" | "located_in" | "part_of" | "wants" | "knows" | "drink" | "work_at" | "fly"
    )
}

/// Render the claim as a sentence, turn "you" back into "I", and parse it.
fn round_trip(r: &Realizer, a: &GroundAtom) -> Result<(), String> {
    let c = r.clause(a);
    let starts_you = c.starts_with("you ");
    let words: Vec<String> = c
        .split(' ')
        .enumerate()
        .map(|(i, w)| match (i, w) {
            (0, "you") => "I".to_string(),
            (_, "you") => "me".to_string(),
            (_, "your") => "my".to_string(),
            (1, "are") if starts_you => "am".to_string(),
            _ => w.to_string(),
        })
        .collect();
    let sentence = format!("{}.", words.join(" "));
    match parse(&sentence, &Context::default()) {
        Intent::Tell { facts, rules, .. } if rules.is_empty() && facts.len() == 1 && facts[0].to_ground().as_ref() == Some(a) => Ok(()),
        other => Err(format!("{a} -> {sentence:?} -> {}", other.canonical())),
    }
}

fn acts(m: &Model) -> Vec<Act> {
    let g = |s: &str| parse_ground(s).unwrap();
    let mut acts = Vec::new();
    // 30 yes answers over model facts (given and derived).
    let mut facts: Vec<GroundAtom> = m.all_facts().into_iter().filter(|a| !a.is_strong_negation()).collect();
    facts.sort();
    for a in facts.iter().take(30) {
        acts.push(Act::YesNo { atom: a.clone(), truth: Truth::Yes });
    }
    // 2 known-no, 2 was-true, 8 unknown.
    acts.push(Act::YesNo { atom: g("likes(bob, tea)"), truth: Truth::No });
    acts.push(Act::YesNo { atom: g("likes(bob, tea)"), truth: Truth::No });
    acts.push(Act::YesNo { atom: g("likes(anna, tea)"), truth: Truth::WasTrue });
    acts.push(Act::YesNo { atom: g("drink(bob, juice)"), truth: Truth::WasTrue });
    for u in [
        "likes(bob, coffee)", "is_a(rex, cat)", "feels(bob, sad)", "located_in(rex, paris)", "has(rex, wing)",
        "property(car, size, small)", "likes(maksim, tea)", "work_at(anna, acme)",
    ] {
        acts.push(Act::YesNo { atom: g(u), truth: Truth::Unknown });
    }
    // 15 wh-questions, answered from the model.
    for p in [
        "likes(anna, W)", "likes(W, coffee)", "is_a(rex, W)", "is_a(tweety, W)", "located_in(anna, W)", "property(car, colour, W)",
        "property(anna, job, W)", "has(tweety, W)", "wants(maksim, W)", "work_at(bob, W)", "likes(bob, W)", "located_in(rex, W)",
        "property(bob, job, W)", "knows(anna, W)", "feels(anna, W)",
    ] {
        let pat = parse_atom(p).unwrap();
        let found = m.query_atom(&pat);
        acts.push(Act::Wh { pattern: pat, found, past: false });
    }
    // 10 explanations from proofs.
    for a in [
        "is_a(rex, animal)", "is_a(tweety, animal)", "has(tweety, wing)", "located_in(anna, germany)", "wants(anna, tea)",
        "wants(anna, coffee)", "wants(maksim, coffee)", "likes(anna, tea)", "property(car, colour, red)", "feels(maksim, tired)",
    ] {
        acts.push(Act::Explain(m.proof(&g(a)).unwrap_or_else(|| panic!("no proof for {a}"))));
    }
    // 8 recalls.
    for e in ["anna", "bob", "rex", "maksim", "tweety", "car", "berlin", "zorg"] {
        let facts: Vec<GroundAtom> = m.all_facts().into_iter().filter(|a| a.args.first().and_then(|v| v.as_sym()) == Some(e)).collect();
        acts.push(Act::Recall { entity: e.into(), facts });
    }
    // 10 acknowledgements.
    let ack = |added: Vec<GroundAtom>, closed: Vec<GroundAtom>, known: Vec<GroundAtom>| Act::Acknowledge { added, closed, known, rules: vec![], past: false };
    acts.push(ack(vec![g("likes(anna, tea)")], vec![], vec![]));
    acts.push(ack(vec![g("-likes(bob, tea)")], vec![], vec![]));
    acts.push(ack(vec![g("feels(maksim, tired)")], vec![], vec![]));
    acts.push(ack(vec![g("likes(anna, tea)"), g("likes(anna, coffee)")], vec![], vec![]));
    acts.push(ack(vec![], vec![], vec![g("is_a(rex, dog)")]));
    acts.push(ack(vec![g("-likes(bob, tea)")], vec![g("likes(bob, tea)")], vec![]));
    acts.push(Act::Acknowledge { added: vec![], closed: vec![], known: vec![], rules: vec![parse_rule("wants(X, D) :- likes(X, D).").unwrap()], past: false });
    acts.push(ack(vec![g("located_in(anna, berlin)")], vec![], vec![]));
    acts.push(Act::Corrected { added: vec![g("is_a(rex, dog)")], retracted: vec![g("is_a(rex, cat)")], rules: vec![] });
    acts.push(Act::Corrected { added: vec![g("property(car, colour, red)")], retracted: vec![], rules: vec![] });
    // 10 curiosity questions.
    for a in ["likes(bob, coffee)", "is_a(rex, animal)", "feels(maksim, happy)", "property(anna, job, teacher)"] {
        acts.push(Act::AskYesNo(g(a)));
    }
    for p in ["likes(bob, W)", "located_in(rex, W)", "property(box, colour, W)", "property(bob, job, W)"] {
        acts.push(Act::AskWh(parse_atom(p).unwrap()));
    }
    acts.push(Act::AskRule(parse_rule("drink(X, coffee) :- likes(X, coffee).").unwrap()));
    acts.push(Act::AskContradiction(g("likes(bob, tea)"), g("-likes(bob, tea)")));
    // 7 social.
    for a in [Act::Greet, Act::Greet, Act::Bye, Act::Bye, Act::NotUnderstood, Act::NotUnderstood, Act::Thanks] {
        acts.push(a);
    }
    acts
}

#[test]
fn hundred_replies() {
    let m = model();
    let r = realizer();
    let acts = acts(&m);
    assert_eq!(acts.len(), 100);
    let truths: Vec<GroundAtom> = m.all_facts();
    let mut problems = Vec::new();
    for act in &acts {
        let reply = r.realize(act);
        for g in grammar_problems(&reply.text) {
            problems.push(format!("GRAMMAR {g}: {}", reply.text));
        }
        // Acknowledged facts are the user's new input; was-true answers come
        // from history. Everything else must be in the model.
        let from_input = matches!(act, Act::Acknowledge { .. } | Act::Corrected { .. } | Act::YesNo { truth: Truth::WasTrue, .. });
        for c in &reply.claims {
            if !truths.contains(c) && !from_input {
                problems.push(format!("CLAIM not in KB {c}: {}", reply.text));
            }
            if round_trippable(c) {
                if let Err(e) = round_trip(&r, c) {
                    problems.push(format!("ROUND-TRIP {e}"));
                }
            }
        }
        println!("[{}] {}", reply.intent, reply.text);
    }
    assert!(problems.is_empty(), "{} problems:\n{}", problems.len(), problems.join("\n"));
}

#[test]
fn explanation_uses_proof() {
    let m = model();
    let r = realizer();
    let p = m.proof(&parse_ground("is_a(rex, animal)").unwrap()).unwrap();
    let text = r.realize(&Act::Explain(p)).text;
    assert!(text.starts_with("Rex is an animal because it is a dog and dog is an animal"), "{text}");
    assert!(text.contains("You told me that rex is a dog"), "{text}");
}
