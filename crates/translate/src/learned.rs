//! Learned input patterns (Design 2.0, tier 2 of the translator).
//!
//! Every model translation that passes the validator is generalized into a
//! template: the words that spell an argument become slots
//! (`Anna likes tea.` + `tell: likes(anna, tea)` -> `{0} likes {1} .` ->
//! `tell: likes({0}, {1})`). A template is used once it has been seen
//! [`ACTIVE_AT`] times with the same translation and never with a different
//! one. Its output still goes through the validator. Sentences that lean on
//! context (pronouns, an argument that is not in the sentence) teach nothing.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;

/// Observations before a template is used.
pub const ACTIVE_AT: u32 = 2;
/// Longest span one slot may cover, in words.
const MAX_SLOT: usize = 4;

/// Words a slot never covers.
const FUNCTION: &[&str] = &[
    "the", "a", "an", "my", "your", "his", "her", "their", "our", "its", "some", "any", "and", "or", "but", "of", "to", "in", "on", "at", "for", "with",
    "is", "are", "was", "were", "be", "do", "does", "did", "not", "no", "yes", "i", "you", "he", "she", "it", "we", "they", "me", "him", "them", "us", "this",
    "that", "these", "those", "there", "here", "what", "who", "where", "when", "why", "how", "which", ".", "?", "!", ",", "very", "also", "too", "really",
];
/// Words that point to context: a sentence with one teaches nothing.
const CONTEXT: &[&str] = &["he", "she", "it", "they", "him", "her", "them", "his", "its", "their", "this", "that", "these", "those", "there", "one"];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Template {
    /// Act lines with `{i}` for slot i and `{speaker}` for the speaker.
    pub acts: Vec<String>,
    /// Per slot: how the surface word was inflected ("", "s", "es", "ies").
    pub inflect: Vec<String>,
    pub seen: u32,
    /// Seen with a different translation: never used.
    pub conflicted: bool,
    /// Per slot: the span lengths (in words) seen in training.
    #[serde(default)]
    pub lengths: Vec<Vec<usize>>,
    /// Per slot: the word class seen ("" for none, else the property an
    /// adjective names, such as "colour"). A slot only takes the same class.
    #[serde(default)]
    pub classes: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LearnedPatterns {
    pub templates: BTreeMap<String, Template>,
    #[serde(skip)]
    pub path: Option<PathBuf>,
    /// Word classes (adjective -> property), from the registry.
    #[serde(skip)]
    pub classes: BTreeMap<String, String>,
}

/// Words left out of templates: articles and fillers carry nothing the acts keep.
const DROP: &[&str] = &["a", "an", "the", "some", "well", "um", "uh", "erm", "oh", "ok", "okay", "so", "just", "actually", "really", "also"];

fn expand(w: &str) -> Vec<String> {
    let w = w.replace('\u{2019}', "'");
    let pairs: &[(&str, &[&str])] = &[
        ("doesn't", &["does", "not"]),
        ("don't", &["do", "not"]),
        ("didn't", &["did", "not"]),
        ("isn't", &["is", "not"]),
        ("aren't", &["are", "not"]),
        ("wasn't", &["was", "not"]),
        ("can't", &["can", "not"]),
        ("cannot", &["can", "not"]),
        ("won't", &["will", "not"]),
        ("i'm", &["i", "am"]),
        ("it's", &["it", "is"]),
        ("i've", &["i", "have"]),
        ("i'd", &["i", "would"]),
    ];
    for (k, v) in pairs {
        if w == *k {
            return v.iter().map(|s| s.to_string()).collect();
        }
    }
    vec![w]
}

fn tokens(s: &str) -> Vec<String> {
    let kept: Vec<String> = raw_tokens(s).into_iter().flat_map(|w| expand(&w)).filter(|w| !DROP.contains(&w.as_str())).collect();
    // No comma at the start or twice in a row (left by a dropped filler).
    let mut out: Vec<String> = Vec::new();
    for w in kept {
        if w == "," && out.last().map_or(true, |l| l == ",") {
            continue;
        }
        out.push(w);
    }
    out
}

fn raw_tokens(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    for w in s.split_whitespace() {
        let w = w.to_lowercase();
        let core = w.trim_end_matches(['.', '?', '!', ',', ';', ':']);
        let tail = &w[core.len()..];
        let core = core.trim_start_matches(['"', '(', '\'']).trim_end_matches(['"', ')']);
        if !core.is_empty() {
            out.push(core.to_string());
        }
        for c in tail.chars().filter(|c| matches!(c, '.' | '?' | '!' | ',')) {
            out.push(c.to_string());
        }
    }
    // A final mark distinguishes questions; "." is the default.
    if !out.last().map_or(false, |t| matches!(t.as_str(), "." | "?" | "!")) {
        out.push(".".into());
    }
    if out.last().map(String::as_str) == Some("!") {
        *out.last_mut().unwrap() = ".".into();
    }
    out
}

/// A single-symbol payload (`request: make_soup`).
fn payload(line: &str) -> Option<&str> {
    let (_, p) = line.split_once(':')?;
    let p = p.trim();
    (!p.is_empty() && !p.contains(['(', ' ', ';']) && p.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')).then_some(p)
}

/// Arguments of the atoms in an act line (`tell: likes(anna, tea); has(x, y)`),
/// or its single-symbol payload.
fn args_of(line: &str) -> Vec<String> {
    if let Some(p) = payload(line) {
        return vec![p.to_string()];
    }
    let mut out = Vec::new();
    let mut depth = 0;
    let mut cur = String::new();
    for c in line.chars() {
        match c {
            '(' => {
                depth += 1;
                cur.clear();
            }
            ',' | ')' if depth > 0 => {
                let a = cur.trim().to_string();
                if !a.is_empty() {
                    out.push(a);
                }
                cur.clear();
                if c == ')' {
                    depth -= 1;
                }
            }
            _ if depth > 0 => cur.push(c),
            _ => {}
        }
    }
    out
}

fn inflect(word: &str, how: &str) -> Option<String> {
    Some(match how {
        "" => word.to_string(),
        "s" => format!("{word}s"),
        "es" => format!("{word}es"),
        "ies" => format!("{}ies", word.strip_suffix('y')?),
        _ => return None,
    })
}

/// Symbols are singular: a slot's last word is made singular.
fn uninflect(word: &str, _how: &str) -> Option<String> {
    if word.len() <= 3 || word.chars().any(|c| c.is_ascii_digit()) || ["ss", "us", "is"].iter().any(|e| word.ends_with(e)) {
        return Some(word.to_string());
    }
    if !word.ends_with('s') {
        return Some(word.to_string());
    }
    if let Some(r) = word.strip_suffix("ies") {
        return Some(format!("{r}y"));
    }
    for e in ["ches", "shes", "sses", "xes", "zes", "oes"] {
        if word.ends_with(e) {
            return Some(word[..word.len() - 2].to_string());
        }
    }
    let r = word.strip_suffix('s')?;
    (!r.ends_with('s')).then(|| r.to_string())
}

/// Replace `{i}`/`{speaker}` in a line.
fn fill(line: &str, vals: &[String], speaker: &str) -> String {
    let mut s = line.replace("{speaker}", speaker);
    for (i, v) in vals.iter().enumerate() {
        s = s.replace(&format!("{{{i}}}"), v);
    }
    s
}

impl LearnedPatterns {
    pub fn load(path: PathBuf) -> Self {
        let mut p: LearnedPatterns = std::fs::read_to_string(&path).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default();
        p.path = Some(path);
        p
    }

    fn save(&self) {
        if let Some(p) = &self.path {
            if let Some(d) = p.parent() {
                let _ = std::fs::create_dir_all(d);
            }
            if let Ok(t) = serde_json::to_string_pretty(self) {
                let _ = std::fs::write(p, t);
            }
        }
    }

    /// Generalize a validated translation of one sentence.
    pub fn observe(&mut self, sentence: &str, lines: &[String], speaker: &str) {
        let Some((key, mut t)) = generalize(sentence, lines, speaker) else { return };
        t.classes = class_of_slots(&key, sentence, &t, &self.classes);
        match self.templates.get_mut(&key) {
            Some(old) if old.acts == t.acts && old.classes == t.classes => {
                old.seen += 1;
                for (i, l) in t.lengths.iter().enumerate() {
                    if let Some(o) = old.lengths.get_mut(i) {
                        for x in l {
                            if !o.contains(x) {
                                o.push(*x);
                            }
                        }
                    }
                }
            }
            Some(old) => old.conflicted = true,
            None => {
                self.templates.insert(key, t);
            }
        }
        self.save();
    }

    /// Templates in use.
    pub fn active(&self) -> usize {
        self.templates.values().filter(|t| t.seen >= ACTIVE_AT && !t.conflicted).count()
    }

    /// Act lines for a sentence, from an active template.
    pub fn apply(&self, sentence: &str, speaker: &str) -> Option<Vec<String>> {
        let toks = tokens(sentence);
        for (key, t) in &self.templates {
            if t.seen < ACTIVE_AT || t.conflicted {
                continue;
            }
            let pat: Vec<&str> = key.split(' ').collect();
            let mut spans = vec![(0usize, 0usize); t.inflect.len()];
            if !matches(&pat, &toks, 0, 0, &mut spans) {
                continue;
            }
            if spans.iter().enumerate().any(|(i, (a, b))| t.lengths.get(i).map_or(false, |l| !l.is_empty() && !l.contains(&(b - a)))) {
                continue;
            }
            if spans.iter().enumerate().any(|(i, (a, b))| {
                let word = toks[*a..*b].join("_");
                self.classes.get(&word).cloned().unwrap_or_default() != t.classes.get(i).cloned().unwrap_or_default()
            }) {
                continue;
            }
            let mut vals = Vec::new();
            for (i, (a, b)) in spans.iter().enumerate() {
                let mut words: Vec<String> = toks[*a..*b].to_vec();
                let last = words.pop()?;
                words.push(uninflect(&last, &t.inflect[i])?);
                vals.push(words.join("_"));
            }
            return Some(t.acts.iter().map(|l| fill(l, &vals, speaker)).collect());
        }
        None
    }
}

/// The word class of each slot in a training sentence.
fn class_of_slots(key: &str, sentence: &str, t: &Template, classes: &BTreeMap<String, String>) -> Vec<String> {
    let toks = tokens(sentence);
    let pat: Vec<&str> = key.split(' ').collect();
    let mut spans = vec![(0usize, 0usize); t.inflect.len()];
    if !matches(&pat, &toks, 0, 0, &mut spans) {
        return vec![String::new(); t.inflect.len()];
    }
    spans.iter().map(|(a, b)| classes.get(&toks[*a..*b].join("_")).cloned().unwrap_or_default()).collect()
}

/// Match pattern tokens against sentence tokens; slots record their spans.
fn matches(pat: &[&str], toks: &[String], pi: usize, ti: usize, spans: &mut Vec<(usize, usize)>) -> bool {
    if pi == pat.len() {
        return ti == toks.len();
    }
    let p = pat[pi];
    if let Some(n) = p.strip_prefix('{').and_then(|r| r.strip_suffix('}')).and_then(|r| r.parse::<usize>().ok()) {
        for len in 1..=MAX_SLOT {
            if ti + len > toks.len() || toks[ti..ti + len].iter().any(|w| FUNCTION.contains(&w.as_str())) {
                break;
            }
            if n < spans.len() {
                spans[n] = (ti, ti + len);
            }
            if matches(pat, toks, pi + 1, ti + len, spans) {
                return true;
            }
        }
        return false;
    }
    ti < toks.len() && toks[ti] == p && matches(pat, toks, pi + 1, ti + 1, spans)
}

/// The template for one translated sentence, if it generalizes.
pub fn generalize(sentence: &str, lines: &[String], speaker: &str) -> Option<(String, Template)> {
    let toks = tokens(sentence);
    if toks.iter().any(|t| CONTEXT.contains(&t.as_str())) || lines.is_empty() {
        return None;
    }
    // Distinct arguments, longest first so `ice_cream` wins over `cream`.
    let mut args: Vec<String> = lines.iter().flat_map(|l| args_of(l)).filter(|a| a != speaker).collect();
    args.sort_by_key(|a| std::cmp::Reverse(a.len()));
    args.dedup();
    let mut slot_of: Vec<Option<usize>> = vec![None; toks.len()];
    let mut slots: Vec<(String, String)> = Vec::new(); // (symbol, inflection)
    for a in &args {
        if a.starts_with('"') {
            return None;
        }
        let parts: Vec<&str> = a.split('_').collect();
        let mut found = false;
        'how: for how in ["", "s", "es", "ies"] {
            let Some(last) = inflect(parts[parts.len() - 1], how) else { continue };
            let mut want: Vec<String> = parts[..parts.len() - 1].iter().map(|s| s.to_string()).collect();
            want.push(last);
            for i in 0..toks.len().saturating_sub(want.len() - 1) {
                if toks[i..i + want.len()] == want[..] && slot_of[i..i + want.len()].iter().all(Option::is_none) {
                    if want.iter().any(|w| FUNCTION.contains(&w.as_str())) {
                        return None;
                    }
                    let n = slots.len();
                    for s in &mut slot_of[i..i + want.len()] {
                        *s = Some(n);
                    }
                    slots.push((a.clone(), how.to_string()));
                    found = true;
                    break 'how;
                }
            }
        }
        // Every argument must come from the sentence (numbers too).
        if !found {
            return None;
        }
    }
    // Number slots by where they are in the sentence.
    let mut order: Vec<usize> = Vec::new();
    for s in slot_of.iter().flatten() {
        if !order.contains(s) {
            order.push(*s);
        }
    }
    let renum = |old: usize| order.iter().position(|&o| o == old).unwrap();
    for s in slot_of.iter_mut().flatten() {
        *s = renum(*s);
    }
    let mut sorted = vec![(String::new(), String::new()); slots.len()];
    for (i, x) in slots.into_iter().enumerate() {
        sorted[renum(i)] = x;
    }
    let slots = sorted;
    let mut key = Vec::new();
    let mut i = 0;
    while i < toks.len() {
        match slot_of[i] {
            Some(n) => {
                key.push(format!("{{{n}}}"));
                while i < toks.len() && slot_of[i] == Some(n) {
                    i += 1;
                }
            }
            None => {
                key.push(toks[i].clone());
                i += 1;
            }
        }
    }
    // At least one fixed word that is not punctuation.
    if !key.iter().any(|k| !k.starts_with('{') && !matches!(k.as_str(), "." | "?" | "!" | ",")) {
        return None;
    }
    // Act lines with slots: replace whole arguments only.
    let acts = lines
        .iter()
        .map(|l| {
            if let Some(p) = payload(l) {
                if let Some(n) = slots.iter().position(|(s, _)| s == p) {
                    let head = &l[..l.find(':').unwrap()];
                    return format!("{head}: {{{n}}}");
                }
                return l.clone();
            }
            let mut out = String::new();
            let mut depth = 0;
            let mut cur = String::new();
            for c in l.chars() {
                match c {
                    '(' => {
                        depth += 1;
                        out.push(c);
                        cur.clear();
                    }
                    ',' | ')' if depth > 0 => {
                        let a = cur.trim();
                        let lead = &cur[..cur.len() - cur.trim_start().len()];
                        let rep = if a == speaker { "{speaker}".to_string() } else { slots.iter().position(|(s, _)| s == a).map(|n| format!("{{{n}}}")).unwrap_or_else(|| a.to_string()) };
                        out.push_str(lead);
                        out.push_str(&rep);
                        out.push(c);
                        cur.clear();
                        if c == ')' {
                            depth -= 1;
                        }
                    }
                    _ if depth > 0 => cur.push(c),
                    _ => out.push(c),
                }
            }
            out
        })
        .collect();
    let mut lengths = vec![Vec::new(); slots.len()];
    let mut i = 0;
    while i < slot_of.len() {
        if let Some(n) = slot_of[i] {
            let start = i;
            while i < slot_of.len() && slot_of[i] == Some(n) {
                i += 1;
            }
            lengths[n].push(i - start);
        } else {
            i += 1;
        }
    }
    Some((key.join(" "), Template { acts, inflect: slots.into_iter().map(|s| s.1).collect(), seen: 1, conflicted: false, lengths, classes: Vec::new() }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn l(s: &str) -> Vec<String> {
        vec![s.to_string()]
    }

    #[test]
    fn generalizes_and_applies() {
        let mut p = LearnedPatterns::default();
        p.observe("Anna likes tea.", &l("tell: likes(anna, tea)"), "maksim");
        assert_eq!(p.apply("Bob likes coffee.", "maksim"), None, "seen once: not used yet");
        p.observe("Bob adores cherries.", &l("tell: likes(bob, cherry)"), "maksim");
        p.observe("Tom likes jazz.", &l("tell: likes(tom, jazz)"), "maksim");
        assert_eq!(p.apply("Mary Jane likes ice cream.", "maksim"), None, "two-word slots not seen yet");
        p.observe("Anna Lee likes ice tea.", &l("tell: likes(anna_lee, ice_tea)"), "maksim");
        assert_eq!(p.apply("Mary Jane likes ice cream.", "maksim"), Some(l("tell: likes(mary_jane, ice_cream)")));
        assert_eq!(p.apply("Well, Mary Jane likes the tea.", "maksim"), Some(l("tell: likes(mary_jane, tea)")), "articles and fillers are left out");
        assert_eq!(p.apply("Mary Jane likes my tea.", "maksim"), None, "slots skip function words");
        assert_eq!(p.apply("Does Anna like tea?", "maksim"), None);
    }

    #[test]
    fn slot_lengths_and_classes() {
        let mut p = LearnedPatterns::default();
        p.classes.insert("blue".into(), "colour".into());
        p.classes.insert("red".into(), "colour".into());
        p.observe("No, Rex is a dog.", &l("correct: is_a(rex, dog)"), "maksim");
        p.observe("No, Tom is a cat.", &l("correct: is_a(tom, cat)"), "maksim");
        assert_eq!(p.apply("No, the car is blue.", "maksim"), None, "a colour is not a category");
        p.observe("Can you help me make soup?", &l("request: make_soup"), "maksim");
        p.observe("Can you help me bake cookies?", &l("request: bake_cookie"), "maksim");
        assert_eq!(p.apply("Can you help me fix a leaky faucet?", "maksim"), None, "three words where two were seen");
    }

    #[test]
    fn task_names_are_slots() {
        let mut p = LearnedPatterns::default();
        p.observe("Can you help me make soup?", &l("request: make_soup"), "maksim");
        p.observe("Can you help me bake cookies?", &l("request: bake_cookie"), "maksim");
        assert_eq!(p.apply("Can you help me wash a car?", "maksim"), Some(l("request: wash_car")));
        assert_eq!(p.apply("Actually, can you help me plant trees?", "maksim"), Some(l("request: plant_tree")));
    }

    #[test]
    fn the_speaker_and_inflection() {
        let mut p = LearnedPatterns::default();
        p.observe("I hate mushrooms.", &l("tell: -likes(maksim, mushroom)"), "maksim");
        p.observe("I hate spiders.", &l("tell: -likes(maksim, spider)"), "maksim");
        assert_eq!(p.apply("I hate cherries.", "anna"), Some(l("tell: -likes(anna, cherry)")));
        assert_eq!(p.apply("I hate rain.", "anna"), Some(l("tell: -likes(anna, rain)")));
        assert_eq!(p.apply("I hate onions.", "anna"), Some(l("tell: -likes(anna, onion)")));
    }

    #[test]
    fn context_and_conflicts_teach_nothing() {
        let mut p = LearnedPatterns::default();
        p.observe("She likes tea.", &l("tell: likes(anna, tea)"), "maksim");
        assert!(p.templates.is_empty());
        p.observe("Rex runs fast.", &l("tell: property(rex, speed, fast)"), "maksim");
        p.observe("Bo runs fast.", &l("tell: likes(bo, running)"), "maksim");
        p.observe("Kim runs fast.", &l("tell: property(kim, speed, fast)"), "maksim");
        assert_eq!(p.apply("Al runs fast.", "maksim"), None);
    }
}
