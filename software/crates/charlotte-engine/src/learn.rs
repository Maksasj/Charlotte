//! Learning from a corpus of conversations: rule lists for patterns, phrases for the lexicon.
//! Everything learned is checked on conversations held out from learning before it is trusted.

use crate::decide::{class_to_act, run_rules};
use crate::engine::{Engine, Layers};
use crate::view::{features, selectors, view};
use charlotte_core::*;
use charlotte_vault::{parse_chats, render_chats, Note, Vault};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value as Json};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::Path;

// ---------- corpus ----------

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CorpusTurn {
    pub role: String,
    #[serde(default)]
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facts: Option<Facts>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signal: Option<Signal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub act: Option<Op>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arg: Option<String>,
    /// Several acts in one agent turn, in order; a silent lookup comes before the act that is spoken.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acts: Option<Vec<Act>>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CorpusConv {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
    pub turns: Vec<CorpusTurn>,
}
pub struct Corpus {
    pub source: String,
    pub convs: Vec<CorpusConv>,
    /// Lines that could not be read.
    pub skipped: usize,
}
impl Corpus {
    pub fn open(path: impl AsRef<Path>) -> Result<Corpus, String> {
        let p = path.as_ref();
        let text = std::fs::read_to_string(p).map_err(|e| format!("{}: {e}", p.display()))?;
        Ok(Corpus::parse(&p.to_string_lossy(), &text))
    }
    pub fn parse(source: &str, jsonl: &str) -> Corpus {
        let (mut convs, mut skipped) = (Vec::new(), 0);
        for line in jsonl.lines().map(str::trim).filter(|l| !l.is_empty()) {
            match serde_json::from_str::<CorpusConv>(line) {
                Ok(c) if !c.turns.is_empty() => convs.push(c),
                _ => skipped += 1,
            }
        }
        Corpus { source: source.to_string(), convs, skipped }
    }
}

// ---------- report ----------

#[derive(Clone, Debug, Default, Serialize)]
pub struct Counts {
    pub read: usize,
    pub skipped: usize,
    pub train: usize,
    pub held_out: usize,
}
#[derive(Clone, Debug, Default, Serialize)]
pub struct Labels {
    pub act: f64,
    pub intent: f64,
    pub facts: f64,
}
#[derive(Clone, Debug, Default, Serialize)]
pub struct BeforeAfter {
    pub before: f64,
    pub after: f64,
}
#[derive(Clone, Debug, Serialize)]
pub struct CandidateReport {
    pub id: String,
    pub kind: String,
    pub support: usize,
    pub validated: f64,
    pub promoted: bool,
    pub summary: String,
}
#[derive(Clone, Debug, Serialize)]
pub struct Mistake {
    pub what: String,
    pub count: usize,
}
#[derive(Clone, Debug, Default, Serialize)]
pub struct Report {
    pub conversations: Counts,
    /// Share of agent turns with an act, of conversations with an intent, of user turns with facts.
    pub labels: Labels,
    /// Share of held-out conversations whose every agent act is reproduced.
    pub held_out: BeforeAfter,
    /// Share of held-out labelled user turns whose intent is read from the text.
    pub understanding: BeforeAfter,
    pub candidates: Vec<CandidateReport>,
    pub mistakes: Vec<Mistake>,
    pub seconds: f64,
}
impl std::fmt::Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let pct = |x: f64| format!("{:.0}%", x * 100.0);
        let c = &self.conversations;
        writeln!(f, "conversations   {} read, {} skipped, {} to learn from, {} held out", c.read, c.skipped, c.train, c.held_out)?;
        writeln!(f, "labels          act {}, intent {}, facts {}", pct(self.labels.act), pct(self.labels.intent), pct(self.labels.facts))?;
        writeln!(f, "held-out        {} -> {} of conversations reproduced exactly", pct(self.held_out.before), pct(self.held_out.after))?;
        writeln!(f, "understanding   {} -> {} of intents read from the text", pct(self.understanding.before), pct(self.understanding.after))?;
        for x in &self.candidates {
            writeln!(f, "  {:<9} {:<28} support {:<4} validated {:<5} {}  {}", x.kind, x.id, x.support, pct(x.validated), if x.promoted { "promoted " } else { "candidate" }, x.summary)?;
        }
        for m in &self.mistakes {
            writeln!(f, "  mistake   {} ({})", m.what, m.count)?;
        }
        write!(f, "took            {:.2}s", self.seconds)
    }
}

#[derive(Clone, Debug)]
pub struct LearnOptions {
    /// Trust what passes validation without waiting for review.
    pub auto_promote: bool,
    /// Share of its held-out conversations a pattern must reproduce to be promoted.
    pub threshold: f64,
    pub seed: u64,
}
impl Default for LearnOptions {
    fn default() -> Self {
        LearnOptions { auto_promote: true, threshold: 0.9, seed: 1 }
    }
}

// ---------- conversations to examples ----------

pub struct Example {
    pub conv: usize,
    pub kind: ReqKind,
    pub intent: String,
    pub feats: BTreeMap<String, bool>,
    pub sels: BTreeMap<String, String>,
    pub op: Op,
    pub arg: Option<String>,
    pub class: String,
}

/// Walk a conversation with the tracker and note, before each agent act, the state it was chosen in.
pub fn replay(ci: usize, conv: &CorpusConv, mem: &dyn Memory, l: &Layers) -> Vec<Example> {
    let (mut state, mut cur, mut out) = (State::default(), None, Vec::new());
    for t in &conv.turns {
        if t.role == "user" {
            let reading = if t.intent.is_some() || t.facts.is_some() || t.signal.is_some() {
                Reading { requests: vec![Request { clause: t.text.clone(), intent: t.intent.clone(), facts: t.facts.clone().unwrap_or_default(), signal: t.signal, refer: None, conflicts: Default::default(), matched: vec![] }] }
            } else {
                let ctx = ReadContext { asked: state.frames.iter().rev().filter_map(|f| f.last_asked.clone()).collect() };
                l.understand.read(&t.text, &ctx)
            };
            if let Some(f) = l.track.user(&mut state, &reading, mem).iter().find_map(|t| t.frame) {
                cur = Some(f);
            }
        } else {
            let acts = t.acts.clone().unwrap_or_else(|| t.act.map(|op| vec![Act { op, arg: t.arg.clone() }]).unwrap_or_default());
            for a in acts {
                let Some(fi) = cur else { continue };
                let v = view(&state.frames[fi], mem);
                if matches!(v.kind, ReqKind::Gated | ReqKind::Procedure | ReqKind::Lookup) && Op::LEARNABLE.contains(&a.op) {
                    let f = &state.frames[fi];
                    out.push(Example { conv: ci, kind: v.kind, intent: f.intent.clone(), feats: features(f, &v), sels: selectors(f, &v), op: a.op, arg: a.arg.clone(), class: String::new() });
                }
                l.track.agent(&mut state.frames[fi], &a, mem);
                // A call moves to its subframe; a subframe's closing act moves back to the parent. The
                // examples of the subframe are the callee's own, whoever called it; the conversation stays the unit
                // they are judged in.
                match a.op {
                    Op::Call => cur = Some(l.track.call(&mut state, fi, a.arg.as_deref().unwrap_or_default(), mem)),
                    _ => {
                        if let Some(p) = l.track.ret(&mut state, fi, &a, mem) {
                            cur = Some(p);
                        }
                    }
                }
            }
        }
    }
    out
}

/// Name each concrete argument by a selector: per operation, the selector that explains the most
/// examples, then the next one for whatever is left.
fn label(exs: &mut [Example]) {
    let mut by_op: BTreeMap<Op, Vec<usize>> = BTreeMap::new();
    for (i, e) in exs.iter_mut().enumerate() {
        if e.arg.is_none() {
            e.class = e.op.name().to_string();
        } else {
            by_op.entry(e.op).or_default().push(i);
        }
    }
    for (op, mut rem) in by_op {
        while !rem.is_empty() {
            let mut count: BTreeMap<&str, usize> = BTreeMap::new();
            for &i in &rem {
                for (k, v) in &exs[i].sels {
                    if Some(v) == exs[i].arg.as_ref() {
                        *count.entry(k.as_str()).or_insert(0) += 1;
                    }
                }
            }
            let Some(best) = count.iter().max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(a.0))).map(|(k, _)| k.to_string()) else {
                for &i in &rem {
                    exs[i].class = format!("{}(?)", op.name());
                }
                break;
            };
            rem.retain(|&i| {
                if exs[i].sels.get(&best) == exs[i].arg.as_ref() {
                    exs[i].class = format!("{}({})", op.name(), best);
                    false
                } else {
                    true
                }
            });
        }
    }
}

// ---------- the pattern learner ----------

#[derive(Clone)]
struct Beam {
    lits: Vec<(usize, bool)>,
    sub: Vec<usize>,
    cls: usize,
    q: f64,
}

/// An ordered rule list: repeatedly the most reliable rule (beam search, Laplace accuracy,
/// at most four conditions) for the examples no earlier rule covers.
pub fn learn_rules(exs: &[&Example]) -> (Vec<Rule>, String) {
    let names: Vec<&String> = exs.iter().flat_map(|e| e.feats.keys()).collect::<BTreeSet<_>>().into_iter().collect();
    let classes: Vec<&str> = exs.iter().map(|e| e.class.as_str()).collect::<BTreeSet<_>>().into_iter().collect();
    let k = classes.len();
    let x: Vec<Vec<bool>> = exs.iter().map(|e| names.iter().map(|n| e.feats.get(*n).copied().unwrap_or(false)).collect()).collect();
    let y: Vec<usize> = exs.iter().map(|e| classes.iter().position(|c| *c == e.class).unwrap()).collect();
    let score = |sub: &[usize]| {
        let mut c = vec![0usize; k];
        sub.iter().for_each(|&e| c[y[e]] += 1);
        let (cls, m) = c.iter().enumerate().fold((0, 0), |b, (i, &n)| if n > b.1 { (i, n) } else { b });
        (cls, (m as f64 + 1.0) / (sub.len() as f64 + k as f64))
    };
    let all: Vec<usize> = (0..exs.len()).collect();
    let (mut rem, mut rules) = (all.clone(), Vec::new());
    while !rem.is_empty() {
        let (cls, q) = score(&rem);
        let mut beam = vec![Beam { lits: vec![], sub: rem.clone(), cls, q }];
        let mut best: Option<Beam> = None;
        for _ in 0..4 {
            let mut cand: BTreeMap<Vec<(usize, bool)>, Beam> = BTreeMap::new();
            for b in &beam {
                for f in 0..names.len() {
                    if b.lits.iter().any(|l| l.0 == f) {
                        continue;
                    }
                    for v in [true, false] {
                        let sub: Vec<usize> = b.sub.iter().copied().filter(|&e| x[e][f] == v).collect();
                        if sub.len() < 2 || sub.len() == b.sub.len() {
                            continue;
                        }
                        let (cls, q) = score(&sub);
                        if q <= b.q {
                            continue;
                        }
                        let mut lits = b.lits.clone();
                        lits.push((f, v));
                        let mut key = lits.clone();
                        key.sort();
                        cand.entry(key).or_insert(Beam { lits, sub, cls, q });
                    }
                }
            }
            if cand.is_empty() {
                break;
            }
            let mut v: Vec<Beam> = cand.into_values().collect();
            v.sort_by(|a, b| b.q.partial_cmp(&a.q).unwrap().then(b.sub.len().cmp(&a.sub.len())));
            v.truncate(5);
            if best.as_ref().map_or(true, |x| v[0].q > x.q) {
                best = Some(v[0].clone());
            }
            beam = v;
        }
        let Some(b) = best else { break };
        rules.push(Rule { when: b.lits.iter().map(|(f, v)| if *v { names[*f].clone() } else { format!("not {}", names[*f]) }).collect(), act: classes[b.cls].to_string() });
        let covered: HashSet<usize> = b.sub.into_iter().collect();
        rem.retain(|e| !covered.contains(e));
    }
    let otherwise = classes[score(if rem.is_empty() { &all } else { &rem }).0].to_string();
    (rules, otherwise)
}

type Rules = (Vec<Rule>, String);

/// Per conversation, whether every agent act is reproduced; and the mistakes made.
fn evaluate(models: &BTreeMap<ReqKind, Rules>, exs: &[Example], mistakes: &mut BTreeMap<String, usize>) -> BTreeMap<usize, bool> {
    let mut ok: BTreeMap<usize, bool> = BTreeMap::new();
    for e in exs {
        let got = models.get(&e.kind).and_then(|(rules, otherwise)| class_to_act(run_rules(rules, otherwise, &e.feats).1, &e.sels));
        let right = got.as_ref().map_or(false, |a| a.op == e.op && a.arg == e.arg);
        if !right {
            let what = match &got {
                None => format!("teacher {}, engine has no answer", e.op.name()),
                Some(a) if a.op == e.op => format!("teacher {}, engine {} with the wrong argument", e.op.name(), a.op.name()),
                Some(a) => format!("teacher {}, engine {}", e.op.name(), a.op.name()),
            };
            *mistakes.entry(what).or_insert(0) += 1;
        }
        let entry = ok.entry(e.conv).or_insert(true);
        *entry = *entry && right;
    }
    ok
}
fn share(ok: &BTreeMap<usize, bool>) -> f64 {
    if ok.is_empty() {
        0.0
    } else {
        ok.values().filter(|b| **b).count() as f64 / ok.len() as f64
    }
}

fn pattern_name(kind: ReqKind) -> &'static str {
    match kind {
        ReqKind::Gated => "check-a-rule",
        ReqKind::Procedure => "walk-through-steps",
        _ => "look-up-a-record",
    }
}
fn pattern_note(kind: ReqKind, rules: &Rules, support: usize, validated: f64, intents: &BTreeSet<String>, source: &str, promoted: bool) -> Note {
    let name = pattern_name(kind);
    let mut front = Map::new();
    front.insert("kind".into(), "pattern".into());
    front.insert("applies_to".into(), kind.name().into());
    front.insert("rules".into(), json!(rules.0));
    front.insert("otherwise".into(), rules.1.clone().into());
    front.insert("support".into(), support.into());
    front.insert("validated".into(), json!((validated * 1000.0).round() / 1000.0));
    front.insert("seen_with".into(), json!(intents.iter().map(|i| format!("[[{i}]]")).collect::<Vec<_>>()));
    front.insert("learned_from".into(), source.into());
    front.insert("trust".into(), json!(0.8));
    let mut body = format!("Learned from {support} conversations. First rule that fits wins.\n\n");
    for (i, r) in rules.0.iter().enumerate() {
        body.push_str(&format!("{}. when {} → `{}`\n", i + 1, r.when.join(" and "), r.act));
    }
    body.push_str(&format!("{}. otherwise → `{}`", rules.0.len() + 1, rules.1));
    if promoted {
        front.insert("status".into(), "trusted".into());
        Note { id: name.into(), path: format!("patterns/{name}.md"), front, body }
    } else {
        front.insert("status".into(), "candidate".into());
        front.insert("promotes_to".into(), name.into());
        Note { id: format!("{name}-candidate"), path: format!("candidates/{name}-candidate.md"), front, body }
    }
}

// ---------- the lexicon learner ----------

struct Sample {
    text: String,
    grams: BTreeSet<String>,
    intent: Option<String>,
    facts: Facts,
}
fn grams(text: &str, stop: &HashSet<&str>) -> BTreeSet<String> {
    let lower = text.to_lowercase();
    // Words only; a token with a digit in it (a number, a reference) breaks the run.
    let toks: Vec<&str> = lower.split(|c: char| !(c.is_alphanumeric() || c == '\'' || c == '-')).filter(|t| !t.is_empty()).collect();
    let word = |t: &str| t.chars().all(|c| c.is_alphabetic() || c == '\'');
    let mut out = BTreeSet::new();
    for n in 1..=3 {
        for w in toks.windows(n) {
            if !w.iter().all(|t| word(t)) || w.iter().all(|t| stop.contains(t)) || w.iter().any(|t| t.len() < 2) || (n == 1 && w[0].len() < 3) {
                continue;
            }
            out.insert(w.join(" "));
        }
    }
    out
}
struct Proposal {
    /// The intent or slot note the phrases are for, and for a slot the value they mean.
    target: String,
    value: Option<String>,
    phrases: Vec<(String, usize)>,
}

/// Wordings the reader misses: for each labelled intent or fact value it did not read, the n-grams
/// that occur almost only with that label, most frequent first.
fn propose(samples: &[Sample], vault: &Vault, l: &Layers) -> Vec<Proposal> {
    let mut df: HashMap<&str, usize> = HashMap::new();
    for s in samples {
        for g in &s.grams {
            *df.entry(g.as_str()).or_insert(0) += 1;
        }
    }
    let readings: Vec<Reading> = samples.iter().map(|s| l.understand.read(&s.text, &ReadContext::default())).collect();
    let pick = |labelled: Vec<usize>, mut missed: Vec<usize>| -> Vec<(String, usize)> {
        let mut inside: HashMap<&str, usize> = HashMap::new();
        for &i in &labelled {
            for g in &samples[i].grams {
                *inside.entry(g.as_str()).or_insert(0) += 1;
            }
        }
        let mut out = Vec::new();
        while out.len() < 5 {
            let mut support: BTreeMap<&str, usize> = BTreeMap::new();
            for &i in &missed {
                for g in &samples[i].grams {
                    *support.entry(g.as_str()).or_insert(0) += 1;
                }
            }
            let best = support
                .into_iter()
                .filter(|(g, n)| *n >= 5 && inside[g] as f64 / df[g] as f64 >= 0.9)
                .max_by(|a, b| a.1.cmp(&b.1).then(a.0.split(' ').count().cmp(&b.0.split(' ').count())).then(b.0.cmp(a.0)));
            let Some((g, n)) = best else { break };
            out.push((g.to_string(), n));
            missed.retain(|&i| !samples[i].grams.contains(g));
        }
        out
    };
    let mut out = Vec::new();
    for intent in vault.intents() {
        let labelled: Vec<usize> = (0..samples.len()).filter(|&i| samples[i].intent.as_deref() == Some(intent.id.as_str())).collect();
        let missed = labelled.iter().copied().filter(|&i| readings[i].requests.first().and_then(|r| r.intent.as_deref()) != Some(intent.id.as_str())).collect();
        let phrases = pick(labelled, missed);
        if !phrases.is_empty() {
            out.push(Proposal { target: intent.id.clone(), value: None, phrases });
        }
    }
    for slot in vault.slots().into_iter().filter(|s| matches!(s.ty, SlotType::Enum | SlotType::Bool)) {
        let mut keys: BTreeSet<String> = slot.values.keys().cloned().collect();
        if slot.ty == SlotType::Bool {
            keys.extend(["true".to_string(), "false".to_string()]);
        }
        for key in keys {
            let want = slot.value_of(&key);
            let labelled: Vec<usize> = (0..samples.len()).filter(|&i| samples[i].facts.get(&slot.id).map_or(false, |v| v.same(&want))).collect();
            let missed = labelled.iter().copied().filter(|&i| !readings[i].requests.iter().any(|r| r.facts.get(&slot.id).map_or(false, |v| v.same(&want)))).collect();
            let phrases = pick(labelled, missed);
            if !phrases.is_empty() {
                out.push(Proposal { target: slot.id.clone(), value: Some(key), phrases });
            }
        }
    }
    out
}

/// Add phrases to an intent note, or to one value of a slot note. Phrases already there are left alone.
pub fn add_phrases(note: &mut Note, value: Option<&str>, phrases: Vec<Json>) {
    let list = match value {
        None => note.front.entry("phrases").or_insert_with(|| json!([])),
        Some(v) => {
            let values = note.front.entry("values").or_insert_with(|| json!({}));
            match values.as_object_mut() {
                Some(m) => m.entry(v.to_string()).or_insert_with(|| json!([])),
                None => return,
            }
        }
    };
    let Some(arr) = list.as_array_mut() else { return };
    let text = |p: &Json| p.as_str().map(str::to_string).or_else(|| p.get("text").and_then(|t| t.as_str()).map(str::to_string));
    for p in phrases {
        if !arr.iter().any(|x| text(x) == text(&p)) {
            arr.push(p);
        }
    }
}
/// Add number wordings to a slot note's `patterns`. Wordings already there are left alone.
pub fn add_patterns(note: &mut Note, items: Vec<Json>) {
    let Some(arr) = note.front.entry("patterns").or_insert_with(|| json!([])).as_array_mut() else { return };
    for p in items {
        let text = p.get("text").and_then(|t| t.as_str()).or_else(|| p.as_str()).unwrap_or_default().to_string();
        if !text.is_empty() && !arr.iter().any(|x| x.get("text").and_then(|t| t.as_str()) == Some(text.as_str())) {
            arr.push(json!({ "text": text }));
        }
    }
}
/// Add split connectors to the lexicon note. Connectors already there are left alone.
pub fn add_connectors(note: &mut Note, items: Vec<Json>) {
    let Some(arr) = note.front.entry("connectors").or_insert_with(|| json!([])).as_array_mut() else { return };
    for p in items {
        let text = p.get("text").and_then(|t| t.as_str()).or_else(|| p.as_str()).unwrap_or_default().to_string();
        if !text.is_empty() && !arr.iter().any(|x| x.as_str() == Some(text.as_str())) {
            arr.push(json!(text));
        }
    }
}
fn phrase_json(p: &[(String, usize)]) -> Vec<Json> {
    p.iter().map(|(t, n)| json!({ "text": t, "weight": 0.5, "support": n, "learned": true })).collect()
}
fn intent_accuracy(samples: &[Sample], l: &Layers) -> f64 {
    let labelled: Vec<&Sample> = samples.iter().filter(|s| s.intent.is_some()).collect();
    if labelled.is_empty() {
        return 0.0;
    }
    let right = labelled.iter().filter(|s| l.understand.read(&s.text, &ReadContext::default()).requests.first().and_then(|r| r.intent.clone()) == s.intent).count();
    right as f64 / labelled.len() as f64
}

// ---------- the pipeline ----------

pub struct Outcome {
    pub report: Report,
    /// Notes to write: promoted items in place, the rest under `candidates/`.
    pub notes: Vec<Note>,
}

fn shuffled(n: usize, seed: u64) -> Vec<usize> {
    let mut idx: Vec<usize> = (0..n).collect();
    let mut s = seed.wrapping_mul(0x9E3779B97F4A7C15) | 1;
    for i in (1..n).rev() {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        idx.swap(i, (s % (i as u64 + 1)) as usize);
    }
    idx
}

pub fn run(vault: &Vault, rebuild: &dyn Fn(&Vault) -> Layers, corpus: &Corpus, opts: &LearnOptions, stage: &dyn Fn(&str)) -> Outcome {
    let started = std::time::Instant::now();
    let l = rebuild(vault);
    let mem: &dyn Memory = vault;
    stage("ingest");
    let n = corpus.convs.len();
    // Hold out a tenth, but never fewer than 16 conversations (or half of a small corpus).
    let held = ((n as f64 * 0.1).ceil() as usize).max(16.min(n / 2));
    let order = shuffled(n, opts.seed);
    let held_set: HashSet<usize> = order[..held].iter().copied().collect();
    let mut report = Report { conversations: Counts { read: n, skipped: corpus.skipped, train: n - held, held_out: held }, ..Default::default() };
    let turns = |role: &str| corpus.convs.iter().flat_map(|c| c.turns.iter()).filter(|t| t.role == role).count().max(1) as f64;
    report.labels = Labels {
        act: corpus.convs.iter().flat_map(|c| c.turns.iter()).filter(|t| t.role != "user" && (t.act.is_some() || t.acts.is_some())).count() as f64 / turns("agent"),
        intent: corpus.convs.iter().filter(|c| c.turns.iter().any(|t| t.intent.is_some())).count() as f64 / n.max(1) as f64,
        facts: corpus.convs.iter().flat_map(|c| c.turns.iter()).filter(|t| t.role == "user" && t.facts.is_some()).count() as f64 / turns("user"),
    };
    let (mut train, mut test) = (Vec::new(), Vec::new());
    let stop: HashSet<&str> = mem.lexicon().stopwords.iter().map(String::as_str).collect();
    let (mut train_s, mut test_s) = (Vec::new(), Vec::new());
    // A conversation with no labels at all is a plain chat: nothing to induce from, only replies to remember.
    let plain = |c: &CorpusConv| !c.turns.iter().any(|t| t.act.is_some() || t.acts.is_some() || t.intent.is_some() || t.facts.is_some() || t.signal.is_some());
    for (ci, c) in corpus.convs.iter().enumerate() {
        if plain(c) {
            continue;
        }
        let is_held = held_set.contains(&ci);
        (if is_held { &mut test } else { &mut train }).extend(replay(ci, c, mem, &l));
        for t in c.turns.iter().filter(|t| t.role == "user" && !t.text.is_empty() && (t.intent.is_some() || t.facts.is_some())) {
            // `none` marks a message labelled as no known intent: a negative example for every wording.
            let intent = t.intent.clone().filter(|i| i != "none");
            let s = Sample { text: t.text.clone(), grams: grams(&t.text, &stop), intent, facts: t.facts.clone().unwrap_or_default() };
            (if is_held { &mut test_s } else { &mut train_s }).push(s);
        }
    }

    stage("induce");
    label(&mut train);
    let current: BTreeMap<ReqKind, Rules> =
        [ReqKind::Gated, ReqKind::Procedure, ReqKind::Lookup].into_iter().filter_map(|k| mem.pattern(k).map(|p| (k, (p.rules.clone(), p.otherwise.clone())))).collect();
    let mut learned: BTreeMap<ReqKind, Rules> = BTreeMap::new();
    for kind in [ReqKind::Gated, ReqKind::Procedure, ReqKind::Lookup] {
        let exs: Vec<&Example> = train.iter().filter(|e| e.kind == kind).collect();
        if exs.len() >= 10 {
            learned.insert(kind, learn_rules(&exs));
        }
    }
    let proposals = propose(&train_s, vault, &l);

    stage("validate");
    let mut after_models = current.clone();
    after_models.extend(learned.clone());
    let (mut ignore, mut mistakes) = (BTreeMap::new(), BTreeMap::new());
    let before = evaluate(&current, &test, &mut ignore);
    let after = evaluate(&after_models, &test, &mut mistakes);
    report.held_out = BeforeAfter { before: share(&before), after: share(&after) };
    let mut notes = Vec::new();
    for (kind, rules) in &learned {
        let convs = |exs: &[Example]| exs.iter().filter(|e| e.kind == *kind).map(|e| e.conv).collect::<BTreeSet<_>>();
        let only = |ok: &BTreeMap<usize, bool>| ok.iter().filter(|(c, _)| convs(&test).contains(c)).map(|(c, b)| (*c, *b)).collect::<BTreeMap<_, _>>();
        let (was, validated, support) = (share(&only(&before)), share(&only(&after)), convs(&train).len());
        let same = current.get(kind).map_or(false, |c| serde_json::to_string(c).ok() == serde_json::to_string(rules).ok());
        let promoted = !same && opts.auto_promote && validated >= opts.threshold && validated >= was;
        let summary = if same { "same as the trusted pattern".to_string() } else { format!("{} rules for {} requests (held-out {:.0}% before)", rules.0.len() + 1, kind.name(), was * 100.0) };
        if !same {
            let intents = train.iter().filter(|e| e.kind == *kind).map(|e| e.intent.clone()).collect();
            let note = pattern_note(*kind, rules, support, validated, &intents, &corpus.source, promoted);
            report.candidates.push(CandidateReport { id: note.id.clone(), kind: "pattern".into(), support, validated, promoted, summary });
            notes.push(note);
        } else {
            report.candidates.push(CandidateReport { id: pattern_name(*kind).into(), kind: "pattern".into(), support, validated, promoted: false, summary });
        }
    }
    // Phrases are judged together: they are kept if reading the held-out text does not get worse.
    report.understanding.before = intent_accuracy(&test_s, &l);
    report.understanding.after = report.understanding.before;
    if !proposals.is_empty() {
        let mut trial = vault.clone();
        let mut changed: BTreeMap<String, Note> = BTreeMap::new();
        for p in &proposals {
            if let Some(mut note) = changed.get(&p.target).or_else(|| trial.notes.get(&p.target)).cloned() {
                add_phrases(&mut note, p.value.as_deref(), phrase_json(&p.phrases));
                changed.insert(p.target.clone(), note);
            }
        }
        for note in changed.values() {
            let _ = trial.put_mem(note.clone());
        }
        report.understanding.after = intent_accuracy(&test_s, &rebuild(&trial));
        let promoted = opts.auto_promote && report.understanding.after >= report.understanding.before;
        for p in &proposals {
            let id = format!("phrases-{}{}", p.target, p.value.as_ref().map(|v| format!("-{v}")).unwrap_or_default());
            let words = p.phrases.iter().map(|(t, _)| format!("\"{t}\"")).collect::<Vec<_>>().join(", ");
            let for_what = p.value.as_ref().map(|v| format!("{} = {v}", p.target)).unwrap_or_else(|| p.target.clone());
            report.candidates.push(CandidateReport { id: id.clone(), kind: "phrases".into(), support: p.phrases.iter().map(|x| x.1).sum(), validated: report.understanding.after, promoted, summary: format!("{for_what}: {words}") });
            if !promoted {
                let mut front = Map::new();
                front.insert("kind".into(), "phrases".into());
                front.insert(if vault.intent(&p.target).is_some() { "intent" } else { "slot" }.into(), format!("[[{}]]", p.target).into());
                if let Some(v) = &p.value {
                    front.insert("value".into(), v.clone().into());
                }
                front.insert("phrases".into(), Json::Array(phrase_json(&p.phrases)));
                front.insert("status".into(), "candidate".into());
                notes.push(Note { id: id.clone(), path: format!("candidates/{id}.md"), front, body: format!("Wordings for {for_what} found in {}.", corpus.source) });
            }
        }
        if promoted {
            notes.extend(changed.into_values());
        }
    }
    // Plain chats: remember every exchange, one note per topic. Checked by asking the memory built from
    // the conversations to learn from for a reply to each held-out message.
    let chats: Vec<(usize, &CorpusConv)> = corpus.convs.iter().enumerate().filter(|(_, c)| plain(c)).collect();
    if !chats.is_empty() {
        let slug = |s: &str| s.to_lowercase().split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).collect::<Vec<_>>().join("-");
        let topic = |c: &CorpusConv| c.domain.clone().filter(|d| !d.trim().is_empty()).unwrap_or_else(|| "general".to_string());
        let note_id = |c: &CorpusConv| format!("chats-{}", slug(&topic(c)));
        let lines = |c: &CorpusConv| -> Vec<(char, String)> {
            c.turns.iter().filter(|t| !t.text.trim().is_empty()).map(|t| (if t.role == "user" { 'U' } else { 'A' }, t.text.split_whitespace().collect::<Vec<_>>().join(" "))).collect()
        };
        let pairs = |conv: &[(char, String)]| (0..conv.len().saturating_sub(1)).filter(|&i| conv[i].0 == 'U' && conv[i + 1].0 == 'A').count();
        let build = |with_held: bool| -> Vec<Note> {
            let mut by: BTreeMap<String, (String, Vec<Vec<(char, String)>>)> = BTreeMap::new();
            for (ci, c) in &chats {
                if with_held || !held_set.contains(ci) {
                    by.entry(note_id(c)).or_insert_with(|| (topic(c), Vec::new())).1.push(lines(c));
                }
            }
            by.into_iter()
                .map(|(id, (topic, new))| {
                    let old = vault.notes.get(&id).filter(|n| n.kind() == "chats");
                    let mut convs = old.map(|n| parse_chats(&n.body)).unwrap_or_default();
                    for c in new {
                        if !convs.contains(&c) {
                            convs.push(c);
                        }
                    }
                    let mut front = Map::new();
                    front.insert("kind".into(), "chats".into());
                    front.insert("topic".into(), topic.into());
                    front.insert("conversations".into(), convs.len().into());
                    front.insert("learned_from".into(), corpus.source.clone().into());
                    front.insert("trust".into(), json!(old.map(|n| n.trust()).unwrap_or(0.8)));
                    Note { path: old.map(|n| n.path.clone()).unwrap_or_else(|| format!("chats/{id}.md")), id, front, body: render_chats(&convs) }
                })
                .collect()
        };
        let mut trial = vault.clone();
        for n in build(false) {
            let _ = trial.put_mem(n);
        }
        let memory = rebuild(&trial).recall;
        let (mut asked, mut found, mut same) = (0usize, 0usize, 0usize);
        for (_, c) in chats.iter().filter(|(ci, _)| held_set.contains(ci)) {
            let conv = lines(c);
            for i in 0..conv.len().saturating_sub(1) {
                if conv[i].0 == 'U' && conv[i + 1].0 == 'A' {
                    asked += 1;
                    if let Some(r) = memory.recall(&conv[i].1, if i > 0 { Some(conv[i - 1].1.as_str()) } else { None }) {
                        found += 1;
                        if r.source == note_id(c) {
                            same += 1;
                        }
                    }
                }
            }
        }
        let all = build(true);
        let replies: usize = chats.iter().map(|(_, c)| pairs(&lines(c))).sum();
        let summary = format!(
            "{} replies remembered from {} conversations on {} topics; for {} of {} held-out messages a similar remembered message was found ({} of them on the same topic). This measures whether something is found, not whether the reply fits",
            replies, chats.len(), all.len(), found, asked, same
        );
        report.candidates.push(CandidateReport { id: "chats".into(), kind: "chats".into(), support: chats.len(), validated: if asked > 0 { found as f64 / asked as f64 } else { 0.0 }, promoted: opts.auto_promote, summary });
        for mut n in all {
            if opts.auto_promote {
                n.front.insert("status".into(), "trusted".into());
            } else {
                n.front.insert("status".into(), "candidate".into());
                n.front.insert("promotes_to".into(), n.id.clone().into());
                n.id = format!("{}-candidate", n.id);
                n.path = format!("candidates/{}.md", n.id);
            }
            notes.push(n);
        }
    }
    let mut m: Vec<Mistake> = mistakes.into_iter().map(|(what, count)| Mistake { what, count }).collect();
    m.sort_by(|a, b| b.count.cmp(&a.count));
    m.truncate(6);
    report.mistakes = m;
    report.seconds = (started.elapsed().as_secs_f64() * 100.0).round() / 100.0;
    Outcome { report, notes }
}

impl Engine {
    /// Learn from a corpus and write what was learned into the vault.
    pub fn learn(&self, corpus: &Corpus, opts: &LearnOptions, stage: &dyn Fn(&str)) -> Result<Report, String> {
        let snapshot = self.vault(|v| v.clone());
        let outcome = run(&snapshot, &*self.factory, corpus, opts, stage);
        stage("write");
        self.change(|v| {
            for note in outcome.notes {
                v.put(note)?;
            }
            Ok(())
        })?;
        stage("done");
        Ok(outcome.report)
    }
}
