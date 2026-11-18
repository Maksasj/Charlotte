//! The agent's vault: facts with provenance (schema section 7) and stored cases.
//! Corrections never delete: a fact is marked retracted and the new one is added.

use crate::fact::Fact;
use crate::procedure::Procedure;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    Teach,
    Dataset,
    Derived,
    Seed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provenance {
    pub source: Source,
    pub session: String,
    pub turn: u32,
    /// "learned" or "fallback"
    pub parser: String,
    pub confidence: f64,
    pub uses: u32,
    pub confirmed: u32,
    pub retracted_by: Option<String>,
}

impl Provenance {
    pub fn new(source: Source, session: &str, turn: u32, parser: &str, confidence: f64) -> Self {
        Provenance { source, session: session.into(), turn, parser: parser.into(), confidence, uses: 0, confirmed: 0, retracted_by: None }
    }
}

/// A stored adaptation case: a situation and the change it caused, for one task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Case {
    pub task: String,
    pub situation: Vec<Fact>,
    pub difference: Vec<Fact>,
    pub source: Source,
    pub session: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Vault {
    pub facts: Vec<(Fact, Provenance)>,
    pub cases: Vec<Case>,
}

impl Vault {
    pub fn add(&mut self, f: Fact, p: Provenance) -> bool {
        if self.live().any(|g| *g == f) {
            return false;
        }
        self.facts.push((f, p));
        true
    }

    /// Retract every live fact matching `pred` and `args[..key]`, recording the correction.
    pub fn retract(&mut self, pred: &str, key: &[&str], by: &str) -> usize {
        let mut n = 0;
        for (f, p) in self.facts.iter_mut() {
            if p.retracted_by.is_none() && f.pred == pred && key.iter().enumerate().all(|(i, k)| f.arg(i) == *k) {
                p.retracted_by = Some(by.to_string());
                n += 1;
            }
        }
        n
    }

    pub fn live(&self) -> impl Iterator<Item = &Fact> {
        self.facts.iter().filter(|(_, p)| p.retracted_by.is_none()).map(|(f, _)| f)
    }

    pub fn live_facts(&self) -> Vec<Fact> {
        self.live().cloned().collect()
    }

    pub fn tasks(&self) -> Vec<String> {
        let mut t: Vec<String> = self.live().filter(|f| f.pred == "task").map(|f| f.arg(0).to_string()).collect();
        t.sort();
        t.dedup();
        t
    }

    pub fn procedure(&self, task: &str) -> Procedure {
        let facts: Vec<Fact> = self.live().cloned().collect();
        Procedure::from_facts(task, &facts)
    }

    pub fn add_case(&mut self, c: Case) {
        self.cases.push(c);
    }

    pub fn save(&self, path: &Path) -> Result<(), String> {
        if let Some(d) = path.parent() {
            std::fs::create_dir_all(d).map_err(|e| e.to_string())?;
        }
        std::fs::write(path, serde_json::to_string(self).map_err(|e| e.to_string())?).map_err(|e| e.to_string())
    }

    pub fn load(path: &Path) -> Result<Vault, String> {
        let t = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_json::from_str(&t).map_err(|e| e.to_string())
    }

    /// Count of live facts by source, for reports.
    pub fn by_source(&self) -> BTreeMap<String, usize> {
        let mut m = BTreeMap::new();
        for (_, p) in self.facts.iter().filter(|(_, p)| p.retracted_by.is_none()) {
            *m.entry(format!("{:?}", p.source).to_lowercase()).or_insert(0) += 1;
        }
        m
    }
}
