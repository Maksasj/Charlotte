//! Reloading the vault into a running engine without losing what it learned.
//!
//! Everything that came from the vault (provenance / source starting with
//! `vault:`) is replaced by a fresh load; everything else — facts written in
//! conversation, taught or induced rules, notes, chat memory, messages — is
//! kept. Feedback on a vault rule (trust, support) carries over when the rule
//! still exists under the same name in the same file.

use super::Engine;
use crate::types::{Episode, Fact, Pattern};
use crate::vault::load_vault;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::path::Path;

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultReload {
    pub facts: usize,
    pub patterns: usize,
    pub added_rules: Vec<String>,
    pub removed_rules: Vec<String>,
    pub facts_delta: i64,
}

fn from_vault(provenance: &str) -> bool {
    provenance.starts_with("vault:")
}

impl Engine {
    /// Replace the vault's facts, rules and episodes with a fresh load of `dir`,
    /// keeping all runtime knowledge. On a parse error nothing changes.
    pub fn reload_vault(&mut self, dir: &Path) -> Result<VaultReload, String> {
        let fresh = load_vault(dir)?;

        let old_rules: HashMap<(String, String), &Pattern> = self
            .state
            .patterns
            .iter()
            .filter(|p| from_vault(&p.provenance))
            .map(|p| ((p.provenance.clone(), p.name.clone()), p))
            .collect();
        let old_vault_facts = self
            .state
            .facts
            .iter()
            .filter(|f| from_vault(&f.provenance))
            .count();

        let mut patterns: Vec<Pattern> = fresh.patterns;
        for p in &mut patterns {
            if let Some(old) = old_rules.get(&(p.provenance.clone(), p.name.clone())) {
                p.trust = old.trust;
                p.support = old.support;
            }
        }
        let fresh_names: HashSet<(String, String)> = patterns
            .iter()
            .map(|p| (p.provenance.clone(), p.name.clone()))
            .collect();
        let added_rules: Vec<String> = patterns
            .iter()
            .filter(|p| !old_rules.contains_key(&(p.provenance.clone(), p.name.clone())))
            .map(|p| p.name.clone())
            .collect();
        let removed_rules: Vec<String> = old_rules
            .keys()
            .filter(|k| !fresh_names.contains(*k))
            .map(|(_, name)| name.clone())
            .collect();

        let summary = VaultReload {
            facts: fresh.facts.len(),
            patterns: patterns.len(),
            added_rules,
            removed_rules,
            facts_delta: fresh.facts.len() as i64 - old_vault_facts as i64,
        };

        // Runtime items keep their order, after the vault's (so they stay "newer").
        let runtime_facts: Vec<Fact> = self
            .state
            .facts
            .drain(..)
            .filter(|f| !from_vault(&f.provenance))
            .collect();
        let runtime_patterns: Vec<Pattern> = self
            .state
            .patterns
            .drain(..)
            .filter(|p| !from_vault(&p.provenance))
            .collect();
        let runtime_episodes: Vec<Episode> = self
            .state
            .episodes
            .drain(..)
            .filter(|e| !from_vault(&e.source))
            .collect();

        self.state.next_id = self.state.next_id.max(fresh.next_id);
        let taken: HashSet<String> = fresh
            .facts
            .iter()
            .map(|f| f.id.clone())
            .chain(patterns.iter().map(|p| p.id.clone()))
            .chain(fresh.episodes.iter().map(|e| e.id.clone()))
            .collect();

        self.state.facts = fresh.facts;
        self.state.patterns = patterns;
        self.state.episodes = fresh.episodes;

        // A runtime id that now clashes with a vault id gets a new one.
        for mut f in runtime_facts {
            if taken.contains(&f.id) {
                f.id = self.alloc_id("f");
            }
            self.state.facts.push(f);
        }
        for mut p in runtime_patterns {
            if taken.contains(&p.id) {
                p.id = self.alloc_id("p");
            }
            self.state.patterns.push(p);
        }
        for mut e in runtime_episodes {
            if taken.contains(&e.id) {
                e.id = self.alloc_id("e");
            }
            self.state.episodes.push(e);
        }
        Ok(summary)
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::Engine;
    use crate::vault::default_vault_dir;

    #[test]
    fn reload_keeps_runtime_knowledge() {
        let dir = default_vault_dir();
        let mut eng = Engine::from_vault_dir(&dir).unwrap();
        eng.turn("my name is Maksim");
        eng.turn("Oscar is my friend");
        let rules_before = eng.state.patterns.len();

        let summary = eng.reload_vault(&dir).unwrap();
        assert!(summary.added_rules.is_empty() && summary.removed_rules.is_empty());
        assert_eq!(eng.state.patterns.len(), rules_before);

        let name = eng.turn("what is my name");
        assert!(name.reply.contains("Maksim"), "{}", name.reply);
        let friend = eng.turn("who is my friend");
        assert!(friend.reply.contains("Oscar"), "{}", friend.reply);
        // No duplicate ids after the merge.
        let mut ids: Vec<&str> = eng.state.facts.iter().map(|f| f.id.as_str()).collect();
        let n = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), n);
    }
}
