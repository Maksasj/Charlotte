use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    Working,
    LongTerm,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fact {
    pub id: String,
    pub rel: String,
    pub args: BTreeMap<String, String>,
    pub scope: Scope,
    pub provenance: String,
    pub turn: usize,
}

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolInfo {
    pub name: String,
    pub description: String,
    pub meta: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfTestResult {
    pub name: String,
    pub passed: bool,
    pub detail: String,
}
