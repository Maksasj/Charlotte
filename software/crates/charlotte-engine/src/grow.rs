//! Seed and grow: new wordings for an intent, found in chats nobody labelled.
//!
//! The vault's phrases for an intent are the seed. The replies they drew in the chats say what the
//! intent *does*: after "thank you" people answer "you're welcome". Messages that drew the same kind
//! of reply, and share a wording the seed lacks, are candidates for the intent. Rounds repeat with the
//! accepted wordings added to the seed.
//!
//! Nothing here is trusted. A wording is proposed only when
//! - the seed's replies are coherent (the intent is an act, not a topic: most replies share a few
//!   distinctive words), otherwise the intent is skipped,
//! - enough messages carry it and most of them drew such a reply, far above the base rate,
//! - the same holds on conversations held out from the search.
//!
//! The result is a `phrases` candidate note per intent, to be promoted by hand. `vet` runs the same
//! test on candidates that came from elsewhere (labels of another dataset), as a cross-source check.

use crate::learn::{Corpus, CorpusTurn};
use crate::understand::phrase_re;
use charlotte_core::*;
use charlotte_vault::{Note, Vault};
use regex::Regex;
use serde_json::{json, Map};
use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct Exchange {
    pub conv: usize,
    pub message: String,
    pub reply: String,
}

/// Every message with the message that followed it, whoever said it.
pub fn exchanges(corpus: &Corpus) -> Vec<Exchange> {
    let mut out = Vec::new();
    for (ci, c) in corpus.convs.iter().enumerate() {
        let said: Vec<&CorpusTurn> = c.turns.iter().filter(|t| !t.text.trim().is_empty()).collect();
        for w in said.windows(2) {
            out.push(Exchange { conv: ci, message: w[0].text.split_whitespace().collect::<Vec<_>>().join(" "), reply: w[1].text.split_whitespace().collect::<Vec<_>>().join(" ") });
        }
    }
    out
}

#[derive(Clone, Debug)]
pub struct GrowOptions {
    pub rounds: usize,
    /// Messages with a wording, on the search side, for it to be proposed.
    pub min_support: usize,
    /// Share of those messages that drew a reply of the intent's kind, as a share of what the seed's
    /// own messages draw.
    pub min_relative: f64,
    /// ... and how many times the base rate that is.
    pub min_lift: f64,
    /// ... and at least this many of them, so that a handful of lucky replies is not enough.
    pub min_hits: usize,
    /// The same share on held-out conversations, over at least `min_held` messages.
    pub min_held: usize,
    /// The seed's own replies must share a distinctive wording this often for the intent to be grown.
    pub min_coherence: f64,
    /// Longer messages are not read for wordings.
    pub max_words: usize,
    /// One conversation in this many is held out.
    pub held_every: usize,
}
impl Default for GrowOptions {
    fn default() -> Self {
        GrowOptions { rounds: 4, min_support: 8, min_hits: 8, min_relative: 0.6, min_lift: 5.0, min_held: 3, min_coherence: 0.15, max_words: 12, held_every: 5 }
    }
}

#[derive(Clone, Debug)]
pub struct Grown {
    pub phrase: String,
    pub support: usize,
    pub precision: f64,
    pub held_precision: f64,
    pub held_n: usize,
    pub examples: Vec<String>,
}
#[derive(Clone, Debug, Default)]
pub struct IntentGrowth {
    pub intent: String,
    pub seed_messages: usize,
    /// Distinctive reply wordings of the seed.
    pub signature: Vec<String>,
    pub coherence: f64,
    pub base_rate: f64,
    pub skipped: Option<String>,
    pub grown: Vec<Grown>,
}

fn words(text: &str) -> Vec<String> {
    text.split(|c: char| !(c.is_alphanumeric() || c == '\'')).map(|w| w.trim_matches('\'').to_lowercase()).filter(|w| !w.is_empty()).collect()
}
fn grams_of(toks: &[String], max: usize, content: &dyn Fn(&str) -> bool) -> HashSet<String> {
    let mut out = HashSet::new();
    for n in 1..=max {
        for w in toks.windows(n) {
            if w.iter().any(|t| t.len() < 2 || t.chars().any(|c| c.is_ascii_digit())) || !w.iter().any(|t| content(t)) {
                continue;
            }
            out.insert(w.join(" "));
        }
    }
    out
}
fn has_gram(text_words: &[String], gram: &str) -> bool {
    let g: Vec<&str> = gram.split(' ').collect();
    text_words.windows(g.len()).any(|w| w.iter().zip(&g).all(|(a, b)| a == b))
}

struct Prepared<'a> {
    ex: &'a Exchange,
    msg: Vec<String>,
    held: bool,
    short: bool,
}

/// Grow one intent. `seed` are its current phrases; `others` are the phrases of the other social intents,
/// whose wordings say nothing about this one when they turn up in replies ("bye" ends many chats).
pub fn grow_intent(id: &str, seed: &[String], others: &[String], all: &[Exchange], stop: &HashSet<String>, opts: &GrowOptions) -> IntentGrowth {
    let mut res = IntentGrowth { intent: id.to_string(), ..Default::default() };
    let content = |t: &str| !stop.contains(t);
    let any = |t: &str| !t.is_empty();
    let prep: Vec<Prepared> = all
        .iter()
        .map(|e| {
            let msg = words(&e.message);
            Prepared { ex: e, held: e.conv % opts.held_every == 0, short: msg.len() <= opts.max_words, msg }
        })
        .collect();
    let reply_words: Vec<Vec<String>> = prep.iter().map(|p| words(&p.ex.reply)).collect();
    let mut phrases: Vec<(Regex, String)> = seed.iter().filter_map(|p| phrase_re(p).map(|r| (r, p.clone()))).collect();
    if phrases.is_empty() {
        res.skipped = Some("no seed phrases".into());
        return res;
    }
    let mut matched: Vec<bool> = prep.iter().map(|p| p.short && phrases.iter().any(|(r, _)| r.is_match(&p.ex.message))).collect();
    // The kind of reply the seed draws, from the search side only.
    let train: Vec<usize> = (0..prep.len()).filter(|&i| !prep[i].held).collect();
    let in_seed: Vec<usize> = train.iter().copied().filter(|&i| matched[i]).collect();
    res.seed_messages = in_seed.len();
    if in_seed.len() < opts.min_support.max(20) {
        res.skipped = Some(format!("only {} seed messages", in_seed.len()));
        return res;
    }
    let mut df: HashMap<String, usize> = HashMap::new();
    for &i in &train {
        for g in grams_of(&reply_words[i], 2, &any) {
            *df.entry(g).or_insert(0) += 1;
        }
    }
    let mut in_df: HashMap<String, usize> = HashMap::new();
    for &i in &in_seed {
        for g in grams_of(&reply_words[i], 2, &any) {
            *in_df.entry(g).or_insert(0) += 1;
        }
    }
    let n_train = train.len() as f64;
    let n_seed = in_seed.len() as f64;
    let foreign = |g: &str| {
        let gw = words(g);
        others.iter().any(|o| {
            let ow = words(o);
            !ow.is_empty() && (has_gram(&gw, &ow.join(" ")) || has_gram(&ow, g))
        })
    };
    let mut sig: Vec<(String, usize)> = in_df
        .into_iter()
        .filter(|(g, _)| !foreign(g))
        .filter(|(g, n)| *n as f64 >= (0.03 * n_seed).max(5.0) && (*n as f64 / n_seed) / (df[g] as f64 / n_train) >= 8.0)
        .collect();
    sig.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    sig.truncate(12);
    res.signature = sig.iter().map(|(g, _)| g.clone()).collect();
    let hit = |i: usize| res.signature.iter().any(|g| has_gram(&reply_words[i], g));
    res.coherence = in_seed.iter().filter(|&&i| hit(i)).count() as f64 / n_seed;
    res.base_rate = train.iter().filter(|&&i| hit(i)).count() as f64 / n_train;
    if res.signature.is_empty() || res.coherence < opts.min_coherence || res.coherence / res.base_rate.max(1e-4) < opts.min_lift {
        res.skipped = Some(format!("replies are not coherent ({:.0}% share a wording, base rate {:.1}%): a topic, not an act", res.coherence * 100.0, res.base_rate * 100.0));
        return res;
    }
    let hits: Vec<bool> = (0..prep.len()).map(hit).collect();
    let rate = res.base_rate.max(1e-4);
    let bar = opts.min_relative * res.coherence;

    for _ in 0..opts.rounds {
        // Wordings of the messages the seed does not cover yet: how many, and how many drew such a reply.
        let mut tot: HashMap<String, (usize, usize, usize, usize)> = HashMap::new();
        for (i, p) in prep.iter().enumerate() {
            if matched[i] || !p.short {
                continue;
            }
            for g in grams_of(&p.msg, 3, &content) {
                let e = tot.entry(g).or_insert((0, 0, 0, 0));
                if p.held {
                    e.2 += 1;
                    e.3 += hits[i] as usize;
                } else {
                    e.0 += 1;
                    e.1 += hits[i] as usize;
                }
            }
        }
        let mut cands: Vec<(String, (usize, usize, usize, usize))> = tot
            .into_iter()
            .filter(|(_, (n, h, hn, hh))| {
                let p = *h as f64 / (*n).max(1) as f64;
                *n >= opts.min_support
                    && *h >= opts.min_hits
                    && wilson_lower(*h, *n) >= 0.5 * bar
                    && p / rate >= opts.min_lift
                    && *hn >= opts.min_held
                    && *hh as f64 / (*hn).max(1) as f64 >= bar
            })
            .collect();
        cands.sort_by(|a, b| b.1 .0.cmp(&a.1 .0).then(a.0.cmp(&b.0)));
        let mut taken: Vec<String> = Vec::new();
        for (g, (n, h, hn, hh)) in cands {
            if taken.len() >= 5 || res.grown.len() >= 30 {
                break;
            }
            // A wording that contains an accepted one, or is part of it, adds nothing.
            if taken.iter().chain(seed.iter()).any(|t| g.contains(t.as_str()) || t.contains(g.as_str())) || res.grown.iter().any(|x| g.contains(&x.phrase) || x.phrase.contains(&g)) {
                continue;
            }
            let examples: Vec<String> = prep.iter().enumerate().filter(|(i, p)| !matched[*i] && p.short && has_gram(&p.msg, &g)).take(3).map(|(_, p)| p.ex.message.clone()).collect();
            res.grown.push(Grown { phrase: g.clone(), support: n, precision: h as f64 / n as f64, held_precision: hh as f64 / hn as f64, held_n: hn, examples });
            taken.push(g);
        }
        if taken.is_empty() {
            break;
        }
        for g in &taken {
            if let Some(r) = phrase_re(g) {
                phrases.push((r, g.clone()));
            }
        }
        for (i, p) in prep.iter().enumerate() {
            if !matched[i] && p.short && taken.iter().any(|g| has_gram(&p.msg, g)) {
                matched[i] = true;
            }
        }
    }
    res
}

/// Grow every social intent of the vault.
pub fn grow(vault: &Vault, all: &[Exchange], opts: &GrowOptions, only: Option<&str>) -> Vec<IntentGrowth> {
    let stop: HashSet<String> = vault.lexicon().stopwords.iter().cloned().collect();
    let social: Vec<&Intent> = vault.intents().into_iter().filter(|i| i.social).collect();
    social
        .iter()
        .filter(|i| only.map_or(true, |o| o == i.id))
        .map(|i| {
            let seed: Vec<String> = i.phrases.iter().map(|p| p.text().to_string()).collect();
            grow_intent(&i.id, &seed, &others_of(&social, &i.id), all, &stop, opts)
        })
        .collect()
}

fn others_of(social: &[&Intent], id: &str) -> Vec<String> {
    social.iter().filter(|i| i.id != id).flat_map(|i| i.phrases.iter().map(|p| p.text().to_string())).collect()
}

/// The candidate note for what was grown: `phrases` for the intent, promoted by hand.
pub fn candidate_note(g: &IntentGrowth, source: &str) -> Option<Note> {
    if g.grown.is_empty() {
        return None;
    }
    let id = format!("phrases-{}-grown", g.intent);
    let mut front = Map::new();
    front.insert("kind".into(), "phrases".into());
    front.insert("intent".into(), format!("[[{}]]", g.intent).into());
    let phrases: Vec<_> = g
        .grown
        .iter()
        .map(|x| json!({ "text": x.phrase, "weight": 0.5, "support": x.support, "precision": round(x.precision), "held_precision": round(x.held_precision), "learned": true }))
        .collect();
    front.insert("phrases".into(), phrases.into());
    front.insert("grown_from".into(), source.into());
    front.insert("coherence".into(), round(g.coherence).into());
    front.insert("status".into(), "candidate".into());
    let mut body = format!("Wordings for {} that drew the same replies as the seed ({}) in {source}.\n\n", g.intent, g.signature.join(", "));
    for x in &g.grown {
        body.push_str(&format!("- \"{}\": {} messages, {:.0}% drew such a reply ({:.0}% of {} held out). E.g. {}\n", x.phrase, x.support, x.precision * 100.0, x.held_precision * 100.0, x.held_n, x.examples.iter().map(|e| format!("\"{e}\"")).collect::<Vec<_>>().join("; ")));
    }
    Some(Note { id: id.clone(), path: format!("candidates/{id}.md"), front, body })
}
/// Lower end of the 95% interval for a share of `h` in `n`.
fn wilson_lower(h: usize, n: usize) -> f64 {
    let (h, n, z) = (h as f64, n as f64, 1.96f64);
    let p = h / n;
    (p + z * z / (2.0 * n) - z * ((p * (1.0 - p) + z * z / (4.0 * n)) / n).sqrt()) / (1.0 + z * z / n)
}
fn round(x: f64) -> f64 {
    (x * 1000.0).round() / 1000.0
}

/// The same test for wordings that came from somewhere else: how often does a message with the wording
/// draw the reply the intent's seed draws in these chats, against the base rate. One entry per phrase.
#[derive(Clone, Debug)]
pub struct Vetted {
    pub intent: String,
    pub phrase: String,
    pub messages: usize,
    pub precision: f64,
    pub lift: f64,
}
pub fn vet(vault: &Vault, all: &[Exchange], opts: &GrowOptions, intent: &str, phrases: &[String]) -> Result<Vec<Vetted>, String> {
    let i = vault.intent(intent).ok_or_else(|| format!("no intent '{intent}'"))?;
    let stop: HashSet<String> = vault.lexicon().stopwords.iter().cloned().collect();
    let seed: Vec<String> = i.phrases.iter().map(|p| p.text().to_string()).collect();
    // The signature comes from the seed alone: no rounds.
    let social: Vec<&Intent> = vault.intents().into_iter().filter(|i| i.social).collect();
    let probe = grow_intent(intent, &seed, &others_of(&social, intent), all, &stop, &GrowOptions { rounds: 0, ..opts.clone() });
    if let Some(why) = probe.skipped {
        return Err(format!("{intent}: {why}"));
    }
    let reply_words: Vec<Vec<String>> = all.iter().map(|e| words(&e.reply)).collect();
    let hit = |i: usize| probe.signature.iter().any(|g| has_gram(&reply_words[i], g));
    let msgs: Vec<Vec<String>> = all.iter().map(|e| words(&e.message)).collect();
    let covered: Vec<bool> = all.iter().map(|e| seed.iter().filter_map(|p| phrase_re(p)).any(|r| r.is_match(&e.message))).collect();
    let rate = probe.base_rate.max(1e-4);
    let mut out = Vec::new();
    for ph in phrases {
        let Some(re) = phrase_re(ph) else { continue };
        let ids: Vec<usize> = (0..all.len()).filter(|&k| !covered[k] && msgs[k].len() <= opts.max_words && re.is_match(&all[k].message)).collect();
        let p = if ids.is_empty() { 0.0 } else { ids.iter().filter(|&&k| hit(k)).count() as f64 / ids.len() as f64 };
        out.push(Vetted { intent: intent.to_string(), phrase: ph.clone(), messages: ids.len(), precision: p, lift: p / rate });
    }
    Ok(out)
}

/// Run `vet` over every phrase candidate in the vault that targets an intent, and write the result into the note.
pub fn vet_candidates(vault: &mut Vault, all: &[Exchange], opts: &GrowOptions, source: &str) -> Vec<(String, Result<Vec<Vetted>, String>)> {
    let cands: Vec<Note> = vault.notes.values().filter(|n| n.kind() == "phrases" && n.status() == "candidate" && n.front.contains_key("intent")).cloned().collect();
    let mut out = Vec::new();
    for mut n in cands {
        let intent = n.front["intent"].as_str().unwrap_or("").trim_start_matches("[[").trim_end_matches("]]").to_string();
        let phrases: Vec<String> = n.front.get("phrases").and_then(|p| p.as_array()).map(|a| a.iter().filter_map(|x| x.get("text").and_then(|t| t.as_str()).or_else(|| x.as_str()).map(str::to_string)).collect()).unwrap_or_default();
        let r = vet(vault, all, opts, &intent, &phrases);
        if let Ok(v) = &r {
            let by: BTreeMap<&str, &Vetted> = v.iter().map(|x| (x.phrase.as_str(), x)).collect();
            if let Some(arr) = n.front.get_mut("phrases").and_then(|p| p.as_array_mut()) {
                for x in arr.iter_mut() {
                    let t = x.get("text").and_then(|t| t.as_str()).unwrap_or("").to_string();
                    if let (Some(v), Some(o)) = (by.get(t.as_str()), x.as_object_mut()) {
                        o.insert(format!("cross_precision[{source}]"), json!(round(v.precision)));
                        o.insert(format!("cross_messages[{source}]"), json!(v.messages));
                    }
                }
            }
            let _ = vault.put(n.clone());
        }
        out.push((n.id.clone(), r));
    }
    out
}
