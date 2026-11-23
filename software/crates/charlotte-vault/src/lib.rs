//! The knowledge vault: a folder of Markdown notes, one per item. The file name is the item's id,
//! the YAML front matter is its formal part, the body is its text, `[[links]]` point at other notes.
//! The vault on disk is the source of truth; the typed index is rebuilt from it.

use charlotte_core::*;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value as Json};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

pub const KINDS: [&str; 10] = ["pattern", "atom", "procedure", "intent", "slot", "records", "move", "lexicon", "phrases", "chats"];

#[derive(Clone, Debug)]
pub struct Note {
    pub id: String,
    /// Path relative to the vault root, with forward slashes.
    pub path: String,
    pub front: Map<String, Json>,
    pub body: String,
}

fn yaml_to_json(v: serde_yaml::Value) -> Json {
    use serde_yaml::Value as Y;
    match v {
        Y::Null => Json::Null,
        Y::Bool(b) => Json::Bool(b),
        Y::Number(n) => n.as_i64().map(Json::from).or_else(|| n.as_f64().map(Json::from)).unwrap_or(Json::Null),
        Y::String(s) => Json::String(s),
        Y::Sequence(s) => Json::Array(s.into_iter().map(yaml_to_json).collect()),
        Y::Mapping(m) => Json::Object(
            m.into_iter()
                .map(|(k, v)| {
                    let key = match k {
                        Y::String(s) => s,
                        Y::Bool(b) => b.to_string(),
                        Y::Number(n) => n.to_string(),
                        other => serde_yaml::to_string(&other).unwrap_or_default().trim().to_string(),
                    };
                    (key, yaml_to_json(v))
                })
                .collect(),
        ),
        Y::Tagged(t) => yaml_to_json(t.value),
    }
}
fn link_target(s: &str) -> Option<&str> {
    let inner = s.strip_prefix("[[")?.strip_suffix("]]")?;
    Some(inner.split('|').next().unwrap_or(inner).trim())
}
/// The same value with every `"[[x]]"` replaced by `"x"`.
fn strip_links(v: &Json) -> Json {
    match v {
        Json::String(s) => Json::String(link_target(s).unwrap_or(s).to_string()),
        Json::Array(a) => Json::Array(a.iter().map(strip_links).collect()),
        Json::Object(m) => Json::Object(m.iter().map(|(k, v)| (k.clone(), strip_links(v))).collect()),
        other => other.clone(),
    }
}
fn collect_links(v: &Json, out: &mut Vec<String>) {
    match v {
        Json::String(s) => {
            if let Some(t) = link_target(s) {
                out.push(t.to_string())
            }
        }
        Json::Array(a) => a.iter().for_each(|x| collect_links(x, out)),
        Json::Object(m) => m.values().for_each(|x| collect_links(x, out)),
        _ => {}
    }
}

/// The conversations in a chats note: "Entry N" headings followed by "U: " and "A: " lines.
pub fn parse_chats(body: &str) -> Vec<Vec<(char, String)>> {
    let mut out: Vec<Vec<(char, String)>> = Vec::new();
    for line in body.lines() {
        let l = line.trim();
        if l.get(..6).is_some_and(|h| h.eq_ignore_ascii_case("entry ")) && l.len() > 6 && l[6..].trim().chars().all(|c| c.is_ascii_digit()) {
            out.push(Vec::new());
        } else if let (Some(conv), Some(who)) = (out.last_mut(), l.chars().next().filter(|c| (*c == 'U' || *c == 'A') && l[1..].starts_with(": "))) {
            conv.push((who, l[3..].trim().to_string()));
        }
    }
    out.retain(|c| !c.is_empty());
    out
}
/// The same conversations as a note body.
pub fn render_chats(convs: &[Vec<(char, String)>]) -> String {
    convs.iter().enumerate().map(|(i, c)| format!("Entry {}\n{}", i + 1, c.iter().map(|(w, t)| format!("{w}: {t}\n")).collect::<String>())).collect::<Vec<_>>().join("\n").trim_end().to_string()
}

impl Note {
    pub fn parse(path: &str, text: &str) -> Result<Note, String> {
        let id = Path::new(path).file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string();
        let text = text.replace("\r\n", "\n");
        let rest = text.strip_prefix("---\n").ok_or("no front matter")?;
        let end = rest.find("\n---").ok_or("front matter is not closed")?;
        let yaml: serde_yaml::Value = serde_yaml::from_str(&rest[..end]).map_err(|e| format!("front matter: {e}"))?;
        let front = match yaml_to_json(yaml) {
            Json::Object(m) => m,
            _ => return Err("front matter is not a mapping".into()),
        };
        let after = &rest[end + 4..];
        let body = after.strip_prefix('\n').unwrap_or(after).trim_end().to_string();
        Ok(Note { id, path: path.to_string(), front, body })
    }
    pub fn render(&self) -> String {
        let yaml = serde_yaml::to_string(&Json::Object(self.front.clone())).unwrap_or_default();
        if self.body.is_empty() {
            format!("---\n{yaml}---\n")
        } else {
            format!("---\n{yaml}---\n{}\n", self.body)
        }
    }
    pub fn kind(&self) -> &str {
        self.front.get("kind").and_then(|k| k.as_str()).unwrap_or("")
    }
    pub fn status(&self) -> &str {
        if self.path.starts_with("candidates/") {
            return "candidate";
        }
        self.front.get("status").and_then(|k| k.as_str()).unwrap_or("trusted")
    }
    pub fn trust(&self) -> f64 {
        self.front.get("trust").and_then(|k| k.as_f64()).unwrap_or(0.8)
    }
    pub fn links(&self) -> Vec<String> {
        let mut out = Vec::new();
        collect_links(&Json::Object(self.front.clone()), &mut out);
        let mut rest = self.body.as_str();
        while let Some(i) = rest.find("[[") {
            rest = &rest[i + 2..];
            if let Some(j) = rest.find("]]") {
                out.push(rest[..j].split('|').next().unwrap_or("").trim().to_string());
                rest = &rest[j + 2..];
            }
        }
        out.sort();
        out.dedup();
        out
    }
    pub fn title(&self) -> String {
        let s = |k: &str| strip_links(self.front.get(k).unwrap_or(&Json::Null)).as_str().map(str::to_string);
        match self.kind() {
            "atom" => format!("{} · {}", s("intent").unwrap_or_default(), s("object").unwrap_or_else(|| "any".into())),
            "pattern" => format!("{} requests", s("applies_to").unwrap_or_default()),
            "procedure" | "move" => s("intent").unwrap_or_else(|| self.id.clone()),
            "phrases" => format!("phrases for {}", s("intent").or_else(|| s("slot")).unwrap_or_default()),
            "chats" => s("topic").unwrap_or_else(|| self.id.clone()),
            _ => self.id.clone(),
        }
    }
    fn typed<T: DeserializeOwned>(&self) -> Result<T, String> {
        serde_json::from_value(strip_links(&Json::Object(self.front.clone()))).map_err(|e| format!("{}: {e}", self.path))
    }
}

#[derive(Clone, Default)]
struct Typed {
    intents: BTreeMap<String, Intent>,
    slots: BTreeMap<String, Slot>,
    atoms: Vec<Atom>,
    procedures: Vec<Procedure>,
    records: Vec<Records>,
    patterns: Vec<Pattern>,
    moves: Vec<Move>,
    lexicon: Lexicon,
    replies: Vec<ReplyCase>,
}

#[derive(Clone)]
pub struct Vault {
    pub root: PathBuf,
    pub notes: BTreeMap<String, Note>,
    /// Notes that could not be read or do not fit their kind.
    pub problems: Vec<String>,
    typed: Typed,
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else { return };
    let mut entries: Vec<_> = rd.filter_map(|e| e.ok()).map(|e| e.path()).collect();
    entries.sort();
    for p in entries {
        let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if name.starts_with('.') {
            continue;
        }
        if p.is_dir() {
            walk(&p, out)
        } else if name.ends_with(".md") {
            out.push(p)
        }
    }
}

impl Vault {
    pub fn open(root: impl AsRef<Path>) -> Result<Vault, String> {
        let root = root.as_ref().to_path_buf();
        if !root.is_dir() {
            return Err(format!("vault folder not found: {}", root.display()));
        }
        let mut files = Vec::new();
        walk(&root, &mut files);
        let mut v = Vault { root: root.clone(), notes: BTreeMap::new(), problems: Vec::new(), typed: Typed::default() };
        for f in files {
            let rel = f.strip_prefix(&root).unwrap_or(&f).to_string_lossy().replace('\\', "/");
            match fs::read_to_string(&f).map_err(|e| e.to_string()).and_then(|t| Note::parse(&rel, &t)) {
                Ok(n) => {
                    if let Some(prev) = v.notes.get(&n.id) {
                        v.problems.push(format!("{rel}: the id '{}' is already used by {}", n.id, prev.path));
                    } else {
                        v.notes.insert(n.id.clone(), n);
                    }
                }
                Err(e) => v.problems.push(format!("{rel}: {e}")),
            }
        }
        v.reindex();
        Ok(v)
    }

    /// Rebuild the typed index from the notes. Candidates and retired notes are left out.
    pub fn reindex(&mut self) {
        let mut t = Typed::default();
        self.problems.retain(|p| !p.starts_with("index: "));
        let mut bad = Vec::new();
        for n in self.notes.values() {
            if n.status() != "trusted" {
                continue;
            }
            let r: Result<(), String> = (|| {
                match n.kind() {
                    "intent" => {
                        let mut x: Intent = n.typed()?;
                        x.id = n.id.clone();
                        t.intents.insert(x.id.clone(), x);
                    }
                    "slot" => {
                        let mut x: Slot = n.typed()?;
                        x.id = n.id.clone();
                        t.slots.insert(x.id.clone(), x);
                    }
                    "atom" => {
                        let mut x: Atom = n.typed()?;
                        x.id = n.id.clone();
                        x.payload = n.body.clone();
                        t.atoms.push(x);
                    }
                    "procedure" => {
                        let mut x: Procedure = n.typed()?;
                        x.id = n.id.clone();
                        t.procedures.push(x);
                    }
                    "records" => {
                        let mut x: Records = n.typed()?;
                        x.id = n.id.clone();
                        t.records.push(x);
                    }
                    "pattern" => {
                        let mut x: Pattern = n.typed()?;
                        x.id = n.id.clone();
                        t.patterns.push(x);
                    }
                    "move" => {
                        let mut x: Move = n.typed()?;
                        x.id = n.id.clone();
                        x.reply = n.body.clone();
                        t.moves.push(x);
                    }
                    "lexicon" => t.lexicon = n.typed()?,
                    "phrases" => {}
                    "chats" => {
                        for (e, conv) in parse_chats(&n.body).iter().enumerate() {
                            for i in 0..conv.len().saturating_sub(1) {
                                if conv[i].0 == 'U' && conv[i + 1].0 == 'A' {
                                    let previous = if i > 0 { Some(conv[i - 1].1.clone()) } else { None };
                                    t.replies.push(ReplyCase { source: n.id.clone(), entry: e + 1, previous, message: conv[i].1.clone(), reply: conv[i + 1].1.clone() });
                                }
                            }
                        }
                    }
                    other => return Err(format!("{}: unknown kind '{other}'", n.path)),
                }
                Ok(())
            })();
            if let Err(e) = r {
                bad.push(format!("index: {e}"));
            }
        }
        self.problems.extend(bad);
        self.typed = t;
    }

    /// Check a note against its kind without storing it.
    pub fn validate(note: &Note) -> Result<(), String> {
        match note.kind() {
            "intent" => note.typed::<Intent>().map(|_| ()),
            "slot" => note.typed::<Slot>().map(|_| ()),
            "atom" => note.typed::<Atom>().map(|_| ()),
            "procedure" => note.typed::<Procedure>().map(|_| ()),
            "records" => note.typed::<Records>().map(|_| ()),
            "pattern" => note.typed::<Pattern>().map(|_| ()),
            "move" => note.typed::<Move>().map(|_| ()),
            "lexicon" => note.typed::<Lexicon>().map(|_| ()),
            "phrases" | "chats" => Ok(()),
            other => Err(format!("unknown kind '{other}'")),
        }
    }

    /// Add or replace a note in memory only.
    pub fn put_mem(&mut self, note: Note) -> Result<(), String> {
        Vault::validate(&note)?;
        self.notes.insert(note.id.clone(), note);
        self.reindex();
        Ok(())
    }
    /// Add or replace a note and write its file.
    pub fn put(&mut self, note: Note) -> Result<(), String> {
        Vault::validate(&note)?;
        let file = self.root.join(&note.path);
        if let Some(old) = self.notes.get(&note.id) {
            if old.path != note.path {
                let _ = fs::remove_file(self.root.join(&old.path));
            }
        }
        if let Some(dir) = file.parent() {
            fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        }
        fs::write(&file, note.render()).map_err(|e| format!("{}: {e}", file.display()))?;
        self.notes.insert(note.id.clone(), note);
        self.reindex();
        Ok(())
    }
    pub fn remove(&mut self, id: &str) -> Result<(), String> {
        let note = self.notes.remove(id).ok_or_else(|| format!("no item '{id}'"))?;
        let _ = fs::remove_file(self.root.join(&note.path));
        self.reindex();
        Ok(())
    }
    pub fn backlinks(&self, id: &str) -> Vec<String> {
        self.notes.values().filter(|n| n.id != id && n.links().iter().any(|l| l == id)).map(|n| n.id.clone()).collect()
    }

    /// Problems a person should fix: unreadable notes, links to nothing, atoms about unknown intents or slots.
    pub fn lint(&self) -> Vec<String> {
        let mut out = self.problems.clone();
        for n in self.notes.values() {
            for l in n.links() {
                if !self.notes.contains_key(&l) {
                    out.push(format!("{}: link to missing note [[{l}]]", n.path));
                }
            }
        }
        for a in &self.typed.atoms {
            if !self.typed.intents.contains_key(&a.intent) {
                out.push(format!("atom {}: unknown intent '{}'", a.id, a.intent));
            }
            for c in &a.conditions {
                if !self.typed.slots.contains_key(&c.slot) {
                    out.push(format!("atom {}: unknown slot '{}'", a.id, c.slot));
                }
            }
        }
        for p in &self.typed.procedures {
            if !self.typed.intents.contains_key(&p.intent) {
                out.push(format!("procedure {}: unknown intent '{}'", p.id, p.intent));
            }
        }
        out
    }

    /// Rewrite every note in the form the engine writes, so later edits by the engine change nothing else.
    pub fn normalize(&self) -> Result<usize, String> {
        let mut changed = 0;
        for n in self.notes.values() {
            let file = self.root.join(&n.path);
            let new = n.render();
            if fs::read_to_string(&file).map_err(|e| e.to_string())? != new {
                fs::write(&file, new).map_err(|e| e.to_string())?;
                changed += 1;
            }
        }
        Ok(changed)
    }
}

impl Memory for Vault {
    fn intents(&self) -> Vec<&Intent> {
        self.typed.intents.values().collect()
    }
    fn intent(&self, id: &str) -> Option<&Intent> {
        self.typed.intents.get(id)
    }
    fn slots(&self) -> Vec<&Slot> {
        self.typed.slots.values().collect()
    }
    fn slot(&self, id: &str) -> Option<&Slot> {
        self.typed.slots.get(id)
    }
    fn lexicon(&self) -> &Lexicon {
        &self.typed.lexicon
    }
    fn atoms(&self, intent: &str) -> Vec<&Atom> {
        self.typed.atoms.iter().filter(|a| a.intent == intent).collect()
    }
    fn procedure(&self, intent: &str) -> Option<&Procedure> {
        self.typed.procedures.iter().find(|p| p.intent == intent)
    }
    fn record(&self, subject: &str) -> Option<&RecordRow> {
        self.typed.records.iter().flat_map(|r| r.rows.iter()).find(|r| r.id.eq_ignore_ascii_case(subject))
    }
    fn records(&self) -> Vec<&Records> {
        self.typed.records.iter().collect()
    }
    fn pattern(&self, kind: ReqKind) -> Option<&Pattern> {
        self.typed.patterns.iter().find(|p| p.applies_to == kind)
    }
    fn moves(&self) -> Vec<&Move> {
        self.typed.moves.iter().collect()
    }
    fn replies(&self) -> &[ReplyCase] {
        &self.typed.replies
    }
}
