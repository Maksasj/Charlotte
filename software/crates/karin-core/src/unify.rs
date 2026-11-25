use crate::types::{Condition, Fact};
use std::collections::BTreeMap;

pub type Bindings = BTreeMap<String, String>;

pub fn normalize(s: &str) -> String {
    let trimmed = s.trim();
    let mut out = String::with_capacity(trimmed.len());
    let mut prev_space = false;
    for ch in trimmed.chars() {
        let lower = ch.to_ascii_lowercase();
        if lower.is_whitespace() {
            if !prev_space && !out.is_empty() {
                out.push(' ');
                prev_space = true;
            }
        } else {
            out.push(lower);
            prev_space = false;
        }
    }
    out.trim_end_matches(['?', '!', '.', ',', ';'])
        .trim()
        .to_string()
}

pub fn is_var(s: &str) -> bool {
    s.starts_with('$')
        && s.len() > 1
        && s[1..]
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub fn subst(template: &str, bindings: &Bindings) -> String {
    let mut result = template.to_string();
    let mut keys: Vec<&String> = bindings.keys().collect();
    keys.sort_by_key(|key| std::cmp::Reverse(key.len()));
    for k in keys {
        if let Some(v) = bindings.get(k) {
            result = result.replace(k.as_str(), v.as_str());
        }
    }
    result
}

pub fn subst_args(
    args: &BTreeMap<String, String>,
    bindings: &Bindings,
) -> BTreeMap<String, String> {
    args.iter()
        .map(|(k, v)| (k.clone(), subst(v, bindings)))
        .collect()
}

pub fn bind_value(pattern_val: &str, actual_val: &str, bindings: &Bindings) -> Option<Bindings> {
    if is_var(pattern_val) {
        if let Some(existing) = bindings.get(pattern_val) {
            if normalize(existing) == normalize(actual_val) {
                Some(bindings.clone())
            } else {
                None
            }
        } else {
            let mut next = bindings.clone();
            next.insert(pattern_val.to_string(), actual_val.to_string());
            Some(next)
        }
    } else if normalize(pattern_val) == normalize(actual_val) {
        Some(bindings.clone())
    } else {
        None
    }
}

pub fn match_text_template(template: &str, input: &str, bindings: &Bindings) -> Option<Bindings> {
    let pat_norm = normalize(template);
    let inp_norm = normalize(input);
    let pat_tokens: Vec<&str> = pat_norm.split_whitespace().collect();
    let inp_tokens: Vec<&str> = inp_norm.split_whitespace().collect();
    let raw_tokens: Vec<&str> = input
        .trim()
        .trim_end_matches(['?', '!', '.', ',', ';'])
        .split_whitespace()
        .collect();

    if pat_tokens.is_empty() {
        return None;
    }

    match_tokens(&pat_tokens, &inp_tokens, &raw_tokens, 0, 0, bindings)
}

fn match_tokens(
    pat: &[&str],
    inp: &[&str],
    raw: &[&str],
    pi: usize,
    ii: usize,
    bindings: &Bindings,
) -> Option<Bindings> {
    if pi == pat.len() && ii == inp.len() {
        return Some(bindings.clone());
    }
    if pi == pat.len() || ii == inp.len() {
        return None;
    }

    let p = pat[pi];
    if is_var(p) {
        let remaining_pat = pat.len() - pi - 1;
        if inp.len() < ii + 1 + remaining_pat {
            return None;
        }
        let max_end = inp.len() - remaining_pat;
        if remaining_pat == 0 {
            let captured = slice_joined(raw, inp, ii, inp.len());
            return bind_value(p, &captured, bindings);
        }
        for end in (ii + 1)..=max_end {
            let captured = slice_joined(raw, inp, ii, end);
            if let Some(next) = bind_value(p, &captured, bindings)
                && let Some(done) = match_tokens(pat, inp, raw, pi + 1, end, &next)
            {
                return Some(done);
            }
        }
        None
    } else if p == inp[ii] {
        match_tokens(pat, inp, raw, pi + 1, ii + 1, bindings)
    } else {
        None
    }
}

fn slice_joined(raw: &[&str], inp: &[&str], start: usize, end: usize) -> String {
    if raw.len() == inp.len() {
        raw[start..end].join(" ")
    } else {
        inp[start..end].join(" ")
    }
}

fn match_fact_args(
    cond_args: &BTreeMap<String, String>,
    fact_args: &BTreeMap<String, String>,
    bindings: &Bindings,
) -> Option<Bindings> {
    let mut current = bindings.clone();
    for (k, pat_v) in cond_args {
        let actual_v = fact_args.get(k)?;
        current = bind_value(pat_v, actual_v, &current)?;
    }
    Some(current)
}

pub fn match_conditions(conditions: &[Condition], facts: &[Fact], input: &str) -> Option<Bindings> {
    let empty = Bindings::new();
    match_cond_rec(conditions, 0, facts, input, &empty)
}

fn match_cond_rec(
    conditions: &[Condition],
    idx: usize,
    facts: &[Fact],
    input: &str,
    bindings: &Bindings,
) -> Option<Bindings> {
    if idx == conditions.len() {
        return Some(bindings.clone());
    }

    match &conditions[idx] {
        Condition::Text { template } => {
            let next = match_text_template(template, input, bindings)?;
            match_cond_rec(conditions, idx + 1, facts, input, &next)
        }
        Condition::Contains { phrase } => {
            let inp = format!(" {} ", normalize(input));
            let ph = format!(" {} ", normalize(phrase));
            if inp.contains(&ph) {
                match_cond_rec(conditions, idx + 1, facts, input, bindings)
            } else {
                None
            }
        }
        Condition::Fact { rel, args } => {
            for fact in facts.iter().rev() {
                if fact.rel != *rel {
                    continue;
                }
                if let Some(next) = match_fact_args(args, &fact.args, bindings)
                    && let Some(done) = match_cond_rec(conditions, idx + 1, facts, input, &next)
                {
                    return Some(done);
                }
            }
            None
        }
        Condition::NotFact { rel, args } => {
            let exists = facts.iter().any(|fact| {
                fact.rel == *rel && match_fact_args(args, &fact.args, bindings).is_some()
            });
            if exists {
                None
            } else {
                match_cond_rec(conditions, idx + 1, facts, input, bindings)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_matches_multiword_var() {
        let b = match_text_template("weather in $city", "Weather in New York?", &Bindings::new())
            .expect("should match");
        assert_eq!(b.get("$city").map(String::as_str), Some("New York"));
    }

    #[test]
    fn template_matches_middle_var() {
        let b = match_text_template(
            "say hello in $lang please",
            "say hello in Spanish please!",
            &Bindings::new(),
        )
        .expect("should match");
        assert_eq!(b.get("$lang").map(String::as_str), Some("Spanish"));
    }
}
