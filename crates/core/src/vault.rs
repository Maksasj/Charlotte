//! The file-based vault.
//!
//! ```text
//! vault/facts/<entity>.md   facts about one entity (first argument)
//! vault/rules/<id>.md       one rule group with front matter
//! vault/core/*.md           built-in ontology, read-only for the learner
//! vault/log/<date>.md       append-only change log
//! ```

use crate::doc::{BlockKind, Document, Item};
use crate::term::{fmt_conf, Fact, Rule, Value};
use crate::time::Timestamp;
use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleStatus {
    Hypothesis,
    Confirmed,
    Rejected,
}

impl RuleStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            RuleStatus::Hypothesis => "hypothesis",
            RuleStatus::Confirmed => "confirmed",
            RuleStatus::Rejected => "rejected",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim() {
            "hypothesis" => Some(RuleStatus::Hypothesis),
            "confirmed" => Some(RuleStatus::Confirmed),
            "rejected" => Some(RuleStatus::Rejected),
            _ => None,
        }
    }
}

/// Ordered `key: value` front matter, written between `---` lines.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FrontMatter {
    pub entries: Vec<(String, String)>,
    /// Original text (including the `---` lines and newlines) while unmodified.
    raw: Option<String>,
}

impl FrontMatter {
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
    }

    pub fn set(&mut self, key: &str, value: impl Into<String>) {
        let value = value.into();
        if self.get(key) == Some(value.as_str()) {
            return;
        }
        self.raw = None;
        match self.entries.iter_mut().find(|(k, _)| k == key) {
            Some(e) => e.1 = value,
            None => self.entries.push((key.to_string(), value)),
        }
    }

    fn render(&self, nl: &str) -> String {
        if let Some(r) = &self.raw {
            return r.clone();
        }
        let mut s = format!("---{nl}");
        for (k, v) in &self.entries {
            s.push_str(&format!("{k}: {v}{nl}"));
        }
        s.push_str(&format!("---{nl}"));
        s
    }

    /// Split `src` into front matter and the remaining body.
    fn split(src: &str) -> (Option<FrontMatter>, &str) {
        let nl = if src.contains("\r\n") { "\r\n" } else { "\n" };
        let open = format!("---{nl}");
        if !src.starts_with(&open) {
            return (None, src);
        }
        let mut pos = open.len();
        let mut entries = Vec::new();
        loop {
            let rest = &src[pos..];
            let (line, adv) = match rest.find(nl) {
                Some(i) => (&rest[..i], i + nl.len()),
                None => (rest, rest.len()),
            };
            if line == "---" {
                pos += adv;
                let raw = src[..pos].to_string();
                return (Some(FrontMatter { entries, raw: Some(raw) }), &src[pos..]);
            }
            if adv == 0 {
                return (None, src); // unterminated: treat as plain text
            }
            if let Some((k, v)) = line.split_once(':') {
                entries.push((k.trim().to_string(), v.trim().to_string()));
            }
            pos += adv;
        }
    }
}

/// Metadata of a rule file, read from front matter.
#[derive(Debug, Clone, PartialEq)]
pub struct RuleMeta {
    pub id: String,
    pub status: RuleStatus,
    pub conf: f64,
    /// Supporting cases seen.
    pub evidence: u64,
    /// Counterexamples seen.
    pub counter: u64,
    pub module: String,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuleFile {
    pub front: FrontMatter,
    pub doc: Document,
}

impl RuleFile {
    pub fn new(meta: &RuleMeta, note: &str, rules: &[Rule]) -> Self {
        let mut front = FrontMatter::default();
        Self::write_meta(&mut front, meta);
        let mut doc = Document::parse(&format!("# {}\n\n{}\n", meta.id, note.trim()), BlockKind::Rules);
        for r in rules {
            doc.push(Item::Rule(r.clone()));
        }
        RuleFile { front, doc }
    }

    fn write_meta(front: &mut FrontMatter, m: &RuleMeta) {
        front.set("id", m.id.clone());
        front.set("status", m.status.as_str());
        front.set("conf", fmt_conf(m.conf));
        front.set("evidence", m.evidence.to_string());
        front.set("counter", m.counter.to_string());
        front.set("module", m.module.clone());
        front.set("source", m.source.clone());
    }

    pub fn parse(src: &str, fallback_id: &str) -> Self {
        let (front, body) = FrontMatter::split(src);
        let front = front.unwrap_or_else(|| {
            let mut f = FrontMatter::default();
            f.set("id", fallback_id);
            // No front matter in the source: rendering will add it.
            f
        });
        RuleFile { front, doc: Document::parse(body, BlockKind::Rules) }
    }

    pub fn render(&self) -> String {
        format!("{}{}", self.front.render(&self.doc.newline), self.doc.render())
    }

    pub fn meta(&self) -> RuleMeta {
        let f = &self.front;
        RuleMeta {
            id: f.get("id").unwrap_or("").to_string(),
            status: f.get("status").and_then(RuleStatus::parse).unwrap_or(RuleStatus::Confirmed),
            conf: f.get("conf").and_then(|c| c.parse().ok()).unwrap_or(1.0),
            evidence: f.get("evidence").and_then(|c| c.parse().ok()).unwrap_or(0),
            counter: f.get("counter").and_then(|c| c.parse().ok()).unwrap_or(0),
            module: f.get("module").unwrap_or("general").to_string(),
            source: f.get("source").unwrap_or("user").to_string(),
        }
    }

    pub fn set_meta(&mut self, m: &RuleMeta) {
        Self::write_meta(&mut self.front, m);
    }

    pub fn rules(&self) -> Vec<Rule> {
        self.doc.rules().cloned().collect()
    }

    /// The plain-English note: body text before the first block, without the title.
    pub fn note(&self) -> String {
        let mut out = String::new();
        for seg in &self.doc.segments {
            match seg {
                crate::doc::Segment::Text(t) => {
                    for l in t.lines() {
                        if !l.starts_with('#') && !l.trim().is_empty() {
                            if !out.is_empty() {
                                out.push(' ');
                            }
                            out.push_str(l.trim());
                        }
                    }
                }
                crate::doc::Segment::Block(_) => break,
            }
        }
        out
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FactFile {
    pub doc: Document,
}

impl FactFile {
    pub fn new(entity: &str) -> Self {
        FactFile { doc: Document::parse(&format!("# {entity}\n"), BlockKind::Facts) }
    }
    pub fn parse(src: &str) -> Self {
        FactFile { doc: Document::parse(src, BlockKind::Facts) }
    }
    pub fn render(&self) -> String {
        self.doc.render()
    }
}

/// File stem for an entity name: lowercase, `[a-z0-9_-]` only.
pub fn file_stem(name: &str) -> String {
    let s: String = name
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '_' || c == '-' { c.to_ascii_lowercase() } else { '_' })
        .collect();
    if s.is_empty() {
        "_".into()
    } else {
        s
    }
}

/// Which fact file a fact belongs to: its first argument.
pub fn entity_of(f: &Fact) -> String {
    match f.atom.args.first() {
        Some(Value::Sym(s)) => file_stem(s),
        Some(Value::Int(_)) => "_numbers".into(),
        None => "_global".into(),
    }
}

#[derive(Debug, Default)]
pub struct Vault {
    pub root: PathBuf,
    /// Entity stem -> file.
    pub facts: BTreeMap<String, FactFile>,
    /// Rule id (file stem) -> file.
    pub rules: BTreeMap<String, RuleFile>,
    /// Core file stem -> file (rule format).
    pub core: BTreeMap<String, RuleFile>,
    /// Files changed since load, as (dir, stem).
    dirty: std::collections::BTreeSet<(String, String)>,
}

fn read_md_dir(dir: &Path) -> io::Result<Vec<(String, String)>> {
    let mut out = Vec::new();
    if !dir.exists() {
        return Ok(out);
    }
    for e in fs::read_dir(dir)? {
        let p = e?.path();
        if p.extension().and_then(|x| x.to_str()) == Some("md") {
            let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string();
            out.push((stem, fs::read_to_string(&p)?));
        }
    }
    out.sort();
    Ok(out)
}

impl Vault {
    /// An empty in-memory vault rooted at `root` (nothing read).
    pub fn empty(root: impl Into<PathBuf>) -> Self {
        Vault { root: root.into(), ..Default::default() }
    }

    pub fn load(root: impl Into<PathBuf>) -> io::Result<Self> {
        let mut v = Vault::empty(root);
        for (stem, src) in read_md_dir(&v.root.join("facts"))? {
            v.facts.insert(stem, FactFile::parse(&src));
        }
        for (stem, src) in read_md_dir(&v.root.join("rules"))? {
            let rf = RuleFile::parse(&src, &stem);
            v.rules.insert(stem, rf);
        }
        for (stem, src) in read_md_dir(&v.root.join("core"))? {
            let rf = RuleFile::parse(&src, &stem);
            v.core.insert(stem, rf);
        }
        Ok(v)
    }

    /// Re-read everything from disk, dropping unsaved changes.
    pub fn reload(&mut self) -> io::Result<()> {
        *self = Vault::load(self.root.clone())?;
        Ok(())
    }

    pub fn mark_fact_dirty(&mut self, stem: &str) {
        self.dirty.insert(("facts".into(), stem.into()));
    }

    pub fn mark_rule_dirty(&mut self, stem: &str) {
        self.dirty.insert(("rules".into(), stem.into()));
    }

    /// Write changed files. Rule files that were removed are deleted on disk.
    pub fn save(&mut self) -> io::Result<()> {
        for (dir, stem) in std::mem::take(&mut self.dirty) {
            let d = self.root.join(&dir);
            fs::create_dir_all(&d)?;
            let path = d.join(format!("{stem}.md"));
            let content = match dir.as_str() {
                "facts" => self.facts.get(&stem).map(|f| f.render()),
                "rules" => self.rules.get(&stem).map(|r| r.render()),
                _ => None,
            };
            match content {
                Some(c) => write_if_changed(&path, &c)?,
                None => {
                    if path.exists() {
                        fs::remove_file(&path)?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Write every file (used when creating a vault from scratch).
    pub fn save_all(&mut self) -> io::Result<()> {
        let fact_keys: Vec<String> = self.facts.keys().cloned().collect();
        for k in fact_keys {
            self.mark_fact_dirty(&k);
        }
        let rule_keys: Vec<String> = self.rules.keys().cloned().collect();
        for k in rule_keys {
            self.mark_rule_dirty(&k);
        }
        let core_dir = self.root.join("core");
        fs::create_dir_all(&core_dir)?;
        for (stem, rf) in &self.core {
            write_if_changed(&core_dir.join(format!("{stem}.md")), &rf.render())?;
        }
        fs::create_dir_all(self.root.join("log"))?;
        self.save()
    }

    /// All facts with their file stem and position (for updates).
    pub fn all_facts(&self) -> Vec<(String, Fact)> {
        let mut out = Vec::new();
        for (stem, ff) in &self.facts {
            for f in ff.doc.facts() {
                out.push((stem.clone(), f.clone()));
            }
        }
        out
    }

    /// Store a new fact in its entity file.
    pub fn add_fact(&mut self, fact: Fact) {
        let stem = entity_of(&fact);
        self.facts.entry(stem.clone()).or_insert_with(|| FactFile::new(&stem)).doc.push(Item::Fact(fact));
        self.mark_fact_dirty(&stem);
    }

    /// Apply `f` to every stored fact; returns how many changed.
    pub fn update_facts(&mut self, mut f: impl FnMut(&mut Fact) -> bool) -> usize {
        let mut total = 0;
        let mut changed = Vec::new();
        for (stem, ff) in self.facts.iter_mut() {
            let n = ff.doc.update_items(|i| match i {
                Item::Fact(fact) => f(fact),
                Item::Rule(_) => false,
            });
            if n > 0 {
                changed.push(stem.clone());
                total += n;
            }
        }
        for s in changed {
            self.mark_fact_dirty(&s);
        }
        total
    }

    pub fn put_rule_file(&mut self, rf: RuleFile) {
        let stem = file_stem(&rf.meta().id);
        self.rules.insert(stem.clone(), rf);
        self.mark_rule_dirty(&stem);
    }

    pub fn remove_rule_file(&mut self, id: &str) -> Option<RuleFile> {
        let stem = file_stem(id);
        let r = self.rules.remove(&stem);
        if r.is_some() {
            self.mark_rule_dirty(&stem);
        }
        r
    }

    /// All load errors: (file, line, error).
    pub fn errors(&self) -> Vec<(String, String, String)> {
        let mut out = Vec::new();
        for (s, f) in &self.facts {
            for (l, e) in f.doc.errors() {
                out.push((format!("facts/{s}.md"), l, e));
            }
        }
        for (s, f) in &self.rules {
            for (l, e) in f.doc.errors() {
                out.push((format!("rules/{s}.md"), l, e));
            }
        }
        for (s, f) in &self.core {
            for (l, e) in f.doc.errors() {
                out.push((format!("core/{s}.md"), l, e));
            }
        }
        out
    }

    /// Append a line to today's log file: `- <time> | <action> | <subject> | <reason>`.
    pub fn log(&self, at: Timestamp, action: &str, subject: &str, reason: &str) -> io::Result<()> {
        let dir = self.root.join("log");
        fs::create_dir_all(&dir)?;
        let (y, m, d) = at.ymd();
        let path = dir.join(format!("{y:04}-{m:02}-{d:02}.md"));
        let new = !path.exists();
        let mut f = fs::OpenOptions::new().create(true).append(true).open(&path)?;
        if new {
            writeln!(f, "# Change log {y:04}-{m:02}-{d:02}\n")?;
        }
        let clean = |s: &str| s.replace(['\n', '\r'], " ").replace('|', "/");
        writeln!(f, "- {at} | {} | {} | {}", clean(action), clean(subject), clean(reason))
    }

    /// Read all log entries in order: (time, action, subject, reason).
    pub fn read_log(&self) -> io::Result<Vec<(String, String, String, String)>> {
        let mut out = Vec::new();
        for (_, src) in read_md_dir(&self.root.join("log"))? {
            for l in src.lines() {
                if let Some(rest) = l.strip_prefix("- ") {
                    let p: Vec<&str> = rest.splitn(4, " | ").collect();
                    if p.len() == 4 {
                        out.push((p[0].into(), p[1].into(), p[2].into(), p[3].into()));
                    }
                }
            }
        }
        Ok(out)
    }
}

fn write_if_changed(path: &Path, content: &str) -> io::Result<()> {
    if let Ok(old) = fs::read_to_string(path) {
        if old == content {
            return Ok(());
        }
    }
    fs::write(path, content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::parse_rule;

    #[test]
    fn rule_file_roundtrip() {
        let src = "---\nid: coffee_morning\nstatus: hypothesis\nconf: 0.4\nevidence: 3\ncounter: 0\nmodule: food\nsource: learner\n---\n# coffee_morning\n\nPeople drink coffee in the morning.\n\n```datalog\ndrinks(X, coffee) :- person(X), morning(T), during(X, T).\n```\n";
        let rf = RuleFile::parse(src, "x");
        assert_eq!(rf.render(), src);
        let m = rf.meta();
        assert_eq!(m.status, RuleStatus::Hypothesis);
        assert_eq!(m.evidence, 3);
        assert_eq!(rf.note(), "People drink coffee in the morning.");
        assert_eq!(rf.rules().len(), 1);
    }

    #[test]
    fn rule_file_meta_change() {
        let meta = RuleMeta {
            id: "r1".into(),
            status: RuleStatus::Hypothesis,
            conf: 0.3,
            evidence: 1,
            counter: 0,
            module: "general".into(),
            source: "learner".into(),
        };
        let mut rf = RuleFile::new(&meta, "Note.", &[parse_rule("p(X) :- q(X).").unwrap()]);
        let before = rf.render();
        let mut m2 = rf.meta();
        assert_eq!(m2, meta);
        m2.conf = 0.5;
        rf.set_meta(&m2);
        let after = rf.render();
        let diff: Vec<_> = before.lines().zip(after.lines()).filter(|(a, b)| a != b).collect();
        assert_eq!(diff, vec![("conf: 0.3", "conf: 0.5")]);
    }
}
