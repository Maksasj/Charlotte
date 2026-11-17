//! The oracle parser (Design 2.0, evaluation): it "translates" by looking up
//! gold parses, so a run measures everything after understanding on its own.
//! Comparing a run with the oracle and with the real translator shows how
//! much of a failure is understanding and how much is the rest.
//!
//! Gold file: JSON lines `{"text": "...", "acts": ["greet:", "tell: ..."]}`.
//! A message with no gold entry is `unknown` and counted as a miss.

use crate::llm::from_canonical;
use crate::{Context, Intent, Parser};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Deserialize)]
struct GoldLine {
    text: String,
    acts: Vec<String>,
}

#[derive(Debug, Default, Clone)]
pub struct OracleParser {
    gold: HashMap<String, Vec<Intent>>,
    /// Messages asked for that had no gold parse.
    pub misses: Vec<String>,
}

fn key(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

impl OracleParser {
    /// Read a gold file. Every act must be a valid canonical line.
    pub fn load(path: &Path) -> Result<Self, String> {
        let src = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let mut o = OracleParser::default();
        for (n, l) in src.lines().enumerate().filter(|(_, l)| !l.trim().is_empty()) {
            let g: GoldLine = serde_json::from_str(l).map_err(|e| format!("{}:{}: {e}", path.display(), n + 1))?;
            o.insert(&g.text, &g.acts).map_err(|e| format!("{}:{}: {e}", path.display(), n + 1))?;
        }
        Ok(o)
    }

    pub fn insert(&mut self, text: &str, acts: &[String]) -> Result<(), String> {
        let intents = acts.iter().map(|a| from_canonical(a).ok_or_else(|| format!("not a canonical line: {a}"))).collect::<Result<Vec<_>, _>>()?;
        self.gold.insert(key(text), intents);
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.gold.len()
    }

    pub fn is_empty(&self) -> bool {
        self.gold.is_empty()
    }
}

impl Parser for OracleParser {
    fn parse(&mut self, text: &str, ctx: &Context) -> Intent {
        self.parse_many(text, ctx).into_iter().next().unwrap_or(Intent::Unknown)
    }

    fn parse_many(&mut self, text: &str, _ctx: &Context) -> Vec<Intent> {
        match self.gold.get(&key(text)) {
            Some(v) if !v.is_empty() => v.clone(),
            _ => {
                self.misses.push(text.to_string());
                vec![Intent::Unknown]
            }
        }
    }

    fn name(&self) -> &str {
        "oracle"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_and_misses() {
        let mut o = OracleParser::default();
        o.insert("Hi!  Anna likes tea.", &["greet:".into(), "tell: likes(anna, tea)".into()]).unwrap();
        assert!(o.insert("x", &["not canonical".into()]).is_err());
        let ctx = Context::default();
        let v = o.parse_many("Hi! Anna likes tea.", &ctx);
        assert_eq!(v.iter().map(|i| i.canonical()).collect::<Vec<_>>(), vec!["greet:", "tell: likes(anna, tea)"]);
        assert_eq!(o.parse_many("Something else.", &ctx), vec![Intent::Unknown]);
        assert_eq!(o.misses, vec!["Something else."]);
    }
}
