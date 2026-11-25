use crate::types::{
    Action, AgentState, Condition, Episode, Fact, Pattern, PatternKind, PatternStatus, Scope,
};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Locate `vault/core` (from `VAULT_CORE_DIR`, cwd, or relative to crate root).
pub fn default_vault_dir() -> PathBuf {
    if let Ok(env_dir) = std::env::var("VAULT_CORE_DIR") {
        return PathBuf::from(env_dir);
    }
    let cwd_candidates = [
        PathBuf::from("vault/core"),
        PathBuf::from("../vault/core"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../vault/core"),
    ];
    for c in cwd_candidates {
        if c.join("patterns").is_dir() {
            return c;
        }
    }
    PathBuf::from("vault/core")
}

/// Load core facts, patterns, and episodes from Compact Rule DSL Markdown files
/// under `vault/core/{facts,patterns,episodes}/*.md`.
pub fn load_vault(dir: &Path) -> Result<AgentState, String> {
    let mut facts = Vec::new();
    let mut next_fact = 1usize;
    for path in list_md_files(&dir.join("facts"))? {
        let stem = file_stem(&path);
        let raw = fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
        let parsed = parse_facts_md(&raw, &stem, &mut next_fact)
            .map_err(|e| format!("{}: {e}", path.display()))?;
        facts.extend(parsed);
    }

    let mut patterns = Vec::new();
    let mut next_pat = 1usize;
    for path in list_md_files(&dir.join("patterns"))? {
        let stem = file_stem(&path);
        let raw = fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
        let parsed = parse_patterns_md(&raw, &stem, &mut next_pat)
            .map_err(|e| format!("{}: {e}", path.display()))?;
        patterns.extend(parsed);
    }

    let mut episodes = Vec::new();
    let mut next_ep = 1usize;
    for path in list_md_files(&dir.join("episodes"))? {
        let stem = file_stem(&path);
        let raw = fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
        let parsed = parse_episodes_md(&raw, &stem, &mut next_ep)
            .map_err(|e| format!("{}: {e}", path.display()))?;
        episodes.extend(parsed);
    }

    let max_num = next_fact.max(next_pat).max(next_ep);

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
    let dir = default_vault_dir();
    load_vault(&dir).unwrap_or_else(|err| {
        panic!("Failed to load Markdown vault from {}: {err}", dir.display())
    })
}

fn file_stem(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("vault")
        .to_string()
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

/// Compact fact lines:
///   `persona_name(value="Karin")`
///   `[working] asked_slot(slot="city")`
fn parse_facts_md(raw: &str, stem: &str, next_id: &mut usize) -> Result<Vec<Fact>, String> {
    let mut out = Vec::new();
    for line in raw.lines() {
        let mut t = line.trim();
        if t.is_empty() || t.starts_with('#') || t.starts_with('>') || t.starts_with("//") {
            continue;
        }
        if let Some(rest) = t.strip_prefix("- ") {
            t = rest.trim();
        }
        let (scope, expr) = if let Some(rest) = t.strip_prefix("[working]") {
            (Scope::Working, rest.trim())
        } else if let Some(rest) = t.strip_prefix("[long_term]") {
            (Scope::LongTerm, rest.trim())
        } else {
            (Scope::LongTerm, t)
        };
        if !expr.contains('(') {
            continue;
        }
        let (rel, args) = parse_call(expr)?;
        let id = format!("f{}", *next_id);
        *next_id += 1;
        out.push(Fact {
            id,
            rel,
            args,
            scope,
            provenance: format!("vault:{stem}"),
            turn: 0,
        });
    }
    Ok(out)
}

/// Compact pattern DSL:
///   ## rule-name
///   when: "weather in ?city", !want_weather()
///   then: +want_weather(city=?city)
///
/// Multi-line `when:` / `then:` blocks are also supported, plus optional
/// `status:`, `kind:`, `trust:`, `support:` overrides.
fn parse_patterns_md(raw: &str, stem: &str, next_id: &mut usize) -> Result<Vec<Pattern>, String> {
    #[derive(PartialEq)]
    enum Block {
        None,
        When,
        Then,
    }

    let mut out = Vec::new();
    let mut current: Option<Pattern> = None;
    let mut block = Block::None;

    for (line_no, raw_line) in raw.lines().enumerate() {
        let t = raw_line.trim();
        if t.is_empty() || t.starts_with('>') || t.starts_with("//") || (t.starts_with("# ") && !t.starts_with("## ")) {
            continue;
        }

        if let Some(heading) = t.strip_prefix("## ") {
            if let Some(mut p) = current.take() {
                infer_kind_if_unset(&mut p);
                out.push(p);
            }
            let name = heading.trim().to_string();
            let id = format!("p{}", *next_id);
            *next_id += 1;
            current = Some(Pattern {
                id,
                name,
                status: PatternStatus::Active,
                when: Vec::new(),
                then: Vec::new(),
                trust: 0.9,
                support: 3,
                provenance: format!("vault:{stem}"),
                kind: PatternKind::Reply,
            });
            block = Block::None;
            continue;
        }

        let Some(p) = current.as_mut() else {
            continue;
        };

        if let Some(rest) = t.strip_prefix("when:") {
            block = Block::When;
            let rest = rest.trim();
            if !rest.is_empty() {
                for item in split_top_level(rest, ',') {
                    p.when.push(parse_condition(&item).map_err(|e| format!("line {}: {e}", line_no + 1))?);
                }
            }
        } else if let Some(rest) = t.strip_prefix("then:") {
            block = Block::Then;
            let rest = rest.trim();
            if !rest.is_empty() {
                for item in split_top_level(rest, ',') {
                    p.then.push(parse_action(&item).map_err(|e| format!("line {}: {e}", line_no + 1))?);
                }
            }
        } else if let Some(rest) = t.strip_prefix("status:") {
            block = Block::None;
            p.status = match rest.trim() {
                "candidate" => PatternStatus::Candidate,
                _ => PatternStatus::Active,
            };
        } else if let Some(rest) = t.strip_prefix("kind:") {
            block = Block::None;
            p.kind = match rest.trim() {
                "language" => PatternKind::Language,
                "reasoning" => PatternKind::Reasoning,
                "composite" => PatternKind::Composite,
                "induced" => PatternKind::Induced,
                _ => PatternKind::Reply,
            };
        } else if let Some(rest) = t.strip_prefix("trust:") {
            block = Block::None;
            p.trust = rest
                .trim()
                .parse()
                .map_err(|_| format!("line {}: invalid trust '{}'", line_no + 1, rest.trim()))?;
        } else if let Some(rest) = t.strip_prefix("support:") {
            block = Block::None;
            p.support = rest
                .trim()
                .parse()
                .map_err(|_| format!("line {}: invalid support '{}'", line_no + 1, rest.trim()))?;
        } else {
            let item = t.strip_prefix("- ").map(str::trim).unwrap_or(t);
            match block {
                Block::When => {
                    for sub in split_top_level(item, ',') {
                        p.when.push(parse_condition(&sub).map_err(|e| format!("line {}: {e}", line_no + 1))?);
                    }
                }
                Block::Then => {
                    for sub in split_top_level(item, ',') {
                        p.then.push(parse_action(&sub).map_err(|e| format!("line {}: {e}", line_no + 1))?);
                    }
                }
                Block::None => {}
            }
        }
    }

    if let Some(mut p) = current {
        infer_kind_if_unset(&mut p);
        out.push(p);
    }
    Ok(out)
}

fn infer_kind_if_unset(p: &mut Pattern) {
    let has_reply = p.then.iter().any(|a| matches!(a, Action::Reply { .. }));
    let has_text = p.when.iter().any(|c| matches!(c, Condition::Text { .. } | Condition::Contains { .. }));
    let has_tool = p.then.iter().any(|a| matches!(a, Action::Tool { .. }));
    if !has_reply {
        if has_text {
            p.kind = PatternKind::Language;
        } else if has_tool {
            p.kind = PatternKind::Reasoning;
        }
    }
}

/// Condition syntax:
///   `"weather in ?city"`   -> Text
///   `~"thank"`             -> Contains
///   `!want_weather()`      -> NotFact
///   `want_weather(city=?c)`-> Fact
fn parse_condition(s: &str) -> Result<Condition, String> {
    let t = s.trim();
    if let Some(rest) = t.strip_prefix('~') {
        let phrase = unquote(rest.trim())?;
        Ok(Condition::Contains { phrase })
    } else if t.starts_with('"') {
        let template = unquote(t)?;
        Ok(Condition::Text { template })
    } else if let Some(rest) = t.strip_prefix('!') {
        let (rel, args) = parse_call(rest.trim())?;
        Ok(Condition::NotFact { rel, args })
    } else {
        let (rel, args) = parse_call(t)?;
        Ok(Condition::Fact { rel, args })
    }
}

/// Action syntax:
///   `=> "Hello ?name"`         -> Reply
///   `@weather(city=?city)`     -> Tool
///   `++user_name(value=?name)` -> Assert (long_term)
///   `+want_weather(city=?c)`   -> Assert (working)
///   `-asked_slot`              -> Retract
fn parse_action(s: &str) -> Result<Action, String> {
    let t = s.trim();
    if let Some(rest) = t.strip_prefix("=>") {
        let template = unquote(rest.trim())?;
        Ok(Action::Reply { template })
    } else if let Some(rest) = t.strip_prefix('@') {
        let (name, args) = parse_call(rest.trim())?;
        Ok(Action::Tool { name, args })
    } else if let Some(rest) = t.strip_prefix("++") {
        let (rel, args) = parse_call(rest.trim())?;
        Ok(Action::Assert {
            rel,
            args,
            scope: Scope::LongTerm,
        })
    } else if let Some(rest) = t.strip_prefix('+') {
        let (rel, args) = parse_call(rest.trim())?;
        Ok(Action::Assert {
            rel,
            args,
            scope: Scope::Working,
        })
    } else if let Some(rest) = t.strip_prefix('-') {
        let rest = rest.trim();
        if rest.contains('(') {
            let (rel, args) = parse_call(rest)?;
            Ok(Action::Retract {
                rel,
                args: if args.is_empty() { None } else { Some(args) },
            })
        } else {
            Ok(Action::Retract {
                rel: rest.into(),
                args: None,
            })
        }
    } else {
        Err(format!("invalid action '{t}' (expected =>, @, +, ++, or -)"))
    }
}

/// Compact episode lines:
///   `"greet Alice" => "Warm greetings, Alice! Glad you're here."`
fn parse_episodes_md(raw: &str, stem: &str, next_id: &mut usize) -> Result<Vec<Episode>, String> {
    let mut out = Vec::new();
    for (line_no, line) in raw.lines().enumerate() {
        let mut t = line.trim();
        if t.is_empty() || t.starts_with('#') || t.starts_with('>') || t.starts_with("//") {
            continue;
        }
        if let Some(rest) = t.strip_prefix("- ") {
            t = rest.trim();
        }
        let Some((lhs, rhs)) = split_outside_quotes(t, "=>") else {
            continue;
        };
        let input = unquote(lhs.trim()).map_err(|e| format!("line {}: {e}", line_no + 1))?;
        let reply = unquote(rhs.trim()).map_err(|e| format!("line {}: {e}", line_no + 1))?;
        let id = format!("e{}", *next_id);
        *next_id += 1;
        out.push(Episode {
            id,
            input,
            reply,
            source: format!("vault:{stem}"),
            generalized: false,
        });
    }
    Ok(out)
}

/// Parse `rel(k="v", k2=?var)` or `rel()` or `rel`.
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
        for part in split_top_level(inner, ',') {
            let eq = part
                .find('=')
                .ok_or_else(|| format!("expected k=v in '{part}'"))?;
            let k = part[..eq].trim().to_string();
            let raw_v = part[eq + 1..].trim();
            let v = if raw_v.starts_with('"') {
                unquote(raw_v)?
            } else {
                raw_v.to_string()
            };
            args.insert(k, v);
        }
    }
    Ok((name, args))
}

fn unquote(s: &str) -> Result<String, String> {
    let t = s.trim();
    if !t.starts_with('"') || !t.ends_with('"') || t.len() < 2 {
        return Err(format!("expected quoted string, got '{t}'"));
    }
    let inner = &t[1..t.len() - 1];
    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('"') => out.push('"'),
                Some('\\') => out.push('\\'),
                Some('n') => out.push('\n'),
                Some(other) => {
                    out.push('\\');
                    out.push(other);
                }
                None => out.push('\\'),
            }
        } else {
            out.push(ch);
        }
    }
    Ok(out)
}

/// Split by `sep` only when outside `"..."` quotes and `(...)` parentheses.
fn split_top_level(s: &str, sep: char) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    let mut escaped = false;
    let mut depth = 0usize;

    for ch in s.chars() {
        if escaped {
            cur.push(ch);
            escaped = false;
            continue;
        }
        if ch == '\\' && in_quotes {
            cur.push(ch);
            escaped = true;
            continue;
        }
        if ch == '"' {
            in_quotes = !in_quotes;
            cur.push(ch);
            continue;
        }
        if !in_quotes {
            if ch == '(' {
                depth += 1;
            } else if ch == ')' && depth > 0 {
                depth -= 1;
            } else if ch == sep && depth == 0 {
                let piece = cur.trim().to_string();
                if !piece.is_empty() {
                    out.push(piece);
                }
                cur.clear();
                continue;
            }
        }
        cur.push(ch);
    }
    let piece = cur.trim().to_string();
    if !piece.is_empty() {
        out.push(piece);
    }
    out
}

fn split_outside_quotes<'a>(s: &'a str, needle: &str) -> Option<(&'a str, &'a str)> {
    let mut in_quotes = false;
    let mut escaped = false;
    for (idx, ch) in s.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' && in_quotes {
            escaped = true;
            continue;
        }
        if ch == '"' {
            in_quotes = !in_quotes;
            continue;
        }
        if !in_quotes && s[idx..].starts_with(needle) {
            return Some((&s[..idx], &s[idx + needle.len()..]));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_compact_dsl_vault_core() {
        let state = initial_state();
        assert_eq!(state.facts.len(), 10);
        assert_eq!(state.patterns.len(), 29);
        assert_eq!(state.episodes.len(), 4);
    }
}
