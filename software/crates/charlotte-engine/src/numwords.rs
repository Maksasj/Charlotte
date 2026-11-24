//! Number wording induction: how people write the value of a number slot.
//!
//! Corpus turns labelled with a number fact ("amount": 1630) show where the number sits in the text.
//! The words around it ("$", "{n} dollars", "send {n}") become slot patterns. A pattern is proposed
//! only when it is precise (most numbers it finds are the labelled value, counting the unlabelled
//! mentions in the same conversations against it), seen often enough, and the same holds on
//! conversations held out from the search. The result is a candidate note for the slot, promoted by hand.

use crate::learn::Corpus;
use crate::understand::phrase_re;
use charlotte_core::*;
use charlotte_vault::{Note, Vault};
use regex::Regex;
use serde_json::{json, Map};
use std::collections::{BTreeSet, HashMap};

#[derive(Clone, Debug)]
pub struct NumOptions {
    pub min_support: usize,
    pub min_precision: f64,
    pub min_held: usize,
    /// A pattern must find the value in this many turns no accepted pattern finds.
    pub min_new: usize,
}
impl Default for NumOptions {
    fn default() -> Self {
        NumOptions { min_support: 5, min_precision: 0.85, min_held: 3, min_new: 3 }
    }
}

#[derive(Clone, Debug)]
pub struct NumPattern {
    pub text: String,
    pub support: usize,
    pub precision: f64,
    pub held_n: usize,
    pub held_precision: f64,
    pub example: String,
}
#[derive(Clone, Debug, Default)]
pub struct SlotInduction {
    pub slot: String,
    pub labelled: usize,
    pub skipped: Option<String>,
    pub patterns: Vec<NumPattern>,
}

#[derive(Default, Clone)]
struct Tally {
    n: usize,
    hits: usize,
    held_n: usize,
    held_hits: usize,
    hit_turns: BTreeSet<usize>,
    example: String,
}

fn words_of(s: &str, rev: bool) -> Vec<String> {
    let mut w: Vec<String> = s.split_whitespace().map(|x| x.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase()).collect();
    if rev {
        w.reverse();
    }
    w
}
fn alpha(w: &str) -> bool {
    !w.is_empty() && w.chars().all(|c| c.is_alphabetic() || c == '\'')
}

/// The patterns that would find the number at `start..end` of `text`.
fn contexts(text: &str, start: usize, end: usize) -> Vec<String> {
    let mut out = Vec::new();
    let before = &text[..start];
    let after = &text[end..];
    let symbol_before: String = before.chars().last().filter(|c| !c.is_alphanumeric() && !c.is_whitespace() && !".,!?'\"".contains(*c)).map(String::from).unwrap_or_default();
    let symbol_after: String = after.chars().next().filter(|c| *c == '%').map(String::from).unwrap_or_default();
    let adjacent_before = before.ends_with(char::is_whitespace) || !symbol_before.is_empty() || before.is_empty();
    let adjacent_after = after.starts_with(char::is_whitespace) || !symbol_after.is_empty() || after.is_empty();
    let b: Vec<String> = if before.ends_with(char::is_alphanumeric) || !adjacent_before { vec![] } else { words_of(before, true).into_iter().take_while(|w| alpha(w)).take(2).collect() };
    let a: Vec<String> = if after.starts_with(char::is_alphanumeric) || !adjacent_after { vec![] } else { words_of(after, false).into_iter().take_while(|w| alpha(w)).take(2).collect() };
    let core = format!("{symbol_before}{{n}}{symbol_after}");
    if !symbol_before.is_empty() || !symbol_after.is_empty() {
        out.push(core.clone());
    }
    for i in 1..=a.len() {
        out.push(format!("{core} {}", a[..i].join(" ")));
    }
    for i in 1..=b.len() {
        let pre: Vec<&str> = b[..i].iter().rev().map(|s| s.as_str()).collect();
        out.push(format!("{} {core}", pre.join(" ")));
    }
    if let (Some(x), Some(y)) = (b.first(), a.first()) {
        out.push(format!("{x} {core} {y}"));
    }
    out
}

pub fn induce(vault: &Vault, corpus: &Corpus, opts: &NumOptions, only: Option<&str>) -> Vec<SlotInduction> {
    let numtok = Regex::new(r"\d{1,3}(?:,\d{3})+(?:\.\d+)?|\d+(?:\.\d+)?").unwrap();
    let parse = |s: &str| s.replace(',', "").parse::<f64>().ok();
    let mut out = Vec::new();
    for slot in vault.slots().into_iter().filter(|s| s.ty == SlotType::Number && only.map_or(true, |o| o == s.id)) {
        let mut res = SlotInduction { slot: slot.id.clone(), ..Default::default() };
        let mut tallies: HashMap<String, Tally> = HashMap::new();
        let mut turn_id = 0;
        for (ci, conv) in corpus.convs.iter().enumerate() {
            let label = |t: &crate::learn::CorpusTurn| match t.facts.as_ref().and_then(|f| f.get(&slot.id)) {
                Some(Value::Num(n)) => Some(*n),
                _ => None,
            };
            if !conv.turns.iter().any(|t| t.role == "user" && label(t).is_some()) {
                continue;
            }
            let held = ci % 5 == 0;
            for t in conv.turns.iter().filter(|t| t.role == "user") {
                turn_id += 1;
                let want = label(t);
                res.labelled += want.is_some() as usize;
                for m in numtok.find_iter(&t.text) {
                    let Some(v) = parse(m.as_str()) else { continue };
                    let hit = want.map_or(false, |w| (w - v).abs() < 1e-9);
                    let mut seen = BTreeSet::new();
                    for c in contexts(&t.text, m.start(), m.end()) {
                        if !seen.insert(c.clone()) {
                            continue;
                        }
                        let e = tallies.entry(c).or_default();
                        if held {
                            e.held_n += 1;
                            e.held_hits += hit as usize;
                        } else {
                            e.n += 1;
                            e.hits += hit as usize;
                            if hit {
                                e.hit_turns.insert(turn_id);
                                if e.example.is_empty() {
                                    e.example = t.text.clone();
                                }
                            }
                        }
                    }
                }
            }
        }
        if res.labelled < opts.min_support {
            res.skipped = Some(format!("only {} labelled turns", res.labelled));
            out.push(res);
            continue;
        }
        let known: Vec<String> = slot.patterns.iter().map(|p| p.text.trim().to_lowercase()).collect();
        let mut cands: Vec<(String, Tally)> = tallies
            .into_iter()
            .filter(|(text, t)| {
                let p = t.hits as f64 / t.n.max(1) as f64;
                t.n >= opts.min_support
                    && p >= opts.min_precision
                    && t.held_n >= opts.min_held
                    && t.held_hits as f64 / t.held_n as f64 >= opts.min_precision
                    && !known.iter().any(|k| text.to_lowercase().contains(k.as_str()))
                    && phrase_re(text).is_some()
            })
            .collect();
        // Broad wordings first; later ones count only for the turns they add.
        cands.sort_by(|a, b| b.1.hit_turns.len().cmp(&a.1.hit_turns.len()).then(a.0.cmp(&b.0)));
        let mut covered: BTreeSet<usize> = BTreeSet::new();
        for (text, t) in cands {
            if t.hit_turns.difference(&covered).count() < opts.min_new {
                continue;
            }
            covered.extend(t.hit_turns.iter().copied());
            res.patterns.push(NumPattern {
                text,
                support: t.n,
                precision: t.hits as f64 / t.n as f64,
                held_n: t.held_n,
                held_precision: t.held_hits as f64 / t.held_n as f64,
                example: t.example.clone(),
            });
        }
        out.push(res);
    }
    out
}

pub fn candidate_note(r: &SlotInduction, source: &str) -> Option<Note> {
    if r.patterns.is_empty() {
        return None;
    }
    let id = format!("patterns-{}-numbers", r.slot);
    let mut front = Map::new();
    front.insert("kind".into(), "phrases".into());
    front.insert("slot".into(), format!("[[{}]]", r.slot).into());
    front.insert("as".into(), "patterns".into());
    let round = |x: f64| (x * 1000.0).round() / 1000.0;
    let list: Vec<_> = r.patterns.iter().map(|p| json!({ "text": p.text, "support": p.support, "precision": round(p.precision), "held_precision": round(p.held_precision), "learned": true })).collect();
    front.insert("phrases".into(), list.into());
    front.insert("induced_from".into(), source.into());
    front.insert("status".into(), "candidate".into());
    let mut body = format!("Ways of writing the {} number, from turns labelled with it in {source}.\n\n", r.slot);
    for p in &r.patterns {
        body.push_str(&format!("- `{}`: {} numbers, {:.0}% were the labelled value ({:.0}% of {} held out). E.g. \"{}\"\n", p.text, p.support, p.precision * 100.0, p.held_precision * 100.0, p.held_n, p.example));
    }
    Some(Note { id: id.clone(), path: format!("candidates/{id}.md"), front, body })
}
