mod calc;
mod collect;
mod define;
mod math;
mod meta;
mod notes;
mod weather;

use crate::types::{Fact, Scope, ToolInfo};
use crate::unify::normalize;
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

pub trait WorldTool: Sync {
    fn info(&self) -> ToolInfo;
    fn run(&self, args: &BTreeMap<String, String>, facts: &[Fact]) -> ToolOutcome;
}

fn registered_tools() -> &'static [&'static dyn WorldTool] {
    &[
        &calc::CalcTool,
        &weather::WeatherTool,
        &define::DefineTool,
        &notes::NoteSaveTool,
        &notes::NoteListTool,
        &collect::CollectTool,
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

pub(crate) fn working_fact(rel: &str, args: BTreeMap<String, String>, provenance: &str) -> NewFact {
    NewFact {
        rel: rel.into(),
        args,
        scope: Scope::Working,
        provenance: provenance.into(),
    }
}

pub(crate) fn find_fact<'a>(
    facts: &'a [Fact],
    rel: &str,
    key: &str,
    value: &str,
) -> Option<&'a Fact> {
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
