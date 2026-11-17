//! The real-turn regression set and the outcome command (Design 2.0,
//! milestone 12).
//!
//! `data/regression/turns.jsonl` holds the 296 agent turns v1 produced in
//! real use (68 help-scenario trainer turns, 228 judge turns). The outcome
//! command either labels the replies v1 gave (`--v1`, the baseline) or replays
//! the same user messages through the current build and labels the new
//! replies. It prints the outcome mix with 95% intervals, appends a row to
//! `eval/outcomes.csv`, compares it with the previous comparable row, and
//! writes a per-turn report.

use crate::outcome::{Label, LabelSource, Labeller, Mix, TurnToLabel};
use serde::{Deserialize, Serialize};
use sla_agent::Agent;
use sla_core::Timestamp;
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RegTurn {
    pub id: String,
    pub set: String,
    pub conv: String,
    pub turn: usize,
    pub scenario: String,
    pub user: String,
    pub v1_reply: String,
}

pub fn load_turns(path: &Path) -> std::io::Result<Vec<RegTurn>> {
    let src = std::fs::read_to_string(path)?;
    Ok(src.lines().filter(|l| !l.trim().is_empty()).filter_map(|l| serde_json::from_str(l).ok()).collect())
}

/// Hand labels: `id<TAB>label<TAB>note`, `#` comments.
pub fn load_gold_labels(path: &Path) -> HashMap<String, Label> {
    let mut out = HashMap::new();
    for l in std::fs::read_to_string(path).unwrap_or_default().lines() {
        if l.starts_with('#') || l.trim().is_empty() {
            continue;
        }
        let p: Vec<&str> = l.split('\t').collect();
        if let (Some(id), Some(lab)) = (p.first(), p.get(1).and_then(|x| Label::parse(x))) {
            out.insert(id.to_string(), lab);
        }
    }
    out
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParserMode {
    Pattern,
    /// The LLM translator with this model.
    Llm(String),
    /// Gold parses from this file.
    Oracle(PathBuf),
}

impl ParserMode {
    pub fn name(&self) -> String {
        match self {
            ParserMode::Pattern => "pattern".into(),
            ParserMode::Llm(m) => format!("llm:{m}"),
            ParserMode::Oracle(_) => "oracle".into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OutcomeConfig {
    pub root: PathBuf,
    /// Label the recorded v1 replies instead of replaying.
    pub v1: bool,
    pub parser: ParserMode,
    /// Judge model for content replies; `None`: templates only.
    pub judge: Option<String>,
    /// Use hand labels where they exist (they win over the judge).
    pub gold: bool,
    /// Only this set (`trainer` or `judge`).
    pub set: Option<String>,
    pub notes: String,
    /// Append the row and write the report.
    pub write: bool,
}

impl OutcomeConfig {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        OutcomeConfig { root: root.into(), v1: false, parser: ParserMode::Pattern, judge: None, gold: false, set: None, notes: String::new(), write: true }
    }

    fn key(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}",
            if self.v1 { "v1" } else { "replay" },
            if self.v1 { "-".into() } else { self.parser.name() },
            self.judge.as_deref().unwrap_or("none"),
            if self.gold { "gold" } else { "nogold" },
            self.set.as_deref().unwrap_or("all")
        )
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct LabelledTurn {
    pub id: String,
    pub set: String,
    pub scenario: String,
    pub user: String,
    pub reply: String,
    pub label: Label,
    pub source: LabelSource,
    pub think_ms: f64,
    /// Translation time of the turn (ms), outside the vault lock.
    pub translate_ms: f64,
    pub acts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct OutcomeRun {
    pub key: String,
    pub turns: Vec<LabelledTurn>,
    pub mix: Mix,
    pub by_set: BTreeMap<String, Mix>,
    pub judge_calls: usize,
    pub oracle_misses: usize,
    /// The agent's own work per turn (ms), replay only.
    pub think_ms: Vec<f64>,
    /// Hand labels vs. the automatic labeller on the same turns: (agree, total).
    pub calibration: Option<(usize, usize)>,
    /// Where they disagree: (hand label, automatic label, turns).
    pub disagreements: Vec<(String, String, usize)>,
}

pub fn percentile(v: &[f64], p: f64) -> f64 {
    if v.is_empty() {
        return 0.0;
    }
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let i = ((p / 100.0) * (s.len() - 1) as f64).round() as usize;
    s[i.min(s.len() - 1)]
}

/// Group turns by conversation, keeping file order.
fn conversations(turns: &[RegTurn]) -> Vec<Vec<RegTurn>> {
    let mut out: Vec<Vec<RegTurn>> = Vec::new();
    for t in turns {
        match out.last_mut() {
            Some(c) if c[0].conv == t.conv => c.push(t.clone()),
            _ => out.push(vec![t.clone()]),
        }
    }
    out
}

fn temp_vault(tag: &str) -> PathBuf {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static N: AtomicUsize = AtomicUsize::new(0);
    let d = std::env::temp_dir().join(format!("sla_outcome_{tag}_{}_{}", std::process::id(), N.fetch_add(1, Ordering::SeqCst)));
    let _ = std::fs::remove_dir_all(&d);
    d
}

/// Run the outcome measurement.
pub fn run_outcomes(cfg: &OutcomeConfig) -> std::io::Result<OutcomeRun> {
    let reg = cfg.root.join("data").join("regression");
    let mut turns = load_turns(&reg.join("turns.jsonl"))?;
    if let Some(s) = &cfg.set {
        turns.retain(|t| &t.set == s);
    }
    let gold = load_gold_labels(&reg.join("labels_v1_trainer.tsv"));
    let mut labeller = Labeller::new(cfg.judge.clone(), Some(cfg.root.join("eval").join("label_cache.jsonl")));
    let oracle = match &cfg.parser {
        ParserMode::Oracle(p) => Some(sla_parser::oracle::OracleParser::load(p).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?),
        _ => None,
    };
    let mut run = OutcomeRun { key: cfg.key(), ..Default::default() };

    // 1. The replies: recorded (v1) or produced by this build.
    struct Done {
        turn: RegTurn,
        label_input: TurnToLabel,
        think: f64,
        translate: f64,
        acts: Vec<String>,
    }
    let mut done: Vec<Done> = Vec::new();
    for conv in conversations(&turns) {
        let mut history: Vec<(String, String)> = Vec::new();
        // Replay: a fresh agent on a fresh vault per conversation, on a fixed clock.
        let root = temp_vault("replay");
        let mut agent = None;
        if !cfg.v1 {
            let speaker = if conv[0].set == "trainer" { "friend" } else { "maksim" };
            let mut a = Agent::open(&root, speaker)?;
            a.publish_events = false;
            match &cfg.parser {
                ParserMode::Pattern => {}
                ParserMode::Llm(m) => a.enable_llm(m),
                ParserMode::Oracle(_) => a.set_parser(Box::new(oracle.clone().unwrap_or_default())),
            }
            agent = Some(a);
        }
        let mut clock = Timestamp::from_ymd_hms(2026, 1, 5, 9, 0, 0);
        for t in &conv {
            let (reply, think, translate, acts) = match agent.as_mut() {
                None => (t.v1_reply.clone(), 0.0, 0.0, Vec::new()),
                Some(a) => {
                    clock = clock.plus_secs(60);
                    a.clock = Some(clock);
                    let r = a.respond(&t.user).text;
                    let tr = a.last_trace.clone().unwrap_or_default();
                    (r, tr.think_ms, tr.translate_ms, tr.parts.iter().flat_map(|p| p.acts.clone()).collect())
                }
            };
            let label_input = TurnToLabel { id: t.id.clone(), scenario: t.scenario.clone(), history: history.clone(), user: t.user.clone(), reply: reply.clone() };
            history.push((t.user.clone(), reply));
            done.push(Done { turn: t.clone(), label_input, think, translate, acts });
        }
        drop(agent);
        let _ = std::fs::remove_dir_all(&root);
    }

    // 2. Judge labels for content replies, in parallel, into the cache. For
    //    calibration the hand-labelled turns are judged too.
    if cfg.judge.is_some() {
        let inputs: Vec<TurnToLabel> = done.iter().map(|d| d.label_input.clone()).collect();
        labeller.prefetch(&inputs, 8);
    }

    // 3. One label per turn: hand label (v1 replies only), template, or judge.
    for d in &done {
        let t = &d.turn;
        let (label, source) = match (cfg.v1 && cfg.gold).then(|| gold.get(&t.id)).flatten() {
            Some(l) => (*l, LabelSource::Gold),
            None => labeller.label(&d.label_input),
        };
        run.mix.add(&t.user, label);
        run.by_set.entry(t.set.clone()).or_default().add(&t.user, label);
        if !cfg.v1 {
            run.think_ms.push(d.think);
        }
        run.turns.push(LabelledTurn {
            id: t.id.clone(),
            set: t.set.clone(),
            scenario: t.scenario.clone(),
            user: t.user.clone(),
            reply: d.label_input.reply.clone(),
            label,
            source,
            think_ms: d.think,
            translate_ms: d.translate,
            acts: d.acts.clone(),
        });
    }
    if oracle.is_some() {
        // The oracle answers `unknown:` exactly when a message has no gold parse.
        run.oracle_misses = run.turns.iter().filter(|t| t.acts.iter().all(|a| a == "unknown:")).count();
    }
    // Calibration: on hand-labelled v1 turns, how often does the automatic labeller agree?
    if cfg.v1 && cfg.judge.is_some() {
        let mut agree = 0;
        let mut total = 0;
        let mut confusion: BTreeMap<(Label, Label), usize> = BTreeMap::new();
        for d in &done {
            if let Some(g) = gold.get(&d.turn.id) {
                let (auto, _) = labeller.label(&d.label_input);
                total += 1;
                if auto == *g {
                    agree += 1;
                } else {
                    *confusion.entry((*g, auto)).or_default() += 1;
                }
            }
        }
        run.calibration = Some((agree, total));
        run.disagreements = confusion.into_iter().map(|((g, a), n)| (g.name().to_string(), a.name().to_string(), n)).collect();
    }
    run.judge_calls = labeller.judge_calls;
    Ok(run)
}

pub const CSV_HEADER: &str = "timestamp,commit,key,n,helpful,partial,clarify,teach_me,not_understood,wrong_move,wrong_content,echo,unjudged,good,good_lo,good_hi,request_fit,requests,think_p50_ms,think_p95_ms,notes";

impl OutcomeRun {
    pub fn csv_row(&self, ts: &str, commit: &str, notes: &str) -> String {
        let m = &self.mix;
        let r = |l: Label| format!("{:.3}", m.rate(l).0);
        let (g, lo, hi) = m.good();
        format!(
            "{ts},{commit},{},{},{},{},{},{},{},{},{},{},{},{g:.3},{lo:.3},{hi:.3},{:.3},{},{:.1},{:.1},{}",
            self.key,
            m.n,
            r(Label::Helpful),
            r(Label::Partial),
            r(Label::Clarify),
            r(Label::TeachMe),
            r(Label::NotUnderstood),
            r(Label::WrongMove),
            r(Label::WrongContent),
            r(Label::Echo),
            r(Label::Unjudged),
            m.request_fit().0,
            m.requests,
            percentile(&self.think_ms, 50.0),
            percentile(&self.think_ms, 95.0),
            notes.replace([',', '\n'], ";")
        )
    }

    pub fn markdown(&self, ts: &str, commit: &str) -> String {
        let mut s = format!("# Outcome report {ts}\n\ncommit `{commit}`, run `{}`\n\n## All turns\n\n```\n{}```\n", self.key, self.mix.table());
        for (set, m) in &self.by_set {
            s.push_str(&format!("\n## Set: {set}\n\n```\n{}```\n", m.table()));
        }
        if !self.think_ms.is_empty() {
            s.push_str(&format!(
                "\nAgent's own work per turn: median {:.1} ms, 95th percentile {:.1} ms, max {:.1} ms.\n",
                percentile(&self.think_ms, 50.0),
                percentile(&self.think_ms, 95.0),
                percentile(&self.think_ms, 100.0)
            ));
        }
        if let Some((a, t)) = self.calibration {
            s.push_str(&format!("\nLabeller vs. hand labels on the same turns: {a}/{t} agree.\n"));
        }
        s.push_str("\n## Turns\n\n| id | label | how | user | reply |\n| --- | --- | --- | --- | --- |\n");
        let cell = |x: &str| x.replace('|', "/").replace('\n', " ").chars().take(160).collect::<String>();
        for t in &self.turns {
            s.push_str(&format!("| {} | {} | {:?} | {} | {} |\n", t.id, t.label.name(), t.source, cell(&t.user), cell(&t.reply)));
        }
        s
    }
}

/// The last history row with the same key, as (column -> value).
pub fn previous_row(history: &Path, key: &str) -> Option<BTreeMap<String, String>> {
    let src = std::fs::read_to_string(history).ok()?;
    let mut lines = src.lines();
    let header: Vec<&str> = lines.next()?.split(',').collect();
    lines
        .filter_map(|l| {
            let v: Vec<&str> = l.split(',').collect();
            (v.get(2) == Some(&key)).then(|| header.iter().zip(v.iter()).map(|(h, x)| (h.to_string(), x.to_string())).collect())
        })
        .last()
}

/// Lines comparing this run with a previous row, in percentage points.
pub fn compare(prev: &BTreeMap<String, String>, run: &OutcomeRun) -> Vec<String> {
    let mut out = vec![format!("compared with {} (commit {}):", prev.get("timestamp").map(String::as_str).unwrap_or("?"), prev.get("commit").map(String::as_str).unwrap_or("?"))];
    for l in Label::ALL {
        let Some(old) = prev.get(l.name()).and_then(|x| x.parse::<f64>().ok()) else { continue };
        let new = run.mix.rate(l).0;
        if (new - old).abs() >= 0.0005 {
            out.push(format!("  {:<15} {:5.1}% -> {:5.1}%  ({:+.1} points)", l.name(), old * 100.0, new * 100.0, (new - old) * 100.0));
        }
    }
    if out.len() == 1 {
        out.push("  no change in any label rate".into());
    }
    out
}

pub fn git_commit(root: &Path) -> String {
    std::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .current_dir(root)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|| "unknown".into())
}

/// Run, print, append the row and write the report. Returns the run.
pub fn outcome_command(cfg: &OutcomeConfig) -> std::io::Result<OutcomeRun> {
    let run = run_outcomes(cfg)?;
    let ts = Timestamp::now().to_string();
    let ts = ts.trim_end_matches('Z').to_string();
    let commit = git_commit(&cfg.root);
    println!("outcome mix, run {} (commit {commit})\n{}", run.key, run.mix.table());
    for (set, m) in &run.by_set {
        let (g, lo, hi) = m.good();
        println!("  set {set:<8} {:>4} turns: helpful + clarify {:.1}% [{:.1}, {:.1}], not understood {:.1}%", m.n, g * 100.0, lo * 100.0, hi * 100.0, m.rate(Label::NotUnderstood).0 * 100.0);
    }
    if !run.think_ms.is_empty() {
        println!("agent's own work per turn: median {:.1} ms, 95th percentile {:.1} ms", percentile(&run.think_ms, 50.0), percentile(&run.think_ms, 95.0));
        let tr: Vec<f64> = run.turns.iter().map(|t| t.translate_ms).collect();
        println!("translation per turn: median {:.0} ms, 95th percentile {:.0} ms", percentile(&tr, 50.0), percentile(&tr, 95.0));
    }
    if let Some((a, t)) = run.calibration {
        println!("labeller agrees with the hand labels on {a} of {t} turns ({:.0}%)", a as f64 * 100.0 / t.max(1) as f64);
        for (g, x, n) in &run.disagreements {
            println!("  hand {g}, labeller {x}: {n}");
        }
    }
    if run.judge_calls > 0 {
        println!("judge calls: {}", run.judge_calls);
    }
    // Failed model calls make a run meaningless: say so, and do not record it.
    let failed: u64 = sla_parser::claude::global_usage().values().map(|u| u.errors).sum();
    let calls: u64 = sla_parser::claude::global_usage().values().map(|u| u.calls).sum();
    if calls > 0 && failed * 10 > calls {
        eprintln!("WARNING: {failed} of {calls} model calls failed (plan limit or rate limit?); this run is not recorded.");
        return Ok(run);
    }
    if cfg.write {
        let history = cfg.root.join("eval").join("outcomes.csv");
        if let Some(prev) = previous_row(&history, &run.key) {
            for l in compare(&prev, &run) {
                println!("{l}");
            }
        }
        let new = !history.exists();
        let mut text = if new { format!("{CSV_HEADER}\n") } else { String::new() };
        text.push_str(&run.csv_row(&ts, &commit, &cfg.notes));
        text.push('\n');
        use std::io::Write;
        std::fs::OpenOptions::new().create(true).append(true).open(&history)?.write_all(text.as_bytes())?;
        let dir = cfg.root.join("eval").join("outcomes");
        std::fs::create_dir_all(&dir)?;
        let file = dir.join(format!("{}.md", ts.replace(':', "-")));
        std::fs::write(&file, run.markdown(&ts, &commit))?;
        println!("appended a row to {} and wrote {}", history.display(), file.display());
    }
    Ok(run)
}
