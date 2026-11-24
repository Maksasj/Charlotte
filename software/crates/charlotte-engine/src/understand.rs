//! Layer 1, first implementation: reads a message with the phrase lexicon kept in the vault.
//! Every word it knows is data in a note; nothing about any domain is written here.

use charlotte_core::*;
use regex::Regex;
use std::collections::BTreeMap;

const NUM: &str = r"(\d{1,3}(?:,\d{3})+(?:\.\d+)?|\d+(?:\.\d+)?|an?|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|twenty|thirty)";
const ID: &str = r"([A-Za-z]{0,5}-?\d[A-Za-z0-9-]*)";

fn number(s: &str) -> Option<f64> {
    const WORDS: [(&str, f64); 18] = [
        ("a", 1.0), ("an", 1.0), ("one", 1.0), ("two", 2.0), ("three", 3.0), ("four", 4.0), ("five", 5.0), ("six", 6.0), ("seven", 7.0),
        ("eight", 8.0), ("nine", 9.0), ("ten", 10.0), ("eleven", 11.0), ("twelve", 12.0), ("thirteen", 13.0), ("fourteen", 14.0),
        ("fifteen", 15.0), ("twenty", 20.0),
    ];
    let l = s.to_lowercase();
    if l == "thirty" {
        return Some(30.0);
    }
    WORDS.iter().find(|(w, _)| *w == l).map(|(_, n)| *n).or_else(|| l.replace(',', "").parse().ok())
}

/// A phrase from the vault as a pattern: whole words, any spacing, `{n}` a number, `{id}` an identifier.
pub fn phrase_re(text: &str) -> Option<Regex> {
    let t = text.trim();
    if t.is_empty() {
        return None;
    }
    let body = regex::escape(t).replace(r"\{n\}", NUM).replace(r"\{id\}", ID).replace(' ', r"\s+");
    let word = |c: Option<char>| c.map(|c| c.is_alphanumeric()).unwrap_or(false);
    let pre = if word(t.chars().next()) { r"\b" } else { "" };
    let post = if word(t.chars().last()) || t.ends_with('}') { r"\b" } else { "" };
    Regex::new(&format!("(?i){pre}{body}{post}")).ok()
}

struct IntentRe {
    id: String,
    social: bool,
    priority: i32,
    phrases: Vec<(Regex, f64, String)>,
}
struct ValueRe {
    slot: String,
    value: Value,
    flips: bool,
    re: Regex,
}
struct PatternRe {
    slot: String,
    re: Regex,
    scale: f64,
    value: Option<Value>,
    sets: Facts,
    is_id: bool,
}
struct SlotInfo {
    ty: SlotType,
    yes: Option<Value>,
    no: Option<Value>,
}

pub struct LexiconReader {
    intents: Vec<IntentRe>,
    values: Vec<ValueRe>,
    patterns: Vec<PatternRe>,
    signals: Vec<(Signal, Regex)>,
    connectors: Option<Regex>,
    negated: Option<Regex>,
    slots: BTreeMap<String, SlotInfo>,
    bare_num: Regex,
    bare_id: Regex,
}

struct Hit {
    intent: usize,
    pos: usize,
    weight: f64,
    phrase: String,
}
struct Cand {
    start: usize,
    end: usize,
    slot: String,
    value: Value,
    sets: Facts,
    phrase: String,
}

impl LexiconReader {
    pub fn new(mem: &dyn Memory) -> LexiconReader {
        let intents = mem
            .intents()
            .into_iter()
            .map(|i| IntentRe {
                id: i.id.clone(),
                social: i.social,
                priority: i.priority,
                phrases: i.phrases.iter().filter_map(|p| phrase_re(p.text()).map(|r| (r, p.weight(), p.text().to_string()))).collect(),
            })
            .collect();
        let (mut values, mut patterns, mut slots) = (Vec::new(), Vec::new(), BTreeMap::new());
        for s in mem.slots() {
            slots.insert(s.id.clone(), SlotInfo { ty: s.ty, yes: s.yes.clone(), no: s.no.clone() });
            for (key, phrases) in &s.values {
                for p in phrases {
                    if let Some(re) = phrase_re(p.text()) {
                        values.push(ValueRe { slot: s.id.clone(), value: s.value_of(key), flips: s.ty == SlotType::Bool, re });
                    }
                }
            }
            for p in &s.patterns {
                if let Some(re) = phrase_re(&p.text) {
                    patterns.push(PatternRe { slot: s.id.clone(), re, scale: p.scale, value: p.value.clone(), sets: p.sets.clone(), is_id: p.text.contains("{id}") });
                }
            }
        }
        let lex = mem.lexicon();
        let signal = |name: &str, s: Signal| lex.signals.get(name).into_iter().flatten().filter_map(move |p| phrase_re(p).map(|r| (s, r)));
        let signals = signal("failed", Signal::Failed).chain(signal("done", Signal::Done)).chain(signal("no", Signal::No)).chain(signal("yes", Signal::Yes)).collect();
        let alt = |words: &[String]| {
            words.iter().map(|w| { let e = regex::escape(w.trim()); if w.chars().all(|c| c.is_alphanumeric() || c == ' ') { format!(r"\b{e}\b") } else { e } }).collect::<Vec<_>>().join("|")
        };
        let connectors = if lex.connectors.is_empty() { None } else { Regex::new(&format!(r"(?i)\s*(?:{})\s*", alt(&lex.connectors))).ok() };
        let negated = if lex.negators.is_empty() { None } else { Regex::new(&format!(r"(?i)(?:{}|n't)\s+(?:\w+\s+)?$", alt(&lex.negators))).ok() };
        LexiconReader {
            intents,
            values,
            patterns,
            signals,
            connectors,
            negated,
            slots,
            bare_num: Regex::new(r"\b(?:\d{1,3}(?:,\d{3})+|\d+)(?:\.\d+)?\b").unwrap(),
            bare_id: Regex::new(&format!(r"\b{ID}\b")).unwrap(),
        }
    }

    fn hits(&self, text: &str) -> Vec<Hit> {
        let mut out = Vec::new();
        for (i, it) in self.intents.iter().enumerate() {
            for (re, w, phrase) in &it.phrases {
                if let Some(m) = re.find(text) {
                    out.push(Hit { intent: i, pos: m.start(), weight: *w, phrase: phrase.clone() });
                }
            }
        }
        out
    }
    /// The strongest intent among the hits: most phrase weight, then priority, then earliest.
    fn best(&self, hits: &[Hit], social: bool) -> Option<usize> {
        let mut score: BTreeMap<usize, (f64, usize)> = BTreeMap::new();
        for h in hits.iter().filter(|h| self.intents[h.intent].social == social) {
            let e = score.entry(h.intent).or_insert((0.0, usize::MAX));
            e.0 += h.weight;
            e.1 = e.1.min(h.pos);
        }
        score
            .into_iter()
            .max_by(|a, b| {
                a.1 .0.partial_cmp(&b.1 .0).unwrap().then(self.intents[a.0].priority.cmp(&self.intents[b.0].priority)).then(b.1 .1.cmp(&a.1 .1))
            })
            .map(|(i, _)| i)
    }

    fn facts(&self, text: &str, matched: &mut Vec<Match>) -> Facts {
        let mut cands: Vec<Cand> = Vec::new();
        for v in &self.values {
            for m in v.re.find_iter(text) {
                let mut value = v.value.clone();
                if v.flips {
                    if let (Some(neg), Value::Bool(b)) = (&self.negated, &value) {
                        if neg.is_match(&text[..m.start()]) {
                            value = Value::Bool(!b);
                        }
                    }
                }
                cands.push(Cand { start: m.start(), end: m.end(), slot: v.slot.clone(), value, sets: Facts::new(), phrase: m.as_str().to_string() });
            }
        }
        for p in &self.patterns {
            for c in p.re.captures_iter(text) {
                let m = c.get(0).unwrap();
                let value = match (&p.value, c.get(1)) {
                    (Some(v), _) => v.clone(),
                    (None, Some(g)) if p.is_id => Value::Str(g.as_str().to_string()),
                    (None, Some(g)) => match number(g.as_str()) {
                        Some(n) => Value::Num(n * p.scale),
                        None => continue,
                    },
                    _ => continue,
                };
                cands.push(Cand { start: m.start(), end: m.end(), slot: p.slot.clone(), value, sets: p.sets.clone(), phrase: m.as_str().to_string() });
            }
        }
        // Longest wording wins where two overlap ("never dropped" over "dropped"); identical spans both stand.
        cands.sort_by(|a, b| (b.end - b.start).cmp(&(a.end - a.start)).then(a.start.cmp(&b.start)));
        let mut kept: Vec<Cand> = Vec::new();
        for c in cands {
            let clash = kept.iter().any(|k| c.start < k.end && k.start < c.end && !(c.start == k.start && c.end == k.end));
            if !clash {
                kept.push(c);
            }
        }
        kept.sort_by_key(|c| c.start);
        let mut facts = Facts::new();
        for c in kept {
            if facts.contains_key(&c.slot) {
                continue;
            }
            matched.push(Match { phrase: c.phrase, kind: "fact".into(), target: c.slot.clone() });
            for (k, v) in c.sets {
                facts.entry(k).or_insert(v);
            }
            facts.insert(c.slot, c.value);
        }
        facts
    }

    fn clause(&self, text: &str, intent: Option<usize>, hits: &[Hit], ctx: &ReadContext) -> Request {
        let mut r = Request { clause: text.to_string(), ..Default::default() };
        if let Some(i) = intent {
            r.intent = Some(self.intents[i].id.clone());
            for h in hits.iter().filter(|h| h.intent == i) {
                r.matched.push(Match { phrase: h.phrase.clone(), kind: "intent".into(), target: self.intents[i].id.clone() });
            }
        }
        r.facts = self.facts(text, &mut r.matched);
        if r.intent.is_none() {
            if let Some((s, m)) = self.signals.iter().find_map(|(s, re)| re.find(text).map(|m| (*s, m))) {
                r.signal = Some(s);
                r.matched.push(Match { phrase: m.as_str().to_string(), kind: "signal".into(), target: format!("{s:?}").to_lowercase() });
            }
            // A bare answer to the question the agent just asked: "yes", "5", "7741".
            let short = text.split_whitespace().count() <= 4;
            for slot in &ctx.asked {
                let Some(info) = self.slots.get(slot) else { continue };
                if r.facts.contains_key(slot) {
                    continue;
                }
                let yes_no = match r.signal {
                    Some(Signal::Yes) => info.yes.clone().or((info.ty == SlotType::Bool).then_some(Value::Bool(true))),
                    Some(Signal::No) => info.no.clone().or((info.ty == SlotType::Bool).then_some(Value::Bool(false))),
                    _ => None,
                };
                let bare = match info.ty {
                    SlotType::Number if short && r.facts.is_empty() => self.bare_num.find(text).and_then(|m| number(m.as_str())).map(Value::Num),
                    SlotType::Id if short && r.facts.is_empty() => self.bare_id.find(text).map(|m| Value::Str(m.as_str().to_string())),
                    _ => None,
                };
                if let Some(v) = yes_no.or(bare) {
                    r.matched.push(Match { phrase: text.to_string(), kind: "answer".into(), target: slot.clone() });
                    r.facts.insert(slot.clone(), v);
                    break;
                }
            }
        }
        r
    }
}

fn tidy(s: &str) -> String {
    s.trim_matches(|c: char| c.is_whitespace() || ",.;!?".contains(c)).to_string()
}

impl Understand for LexiconReader {
    fn read(&self, text: &str, ctx: &ReadContext) -> Reading {
        let text = text.trim();
        let hits = self.hits(text);
        let mut real: Vec<(usize, usize)> = Vec::new(); // (first position, intent), non-social
        for h in hits.iter().filter(|h| !self.intents[h.intent].social) {
            match real.iter_mut().find(|(_, i)| *i == h.intent) {
                Some(e) => e.0 = e.0.min(h.pos),
                None => real.push((h.pos, h.intent)),
            }
        }
        real.sort();
        if real.len() < 2 {
            let intent = self.best(&hits, false).or_else(|| self.best(&hits, true));
            return Reading { requests: vec![self.clause(text, intent, &hits, ctx)] };
        }
        // Several requests in one message: cut at connectors, else where each next intent starts.
        let mut groups: Vec<(Option<usize>, String)> = Vec::new();
        if let Some(re) = &self.connectors {
            let mut pending = String::new();
            for part in re.split(text).map(tidy).filter(|p| !p.is_empty()) {
                let it = self.best(&self.hits(&part), false);
                match groups.last_mut() {
                    Some((Some(prev), t)) if it.is_none() || it == Some(*prev) => {
                        t.push_str(", ");
                        t.push_str(&part);
                    }
                    _ if it.is_none() => {
                        pending.push_str(&part);
                        pending.push_str(", ");
                    }
                    _ => groups.push((it, format!("{}{}", std::mem::take(&mut pending), part))),
                }
            }
        }
        if groups.len() < 2 {
            groups.clear();
            for (k, (pos, it)) in real.iter().enumerate() {
                let start = if k == 0 { 0 } else { *pos };
                let end = real.get(k + 1).map(|(p, _)| *p).unwrap_or(text.len());
                groups.push((Some(*it), tidy(&text[start..end])));
            }
        }
        Reading { requests: groups.into_iter().map(|(it, t)| { let h = self.hits(&t); self.clause(&t, it, &h, ctx) }).collect() }
    }
}
