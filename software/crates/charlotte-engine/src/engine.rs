//! The engine: a vault, four working layers built from it, and the turn pipeline that runs them.

use crate::decide::RuleDecide;
use crate::realize::TemplateRealize;
use crate::track::FrameTracker;
use crate::understand::LexiconReader;
use charlotte_core::*;
use charlotte_vault::{Note, Vault};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value as Json};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::sync::RwLock;

pub struct Layers {
    pub understand: Box<dyn Understand>,
    pub track: Box<dyn Track>,
    pub decide: Box<dyn Decide>,
    pub realize: Box<dyn Realize>,
    pub recall: Box<dyn Recall>,
}
/// Builds the layers for a vault. Replace one layer by passing a factory that returns a different one.
pub type LayerFactory = Box<dyn Fn(&Vault) -> Layers + Send + Sync>;

pub fn default_layers(vault: &Vault) -> Layers {
    Layers {
        understand: Box::new(LexiconReader::new(vault)),
        track: Box::new(FrameTracker),
        decide: Box::new(RuleDecide),
        realize: Box::new(TemplateRealize),
        recall: Box::new(crate::recall::ReplyMemory::new(vault)),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Verdict {
    Resolved,
    Partial,
    Failed,
}
#[derive(Clone, Debug, Serialize)]
pub struct TrustChange {
    pub id: String,
    pub before: f64,
    pub after: f64,
}
#[derive(Clone, Debug, Serialize)]
pub struct Message {
    pub role: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<Trace>,
}
/// One conversation's data. Owned by the caller; the engine only reads and updates it.
#[derive(Clone, Debug, Default, Serialize)]
pub struct ConvState {
    pub id: String,
    pub closed: bool,
    pub verdict: Option<Verdict>,
    pub messages: Vec<Message>,
    #[serde(rename = "frames", serialize_with = "frames_only")]
    pub state: State,
    #[serde(skip)]
    pub used: BTreeSet<String>,
}
fn frames_only<S: serde::Serializer>(s: &State, ser: S) -> Result<S::Ok, S::Error> {
    s.frames.serialize(ser)
}

#[derive(Clone, Debug, Serialize)]
pub struct ItemSummary {
    pub id: String,
    pub kind: String,
    pub status: String,
    pub trust: f64,
    pub path: String,
    pub title: String,
}
#[derive(Clone, Debug, Serialize)]
pub struct Item {
    pub id: String,
    pub kind: String,
    pub status: String,
    pub trust: f64,
    pub path: String,
    pub front: Map<String, Json>,
    pub body: String,
    pub links: Vec<String>,
    pub backlinks: Vec<String>,
}

pub(crate) struct Inner {
    pub vault: Vault,
    pub layers: Layers,
}
pub struct Engine {
    pub(crate) inner: RwLock<Inner>,
    pub(crate) factory: LayerFactory,
}

/// A conversation held through the library.
pub struct Conversation<'a> {
    engine: &'a Engine,
    pub state: ConvState,
}
impl Conversation<'_> {
    pub fn say(&mut self, text: &str) -> Reply {
        self.engine.say(&mut self.state, text)
    }
    /// Teach what the agent should have said to the last message.
    pub fn teach(&self, reply: &str) -> Result<Item, String> {
        self.engine.teach(&self.state, reply)
    }
    pub fn close(&mut self, verdict: Verdict) -> Result<Vec<TrustChange>, String> {
        self.engine.close(&mut self.state, verdict)
    }
}

fn words(s: &str) -> BTreeSet<String> {
    s.to_lowercase().split(|c: char| !c.is_alphanumeric()).filter(|w| w.len() > 2).map(str::to_string).collect()
}
fn folder(kind: &str) -> &'static str {
    match kind {
        "pattern" => "patterns",
        "atom" => "atoms",
        "procedure" => "procedures",
        "intent" => "lexicon/intents",
        "slot" => "lexicon/slots",
        "records" => "records",
        "move" => "moves",
        "chats" => "chats",
        _ => "lexicon",
    }
}

impl Engine {
    pub fn open(path: impl AsRef<Path>) -> Result<Engine, String> {
        Ok(Engine::with_layers(Vault::open(path)?, Box::new(default_layers)))
    }
    pub fn with_layers(vault: Vault, factory: LayerFactory) -> Engine {
        let layers = factory(&vault);
        Engine { inner: RwLock::new(Inner { vault, layers }), factory }
    }
    pub fn conversation(&self) -> Conversation<'_> {
        Conversation { engine: self, state: ConvState::default() }
    }
    /// Read access to the vault.
    pub fn vault<R>(&self, f: impl FnOnce(&Vault) -> R) -> R {
        f(&self.inner.read().unwrap().vault)
    }
    /// Change the vault, then rebuild the layers from it.
    pub(crate) fn change<R>(&self, f: impl FnOnce(&mut Vault) -> Result<R, String>) -> Result<R, String> {
        let mut inner = self.inner.write().unwrap();
        let r = f(&mut inner.vault);
        inner.layers = (self.factory)(&inner.vault);
        r
    }

    /// One turn: understand, track, decide, realize.
    pub fn say(&self, conv: &mut ConvState, text: &str) -> Reply {
        let inner = self.inner.read().unwrap();
        let (mem, l): (&dyn Memory, &Layers) = (&inner.vault, &inner.layers);
        // What the agent said last, for recalling replies in context.
        let previous = conv.messages.last().filter(|m| m.role == "agent").map(|m| m.text.clone());
        conv.messages.push(Message { role: "user".into(), text: text.to_string(), trace: None });
        let remembered = |clause: &str, trace: &mut Trace, used: &mut BTreeSet<String>| {
            l.recall.recall(clause, previous.as_deref()).map(|r| {
                trace.notes.push(format!("recalled from [[{}]] entry {}, similarity {:.2}: \"{}\"", r.source, r.entry, r.score, r.message));
                used.insert(r.source.clone());
                let why = RecalledTrace { source: r.source.clone(), entry: r.entry, message: r.message, score: r.score, threshold: crate::recall::THRESHOLD };
                (Act::with(Op::Say, format!("{}#{}", r.source, r.entry)), r.reply, why)
            })
        };
        let ctx = ReadContext { asked: conv.state.frames.iter().rev().filter_map(|f| f.last_asked.clone()).collect() };
        let reading = l.understand.read(text, &ctx);
        let touches = l.track.user(&mut conv.state, &reading, mem);
        let (mut trace, mut acts, mut parts) = (Trace::default(), Vec::new(), Vec::<String>::new());
        for t in touches {
            let r = &reading.requests[t.request];
            let mut rt = RequestTrace {
                clause: r.clause.clone(),
                intent: r.intent.clone(),
                matched: r.matched.clone(),
                facts: r.facts.clone(),
                user_act: t.user_act,
                kind: ReqKind::Unknown,
                pattern: None,
                atom: None,
                alternative: None,
                steps: Vec::new(),
                text: String::new(),
                frame: None,
                recalled: None,
            };
            let act = match t.frame {
                Some(fi) => {
                    let mut last = Act::new(Op::Escalate);
                    // The active frame: the touched one, or a subframe it called. Lookups and calls are silent,
                    // and a subframe's closing act pops it: the parent resumes and decides again.
                    let mut cur = fi;
                    let mut closed = false;
                    for _ in 0..8 {
                        let mut d = l.decide.decide(&conv.state.frames[cur], mem);
                        if d.act.op == Op::Call && !d.act.arg.as_deref().map_or(false, |i| mem.intent(i).is_some()) {
                            d.note = Some("a call named no intent in the vault".into());
                            d.act = Act::new(Op::Escalate);
                        }
                        rt.kind = d.kind;
                        rt.pattern = d.pattern.clone();
                        rt.atom = d.atom.clone();
                        rt.alternative = d.alternative.clone();
                        trace.notes.extend(d.note.clone());
                        conv.used.extend(d.pattern.iter().cloned());
                        match d.act.op {
                            Op::Grant | Op::Deny | Op::Ask => conv.used.extend(d.atom.iter().cloned()),
                            Op::Offer => conv.used.extend(d.alternative.iter().cloned()),
                            Op::Say => conv.used.extend(d.act.arg.iter().cloned()),
                            _ => {}
                        }
                        l.track.agent(&mut conv.state.frames[cur], &d.act, mem);
                        rt.steps.push(StepTrace { features: d.features, rule: d.rule, act: d.act.clone() });
                        last = d.act;
                        match last.op {
                            Op::Lookup => {}
                            Op::Call => cur = l.track.call(&mut conv.state, cur, last.arg.as_deref().unwrap_or_default(), mem),
                            _ => match l.track.ret(&mut conv.state, cur, &last, mem) {
                                Some(p) => cur = p,
                                None => {
                                    closed = true;
                                    break;
                                }
                            },
                        }
                    }
                    if !closed {
                        // Out of steps while still waiting on something silent: hand over.
                        last = Act::new(Op::Escalate);
                        l.track.agent(&mut conv.state.frames[cur], &last, mem);
                        rt.steps.push(StepTrace { features: Vec::new(), rule: None, act: last.clone() });
                    }
                    rt.text = l.realize.realize(&conv.state.frames[cur], &last, mem);
                    rt.frame = Some(conv.state.frames[cur].clone());
                    last
                }
                None => {
                    // Not understood at all: a reply remembered from a taught chat may fit. An understood request is never overridden.
                    if let Some((act, reply, why)) = remembered(&r.clause, &mut trace, &mut conv.used) {
                        rt.recalled = Some(why);
                        rt.kind = ReqKind::Move;
                        rt.text = reply.clone();
                        rt.steps.push(StepTrace { features: Vec::new(), rule: None, act: act.clone() });
                        if !parts.contains(&reply) {
                            parts.push(reply);
                        }
                        acts.push(act);
                        trace.requests.push(rt);
                        continue;
                    }
                    // Else a stock reply may still fit the wording.
                    let w = words(&r.clause);
                    let best = mem
                        .moves()
                        .into_iter()
                        .filter_map(|m| {
                            let s = m.examples.iter().map(|e| { let x = words(e); let i = w.intersection(&x).count() as f64; i / (w.len() + x.len()).max(1) as f64 * 2.0 }).fold(0.0, f64::max);
                            (s >= 0.6).then_some((s, m))
                        })
                        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                    let act = match best {
                        Some((_, m)) => {
                            rt.kind = ReqKind::Move;
                            conv.used.insert(m.id.clone());
                            Act::with(Op::Say, m.id.clone())
                        }
                        None => Act::new(Op::TeachMe),
                    };
                    rt.text = l.realize.realize(&Frame::default(), &act, mem);
                    rt.steps.push(StepTrace { features: Vec::new(), rule: None, act: act.clone() });
                    act
                }
            };
            if !rt.text.is_empty() && !parts.contains(&rt.text) {
                parts.push(rt.text.clone());
            }
            acts.push(act);
            trace.requests.push(rt);
        }
        let text = if parts.is_empty() { "I don't know how to help with that yet.".to_string() } else { parts.join(" ") };
        conv.messages.push(Message { role: "agent".into(), text: text.clone(), trace: Some(trace.clone()) });
        Reply { text, acts, trace }
    }

    /// Close with a verdict: everything the conversation used moves a quarter of the way towards it.
    pub fn close(&self, conv: &mut ConvState, verdict: Verdict) -> Result<Vec<TrustChange>, String> {
        if conv.closed {
            return Err("the conversation is already closed".into());
        }
        let target = match verdict {
            Verdict::Resolved => 1.0,
            Verdict::Partial => 0.5,
            Verdict::Failed => 0.0,
        };
        let used: Vec<String> = conv.used.iter().cloned().collect();
        let changes = self.change(|vault| {
            let mut out = Vec::new();
            for id in used {
                let Some(mut note) = vault.notes.get(&id).cloned() else { continue };
                let before = note.trust();
                let after = ((before + 0.25 * (target - before)) * 1000.0).round() / 1000.0;
                note.front.insert("trust".into(), Json::from(after));
                vault.put(note)?;
                out.push(TrustChange { id, before, after });
            }
            Ok(out)
        })?;
        conv.closed = true;
        conv.verdict = Some(verdict);
        Ok(changes)
    }

    // ---------- knowledge ----------

    fn item_of(vault: &Vault, n: &Note) -> Item {
        Item {
            id: n.id.clone(),
            kind: n.kind().to_string(),
            status: n.status().to_string(),
            trust: n.trust(),
            path: n.path.clone(),
            front: n.front.clone(),
            body: n.body.clone(),
            links: n.links(),
            backlinks: vault.backlinks(&n.id),
        }
    }
    pub fn items(&self, kind: Option<&str>, status: Option<&str>, q: Option<&str>) -> Vec<ItemSummary> {
        let q = q.map(str::to_lowercase).filter(|q| !q.is_empty());
        self.vault(|v| {
            v.notes
                .values()
                .filter(|n| kind.map_or(true, |k| k.is_empty() || n.kind() == k) && status.map_or(true, |s| s.is_empty() || n.status() == s))
                .filter(|n| q.as_ref().map_or(true, |q| n.id.to_lowercase().contains(q) || n.body.to_lowercase().contains(q) || Json::Object(n.front.clone()).to_string().to_lowercase().contains(q)))
                .map(|n| ItemSummary { id: n.id.clone(), kind: n.kind().into(), status: n.status().into(), trust: n.trust(), path: n.path.clone(), title: n.title() })
                .collect()
        })
    }
    pub fn item(&self, id: &str) -> Option<Item> {
        self.vault(|v| v.notes.get(id).map(|n| Engine::item_of(v, n)))
    }
    pub fn edit(&self, id: &str, front: Map<String, Json>, body: String) -> Result<Item, String> {
        self.change(|v| {
            let path = v.notes.get(id).map(|n| n.path.clone()).ok_or_else(|| format!("no item '{id}'"))?;
            v.put(Note { id: id.to_string(), path, front, body: body.trim_end().to_string() })?;
            Ok(Engine::item_of(v, &v.notes[id]))
        })
    }
    /// Make a candidate trusted. A candidate set of phrases is merged into the note it is for.
    pub fn promote(&self, id: &str) -> Result<Item, String> {
        self.change(|v| {
            let cand = v.notes.get(id).cloned().ok_or_else(|| format!("no item '{id}'"))?;
            if cand.status() != "candidate" {
                return Err(format!("'{id}' is not a candidate"));
            }
            let link = |k: &str| cand.front.get(k).and_then(|x| x.as_str()).map(|s| s.trim_start_matches("[[").trim_end_matches("]]").to_string());
            let target = if cand.kind() == "phrases" {
                let target = link("intent").or_else(|| link("slot")).or_else(|| link("lexicon")).ok_or("the candidate names no intent or slot")?;
                let mut note = v.notes.get(&target).cloned().ok_or_else(|| format!("no note '{target}' to add the phrases to"))?;
                let phrases = cand.front.get("phrases").and_then(|p| p.as_array()).cloned().unwrap_or_default();
                match cand.front.get("as").and_then(|x| x.as_str()) {
                    Some("patterns") => crate::learn::add_patterns(&mut note, phrases),
                    Some("connectors") => crate::learn::add_connectors(&mut note, phrases),
                    _ => crate::learn::add_phrases(&mut note, cand.front.get("value").and_then(|x| x.as_str()), phrases),
                }
                v.put(note)?;
                target
            } else {
                let target = link("promotes_to").unwrap_or_else(|| id.to_string());
                let mut note = cand.clone();
                note.id = target.clone();
                note.path = format!("{}/{}.md", folder(cand.kind()), target);
                note.front.remove("promotes_to");
                note.front.insert("status".into(), "trusted".into());
                v.put(note)?;
                target
            };
            if target != id {
                v.remove(id)?;
            }
            Ok(Engine::item_of(v, &v.notes[&target]))
        })
    }
    /// Teach a reply to the last thing the user said, for when the agent has nothing of its own. It lands as a
    /// candidate chat, with the agent's previous message as its context; once promoted, a message like it
    /// draws that reply. Nothing changes before then.
    pub fn teach(&self, conv: &ConvState, reply: &str) -> Result<Item, String> {
        let squash = |t: &str| t.split_whitespace().collect::<Vec<_>>().join(" ");
        let reply = squash(reply);
        if reply.is_empty() {
            return Err("give the reply to teach".into());
        }
        let at = conv.messages.iter().rposition(|m| m.role == "user").ok_or("nothing has been said yet to teach a reply to")?;
        let message = squash(&conv.messages[at].text);
        let mut lines = Vec::new();
        if let Some(prev) = at.checked_sub(1).map(|i| &conv.messages[i]).filter(|m| m.role == "agent" && !m.text.trim().is_empty()) {
            lines.push(('A', squash(&prev.text)));
        }
        lines.push(('U', message.clone()));
        lines.push(('A', reply));
        let base = format!("taught-{}", message.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).take(3).collect::<Vec<_>>().join("-").to_lowercase());
        self.change(|v| {
            let mut id = base.clone();
            let mut n = 1;
            while v.notes.contains_key(&id) {
                n += 1;
                id = format!("{base}-{n}");
            }
            let mut front = Map::new();
            front.insert("kind".into(), "chats".into());
            front.insert("topic".into(), "taught by hand".into());
            front.insert("conversations".into(), 1.into());
            front.insert("learned_from".into(), "teach".into());
            front.insert("trust".into(), 0.8.into());
            v.put(Note { id: id.clone(), path: format!("candidates/{id}.md"), front, body: charlotte_vault::render_chats(&[lines]) })?;
            Ok(Engine::item_of(v, &v.notes[&id]))
        })
    }
    pub fn reject(&self, id: &str) -> Result<(), String> {
        self.change(|v| match v.notes.get(id).map(|n| n.status() == "candidate") {
            Some(true) => v.remove(id),
            Some(false) => Err(format!("'{id}' is not a candidate")),
            None => Err(format!("no item '{id}'")),
        })
    }
    /// Trusted items per kind, and the number of candidates.
    pub fn counts(&self) -> (BTreeMap<String, usize>, usize) {
        self.vault(|v| {
            let mut m = BTreeMap::new();
            let mut c = 0;
            for n in v.notes.values() {
                match n.status() {
                    "trusted" => *m.entry(n.kind().to_string()).or_insert(0) += 1,
                    "candidate" => c += 1,
                    _ => {}
                }
            }
            (m, c)
        })
    }
}
