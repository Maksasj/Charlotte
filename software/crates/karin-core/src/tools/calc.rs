use super::{ToolOutcome, WorldTool, math::eval_expr, math::format_number, working_fact};
use crate::types::{Fact, ToolInfo};
use std::collections::BTreeMap;

pub struct CalcTool;

impl WorldTool for CalcTool {
    fn info(&self) -> ToolInfo {
        ToolInfo {
            name: "calc".into(),
            description: "Evaluate a safe arithmetic expression (+, -, *, /, parentheses).".into(),
            meta: false,
        }
    }

    fn run(&self, args: &BTreeMap<String, String>, _facts: &[Fact]) -> ToolOutcome {
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
}
