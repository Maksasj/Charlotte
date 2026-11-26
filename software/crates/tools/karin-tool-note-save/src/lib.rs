//! Karin tool: save a note to long-term memory.

use karin_tool::*;
use std::collections::BTreeMap;

pub struct NoteSaveTool;

impl WorldTool for NoteSaveTool {
    fn info(&self) -> ToolInfo {
        ToolInfo {
            name: "note_save".into(),
            description: "Save a personal note into long-term memory.".into(),
            meta: false,
            package: "karin-tool-note-save".into(),
            args: vec![arg("text", true, "the note text")],
            produces: vec![
                output(
                    "note",
                    &["text"],
                    Scope::LongTerm,
                    "always (the stored note)",
                ),
                output(
                    "note_saved",
                    &["text"],
                    Scope::Working,
                    "always (for the reply)",
                ),
            ],
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
