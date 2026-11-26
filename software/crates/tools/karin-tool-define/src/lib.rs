//! Karin tool: look up a concept definition.

use karin_tool::*;
use std::collections::BTreeMap;

pub struct DefineTool;

impl WorldTool for DefineTool {
    fn info(&self) -> ToolInfo {
        ToolInfo {
            name: "define".into(),
            description: "Look up a concept definition from long-term knowledge.".into(),
            meta: false,
            package: "karin-tool-define".into(),
            args: vec![arg(
                "topic",
                true,
                "concept, matched against definition(topic=...) facts",
            )],
            produces: vec![
                output(
                    "definition_result",
                    &["topic", "text"],
                    Scope::Working,
                    "when a definition exists",
                ),
                output(
                    "definition_unknown",
                    &["topic"],
                    Scope::Working,
                    "when there is none",
                ),
            ],
        }
    }

    fn run(&self, args: &BTreeMap<String, String>, facts: &[Fact]) -> ToolOutcome {
        let topic = args.get("topic").cloned().unwrap_or_default();
        let mut fact_args = BTreeMap::from([("topic".into(), topic.clone())]);

        if let Some(found) = find_fact(facts, "definition", "topic", &topic) {
            let text = found.args.get("text").cloned().unwrap_or_default();
            fact_args.insert("text".into(), text);
            ToolOutcome {
                facts: vec![working_fact("definition_result", fact_args, "tool:define")],
                summary: format!("define({topic}) -> found"),
            }
        } else {
            ToolOutcome {
                facts: vec![working_fact("definition_unknown", fact_args, "tool:define")],
                summary: format!("define({topic}) -> unknown"),
            }
        }
    }
}
