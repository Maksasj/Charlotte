//! Shared API for Karin's tools.
//!
//! A tool is called from a rule with `@name(key=value, ...)`, reads the current
//! facts, and answers by adding facts that later rules react to. Each tool lives
//! in its own package (`karin-tool-<name>`) and implements [`WorldTool`];
//! `karin-core` registers them.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    /// Lives for one turn.
    Working,
    /// Lives for one chat: survives turns, cleared by `new_chat`, never saved to the vault.
    Chat,
    /// Lives forever (persisted).
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

/// What a tool is and how to call it, shown in the UI's Tools view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolInfo {
    pub name: String,
    pub description: String,
    /// Meta tools act on Karin's own rules (learning); world tools act on facts.
    pub meta: bool,
    /// The Cargo package the tool lives in.
    #[serde(default)]
    pub package: String,
    /// Arguments a rule passes in `@name(key=value, ...)`.
    #[serde(default)]
    pub args: Vec<ToolArg>,
    /// Facts the tool adds, which later rules can react to.
    #[serde(default)]
    pub produces: Vec<ToolOutput>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolArg {
    pub name: String,
    pub required: bool,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolOutput {
    pub rel: String,
    pub args: Vec<String>,
    pub scope: Scope,
    /// When the tool produces it ("found", "on error", ...).
    pub when: String,
}

/// Shorthands for describing tools.
pub fn arg(name: &str, required: bool, description: &str) -> ToolArg {
    ToolArg {
        name: name.into(),
        required,
        description: description.into(),
    }
}

pub fn output(rel: &str, args: &[&str], scope: Scope, when: &str) -> ToolOutput {
    ToolOutput {
        rel: rel.into(),
        args: args.iter().map(|a| a.to_string()).collect(),
        scope,
        when: when.into(),
    }
}

pub struct NewFact {
    pub rel: String,
    pub args: BTreeMap<String, String>,
    pub scope: Scope,
    pub provenance: String,
}

pub struct ToolOutcome {
    pub facts: Vec<NewFact>,
    pub summary: String,
}

pub trait WorldTool: Sync {
    fn info(&self) -> ToolInfo;
    fn run(&self, args: &BTreeMap<String, String>, facts: &[Fact]) -> ToolOutcome;
}

pub fn working_fact(rel: &str, args: BTreeMap<String, String>, provenance: &str) -> NewFact {
    NewFact {
        rel: rel.into(),
        args,
        scope: Scope::Working,
        provenance: provenance.into(),
    }
}

pub fn find_fact<'a>(facts: &'a [Fact], rel: &str, key: &str, value: &str) -> Option<&'a Fact> {
    let target = normalize(value);
    facts.iter().rev().find(|fact| {
        fact.rel == rel
            && fact
                .args
                .get(key)
                .map(|actual| normalize(actual) == target)
                .unwrap_or(false)
    })
}

pub fn normalize(s: &str) -> String {
    let trimmed = s.trim();
    let mut out = String::with_capacity(trimmed.len());
    let mut prev_space = false;
    for ch in trimmed.chars() {
        let lower = ch.to_ascii_lowercase();
        if lower.is_whitespace() {
            if !prev_space && !out.is_empty() {
                out.push(' ');
                prev_space = true;
            }
        } else {
            out.push(lower);
            prev_space = false;
        }
    }
    out.trim_end_matches(['?', '!', '.', ',', ';'])
        .trim()
        .to_string()
}
