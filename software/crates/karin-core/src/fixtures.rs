use crate::types::{AgentState, Episode, Fact, Pattern};
use std::fs;
use std::path::{Path, PathBuf};

/// Default path to `vault/fixtures` relative to this crate (`software/crates/karin-core`).
pub fn default_fixtures_dir() -> PathBuf {
    if let Ok(env_dir) = std::env::var("FIXTURES_DIR") {
        return PathBuf::from(env_dir);
    }
    let cwd_candidates = [
        PathBuf::from("vault/fixtures"),
        PathBuf::from("../vault/fixtures"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../vault/fixtures"),
    ];
    for c in cwd_candidates {
        if c.join("patterns.json").exists() {
            return c;
        }
    }
    PathBuf::from("vault/fixtures")
}

/// Load seed facts, patterns, and episodes from `vault/fixtures/{facts,patterns,episodes}.json`.
pub fn load_fixtures(dir: &Path) -> Result<AgentState, String> {
    let facts: Vec<Fact> = read_json(&dir.join("facts.json"))?;
    let patterns: Vec<Pattern> = read_json(&dir.join("patterns.json"))?;
    let episodes: Vec<Episode> = read_json(&dir.join("episodes.json"))?;

    let max_num = facts
        .iter()
        .map(|f| numeric_suffix(&f.id))
        .chain(patterns.iter().map(|p| numeric_suffix(&p.id)))
        .chain(episodes.iter().map(|e| numeric_suffix(&e.id)))
        .max()
        .unwrap_or(0);

    Ok(AgentState {
        turn: 0,
        facts,
        patterns,
        episodes,
        chains: Vec::new(),
        messages: Vec::new(),
        next_id: (max_num + 1).max(100),
    })
}

pub fn initial_state() -> AgentState {
    let dir = default_fixtures_dir();
    load_fixtures(&dir).unwrap_or_else(|err| {
        panic!(
            "Failed to load fixtures from {}: {err}",
            dir.display()
        )
    })
}

fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, String> {
    let raw = fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    serde_json::from_str(&raw).map_err(|e| format!("parse {}: {e}", path.display()))
}

fn numeric_suffix(id: &str) -> usize {
    let digits: String = id.chars().skip_while(|c| !c.is_ascii_digit()).collect();
    digits.parse().unwrap_or(0)
}
