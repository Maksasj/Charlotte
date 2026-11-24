//! Learned split connectors: the words people put between two requests ("on top of that", "besides").
//!
//! The reader splits a message with several requests at the lexicon's connectors. Here the connectors are
//! found in chats instead: where a message has wordings of two different intents, what lies between them
//! is a candidate. A wording is proposed only when
//! - enough messages have it between two requests,
//! - most of the messages that carry it at all (with any request in them) have it there, and
//! - the same holds on conversations held out from the search.
//!
//! The result is a `phrases` candidate for the lexicon, to be promoted by hand.

use crate::learn::Corpus;
use crate::understand::LexiconReader;
use charlotte_core::*;
use charlotte_vault::Note;
use serde_json::{json, Map};
use std::collections::{BTreeMap, BTreeSet, HashSet};

#[derive(Clone, Debug)]
pub struct Options {
    /// Messages with the wording between two requests, on the search side.
    pub min_support: usize,
    /// Share of the messages carrying the wording (and any request) that have it between two requests.
    pub min_precision: f64,
    /// The same share on held-out conversations, over at least `min_held` messages.
    pub min_held: usize,
    pub max_gram: usize,
    /// One conversation in this many is held out.
    pub held_every: usize,
}
impl Default for Options {
    fn default() -> Self {
        Options { min_support: 5, min_precision: 0.6, min_held: 3, max_gram: 4, held_every: 5 }
    }
}

#[derive(Clone, Debug)]
pub struct Connector {
    pub phrase: String,
    pub support: usize,
    pub precision: f64,
    pub held_n: usize,
    pub held_precision: f64,
    pub example: String,
}

#[derive(Default)]
struct Count {
    between: usize,
    carried: usize,
    held_between: usize,
    held_carried: usize,
    example: String,
}

fn words(text: &str) -> Vec<String> {
    text.split_whitespace().map(|w| w.trim_matches(|c: char| !c.is_alphanumeric() && c != '\'').to_lowercase()).filter(|w| !w.is_empty()).collect()
}
fn grams(words: &[String], max: usize) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for n in 1..=max {
        for w in words.windows(n) {
            out.insert(w.join(" "));
        }
    }
    out
}

/// Connector candidates the vault does not have yet.
pub fn induce(mem: &dyn Memory, corpus: &Corpus, opts: &Options) -> Vec<Connector> {
    let reader = LexiconReader::new(mem);
    let known: Vec<Vec<String>> = mem.lexicon().connectors.iter().map(|c| words(c)).collect();
    let covered = |g: &str| {
        let g: Vec<String> = g.split(' ').map(str::to_string).collect();
        known.iter().any(|k| k.windows(g.len()).any(|w| w == g.as_slice()))
    };
    let mut counts: BTreeMap<String, Count> = BTreeMap::new();
    for (ci, c) in corpus.convs.iter().enumerate() {
        let held = ci % opts.held_every == opts.held_every - 1;
        for t in c.turns.iter().filter(|t| t.role == "user" && !t.text.trim().is_empty()) {
            let text = t.text.split_whitespace().collect::<Vec<_>>().join(" ");
            if !reader.has_intent(&text) {
                continue;
            }
            let between: HashSet<String> = reader.gaps(&text).iter().flat_map(|g| grams(&words(g), opts.max_gram)).collect();
            for g in grams(&words(&text), opts.max_gram) {
                let e = counts.entry(g.clone()).or_default();
                match (held, between.contains(&g)) {
                    (false, b) => {
                        e.carried += 1;
                        if b {
                            e.between += 1;
                            if e.example.is_empty() {
                                e.example = text.clone();
                            }
                        }
                    }
                    (true, b) => {
                        e.held_carried += 1;
                        e.held_between += b as usize;
                    }
                }
            }
        }
    }
    let mut found: Vec<Connector> = counts
        .into_iter()
        .filter(|(g, e)| {
            e.between >= opts.min_support
                && e.between as f64 / e.carried as f64 >= opts.min_precision
                && e.held_carried >= opts.min_held
                && e.held_between as f64 / e.held_carried as f64 >= opts.min_precision
                && g.chars().any(|c| c.is_alphabetic())
                && !covered(g)
        })
        .map(|(g, e)| Connector {
            phrase: g,
            support: e.between,
            precision: e.between as f64 / e.carried as f64,
            held_n: e.held_carried,
            held_precision: e.held_between as f64 / e.held_carried as f64,
            example: e.example,
        })
        .collect();
    // A longer wording that holds just as often stands for the shorter ones inside it.
    let all = found.clone();
    found.retain(|c| {
        !all.iter().any(|o| o.phrase.len() > c.phrase.len() && format!(" {} ", o.phrase).contains(&format!(" {} ", c.phrase)) && o.support as f64 >= 0.8 * c.support as f64)
    });
    found.sort_by(|a, b| b.support.cmp(&a.support).then(a.phrase.cmp(&b.phrase)));
    found
}

pub fn candidate_note(found: &[Connector], source: &str) -> Option<Note> {
    if found.is_empty() {
        return None;
    }
    let id = "phrases-connectors".to_string();
    let mut front = Map::new();
    front.insert("kind".into(), "phrases".into());
    front.insert("lexicon".into(), "[[lexicon]]".into());
    front.insert("as".into(), "connectors".into());
    let round = |x: f64| (x * 1000.0).round() / 1000.0;
    let list: Vec<_> = found.iter().map(|c| json!({ "text": c.phrase, "support": c.support, "precision": round(c.precision), "held_precision": round(c.held_precision), "learned": true })).collect();
    front.insert("phrases".into(), list.into());
    front.insert("induced_from".into(), source.into());
    front.insert("status".into(), "candidate".into());
    let mut body = format!("Wordings that join two requests in one message, from {source}.\n\n");
    for c in found {
        body.push_str(&format!("- `{}`: between two requests in {} messages ({:.0}% of those that have it; {:.0}% of {} held out). E.g. \"{}\"\n", c.phrase, c.support, c.precision * 100.0, c.held_precision * 100.0, c.held_n, c.example));
    }
    Some(Note { id: id.clone(), path: format!("candidates/{id}.md"), front, body })
}
