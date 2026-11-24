//! Task intents from labelled task dialogues (Schema-Guided Dialogue format).
//!
//! The dialogues say what the user wanted (`INFORM_INTENT`), what the agent had to ask before it could act
//! (`REQUEST`), and which values the user gave (`INFORM`). From those, for every task intent:
//! - **wordings**: n-grams of the user's first request, kept when they point at the intent (precision) and
//!   the same holds on the dialogues held out;
//! - **required slots**, in the order the agent asks them: slots asked in at least `min_required` of the
//!   dialogues of the intent;
//! - **slots**: type from the values given (bool, number, a closed set, or free text), and the agent's own
//!   question as the ask text;
//! - **an atom** whose conditions are "slot given" for each required slot, so the existing rule
//!   "ask what is missing, then act" does the dialogue; its result is a hand-over.
//!
//! Everything lands as candidates; nothing is promoted here.

use crate::engine::Engine;
use charlotte_core::*;
use charlotte_vault::Note;
use serde_json::{json, Map, Value as Json};
use std::collections::{BTreeMap, BTreeSet, HashMap};

#[derive(Clone, Debug)]
pub struct Options {
    pub min_dialogues: usize,
    pub min_required: f64,
    pub min_support: usize,
    pub min_precision: f64,
    pub min_held: usize,
    pub max_gram: usize,
    pub max_phrases: usize,
    /// Closed sets up to this many values become enums.
    pub max_enum: usize,
}
impl Default for Options {
    fn default() -> Self {
        Options { min_dialogues: 30, min_required: 0.4, min_support: 8, min_precision: 0.5, min_held: 3, max_gram: 4, max_phrases: 30, max_enum: 12 }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Dialogue {
    pub id: String,
    pub domain: String,
    pub held: bool,
    pub turns: Vec<Turn>,
}
#[derive(Clone, Debug, Default)]
pub struct Turn {
    pub user: bool,
    pub text: String,
    pub frames: Vec<Frame_>,
}
#[derive(Clone, Debug, Default)]
pub struct Frame_ {
    pub service: String,
    pub intent: String,
    pub requests: Vec<String>,
    pub informs: Vec<(String, String)>,
    pub wants: Option<String>,
}

pub fn parse(text: &str) -> Vec<Dialogue> {
    let mut out = Vec::new();
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let Ok(j) = serde_json::from_str::<Json>(line) else { continue };
        let split = j["split"].as_str().unwrap_or("train");
        let mut d = Dialogue { id: j["id"].as_str().unwrap_or("").into(), domain: j["domain"].as_str().unwrap_or("").into(), held: split != "train", turns: vec![] };
        for t in j["turns"].as_array().into_iter().flatten() {
            let mut turn = Turn { user: t["role"] == "user", text: t["text"].as_str().unwrap_or("").into(), frames: vec![] };
            for l in t["labels"].as_array().into_iter().flatten() {
                let mut f = Frame_ { service: l["service"].as_str().unwrap_or("").into(), intent: l["intent"].as_str().unwrap_or("").into(), ..Default::default() };
                for a in l["acts"].as_array().into_iter().flatten() {
                    let slot = a["slot"].as_str().unwrap_or("");
                    match a["act"].as_str().unwrap_or("") {
                        "REQUEST" if !slot.is_empty() => f.requests.push(slot.into()),
                        "INFORM_INTENT" => f.wants = a["values"][0].as_str().map(str::to_string),
                        "INFORM" | "SELECT" => {
                            for v in a["values"].as_array().into_iter().flatten().filter_map(|v| v.as_str()) {
                                f.informs.push((slot.into(), v.into()));
                            }
                        }
                        _ => {}
                    }
                }
                turn.frames.push(f);
            }
            d.turns.push(turn);
        }
        out.push(d);
    }
    out
}

pub fn kebab(s: &str) -> String {
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if c == '_' || c == ' ' {
            out.push('-');
        } else if c.is_uppercase() {
            if i > 0 && !out.ends_with('-') {
                out.push('-');
            }
            out.extend(c.to_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

fn words(text: &str) -> Vec<String> {
    text.split_whitespace().map(|w| w.trim_matches(|c: char| !c.is_alphanumeric() && c != '\'').to_lowercase()).filter(|w| !w.is_empty()).collect()
}
fn grams(text: &str, max: usize) -> BTreeSet<String> {
    let w = words(text);
    let mut out = BTreeSet::new();
    for n in 1..=max {
        for g in w.windows(n) {
            out.insert(g.join(" "));
        }
    }
    out
}
fn jaccard(a: &BTreeSet<String>, b: &BTreeSet<String>) -> f64 {
    let i = a.intersection(b).count() as f64;
    let u = a.union(b).count() as f64;
    if u == 0.0 {
        0.0
    } else {
        i / u
    }
}

#[derive(Clone, Debug)]
pub struct Task {
    pub intent: String,
    pub id: String,
    pub domain: String,
    pub dialogues: usize,
    pub phrases: Vec<(String, usize, f64, f64)>,
    pub slots: Vec<TaskSlot>,
}
#[derive(Clone, Debug)]
pub struct TaskSlot {
    pub name: String,
    pub id: String,
    pub share: f64,
    pub position: f64,
    pub ty: SlotType,
    pub values: Vec<String>,
    pub ask: String,
}

/// The task intents the dialogues support.
pub fn induce(dialogues: &[Dialogue], opts: &Options) -> Vec<Task> {
    // Dialogues per intent; for each the order in which slots were asked while it was active.
    struct Seen {
        dialogues: usize,
        asked: BTreeMap<String, (usize, f64)>,
        values: BTreeMap<String, BTreeMap<String, usize>>,
        asks: BTreeMap<String, Vec<String>>,
        domain: String,
    }
    let mut seen: BTreeMap<String, Seen> = BTreeMap::new();
    for d in dialogues.iter().filter(|d| !d.held) {
        let mut active: HashMap<String, String> = HashMap::new();
        let mut used: BTreeSet<String> = BTreeSet::new();
        let mut order: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut asked_texts: BTreeMap<(String, String), String> = BTreeMap::new();
        for t in &d.turns {
            for f in &t.frames {
                if t.user {
                    if !f.intent.is_empty() {
                        active.insert(f.service.clone(), f.intent.clone());
                        used.insert(f.intent.clone());
                    }
                    if let Some(i) = active.get(&f.service) {
                        for (s, v) in &f.informs {
                            *seen.entry(i.clone()).or_insert_with(|| Seen { dialogues: 0, asked: BTreeMap::new(), values: BTreeMap::new(), asks: BTreeMap::new(), domain: d.domain.clone() }).values.entry(s.clone()).or_default().entry(v.to_lowercase()).or_insert(0) += 1;
                        }
                    }
                } else if let Some(i) = active.get(&f.service) {
                    for s in &f.requests {
                        let o = order.entry(i.clone()).or_default();
                        if !o.contains(s) {
                            o.push(s.clone());
                        }
                        if t.frames.len() == 1 && f.requests.len() == 1 {
                            asked_texts.insert((i.clone(), s.clone()), t.text.clone());
                        }
                    }
                }
            }
        }
        for i in &used {
            let e = seen.entry(i.clone()).or_insert_with(|| Seen { dialogues: 0, asked: BTreeMap::new(), values: BTreeMap::new(), asks: BTreeMap::new(), domain: d.domain.clone() });
            e.dialogues += 1;
            for (pos, s) in order.get(i).into_iter().flatten().enumerate() {
                let a = e.asked.entry(s.clone()).or_insert((0, 0.0));
                a.0 += 1;
                a.1 += pos as f64;
            }
        }
        for ((i, s), text) in asked_texts {
            if let Some(e) = seen.get_mut(&i) {
                e.asks.entry(s).or_default().push(text);
            }
        }
    }

    // Wordings: first request of each dialogue (where the user says what they want).
    let mut firsts: Vec<(BTreeSet<String>, Option<String>, bool)> = Vec::new();
    for d in dialogues {
        for t in d.turns.iter().filter(|t| t.user) {
            let wants = t.frames.iter().find_map(|f| f.wants.clone());
            firsts.push((grams(&t.text, opts.max_gram), wants, d.held));
        }
    }
    let mut tasks = Vec::new();
    for (intent, s) in &seen {
        if s.dialogues < opts.min_dialogues {
            continue;
        }
        let mut slots: Vec<TaskSlot> = s
            .asked
            .iter()
            .filter(|(_, (n, _))| *n as f64 / s.dialogues as f64 >= opts.min_required)
            .map(|(name, (n, pos))| {
                let vals = s.values.get(name).cloned().unwrap_or_default();
                let total: usize = vals.values().sum();
                let lower: Vec<&String> = vals.keys().collect();
                let is_bool = !lower.is_empty() && lower.iter().all(|v| *v == "true" || *v == "false");
                let is_num = !lower.is_empty() && lower.iter().all(|v| v.parse::<f64>().is_ok());
                let closed: Vec<String> = vals.iter().filter(|(_, c)| **c >= 3).map(|(v, _)| v.clone()).collect();
                let covered: usize = vals.iter().filter(|(_, c)| **c >= 3).map(|(_, c)| *c).sum();
                let (ty, values) = if is_bool {
                    (SlotType::Bool, vec![])
                } else if is_num {
                    (SlotType::Number, vec![])
                } else if vals.len() <= opts.max_enum && closed.len() >= 2 && covered as f64 >= 0.9 * total as f64 {
                    (SlotType::Enum, closed)
                } else {
                    (SlotType::Text, vec![])
                };
                TaskSlot { name: name.clone(), id: format!("{}-{}", kebab(intent), kebab(name)), share: *n as f64 / s.dialogues as f64, position: pos / *n as f64, ty, values, ask: medoid(s.asks.get(name)) }
            })
            .collect();
        slots.sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap().then(a.name.cmp(&b.name)));

        // n-gram precision on the search side, checked on the held-out side.
        let mut c: BTreeMap<&String, [usize; 4]> = BTreeMap::new(); // hit, all, held hit, held all
        for (g, wants, held) in &firsts {
            let is = wants.as_deref() == Some(intent.as_str());
            for gram in g {
                let e = c.entry(gram).or_insert([0; 4]);
                let k = if *held { 2 } else { 0 };
                e[k + 1] += 1;
                if is {
                    e[k] += 1;
                }
            }
        }
        let mut phrases: Vec<(String, usize, f64, f64)> = c
            .into_iter()
            .filter(|(g, e)| {
                e[0] >= opts.min_support
                    && e[0] as f64 / e[1] as f64 >= opts.min_precision
                    && e[3] >= opts.min_held
                    && e[2] as f64 / e[3] as f64 >= opts.min_precision
                    && g.chars().any(|ch| ch.is_alphabetic())
            })
            .map(|(g, e)| (g.clone(), e[0], e[0] as f64 / e[1] as f64, e[2] as f64 / e[3] as f64))
            .collect();
        // Prefer wordings that cover many dialogues; drop a short one that a longer, equally frequent one contains.
        let all = phrases.clone();
        phrases.retain(|p| !all.iter().any(|o| o.0.len() > p.0.len() && format!(" {} ", o.0).contains(&format!(" {} ", p.0)) && o.1 as f64 >= 0.8 * p.1 as f64));
        phrases.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        phrases.truncate(opts.max_phrases);
        if phrases.is_empty() {
            continue;
        }
        tasks.push(Task { intent: intent.clone(), id: kebab(intent), domain: s.domain.clone(), dialogues: s.dialogues, phrases, slots });
    }
    tasks
}

/// The agent question most like the others asked for the slot.
fn medoid(asks: Option<&Vec<String>>) -> String {
    let Some(asks) = asks else { return String::new() };
    let sample: Vec<&String> = asks.iter().step_by((asks.len() / 150).max(1)).collect();
    let sets: Vec<BTreeSet<String>> = sample.iter().map(|a| grams(a, 1)).collect();
    let best = (0..sets.len()).max_by(|&i, &j| {
        let score = |i: usize| sets.iter().enumerate().filter(|(k, _)| *k != i).map(|(_, s)| jaccard(&sets[i], s)).sum::<f64>() / (sets[i].len().max(6) as f64);
        score(i).partial_cmp(&score(j)).unwrap()
    });
    best.map(|i| sample[i].trim().to_string()).unwrap_or_default()
}

fn round(x: f64) -> f64 {
    (x * 1000.0).round() / 1000.0
}

/// Candidate notes for the tasks: an intent, a slot per required slot, and an atom.
pub fn candidate_notes(tasks: &[Task], source: &str, mem: &dyn Memory) -> Vec<Note> {
    let mut notes = Vec::new();
    let status = |front: &mut Map<String, Json>| {
        front.insert("status".into(), "candidate".into());
        front.insert("induced_from".into(), source.into());
    };
    let known: BTreeSet<String> = mem.intents().iter().map(|i| i.id.clone()).collect();
    for t in tasks {
        if known.contains(&t.id) {
            continue;
        }
        let mut front = Map::new();
        front.insert("kind".into(), "intent".into());
        let list: Vec<_> = t.phrases.iter().map(|p| json!({ "text": p.0, "support": p.1, "precision": round(p.2), "held_precision": round(p.3), "learned": true })).collect();
        front.insert("phrases".into(), list.into());
        status(&mut front);
        let mut body = format!("Task intent {} from {source}: {} dialogues. Wordings that point at it:\n\n", t.intent, t.dialogues);
        for p in &t.phrases {
            body.push_str(&format!("- `{}`: {} requests, {:.0}% of the messages that have it; {:.0}% held out\n", p.0, p.1, p.2 * 100.0, p.3 * 100.0));
        }
        notes.push(Note { id: t.id.clone(), path: format!("candidates/{}.md", t.id), front, body });

        for s in &t.slots {
            let mut front = Map::new();
            front.insert("kind".into(), "slot".into());
            front.insert(
                "type".into(),
                match s.ty {
                    SlotType::Bool => "bool",
                    SlotType::Number => "number",
                    SlotType::Enum => "enum",
                    _ => "text",
                }
                .into(),
            );
            if !s.ask.is_empty() {
                front.insert("ask".into(), s.ask.clone().into());
            }
            if s.ty == SlotType::Enum {
                let mut m = Map::new();
                for v in &s.values {
                    m.insert(v.clone(), json!([v]));
                }
                front.insert("values".into(), m.into());
            }
            status(&mut front);
            let body = format!("Asked in {:.0}% of the {} dialogues of {}, on average as question {:.1}.\n", s.share * 100.0, t.dialogues, t.intent, s.position + 1.0);
            notes.push(Note { id: s.id.clone(), path: format!("candidates/{}.md", s.id), front, body });
        }

        let mut front = Map::new();
        front.insert("kind".into(), "atom".into());
        front.insert("intent".into(), format!("[[{}]]", t.id).into());
        front.insert("object".into(), t.id.clone().into());
        front.insert("domain".into(), t.domain.clone().into());
        front.insert("priority".into(), 1.into());
        front.insert("result".into(), "handoff".into());
        let conds: Vec<_> = t.slots.iter().map(|s| json!({ "slot": format!("[[{}]]", s.id), "op": "!=", "value": "" })).collect();
        front.insert("conditions".into(), conds.into());
        front.insert("trust".into(), 0.5.into());
        status(&mut front);
        let given: Vec<String> = t.slots.iter().map(|s| format!("{} {{{}}}", s.name.replace('_', " "), s.id)).collect();
        let body = if given.is_empty() { "That is everything I need. I'm handing this over to be carried out.".to_string() } else { format!("I have everything I need ({}). I'm handing this over to be carried out.", given.join(", ")) };
        notes.push(Note { id: format!("{}-handoff", t.id), path: format!("candidates/{}-handoff.md", t.id), front, body });
    }
    notes
}

#[derive(Clone, Debug, Default)]
pub struct Score {
    pub dialogues: usize,
    pub intent_read: usize,
    pub asked_right: usize,
    pub handoff: usize,
    pub slot_precision: (usize, usize),
    pub slot_recall: (usize, usize),
}

/// Plays held-out dialogues against the engine: the user's first request, then each question answered with the
/// value the dialogue gives for that slot. Counts the intent read, the right slots asked, and the hand-over.
pub fn evaluate(engine: &Engine, dialogues: &[Dialogue], tasks: &[Task], limit: usize) -> Score {
    let by_intent: HashMap<&str, &Task> = tasks.iter().map(|t| (t.intent.as_str(), t)).collect();
    let mut score = Score::default();
    for d in dialogues.iter().filter(|d| d.held).take(limit) {
        let Some(first) = d.turns.iter().find(|t| t.user) else { continue };
        let Some((intent, service)) = first.frames.iter().find_map(|f| f.wants.clone().map(|w| (w, f.service.clone()))) else { continue };
        let Some(task) = by_intent.get(intent.as_str()) else { continue };
        score.dialogues += 1;
        // What the dialogue says for each slot of the service, the last value given.
        let mut said: HashMap<String, String> = HashMap::new();
        let mut real: Vec<String> = Vec::new();
        for t in &d.turns {
            for f in t.frames.iter().filter(|f| f.service == service) {
                for (s, v) in &f.informs {
                    said.insert(s.clone(), v.clone());
                }
                if !t.user {
                    real.extend(f.requests.iter().cloned());
                }
            }
        }
        let mut conv = engine.conversation();
        let mut reply = conv.say(&first.text);
        let mut asked: Vec<String> = Vec::new();
        let mut read = false;
        let mut done = false;
        for _ in 0..task.slots.len() + 2 {
            if reply.trace.requests.iter().any(|r| r.intent.as_deref() == Some(task.id.as_str())) {
                read = true;
            }
            if reply.acts.iter().any(|a| a.op == Op::Grant) {
                done = true;
                break;
            }
            let Some(slot) = reply.acts.iter().find(|a| a.op == Op::Ask).and_then(|a| a.arg.clone()) else { break };
            asked.push(slot.clone());
            let Some(ts) = task.slots.iter().find(|s| s.id == slot) else { break };
            let answer = said.get(&ts.name).cloned().unwrap_or_else(|| "i don't mind".into());
            reply = conv.say(&answer);
        }
        if read {
            score.intent_read += 1;
        }
        if done {
            score.handoff += 1;
        }
        let names: Vec<&str> = asked.iter().filter_map(|a| task.slots.iter().find(|s| &s.id == a)).map(|s| s.name.as_str()).collect();
        score.slot_precision.1 += names.len();
        score.slot_precision.0 += names.iter().filter(|n| real.iter().any(|r| r == **n)).count();
        let required: Vec<&str> = task.slots.iter().map(|s| s.name.as_str()).collect();
        let real_required: Vec<&&str> = required.iter().filter(|n| real.iter().any(|r| r == **n)).collect();
        score.slot_recall.1 += real_required.len();
        score.slot_recall.0 += real_required.iter().filter(|n| names.contains(**n)).count();
        if names == real.iter().filter(|r| required.contains(&r.as_str())).map(|s| s.as_str()).collect::<Vec<_>>() {
            score.asked_right += 1;
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dialogue(i: usize, held: bool) -> String {
        let user = format!(r#"{{"role":"user","text":"please book a table for me {i}","labels":[{{"service":"R","acts":[{{"act":"INFORM_INTENT","slot":"intent","values":["BookTable"]}}],"intent":"BookTable","slot_values":{{}}}}]}}"#);
        let ask = r#"{"role":"agent","text":"What time?","labels":[{"service":"R","acts":[{"act":"REQUEST","slot":"time"}]}]}"#;
        let ans = r#"{"role":"user","text":"7 pm","labels":[{"service":"R","acts":[{"act":"INFORM","slot":"time","values":["7 pm"]}],"intent":"BookTable","slot_values":{"time":["7 pm"]}}]}"#;
        format!(r#"{{"id":"d{i}","domain":"x","split":"{}","turns":[{user},{ask},{ans}]}}"#, if held { "test" } else { "train" })
    }

    #[test]
    fn learns_intent_wording_and_asked_slots() {
        let text: String = (0..80).map(|i| dialogue(i, i % 4 == 3) + "\n").collect();
        let found = induce(&parse(&text), &Options::default());
        assert_eq!(found.len(), 1);
        let t = &found[0];
        assert_eq!(t.id, "book-table");
        assert!(t.phrases.iter().any(|p| p.0.contains("book a table")));
        assert_eq!(t.slots.len(), 1);
        assert_eq!((t.slots[0].name.as_str(), t.slots[0].ty), ("time", SlotType::Text));
        assert_eq!(t.slots[0].ask, "What time?");
    }
}
