//! Tool registry. Each world tool is its own package under `crates/tools/`
//! (`karin-tool-<name>`); meta tools act on the engine itself and live here.

mod meta;

pub use karin_tool::{NewFact, ToolOutcome, WorldTool};
use crate::types::{Fact, ToolInfo};
use std::collections::BTreeMap;

fn registered_tools() -> &'static [&'static dyn WorldTool] {
    &[
        &karin_tool_calc::CalcTool,
        &karin_tool_weather::WeatherTool,
        &karin_tool_define::DefineTool,
        &karin_tool_note_save::NoteSaveTool,
        &karin_tool_note_list::NoteListTool,
        &karin_tool_collect::CollectTool,
    ]
}

pub fn list_tools() -> Vec<ToolInfo> {
    let mut tools: Vec<ToolInfo> = registered_tools().iter().map(|tool| tool.info()).collect();
    tools.extend(meta::meta_tools());
    tools
}

pub fn run_world_tool(
    name: &str,
    args: &BTreeMap<String, String>,
    facts: &[Fact],
) -> Option<ToolOutcome> {
    registered_tools()
        .iter()
        .find(|tool| tool.info().name == name)
        .map(|tool| tool.run(args, facts))
}
