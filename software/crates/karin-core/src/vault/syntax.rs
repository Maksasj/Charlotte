use crate::types::{Action, Condition, Scope};
use std::collections::BTreeMap;

pub(crate) fn parse_condition(raw: &str) -> Result<Condition, String> {
    let trimmed = raw.trim();
    if let Some(rest) = trimmed.strip_prefix('~') {
        Ok(Condition::Contains {
            phrase: unquote(rest.trim())?,
        })
    } else if trimmed.starts_with('"') {
        Ok(Condition::Text {
            template: unquote(trimmed)?,
        })
    } else if let Some(rest) = trimmed.strip_prefix('!') {
        let (rel, args) = parse_call(rest.trim())?;
        Ok(Condition::NotFact { rel, args })
    } else {
        let (rel, args) = parse_call(trimmed)?;
        Ok(Condition::Fact { rel, args })
    }
}

pub(crate) fn parse_action(raw: &str) -> Result<Action, String> {
    let trimmed = raw.trim();
    if let Some(rest) = trimmed.strip_prefix("=>") {
        Ok(Action::Reply {
            template: unquote(rest.trim())?,
        })
    } else if let Some(rest) = trimmed.strip_prefix('@') {
        let (name, args) = parse_call(rest.trim())?;
        Ok(Action::Tool { name, args })
    } else if let Some(rest) = trimmed.strip_prefix("++") {
        let (rel, args) = parse_call(rest.trim())?;
        Ok(Action::Assert {
            rel,
            args,
            scope: Scope::LongTerm,
        })
    } else if let Some(rest) = trimmed.strip_prefix('+') {
        let (rel, args) = parse_call(rest.trim())?;
        Ok(Action::Assert {
            rel,
            args,
            scope: Scope::Working,
        })
    } else if let Some(rest) = trimmed.strip_prefix('-') {
        let rest = rest.trim();
        if rest.contains('(') {
            let (rel, args) = parse_call(rest)?;
            Ok(Action::Retract {
                rel,
                args: (!args.is_empty()).then_some(args),
            })
        } else {
            Ok(Action::Retract {
                rel: rest.into(),
                args: None,
            })
        }
    } else {
        Err(format!(
            "invalid action '{trimmed}' (expected =>, @, +, ++, or -)"
        ))
    }
}

pub(crate) fn parse_call(raw: &str) -> Result<(String, BTreeMap<String, String>), String> {
    let Some(open) = raw.find('(') else {
        return Ok((raw.trim().into(), BTreeMap::new()));
    };
    let close = raw
        .rfind(')')
        .ok_or_else(|| format!("missing ')' in '{raw}'"))?;
    let name = raw[..open].trim().to_string();
    let inner = raw[open + 1..close].trim();

    let mut args = BTreeMap::new();
    if !inner.is_empty() {
        for part in split_top_level(inner, ',') {
            let eq = part
                .find('=')
                .ok_or_else(|| format!("expected k=v in '{part}'"))?;
            let key = part[..eq].trim().to_string();
            let raw_val = part[eq + 1..].trim();
            let value = if raw_val.starts_with('"') {
                unquote(raw_val)?
            } else {
                raw_val.to_string()
            };
            args.insert(key, value);
        }
    }
    Ok((name, args))
}

pub(crate) fn unquote(raw: &str) -> Result<String, String> {
    let trimmed = raw.trim();
    if !trimmed.starts_with('"') || !trimmed.ends_with('"') || trimmed.len() < 2 {
        return Err(format!("expected quoted string, got '{trimmed}'"));
    }

    let inner = &trimmed[1..trimmed.len() - 1];
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

pub(crate) fn split_top_level(input: &str, sep: char) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut escaped = false;
    let mut depth = 0usize;

    for ch in input.chars() {
        if escaped {
            current.push(ch);
            escaped = false;
            continue;
        }
        if ch == '\\' && in_quotes {
            current.push(ch);
            escaped = true;
            continue;
        }
        if ch == '"' {
            in_quotes = !in_quotes;
            current.push(ch);
            continue;
        }
        if !in_quotes {
            match ch {
                '(' => depth += 1,
                ')' if depth > 0 => depth -= 1,
                c if c == sep && depth == 0 => {
                    let piece = current.trim().to_string();
                    if !piece.is_empty() {
                        parts.push(piece);
                    }
                    current.clear();
                    continue;
                }
                _ => {}
            }
        }
        current.push(ch);
    }

    let piece = current.trim().to_string();
    if !piece.is_empty() {
        parts.push(piece);
    }
    parts
}

pub(crate) fn split_outside_quotes<'a>(input: &'a str, needle: &str) -> Option<(&'a str, &'a str)> {
    let mut in_quotes = false;
    let mut escaped = false;

    for (idx, ch) in input.char_indices() {
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
        if !in_quotes && input[idx..].starts_with(needle) {
            return Some((&input[..idx], &input[idx + needle.len()..]));
        }
    }
    None
}
