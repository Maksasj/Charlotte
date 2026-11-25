use crate::types::{
    Action, AgentState, Condition, Episode, Fact, Pattern, PatternKind, PatternStatus, Scope,
};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Locate `vault/fixtures` (from `FIXTURES_DIR`, cwd, or relative to crate root).
pub fn default_fixtures_dir() -> PathBuf {
    if let Ok(env_dir) = std::env::var("FIXTURES_DIR") {
        return PathBuf::from(env_dir);
    }
    let cwd_candidates = [
        PathBuf::from("vault/fixtures"),
        PathBuf::from("../vault/fixtures"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../vault/fixtures"),
    ];
    for c in cwd_candidates {
        if c.join("patterns").is_dir() {
            return c;
        }
    }
    PathBuf::from("vault/fixtures")
}

/// Load seed facts, patterns, and episodes from Markdown notes under
/// `vault/fixtures/{facts,patterns,episodes}/*.md`.
pub fn load_fixtures(dir: &Path) -> Result<AgentState, String> {
    let mut facts = Vec::new();
    for path in list_md_files(&dir.join("facts"))? {
        let raw = fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
        facts.extend(parse_facts_md(&raw).map_err(|e| format!("{}: {e}", path.display()))?);
    }

    let mut patterns = Vec::new();
    for path in list_md_files(&dir.join("patterns"))? {
        let raw = fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
        patterns.extend(parse_patterns_md(&raw).map_err(|e| format!("{}: {e}", path.display()))?);
    }

    let mut episodes = Vec::new();
    for path in list_md_files(&dir.join("episodes"))? {
        let raw = fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
        episodes.extend(parse_episodes_md(&raw).map_err(|e| format!("{}: {e}", path.display()))?);
    }

    // Sort by numeric ID so ordering is deterministic across files
    facts.sort_by_key(|f| numeric_suffix(&f.id));
    patterns.sort_by_key(|p| numeric_suffix(&p.id));
    episodes.sort_by_key(|e| numeric_suffix(&e.id));

    let max_num = facts
        .iter()
        .map(|f| numeric_suffix(&f.id))
        .chain(patterns.iter().map(|p| numeric_suffix(&p.id)))
        .chain(episodes.iter().map(|e| numeric_suffix(&e.id)))
        .max()
        .unwrap_or(0);

    Ok(AgentState {
        turn: 0,
        facts,
        patterns,
        episodes,
        chains: Vec::new(),
        messages: Vec::new(),
        next_id: (max_num + 1).max(100),
    })
}

pub fn initial_state() -> AgentState {
    let dir = default_fixtures_dir();
    load_fixtures(&dir)
        .unwrap_or_else(|err| panic!("Failed to load Markdown fixtures from {}: {err}", dir.display()))
}

fn list_md_files(dir: &Path) -> Result<Vec<PathBuf>, String> {
    if !dir.is_dir() {
        return Ok(Vec::new());
    }
    let mut files: Vec<PathBuf> = fs::read_dir(dir)
        .map_err(|e| format!("readdir {}: {e}", dir.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("md"))
        .collect();
    files.sort();
    Ok(files)
}

fn parse_facts_md(raw: &str) -> Result<Vec<Fact>, String> {
    let mut out = Vec::new();
    let mut current: Option<Fact> = None;

    for line in raw.lines() {
        let t = line.trim();
        if let Some(heading) = t.strip_prefix("## ") {
            if let Some(f) = current.take() {
                out.push(f);
            }
            let (id, rel) = split_pair(heading, ':')
                .ok_or_else(|| format!("expected '## id: rel', got '{t}'"))?;
            current = Some(Fact {
                id: id.into(),
                rel: rel.into(),
                args: BTreeMap::new(),
                scope: Scope::LongTerm,
                provenance: "vault".into(),
                turn: 0,
            });
        } else if let Some(item) = t.strip_prefix("- ") {
            if let Some(f) = current.as_mut() {
                if let Some((k, v)) = split_pair(item, ':') {
                    match k {
                        "scope" => {
                            f.scope = match v {
                                "working" => Scope::Working,
                                _ => Scope::LongTerm,
                            };
                        }
                        "provenance" => f.provenance = v.into(),
                        _ => {
                            f.args.insert(k.into(), v.into());
                        }
                    }
                }
            }
        }
    }
    if let Some(f) = current {
        out.push(f);
    }
    Ok(out)
}

fn parse_patterns_md(raw: &str) -> Result<Vec<Pattern>, String> {
    #[derive(PartialEq)]
    enum Section {
        Meta,
        When,
        Then,
    }

    let mut out = Vec::new();
    let mut current: Option<Pattern> = None;
    let mut section = Section::Meta;

    for line in raw.lines() {
        let t = line.trim();
        if let Some(heading) = t.strip_prefix("## ") {
            if let Some(p) = current.take() {
                out.push(p);
            }
            let (id, name) = split_pair(heading, ':')
                .ok_or_else(|| format!("expected '## id: name', got '{t}'"))?;
            current = Some(Pattern {
                id: id.into(),
                name: name.into(),
                status: PatternStatus::Active,
                when: Vec::new(),
                then: Vec::new(),
                trust: 0.9,
                support: 1,
                provenance: "vault".into(),
                kind: PatternKind::Reply,
            });
            section = Section::Meta;
        } else if t.eq_ignore_ascii_case("### when") {
            section = Section::When;
        } else if t.eq_ignore_ascii_case("### then") {
            section = Section::Then;
        } else if let Some(item) = t.strip_prefix("- ") {
            let Some(p) = current.as_mut() else {
                continue;
            };
            let (k, v) = split_pair(item, ':')
                .ok_or_else(|| format!("expected '- key: value', got '{t}'"))?;
            match section {
                Section::Meta => match k {
                    "status" => {
                        p.status = match v {
                            "candidate" => PatternStatus::Candidate,
                            _ => PatternStatus::Active,
                        };
                    }
                    "kind" => {
                        p.kind = match v {
                            "language" => PatternKind::Language,
                            "reasoning" => PatternKind::Reasoning,
                            "composite" => PatternKind::Composite,
                            "induced" => PatternKind::Induced,
                            _ => PatternKind::Reply,
                        };
                    }
                    "trust" => {
                        p.trust = v.parse().map_err(|_| format!("invalid trust '{v}'"))?;
                    }
                    "support" => {
                        p.support = v.parse().map_err(|_| format!("invalid support '{v}'"))?;
                    }
                    "provenance" => p.provenance = v.into(),
                    _ => {}
                },
                Section::When => {
                    let cond = match k {
                        "text" => Condition::Text { template: v.into() },
                        "contains" => Condition::Contains { phrase: v.into() },
                        "fact" => {
                            let (rel, args) = parse_call(v)?;
                            Condition::Fact { rel, args }
                        }
                        "not_fact" => {
                            let (rel, args) = parse_call(v)?;
                            Condition::NotFact { rel, args }
                        }
                        other => return Err(format!("unknown when condition '{other}'")),
                    };
                    p.when.push(cond);
                }
                Section::Then => {
                    let act = match k {
                        "assert[working]" => {
                            let (rel, args) = parse_call(v)?;
                            Action::Assert {
                                rel,
                                args,
                                scope: Scope::Working,
                            }
                        }
                        "assert[long_term]" | "assert" => {
                            let (rel, args) = parse_call(v)?;
                            Action::Assert {
                                rel,
                                args,
                                scope: Scope::LongTerm,
                            }
                        }
                        "retract" => {
                            if v.contains('(') {
                                let (rel, args) = parse_call(v)?;
                                Action::Retract {
                                    rel,
                                    args: if args.is_empty() { None } else { Some(args) },
                                }
                            } else {
                                Action::Retract {
                                    rel: v.into(),
                                    args: None,
                                }
                            }
                        }
                        "tool" => {
                            let (name, args) = parse_call(v)?;
                            Action::Tool { name, args }
                        }
                        "reply" => Action::Reply { template: v.into() },
                        other => return Err(format!("unknown then action '{other}'")),
                    };
                    p.then.push(act);
                }
            }
        }
    }
    if let Some(p) = current {
        out.push(p);
    }
    Ok(out)
}

fn parse_episodes_md(raw: &str) -> Result<Vec<Episode>, String> {
    let mut out = Vec::new();
    let mut current: Option<Episode> = None;

    for line in raw.lines() {
        let t = line.trim();
        if let Some(id) = t.strip_prefix("## ") {
            if let Some(e) = current.take() {
                out.push(e);
            }
            current = Some(Episode {
                id: id.trim().into(),
                input: String::new(),
                reply: String::new(),
                source: "vault".into(),
                generalized: false,
            });
        } else if let Some(item) = t.strip_prefix("- ") {
            if let Some(e) = current.as_mut() {
                if let Some((k, v)) = split_pair(item, ':') {
                    match k {
                        "source" => e.source = v.into(),
                        "input" => e.input = v.into(),
                        "reply" => e.reply = v.into(),
                        "generalized" => e.generalized = v == "true",
                        _ => {}
                    }
                }
            }
        }
    }
    if let Some(e) = current {
        out.push(e);
    }
    Ok(out)
}

fn split_pair(s: &str, sep: char) -> Option<(&str, &str)> {
    let idx = s.find(sep)?;
    Some((s[..idx].trim(), s[idx + sep.len_utf8()..].trim()))
}

/// Parse `rel(k=v, k2=v2)` or `rel()`.
fn parse_call(s: &str) -> Result<(String, BTreeMap<String, String>), String> {
    let Some(open) = s.find('(') else {
        return Ok((s.trim().into(), BTreeMap::new()));
    };
    let close = s
        .rfind(')')
        .ok_or_else(|| format!("missing ')' in '{s}'"))?;
    let name = s[..open].trim().to_string();
    let inner = s[open + 1..close].trim();
    let mut args = BTreeMap::new();
    if !inner.is_empty() {
        for part in inner.split(',') {
            if let Some((k, v)) = split_pair(part, '=') {
                args.insert(k.into(), v.into());
            }
        }
    }
    Ok((name, args))
}

fn numeric_suffix(id: &str) -> usize {
    let digits: String = id.chars().skip_while(|c| !c.is_ascii_digit()).collect();
    digits.parse().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_markdown_vault_fixtures() {
        let state = initial_state();
        assert_eq!(state.facts.len(), 10);
        assert_eq!(state.patterns.len(), 29);
        assert_eq!(state.episodes.len(), 4);
    }
}
