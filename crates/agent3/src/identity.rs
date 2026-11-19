//! Task identity (decision D17): IDF-weighted overlap between the request's words and what the
//! agent knows about each task (its symbols and the names it learned from teaching). Words are
//! compared by a shared 4-letter stem (phase A experiment X2). No word lists.

use kb3::fact::unquote;
use kb3::Vault;
use parse3::{tokenize, Models};
use std::collections::{BTreeMap, BTreeSet};

const STEM: usize = 4;

fn stem(w: &str) -> String {
    w.chars().take(STEM).collect()
}

fn is_word(w: &str) -> bool {
    w.chars().count() > 2 && w.chars().any(|c| c.is_ascii_alphabetic())
}

pub fn words(text: &str) -> Vec<String> {
    tokenize(text).into_iter().filter(|w| is_word(w) && !w.chars().all(|c| c.is_ascii_digit())).map(|w| stem(&w)).collect()
}

/// Lower-case letters and spaces only, for storing a learned name.
pub fn clean(text: &str) -> String {
    tokenize(text).into_iter().filter(|w| w.chars().all(|c| c.is_ascii_alphanumeric() || c == '\'')).collect::<Vec<_>>().join(" ")
}

/// The words of a teach line that name the task: before the line's content (the colon of a step
/// line, or the first listed item of a needs line). Numbers and anchors are left out.
pub fn task_phrase(models: &Models, line: &str) -> Option<String> {
    let toks = tokenize(line);
    let tagger = if models.line_kind(line) == parse3::LineKind::Needs { &models.needs } else { &models.teach };
    let (tags, _) = tagger.tag(&toks, &BTreeSet::new());
    let end = toks.iter().position(|t| t == ":").or_else(|| tags.iter().position(|g| g.starts_with("B-NEED")))?;
    let ws: Vec<String> = toks[..end]
        .iter()
        .zip(&tags[..end])
        .filter(|(t, g)| *g == "O" && t.chars().all(|c| c.is_ascii_alphabetic() || c == '\''))
        .map(|(t, _)| t.clone())
        .collect();
    if ws.is_empty() { None } else { Some(ws.join(" ")) }
}

fn knowledge(vault: &Vault) -> BTreeMap<String, BTreeMap<String, f64>> {
    let mut k: BTreeMap<String, BTreeMap<String, f64>> = BTreeMap::new();
    for t in vault.tasks() {
        k.entry(t.clone()).or_default();
    }
    let mut step_task: BTreeMap<String, String> = BTreeMap::new();
    for f in vault.live() {
        if f.pred == "step" {
            step_task.insert(f.arg(1).to_string(), f.arg(0).to_string());
        }
    }
    let add = |k: &mut BTreeMap<String, BTreeMap<String, f64>>, task: &str, text: &str, w: f64| {
        let e = k.entry(task.to_string()).or_default();
        for s in text.split(|c: char| c == '_' || c == ' ').filter(|x| is_word(x)).map(stem) {
            *e.entry(s).or_insert(0.0) += w;
        }
    };
    for t in vault.tasks() {
        add(&mut k, &t, &t, 3.0);
    }
    for f in vault.live() {
        match f.pred.as_str() {
            "needs" | "goal" => add(&mut k, f.arg(0), f.arg(1), 1.0),
            "task_alias" => add(&mut k, f.arg(0), unquote(f.arg(1)), 2.0),
            "action" | "object_of" | "uses" => {
                if let Some(t) = step_task.get(f.arg(0)) {
                    let t = t.clone();
                    add(&mut k, &t, f.arg(1), 1.0);
                }
            }
            _ => {}
        }
    }
    k
}

/// Tasks ranked by score for a request, best first.
pub fn rank(vault: &Vault, request: &str) -> Vec<(String, f64)> {
    let k = knowledge(vault);
    let n = k.len() as f64;
    let mut df: BTreeMap<&str, f64> = BTreeMap::new();
    for ws in k.values() {
        for w in ws.keys() {
            *df.entry(w.as_str()).or_insert(0.0) += 1.0;
        }
    }
    // how specific each request word is, learned from the requests the agent has seen: a word
    // in many requests (chatter) weighs little, a word in few weighs fully
    let reqs: Vec<BTreeSet<String>> = vault
        .facts
        .iter()
        .filter(|(f, p)| p.retracted_by.is_none() && f.pred == "task_alias" && p.parser == "request")
        .map(|(f, _)| words(unquote(f.arg(1))).into_iter().collect())
        .collect();
    let r = reqs.len() as f64;
    let spec = |w: &str| -> f64 {
        if r < 5.0 {
            return 1.0;
        }
        let d = reqs.iter().filter(|s| s.contains(w)).count() as f64;
        (((r + 1.0) / (d + 0.5)).ln() / (r + 1.0).ln() / 0.9).clamp(0.0, 1.0).powi(2)
    };
    let rw: BTreeSet<String> = words(request).into_iter().collect();
    let mut out: Vec<(String, f64)> = k
        .iter()
        .map(|(t, ws)| {
            let s: f64 = rw
                .iter()
                .filter_map(|w| ws.get(w).map(|c| ((n + 1.0) / (df[w.as_str()] + 0.5)).ln() * c.min(3.0) * spec(w)))
                .sum();
            (t.clone(), s)
        })
        .collect();
    out.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap().then(a.0.cmp(&b.0)));
    out
}
