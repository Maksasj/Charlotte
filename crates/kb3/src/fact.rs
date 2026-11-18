//! Facts of schema v3 (`research/DESIGN_3_schema.md`): `pred(arg, arg, ...)`.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Predicate names and arities of schema v3. These are the formal language, not English.
pub const SCHEMA: &[(&str, usize)] = &[
    ("task", 1), ("step", 2), ("goal", 2), ("domain", 2),
    ("needs", 2), ("optional_need", 2), ("serves", 2), ("takes_time", 4),
    ("action", 2), ("object_of", 2), ("uses", 2), ("role", 3), ("into", 2), ("onto", 2), ("from", 2),
    ("with_part", 2), ("setting", 4), ("amount", 4), ("until", 2), ("repeat", 2),
    ("before", 2), ("overlaps", 2), ("optional", 1), ("alternative", 2),
    ("purpose", 2), ("achieves", 2), ("requires_state", 2), ("condition", 2), ("warning", 2),
    ("avoid", 3), ("check", 2),
    ("alias", 2), ("task_alias", 2),
    // situations
    ("lacks", 2), ("has", 2), ("wants", 2), ("excludes", 2), ("time_limit", 3), ("count", 3),
    // differences
    ("swap", 3), ("drop", 1), ("add_step", 2), ("change", 4), ("extra_need", 1), ("no_need", 1), ("cannot", 1),
];

pub fn arity(pred: &str) -> Option<usize> {
    SCHEMA.iter().find(|(p, _)| *p == pred).map(|(_, n)| *n)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Fact {
    pub pred: String,
    pub args: Vec<String>,
}

impl Fact {
    pub fn new(pred: &str, args: &[&str]) -> Self {
        Fact { pred: pred.to_string(), args: args.iter().map(|a| a.to_string()).collect() }
    }

    pub fn arg(&self, i: usize) -> &str {
        self.args.get(i).map(String::as_str).unwrap_or("")
    }

    /// Parse one Datalog line `pred(a, b).`; quoted arguments may hold commas.
    pub fn parse(line: &str) -> Result<Fact, String> {
        let t = line.trim();
        let t = t.strip_suffix('.').ok_or_else(|| format!("missing '.': {t}"))?;
        let open = t.find('(').ok_or_else(|| format!("missing '(': {t}"))?;
        let body = t[open + 1..].strip_suffix(')').ok_or_else(|| format!("missing ')': {t}"))?;
        let pred = t[..open].trim().to_string();
        if pred.is_empty() || !pred.chars().all(|c| c.is_ascii_lowercase() || c == '_') {
            return Err(format!("bad predicate: {t}"));
        }
        let mut args = Vec::new();
        let mut cur = String::new();
        let mut quoted = false;
        for ch in body.chars() {
            match ch {
                '"' => {
                    quoted = !quoted;
                    cur.push(ch);
                }
                ',' if !quoted => {
                    args.push(cur.trim().to_string());
                    cur.clear();
                }
                _ => cur.push(ch),
            }
        }
        if !cur.trim().is_empty() {
            args.push(cur.trim().to_string());
        }
        Ok(Fact { pred, args })
    }

    /// Check predicate and arity against the schema.
    pub fn valid(&self) -> Result<(), String> {
        match arity(&self.pred) {
            None => Err(format!("unknown predicate {}", self.pred)),
            Some(n) if n != self.args.len() => Err(format!("{} needs {n} args, got {}", self.pred, self.args.len())),
            _ => Ok(()),
        }
    }
}

impl fmt::Display for Fact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({}).", self.pred, self.args.join(", "))
    }
}

/// Unquote an alias text argument.
pub fn unquote(s: &str) -> &str {
    s.trim().trim_matches('"')
}
