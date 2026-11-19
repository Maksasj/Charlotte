//! The 4.0 message grammar (Design 4.0 section 6.1): formal lines in both directions.

use reason4::parse_atom;
use sla_core::term::GroundAtom;

#[derive(Debug, Clone, PartialEq)]
pub enum Msg {
    Topic(String),
    New { sym: String, sort: String },
    Fact { atom: GroundAtom, negated: bool },
    Retract { atom: GroundAtom, negated: bool },
    Answer { n: usize, text: String },
    /// `query: <goal>. given: <facts>.` (goal may contain variables, written with upper case)
    Query { goal: String, given: Vec<(GroundAtom, bool)> },
    /// `request(<task>). <situation facts>.`
    Request { task: String, given: Vec<(GroundAtom, bool)> },
}

fn split_statements(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut cur = String::new();
    for ch in s.chars() {
        match ch {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {}
        }
        if ch == '.' && depth == 0 {
            if !cur.trim().is_empty() {
                out.push(cur.trim().to_string());
            }
            cur.clear();
        } else {
            cur.push(ch);
        }
    }
    if !cur.trim().is_empty() {
        out.push(cur.trim().to_string());
    }
    out
}

fn fact(s: &str) -> Result<(GroundAtom, bool), String> {
    let s = s.trim();
    match s.strip_prefix("not ") {
        Some(r) => Ok((parse_atom(r)?, true)),
        None => Ok((parse_atom(s)?, false)),
    }
}

/// Parse one message line. Comments (`%`) and blank lines give `Ok(None)`.
pub fn parse(line: &str) -> Result<Option<Msg>, String> {
    let line = line.split('%').next().unwrap_or("").trim();
    if line.is_empty() {
        return Ok(None);
    }
    if let Some(rest) = line.strip_prefix("query:") {
        let (goal, given) = match rest.split_once("given:") {
            Some((g, f)) => (g, f),
            None => (rest, ""),
        };
        let goal = goal.trim().trim_end_matches('.').trim().to_string();
        let given = split_statements(given).iter().map(|s| fact(s)).collect::<Result<_, _>>()?;
        return Ok(Some(Msg::Query { goal, given }));
    }
    if line.starts_with("request(") {
        let st = split_statements(line);
        let task = st[0].trim_start_matches("request(").trim_end_matches(')').trim().to_string();
        let given = st[1..].iter().map(|s| fact(s)).collect::<Result<_, _>>()?;
        return Ok(Some(Msg::Request { task, given }));
    }
    if let Some(rest) = line.strip_prefix("topic ") {
        return Ok(Some(Msg::Topic(rest.trim().trim_end_matches('.').to_string())));
    }
    if let Some(rest) = line.strip_prefix("answer ") {
        let (n, text) = rest.split_once(':').ok_or("answer N: text")?;
        let n = n.trim().parse::<usize>().map_err(|e| e.to_string())?;
        return Ok(Some(Msg::Answer { n, text: text.trim().trim_end_matches('.').to_string() }));
    }
    if let Some(rest) = line.strip_prefix("retract ") {
        let (atom, negated) = fact(rest.trim_end_matches('.'))?;
        return Ok(Some(Msg::Retract { atom, negated }));
    }
    let body = line.trim_end_matches('.');
    if let Some(inner) = body.strip_prefix("new(") {
        let inner = inner.trim_end_matches(')');
        let (sym, sort) = inner.split_once(',').ok_or("new(sym, sort)")?;
        return Ok(Some(Msg::New { sym: sym.trim().into(), sort: sort.trim().into() }));
    }
    let (atom, negated) = fact(body)?;
    Ok(Some(Msg::Fact { atom, negated }))
}
