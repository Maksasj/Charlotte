//! Charlotte 3.0 evaluation: curriculum runner (C1 dialogue teaching), calibration of the
//! fallback threshold, scoring with the fact-and-relation check, Wilson intervals, and the
//! append-only event log of the training lab.

pub mod lab;
pub mod stats;

use agent3::{Agent, Arm};
use kb3::vault::Source;
use kb3::{check, gold, lacked, lines, Aliases, Scenario};
use parse3::Models;
use serde::Serialize;
use serde_json::json;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use teach3::Teacher;

pub fn root() -> PathBuf {
    kb3::repo_root()
}

pub fn load(dir: &str) -> Vec<Scenario> {
    Scenario::load_dir(&root().join(dir)).unwrap_or_default()
}

/// Step-line exactness of a learned parse against gold (the same slots as phase A's X1).
pub fn line_exact(models: &Models, sc: &Scenario, line: &str, known: &BTreeSet<String>) -> Option<(bool, f64)> {
    let p = models.parse_teach(&sc.task, line, known);
    if p.kind != parse3::LineKind::Step {
        return None;
    }
    let g = gold(sc, None);
    let sid = p.facts.iter().find(|f| f.pred == "step").map(|f| f.arg(1).to_string())?;
    let gs = g.step(&sid)?;
    let mut tmp = kb3::Vault::default();
    for f in &p.facts {
        tmp.add(f.clone(), kb3::Provenance::new(Source::Teach, "x", 0, "learned", 1.0));
    }
    let pp = tmp.procedure(&sc.task);
    let ps = pp.step(&sid)?;
    let uses_g: BTreeSet<&String> = gs.uses.keys().collect();
    let uses_p: BTreeSet<&String> = ps.uses.keys().collect();
    let ok = ps.action == gs.action && ps.object == gs.object && uses_g == uses_p && ps.until == gs.until && ps.time == gs.time;
    Some((ok, p.confidence))
}

/// Choose θ on dev: the lowest margin such that lines at or above it are >= 95% exact.
pub fn calibrate(models: &Models, dev: &[Scenario]) -> (f64, f64, usize) {
    let mut rows = Vec::new();
    for sc in dev {
        let needs = sc.teach.iter().find(|l| models.parse_teach(&sc.task, l, &BTreeSet::new()).kind == parse3::LineKind::Needs);
        let known = needs.map(|l| models.known_words(l)).unwrap_or_default();
        for l in &sc.teach {
            if let Some((ok, m)) = line_exact(models, sc, l, &known) {
                rows.push((m, ok));
            }
        }
    }
    rows.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    let mut theta = f64::INFINITY;
    let mut cover = 0;
    let mut ok = 0;
    for (k, (m, o)) in rows.iter().enumerate() {
        ok += *o as usize;
        if ok as f64 / (k + 1) as f64 >= 0.95 {
            theta = *m;
            cover = k + 1;
        }
    }
    (theta, cover as f64 / rows.len().max(1) as f64, rows.len())
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct Encounter {
    pub scenario: String,
    pub kind: String,
    pub variant: Option<usize>,
    pub request: String,
    pub identified: Option<String>,
    pub identity_ok: bool,
    pub strict_pass: bool,
    pub core_recall: f64,
    pub recall: f64,
    pub precision: f64,
    pub wrong: Vec<String>,
    pub gaps: usize,
    pub asked: bool,
    pub answer: String,
}

pub struct RunOpts {
    pub name: String,
    pub arm: Arm,
    pub live: bool,
    pub use_fallback: bool,
    /// teach variants after a failed variant encounter (dialogue correction)
    pub teach_variants: bool,
    pub seeds: bool,
    /// start from a saved vault (e.g. after the C1 curriculum)
    pub vault: Option<PathBuf>,
}

pub struct Runner {
    pub agent: Agent,
    pub log: lab::Log,
    pub encounters: Vec<Encounter>,
    pub hidden: bool,
}

impl Runner {
    pub fn new(opts: &RunOpts, theta: f64) -> Runner {
        let models = Models::load(&Models::default_path()).expect("models: run tools/design3/train_parser.py");
        let teacher = Teacher::open(opts.live);
        let mut agent = Agent::new(models, teacher, &opts.name, theta);
        agent.arm = opts.arm;
        agent.use_fallback = opts.use_fallback;
        if opts.seeds {
            for sc in load("data/curriculum3/seed") {
                let mut fs = sc.facts.clone();
                fs.extend(sc.aliases.iter().filter(|f| f.pred == "task_alias").cloned());
                agent.seed(&fs);
            }
        }
        if let Some(v) = &opts.vault {
            agent.vault = kb3::Vault::load(v).expect("vault");
        }
        Runner { agent, log: lab::Log::open(&opts.name), encounters: Vec::new(), hidden: false }
    }

    pub fn score(&mut self, sc: &Scenario, kind: &str, variant: Option<usize>, request: &str, situation: Option<&str>) -> Encounter {
        let reply = self.agent.answer(request, situation);
        let g = gold(sc, variant);
        let al = Aliases::from_facts(&sc.aliases);
        let lacks = variant.map(|k| lacked(&sc.variants[k])).unwrap_or_default();
        let parsed = lines::parse(&reply.lines);
        let mut proc = parsed.procedure.clone();
        proc.task = sc.task.clone();
        let identity_ok = reply.task.as_deref() == Some(sc.task.as_str());
        let r = check(&if identity_ok { proc } else { kb3::Procedure { task: sc.task.clone(), ..Default::default() } }, &g, &al, &lacks);
        let e = Encounter {
            scenario: sc.id.clone(),
            kind: kind.into(),
            variant,
            request: request.into(),
            identified: reply.task.clone(),
            identity_ok,
            strict_pass: identity_ok && r.strict_pass,
            core_recall: r.core_recall(),
            recall: r.recall(),
            precision: r.precision(),
            wrong: r.wrong.clone(),
            gaps: reply.gaps.len(),
            asked: !reply.asks.is_empty(),
            answer: reply.lines.clone(),
        };
        let payload = if self.hidden {
            json!({"scenario": "-", "kind": kind, "pass": e.strict_pass})
        } else {
            json!({"scenario": sc.id, "kind": kind, "variant": variant, "pass": e.strict_pass, "identity": e.identity_ok, "recall": e.recall, "wrong": e.wrong.len(), "gaps": e.gaps})
        };
        self.log.event("encounter", payload);
        self.encounters.push(e.clone());
        e
    }

    /// Stored-fact accuracy after teaching (M3): the vault's procedure against gold.
    pub fn stored(&mut self, sc: &Scenario) -> kb3::CheckResult {
        let p = self.agent.known_procedure(&sc.task);
        let r = check(&p, &gold(sc, None), &Aliases::from_facts(&sc.aliases), &[]);
        let payload = if self.hidden { json!({"pass": r.strict_pass}) } else { json!({"scenario": sc.id, "pass": r.strict_pass, "recall": r.recall(), "core": r.core_recall()}) };
        self.log.event("stored", payload);
        r
    }

    /// Teach a scenario through dialogue: the first request, then the teach lines.
    pub fn teach(&mut self, sc: &Scenario) -> kb3::CheckResult {
        let req = sc.requests.first().cloned();
        let _first = self.agent.answer(req.as_deref().unwrap_or(&sc.task), None);
        let st = self.agent.teach(&sc.task, req.as_deref(), &sc.teach, Source::Teach);
        self.log.event("teach", json!({"scenario": if self.hidden { "-" } else { sc.id.as_str() }, "lines": st.lines, "learned": st.learned, "fallback": st.fallback, "failed": st.fallback_failed, "rejected": st.rejected}));
        self.stored(sc)
    }

    /// After a failed variant: the teacher shows the adapted procedure as teach lines; the agent
    /// parses them and stores the case.
    pub fn teach_variant(&mut self, sc: &Scenario, k: usize) -> bool {
        let v = &sc.variants[k];
        let target = gold(sc, Some(k));
        let facts = target.to_facts();
        let q = format!("How do I {} in this situation: {} {}? Teach me every step, one line per step.", sc.task.replace('_', " "), v.request, v.situation_text);
        let Ok(answer) = self.agent.teacher.answer(&facts, &v.situation, &q) else { return false };
        let taught_lines: Vec<String> = answer.lines().map(|l| l.trim().trim_start_matches("- ").to_string()).filter(|l| !l.is_empty()).collect();
        let taught = self.agent.parse_procedure(&sc.task, &taught_lines);
        let sit = agent3::adapt::link_situation(&self.agent.vault, &self.agent.known_procedure(&sc.task), v.situation.clone());
        self.agent.learn_variant(&sc.task, &sit, &taught, Source::Teach);
        self.log.event("teach_variant", json!({"scenario": if self.hidden { "-" } else { sc.id.as_str() }, "variant": k, "lines": taught_lines.len()}));
        true
    }

    pub fn save(&self, dir: &Path) {
        let _ = std::fs::create_dir_all(dir);
        let mut out = String::new();
        for e in &self.encounters {
            out.push_str(&serde_json::to_string(e).unwrap());
            out.push('\n');
        }
        let _ = std::fs::write(dir.join("encounters.jsonl"), out);
        let _ = self.agent.vault.save(&dir.join("vault.json"));
    }
}

/// The C1 curriculum on a set: teach each task, then second encounter, variants (with
/// teaching after a failure, then a repeat), and retention at the end.
pub fn curriculum(r: &mut Runner, scs: &[Scenario], teach_variants: bool) {
    for sc in scs {
        r.teach(sc);
        if let Some(q) = sc.requests.get(1) {
            r.score(sc, "second", None, q, sc.situation.first().map(String::as_str));
        }
        for k in 0..sc.variants.len() {
            let v = sc.variants[k].clone();
            let e = r.score(sc, "variant", Some(k), &v.request, Some(&v.situation_text));
            if !e.strict_pass && teach_variants && r.teach_variant(sc, k) {
                r.score(sc, "variant_repeat", Some(k), &v.request, Some(&v.situation_text));
            }
        }
    }
    for sc in scs {
        if let Some(q) = sc.requests.get(2).or(sc.requests.first()) {
            r.score(sc, "retention", None, q, None);
        }
    }
}

/// H3 tier S: one first encounter per case on a known task (no teaching before it).
pub fn tier_s(r: &mut Runner, scs: &[Scenario]) {
    for sc in scs {
        if let Some(v) = sc.variants.first().cloned() {
            let req = if v.request.is_empty() { sc.requests.first().cloned().unwrap_or_default() } else { v.request.clone() };
            r.score(sc, "S_first", Some(0), &req, Some(&v.situation_text));
        }
    }
}

/// H3 tiers T and G: the task is taught once, then measured as in the curriculum.
pub fn tier_taught(r: &mut Runner, scs: &[Scenario], tier: &str, teach_variants: bool) -> Vec<kb3::CheckResult> {
    let mut stored = Vec::new();
    for sc in scs {
        stored.push(r.teach(sc));
        if let Some(q) = sc.requests.get(1) {
            r.score(sc, &format!("{tier}_second"), None, q, sc.situation.first().map(String::as_str));
        }
        for k in 0..sc.variants.len() {
            let v = sc.variants[k].clone();
            let e = r.score(sc, &format!("{tier}_variant"), Some(k), &v.request, Some(&v.situation_text));
            if !e.strict_pass && teach_variants && r.teach_variant(sc, k) {
                r.score(sc, &format!("{tier}_variant_repeat"), Some(k), &v.request, Some(&v.situation_text));
            }
        }
    }
    for sc in scs {
        if let Some(q) = sc.requests.get(2) {
            r.score(sc, &format!("{tier}_retention"), None, q, None);
        }
    }
    stored
}

/// Summary by encounter kind: passes, n, Wilson 95% interval.
pub fn summary(encs: &[Encounter]) -> serde_json::Value {
    let mut kinds: Vec<String> = encs.iter().map(|e| e.kind.clone()).collect();
    kinds.sort();
    kinds.dedup();
    let mut out = serde_json::Map::new();
    for k in kinds {
        let es: Vec<&Encounter> = encs.iter().filter(|e| e.kind == k).collect();
        let pass = es.iter().filter(|e| e.strict_pass).count();
        let (lo, hi) = stats::wilson(pass, es.len());
        let wrong = es.iter().filter(|e| !e.wrong.is_empty()).count();
        let ident = es.iter().filter(|e| e.identity_ok).count();
        let rec: f64 = es.iter().map(|e| e.recall).sum::<f64>() / es.len().max(1) as f64;
        out.insert(k, json!({"n": es.len(), "pass": pass, "rate": stats::round3(pass as f64 / es.len().max(1) as f64),
            "ci95": [stats::round3(lo), stats::round3(hi)], "identity": ident, "with_wrong_claim": wrong, "mean_recall": stats::round3(rec)}));
    }
    serde_json::Value::Object(out)
}
