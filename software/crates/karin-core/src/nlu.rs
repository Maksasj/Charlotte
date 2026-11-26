//! Language layer: turns what the user said (often a noisy speech transcript)
//! into the canonical sentences the vault's patterns are written for.
//!
//! The code here is generic. Every word it knows comes from lexicon facts in
//! the vault, so the language grows by adding facts, never by editing Rust:
//!
//! | fact                                   | effect                                                        |
//! |----------------------------------------|---------------------------------------------------------------|
//! | `alias(from, to)`                      | names / mishearings, applied first ("karen" → "karin")        |
//! | `rewrite(from, to)`                    | canonical wording ("what's" → "what is", "i love" → "i like") |
//! | `filler(text, at=start\|end\|any)`     | words that carry no meaning ("please", "so", "right now")     |
//! | `vocative(text)`                       | ways of addressing Karin, dropped at the start / end          |
//! | `vocative_block(word, side)`           | keeps a vocative when it is really the subject / object       |
//! | `greeting(word)`                       | "hey karin what can you do" → "what can you do"               |
//! | `clause_joiner(word)`                  | where a long utterance may be split into clauses              |
//! | `fixed_phrase(text)`                   | never split inside ("rock and roll")                          |
//! | `pronoun(word, kind)`                  | resolved against the chat's most recent `mention(entity, kind)`|
//! | `hint(word, text)`                     | what to suggest when nothing matched                          |
//!
//! Phrases are matched on whole lowercase tokens; untouched tokens keep their
//! original spelling, so captured names stay capitalised ("Maksim").

use crate::types::{Fact, Scope};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
struct Tok {
    raw: String,
    low: String,
}

impl Tok {
    fn new(raw: &str) -> Self {
        Self {
            raw: raw.to_string(),
            low: raw.to_lowercase(),
        }
    }
}

type Phrase = Vec<String>;

#[derive(Default)]
pub struct Lexicon {
    aliases: Vec<(Phrase, Vec<String>)>,
    rewrites: Vec<(Phrase, Vec<String>)>,
    fill_start: Vec<Phrase>,
    fill_end: Vec<Phrase>,
    fill_any: Vec<Phrase>,
    vocatives: Vec<Phrase>,
    block_after: HashSet<String>,
    block_before: HashSet<String>,
    greetings: HashSet<String>,
    joiners: HashSet<String>,
    fixed: Vec<Phrase>,
    pronouns: HashMap<String, String>,
    hints: Vec<(Phrase, String)>,
}

fn phrase(s: &str) -> Phrase {
    tokenize(s).into_iter().map(|t| t.low).collect()
}

fn words(s: &str) -> Vec<String> {
    tokenize(s).into_iter().map(|t| t.raw).collect()
}

impl Lexicon {
    pub fn from_facts(facts: &[Fact]) -> Self {
        let mut lex = Lexicon::default();
        for f in facts {
            let a = |k: &str| f.args.get(k).map(String::as_str).unwrap_or("");
            match f.rel.as_str() {
                "alias" => lex.aliases.push((phrase(a("from")), words(a("to")))),
                "rewrite" => lex.rewrites.push((phrase(a("from")), words(a("to")))),
                "filler" => {
                    let p = phrase(a("text"));
                    match a("at") {
                        "start" => lex.fill_start.push(p),
                        "end" => lex.fill_end.push(p),
                        _ => lex.fill_any.push(p),
                    }
                }
                "vocative" => lex.vocatives.push(phrase(a("text"))),
                "vocative_block" => {
                    let w = a("word").to_lowercase();
                    if a("side") == "before" {
                        lex.block_before.insert(w);
                    } else {
                        lex.block_after.insert(w);
                    }
                }
                "greeting" => {
                    lex.greetings.insert(a("word").to_lowercase());
                }
                "clause_joiner" => {
                    lex.joiners.insert(a("word").to_lowercase());
                }
                "fixed_phrase" => lex.fixed.push(phrase(a("text"))),
                "pronoun" => {
                    lex.pronouns
                        .insert(a("word").to_lowercase(), a("kind").to_string());
                }
                "hint" => lex.hints.push((phrase(a("word")), a("text").to_string())),
                _ => {}
            }
        }
        // Longest phrases first, so "can you please" wins over "can you".
        for table in [&mut lex.aliases, &mut lex.rewrites] {
            table.sort_by_key(|(from, _)| std::cmp::Reverse(from.len()));
        }
        for table in [&mut lex.fill_start, &mut lex.fill_end, &mut lex.fill_any, &mut lex.vocatives] {
            table.sort_by_key(|p| std::cmp::Reverse(p.len()));
        }
        lex
    }

    /// Canonical form of one clause.
    pub fn normalize(&self, text: &str) -> String {
        let mut toks = tokenize(text);
        toks = replace(&toks, &self.aliases);
        toks = replace(&toks, &self.rewrites);
        toks = split_possessive(toks);
        for _ in 0..6 {
            let before: Vec<String> = toks.iter().map(|t| t.low.clone()).collect();
            toks = self.strip(toks);
            toks = replace(&toks, &self.rewrites);
            if toks.iter().map(|t| t.low.clone()).collect::<Vec<_>>() == before {
                break;
            }
        }
        join(&toks)
    }

    fn strip(&self, mut toks: Vec<Tok>) -> Vec<Tok> {
        // Fillers at the start / end / anywhere (never strip everything).
        while let Some(n) = starts_with_any(&toks, 0, &self.fill_start) {
            if n >= toks.len() {
                break;
            }
            toks.drain(..n);
        }
        while let Some(n) = ends_with_any(&toks, &self.fill_end) {
            if n >= toks.len() {
                break;
            }
            toks.truncate(toks.len() - n);
        }
        let mut i = 0;
        while i < toks.len() {
            match starts_with_any(&toks, i, &self.fill_any) {
                Some(n) if n < toks.len() => {
                    toks.drain(i..i + n);
                }
                _ => i += 1,
            }
        }
        // "hey karin, what can you do" → "what can you do"; "hi karin" → "hi".
        if toks.len() >= 2 && self.greetings.contains(&toks[0].low) {
            if let Some(n) = starts_with_any(&toks, 1, &self.vocatives) {
                if 1 + n >= toks.len() {
                    toks.drain(1..1 + n);
                } else {
                    toks.drain(..1 + n);
                }
            }
        }
        // "karin what is my name" → "what is my name" (but keep "karin is smart").
        if let Some(n) = starts_with_any(&toks, 0, &self.vocatives) {
            let next = toks.get(n).map(|t| t.low.as_str());
            if n < toks.len() && !next.is_some_and(|w| self.block_after.contains(w)) {
                toks.drain(..n);
            }
        }
        // "what is my name karin" → "what is my name" (but keep "who is karin").
        if let Some(n) = ends_with_any(&toks, &self.vocatives) {
            if n < toks.len() {
                let prev = &toks[toks.len() - n - 1].low;
                if !self.block_before.contains(prev) {
                    toks.truncate(toks.len() - n);
                }
            }
        }
        toks
    }

    /// Split an utterance into clauses at sentence punctuation and joiner
    /// words ("and", "also", ...), never inside a fixed phrase.
    pub fn split_clauses(&self, text: &str) -> Vec<String> {
        let mut out = Vec::new();
        for sentence in split_sentences(text) {
            let toks = tokenize(&sentence);
            let mut cur: Vec<Tok> = Vec::new();
            let mut i = 0;
            while i < toks.len() {
                if let Some(n) = starts_with_any(&toks, i, &self.fixed) {
                    cur.extend(toks[i..i + n].iter().cloned());
                    i += n;
                    continue;
                }
                if self.joiners.contains(&toks[i].low) && !cur.is_empty() {
                    out.push(join(&cur));
                    cur.clear();
                } else {
                    cur.push(toks[i].clone());
                }
                i += 1;
            }
            if !cur.is_empty() {
                out.push(join(&cur));
            }
        }
        out.retain(|c| !c.trim().is_empty());
        out
    }

    /// Replace pronouns with the most recent chat `mention` of the right kind.
    pub fn resolve_pronouns(&self, text: &str, facts: &[Fact]) -> String {
        if self.pronouns.is_empty() {
            return text.to_string();
        }
        let toks: Vec<Tok> = tokenize(text)
            .into_iter()
            .map(|t| match self.pronouns.get(&t.low) {
                Some(kind) => facts
                    .iter()
                    .rev()
                    .find(|f| {
                        f.rel == "mention"
                            && f.scope == Scope::Chat
                            && f.args.get("kind").is_some_and(|k| k == kind)
                    })
                    .and_then(|f| f.args.get("entity"))
                    .map(|e| Tok::new(e))
                    .unwrap_or(t),
                None => t,
            })
            .collect();
        join(&toks)
    }

    pub fn hint_for(&self, text: &str) -> Option<String> {
        let toks = tokenize(text);
        (0..toks.len()).find_map(|i| {
            self.hints
                .iter()
                .find(|(p, _)| matches_at(&toks, i, p))
                .map(|(_, h)| h.clone())
        })
    }

    pub fn token_count(text: &str) -> usize {
        tokenize(text).len()
    }
}

fn tokenize(text: &str) -> Vec<Tok> {
    let text = text.replace(['’', '‘'], "'");
    text.split_whitespace()
        .map(|w| w.trim_matches(|c: char| ",.!?;:\"“”()".contains(c)))
        .filter(|w| !w.is_empty())
        .map(Tok::new)
        .collect()
}

fn join(toks: &[Tok]) -> String {
    toks.iter().map(|t| t.raw.as_str()).collect::<Vec<_>>().join(" ")
}

fn matches_at(toks: &[Tok], i: usize, p: &Phrase) -> bool {
    !p.is_empty()
        && i + p.len() <= toks.len()
        && p.iter().zip(&toks[i..]).all(|(w, t)| *w == t.low)
}

fn starts_with_any(toks: &[Tok], i: usize, table: &[Phrase]) -> Option<usize> {
    table.iter().find(|p| matches_at(toks, i, p)).map(Vec::len)
}

fn ends_with_any(toks: &[Tok], table: &[Phrase]) -> Option<usize> {
    table
        .iter()
        .find(|p| p.len() <= toks.len() && matches_at(toks, toks.len() - p.len(), p))
        .map(Vec::len)
}

fn replace(toks: &[Tok], table: &[(Phrase, Vec<String>)]) -> Vec<Tok> {
    let mut out = Vec::with_capacity(toks.len());
    let mut i = 0;
    while i < toks.len() {
        match table.iter().find(|(from, _)| matches_at(toks, i, from)) {
            Some((from, to)) => {
                out.extend(to.iter().map(|w| Tok::new(w)));
                i += from.len();
            }
            None => {
                out.push(toks[i].clone());
                i += 1;
            }
        }
    }
    out
}

/// "dad's" → "dad 's", so templates can say `my $rel 's name is $name`.
fn split_possessive(toks: Vec<Tok>) -> Vec<Tok> {
    let mut out = Vec::with_capacity(toks.len());
    for t in toks {
        if t.low.len() > 2 && t.low.ends_with("'s") {
            out.push(Tok::new(&t.raw[..t.raw.len() - 2]));
            out.push(Tok::new("'s"));
        } else {
            out.push(t);
        }
    }
    out
}

/// Split at . ! ? ; when followed by whitespace or the end (so "5.7" stays whole).
fn split_sentences(text: &str) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    let mut out = Vec::new();
    let mut cur = String::new();
    for (i, &c) in chars.iter().enumerate() {
        cur.push(c);
        let boundary = ".!?;".contains(c)
            && chars.get(i + 1).is_none_or(|n| n.is_whitespace());
        if boundary {
            out.push(std::mem::take(&mut cur));
        }
    }
    out.push(cur);
    out.into_iter()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn fact(rel: &str, args: &[(&str, &str)]) -> Fact {
        Fact {
            id: String::new(),
            rel: rel.into(),
            args: args
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect::<BTreeMap<_, _>>(),
            scope: Scope::LongTerm,
            provenance: String::new(),
            turn: 0,
        }
    }

    fn lex() -> Lexicon {
        Lexicon::from_facts(&[
            fact("alias", &[("from", "karen"), ("to", "karin")]),
            fact("rewrite", &[("from", "what's"), ("to", "what is")]),
            fact("filler", &[("text", "please"), ("at", "any")]),
            fact("filler", &[("text", "so"), ("at", "start")]),
            fact("filler", &[("text", "can you"), ("at", "start")]),
            fact("filler", &[("text", "right now"), ("at", "end")]),
            fact("vocative", &[("text", "karin")]),
            fact("vocative_block", &[("word", "is"), ("side", "after")]),
            fact("vocative_block", &[("word", "is"), ("side", "before")]),
            fact("greeting", &[("word", "hey")]),
            fact("clause_joiner", &[("word", "and")]),
            fact("fixed_phrase", &[("text", "rock and roll")]),
            fact("pronoun", &[("word", "he"), ("kind", "person")]),
        ])
    }

    #[test]
    fn strips_vocatives_fillers_and_keeps_case() {
        let l = lex();
        assert_eq!(l.normalize("Karen, so what's my name?"), "what is my name");
        assert_eq!(l.normalize("hey Karen can you list my notes please"), "list my notes");
        assert_eq!(l.normalize("hey Karen"), "hey");
        assert_eq!(l.normalize("Karen is smart"), "karin is smart");
        assert_eq!(l.normalize("who is Karen"), "who is karin");
        assert_eq!(l.normalize("weather in Vilnius right now"), "weather in Vilnius");
        assert_eq!(l.normalize("my dad's name is Vasily"), "my dad 's name is Vasily");
    }

    #[test]
    fn splits_clauses_but_not_fixed_phrases() {
        let l = lex();
        assert_eq!(
            l.split_clauses("Oscar is my friend and he lives in Venus. I like rock and roll"),
            vec!["Oscar is my friend", "he lives in Venus", "I like rock and roll"]
        );
    }

    #[test]
    fn resolves_pronouns_from_chat_mentions() {
        let l = lex();
        let mut m = fact("mention", &[("entity", "Oscar"), ("kind", "person")]);
        m.scope = Scope::Chat;
        assert_eq!(l.resolve_pronouns("he lives in Venus", &[m]), "Oscar lives in Venus");
    }
}
