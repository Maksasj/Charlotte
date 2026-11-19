//! Design 3.0 rule: no hard-coded English word lists in agent code.
//!
//! Scans the 3.0 agent crates for lists of string literals (array or slice literals, `vec![..]`,
//! and string alternatives in `match` arms / `matches!`). Every word in such a list must belong
//! to the formal vocabulary: schema predicates, relation-line keywords, model tags and
//! technical names. English words (verbs, nouns, cue words) must live in learned data.

use std::path::Path;

const CRATES: &[&str] = &["kb3", "parse3", "teach3", "agent3"];

fn formal() -> Vec<String> {
    let mut v: Vec<String> = kb3::fact::SCHEMA.iter().map(|(p, _)| p.to_string()).collect();
    // relation-line keywords and procedure slots (schema section 5)
    for w in ["do", "need", "time", "gap", "ask", "uses", "user", "start", "level", "teach", "dataset", "derived", "seed", "learned", "fallback", "core", "none", "object", "constraint", "replace"] {
        v.push(w.into());
    }
    // technical names: CLI flags and JSON record keys of the call archive (not language)
    for w in ["key", "answer", "model", "output", "format", "json", "system", "prompt", "tools", "session", "persistence", "strict", "mcp", "config", "result", "unix", "tag", "seconds", "line", "facts", "error"] {
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
