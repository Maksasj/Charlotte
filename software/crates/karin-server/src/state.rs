use karin_core::{
    Action, AgentState, Condition, Engine, PatternKind, PatternStatus, Scope, ToolInfo,
    store::save_state,
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf, sync::Mutex};

pub struct AppState {
    pub engine: Mutex<Engine>,
    pub state_path: PathBuf,
    pub vault_dir: PathBuf,
}

impl AppState {
    pub fn snapshot(&self) -> AgentState {
        self.engine.lock().unwrap().state.clone()
    }

    pub fn mutate<T>(&self, action: impl FnOnce(&mut Engine) -> T) -> (T, AgentState) {
        let mut engine = self.engine.lock().unwrap();
        let output = action(&mut engine);
        let snapshot = engine.state.clone();
        drop(engine);
        self.persist(&snapshot);
        (output, snapshot)
    }

    pub fn try_mutate(&self, action: impl FnOnce(&mut Engine) -> bool) -> Option<AgentState> {
        let mut engine = self.engine.lock().unwrap();
        if !action(&mut engine) {
            return None;
        }
        let snapshot = engine.state.clone();
        drop(engine);
        self.persist(&snapshot);
        Some(snapshot)
    }

    fn persist(&self, state: &AgentState) {
        if let Err(err) = save_state(&self.state_path, state) {
            eprintln!("warning: failed to save state: {err}");
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StateResponse {
    pub state: AgentState,
    pub tools: Vec<ToolInfo>,
}

#[derive(Deserialize)]
pub struct ChatReq {
    pub input: String,
}

#[derive(Deserialize)]
pub struct TeachReq {
    pub input: String,
    pub reply: String,
    #[serde(default = "default_true")]
    pub promote: bool,
}

#[derive(Deserialize)]
pub struct FeedbackReq {
    pub turn: usize,
    pub positive: bool,
}

#[derive(Deserialize)]
pub struct AddFactReq {
    pub rel: String,
    #[serde(default)]
    pub args: BTreeMap<String, String>,
    #[serde(default = "default_long_term")]
    pub scope: Scope,
    #[serde(default = "default_manual")]
    pub provenance: String,
}

#[derive(Deserialize)]
pub struct AddPatternReq {
    pub name: String,
    pub when: Vec<Condition>,
    pub then: Vec<Action>,
    #[serde(default = "default_active")]
    pub status: PatternStatus,
    #[serde(default = "default_reply_kind")]
    pub kind: PatternKind,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MetaReq {
    #[serde(default)]
    pub auto_promote: bool,
}

fn default_true() -> bool {
    true
}

fn default_long_term() -> Scope {
    Scope::LongTerm
}

fn default_manual() -> String {
    "manual".into()
}

fn default_active() -> PatternStatus {
    PatternStatus::Active
}

fn default_reply_kind() -> PatternKind {
    PatternKind::Reply
}
