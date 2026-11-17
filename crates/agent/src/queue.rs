//! The review queue for new words (Design 2.0, "closed vocabulary, one door").
//!
//! A relation no registered predicate fits arrives from the translator as
//! `new_pred` with a definition and the sentence it came from. It is queued
//! (`vault/queue/words.jsonl`), and the facts that use it are kept with the
//! queue item: pending, not believed, so the agent cannot say them. A reviewer
//! approves it with a sentence frame: the predicate is added to the vault's
//! lexicon (`vault/lexicon/learned_words.md`) and its facts are stored. A
//! rejected word's facts are dropped.

use crate::Brain;
use serde::{Deserialize, Serialize};
use sla_core::{parse_ground, Fact, GroundAtom, RuleFile, Timestamp};
use std::io;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WordItem {
    pub pred: String,
    pub arity: usize,
    pub definition: String,
    /// Who used it.
    pub source: String,
    /// Facts waiting for the word, as text.
    pub facts: Vec<String>,
    /// "pending", "approved" or "rejected".
    pub status: String,
    #[serde(default)]
    pub frame: Option<String>,
}

fn path(b: &Brain) -> PathBuf {
    b.kb.vault.root.join("queue").join("words.jsonl")
}

pub fn load(b: &Brain) -> Vec<WordItem> {
    std::fs::read_to_string(path(b)).unwrap_or_default().lines().filter_map(|l| serde_json::from_str(l).ok()).collect()
}

fn save(b: &Brain, items: &[WordItem]) -> io::Result<()> {
    let p = path(b);
    std::fs::create_dir_all(p.parent().unwrap())?;
    let text: String = items.iter().filter_map(|i| serde_json::to_string(i).ok()).map(|l| l + "\n").collect();
    std::fs::write(p, text)
}

/// Words waiting for review.
pub fn pending(b: &Brain) -> Vec<WordItem> {
    load(b).into_iter().filter(|i| i.status == "pending").collect()
}

/// Queue a new word (or add a fact to its queue item).
pub fn enqueue(b: &Brain, fact: &GroundAtom, definition: &str, source: &str) -> io::Result<()> {
    let mut items = load(b);
    let text = fact.to_string();
    match items.iter_mut().find(|i| i.pred == fact.pred && i.status == "pending") {
        Some(i) => {
            if !i.facts.contains(&text) {
                i.facts.push(text);
            }
        }
        None => items.push(WordItem {
            pred: fact.pred.clone(),
            arity: fact.args.len(),
            definition: definition.to_string(),
            source: source.to_string(),
            facts: vec![text],
            status: "pending".into(),
            frame: None,
        }),
    }
    save(b, &items)
}

/// Approve a word with a sentence frame (`{1} grows on {2}`): it joins the
/// lexicon and its waiting facts are stored. Returns how many facts.
pub fn approve(b: &mut Brain, pred: &str, frame: &str, now: Timestamp) -> io::Result<usize> {
    let mut items = load(b);
    let Some(i) = items.iter_mut().find(|i| i.pred == pred && i.status == "pending") else {
        return Err(io::Error::new(io::ErrorKind::NotFound, format!("no pending word {pred}")));
    };
    i.status = "approved".into();
    i.frame = Some(frame.to_string());
    let item = i.clone();
    // The lexicon file of approved words.
    let lex = b.kb.vault.root.join("lexicon").join("learned_words.md");
    let mut src = std::fs::read_to_string(&lex).unwrap_or_else(|_| {
        "---\nid: learned_words\nstatus: confirmed\nconf: 1\nevidence: 0\ncounter: 0\nmodule: lexicon\nsource: review\n---\n# Learned words\n\nWords approved from the review queue.\n\n```datalog\n```\n".into()
    });
    let types = vec!["any"; item.arity].join(" ");
    let q = |s: &str| s.replace('"', "'");
    let entry = format!("pred({pred}, \"{types}\", \"{}\").\nframe({pred}, \"{}\").\n", q(&item.definition), q(frame));
    if let Some(end) = src.rfind("```") {
        src.insert_str(end, &entry);
    }
    std::fs::create_dir_all(lex.parent().unwrap())?;
    std::fs::write(&lex, &src)?;
    b.kb.vault.lexicon.insert("learned_words".into(), RuleFile::parse(&src, "learned_words"));
    let mut n = 0;
    for f in &item.facts {
        if let Ok(g) = parse_ground(f) {
            b.kb.assert_fact(Fact::new(g, &item.source), now, &format!("the word {pred} was approved"));
            n += 1;
        }
    }
    b.kb.log_change(now, "approve_word", pred, frame);
    save(b, &items)?;
    b.save()?;
    Ok(n)
}

/// Reject a word: its waiting facts are dropped.
pub fn reject(b: &mut Brain, pred: &str, now: Timestamp) -> io::Result<()> {
    let mut items = load(b);
    if let Some(i) = items.iter_mut().find(|i| i.pred == pred && i.status == "pending") {
        i.status = "rejected".into();
    }
    b.kb.log_change(now, "reject_word", pred, "reviewer rejected it");
    save(b, &items)
}
