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
        if c.is_dir() {
            return c;
        }
    }
    PathBuf::from("vault/core")
}

/// Recursively load all `.md` modules under `vault/core/` (any topic folder layout).
/// Each Markdown module can freely mix facts, patterns (`## name`), and episodes (`"in" => "out"`).
pub fn load_vault(dir: &Path) -> Result<AgentState, String> {
    let mut facts = Vec::new();
    let mut patterns = Vec::new();
    let mut episodes = Vec::new();
    let mut next_fact = 1usize;
    let mut next_pat = 1usize;
    let mut next_ep = 1usize;

    for path in list_md_files_recursive(dir)? {
        let module = module_id(dir, &path);
        let raw = fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
        parse_module_md(
            &raw,
            &module,
            &mut facts,
            &mut patterns,
            &mut episodes,
            &mut next_fact,
            &mut next_pat,
            &mut next_ep,
        )
        .map_err(|e| format!("{}: {e}", path.display()))?;
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

fn module_id(root: &Path, path: &Path) -> String {
    let rel = path.strip_prefix(root).unwrap_or(path);
    let mut s = rel.with_extension("").to_string_lossy().replace('\\', "/");
    if s.is_empty() {
        s = "vault".to_string();
    }
    s
}

fn list_md_files_recursive(dir: &Path) -> Result<Vec<PathBuf>, String> {
    if !dir.is_dir() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    collect_md_files(dir, &mut out)?;
    out.sort();
    Ok(out)
}

fn collect_md_files(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let mut entries: Vec<PathBuf> = fs::read_dir(dir)
        .map_err(|e| format!("readdir {}: {e}", dir.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            collect_md_files(&path, out)?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("md") {
            out.push(path);
        }
    }
    Ok(())
}

/// Unified literate-Markdown Compact Rule DSL parser.
///
/// Only fenced code blocks (` ``` ` or ` ```karin `) are parsed; all prose, headings,
/// links, and notes outside code fences are ignored as human documentation.
///
/// Inside a code fence, a module may freely mix:
/// - Facts:    `persona_name(value="Karin")` or `[working] asked_slot(slot="city")`
/// - Patterns: `rule <name>` (or `## <name>`) followed by `when:` / `then:`
/// - Episodes: `"greet Alice" => "Warm greetings, Alice! Glad you're here."`
/// - Comments: lines starting with `#` or `//`
#[allow(clippy::too_many_arguments)]
fn parse_module_md(
    raw: &str,
    module: &str,
    facts: &mut Vec<Fact>,
    patterns: &mut Vec<Pattern>,
    episodes: &mut Vec<Episode>,
    next_fact: &mut usize,
    next_pat: &mut usize,
    next_ep: &mut usize,
) -> Result<(), String> {
    #[derive(PartialEq)]
    enum Block {
        None,
        When,
        Then,
    }

    let mut in_fence = false;
    let mut parse_fence = false;
    let mut current: Option<Pattern> = None;
    let mut block = Block::None;

    let flush_pattern = |current: &mut Option<Pattern>, patterns: &mut Vec<Pattern>| {
        if let Some(mut p) = current.take() {
            infer_kind_if_unset(&mut p);
            patterns.push(p);
        }
    };

    for (line_no, raw_line) in raw.lines().enumerate() {
        let t = raw_line.trim();

        // Toggle fenced code blocks (``` or ```karin).
        if let Some(rest) = t.strip_prefix("```") {
            if !in_fence {
                in_fence = true;
                let lang = rest.trim();
                parse_fence = lang.is_empty() || lang.eq_ignore_ascii_case("karin");
            } else {
                flush_pattern(&mut current, patterns);
                block = Block::None;
                in_fence = false;
                parse_fence = false;
            }
            continue;
        }

        // Everything outside a parsed code fence is ignored as comments/prose.
        if !in_fence || !parse_fence {
            continue;
        }

        if t.is_empty() || t.starts_with("//") || (t.starts_with('#') && !t.starts_with("## ")) {
            continue;
        }

        // `rule <name>` or `## <name>` starts a new pattern.
        let rule_heading = t.strip_prefix("rule ").or_else(|| t.strip_prefix("## "));
        if let Some(heading) = rule_heading {
            flush_pattern(&mut current, patterns);
            let name = heading.trim().to_string();
            let id = format!("p{}", *next_pat);
            *next_pat += 1;
            current = Some(Pattern {
                id,
                name,
                status: PatternStatus::Active,
                when: Vec::new(),
                then: Vec::new(),
                trust: 0.9,
                support: 3,
                provenance: format!("vault:{module}"),
                kind: PatternKind::Reply,
            });
            block = Block::None;
            continue;
        }

        // If inside a pattern, check pattern fields or indented/bulleted continuations.
        if let Some(p) = current.as_mut() {
            let is_indented = raw_line.starts_with(' ') || raw_line.starts_with('\t');
            if let Some(rest) = t.strip_prefix("when:") {
                block = Block::When;
                let rest = rest.trim();
                if !rest.is_empty() {
                    for item in split_top_level(rest, ',') {
                        p.when.push(parse_condition(&item).map_err(|e| format!("line {}: {e}", line_no + 1))?);
                    }
                }
                continue;
            } else if let Some(rest) = t.strip_prefix("then:") {
                block = Block::Then;
                let rest = rest.trim();
                if !rest.is_empty() {
                    for item in split_top_level(rest, ',') {
                        p.then.push(parse_action(&item).map_err(|e| format!("line {}: {e}", line_no + 1))?);
                    }
                }
                continue;
            } else if let Some(rest) = t.strip_prefix("status:") {
                block = Block::None;
                p.status = match rest.trim() {
                    "candidate" => PatternStatus::Candidate,
                    _ => PatternStatus::Active,
                };
                continue;
            } else if let Some(rest) = t.strip_prefix("kind:") {
                block = Block::None;
                p.kind = match rest.trim() {
                    "language" => PatternKind::Language,
                    "reasoning" => PatternKind::Reasoning,
                    "composite" => PatternKind::Composite,
                    "induced" => PatternKind::Induced,
                    _ => PatternKind::Reply,
                };
                continue;
            } else if let Some(rest) = t.strip_prefix("trust:") {
                block = Block::None;
                p.trust = rest
                    .trim()
                    .parse()
                    .map_err(|_| format!("line {}: invalid trust '{}'", line_no + 1, rest.trim()))?;
                continue;
            } else if let Some(rest) = t.strip_prefix("support:") {
                block = Block::None;
                p.support = rest
                    .trim()
                    .parse()
                    .map_err(|_| format!("line {}: invalid support '{}'", line_no + 1, rest.trim()))?;
                continue;
            } else if (is_indented || t.starts_with("- ")) && block != Block::None {
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
                continue;
            } else {
                // Non-indented top-level line ends the current pattern block.
                flush_pattern(&mut current, patterns);
                block = Block::None;
            }
        }

        // Top-level item: either an episode (`"in" => "out"`) or a fact (`rel(k="v")`).
        let top = t.strip_prefix("- ").map(str::trim).unwrap_or(t);

        if top.starts_with('"') {
            if let Some((lhs, rhs)) = split_outside_quotes(top, "=>") {
                let input = unquote(lhs.trim()).map_err(|e| format!("line {}: {e}", line_no + 1))?;
                let reply = unquote(rhs.trim()).map_err(|e| format!("line {}: {e}", line_no + 1))?;
                let id = format!("e{}", *next_ep);
                *next_ep += 1;
                episodes.push(Episode {
                    id,
                    input,
                    reply,
                    source: format!("vault:{module}"),
                    generalized: false,
                });
                continue;
            }
        }

        let (scope, expr) = if let Some(rest) = top.strip_prefix("[working]") {
            (Scope::Working, rest.trim())
        } else if let Some(rest) = top.strip_prefix("[long_term]") {
            (Scope::LongTerm, rest.trim())
        } else {
            (Scope::LongTerm, top)
        };
        if expr.contains('(') {
            let (rel, args) = parse_call(expr).map_err(|e| format!("line {}: {e}", line_no + 1))?;
            let id = format!("f{}", *next_fact);
            *next_fact += 1;
            facts.push(Fact {
                id,
                rel,
                args,
                scope,
                provenance: format!("vault:{module}"),
                turn: 0,
            });
        }
    }

    flush_pattern(&mut current, patterns);
    Ok(())
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

    #[test]
    fn ignores_prose_outside_code_fences() {
        let md = r#"
# Notes

This prose has fake_fact(a="1") and "fake" => "episode" and should be ignored!

rule ignored-rule
when: "hi"
then: => "ignored"

```
real_fact(k="v")

rule real-rule
when: "ping"
then: => "pong"
```

More comments outside fences.

```karin
"a b" => "c b"
```
"#;
        let (mut f, mut p, mut e) = (Vec::new(), Vec::new(), Vec::new());
        let (mut nf, mut np, mut ne) = (1, 1, 1);
        parse_module_md(md, "test", &mut f, &mut p, &mut e, &mut nf, &mut np, &mut ne).unwrap();
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].rel, "real_fact");
        assert_eq!(p.len(), 1);
        assert_eq!(p[0].name, "real-rule");
        assert_eq!(e.len(), 1);
        assert_eq!(e[0].input, "a b");
    }
}
