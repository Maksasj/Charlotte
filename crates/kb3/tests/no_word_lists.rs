//! Design 3.0 rule: no hard-coded English word lists in agent code.
//!
//! Scans the 3.0 agent crates for lists of string literals (array or slice literals, `vec![..]`,
//! and string alternatives in `match` arms / `matches!`). Every word in such a list must belong
//! to the formal vocabulary: schema predicates, relation-line keywords, model tags and
//! technical names. English words (verbs, nouns, cue words) must live in learned data.

use std::path::Path;

const CRATES: &[&str] = &["kb3", "parse3", "teach3", "agent3", "reason4", "agent4", "eval4"];

fn formal() -> Vec<String> {
    let mut v: Vec<String> = kb3::fact::SCHEMA.iter().map(|(p, _)| p.to_string()).collect();
    // relation-line keywords and procedure slots (schema section 5)
    for w in ["do", "need", "time", "gap", "ask", "uses", "user", "start", "level", "teach", "dataset", "derived", "seed", "learned", "fallback", "core", "none", "object", "constraint", "replace", "request"] {
        v.push(w.into());
    }
    // technical names: CLI flags and JSON record keys of the call archive (not language)
    for w in ["key", "answer", "model", "output", "format", "json", "system", "prompt", "tools", "session", "persistence", "strict", "mcp", "config", "result", "unix", "tag", "seconds", "line", "facts", "error"] {
        v.push(w.into());
    }
    // Design 4.0 formal vocabulary: relations and sorts (schema v4, section 4.1-4.2), internal
    // predicates of the core program, protocol keywords (section 6.1) and record keys
    for w in ["item", "place", "material", "property", "function", "state", "agent", "same", "any", "relation",
              "is_a", "same_as", "has_part", "made_of", "has_property", "can_do", "can_do_on", "used_for", "can_go_in",
              "effect", "precondition", "demands", "state_of", "holds", "holdsn", "lholds", "lholdsn", "pos", "neg",
              "isa", "part", "stand_in", "ask_fit", "ask_conflict", "not", "new", "query", "retract", "yes", "no",
              "withdrawn", "because", "taught", "support", "counter", "hypothesis", "given", "stored", "question",
              "teacher", "who", "text", "channel", "conf", "negated", "msg", "kind", "fact", "event", "via", "confirming", "can", "case", "cases", "tasks", "subsumed", "status", "shared", "induce_cases", "unknown", "name", "spike", "summary",
              // kb4 record fields and conversation events (Design 4.0 section 8.2)
              "batch", "batch_end", "built_from", "end", "finish", "jsonl", "lang", "layer", "live", "msg_id", "prov", "raw", "rec", "rule", "schema", "speaker", "turn"] {
        v.push(w.into());
    }
    v
}

/// Words in string literals inside list-like constructs of one source file.
fn listed_words(src: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    for (n, line) in src.lines().enumerate() {
        let t = line.trim();
        if t.starts_with("//") {
            continue;
        }
        let lits: Vec<&str> = t.split('"').skip(1).step_by(2).collect();
        let listy = lits.len() >= 2 && (t.contains("[\"") || t.contains("vec![") || t.contains("\" |") || t.contains("| \"") || t.contains("&[") || t.contains("\", \""));
        if !listy {
            continue;
        }
        for l in lits {
            for w in l.split(|c: char| !c.is_ascii_alphabetic() && c != '_') {
                if w.len() >= 3 && w.chars().all(|c| c.is_ascii_lowercase() || c == '_') {
                    out.push((n + 1, w.to_string()));
                }
            }
        }
    }
    out
}

#[test]
fn agent_crates_hold_no_english_word_lists() {
    let allowed = formal();
    let root = kb3::repo_root().join("crates");
    let mut bad = Vec::new();
    for c in CRATES {
        let dir = root.join(c).join("src");
        if !dir.exists() {
            continue;
        }
        for e in walk(&dir) {
            let src = std::fs::read_to_string(&e).unwrap();
            for (n, w) in listed_words(&src) {
                let ok = allowed.iter().any(|a| *a == w) || w.split('_').all(|p| allowed.iter().any(|a| a == p));
                if !ok {
                    bad.push(format!("{}:{n}: {w}", e.strip_prefix(&root).unwrap().display()));
                }
            }
        }
    }
    assert!(bad.is_empty(), "English words in lists in agent code:\n{}", bad.join("\n"));
}

#[test]
fn the_scanner_catches_a_cue_list() {
    let src = "const CUES: &[&str] = &[\"without\", \"broke\", \"out of\"];";
    let w: Vec<String> = listed_words(src).into_iter().map(|(_, w)| w).collect();
    assert!(w.contains(&"without".to_string()) && w.contains(&"broke".to_string()));
    let src = "match w { \"until\" | \"while\" => 1, _ => 0 }";
    assert!(!listed_words(src).is_empty());
}

fn walk(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    for e in std::fs::read_dir(dir).unwrap().flatten() {
        let p = e.path();
        if p.is_dir() {
            out.extend(walk(&p));
        } else if p.extension().map(|x| x == "rs").unwrap_or(false) {
            out.push(p);
        }
    }
    out
}

/// Rule files (Design 4.0 base rules, data in crates/reason4/core/*.dl): every constant used in
/// a rule must be formal vocabulary (relation names, `user`); a rule may not name a world thing.
#[test]
fn rule_files_name_no_world_things() {
    let allowed = formal();
    let dir = kb3::repo_root().join("crates/reason4/core");
    let mut bad = Vec::new();
    for e in std::fs::read_dir(&dir).unwrap().flatten() {
        let src = std::fs::read_to_string(e.path()).unwrap();
        for (n, line) in src.lines().enumerate() {
            let code = line.split('%').next().unwrap_or("");
            // constants: lower-case identifiers inside an argument list
            let mut depth = 0;
            let mut tok = String::new();
            for ch in code.chars().chain(std::iter::once(' ')) {
                if ch.is_ascii_alphanumeric() || ch == '_' {
                    tok.push(ch);
                    continue;
                }
                if depth > 0 && !tok.is_empty() && tok.chars().next().unwrap().is_ascii_lowercase() && ch != '(' && !allowed.iter().any(|a| *a == tok) {
                    bad.push(format!("{}:{}: {tok}", e.path().display(), n + 1));
                }
                tok.clear();
                if ch == '(' {
                    depth += 1;
                }
                if ch == ')' {
                    depth -= 1;
                }
            }
        }
    }
    assert!(bad.is_empty(), "world words in rule files:\n{}", bad.join("\n"));
}
