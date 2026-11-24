//! Exchange-type induction: kinds of exchange that chats show and the vault does not know yet.
//!
//! Only counts and alignment, nothing trained. For chats nobody labelled:
//! 1. *Group* exchanges by the wording of the reply (distinctive reply words that many replies share);
//!    groups whose exchanges overlap are one type.
//! 2. *Coherence*: an act ("greet", "thank") has messages that share a few wordings, a topic has messages
//!    that merely drew the same words back. A group is kept only when a handful of message wordings
//!    ("skeletons") explain most of it.
//! 3. *Skeletons*: a message wording is accepted when it is common, drew this kind of reply much more
//!    often than other messages do, and does so on conversations held out from the search too.
//! 4. *Varying span*: what follows a skeleton in its messages ("my name is ...") is checked for being
//!    echoed in the reply. An echo is reported as evidence for a slot, never turned into one here.
//! 5. *Reply*: the reply the type most often draws, or its most central reply when none repeats.
//!
//! A type already covered by an intent in the vault is reported as such (growing it is `grow`'s job).
//! The rest becomes a pair of candidate notes (an intent and a move) with all counts; a person names and
//! promotes them.

use crate::grow::Exchange;
use crate::understand::phrase_re;
use charlotte_core::*;
use charlotte_vault::{Note, Vault};
use serde_json::{json, Map};
use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct TypeOptions {
    /// Exchanges a group needs on the search side.
    pub min_group: usize,
    /// A reply wording shared by more than this share of replies says nothing.
    pub max_share: f64,
    pub max_keys: usize,
    pub max_words: usize,
    pub min_hits: usize,
    pub min_precision: f64,
    pub min_lift: f64,
    pub min_held: usize,
    /// Share of the group the accepted skeletons must explain.
    pub min_coverage: f64,
    /// Groups that overlap this much are one type.
    pub merge_overlap: f64,
    /// Share of a group's replies that the most common replies must make up.
    pub min_reply_share: f64,
    /// Already an intent when its phrases match this share of the group's messages.
    pub known_share: f64,
    pub held_every: usize,
    /// Only messages the user side sent (corpora whose roles mean something).
    pub user_only: bool,
}
impl Default for TypeOptions {
    fn default() -> Self {
        TypeOptions { min_group: 20, max_share: 0.03, max_keys: 400, max_words: 8, min_hits: 8, min_precision: 0.25, min_lift: 10.0, min_held: 3, min_coverage: 0.3, merge_overlap: 0.5, min_reply_share: 0.15, known_share: 0.5, held_every: 5, user_only: false }
    }
}

#[derive(Clone, Debug)]
pub struct Skeleton {
    pub phrase: String,
    pub support: usize,
    pub precision: f64,
    pub held_n: usize,
    pub held_precision: f64,
    pub lift: f64,
}
#[derive(Clone, Debug)]
pub struct Echo {
    pub after: String,
    pub values: usize,
    pub share: f64,
    pub examples: Vec<String>,
}
#[derive(Clone, Debug, Default)]
pub struct ExchangeType {
    pub id: String,
    pub reply_words: Vec<String>,
    pub exchanges: usize,
    pub coverage: f64,
    pub reply_share: f64,
    pub skeletons: Vec<Skeleton>,
    pub echo: Option<Echo>,
    pub reply: String,
    pub reply_count: usize,
    pub examples: Vec<(String, String)>,
    /// The vault intent that already covers this type.
    pub known: Option<String>,
    pub skipped: Option<String>,
}

fn words(text: &str) -> Vec<String> {
    text.split(|c: char| !(c.is_alphanumeric() || c == '\'')).map(|w| w.trim_matches('\'').to_lowercase()).filter(|w| !w.is_empty()).collect()
}
fn grams(toks: &[String], max: usize, stop: &HashSet<String>) -> HashSet<String> {
    let mut out = HashSet::new();
    for n in 1..=max {
        for w in toks.windows(n) {
            if w.iter().any(|t| t.len() < 2 || t.chars().any(|c| c.is_ascii_digit())) || w.iter().all(|t| stop.contains(t)) {
                continue;
            }
            // A wording does not begin or end on a function word, unless it is all there is.
            if n > 1 && (stop.contains(&w[n - 1]) && stop.contains(&w[0])) {
                continue;
            }
            out.insert(w.join(" "));
        }
    }
    out
}
fn has_gram(toks: &[String], gram: &str) -> bool {
    let g: Vec<&str> = gram.split(' ').collect();
    toks.windows(g.len()).any(|w| w.iter().zip(&g).all(|(a, b)| a == b))
}
fn slug(s: &str) -> String {
    s.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).take(3).collect::<Vec<_>>().join("-")
}
fn wilson_lower(h: usize, n: usize) -> f64 {
    let (h, n, z) = (h as f64, n as f64, 1.96f64);
    let p = h / n;
    (p + z * z / (2.0 * n) - z * ((p * (1.0 - p) + z * z / (4.0 * n)) / n).sqrt()) / (1.0 + z * z / n)
}

struct Row<'a> {
    ex: &'a Exchange,
    msg: Vec<String>,
    held: bool,
}

pub fn induce(vault: &Vault, all: &[Exchange], opts: &TypeOptions) -> Vec<ExchangeType> {
    let stop: HashSet<String> = vault.lexicon().stopwords.iter().cloned().collect();
    let rows: Vec<Row> = all.iter().map(|e| Row { ex: e, msg: words(&e.message), held: e.conv % opts.held_every == 0 }).filter(|r| !r.msg.is_empty() && r.msg.len() <= opts.max_words && (r.ex.user || !opts.user_only)).collect();
    let reply_words: Vec<Vec<String>> = rows.iter().map(|r| words(&r.ex.reply)).collect();
    let train: Vec<usize> = (0..rows.len()).filter(|&i| !rows[i].held).collect();
    if train.len() < opts.min_group {
        return vec![];
    }
    let n_train = train.len() as f64;

    // 1. Reply wordings, and which exchanges carry each.
    let reply_grams: Vec<HashSet<String>> = reply_words.iter().map(|w| grams(&w[..w.len().min(14)], 2, &stop)).collect();
    let mut df: HashMap<&str, usize> = HashMap::new();
    for &i in &train {
        for g in &reply_grams[i] {
            *df.entry(g.as_str()).or_insert(0) += 1;
        }
    }
    let mut keys: Vec<(&str, usize)> = df.into_iter().filter(|(_, c)| *c >= opts.min_group && (*c as f64) / n_train <= opts.max_share).collect();
    keys.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    keys.truncate(opts.max_keys);

    // Message wordings over the whole search side and over the held-out side.
    let msg_grams: Vec<HashSet<String>> = rows.iter().map(|r| grams(&r.msg, 3, &stop)).collect();
    let (mut total, mut held_total): (HashMap<&str, usize>, HashMap<&str, usize>) = (HashMap::new(), HashMap::new());
    for (i, g) in msg_grams.iter().enumerate() {
        let t = if rows[i].held { &mut held_total } else { &mut total };
        for x in g {
            *t.entry(x.as_str()).or_insert(0) += 1;
        }
    }
    let n_held = (0..rows.len()).filter(|&i| rows[i].held).count().max(1) as f64;

    // 2-3. Per reply wording: skeletons that explain the group.
    struct Group {
        key: String,
        members: BTreeSet<usize>,
        skeletons: Vec<Skeleton>,
        coverage: f64,
    }
    let mut groups: Vec<Group> = Vec::new();
    for (key, _) in keys {
        let members: Vec<usize> = train.iter().copied().filter(|&i| reply_grams[i].contains(key)).collect();
        let held_members: Vec<usize> = (0..rows.len()).filter(|&i| rows[i].held && reply_grams[i].contains(key)).collect();
        let rate = members.len() as f64 / n_train;
        let mut inn: HashMap<&str, usize> = HashMap::new();
        for &i in &members {
            for g in &msg_grams[i] {
                *inn.entry(g.as_str()).or_insert(0) += 1;
            }
        }
        let mut held_in: HashMap<&str, usize> = HashMap::new();
        for &i in &held_members {
            for g in &msg_grams[i] {
                *held_in.entry(g.as_str()).or_insert(0) += 1;
            }
        }
        let mut cands: Vec<Skeleton> = Vec::new();
        for (g, &h) in &inn {
            let n = total[g];
            let p = h as f64 / n as f64;
            let (hn, hh) = (held_total.get(g).copied().unwrap_or(0), held_in.get(g).copied().unwrap_or(0));
            let held_rate = held_members.len() as f64 / n_held;
            if h >= opts.min_hits
                && wilson_lower(h, n) >= opts.min_precision * 0.7
                && p >= opts.min_precision
                && p / rate >= opts.min_lift
                && hn >= opts.min_held
                && hh as f64 / hn as f64 >= opts.min_precision
                && (hh as f64 / hn as f64) / held_rate.max(1e-4) >= opts.min_lift
            {
                cands.push(Skeleton { phrase: g.to_string(), support: n, precision: p, held_n: hn, held_precision: hh as f64 / hn as f64, lift: p / rate });
            }
        }
        cands.sort_by(|a, b| b.support.cmp(&a.support).then(a.phrase.cmp(&b.phrase)));
        let mut skeletons: Vec<Skeleton> = Vec::new();
        for c in cands {
            if skeletons.iter().any(|s| c.phrase.contains(&s.phrase) || s.phrase.contains(&c.phrase)) {
                continue;
            }
            skeletons.push(c);
            if skeletons.len() >= 8 {
                break;
            }
        }
        if skeletons.is_empty() {
            continue;
        }
        let explained = members.iter().filter(|&&i| skeletons.iter().any(|s| has_gram(&rows[i].msg, &s.phrase))).count();
        let coverage = explained as f64 / members.len() as f64;
        if coverage < opts.min_coverage {
            continue;
        }
        groups.push(Group { key: key.to_string(), members: members.into_iter().collect(), skeletons, coverage });
    }

    // Groups whose exchanges overlap are one type: the larger one absorbs the others.
    groups.sort_by(|a, b| b.members.len().cmp(&a.members.len()).then(a.key.cmp(&b.key)));
    let mut merged: Vec<(Vec<String>, BTreeSet<usize>, Vec<Skeleton>, f64)> = Vec::new();
    for g in groups {
        let hit = merged.iter_mut().find(|m| {
            let inter = m.1.intersection(&g.members).count() as f64;
            inter / g.members.len().min(m.1.len()) as f64 >= opts.merge_overlap
        });
        match hit {
            Some(m) => {
                m.0.push(g.key);
                m.1.extend(g.members);
                for s in g.skeletons {
                    if !m.2.iter().any(|x| x.phrase.contains(&s.phrase) || s.phrase.contains(&x.phrase)) {
                        m.2.push(s);
                    }
                }
            }
            None => merged.push((vec![g.key], g.members, g.skeletons, g.coverage)),
        }
    }

    // 4-5. Echo, reply, and whether the vault knows the type already.
    let known: Vec<(String, Vec<regex::Regex>)> = vault.intents().into_iter().map(|i| (i.id.clone(), i.phrases.iter().filter_map(|p| phrase_re(p.text())).collect())).collect();
    let mut out = Vec::new();
    for (keys, members, skeletons, _) in merged {
        let mut t = ExchangeType { reply_words: keys.clone(), exchanges: members.len(), skeletons, ..Default::default() };
        let idx: Vec<usize> = members.iter().copied().collect();
        t.coverage = idx.iter().filter(|&&i| t.skeletons.iter().any(|s| has_gram(&rows[i].msg, &s.phrase))).count() as f64 / idx.len() as f64;
        t.id = slug(&t.skeletons[0].phrase);

        // Replies: the most common ones, as a share of the group.
        let mut by_reply: HashMap<String, usize> = HashMap::new();
        for &i in &idx {
            *by_reply.entry(rows[i].ex.reply.clone()).or_insert(0) += 1;
        }
        let mut ranked: Vec<(&String, &usize)> = by_reply.iter().collect();
        ranked.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
        let top3: usize = ranked.iter().take(3).map(|(_, c)| **c).sum();
        t.reply_share = top3 as f64 / idx.len() as f64;
        let (best, count) = ranked.first().map(|(r, c)| ((*r).clone(), **c)).unwrap_or_default();
        if count >= 3 {
            t.reply = best;
            t.reply_count = count;
        } else {
            // Nothing repeats: the reply sharing most words with the others.
            let sample: Vec<&String> = ranked.iter().take(150).map(|(r, _)| *r).collect();
            let ws: Vec<HashSet<String>> = sample.iter().map(|r| words(r).into_iter().collect()).collect();
            let score = |a: &HashSet<String>| ws.iter().map(|b| a.intersection(b).count()).sum::<usize>() as f64 / (a.len().max(1) as f64).sqrt();
            if let Some((k, _)) = ws.iter().enumerate().map(|(k, a)| (k, score(a))).max_by(|a, b| a.1.partial_cmp(&b.1).unwrap().then(b.0.cmp(&a.0))) {
                t.reply = sample[k].clone();
            }
        }
        t.examples = idx.iter().take(3).map(|&i| (rows[i].ex.message.clone(), rows[i].ex.reply.clone())).collect();

        // Varying span: what follows a skeleton, and whether the reply says it back.
        for s in &t.skeletons {
            let mut spans: Vec<(String, bool)> = Vec::new();
            for &i in &idx {
                let m = &rows[i].msg;
                let g: Vec<&str> = s.phrase.split(' ').collect();
                let Some(p) = m.windows(g.len()).position(|w| w.iter().zip(&g).all(|(a, b)| a == b)) else { continue };
                let tail: Vec<&String> = m[p + g.len()..].iter().collect();
                if tail.is_empty() || tail.len() > 3 || tail.iter().any(|w| g.contains(&w.as_str())) {
                    continue;
                }
                let echoed = tail.iter().all(|w| reply_words[i].contains(*w));
                spans.push((tail.iter().map(|w| w.as_str()).collect::<Vec<_>>().join(" "), echoed));
            }
            let distinct: BTreeSet<&String> = spans.iter().map(|(s, _)| s).collect();
            let echoed = spans.iter().filter(|(_, e)| *e).count();
            if spans.len() >= 8 && distinct.len() >= 5 && echoed as f64 / spans.len() as f64 >= 0.6 && t.echo.as_ref().map_or(true, |e| spans.len() > e.values) {
                t.echo = Some(Echo { after: s.phrase.clone(), values: distinct.len(), share: echoed as f64 / spans.len() as f64, examples: spans.iter().take(3).map(|(s, _)| s.clone()).collect() });
            }
        }

        // Known?
        let covered = |re: &Vec<regex::Regex>| idx.iter().filter(|&&i| re.iter().any(|r| r.is_match(&rows[i].ex.message))).count() as f64 / idx.len() as f64;
        t.known = known.iter().filter(|(_, re)| !re.is_empty()).map(|(id, re)| (id, covered(re))).filter(|(_, c)| *c >= opts.known_share).max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).map(|(id, _)| id.clone());
        if t.reply_share < opts.min_reply_share && t.known.is_none() {
            t.skipped = Some(format!("replies are too varied ({:.0}% in the three most common): a topic, not an act", t.reply_share * 100.0));
        }
        if t.reply.is_empty() {
            t.skipped = Some("no reply".into());
        }
        out.push(t);
    }
    out.sort_by(|a, b| b.exchanges.cmp(&a.exchanges).then(a.id.cmp(&b.id)));
    // Two types may land on the same name.
    let mut seen: HashMap<String, usize> = HashMap::new();
    for t in &mut out {
        let n = seen.entry(t.id.clone()).or_insert(0);
        *n += 1;
        if *n > 1 {
            t.id = format!("{}-{}", t.id, n);
        }
    }
    out
}

fn round(x: f64) -> f64 {
    (x * 1000.0).round() / 1000.0
}

/// The candidate notes for a new type: an intent and a move. Both promote under the type's name; a person
/// should rename them (`promotes_to`) and give the intent an `act` if it is one.
pub fn candidate_notes(t: &ExchangeType, source: &str) -> Vec<Note> {
    if t.known.is_some() || t.skipped.is_some() {
        return vec![];
    }
    let id = format!("type-{}", t.id);
    let target = t.id.clone();
    let mut intent = Map::new();
    intent.insert("kind".into(), "intent".into());
    intent.insert("social".into(), true.into());
    let phrases: Vec<_> = t.skeletons.iter().map(|s| json!({ "text": s.phrase, "weight": 0.5, "support": s.support, "precision": round(s.precision), "held_precision": round(s.held_precision), "learned": true })).collect();
    intent.insert("phrases".into(), phrases.into());
    intent.insert("promotes_to".into(), format!("[[{target}]]").into());
    intent.insert("induced_from".into(), source.into());
    intent.insert("coverage".into(), round(t.coverage).into());
    intent.insert("status".into(), "candidate".into());
    let mut body = format!(
        "A kind of exchange found in {source}: {} messages drew replies with {}. The wordings below explain {:.0}% of them.\n\n",
        t.exchanges,
        t.reply_words.iter().map(|w| format!("\"{w}\"")).collect::<Vec<_>>().join(", "),
        t.coverage * 100.0
    );
    for s in &t.skeletons {
        body.push_str(&format!("- \"{}\": {} messages, {:.0}% drew such a reply, lift {:.0} ({:.0}% of {} held out)\n", s.phrase, s.support, s.precision * 100.0, s.lift, s.held_precision * 100.0, s.held_n));
    }
    if let Some(e) = &t.echo {
        body.push_str(&format!("\nWhat follows \"{}\" varies ({} values, e.g. {}) and {:.0}% of the replies say it back. That looks like a slot; none was made.\n", e.after, e.values, e.examples.join(", "), e.share * 100.0));
    }
    body.push_str("\nName it, give it an `act` if it is one, check the reply, then promote.\n");
    let intent_note = Note { id: id.clone(), path: format!("candidates/{id}.md"), front: intent, body };

    let mut mv = Map::new();
    mv.insert("kind".into(), "move".into());
    mv.insert("intent".into(), format!("[[{target}]]").into());
    mv.insert("examples".into(), t.examples.iter().map(|(m, _)| json!(m)).collect::<Vec<_>>().into());
    mv.insert("trust".into(), 0.5.into());
    mv.insert("promotes_to".into(), format!("[[{target}-reply]]").into());
    mv.insert("status".into(), "candidate".into());
    mv.insert("reply_support".into(), json!({ "most_common": t.reply_count, "of": t.exchanges, "top3_share": round(t.reply_share) }));
    let mid = format!("{id}-reply");
    let move_note = Note { id: mid.clone(), path: format!("candidates/{mid}.md"), front: mv, body: t.reply.clone() };
    vec![intent_note, move_note]
}
