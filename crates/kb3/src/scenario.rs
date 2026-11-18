//! v3 scenario files (`data/curriculum3/*/*.md`, `data/heldout3/*/*.md`).

use crate::fact::Fact;
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct Variant {
    pub change: String,
    pub request: String,
    pub situation_text: String,
    pub expect: String,
    pub situation: Vec<Fact>,
    pub difference: Vec<Fact>,
}

#[derive(Debug, Clone, Default)]
pub struct Scenario {
    pub id: String,
    pub task: String,
    pub head: BTreeMap<String, String>,
    pub requests: Vec<String>,
    pub situation: Vec<String>,
    pub teach: Vec<String>,
    pub checklist: Vec<String>,
    pub facts: Vec<Fact>,
    pub aliases: Vec<Fact>,
    pub variants: Vec<Variant>,
}

fn bullets(lines: &[String]) -> Vec<String> {
    lines.iter().filter_map(|l| l.strip_prefix("- ").map(|s| s.trim().to_string())).collect()
}

impl Scenario {
    pub fn load(path: &Path) -> Result<Scenario, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        Scenario::parse(&text).map_err(|e| format!("{}: {e}", path.display()))
    }

    pub fn parse(text: &str) -> Result<Scenario, String> {
        let mut sc = Scenario::default();
        let lines: Vec<&str> = text.lines().collect();
        let mut i = 0;
        if lines.first() == Some(&"---") {
            i = 1;
            while i < lines.len() && lines[i] != "---" {
                if let Some((k, v)) = lines[i].split_once(':') {
                    sc.head.insert(k.trim().to_string(), v.trim().to_string());
                }
                i += 1;
            }
            i += 1;
        }
        let mut section = String::new();
        let mut sections: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut blocks: Vec<(String, String, Vec<Fact>)> = Vec::new();
        let mut block: Option<(String, String, Vec<Fact>)> = None;
        while i < lines.len() {
            let line = lines[i];
            i += 1;
            if let Some(b) = block.as_mut() {
                if line.starts_with("```") {
                    blocks.push(block.take().unwrap());
                    continue;
                }
                let t = line.trim();
                if t.is_empty() || t.starts_with('%') {
                    continue;
                }
                b.2.push(Fact::parse(t)?);
                continue;
            }
            if let Some(rest) = line.strip_prefix("```") {
                let label = rest.split_whitespace().nth(1).unwrap_or("").to_string();
                block = Some((section.clone(), label, Vec::new()));
            } else if let Some(name) = line.strip_prefix("## ") {
                section = name.trim().to_string();
                sections.entry(section.clone()).or_default();
            } else if !section.is_empty() {
                sections.entry(section.clone()).or_default().push(line.to_string());
            }
        }
        sc.id = sc.head.get("id").cloned().unwrap_or_default();
        sc.task = sc.head.get("task").cloned().unwrap_or_else(|| sc.id.clone());
        let get = |k: &str| sections.get(k).map(|v| bullets(v)).unwrap_or_default();
        sc.requests = get("requests");
        sc.situation = get("situation");
        sc.teach = get("teach");
        sc.checklist = get("checklist");
        for (sec, _, facts) in &blocks {
            match sec.as_str() {
                "ground truth" => sc.facts.extend(facts.iter().cloned()),
                "aliases" => sc.aliases.extend(facts.iter().cloned()),
                _ => {}
            }
        }
        let mut vs: Vec<Variant> = Vec::new();
        for l in get("variant") {
            if let Some((k, v)) = l.split_once(':') {
                let v = v.trim().to_string();
                match k.trim() {
                    "change" => vs.push(Variant { change: v, ..Default::default() }),
                    "request" => if let Some(x) = vs.last_mut() { x.request = v },
                    "situation" => if let Some(x) = vs.last_mut() { x.situation_text = v },
                    "expect" => if let Some(x) = vs.last_mut() { x.expect = v },
                    _ => {}
                }
            }
        }
        let sit: Vec<_> = blocks.iter().filter(|b| b.0 == "variant" && b.1 == "situation").collect();
        let dif: Vec<_> = blocks.iter().filter(|b| b.0 == "variant" && b.1 == "difference").collect();
        for (k, v) in vs.iter_mut().enumerate() {
            if let Some(b) = sit.get(k) {
                v.situation = b.2.clone();
            }
            if let Some(b) = dif.get(k) {
                v.difference = b.2.clone();
            }
        }
        sc.variants = vs;
        Ok(sc)
    }

    /// Load every `.md` scenario in a folder, sorted by file name.
    pub fn load_dir(dir: &Path) -> Result<Vec<Scenario>, String> {
        let mut paths: Vec<_> = std::fs::read_dir(dir)
            .map_err(|e| format!("{}: {e}", dir.display()))?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.extension().map(|x| x == "md").unwrap_or(false))
            .collect();
        paths.sort();
        paths.iter().map(|p| Scenario::load(p)).collect()
    }
}
