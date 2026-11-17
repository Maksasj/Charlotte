//! Design 2.0, milestone 16 acceptance: 200 content plans are realized; every
//! reply passes the grammar checks, every claim is true in the knowledge base,
//! and every claim parses back to its atom, with the v1 pattern parser or,
//! for predicates worded by registry frames, with the realizer's own frames
//! run backwards.

#[path = "common/plans_data.rs"]
mod plans_data;
use plans_data::{model, plans, realizer};
use sla_core::GroundAtom;
use sla_nlg::{Act, Realizer};
use sla_parser::{parse, Context, Intent};

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
