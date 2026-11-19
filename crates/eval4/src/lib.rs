//! Design 4.0 spike harness (milestone 31-32): the fixture delivered through the conversation
//! path by an oracle teacher, the leak check, the derivation scorer, ablations, curriculum
//! prefixes and size stress. Zero model calls.

pub mod scale;

use agent4::{Agent, Answer, Teacher};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as J};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

pub fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Gold {
    pub kind: String,
    #[serde(default)]
    pub atoms: Vec<String>,
    #[serde(default)]
    pub proof_lines: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Question {
    pub id: String,
    #[serde(rename = "type")]
    pub qtype: String,
    pub hops: Option<u32>,
    #[serde(default)]
    pub near_miss: bool,
    pub query: String,
    pub gold: Gold,
}

#[derive(Debug, Clone)]
pub struct Line {
    pub file: String,
    pub no: usize,
    pub text: String,
}

pub struct Fixture {
    pub teach: Vec<Line>,
    pub answers: BTreeMap<String, String>,
    pub questions: Vec<Question>,
    pub deny: BTreeSet<String>,
}

/// Normalise an atom line for comparison: no spaces, no trailing period.
pub fn norm(s: &str) -> String {
    s.trim().trim_end_matches('.').chars().filter(|c| !c.is_whitespace()).collect()
}

impl Fixture {
    pub fn load(dir: &Path) -> Result<Fixture, String> {
        let mut teach = Vec::new();
        // the curriculum order is part of the fixture (teach/order.txt), not of this code
        let order: Vec<String> = match std::fs::read_to_string(dir.join("teach/order.txt")) {
            Ok(t) => t.lines().map(|l| l.trim().to_string()).filter(|l| !l.is_empty()).collect(),
            Err(_) => {
                let mut v: Vec<String> = std::fs::read_dir(dir.join("teach")).map(|r| r.flatten().map(|e| e.file_name().to_string_lossy().to_string()).filter(|n| n.ends_with(".msgs")).collect()).unwrap_or_default();
                v.sort();
                v
            }
        };
        for f in &order {
            let p = dir.join("teach").join(f);
            let Ok(t) = std::fs::read_to_string(&p) else { continue };
            for (k, l) in t.lines().enumerate() {
                let text = l.split('%').next().unwrap_or("").trim().to_string();
                if !text.is_empty() {
                    teach.push(Line { file: format!("teach/{f}"), no: k + 1, text });
                }
            }
        }
        let mut answers = BTreeMap::new();
        if let Ok(t) = std::fs::read_to_string(dir.join("script/answers.jsonl")) {
            for l in t.lines().filter(|l| !l.trim().is_empty()) {
                let v: J = serde_json::from_str(l).map_err(|e| format!("answers: {e}"))?;
                answers.insert(norm(v["q"].as_str().unwrap_or("")), v["a"].as_str().unwrap_or("unknown").trim().trim_end_matches('.').to_string());
            }
        }
        let qt = std::fs::read_to_string(dir.join("bench/questions.jsonl")).map_err(|e| format!("questions: {e}"))?;
        let questions = qt.lines().filter(|l| !l.trim().is_empty()).map(|l| serde_json::from_str(l).map_err(|e| format!("question {l}: {e}"))).collect::<Result<_, _>>()?;
        let deny = std::fs::read_to_string(dir.join("bench/deny.txt")).unwrap_or_default().lines().map(norm).filter(|s| !s.is_empty() && !s.starts_with('%')).collect();
        Ok(Fixture { teach, answers, questions, deny })
    }
}

/// The oracle teacher: scripted answers; denied content is withheld (D4.8).
pub struct Oracle<'a> {
    pub answers: &'a BTreeMap<String, String>,
    pub deny: &'a BTreeSet<String>,
    pub unexpected: Vec<String>,
    pub withheld: usize,
}

/// The atoms a question mentions, as normalised strings (ignoring `not`).
fn atoms_in(q: &str) -> Vec<String> {
    let mut out = Vec::new();
    let b = q.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i].is_ascii_lowercase() {
            let start = i;
            while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
                i += 1;
            }
            if i < b.len() && b[i] == b'(' {
                let mut depth = 0;
                let mut j = i;
                while j < b.len() {
                    if b[j] == b'(' { depth += 1 }
                    if b[j] == b')' { depth -= 1; if depth == 0 { break } }
                    j += 1;
                }
                out.push(norm(&q[start..=j.min(b.len() - 1)]));
                i = j + 1;
                continue;
            }
        }
        i += 1;
    }
    out
}

fn qkind(q: &str) -> &'static str {
    let l = q.to_lowercase();
    if l.contains("same_as(") && l.contains('?') { "naming" } else if l.starts_with("conflict") || l.contains("exception?") { "conflict" } else { "ask" }
}

impl Teacher for Oracle<'_> {
    fn answer(&mut self, q: &str) -> String {
        let key = norm(q);
        let found = self.answers.get(&key).cloned().or_else(|| {
            // tolerant: same kind of question about the same first atom
            let (k, a) = (qkind(q), atoms_in(q));
            self.answers.iter().find(|(kq, _)| qkind(kq) == k && !a.is_empty() && atoms_in(kq).first() == a.first()).map(|(_, v)| v.clone())
        });
        match found.as_ref() {
            Some(a) => {
                if self.deny.contains(&norm(a.trim_start_matches("not "))) || self.deny.contains(&norm(a)) {
                    self.withheld += 1;
                    "unknown".into()
                } else {
                    a.clone()
                }
            }
            None => {
                // questions about a denied atom are never answered
                let atom = q.trim_start_matches("ask:").trim().trim_end_matches('?').trim_start_matches("not ").to_string();
                if self.deny.contains(&norm(&atom)) {
                    self.withheld += 1;
                }
                self.unexpected.push(q.to_string());
                "unknown".into()
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Opts {
    pub off: Vec<String>,
    pub learn: bool,
    pub naming: bool,
    pub prefix: f64,
    pub order_seed: Option<u64>,
    pub min_support: usize,
}

pub struct Delivery {
    pub agent: Agent,
    pub delivered: usize,
    pub withheld: usize,
    pub unexpected: Vec<String>,
    pub leaks: Vec<String>,
    pub learn_lines: Vec<String>,
    /// (file:line, reply) for every rejected message
    pub rejects: Vec<(String, String)>,
}

fn shuffled<T: Clone>(v: &[T], seed: u64) -> Vec<T> {
    // a small deterministic shuffle (xorshift), by topic blocks so declarations keep order
    let mut out: Vec<T> = v.to_vec();
    let mut x = seed.wrapping_mul(0x9E3779B97F4A7C15) | 1;
    for i in (1..out.len()).rev() {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        let j = (x % (i as u64 + 1)) as usize;
        out.swap(i, j);
    }
    out
}

/// Teach the fixture (a prefix of it, in curriculum order or a seeded order of topic blocks).
pub fn deliver(fx: &Fixture, o: &Opts) -> Delivery {
    agent4::learn::MIN_SUPPORT.store(o.min_support.max(1), std::sync::atomic::Ordering::SeqCst);
    let mut agent = Agent::new("spike");
    agent.off = o.off.clone();
    agent.learning = o.learn;
    agent.naming = o.naming;
    let mut oracle = Oracle { answers: &fx.answers, deny: &fx.deny, unexpected: vec![], withheld: 0 };
    // topic blocks: a block starts at each `topic` line
    let mut blocks: Vec<Vec<Line>> = Vec::new();
    for l in &fx.teach {
        if l.text.starts_with("topic ") || blocks.is_empty() {
            blocks.push(Vec::new());
        }
        blocks.last_mut().unwrap().push(l.clone());
    }
    // declarations stay first so a reordered block still type-checks
    let decls: Vec<Line> = fx.teach.iter().filter(|l| l.text.starts_with("new(")).cloned().collect();
    if let Some(seed) = o.order_seed {
        blocks = shuffled(&blocks, seed);
    }
    let all: Vec<Line> = blocks.into_iter().flatten().filter(|l| !l.text.starts_with("new(") || o.order_seed.is_none()).collect();
    let n = ((all.len() as f64) * o.prefix).round() as usize;
    let mut learn_lines = Vec::new();
    let mut rejects = Vec::new();
    let mut delivered = 0;
    let mut withheld = 0;
    if o.order_seed.is_some() {
        for d in &decls {
            agent.hear(&d.text, "fixture", &mut oracle);
        }
    }
    for l in all.into_iter().take(n) {
        let atom = l.text.trim_start_matches("not ").to_string();
        if fx.deny.contains(&norm(&atom)) {
            withheld += 1;
            continue;
        }
        if l.text.starts_with("topic ") && delivered > 0 {
            learn_lines.extend(agent.end_batch(&mut oracle));
        }
        for r in agent.hear(&l.text, "fixture", &mut oracle) {
            if r.starts_with("reject:") {
                rejects.push((format!("{}:{}", l.file, l.no), r));
            }
        }
        delivered += 1;
    }
    learn_lines.extend(agent.end_batch(&mut oracle));
    learn_lines.extend(agent4::learn::retry_suspended(&mut agent, true));
    // delivered leak check: no denied atom may be stored
    let leaks: Vec<String> = agent
        .facts
        .iter()
        .filter(|(_, s)| s.retracted_by.is_none())
        .map(|(a, s)| if s.negated { format!("not {a}") } else { a.to_string() })
        .filter(|f| fx.deny.contains(&norm(f.trim_start_matches("not "))) || fx.deny.contains(&norm(f)))
        .collect();
    Delivery { withheld: withheld + oracle.withheld, unexpected: oracle.unexpected, agent, delivered, leaks, learn_lines, rejects }
}

#[derive(Debug, Clone, Serialize)]
pub struct Scored {
    pub id: String,
    pub qtype: String,
    pub hops: Option<u32>,
    pub near_miss: bool,
    pub gold_kind: String,
    pub class: String,
    pub claims: Vec<String>,
    pub kinds: Vec<String>,
    pub proof_ok: bool,
    pub asks: usize,
}

/// A claim's proof is valid if it has a record and every leaf is a delivered taught fact or a
/// `given` premise (the Rust self-check; `tools/design4/ref_reason.py` is the checker of record).
fn proof_valid(rec: &J, taught: &BTreeSet<String>, given: &BTreeSet<String>) -> bool {
    let Some(leaves) = rec["leaves"].as_array() else { return false };
    leaves.iter().all(|l| {
        let s = norm(l.as_str().unwrap_or(""));
        taught.contains(&s) || given.contains(&s) || s.starts_with("inheritable(")
    })
}

pub fn score(agent: &mut Agent, q: &Question) -> Scored {
    let ans: Answer = agent.respond(&q.query);
    let taught: BTreeSet<String> = agent.engine_facts(&[]).iter().map(|f| norm(&f.atom.to_string())).collect();
    let given: BTreeSet<String> = match agent4::msg::parse(&q.query) {
        Ok(Some(agent4::msg::Msg::Query { given, .. })) | Ok(Some(agent4::msg::Msg::Request { given, .. })) => {
            given.iter().map(|(a, n)| norm(&reason4::encode(a, *n).to_string())).collect()
        }
        _ => BTreeSet::new(),
    };
    let claims: Vec<String> = ans.claims.iter().map(|(c, _)| norm(c)).collect();
    let kinds: Vec<String> = ans.claims.iter().map(|(_, r)| r["kind"].as_str().unwrap_or("none").to_string()).collect();
    let proof_ok = ans.claims.iter().all(|(_, r)| proof_valid(r, &taught, &given));
    let gold: Vec<String> = q.gold.atoms.iter().map(|a| norm(a)).collect();
    let has_all = !gold.is_empty() && gold.iter().all(|g| claims.contains(g));
    let hyp_used = ans.claims.iter().filter(|(c, _)| gold.contains(&norm(c))).any(|(_, r)| r["kind"] == "hypothesis");
    let class = if !proof_ok && !claims.is_empty() {
        "invented"
    } else {
        match q.gold.kind.as_str() {
            "answer" => {
                if has_all && !hyp_used {
                    "correct"
                } else if has_all {
                    "hyp"
                } else if claims.is_empty() {
                    "gap"
                } else {
                    "wrong"
                }
            }
            "hyp" => {
                if has_all {
                    "correct"
                } else if claims.is_empty() {
                    "gap"
                } else {
                    "wrong"
                }
            }
            _ => {
                if claims.is_empty() {
                    "correct"
                } else {
                    "wrong"
                }
            }
        }
    };
    Scored { id: q.id.clone(), qtype: q.qtype.clone(), hops: q.hops, near_miss: q.near_miss, gold_kind: q.gold.kind.clone(), class: class.into(), claims, kinds, proof_ok, asks: ans.asks.len() }
}

pub fn wilson(k: usize, n: usize) -> (f64, f64) {
    if n == 0 {
        return (0.0, 0.0);
    }
    let z = 1.959964f64;
    let nf = n as f64;
    let p = k as f64 / nf;
    let d = 1.0 + z * z / nf;
    let c = (p + z * z / (2.0 * nf)) / d;
    let h = z * ((p * (1.0 - p) / nf + z * z / (4.0 * nf * nf)).sqrt()) / d;
    ((c - h).max(0.0), (c + h).min(1.0))
}

fn r3(x: f64) -> f64 {
    (x * 1000.0).round() / 1000.0
}

/// Summary of a scored run: classes overall, by type and by hops; the G1 stratum.
pub fn summary(sc: &[Scored]) -> J {
    let count = |f: &dyn Fn(&Scored) -> bool| -> BTreeMap<String, usize> {
        let mut m = BTreeMap::new();
        for s in sc.iter().filter(|s| f(s)) {
            *m.entry(s.class.clone()).or_insert(0) += 1;
        }
        m
    };
    let mut by_type = serde_json::Map::new();
    let types: BTreeSet<String> = sc.iter().map(|s| s.qtype.clone()).collect();
    for t in types {
        by_type.insert(t.clone(), json!(count(&|s: &Scored| s.qtype == t)));
    }
    let mut by_hops = serde_json::Map::new();
    for h in [1u32, 2, 3] {
        by_hops.insert(h.to_string(), json!(count(&|s: &Scored| s.hops == Some(h) && s.gold_kind == "answer")));
    }
    // G1 stratum: answerable deductive 2-hop questions
    let g1: Vec<&Scored> = sc.iter().filter(|s| s.hops == Some(2) && s.gold_kind == "answer" && s.qtype != "IND" && s.qtype != "GAP").collect();
    let g1_ok = g1.iter().filter(|s| s.class == "correct").count();
    let (lo, hi) = wilson(g1_ok, g1.len());
    let invented = sc.iter().filter(|s| s.class == "invented").count();
    let proofs_ok = sc.iter().filter(|s| !s.claims.is_empty()).all(|s| s.proof_ok);
    json!({
        "n": sc.len(),
        "classes": count(&|_| true),
        "by_type": by_type,
        "by_hops_answerable": by_hops,
        "g1_two_hop": {"n": g1.len(), "correct": g1_ok, "rate": r3(g1_ok as f64 / g1.len().max(1) as f64), "ci95": [r3(lo), r3(hi)]},
        "invented": invented,
        "all_proofs_valid": proofs_ok,
        "ind_correct": sc.iter().filter(|s| s.qtype == "IND" && s.class == "correct").count(),
        "wrong": sc.iter().filter(|s| s.class == "wrong").count(),
        "gap_correct": sc.iter().filter(|s| s.gold_kind == "gap" && s.class == "correct").count(),
        "asks": sc.iter().map(|s| s.asks).sum::<usize>(),
    })
}

/// Naming drift (milestone 32): every n-th item symbol is written as a synonym `alt_<x>` in
/// its mentions after the first one; the oracle knows `same_as(alt_<x>, <x>)`.
pub fn drift(fx: &Fixture, every: usize) -> (Fixture, usize) {
    let base = deliver(fx, &Opts { learn: false, naming: false, prefix: 1.0, min_support: 3, ..Default::default() });
    let items: Vec<String> = base.agent.sorts.iter().filter(|(_, s)| *s == "item").map(|(k, _)| k.clone()).collect();
    let chosen: BTreeSet<String> = items.iter().enumerate().filter(|(k, _)| k % every == 0).map(|(_, s)| s.clone()).collect();
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut teach = Vec::new();
    let mut answers = fx.answers.clone();
    for l in &fx.teach {
        let mut out = String::new();
        let mut tok = String::new();
        let flush = |tok: &mut String, out: &mut String, seen: &mut BTreeSet<String>| {
            if !tok.is_empty() {
                if chosen.contains(tok.as_str()) && seen.contains(tok.as_str()) && !l.text.starts_with("new(") {
                    out.push_str(&format!("alt_{tok}"));
                } else {
                    out.push_str(tok);
                    seen.insert(tok.clone());
                }
                tok.clear();
            }
        };
        for ch in l.text.chars() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                tok.push(ch);
            } else {
                flush(&mut tok, &mut out, &mut seen);
                out.push(ch);
            }
        }
        flush(&mut tok, &mut out, &mut seen);
        teach.push(Line { text: out, ..l.clone() });
    }
    for x in &chosen {
        answers.insert(norm(&format!("same_as(alt_{x}, {x})? or is_a(alt_{x}, {x})?")), format!("same_as(alt_{x}, {x})"));
    }
    (Fixture { teach, answers, questions: fx.questions.clone(), deny: fx.deny.clone() }, chosen.len())
}
