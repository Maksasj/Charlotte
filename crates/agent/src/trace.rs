//! The turn trace: one record per message, written by every stage, read by
//! the learner, the evaluation and the training dashboard. With it, any
//! failure can be placed at one stage.

use serde::{Deserialize, Serialize};

/// One part of a message (a sentence, later a clause) and what became of it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PartTrace {
    pub text: String,
    /// The dialogue acts it was translated into, in canonical form.
    pub acts: Vec<String>,
    /// Which translator tier produced the acts: `pattern`, `model`, `oracle`.
    pub tier: String,
    pub understood: bool,
}

/// A change to the vault made during the turn (from the knowledge base journal).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ChangeTrace {
    pub action: String,
    pub subject: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TurnTrace {
    pub session: String,
    /// 1-based turn number within the session.
    pub turn: usize,
    /// ISO time of the turn (the agent's clock).
    pub at: String,
    pub speaker: String,
    pub source: String,
    pub message: String,
    pub parser: String,
    /// Stage 1-2: the message split into parts, each translated.
    pub parts: Vec<PartTrace>,
    /// Stage 3-4: the move chosen for each handled act (`acknowledge`, `answer`, ...).
    pub moves: Vec<String>,
    /// A curiosity question appended to the reply, if any (its goal key).
    pub question: Option<String>,
    /// Stage 6: the reply and the atoms it claims.
    pub reply: String,
    pub claims: Vec<String>,
    /// What the turn changed in the vault.
    pub changes: Vec<ChangeTrace>,
    /// Milliseconds translating (outside the vault lock) and thinking (inside it).
    pub translate_ms: f64,
    pub think_ms: f64,
    /// Model calls made by the translator this turn.
    pub model_calls: u64,
    /// Full evaluations of the knowledge base this turn.
    pub evaluations: u64,
    pub understood: bool,
    /// Problems noticed this turn (quarantined rules, failed saves, ...).
    pub errors: Vec<String>,
}

impl TurnTrace {
    pub fn facts_added(&self) -> Vec<&str> {
        self.changes.iter().filter(|c| c.action == "add" || c.action == "add_past").map(|c| c.subject.as_str()).collect()
    }

    pub fn rules_added(&self) -> Vec<&str> {
        self.changes.iter().filter(|c| c.action == "add_rule").map(|c| c.subject.as_str()).collect()
    }
}
