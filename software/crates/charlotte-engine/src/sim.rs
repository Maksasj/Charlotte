//! A simulated teacher. It holds labelled conversations over whatever is in the vault, following a
//! fixed policy the learner never sees, with a simulated user who answers, corrects and gets stuck.
//! Its output is a corpus in the same format a real one would have.

use crate::engine::Layers;
use crate::learn::{CorpusConv, CorpusTurn};
use crate::view::{view, View};
use charlotte_core::*;
use charlotte_vault::Vault;
use serde::Deserialize;
use std::collections::BTreeMap;

/// Extra wordings for the simulated user, beyond the lexicon: whole request sentences per intent
/// (`{subject}` is replaced) and wordings per slot value.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Paraphrases {
    #[serde(default)]
    pub intents: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    pub values: BTreeMap<String, BTreeMap<String, Vec<String>>>,
}
pub struct SimOptions {
    pub n: usize,
    /// When set, `n` conversations of each shape (rule check, procedure, lookup) instead of `n` in all.
    pub per_shape: bool,
    pub seed: u64,
    /// Share of rule-check conversations in which the user may correct a fact.
    pub corrections: f64,
    /// A request that needs a record calls a lookup subframe instead of fetching it itself.
    pub calls: bool,
}

struct Rng(u32);
impl Rng {
    fn f(&mut self) -> f64 {
        self.0 = self.0.wrapping_add(0x6D2B79F5);
        let mut t = (self.0 ^ (self.0 >> 15)).wrapping_mul(1 | self.0);
        t = (t.wrapping_add((t ^ (t >> 7)).wrapping_mul(61 | t))) ^ t;
        ((t ^ (t >> 14)) as f64) / 4294967296.0
    }
    fn below(&mut self, n: usize) -> usize {
        ((self.f() * n as f64) as usize).min(n.saturating_sub(1))
    }
    fn pick<'a, T>(&mut self, xs: &'a [T]) -> Option<&'a T> {
        if xs.is_empty() {
            None
        } else {
            Some(&xs[self.below(xs.len())])
        }
    }
}

/// The policy the learner has to recover.
fn teacher(frame: &Frame, v: &View, calls: bool) -> Act {
    let subject = frame.facts.contains_key("subject");
    let fetch = || if subject { Act::new(Op::Lookup) } else { Act::with(Op::Ask, "subject") };
    let d = match v.kind {
        ReqKind::Procedure => {
            let n = v.procedure.map(|p| p.steps.len()).unwrap_or(0);
            if frame.user_acts.last() == Some(&UserAct::Failed) {
                if frame.agent_acts.last() == Some(&Op::Repeat) { Act::new(Op::Escalate) } else { Act::with(Op::Repeat, frame.steps_done.to_string()) }
            } else if frame.steps_done < n {
                Act::with(Op::Instruct, frame.steps_done.to_string())
            } else {
                Act::new(Op::Finish)
            }
        }
        ReqKind::Lookup => {
            if !frame.fetched { fetch() } else if frame.found { Act::new(Op::Report) } else { Act::new(Op::Escalate) }
        }
        _ => {
            let p = v.prim.expect("a gated request has a primary atom");
            if v.p.violated.is_some() {
                match v.alt {
                    Some(a) if v.a.violated.is_none() => match v.a.unbound.first() {
                        Some(&i) => Act::with(Op::Ask, a.conditions[i].slot.clone()),
                        None => Act::new(Op::Offer),
                    },
                    _ => Act::new(Op::Deny),
                }
            } else if v.p.unbound.iter().any(|&i| p.conditions[i].source == Source::Record) {
                match (&v.record_source, calls) {
                    (Some(r), true) if frame.returned_from.is_none() => Act::with(Op::Call, r.clone()),
                    (_, true) => Act::new(Op::Escalate),
                    _ if !frame.fetched => fetch(),
                    _ => Act::new(Op::Escalate),
                }
            } else {
                match v.p.unbound.first() {
                    Some(&i) => Act::with(Op::Ask, p.conditions[i].slot.clone()),
                    None => Act::new(Op::Grant),
                }
            }
        }
    };
    // After a correction, a decision that still stands is confirmed rather than repeated.
    if frame.user_acts.last() == Some(&UserAct::Correct) && frame.standing == Some(d.op) { Act::new(Op::Confirm) } else { d }
}

/// A value for a condition's slot that meets the condition, or breaks it.
fn value_for(c: &Condition, meet: bool, slot: Option<&Slot>, r: &mut Rng) -> Value {
    let b = c.value.as_f64();
    let other = |r: &mut Rng| -> Value {
        if let Value::Bool(x) = c.value {
            return Value::Bool(!x);
        }
        let keys: Vec<Value> = slot.map(|s| s.values.keys().map(|k| s.value_of(k)).filter(|v| !v.same(&c.value)).collect()).unwrap_or_default();
        r.pick(&keys).cloned().unwrap_or_else(|| match b {
            Some(n) => Value::Num(n + 1.0),
            None => Value::Str("other".into()),
        })
    };
    let num = |lo: f64, hi: f64, r: &mut Rng| Value::Num(lo + r.below((hi - lo).max(0.0) as usize + 1) as f64);
    match (c.op, b) {
        (CmpOp::Eq, _) => if meet { c.value.clone() } else { other(r) },
        (CmpOp::Ne, _) => if meet { other(r) } else { c.value.clone() },
        (CmpOp::Le, Some(n)) => if meet { num((n - 20.0).max(1.0).min(n), n, r) } else { num(n + 1.0, n + 30.0, r) },
        (CmpOp::Lt, Some(n)) => if meet { num((n - 20.0).max(0.0).min(n - 1.0), n - 1.0, r) } else { num(n, n + 30.0, r) },
        (CmpOp::Ge, Some(n)) => if meet { num(n, n + 20.0, r) } else { num((n - 20.0).max(0.0), (n - 1.0).max(0.0), r) },
        (CmpOp::Gt, Some(n)) => if meet { num(n + 1.0, n + 20.0, r) } else { num((n - 20.0).max(0.0), n, r) },
        _ => c.value.clone(),
    }
}

struct Sim<'a> {
    vault: &'a Vault,
    l: &'a Layers,
    para: &'a Paraphrases,
    r: Rng,
    calls: bool,
}
impl Sim<'_> {
    /// How a user would state a fact.
    fn say_fact(&mut self, slot: &str, value: &Value) -> String {
        let Some(s) = self.vault.slot(slot) else { return format!("{slot} is {}", value.text()) };
        let mut options: Vec<String> = Vec::new();
        for (key, phrases) in &s.values {
            if s.value_of(key).same(value) {
                options.extend(phrases.iter().map(|p| p.text().to_string()));
                options.extend(self.para.values.get(slot).and_then(|m| m.get(key)).into_iter().flatten().cloned());
            }
        }
        if let Some(n) = value.as_f64() {
            for p in s.patterns.iter().filter(|p| p.text.contains("{n}") && p.scale > 0.0 && (n / p.scale).fract() == 0.0 && n / p.scale >= 1.0) {
                options.push(p.text.replace("{n}", &Value::Num(n / p.scale).text()));
            }
            options.extend(s.patterns.iter().filter(|p| p.value.as_ref().map_or(false, |v| v.same(value))).map(|p| p.text.clone()));
        }
        self.r.pick(&options).cloned().unwrap_or_else(|| format!("{slot} is {}", value.text()))
    }
    fn say_subject(&mut self, id: &str, object: Option<&str>) -> String {
        let pats: Vec<&SlotPattern> = self.vault.slot("subject").map(|s| s.patterns.iter().filter(|p| p.text.contains("{id}")).collect()).unwrap_or_default();
        let fits: Vec<&&SlotPattern> = pats.iter().filter(|p| p.sets.get("object").map(|o| o.text()) == object.map(str::to_string)).collect();
        let plain: Vec<&&SlotPattern> = pats.iter().filter(|p| p.sets.is_empty()).collect();
        match self.r.pick(&fits).or(self.r.pick(&plain)).map(|p| **p) {
            Some(p) => p.text.replace("{id}", id),
            None => id.to_string(),
        }
    }

    fn conversation(&mut self, n: usize, intent: &str, object: Option<&str>, corrections: bool) -> CorpusConv {
        let (vault, l) = (self.vault, self.l);
        let mem: &dyn Memory = vault;
        let mut atoms: Vec<&Atom> = mem.atoms(intent).into_iter().filter(|a| a.object.as_deref() == object).collect();
        atoms.sort_by(|a, b| b.priority.cmp(&a.priority).then(a.id.cmp(&b.id)));
        let conds: Vec<&Condition> = atoms.iter().flat_map(|a| a.conditions.iter()).collect();
        let needs_record = atoms.first().map_or(false, |a| a.conditions.iter().any(|c| c.source == Source::Record));
        let is_lookup = mem.intent(intent).map_or(false, |i| i.lookup) && atoms.is_empty();

        // The subject: a real record most of the time when one is needed, otherwise a made-up reference.
        let rows: Vec<&RecordRow> = mem
            .records()
            .into_iter()
            .filter(|t| object.is_none() || t.object.as_deref() == object)
            .flat_map(|t| t.rows.iter())
            .filter(|row| !needs_record || conds.iter().filter(|c| c.source == Source::Record).all(|c| row.facts.contains_key(&c.slot)))
            .collect();
        let row = if (needs_record || is_lookup) && self.r.f() < 0.85 { self.r.pick(&rows).copied() } else { None };
        let subject = row.map(|r| r.id.clone()).unwrap_or_else(|| format!("{}", 1000 + self.r.below(9000)));

        // What is true in this conversation.
        let mut truth = Facts::new();
        let meet_all = self.r.f() < 0.5;
        for (i, a) in atoms.iter().enumerate() {
            for c in &a.conditions {
                if !truth.contains_key(&c.slot) {
                    let meet = if i == 0 { meet_all || self.r.f() < 0.5 } else { self.r.f() < 0.5 };
                    let v = value_for(c, meet, mem.slot(&c.slot), &mut self.r);
                    truth.insert(c.slot.clone(), v);
                }
            }
        }
        if let Some(row) = row {
            for c in conds.iter().filter(|c| c.source == Source::Record) {
                if let Some(v) = row.facts.get(&c.slot) {
                    truth.insert(c.slot.clone(), v.clone());
                }
            }
        }
        let user_slots: Vec<String> = conds.iter().filter(|c| c.source == Source::User).map(|c| c.slot.clone()).collect::<std::collections::BTreeSet<_>>().into_iter().collect();

        let mut state = State::default();
        let mut turns: Vec<CorpusTurn> = Vec::new();
        let user = |state: &mut State, turns: &mut Vec<CorpusTurn>, text: String, intent: Option<&str>, facts: Facts, signal: Option<Signal>| {
            let reading = Reading { requests: vec![Request { clause: text.clone(), intent: intent.map(str::to_string), facts: facts.clone(), signal, refer: None, conflicts: Default::default(), matched: vec![] }] };
            l.track.user(state, &reading, mem);
            turns.push(CorpusTurn { role: "user".into(), text, intent: intent.map(str::to_string), facts: Some(facts), signal, ..Default::default() });
        };

        // The opening message.
        let mut facts = Facts::new();
        let mention = self.r.f() < 0.5;
        if let Some(o) = object {
            facts.insert("object".into(), Value::Str(o.into()));
        }
        if mention {
            facts.insert("subject".into(), Value::Str(subject.clone()));
        }
        let subject_text = if mention { self.say_subject(&subject, object) } else { object.map(|o| format!("my {o}")).unwrap_or_else(|| "it".into()) };
        let mut options: Vec<String> = self.para.intents.get(intent).cloned().unwrap_or_default();
        options.extend(mem.intent(intent).into_iter().flat_map(|i| i.phrases.iter()).map(|p| format!("{} for {{subject}}", p.text())));
        let mut text = self.r.pick(&options).cloned().unwrap_or_else(|| intent.to_string()).replace("{subject}", &subject_text);
        for s in &user_slots {
            if self.r.f() < 0.4 {
                let said = self.say_fact(s, &truth[s]);
                text.push_str(", ");
                text.push_str(&said);
                facts.insert(s.clone(), truth[s].clone());
            }
        }
        user(&mut state, &mut turns, text, Some(intent), facts, None);

        let signals = |name: &str| mem.lexicon().signals.get(name).cloned().unwrap_or_else(|| vec![name.to_string()]);
        let mut corrected = 0;
        // The active frame: the request itself, or the subframe it called.
        let mut cur = 0;
        for _ in 0..40 {
            // The agent's turn: silent lookups and calls, then one spoken act.
            let mut acts = Vec::new();
            let last = loop {
                let act = teacher(&state.frames[cur], &view(&state.frames[cur], mem), self.calls);
                l.track.agent(&mut state.frames[cur], &act, mem);
                acts.push(act.clone());
                if acts.len() > 6 {
                    break act;
                }
                match act.op {
                    Op::Lookup => {}
                    Op::Call => cur = l.track.call(&mut state, cur, act.arg.as_deref().unwrap_or_default(), mem),
                    _ => match l.track.ret(&mut state, cur, &act, mem) {
                        Some(p) => cur = p,
                        None => break act,
                    },
                }
            };
            let text = l.realize.realize(&state.frames[cur], &last, mem);
            turns.push(CorpusTurn { role: "agent".into(), text, acts: Some(acts), ..Default::default() });

            // A fact the user already gave, said again with the other value.
            let correct = |sim: &mut Sim, truth: &mut Facts, skip: Option<&str>| -> Option<(String, Value, String)> {
                let bound: Vec<&String> = user_slots.iter().filter(|s| state.frames[0].facts.contains_key(*s) && Some(s.as_str()) != skip).collect();
                let slot = (*sim.r.pick(&bound)?).clone();
                let c = conds.iter().find(|c| c.slot == slot)?;
                let v = value_for(c, !c.holds(&truth[&slot]), mem.slot(&slot), &mut sim.r);
                if v.same(&truth[&slot]) {
                    return None;
                }
                truth.insert(slot.clone(), v.clone());
                let said = sim.say_fact(&slot, &v);
                Some((slot, v, said))
            };
            match last.op {
                Op::Ask => {
                    let slot = last.arg.clone().unwrap_or_default();
                    let mut facts = Facts::new();
                    let mut text;
                    if slot == "subject" {
                        text = self.say_subject(&subject, object);
                        facts.insert("subject".into(), Value::Str(subject.clone()));
                    } else {
                        let v = truth.get(&slot).cloned().unwrap_or(Value::Bool(true));
                        text = self.say_fact(&slot, &v);
                        facts.insert(slot.clone(), v);
                        if corrections && self.r.f() < 0.15 {
                            if let Some((s, v, said)) = correct(self, &mut truth, Some(&slot)) {
                                text = format!("{text}, and actually {said}");
                                facts.insert(s, v);
                            }
                        }
                    }
                    user(&mut state, &mut turns, text, None, facts, None);
                }
                Op::Grant | Op::Offer | Op::Deny | Op::Confirm => {
                    let again = corrections && corrected < 3 && self.r.f() < if corrected == 0 { 0.7 } else { 0.5 };
                    match if again { correct(self, &mut truth, None) } else { None } {
                        Some((s, v, said)) => {
                            corrected += 1;
                            user(&mut state, &mut turns, format!("actually, {said}"), None, Facts::from([(s, v)]), None);
                        }
                        None => break,
                    }
                }
                Op::Instruct | Op::Repeat => {
                    let ok = self.r.f() < if last.op == Op::Instruct { 0.8 } else { 0.6 };
                    let words = signals(if ok { "done" } else { "failed" });
                    let text = self.r.pick(&words).cloned().unwrap_or_default();
                    user(&mut state, &mut turns, text, None, Facts::new(), Some(if ok { Signal::Done } else { Signal::Failed }));
                }
                _ => break,
            }
        }
        CorpusConv { id: Some(format!("sim-{:05}", n + 1)), domain: atoms.first().map(|a| a.domain.clone()).filter(|d| !d.is_empty()), outcome: Some("resolved".into()), turns }
    }
}

pub fn simulate(vault: &Vault, l: &Layers, para: &Paraphrases, opts: &SimOptions) -> Vec<CorpusConv> {
    let mem: &dyn Memory = vault;
    // Every request the vault can answer with a pattern: (intent, object, is a rule check).
    let (mut gated, mut procs, mut lookups) = (Vec::new(), Vec::new(), Vec::new());
    for i in mem.intents() {
        let atoms = mem.atoms(&i.id);
        if !atoms.is_empty() {
            let objects: std::collections::BTreeSet<Option<String>> = atoms.iter().map(|a| a.object.clone()).collect();
            gated.extend(objects.into_iter().map(|o| (i.id.clone(), o, true)));
        } else if mem.procedure(&i.id).is_some() {
            procs.push((i.id.clone(), None, false));
        } else if i.lookup {
            lookups.push((i.id.clone(), None, false));
        }
    }
    let mut sim = Sim { vault, l, para, r: Rng(opts.seed as u32), calls: opts.calls };
    let shuffle = |d: &mut Vec<(String, Option<String>, bool)>, sim: &mut Sim| {
        for i in (1..d.len()).rev() {
            let j = sim.r.below(i + 1);
            d.swap(i, j);
        }
    };
    // Which domain each conversation is in: cycling through all of them, or through each shape in turn.
    let plan: Vec<(String, Option<String>, bool)> = if opts.per_shape {
        let mut plan = Vec::new();
        for mut shape in [gated, procs, lookups] {
            shuffle(&mut shape, &mut sim);
            if !shape.is_empty() {
                plan.extend((0..opts.n).map(|i| shape[i % shape.len()].clone()));
            }
        }
        plan
    } else {
        let mut all: Vec<_> = gated.into_iter().chain(procs).chain(lookups).collect();
        shuffle(&mut all, &mut sim);
        if all.is_empty() { Vec::new() } else { (0..opts.n).map(|i| all[i % all.len()].clone()).collect() }
    };
    plan.into_iter()
        .enumerate()
        .map(|(n, (intent, object, gated))| {
            let corrections = gated && sim.r.f() < opts.corrections;
            sim.conversation(n, &intent, object.as_deref(), corrections)
        })
        .collect()
}

pub fn paraphrases_from_yaml(text: &str) -> Result<Paraphrases, String> {
    serde_yaml::from_str(text).map_err(|e| e.to_string())
}
