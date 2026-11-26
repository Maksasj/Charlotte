//! Karin tool: gather matching facts into one list.

use karin_tool::*;
use std::collections::BTreeMap;

use std::collections::HashSet;

pub struct CollectTool;

const RESERVED_KEYS: [&str; 4] = ["rel", "field", "tag", "sep"];

impl WorldTool for CollectTool {
    fn info(&self) -> ToolInfo {
        ToolInfo {
            name: "collect".into(),
            description: "Collect all matching facts for a relation into a joined list.".into(),
            meta: false,
            package: "karin-tool-collect".into(),
            args: vec![
                arg("rel", true, "relation to read, e.g. rel or weather_city"),
                arg(
                    "field",
                    false,
                    "argument whose values are listed (default: value)",
                ),
                arg(
                    "tag",
                    false,
                    "label copied to the result, so reply rules can tell lists apart",
                ),
                arg("sep", false, "separator between items (default: \", \")"),
                arg(
                    "<any other key>",
                    false,
                    "filter: only facts whose argument equals this value",
                ),
            ],
            produces: vec![
                output(
                    "collected",
                    &["tag", "count", "list"],
                    Scope::Working,
                    "when at least one fact matches",
                ),
                output(
                    "collected_empty",
                    &["tag"],
                    Scope::Working,
                    "when nothing matches",
                ),
            ],
        }
    }

    fn run(&self, args: &BTreeMap<String, String>, facts: &[Fact]) -> ToolOutcome {
        let target_rel = args.get("rel").map(String::as_str).unwrap_or("");
        let field = args.get("field").map(String::as_str).unwrap_or("value");
        let tag = args
            .get("tag")
            .cloned()
            .unwrap_or_else(|| target_rel.into());
        let sep = args.get("sep").map(String::as_str).unwrap_or(", ");

        let filters: Vec<(&str, String)> = args
            .iter()
            .filter(|(k, _)| !RESERVED_KEYS.contains(&k.as_str()))
            .map(|(k, v)| (k.as_str(), normalize(v)))
            .collect();

        let mut seen = HashSet::new();
        let mut items = Vec::new();

        for fact in facts {
            if fact.rel != target_rel {
                continue;
            }
            let matches_all = filters.iter().all(|(k, expected)| {
                fact.args
                    .get(*k)
                    .map(|actual| normalize(actual) == *expected)
                    .unwrap_or(false)
            });
            if !matches_all {
                continue;
            }
            if let Some(val) = fact.args.get(field) {
                let norm = normalize(val);
                if seen.insert(norm) {
                    items.push(val.clone());
                }
            }
        }

        if items.is_empty() {
            let fact_args = BTreeMap::from([("tag".into(), tag.clone())]);
            ToolOutcome {
                facts: vec![working_fact("collected_empty", fact_args, "tool:collect")],
                summary: format!("collect({target_rel}.{field}) -> 0 items"),
            }
        } else {
            let list = items.join(sep);
            let count = items.len();
            let fact_args = BTreeMap::from([
                ("tag".into(), tag.clone()),
                ("count".into(), count.to_string()),
                ("list".into(), list),
            ]);
            ToolOutcome {
                facts: vec![working_fact("collected", fact_args, "tool:collect")],
                summary: format!("collect({target_rel}.{field}) -> {count} items"),
            }
        }
    }
}
