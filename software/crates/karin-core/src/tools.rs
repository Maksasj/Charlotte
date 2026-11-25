use crate::types::{Fact, Scope, ToolInfo};
use crate::unify::normalize;
use std::collections::BTreeMap;

pub struct NewFact {
    pub rel: String,
    pub args: BTreeMap<String, String>,
    pub scope: Scope,
    pub provenance: String,
}

pub struct ToolOutcome {
    pub facts: Vec<NewFact>,
    pub summary: String,
}

pub fn list_tools() -> Vec<ToolInfo> {
    vec![
        ToolInfo {
            name: "calc".into(),
            description: "Evaluate a safe arithmetic expression (+, -, *, /, parentheses).".into(),
            meta: false,
        },
        ToolInfo {
            name: "weather".into(),
            description: "Look up weather facts for a city from long-term knowledge.".into(),
            meta: false,
        },
        ToolInfo {
            name: "define".into(),
            description: "Look up a concept definition from long-term knowledge.".into(),
            meta: false,
        },
        ToolInfo {
            name: "note_save".into(),
            description: "Save a personal note into long-term memory.".into(),
            meta: false,
        },
        ToolInfo {
            name: "note_list".into(),
            description: "List all saved notes from long-term memory.".into(),
            meta: false,
        },
        ToolInfo {
            name: "generalize_episodes".into(),
            description: "Meta-tool: anti-unify similar episodes into a candidate general pattern.".into(),
            meta: true,
        },
        ToolInfo {
            name: "chunk_last_chain".into(),
            description: "Meta-tool: compile the last multi-step chain into one composite pattern.".into(),
            meta: true,
        },
    ]
}

pub fn run_world_tool(
    name: &str,
    args: &BTreeMap<String, String>,
    facts: &[Fact],
) -> Option<ToolOutcome> {
    match name {
        "calc" => {
            let expr = args.get("expr").cloned().unwrap_or_default();
            let mut fargs = BTreeMap::new();
            fargs.insert("expr".into(), expr.clone());
            match eval_expr(&expr) {
                Ok(val) => {
                    let formatted = format_number(val);
                    fargs.insert("value".into(), formatted.clone());
                    Some(ToolOutcome {
                        facts: vec![NewFact {
                            rel: "calc_result".into(),
                            args: fargs,
                            scope: Scope::Working,
                            provenance: "tool:calc".into(),
                        }],
                        summary: format!("calc({expr}) = {formatted}"),
                    })
                }
                Err(err) => {
                    fargs.insert("error".into(), err.clone());
                    Some(ToolOutcome {
                        facts: vec![NewFact {
                            rel: "calc_error".into(),
                            args: fargs,
                            scope: Scope::Working,
                            provenance: "tool:calc".into(),
                        }],
                        summary: format!("calc({expr}) error: {err}"),
                    })
                }
            }
        }
        "weather" => {
            let city = args.get("city").cloned().unwrap_or_default();
            let norm_city = normalize(&city);
            let found = facts.iter().rev().find(|f| {
                f.rel == "weather_city"
                    && f.args
                        .get("city")
                        .map(|c| normalize(c) == norm_city)
                        .unwrap_or(false)
            });
            let mut fargs = BTreeMap::new();
            fargs.insert("city".into(), city.clone());
            if let Some(f) = found {
                let cond = f.args.get("condition").cloned().unwrap_or_else(|| "clear".into());
                let temp = f.args.get("temp").cloned().unwrap_or_else(|| "20°C".into());
                fargs.insert("condition".into(), cond.clone());
                fargs.insert("temp".into(), temp.clone());
                Some(ToolOutcome {
                    facts: vec![NewFact {
                        rel: "weather_report".into(),
                        args: fargs,
                        scope: Scope::Working,
                        provenance: "tool:weather".into(),
                    }],
                    summary: format!("weather({city}) -> {cond}, {temp}"),
                })
            } else {
                Some(ToolOutcome {
                    facts: vec![NewFact {
                        rel: "weather_unknown".into(),
                        args: fargs,
                        scope: Scope::Working,
                        provenance: "tool:weather".into(),
                    }],
                    summary: format!("weather({city}) -> unknown"),
                })
            }
        }
        "define" => {
            let topic = args.get("topic").cloned().unwrap_or_default();
            let norm_topic = normalize(&topic);
            let found = facts.iter().rev().find(|f| {
                f.rel == "definition"
                    && f.args
                        .get("topic")
                        .map(|t| normalize(t) == norm_topic)
                        .unwrap_or(false)
            });
            let mut fargs = BTreeMap::new();
            fargs.insert("topic".into(), topic.clone());
            if let Some(f) = found {
                let text = f.args.get("text").cloned().unwrap_or_default();
                fargs.insert("text".into(), text.clone());
                Some(ToolOutcome {
                    facts: vec![NewFact {
                        rel: "definition_result".into(),
                        args: fargs,
                        scope: Scope::Working,
                        provenance: "tool:define".into(),
                    }],
                    summary: format!("define({topic}) -> found"),
                })
            } else {
                Some(ToolOutcome {
                    facts: vec![NewFact {
                        rel: "definition_unknown".into(),
                        args: fargs,
                        scope: Scope::Working,
                        provenance: "tool:define".into(),
                    }],
                    summary: format!("define({topic}) -> unknown"),
                })
            }
        }
        "note_save" => {
            let text = args.get("text").cloned().unwrap_or_default();
            let mut fargs = BTreeMap::new();
            fargs.insert("text".into(), text.clone());
            Some(ToolOutcome {
                facts: vec![
                    NewFact {
                        rel: "note".into(),
                        args: fargs.clone(),
                        scope: Scope::LongTerm,
                        provenance: "tool:note_save".into(),
                    },
                    NewFact {
                        rel: "note_saved".into(),
                        args: fargs,
                        scope: Scope::Working,
                        provenance: "tool:note_save".into(),
                    },
                ],
                summary: format!("note_save(\"{text}\")"),
            })
        }
        "note_list" => {
            let notes: Vec<String> = facts
                .iter()
                .filter(|f| f.rel == "note")
                .filter_map(|f| f.args.get("text").cloned())
                .collect();
            let summary_text = if notes.is_empty() {
                "You don't have any saved notes yet.".to_string()
            } else {
                notes
                    .iter()
                    .enumerate()
                    .map(|(i, n)| format!("{}. {}", i + 1, n))
                    .collect::<Vec<_>>()
                    .join(" | ")
            };
            let mut fargs = BTreeMap::new();
            fargs.insert("list".into(), summary_text.clone());
            fargs.insert("count".into(), notes.len().to_string());
            Some(ToolOutcome {
                facts: vec![NewFact {
                    rel: "notes_listing".into(),
                    args: fargs,
                    scope: Scope::Working,
                    provenance: "tool:note_list".into(),
                }],
                summary: format!("note_list() -> {} notes", notes.len()),
            })
        }
        _ => None,
    }
}

fn format_number(v: f64) -> String {
    if (v.fract()).abs() < 1e-9 {
        format!("{}", v.round() as i64)
    } else {
        format!("{:.4}", v)
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}

/// Safe recursive-descent arithmetic parser (+, -, *, /, parentheses, unary minus).
pub fn eval_expr(input: &str) -> Result<f64, String> {
    let cleaned = input
        .replace('×', "*")
        .replace('÷', "/")
        .replace("plus", "+")
        .replace("minus", "-")
        .replace("times", "*");
    let tokens = tokenize(&cleaned)?;
    let mut pos = 0;
    let val = parse_add_sub(&tokens, &mut pos)?;
    if pos != tokens.len() {
        return Err("unexpected trailing tokens".into());
    }
    if !val.is_finite() {
        return Err("non-finite result".into());
    }
    Ok(val)
}

#[derive(Debug, Clone)]
enum Tok {
    Num(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}

fn tokenize(s: &str) -> Result<Vec<Tok>, String> {
    let mut out = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            ' ' | '\t' | '\n' => i += 1,
            '+' => {
                out.push(Tok::Plus);
                i += 1;
            }
            '-' => {
                out.push(Tok::Minus);
                i += 1;
            }
            '*' => {
                out.push(Tok::Star);
                i += 1;
            }
            '/' => {
                out.push(Tok::Slash);
                i += 1;
            }
            '(' => {
                out.push(Tok::LParen);
                i += 1;
            }
            ')' => {
                out.push(Tok::RParen);
                i += 1;
            }
            c if c.is_ascii_digit() || c == '.' => {
                let start = i;
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    i += 1;
                }
                let slice: String = chars[start..i].iter().collect();
                let n: f64 = slice.parse().map_err(|_| format!("invalid number '{slice}'"))?;
                out.push(Tok::Num(n));
            }
            other => return Err(format!("invalid character '{other}'")),
        }
    }
    if out.is_empty() {
        return Err("empty expression".into());
    }
    Ok(out)
}

fn parse_add_sub(tokens: &[Tok], pos: &mut usize) -> Result<f64, String> {
    let mut left = parse_mul_div(tokens, pos)?;
    while *pos < tokens.len() {
        match tokens[*pos] {
            Tok::Plus => {
                *pos += 1;
                left += parse_mul_div(tokens, pos)?;
            }
            Tok::Minus => {
                *pos += 1;
                left -= parse_mul_div(tokens, pos)?;
            }
            _ => break,
        }
    }
    Ok(left)
}

fn parse_mul_div(tokens: &[Tok], pos: &mut usize) -> Result<f64, String> {
    let mut left = parse_unary(tokens, pos)?;
    while *pos < tokens.len() {
        match tokens[*pos] {
            Tok::Star => {
                *pos += 1;
                left *= parse_unary(tokens, pos)?;
            }
            Tok::Slash => {
                *pos += 1;
                let rhs = parse_unary(tokens, pos)?;
                if rhs.abs() < 1e-12 {
                    return Err("division by zero".into());
                }
                left /= rhs;
            }
            _ => break,
        }
    }
    Ok(left)
}

fn parse_unary(tokens: &[Tok], pos: &mut usize) -> Result<f64, String> {
    if *pos < tokens.len() {
        if let Tok::Minus = tokens[*pos] {
            *pos += 1;
            return Ok(-parse_unary(tokens, pos)?);
        }
    }
    parse_primary(tokens, pos)
}

fn parse_primary(tokens: &[Tok], pos: &mut usize) -> Result<f64, String> {
    if *pos >= tokens.len() {
        return Err("unexpected end of expression".into());
    }
    match tokens[*pos] {
        Tok::Num(n) => {
            *pos += 1;
            Ok(n)
        }
        Tok::LParen => {
            *pos += 1;
            let val = parse_add_sub(tokens, pos)?;
            if *pos >= tokens.len() || !matches!(tokens[*pos], Tok::RParen) {
                return Err("missing closing parenthesis".into());
            }
            *pos += 1;
            Ok(val)
        }
        _ => Err("expected number or '('".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_expr_works() {
        assert_eq!(eval_expr("12 * 7").unwrap(), 84.0);
        assert_eq!(eval_expr("(2 + 3) * 4 - 6 / 2").unwrap(), 17.0);
        assert!(eval_expr("5 / 0").is_err());
    }
}
