//! Karin tool: list saved notes.

use karin_tool::*;
use std::collections::BTreeMap;

pub struct NoteListTool;

impl WorldTool for NoteListTool {
    fn info(&self) -> ToolInfo {
        ToolInfo {
            name: "note_list".into(),
            description: "List all saved notes from long-term memory.".into(),
            meta: false,
            package: "karin-tool-note-list".into(),
            args: vec![],
            produces: vec![output(
                "notes_listing",
                &["list", "count"],
                Scope::Working,
                "always (list may be empty)",
            )],
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
