use crate::types::ToolInfo;

pub fn meta_tools() -> Vec<ToolInfo> {
    vec![
        ToolInfo {
            name: "generalize_episodes".into(),
            description: "Meta-tool: anti-unify similar episodes into a candidate general pattern."
                .into(),
            meta: true,
        },
        ToolInfo {
            name: "chunk_last_chain".into(),
            description: "Meta-tool: compile the last multi-step chain into one composite pattern."
                .into(),
            meta: true,
        },
        ToolInfo {
            name: "induce_from_correction".into(),
            description: "Meta-tool: inspect memory after a user correction to induce a candidate rule or ask for missing facts."
                .into(),
            meta: true,
        },
        ToolInfo {
            name: "approve_pattern".into(),
            description: "Meta-tool: promote a pending candidate pattern into active status."
                .into(),
            meta: true,
        },
    ]
}
