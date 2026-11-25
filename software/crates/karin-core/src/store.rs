use crate::types::{AgentState, ChatMessage};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn load_state(path: &Path) -> Result<Option<AgentState>, String> {
    if !path.exists() {
        return Ok(None);
    }
    let raw = fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let state: AgentState =
        serde_json::from_str(&raw).map_err(|e| format!("parse {}: {e}", path.display()))?;
    Ok(Some(state))
}

pub fn save_state(path: &Path, state: &AgentState) -> Result<(), String> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent).map_err(|e| format!("mkdir {}: {e}", parent.display()))?;
    }
    let json = serde_json::to_string_pretty(state).map_err(|e| format!("serialize: {e}"))?;
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, json).map_err(|e| format!("write tmp: {e}"))?;
    fs::rename(&tmp, path).map_err(|e| format!("rename tmp: {e}"))?;
    Ok(())
}

pub fn next_chat_id(chats_dir: &Path) -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let base = format!("chat-{secs}");
    if !chats_dir.join(format!("{base}.md")).exists() {
        return base;
    }
    for n in 2.. {
        let candidate = format!("{base}-{n}");
        if !chats_dir.join(format!("{candidate}.md")).exists() {
            return candidate;
        }
    }
    unreachable!()
}

pub fn save_chat(
    chats_dir: &Path,
    chat_id: &str,
    messages: &[ChatMessage],
) -> Result<Option<PathBuf>, String> {
    if messages.is_empty() {
        return Ok(None);
    }
    fs::create_dir_all(chats_dir).map_err(|e| format!("mkdir {}: {e}", chats_dir.display()))?;

    let mut out = format!("# Chat {chat_id}\n");
    let mut last_turn = None;

    for msg in messages {
        if last_turn != Some(msg.turn) {
            out.push_str(&format!("\n## Turn {}\n\n", msg.turn));
            last_turn = Some(msg.turn);
        }
        let speaker = match msg.role.as_str() {
            "user" => "User",
            "agent" | "assistant" => "Karin",
            other => other,
        };
        out.push_str(&format!("**{speaker}:** {}\n", msg.text));
        if let Some(trace) = &msg.trace {
            if trace.impasse {
                out.push_str("- Impasse: true\n");
            }
            if !trace.fired_pattern_ids.is_empty() {
                out.push_str(&format!(
                    "- Fired: {}\n",
                    trace.fired_pattern_ids.join(", ")
                ));
            }
        }
        if let Some(fb) = &msg.feedback {
            out.push_str(&format!("- Feedback: {fb}\n"));
        }
    }

    let path = chats_dir.join(format!("{chat_id}.md"));
    let tmp = chats_dir.join(format!("{chat_id}.md.tmp"));
    fs::write(&tmp, out).map_err(|e| format!("write {}: {e}", tmp.display()))?;
    fs::rename(&tmp, &path).map_err(|e| format!("rename {}: {e}", path.display()))?;
    Ok(Some(path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saves_chat_markdown_and_rotates_unique_ids() {
        let dir = std::env::temp_dir().join(format!(
            "karin-chats-test-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let id1 = next_chat_id(&dir);
        assert!(save_chat(&dir, &id1, &[]).unwrap().is_none());

        let msgs = vec![
            ChatMessage {
                id: "m1".into(),
                role: "user".into(),
                text: "hello".into(),
                turn: 1,
                trace: None,
                feedback: None,
            },
            ChatMessage {
                id: "m2".into(),
                role: "assistant".into(),
                text: "Hi there!".into(),
                turn: 1,
                trace: None,
                feedback: Some("positive".into()),
            },
        ];

        let saved = save_chat(&dir, &id1, &msgs).unwrap().unwrap();
        let body = fs::read_to_string(&saved).unwrap();
        assert!(body.contains("**User:** hello"));
        assert!(body.contains("**Karin:** Hi there!"));
        assert!(body.contains("- Feedback: positive"));

        let id2 = next_chat_id(&dir);
        assert_ne!(id1, id2);
        let _ = fs::remove_dir_all(&dir);
    }
}
