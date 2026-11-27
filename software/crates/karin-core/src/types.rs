use serde::{Deserialize, Serialize};

/// Shared with the tool packages.
pub use karin_tool::{Fact, Scope, ToolArg, ToolInfo, ToolOutput};
use std::collections::BTreeMap;



#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Condition {
    Text {
        template: String,
    },
    Contains {
        phrase: String,
    },
    Fact {
        rel: String,
        #[serde(default)]
        args: BTreeMap<String, String>,
    },
    NotFact {
        rel: String,
        #[serde(default)]
        args: BTreeMap<String, String>,
    },
    NotEq {
        left: String,
        right: String,
    },
    /// `words($x) <= 2`: how many words a bound value has.
    WordCount {
        var: String,
        op: String,
        n: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Action {
    Assert {
        rel: String,
        #[serde(default)]
        args: BTreeMap<String, String>,
        scope: Scope,
    },
    Retract {
        rel: String,
        #[serde(default)]
        args: Option<BTreeMap<String, String>>,
    },
    Tool {
        name: String,
        #[serde(default)]
        args: BTreeMap<String, String>,
    },
    Reply {
        template: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatternStatus {
    Active,
    Candidate,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatternKind {
    Language,
    Reasoning,
    Reply,
    Composite,
    Induced,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pattern {
    pub id: String,
    pub name: String,
    pub status: PatternStatus,
    pub when: Vec<Condition>,
    pub then: Vec<Action>,
    pub trust: f64,
    pub support: usize,
    pub provenance: String,
    pub kind: PatternKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Episode {
    pub id: String,
    pub input: String,
    pub reply: String,
    pub source: String,
    #[serde(default)]
    pub generalized: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompetingMatch {
    pub pattern_id: String,
    pub name: String,
    pub score: f64,
    pub bindings: BTreeMap<String, String>,
    /// What the score is made of ("text", "facts", "reply", "trust", ...).
    #[serde(default)]
    pub parts: Vec<ScorePart>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScorePart {
    pub label: String,
    pub value: f64,
}

/// One way the language layer tried to read the input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttemptTrace {
    /// "whole", "clause", "nested" (inside a wrapper), "ellipsis" or "exact".
    pub kind: String,
    pub text: String,
    pub answered: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepTrace {
    pub step: usize,
    pub competing: Vec<CompetingMatch>,
    pub winner_id: String,
    pub winner_name: String,
    pub bindings: BTreeMap<String, String>,
    pub effects: Vec<String>,
    /// The canonical sentence this step matched against.
    #[serde(default)]
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnTrace {
    pub turn: usize,
    pub input: String,
    pub reply: String,
    pub impasse: bool,
    pub steps: Vec<StepTrace>,
    pub fired_pattern_ids: Vec<String>,
    pub chunked_pattern_id: Option<String>,
    /// What the language layer turned the input into: one entry per clause that was understood.
    #[serde(default)]
    pub understood: Vec<String>,
    /// Clauses of the input that no pattern could handle.
    #[serde(default)]
    pub missed: Vec<String>,
    /// Every reading that was tried, in order.
    #[serde(default)]
    pub attempts: Vec<AttemptTrace>,
    /// Chat memory (facts with chat scope) as it stood after this turn.
    #[serde(default)]
    pub chat_memory: Vec<Fact>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    pub id: String,
    pub role: String,
    pub text: String,
    pub turn: usize,
    pub trace: Option<TurnTrace>,
    pub feedback: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainRecord {
    pub id: String,
    pub turn: usize,
    pub input: String,
    pub pattern_ids: Vec<String>,
    pub bindings: BTreeMap<String, String>,
    #[serde(default)]
    pub chunked: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentState {
    pub turn: usize,
    pub facts: Vec<Fact>,
    pub patterns: Vec<Pattern>,
    pub episodes: Vec<Episode>,
    pub chains: Vec<ChainRecord>,
    pub messages: Vec<ChatMessage>,
    pub next_id: usize,
}

