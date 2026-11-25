use super::{NewFact, ToolOutcome, WorldTool, working_fact};
use crate::types::{Fact, Scope, ToolInfo};
use std::collections::BTreeMap;

pub struct NoteSaveTool;
pub struct NoteListTool;

impl WorldTool for NoteSaveTool {
    fn info(&self) -> ToolInfo {
        ToolInfo {
            name: "note_save".into(),
            description: "Save a personal note into long-term memory.".into(),
            meta: false,
        }
    }

    fn run(&self, args: &BTreeMap<String, String>, _facts: &[Fact]) -> ToolOutcome {
        let text = args.get("text").cloned().unwrap_or_default();
        let fact_args = BTreeMap::from([("text".into(), text.clone())]);

        ToolOutcome {
            facts: vec![
                NewFact {
                    rel: "note".into(),
                    args: fact_args.clone(),
                    scope: Scope::LongTerm,
                    provenance: "tool:note_save".into(),
                },
                working_fact("note_saved", fact_args, "tool:note_save"),
            ],
            summary: format!("note_save(\"{text}\")"),
        }
    }
}

impl WorldTool for NoteListTool {
    fn info(&self) -> ToolInfo {
        ToolInfo {
            name: "note_list".into(),
            description: "List all saved notes from long-term memory.".into(),
            meta: false,
        }
    }

    fn run(&self, _args: &BTreeMap<String, String>, facts: &[Fact]) -> ToolOutcome {
        let notes: Vec<String> = facts
            .iter()
            .filter(|fact| fact.rel == "note")
            .filter_map(|fact| fact.args.get("text").cloned())
            .collect();

        let listing = if notes.is_empty() {
            "You don't have any saved notes yet.".to_string()
        } else {
            notes
                .iter()
                .enumerate()
                .map(|(idx, note)| format!("{}. {}", idx + 1, note))
                .collect::<Vec<_>>()
                .join(" | ")
        };

        let fact_args = BTreeMap::from([
            ("list".into(), listing),
            ("count".into(), notes.len().to_string()),
        ]);

        ToolOutcome {
            facts: vec![working_fact("notes_listing", fact_args, "tool:note_list")],
            summary: format!("note_list() -> {} notes", notes.len()),
        }
    }
}
