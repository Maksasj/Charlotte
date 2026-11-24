//! Data types exchanged between the layers, the knowledge items read from the vault,
//! and the five layer traits. Nothing here does any work.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BTreeMap;

// ---------- values ----------

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Bool(bool),
    Num(f64),
    Str(String),
}
pub type Facts = BTreeMap<String, Value>;

impl Value {
    pub fn from_json(v: &serde_json::Value) -> Option<Value> {
        match v {
            serde_json::Value::Bool(b) => Some(Value::Bool(*b)),
            serde_json::Value::Number(n) => n.as_f64().map(Value::Num),
            serde_json::Value::String(s) => Some(Value::Str(s.clone())),
            _ => None,
        }
    }
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Num(n) => Some(*n),
            Value::Str(s) => s.trim().parse().ok(),
            Value::Bool(_) => None,
        }
    }
    pub fn text(&self) -> String {
        match self {
            Value::Bool(b) => b.to_string(),
            Value::Num(n) if n.fract() == 0.0 && n.abs() < 1e15 => format!("{}", *n as i64),
            Value::Num(n) => n.to_string(),
            Value::Str(s) => s.clone(),
        }
    }
    /// Equality as a condition sees it: numbers by value, everything else by text, ignoring case.
    pub fn same(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => a == b,
            _ => match (self.as_f64(), other.as_f64()) {
                (Some(a), Some(b)) => a == b,
                _ => self.text().eq_ignore_ascii_case(&other.text()),
            },
        }
    }
}
impl Serialize for Value {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Value::Bool(b) => s.serialize_bool(*b),
            Value::Num(n) if n.fract() == 0.0 && n.abs() < 1e15 => s.serialize_i64(*n as i64),
            Value::Num(n) => s.serialize_f64(*n),
            Value::Str(t) => s.serialize_str(t),
        }
    }
}
impl<'de> Deserialize<'de> for Value {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v = serde_json::Value::deserialize(d)?;
        Value::from_json(&v).ok_or_else(|| serde::de::Error::custom("expected true/false, a number or text"))
    }
}

// ---------- acts ----------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Op {
    Ask,
    Grant,
    Offer,
    Deny,
    Confirm,
    Lookup,
    Report,
    Instruct,
    Repeat,
    Finish,
    Escalate,
    /// Hand a part of the request to a subframe for another intent; its outcome comes back as facts.
    Call,
    Say,
    TeachMe,
}
impl Op {
    /// Operations a pattern can choose. `say` and `teach_me` are the engine's own fallbacks.
    pub const LEARNABLE: [Op; 12] = [
        Op::Ask, Op::Grant, Op::Offer, Op::Deny, Op::Confirm, Op::Lookup, Op::Report, Op::Instruct, Op::Repeat, Op::Finish, Op::Escalate, Op::Call,
    ];
    pub fn name(self) -> &'static str {
        match self {
            Op::Ask => "ask",
            Op::Grant => "grant",
            Op::Offer => "offer",
            Op::Deny => "deny",
            Op::Confirm => "confirm",
            Op::Lookup => "lookup",
            Op::Report => "report",
            Op::Instruct => "instruct",
            Op::Repeat => "repeat",
            Op::Finish => "finish",
            Op::Escalate => "escalate",
            Op::Call => "call",
            Op::Say => "say",
            Op::TeachMe => "teach_me",
        }
    }
    pub fn parse(s: &str) -> Option<Op> {
        Op::LEARNABLE.iter().chain([Op::Say, Op::TeachMe].iter()).copied().find(|o| o.name() == s)
    }
    pub fn is_decision(self) -> bool {
        matches!(self, Op::Grant | Op::Offer | Op::Deny)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Act {
    pub op: Op,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arg: Option<String>,
}
impl Act {
    pub fn new(op: Op) -> Act {
        Act { op, arg: None }
    }
    pub fn with(op: Op, arg: impl Into<String>) -> Act {
        Act { op, arg: Some(arg.into()) }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserAct {
    Request,
    Provide,
    Correct,
    Done,
    Failed,
    /// The social acts: what an intent marked `social` does in a conversation.
    Greet,
    Thank,
    Agree,
    AskBack,
    Bye,
}
impl UserAct {
    /// The acts patterns can test (`last_user=...`). The social acts open no pattern.
    pub const ALL: [UserAct; 5] = [UserAct::Request, UserAct::Provide, UserAct::Correct, UserAct::Done, UserAct::Failed];
    pub const SOCIAL: [UserAct; 5] = [UserAct::Greet, UserAct::Thank, UserAct::Agree, UserAct::AskBack, UserAct::Bye];
    pub fn name(self) -> &'static str {
        match self {
            UserAct::Request => "request",
            UserAct::Provide => "provide",
            UserAct::Correct => "correct",
            UserAct::Done => "done",
            UserAct::Failed => "failed",
            UserAct::Greet => "greet",
            UserAct::Thank => "thank",
            UserAct::Agree => "agree",
            UserAct::AskBack => "ask_back",
            UserAct::Bye => "bye",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Signal {
    Done,
    Failed,
    Yes,
    No,
}

// ---------- layer 1 output ----------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Match {
    pub phrase: String,
    pub kind: String,
    pub target: String,
}

/// One request found in a message.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Request {
    pub clause: String,
    pub intent: Option<String>,
    pub facts: Facts,
    pub signal: Option<Signal>,
    pub matched: Vec<Match>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Reading {
    pub requests: Vec<Request>,
}
/// What the reader may use from the conversation: the slots the agent is waiting for.
#[derive(Clone, Debug, Default)]
pub struct ReadContext {
    pub asked: Vec<String>,
}

// ---------- layer 2 state ----------

/// The engine's state for one request. `subject` and `object` are kept among the facts.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Frame {
    pub id: u32,
    pub intent: String,
    pub facts: Facts,
    pub agent_acts: Vec<Op>,
    pub user_acts: Vec<UserAct>,
    pub last_asked: Option<String>,
    pub standing: Option<Op>,
    pub fetched: bool,
    pub found: bool,
    pub steps_done: usize,
    /// The frame that called this one, while this one is a subframe.
    #[serde(default)]
    pub parent: Option<u32>,
    /// The subframe this frame is waiting for.
    #[serde(default)]
    pub subgoal: Option<u32>,
    /// What kind of request the last subframe was, once it has returned.
    #[serde(default)]
    pub returned_from: Option<ReqKind>,
    /// The last subframe ended in `escalate`.
    #[serde(default)]
    pub subgoal_failed: bool,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct State {
    pub frames: Vec<Frame>,
}
/// Which frame a request went to, and what the user did there.
#[derive(Clone, Debug)]
pub struct Touch {
    pub request: usize,
    pub frame: Option<usize>,
    pub user_act: Option<UserAct>,
}

// ---------- layer 3 output ----------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReqKind {
    Gated,
    Procedure,
    Lookup,
    Move,
    Unknown,
}
impl ReqKind {
    pub fn name(self) -> &'static str {
        match self {
            ReqKind::Gated => "gated",
            ReqKind::Procedure => "procedure",
            ReqKind::Lookup => "lookup",
            ReqKind::Move => "move",
            ReqKind::Unknown => "unknown",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuleTrace {
    pub index: usize,
    pub when: Vec<String>,
    #[serde(rename = "do")]
    pub act: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Decision {
    pub kind: ReqKind,
    pub pattern: Option<String>,
    pub atom: Option<String>,
    pub alternative: Option<String>,
    pub features: Vec<String>,
    pub rule: Option<RuleTrace>,
    pub act: Act,
    pub note: Option<String>,
}

// ---------- trace and reply ----------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StepTrace {
    pub features: Vec<String>,
    pub rule: Option<RuleTrace>,
    pub act: Act,
}
/// Where a remembered reply came from, and how close the match was.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecalledTrace {
    pub source: String,
    pub entry: usize,
    /// The remembered message that the new one was matched with.
    pub message: String,
    pub score: f64,
    /// The score a match has to reach.
    pub threshold: f64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestTrace {
    pub clause: String,
    pub intent: Option<String>,
    pub matched: Vec<Match>,
    pub facts: Facts,
    pub user_act: Option<UserAct>,
    pub kind: ReqKind,
    pub pattern: Option<String>,
    pub atom: Option<String>,
    pub alternative: Option<String>,
    pub steps: Vec<StepTrace>,
    pub text: String,
    pub frame: Option<Frame>,
    /// Set when the reply was reused from a taught chat.
    pub recalled: Option<RecalledTrace>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Trace {
    pub requests: Vec<RequestTrace>,
    pub notes: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Reply {
    pub text: String,
    pub acts: Vec<Act>,
    pub trace: Trace,
}

// ---------- knowledge items ----------

fn one() -> f64 {
    1.0
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PhraseDef {
    Text(String),
    Full {
        text: String,
        #[serde(default = "one")]
        weight: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        support: Option<u32>,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        learned: bool,
    },
}
impl PhraseDef {
    pub fn text(&self) -> &str {
        match self {
            PhraseDef::Text(t) => t,
            PhraseDef::Full { text, .. } => text,
        }
    }
    pub fn weight(&self) -> f64 {
        match self {
            PhraseDef::Text(_) => 1.0,
            PhraseDef::Full { weight, .. } => *weight,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Intent {
    #[serde(skip)]
    pub id: String,
    #[serde(default)]
    pub social: bool,
    #[serde(default)]
    pub lookup: bool,
    #[serde(default)]
    pub priority: i32,
    /// What saying this does, for a social intent: greet, thank, agree, ask_back or bye.
    #[serde(default)]
    pub act: Option<UserAct>,
    #[serde(default)]
    pub phrases: Vec<PhraseDef>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SlotType {
    Enum,
    Bool,
    Number,
    Id,
    #[default]
    Text,
}
/// A way of saying a number or an identifier: "{n} weeks ago" with scale 7, "yesterday" with value 1,
/// "order {id}" which also sets `object: order`.
#[derive(Clone, Debug, Deserialize)]
pub struct SlotPattern {
    pub text: String,
    #[serde(default = "one")]
    pub scale: f64,
    #[serde(default)]
    pub value: Option<Value>,
    #[serde(default)]
    pub sets: Facts,
}
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Slot {
    #[serde(skip)]
    pub id: String,
    #[serde(default, rename = "type")]
    pub ty: SlotType,
    #[serde(default)]
    pub ask: String,
    #[serde(default)]
    pub values: BTreeMap<String, Vec<PhraseDef>>,
    #[serde(default)]
    pub patterns: Vec<SlotPattern>,
    #[serde(default)]
    pub yes: Option<Value>,
    #[serde(default)]
    pub no: Option<Value>,
}
impl Slot {
    /// The value a key under `values:` stands for.
    pub fn value_of(&self, key: &str) -> Value {
        match self.ty {
            SlotType::Bool => Value::Bool(key == "true" || key == "yes"),
            SlotType::Number => key.parse().map(Value::Num).unwrap_or_else(|_| Value::Str(key.to_string())),
            _ => Value::Str(key.to_string()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CmpOp {
    #[serde(rename = "<=")]
    Le,
    #[serde(rename = ">=")]
    Ge,
    #[serde(rename = "<")]
    Lt,
    #[serde(rename = ">")]
    Gt,
    #[serde(rename = "==")]
    Eq,
    #[serde(rename = "!=")]
    Ne,
}
impl CmpOp {
    pub fn words(self) -> &'static str {
        match self {
            CmpOp::Le => "at most",
            CmpOp::Ge => "at least",
            CmpOp::Lt => "under",
            CmpOp::Gt => "over",
            CmpOp::Eq => "to be",
            CmpOp::Ne => "anything but",
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Source {
    #[default]
    User,
    Record,
}
#[derive(Clone, Debug, Deserialize)]
pub struct Condition {
    pub slot: String,
    pub op: CmpOp,
    pub value: Value,
    #[serde(default)]
    pub source: Source,
}
impl Condition {
    pub fn holds(&self, got: &Value) -> bool {
        let num = |f: fn(f64, f64) -> bool| matches!((got.as_f64(), self.value.as_f64()), (Some(a), Some(b)) if f(a, b));
        match self.op {
            CmpOp::Le => num(|a, b| a <= b),
            CmpOp::Ge => num(|a, b| a >= b),
            CmpOp::Lt => num(|a, b| a < b),
            CmpOp::Gt => num(|a, b| a > b),
            CmpOp::Eq => got.same(&self.value),
            CmpOp::Ne => !got.same(&self.value),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Atom {
    #[serde(skip)]
    pub id: String,
    pub intent: String,
    #[serde(default)]
    pub object: Option<String>,
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub priority: i32,
    #[serde(default)]
    pub result: String,
    #[serde(default)]
    pub conditions: Vec<Condition>,
    #[serde(default)]
    pub deny: Option<String>,
    #[serde(skip)]
    pub payload: String,
}
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Procedure {
    #[serde(skip)]
    pub id: String,
    pub intent: String,
    #[serde(default)]
    pub steps: Vec<String>,
}
#[derive(Clone, Debug, Default, Deserialize)]
pub struct RecordRow {
    pub id: String,
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub note: String,
    #[serde(default)]
    pub facts: Facts,
}
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Records {
    #[serde(skip)]
    pub id: String,
    #[serde(default)]
    pub object: Option<String>,
    #[serde(default)]
    pub rows: Vec<RecordRow>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rule {
    pub when: Vec<String>,
    #[serde(rename = "do")]
    pub act: String,
}
#[derive(Clone, Debug, Deserialize)]
pub struct Pattern {
    #[serde(skip)]
    pub id: String,
    pub applies_to: ReqKind,
    #[serde(default)]
    pub rules: Vec<Rule>,
    pub otherwise: String,
    #[serde(default)]
    pub support: u32,
    #[serde(default)]
    pub validated: f64,
    #[serde(default)]
    pub seen_with: Vec<String>,
}
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Move {
    #[serde(skip)]
    pub id: String,
    #[serde(default)]
    pub intent: Option<String>,
    #[serde(default)]
    pub examples: Vec<String>,
    #[serde(skip)]
    pub reply: String,
}
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Lexicon {
    #[serde(default)]
    pub connectors: Vec<String>,
    #[serde(default)]
    pub negators: Vec<String>,
    #[serde(default)]
    pub stopwords: Vec<String>,
    #[serde(default)]
    pub signals: BTreeMap<String, Vec<String>>,
}

/// One exchange remembered from a taught chat: the message, what came just before it, and the reply.
#[derive(Clone, Debug)]
pub struct ReplyCase {
    /// The note it is in, and the entry number there.
    pub source: String,
    pub entry: usize,
    pub previous: Option<String>,
    pub message: String,
    pub reply: String,
}
/// A remembered reply found for a new message.
#[derive(Clone, Debug)]
pub struct Recalled {
    pub source: String,
    pub entry: usize,
    pub message: String,
    pub reply: String,
    pub score: f64,
}

// ---------- the five layers ----------

/// Layer 5. Everything the other layers may ask of the knowledge store. Only trusted items are returned.
pub trait Memory: Send + Sync {
    fn intents(&self) -> Vec<&Intent>;
    fn intent(&self, id: &str) -> Option<&Intent>;
    fn slots(&self) -> Vec<&Slot>;
    fn slot(&self, id: &str) -> Option<&Slot>;
    fn lexicon(&self) -> &Lexicon;
    fn atoms(&self, intent: &str) -> Vec<&Atom>;
    fn procedure(&self, intent: &str) -> Option<&Procedure>;
    fn record(&self, subject: &str) -> Option<&RecordRow>;
    fn records(&self) -> Vec<&Records>;
    fn pattern(&self, kind: ReqKind) -> Option<&Pattern>;
    fn moves(&self) -> Vec<&Move>;
    /// Exchanges from taught chats.
    fn replies(&self) -> &[ReplyCase];
}

/// Layer 1. Text to requests.
pub trait Understand: Send + Sync {
    fn read(&self, text: &str, ctx: &ReadContext) -> Reading;
}
/// Layer 2. Keeps one frame per request and applies what the user and the agent do to it.
pub trait Track: Send + Sync {
    fn user(&self, state: &mut State, reading: &Reading, mem: &dyn Memory) -> Vec<Touch>;
    fn agent(&self, frame: &mut Frame, act: &Act, mem: &dyn Memory);
    /// A `call` by the frame at `parent`: a subframe for `intent` is pushed, seeded with the subject and
    /// object, and its index returned. The parent waits for it.
    fn call(&self, state: &mut State, parent: usize, intent: &str, mem: &dyn Memory) -> usize;
    /// A subframe's closing act (`report`, `finish`, `escalate`) pops it: the parent resumes with the
    /// outcome landed as facts. Returns the parent's index, or None when `child` is not a subframe
    /// or `act` does not close it.
    fn ret(&self, state: &mut State, child: usize, act: &Act, mem: &dyn Memory) -> Option<usize>;
}
/// Layer 3. Chooses the next act for a frame. No wording.
pub trait Decide: Send + Sync {
    fn decide(&self, frame: &Frame, mem: &dyn Memory) -> Decision;
}
/// Beside the layers: finds a remembered reply for a message nothing else answers.
/// `previous` is what the agent said just before, if anything.
pub trait Recall: Send + Sync {
    fn recall(&self, text: &str, previous: Option<&str>) -> Option<Recalled>;
}
/// Layer 4. Words for an act.
pub trait Realize: Send + Sync {
    fn realize(&self, frame: &Frame, act: &Act, mem: &dyn Memory) -> String;
}
