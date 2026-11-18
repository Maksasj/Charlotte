//! Charlotte 3.0 learned-model runtime (decision D3/D16).
//!
//! Loads the averaged perceptrons trained offline by `tools/design3/train_parser.py` and
//! applies them. The feature functions mirror the Python ones exactly (parity test). No word
//! lists: every word-level decision comes from the weights and the learned symbol forms.

use kb3::Fact;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::Path;

/// Lower-case tokens: runs of letters/digits (with an internal apostrophe part), or one
/// non-space symbol. Mirrors `tokenize` in tools/design3/gt3.py.
pub fn tokenize(text: &str) -> Vec<String> {
    let low = text.to_lowercase();
    let cs: Vec<char> = low.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    let word = |c: char| c.is_ascii_lowercase() || c.is_ascii_digit();
    while i < cs.len() {
        let c = cs[i];
        if word(c) {
            let mut j = i;
            while j < cs.len() && word(cs[j]) {
                j += 1;
            }
            if j + 1 < cs.len() && cs[j] == '\'' && cs[j + 1].is_ascii_lowercase() {
                j += 1;
                while j < cs.len() && cs[j].is_ascii_lowercase() {
                    j += 1;
                }
            }
            out.push(cs[i..j].iter().collect());
            i = j;
        } else if c.is_whitespace() {
            i += 1;
        } else {
            out.push(c.to_string());
            i += 1;
        }
    }
    out
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TaggerModel {
    pub tags: Vec<String>,
    pub w: HashMap<String, HashMap<String, f64>>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Forms {
    pub exact: HashMap<String, String>,
    /// suffix -> (strip, add)
    pub suffix: HashMap<String, (usize, String)>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Models {
    pub version: u32,
    pub teach: TaggerModel,
    pub situation: TaggerModel,
    pub detect: HashMap<String, f64>,
    pub forms: Forms,
    #[serde(default)]
    pub fallback_pairs: usize,
}

fn tf(b: bool) -> &'static str {
    if b { "True" } else { "False" }
}

/// Mirrors `tag_feats` in train_parser.py.
pub fn tag_feats(toks: &[String], i: usize, prev: &str, prev2: &str, known: &BTreeSet<String>) -> Vec<String> {
    let w = toks[i].as_str();
    let n = toks.len();
    let suf = |k: usize| -> String {
        let cs: Vec<char> = w.chars().collect();
        cs[cs.len().saturating_sub(k)..].iter().collect()
    };
    let mut f = vec![
        "b".to_string(),
        format!("w={w}"),
        format!("s3={}", suf(3)),
        format!("s2={}", suf(2)),
        format!("p={prev}"),
        format!("pp={prev}|{prev2}"),
        format!("d={}", tf(!w.is_empty() && w.chars().all(|c| c.is_ascii_digit()))),
    ];
    for k in [-3i64, -2, -1, 1, 2] {
        let j = i as i64 + k;
        let t = if j < 0 { "<s>" } else if j as usize >= n { "</s>" } else { toks[j as usize].as_str() };
        f.push(format!("w{k}={t}"));
    }
    f.push(format!("w-1w={}|{w}", if i > 0 { toks[i - 1].as_str() } else { "<s>" }));
    f.push(format!("ww+1={w}|{}", if i + 1 < n { toks[i + 1].as_str() } else { "</s>" }));
    let colon = toks.iter().position(|t| t == ":");
    let after = matches!(colon, Some(c) if i > c);
    f.push(format!("aftercolon={}", tf(after)));
    if let (true, Some(c)) = (after, colon) {
        f.push(format!("cpos={}", (i - c).min(6)));
    }
    f.push(format!("p+w={prev}|{w}"));
    if !known.is_empty() {
        let kn = known.contains(w);
        f.push(format!("known={}", tf(kn)));
        f.push(format!("known+p={}|{prev}", tf(kn)));
        f.push(format!("known-1={}", tf(i > 0 && known.contains(&toks[i - 1]))));
    }
    f
}

/// Mirrors `bow_feats`.
pub fn bow_feats(toks: &[String]) -> Vec<String> {
    if toks.is_empty() {
        return vec!["b".into()];
    }
    let mut f = vec!["b".to_string()];
    f.extend(toks.iter().map(|t| format!("w={t}")));
    f.push(format!("f={}", toks[0]));
    f.push(format!("l={}", toks[toks.len() - 1]));
    f.extend(toks.windows(2).map(|p| format!("bg={}|{}", p[0], p[1])));
    f
}

impl TaggerModel {
    fn scores(&self, fs: &[String]) -> Vec<(f64, usize)> {
        let mut s = vec![0.0f64; self.tags.len()];
        for (k, t) in self.tags.iter().enumerate() {
            for f in fs {
                if let Some(d) = self.w.get(f) {
                    if let Some(v) = d.get(t) {
                        s[k] += v;
                    }
                }
            }
        }
        s.into_iter().enumerate().map(|(k, v)| (v, k)).collect()
    }

    /// Tags plus the minimum margin (best minus second-best score) over the tokens.
    pub fn tag(&self, toks: &[String], known: &BTreeSet<String>) -> (Vec<String>, f64) {
        let mut out = Vec::with_capacity(toks.len());
        let (mut prev, mut prev2) = ("<s>".to_string(), "<s>".to_string());
        let mut margin = f64::INFINITY;
        for i in 0..toks.len() {
            let sc = self.scores(&tag_feats(toks, i, &prev, &prev2, known));
            // first maximum in tag order (as Python's strict `>` over sorted tags)
            let mut best = 0;
            for k in 1..sc.len() {
                if sc[k].0 > sc[best].0 {
                    best = k;
                }
            }
            let second = sc.iter().enumerate().filter(|(k, _)| *k != best).map(|(_, s)| s.0).fold(f64::NEG_INFINITY, f64::max);
            margin = margin.min(sc[best].0 - second);
            let t = self.tags.get(best).cloned().unwrap_or_else(|| "O".into());
            prev2 = std::mem::replace(&mut prev, t.clone());
            out.push(t);
        }
        (out, if margin.is_finite() { margin } else { 0.0 })
    }
}

impl Forms {
    pub fn symbol(&self, toks: &[String]) -> String {
        toks.iter()
            .map(|t| {
                if let Some(w) = self.exact.get(t) {
                    return w.clone();
                }
                let cs: Vec<char> = t.chars().collect();
                for l in [3usize, 2, 1] {
                    if cs.len() > l {
                        let suf: String = cs[cs.len() - l..].iter().collect();
                        if let Some((strip, add)) = self.suffix.get(&suf) {
                            if cs.len() >= strip + 2 {
                                let stem: String = cs[..cs.len() - strip].iter().collect();
                                return format!("{stem}{add}");
                            }
                            return t.clone();
                        }
                    }
                }
                t.clone()
            })
            .collect::<Vec<_>>()
            .join("_")
    }
}

/// Spans of a tagged line, by role, in order.
pub fn spans(toks: &[String], tags: &[String]) -> BTreeMap<String, Vec<Vec<String>>> {
    let mut out: BTreeMap<String, Vec<Vec<String>>> = BTreeMap::new();
    let mut cur: Option<(String, usize)> = None;
    for (t, g) in toks.iter().zip(tags) {
        if let Some(r) = g.strip_prefix("B-") {
            let v = out.entry(r.to_string()).or_default();
            v.push(vec![t.clone()]);
            cur = Some((r.to_string(), v.len() - 1));
        } else if let (Some(r), Some((cr, k))) = (g.strip_prefix("I-"), cur.as_ref()) {
            if r == cr {
                out.get_mut(cr).unwrap()[*k].push(t.clone());
            } else {
                cur = None;
            }
        } else {
            cur = None;
        }
    }
    out
}

/// What one teaching line says, as schema facts for `task`.
#[derive(Debug, Clone, Default)]
pub struct Parsed {
    pub facts: Vec<Fact>,
    pub confidence: f64,
    pub kind: LineKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineKind {
    Step,
    Needs,
    Time,
    #[default]
    Other,
}

impl Models {
    pub fn load(path: &Path) -> Result<Models, String> {
        let t = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        serde_json::from_str(&t).map_err(|e| format!("{}: {e}", path.display()))
    }

    pub fn default_path() -> std::path::PathBuf {
        kb3::repo_root().join("data/training/parser3/models/current.json")
    }

    /// Request (true) or statement (false).
    pub fn is_request(&self, text: &str) -> bool {
        bow_feats(&tokenize(text)).iter().map(|f| self.detect.get(f).copied().unwrap_or(0.0)).sum::<f64>() > 0.0
    }

    /// Item words the agent already knows for this task (from its parsed needs line).
    pub fn known_words(&self, needs_line: &str) -> BTreeSet<String> {
        let toks = tokenize(needs_line);
        let (tags, _) = self.teach.tag(&toks, &BTreeSet::new());
        toks.iter().zip(&tags).filter(|(_, g)| g.ends_with("NEED")).map(|(t, _)| t.clone()).collect()
    }

    /// Parse one teach line of `task`. `known` comes from the task's needs line.
    pub fn parse_teach(&self, task: &str, line: &str, known: &BTreeSet<String>) -> Parsed {
        let toks = tokenize(line);
        let (tags, margin) = self.teach.tag(&toks, known);
        let sp = spans(&toks, &tags);
        let sym = |r: &str| -> Vec<String> { sp.get(r).map(|v| v.iter().map(|s| self.forms.symbol(s)).collect()).unwrap_or_default() };
        let raw = |r: &str| -> Vec<String> { sp.get(r).map(|v| v.iter().map(|s| s.concat()).collect()).unwrap_or_default() };
        let mut facts = Vec::new();
        let colon = toks.iter().position(|t| t == ":");
        let ovl: BTreeSet<String> = raw("OVL").into_iter().collect();
        // the step number: the first number before the colon that is not an overlap anchor
        let num = colon.and_then(|c| {
            toks[..c].iter().zip(&tags[..c]).find(|(t, g)| t.chars().all(|c| c.is_ascii_digit()) && !g.ends_with("OVL")).map(|(t, _)| t.clone())
        });
        let kind;
        if let (Some(n), Some(act)) = (num.clone(), sym("ACT").first().cloned()) {
            kind = LineKind::Step;
            let s = format!("{task}_s{n}");
            facts.push(Fact::new("step", &[task, &s]));
            facts.push(Fact::new("action", &[&s, &act]));
            if let Some(o) = sym("OBJ").first() {
                facts.push(Fact::new("object_of", &[&s, o]));
            }
            for u in sym("USE") {
                facts.push(Fact::new("uses", &[&s, &u]));
            }
            for (tag, pred) in [("INTO", "into"), ("ONTO", "onto"), ("FROM", "from")] {
                for i in sym(tag) {
                    facts.push(Fact::new(pred, &[&s, &i]));
                }
            }
            if let Some(u) = sym("UNTIL").first() {
                facts.push(Fact::new("until", &[&s, u]));
            }
            if let Some(f) = self.time_fact(&s, &raw("TMIN"), &raw("TMAX"), &sym("UNIT")) {
                facts.push(f);
            }
            for o in &ovl {
                facts.push(Fact::new("overlaps", &[&s, &format!("{task}_s{o}")]));
            }
        } else if sp.contains_key("NEED") {
            kind = LineKind::Needs;
            for i in sym("NEED") {
                facts.push(Fact::new("needs", &[task, &i]));
            }
        } else if let Some(f) = self.time_fact(task, &raw("TMIN"), &raw("TMAX"), &sym("UNIT")) {
            kind = LineKind::Time;
            facts.push(f);
        } else {
            kind = LineKind::Other;
        }
        Parsed { facts, confidence: margin, kind }
    }

    fn time_fact(&self, subject: &str, lo: &[String], hi: &[String], unit: &[String]) -> Option<Fact> {
        let lo = lo.first()?;
        let hi = hi.first().unwrap_or(lo);
        let unit = unit.first()?;
        Some(Fact::new("takes_time", &[subject, lo, hi, unit]))
    }

    /// Situation facts (`has`, `lacks`, `wants`, `excludes`, `time_limit`, `count`) in a text.
    pub fn parse_situation(&self, text: &str) -> (Vec<Fact>, f64) {
        let toks = tokenize(text);
        let (tags, margin) = self.situation.tag(&toks, &BTreeSet::new());
        let sp = spans(&toks, &tags);
        let mut out = Vec::new();
        for (tag, pred) in [("HAS", "has"), ("LACKS", "lacks"), ("WANTS", "wants"), ("EXCL", "excludes")] {
            for s in sp.get(tag).into_iter().flatten() {
                out.push(Fact::new(pred, &["user", &self.forms.symbol(s)]));
            }
        }
        for s in sp.get("TIME").into_iter().flatten() {
            // the unit is the learned form of the token after the number
            let i = toks.iter().position(|t| *t == s[0]).unwrap_or(0);
            let unit = toks.get(i + 1).map(|u| self.forms.symbol(std::slice::from_ref(u))).unwrap_or_default();
            out.push(Fact::new("time_limit", &["user", &s.concat(), &unit]));
        }
        (out, margin)
    }
}
