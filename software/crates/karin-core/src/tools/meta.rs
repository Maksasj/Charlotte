use karin_tool::{Scope, ToolInfo, arg, output};

pub fn meta_tools() -> Vec<ToolInfo> {
    vec![
        ToolInfo {
            name: "generalize_episodes".into(),
            description: "Meta-tool: anti-unify similar episodes into a candidate general pattern."
                .into(),
            meta: true,
            package: "karin-core".into(),
            args: vec![],
            produces: vec![output("meta_result", &["summary", "count"], Scope::Working, "always")],
        },
        ToolInfo {
            name: "chunk_last_chain".into(),
            description: "Meta-tool: compile the last multi-step chain into one composite pattern."
                .into(),
            meta: true,
            package: "karin-core".into(),
            args: vec![],
            produces: vec![output("meta_result", &["summary"], Scope::Working, "always")],
        },
        ToolInfo {
            name: "induce_from_correction".into(),
            description: "Meta-tool: inspect memory after a user correction to induce a candidate rule or ask for missing facts."
                .into(),
            meta: true,
            package: "karin-core".into(),
            args: vec![arg("item", true, "the thing the user corrected about"), arg("topic", true, "its category, e.g. drink")],
            produces: vec![output("correction_outcome", &["status", "item", "topic", "pattern_id"], Scope::Working, "always"), output("dialogue_pending_approval", &["pattern_id", "item", "topic"], Scope::Chat, "when a rule is proposed"), output("dialogue_confirm", &["kind", "item", "topic"], Scope::Chat, "when facts are missing")],
        },
        ToolInfo {
            name: "merge_entities".into(),
            description: "Engine tool: everything known about one entity now belongs to another (e.g. a guest who turns out to be a known person)."
                .into(),
            meta: true,
            package: "karin-core".into(),
            args: vec![
                arg("from", true, "entity id whose facts move"),
                arg("into", true, "entity id that receives them"),
            ],
            produces: vec![output("entities_merged", &["from", "into", "count"], Scope::Working, "always")],
        },
        ToolInfo {
            name: "approve_pattern".into(),
            description: "Meta-tool: promote a pending candidate pattern into active status."
                .into(),
            meta: true,
            package: "karin-core".into(),
            args: vec![arg("id", true, "candidate pattern id")],
            produces: vec![output("approval_result", &["ok", "id"], Scope::Working, "always")],
        },
    ]
}
