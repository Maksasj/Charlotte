//! The three-layer knowledge base (Design 4.0 section 8): raw messages (append-only, never
//! edited), builds (facts, learned rules, vocabulary, provenance: a function of the raw prefix,
//! the agent build and the base-rule hash) and replay through the live entry point.

use crate::{Agent, Teacher};
use serde_json::{json, Value as J};
use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;

pub const RAW_SCHEMA: &str = "raw4/1";
pub const LANG: &str = "formal4/1";
pub const FACTS_SCHEMA: &str = "facts4/1";
pub const PROV_SCHEMA: &str = "prov4/1";

/// A small stable hash (FNV-1a 64) for build ids and the base-rule fingerprint.
pub fn fnv(data: &[u8]) -> String {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in data {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("{h:016x}")
}

/// The agent's conversation as raw4/1 records (one per message, every speaker).
pub fn raw_records(agent: &Agent, channel_default: &str) -> Vec<J> {
    agent
        .raw
        .iter()
        .enumerate()
        .map(|(turn, r)| {
            let who = r["who"].as_str().unwrap_or("");
            let channel = r["channel"].as_str().unwrap_or(channel_default);
            json!({"rec": "raw", "schema": RAW_SCHEMA, "lang": LANG, "msg_id": format!("{}-t{turn:05}", agent.session),
                   "session": agent.session, "turn": turn, "speaker": who, "channel": channel, "text": r["text"]})
        })
        .collect()
}

pub fn write_lines(path: &Path, rows: &[J]) -> std::io::Result<()> {
    if let Some(d) = path.parent() {
        std::fs::create_dir_all(d)?;
    }
    let mut f = std::fs::File::create(path)?;
    for r in rows {
        writeln!(f, "{}", serde_json::to_string(r).unwrap())?;
    }
    Ok(())
}

pub fn read_lines(path: &Path) -> Vec<J> {
    std::fs::read_to_string(path).map(|t| t.lines().filter(|l| !l.trim().is_empty()).filter_map(|l| serde_json::from_str(l).ok()).collect()).unwrap_or_default()
}

/// Write the raw session file and append its entry to raw/INDEX.jsonl.
pub fn write_raw(root: &Path, agent: &Agent, channel: &str) -> std::io::Result<String> {
    let rows = raw_records(agent, channel);
    let path = root.join("raw/sessions").join(format!("{}.jsonl", agent.session));
    write_lines(&path, &rows)?;
    let bytes = std::fs::read(&path)?;
    let idx = json!({"session": agent.session, "channel": channel, "messages": rows.len(), "fnv": fnv(&bytes)});
    let mut index: Vec<J> = read_lines(&root.join("raw/INDEX.jsonl")).into_iter().filter(|r| r["session"] != agent.session.as_str()).collect();
    index.push(idx);
    write_lines(&root.join("raw/INDEX.jsonl"), &index)?;
    Ok(path.display().to_string())
}

/// Write a build: stored knowledge only (derived facts are recomputed, never stored).
pub fn write_build(dir: &Path, agent: &Agent, raw_messages: usize) -> std::io::Result<String> {
    std::fs::create_dir_all(dir)?;
    let base_hash = fnv(reason4::CORE.as_bytes());
    let mut facts = format!("% schema {FACTS_SCHEMA}  base B1-B9 fnv:{base_hash}  raw {raw_messages} msgs\n");
    for (k, (a, s)) in agent.facts.iter().enumerate() {
        if s.retracted_by.is_some() {
            continue;
        }
        let line = if s.negated { format!("-{a}.") } else { format!("{a}.") };
        facts.push_str(&format!("{line:<60} % f:{k:06x} msg {}\n", s.msg));
    }
    std::fs::write(dir.join("facts.dl"), &facts)?;
    let mut rules = String::from("% learned rules: id, layer, status; compiled layer form\n");
    for r in &agent.learned {
        rules.push_str(&format!("% {}  layer {}  status {}  conf {:.2}  support {}  rule {}\n{}\n", r.id, r.layer, r.status, r.conf, r.support.len(), r.show(), r.input.rule));
    }
    std::fs::write(dir.join("rules.dl"), &rules)?;
    let mut vocab: Vec<J> = agent.sorts.iter().map(|(s, sort)| json!({"sym": s, "sort": sort})).collect();
    vocab.extend(agent.canon.iter().map(|(a, c)| json!({"alias": a, "canonical": c})));
    write_lines(&dir.join("vocab.jsonl"), &vocab)?;
    let mut prov: Vec<J> = agent.provenance.iter().map(|p| { let mut p = p.clone(); p["rec"] = json!("prov"); p["schema"] = json!(PROV_SCHEMA); p }).collect();
    for r in &agent.learned {
        prov.push(json!({"rec": "prov", "schema": PROV_SCHEMA, "id": r.id, "kind": "rule", "layer": r.layer, "built_from": r.built_from,
            "support": r.support, "counter": r.counter, "status": r.status, "conf": r.conf, "confirmed_by": r.confirmed_by, "specialised_from": r.specialised_from}));
    }
    write_lines(&dir.join("provenance.jsonl"), &prov)?;
    let counts = json!({"facts": agent.facts.iter().filter(|(_, s)| s.retracted_by.is_none()).count(), "rules": agent.learned.len(), "symbols": agent.sorts.len(), "aliases": agent.canon.len()});
    let id = fnv(format!("{facts}{rules}").as_bytes());
    let build = json!({"build": id, "schemas": {"raw": RAW_SCHEMA, "lang": LANG, "facts": FACTS_SCHEMA, "prov": PROV_SCHEMA},
        "agent": env!("CARGO_PKG_VERSION"), "base_rules": format!("fnv:{base_hash}"), "raw_messages": raw_messages, "counts": counts});
    std::fs::write(dir.join("BUILD.json"), serde_json::to_string_pretty(&build).unwrap())?;
    Ok(id)
}

/// Relation renames applied to old raw messages during replay (`migrations/<from>-<to>.dl`:
/// lines `old_relation -> new_relation.`). Raw is never edited.
pub fn load_migration(path: &Path) -> BTreeMap<String, String> {
    std::fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .filter_map(|l| {
            let l = l.split('%').next()?.trim().trim_end_matches('.');
            let (a, b) = l.split_once("->")?;
            Some((a.trim().to_string(), b.trim().to_string()))
        })
        .collect()
}

pub fn migrate(line: &str, renames: &BTreeMap<String, String>) -> String {
    let mut out = line.to_string();
    for (a, b) in renames {
        for prefix in ["", "not ", "retract "] {
            let from = format!("{prefix}{a}(");
            if out.starts_with(&from) {
                out = format!("{prefix}{b}({}", &out[from.len()..]);
            }
        }
    }
    out
}

/// Answers from raw: the teacher's answer to an identical canonical question at or before
/// that point (replay `--answers raw`).
pub struct RawAnswers {
    pub answers: BTreeMap<String, String>,
    pub unanswered: Vec<String>,
}

impl Teacher for RawAnswers {
    fn answer(&mut self, q: &str) -> String {
        match self.answers.get(q) {
            Some(a) => a.clone(),
            None => {
                self.unanswered.push(q.to_string());
                "unknown".into()
            }
        }
    }
}

/// Replay one raw session through the live entry point. Returns the rebuilt agent and the
/// questions raw could not answer.
pub fn replay(rows: &[J], session: &str, renames: &BTreeMap<String, String>, off: &[String], learning: bool, naming: bool) -> (Agent, Vec<String>) {
    // questions and their answers, by the agent's question index
    let mut asked: BTreeMap<usize, String> = BTreeMap::new();
    let mut answered: BTreeMap<usize, String> = BTreeMap::new();
    for r in rows {
        let t = r["text"].as_str().unwrap_or("");
        if let Some(rest) = t.strip_prefix("ask ") {
            if let Some((n, q)) = rest.split_once(": ") {
                if let Ok(n) = n.parse::<usize>() {
                    asked.insert(n, q.to_string());
                }
            }
        } else if let Some(rest) = t.strip_prefix("answer ") {
            if let Some((n, a)) = rest.split_once(": ") {
                if let Ok(n) = n.parse::<usize>() {
                    answered.insert(n, a.trim_end_matches('.').to_string());
                }
            }
        }
    }
    let mut answers = BTreeMap::new();
    for (n, q) in &asked {
        if let Some(a) = answered.get(n) {
            answers.insert(q.clone(), a.clone());
        }
    }
    let mut oracle = RawAnswers { answers, unanswered: vec![] };
    let mut agent = Agent::new(session);
    agent.off = off.to_vec();
    agent.learning = learning;
    agent.naming = naming;
    let mut started = false;
    for r in rows {
        let speaker = r["speaker"].as_str().unwrap_or("");
        let channel = r["channel"].as_str().unwrap_or("");
        let t = r["text"].as_str().unwrap_or("");
        if speaker != "teacher" || channel == "answer" {
            if speaker == "agent" && channel == "batch_end" {
                agent.end_batch(&mut oracle);
            }
            if speaker == "agent" && channel == "finish" {
                agent.finish();
            }
            if speaker == "agent" && channel == "block_end" {
                if let Some(n) = t.split_whitespace().nth(1).and_then(|v| v.parse().ok()) {
                    agent.mark_block(n);
                }
            }
            continue;
        }
        let line = migrate(t, renames);
        if line.starts_with("topic ") && started {
            // batch boundaries are part of the conversation, so learning replays identically
        }
        started = true;
        agent.hear(&line, channel, &mut oracle);
    }
    (agent, oracle.unanswered)
}
