//! The leak deny filter (Design 4.0 section 10.5, decision D4.8): benchmark answers, every
//! intermediate proof atom and every withheld piece must never reach the agent. It is applied
//! at five points: P1 teacher prompts, P2 teacher replies (statements withheld before the agent
//! sees them), P3 the agent's questions (dropped, never sent), P4 archive imports, and the P5
//! audit of everything delivered. One delivered hit invalidates the run.

use std::collections::BTreeSet;

#[derive(Debug, Clone, Default)]
pub struct Deny {
    atoms: BTreeSet<String>,
}

/// Canonical form for comparison: no spaces, no trailing period, no leading `not`.
pub fn key(s: &str) -> String {
    let t: String = s.trim().trim_end_matches('.').chars().filter(|c| !c.is_whitespace()).collect();
    t.strip_prefix("not").map(|r| r.to_string()).unwrap_or(t)
}

impl Deny {
    pub fn new<I: IntoIterator<Item = String>>(atoms: I) -> Deny {
        Deny { atoms: atoms.into_iter().map(|a| key(&a)).filter(|a| !a.is_empty()).collect() }
    }

    pub fn load(path: &std::path::Path) -> Deny {
        Deny::new(std::fs::read_to_string(path).unwrap_or_default().lines().filter(|l| !l.trim_start().starts_with('%')).map(String::from))
    }

    pub fn len(&self) -> usize {
        self.atoms.len()
    }

    pub fn is_empty(&self) -> bool {
        self.atoms.is_empty()
    }

    /// Does a statement (or any atom inside a longer text, like a prompt) hit the list?
    pub fn hits(&self, text: &str) -> Vec<String> {
        let mut out = Vec::new();
        for part in text.split(['\n', ';']) {
            let k = key(part);
            if self.atoms.contains(&k) {
                out.push(k);
                continue;
            }
            // atoms embedded in a longer line
            for a in &self.atoms {
                if part.chars().filter(|c| !c.is_whitespace()).collect::<String>().contains(a.as_str()) {
                    out.push(a.clone());
                }
            }
        }
        out.sort();
        out.dedup();
        out
    }

    /// P2: the deliverable statements of a teacher reply, and the withheld ones.
    pub fn filter_reply(&self, reply: &str) -> (Vec<String>, Vec<String>) {
        let mut keep = Vec::new();
        let mut withheld = Vec::new();
        for l in reply.lines().map(str::trim).filter(|l| !l.is_empty()) {
            if self.hits(l).is_empty() { keep.push(l.to_string()) } else { withheld.push(l.to_string()) }
        }
        (keep, withheld)
    }
}
