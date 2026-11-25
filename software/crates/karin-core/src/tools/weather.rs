use super::{ToolOutcome, WorldTool, find_fact, working_fact};
use crate::types::{Fact, ToolInfo};
use std::collections::BTreeMap;

pub struct WeatherTool;

impl WorldTool for WeatherTool {
    fn info(&self) -> ToolInfo {
        ToolInfo {
            name: "weather".into(),
            description: "Look up weather facts for a city from long-term knowledge.".into(),
            meta: false,
        }
    }

    fn run(&self, args: &BTreeMap<String, String>, facts: &[Fact]) -> ToolOutcome {
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
}
