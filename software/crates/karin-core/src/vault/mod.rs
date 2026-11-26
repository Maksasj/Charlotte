mod parser;
mod syntax;

use crate::types::{AgentState, Episode, Fact, Pattern};
use parser::ModuleParser;
use std::fs;
use std::path::{Path, PathBuf};

/// Parse one rule condition (exposed for unit tests elsewhere in the crate).
#[cfg(test)]
pub(crate) fn parse_condition_for_tests(raw: &str) -> Result<crate::types::Condition, String> {
    syntax::parse_condition(raw)
}

pub fn default_vault_dir() -> PathBuf {
    if let Ok(env_dir) = std::env::var("VAULT_CORE_DIR") {
        return PathBuf::from(env_dir);
    }

    let candidates = [
        PathBuf::from("vault/core"),
        PathBuf::from("../vault/core"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../vault/core"),
    ];

    candidates
        .into_iter()
        .find(|dir| dir.is_dir())
        .unwrap_or_else(|| PathBuf::from("vault/core"))
}

pub fn load_vault(dir: &Path) -> Result<AgentState, String> {
    let mut builder = VaultBuilder::default();

    for (root, prefix) in vault_roots(dir) {
        for path in list_markdown_files(&root)? {
            let module = format!("{prefix}{}", module_id(&root, &path));
            let raw = fs::read_to_string(&path)
                .map_err(|err| format!("read {}: {err}", path.display()))?;
            builder
                .load_module(&raw, &module)
                .map_err(|err| format!("{}: {err}", path.display()))?;
        }
    }

    Ok(builder.finish())
}

/// A cheap fingerprint of every vault Markdown file (path, size, modified time),
/// over the same folders `load_vault` reads: core, `generated-*`, `.episodes`.
/// It changes when a file is edited, added or removed, or a folder appears.
pub fn vault_fingerprint(dir: &Path) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut h = DefaultHasher::new();
    for (root, prefix) in vault_roots(dir) {
        prefix.hash(&mut h);
        for path in list_markdown_files(&root).unwrap_or_default() {
            path.hash(&mut h);
            if let Ok(meta) = fs::metadata(&path) {
                meta.len().hash(&mut h);
                if let Ok(modified) = meta.modified() {
                    modified.hash(&mut h);
                }
            }
        }
    }
    h.finish()
}

pub fn initial_state() -> AgentState {
    let dir = default_vault_dir();
    load_vault(&dir).unwrap_or_else(|err| {
        panic!(
            "Failed to load Markdown vault from {}: {err}",
            dir.display()
        )
    })
}

pub(crate) struct VaultBuilder {
    pub facts: Vec<Fact>,
    pub patterns: Vec<Pattern>,
    pub episodes: Vec<Episode>,
    next_fact: usize,
    next_pattern: usize,
    next_episode: usize,
}

impl Default for VaultBuilder {
    fn default() -> Self {
        Self {
            facts: Vec::new(),
            patterns: Vec::new(),
            episodes: Vec::new(),
            next_fact: 1,
            next_pattern: 1,
            next_episode: 1,
        }
    }
}

impl VaultBuilder {
    pub fn alloc_fact_id(&mut self) -> String {
        let id = format!("f{}", self.next_fact);
        self.next_fact += 1;
        id
    }

    pub fn alloc_pattern_id(&mut self) -> String {
        let id = format!("p{}", self.next_pattern);
        self.next_pattern += 1;
        id
    }

    pub fn alloc_episode_id(&mut self) -> String {
        let id = format!("e{}", self.next_episode);
        self.next_episode += 1;
        id
    }

    pub fn load_module(&mut self, raw: &str, module: &str) -> Result<(), String> {
        ModuleParser::new(self, module).parse(raw)
    }

    fn finish(self) -> AgentState {
        let max_num = self.next_fact.max(self.next_pattern).max(self.next_episode);

        AgentState {
            turn: 0,
            facts: self.facts,
            patterns: self.patterns,
            episodes: self.episodes,
            chains: Vec::new(),
            messages: Vec::new(),
            next_id: (max_num + 1).max(100),
        }
    }
}

fn vault_roots(core_dir: &Path) -> Vec<(PathBuf, String)> {
    let mut roots = vec![(core_dir.to_path_buf(), String::new())];
    if let Some(parent) = core_dir.parent() {
        let mut generated: Vec<(String, PathBuf)> = fs::read_dir(parent)
            .ok()
            .into_iter()
            .flatten()
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .filter(|path| path.is_dir() && path != core_dir)
            .filter_map(|path| {
                let name = path.file_name()?.to_str()?.to_string();
                name.starts_with("generated-").then_some((name, path))
            })
            .collect();
        generated.sort_by(|a, b| a.0.cmp(&b.0));
        for (name, path) in generated {
            roots.push((path, format!("{name}/")));
        }

        let episodes_dir = parent.join(".episodes");
        if episodes_dir.is_dir() && episodes_dir != core_dir {
            roots.push((episodes_dir, ".episodes/".to_string()));
        }
    }
    roots
}

fn module_id(root: &Path, path: &Path) -> String {
    let rel = path.strip_prefix(root).unwrap_or(path);
    let id = rel.with_extension("").to_string_lossy().replace('\\', "/");
    if id.is_empty() { "vault".into() } else { id }
}

fn list_markdown_files(dir: &Path) -> Result<Vec<PathBuf>, String> {
    if !dir.is_dir() {
        return Ok(Vec::new());
    }
    let mut files = Vec::new();
    collect_markdown_files(dir, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_markdown_files(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let mut entries: Vec<PathBuf> = fs::read_dir(dir)
        .map_err(|err| format!("readdir {}: {err}", dir.display()))?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();
    entries.sort();

    for path in entries {
        if path.is_dir() {
            collect_markdown_files(&path, out)?;
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("md") {
            out.push(path);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_compact_dsl_vault_core() {
        let state = initial_state();
        assert!(state.facts.len() >= 30);
        assert!(state.patterns.len() >= 50);
        assert_eq!(state.episodes.len(), 4);
    }

    #[test]
    fn ignores_prose_outside_code_fences() {
        let md = r#"
# Notes

This prose has fake_fact(a="1") and "fake" => "episode" and should be ignored!

rule ignored-rule
when: "hi"
then: => "ignored"

```
real_fact(k="v")

rule real-rule
when: "ping"
then: => "pong"
```

More comments outside fences.

```karin
"a b" => "c b"
```
"#;
        let mut builder = VaultBuilder::default();
        builder.load_module(md, "test").unwrap();
        assert_eq!(builder.facts.len(), 1);
        assert_eq!(builder.facts[0].rel, "real_fact");
        assert_eq!(builder.patterns.len(), 1);
        assert_eq!(builder.patterns[0].name, "real-rule");
        assert_eq!(builder.episodes.len(), 1);
        assert_eq!(builder.episodes[0].input, "a b");
    }
}
