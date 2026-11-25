mod math;

use crate::types::{Fact, Scope, ToolInfo};
use crate::unify::normalize;
use math::{eval_expr, format_number};
use std::collections::BTreeMap;

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

pub fn list_tools() -> Vec<ToolInfo> {
    vec![
        tool_info(
            "calc",
            "Evaluate a safe arithmetic expression (+, -, *, /, parentheses).",
            false,
        ),
        tool_info(
            "weather",
            "Look up weather facts for a city from long-term knowledge.",
            false,
        ),
        tool_info(
            "define",
            "Look up a concept definition from long-term knowledge.",
            false,
        ),
        tool_info(
            "note_save",
            "Save a personal note into long-term memory.",
            false,
        ),
        tool_info(
            "note_list",
            "List all saved notes from long-term memory.",
            false,
        ),
        tool_info(
            "generalize_episodes",
            "Meta-tool: anti-unify similar episodes into a candidate general pattern.",
            true,
        ),
        tool_info(
            "chunk_last_chain",
            "Meta-tool: compile the last multi-step chain into one composite pattern.",
            true,
        ),
    ]
}

pub fn run_world_tool(
    name: &str,
    args: &BTreeMap<String, String>,
    facts: &[Fact],
) -> Option<ToolOutcome> {
    match name {
        "calc" => Some(run_calc(args)),
        "weather" => Some(run_weather(args, facts)),
        "define" => Some(run_define(args, facts)),
        "note_save" => Some(run_note_save(args)),
        "note_list" => Some(run_note_list(facts)),
        _ => None,
    }
}

fn tool_info(name: &str, description: &str, meta: bool) -> ToolInfo {
    ToolInfo {
        name: name.into(),
        description: description.into(),
        meta,
    }
}

fn working_fact(rel: &str, args: BTreeMap<String, String>, provenance: &str) -> NewFact {
    NewFact {
        rel: rel.into(),
        args,
        scope: Scope::Working,
        provenance: provenance.into(),
    }
}

fn find_fact<'a>(facts: &'a [Fact], rel: &str, key: &str, value: &str) -> Option<&'a Fact> {
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

fn run_calc(args: &BTreeMap<String, String>) -> ToolOutcome {
    let expr = args.get("expr").cloned().unwrap_or_default();
    let mut fact_args = BTreeMap::from([("expr".into(), expr.clone())]);

    match eval_expr(&expr) {
        Ok(value) => {
            let formatted = format_number(value);
            fact_args.insert("value".into(), formatted.clone());
            ToolOutcome {
                facts: vec![working_fact("calc_result", fact_args, "tool:calc")],
                summary: format!("calc({expr}) = {formatted}"),
            }
        }
        Err(err) => {
            fact_args.insert("error".into(), err.clone());
            ToolOutcome {
                facts: vec![working_fact("calc_error", fact_args, "tool:calc")],
                summary: format!("calc({expr}) error: {err}"),
            }
        }
    }
}

fn run_weather(args: &BTreeMap<String, String>, facts: &[Fact]) -> ToolOutcome {
    let city = args.get("city").cloned().unwrap_or_default();
    let mut fact_args = BTreeMap::from([("city".into(), city.clone())]);

    if let Some(found) = find_fact(facts, "weather_city", "city", &city) {
        let condition = found
            .args
            .get("condition")
            .cloned()
            .unwrap_or_else(|| "clear".into());
        let temp = found
            .args
            .get("temp")
            .cloned()
            .unwrap_or_else(|| "20°C".into());
        fact_args.insert("condition".into(), condition.clone());
        fact_args.insert("temp".into(), temp.clone());
        ToolOutcome {
            facts: vec![working_fact("weather_report", fact_args, "tool:weather")],
            summary: format!("weather({city}) -> {condition}, {temp}"),
        }
    } else {
        ToolOutcome {
            facts: vec![working_fact("weather_unknown", fact_args, "tool:weather")],
            summary: format!("weather({city}) -> unknown"),
        }
    }
}

fn run_define(args: &BTreeMap<String, String>, facts: &[Fact]) -> ToolOutcome {
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

fn run_note_save(args: &BTreeMap<String, String>) -> ToolOutcome {
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

fn run_note_list(facts: &[Fact]) -> ToolOutcome {
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
