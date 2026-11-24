//! The regression set: rewordings the reader must understand, and real held-out exchanges the engine
//! answers without having seen them. Run with `charlotte regress FILE`; one JSON case per line.
//!
//! ```text
//! {"id":"r1","kind":"reword","say":"I'd like to return my order","intent":"refund","known_fail":false}
//! {"id":"e1","kind":"exchange","source":"dailydialog-test","history":[["user","..."],["agent","..."]],"say":"...","reply":"the real reply"}
//! ```
//!
//! Exchanges are scored against the reply a person gave. Every remembered exchange whose message is a
//! case's message is left out of the memory, so the answer cannot be looked up. Abstaining is neutral;
//! a wrong answer counts against the score.

use crate::engine::{default_layers, Engine, Message};
use crate::recall::{normalize, ReplyMemory};
use charlotte_core::*;
use charlotte_vault::Vault;
use serde::Deserialize;
use std::collections::{BTreeMap, HashSet};
use std::fmt;

/// An answer counts as good when it shares at least this much (word F1) with the real reply.
pub const GOOD: f64 = 0.2;

#[derive(Debug, Deserialize)]
pub struct Case {
    pub id: String,
    pub kind: String,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub history: Vec<(String, String)>,
    pub say: String,
    #[serde(default)]
    pub intent: Option<String>,
    #[serde(default)]
    pub reply: Option<String>,
    /// A rewording that is known not to work yet: reported, but not counted as a regression.
    #[serde(default)]
    pub known_fail: bool,
}

pub fn parse(text: &str) -> Result<Vec<Case>, String> {
    text.lines().enumerate().filter(|(_, l)| !l.trim().is_empty()).map(|(i, l)| serde_json::from_str(l).map_err(|e| format!("line {}: {e}", i + 1))).collect()
}

#[derive(Default, Debug)]
pub struct Tally {
    pub good: usize,
    pub wrong: usize,
}

#[derive(Default, Debug)]
pub struct Outcome {
    pub reword_pass: usize,
    pub reword_fail: Vec<String>,
    pub reword_known: Vec<String>,
    pub exchanges: usize,
    pub abstained: usize,
    /// By how the reply was made: "recalled" or "stock".
    pub answered: BTreeMap<&'static str, Tally>,
}

impl Outcome {
    /// Good minus wrong answers over all exchanges; abstaining scores nothing.
    pub fn score(&self) -> f64 {
        if self.exchanges == 0 {
            return 0.0;
        }
        let (g, w) = self.answered.values().fold((0, 0), |(g, w), t| (g + t.good, w + t.wrong));
        (g as f64 - w as f64) / self.exchanges as f64
    }
    /// Rewordings that failed without being known to fail.
    pub fn regressions(&self) -> usize {
        self.reword_fail.len()
    }
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total = self.reword_pass + self.reword_fail.len() + self.reword_known.len();
        writeln!(f, "rewordings   {}/{} read ({} known failures, {} regressions)", self.reword_pass, total, self.reword_known.len(), self.reword_fail.len())?;
        for id in &self.reword_fail {
            writeln!(f, "  REGRESSION {id}")?;
        }
        for id in &self.reword_known {
            writeln!(f, "  known      {id}")?;
        }
        let n = self.exchanges.max(1) as f64;
        writeln!(f, "exchanges    {} real held-out; abstained {} ({:.0}%)", self.exchanges, self.abstained, self.abstained as f64 / n * 100.0)?;
        for (how, t) in &self.answered {
            writeln!(f, "  {how:<9}  good {}, wrong {}", t.good, t.wrong)?;
        }
        write!(f, "score        {:+.3}  (good minus wrong, over all exchanges)", self.score())
    }
}

fn content(text: &str, stop: &HashSet<String>) -> Vec<String> {
    text.split(|c: char| !(c.is_alphanumeric() || c == '\'')).map(|w| w.to_lowercase()).filter(|w| !w.is_empty() && !stop.contains(w)).collect()
}

/// Word F1 between two texts, stopwords left out.
pub fn overlap(a: &str, b: &str, stop: &HashSet<String>) -> f64 {
    let (x, y) = (content(a, stop), content(b, stop));
    if x.is_empty() || y.is_empty() {
        return if normalize(a) == normalize(b) { 1.0 } else { 0.0 };
    }
    let mut pool = y.clone();
    let mut hit = 0usize;
    for w in &x {
        if let Some(i) = pool.iter().position(|p| p == w) {
            pool.swap_remove(i);
            hit += 1;
        }
    }
    if hit == 0 {
        return 0.0;
    }
    let (p, r) = (hit as f64 / x.len() as f64, hit as f64 / y.len() as f64);
    2.0 * p * r / (p + r)
}

pub fn run(vault_dir: &str, cases: &[Case]) -> Result<Outcome, String> {
    let skip: HashSet<String> = cases.iter().filter(|c| c.kind == "exchange").map(|c| normalize(&c.say)).collect();
    let vault = Vault::open(vault_dir)?;
    let stop: HashSet<String> = vault.lexicon().stopwords.iter().cloned().collect();
    let engine = Engine::with_layers(
        vault,
        Box::new(move |v: &Vault| {
            let mut l = default_layers(v);
            l.recall = Box::new(ReplyMemory::excluding(v, &skip));
            l
        }),
    );
    let mut out = Outcome::default();
    for c in cases {
        let mut conv = engine.conversation();
        for (role, text) in &c.history {
            conv.state.messages.push(Message { role: role.clone(), text: text.clone(), trace: None });
        }
        let reply = conv.say(&c.say);
        match c.kind.as_str() {
            "reword" => {
                let got = reply.trace.requests.first().and_then(|r| r.intent.clone());
                if got.is_some() && got == c.intent {
                    out.reword_pass += 1;
                } else if c.known_fail {
                    out.reword_known.push(c.id.clone());
                } else {
                    out.reword_fail.push(c.id.clone());
                }
            }
            "exchange" => {
                out.exchanges += 1;
                let gold = c.reply.as_deref().ok_or_else(|| format!("{}: exchange needs a reply", c.id))?;
                if reply.acts.iter().any(|a| a.op == Op::TeachMe) || reply.acts.is_empty() {
                    out.abstained += 1;
                    continue;
                }
                let how = if reply.trace.requests.iter().any(|r| r.recalled.is_some()) { "recalled" } else { "stock" };
                let t = out.answered.entry(how).or_default();
                if overlap(&reply.text, gold, &stop) >= GOOD {
                    t.good += 1;
                } else {
                    t.wrong += 1;
                }
            }
            k => return Err(format!("{}: unknown kind '{k}'", c.id)),
        }
    }
    Ok(out)
}
