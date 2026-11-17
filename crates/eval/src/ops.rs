//! Milestone 12 acceptance commands that need no model:
//!
//! - `bench`: the agent's own work per turn on a copy of a vault (the v1
//!   vault for the acceptance test), replaying the regression set's messages;
//! - `stress`: the real trainer with N parallel teachers for M minutes, every
//!   model role played by a mock, then every log, event, trace and cache line
//!   is checked.

use crate::regression::{load_turns, percentile};
use crate::trainer::{TrainConfig, Trainer};
use sla_agent::{Agent, Brain};
use sla_core::Timestamp;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

fn arg<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).map(String::as_str)
}

fn temp(tag: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("sla_{tag}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    d
}

#[derive(Debug, Clone, Default)]
pub struct BenchResult {
    pub turns: usize,
    pub facts_at_start: usize,
    pub think_ms: Vec<f64>,
    pub evaluations: u64,
    pub eval_ms: f64,
    pub learn_ms: f64,
    pub slowest: Vec<(f64, String, u64, f64, f64)>,
}

/// Replay the regression messages `rounds` times on a copy of `vault`.
pub fn bench(root: &Path, vault: &Path, rounds: usize) -> std::io::Result<BenchResult> {
    let copy = temp("bench").join("vault");
    crate::copy_dir(vault, &copy)?;
    let brain = Brain::shared(&copy)?;
    let mut res = BenchResult { facts_at_start: sla_agent::brain::lock(&brain).kb.all_facts().len(), ..Default::default() };
    let turns = load_turns(&root.join("data").join("regression").join("turns.jsonl"))?;
    let mut clock = Timestamp::from_ymd_hms(2026, 11, 17, 9, 0, 0);
    for _ in 0..rounds {
        let mut agent: Option<(String, Agent)> = None;
        for t in &turns {
            if agent.as_ref().map(|(c, _)| c != &t.conv).unwrap_or(true) {
                let mut a = Agent::with_brain(Arc::clone(&brain), if t.set == "trainer" { "friend" } else { "maksim" });
                a.publish_events = false;
                agent = Some((t.conv.clone(), a));
            }
            let a = &mut agent.as_mut().unwrap().1;
            clock = clock.plus_secs(30);
            a.clock = Some(clock);
            a.respond(&t.user);
            let tr = a.last_trace.as_ref().unwrap();
            res.think_ms.push(tr.think_ms);
            res.evaluations += tr.evaluations;
            res.eval_ms += tr.eval_ms;
            res.learn_ms += tr.learn_ms;
            res.slowest.push((tr.think_ms, t.user.chars().take(70).collect(), tr.evaluations, tr.eval_ms, tr.learn_ms));
            res.turns += 1;
        }
    }
    drop(brain);
    let _ = std::fs::remove_dir_all(copy.parent().unwrap());
    Ok(res)
}

pub fn bench_command(root: &Path, args: &[String]) {
    let Some(vault) = arg(args, "--vault") else {
        eprintln!("usage: sla_eval bench --vault DIR [--rounds N]");
        std::process::exit(2);
    };
    let rounds = arg(args, "--rounds").and_then(|r| r.parse().ok()).unwrap_or(1);
    match bench(root, Path::new(vault), rounds) {
        Ok(r) => {
            println!(
                "{} turns on a copy of {} ({} facts at start): agent's own work median {:.1} ms, 95th percentile {:.1} ms, max {:.1} ms; {:.2} evaluations per turn",
                r.turns,
                vault,
                r.facts_at_start,
                percentile(&r.think_ms, 50.0),
                percentile(&r.think_ms, 95.0),
                percentile(&r.think_ms, 100.0),
                r.evaluations as f64 / r.turns.max(1) as f64
            );
            println!("time in evaluations {:.0} ms, in learner updates {:.0} ms, total {:.0} ms", r.eval_ms, r.learn_ms, r.think_ms.iter().sum::<f64>());
            let mut slow = r.slowest.clone();
            slow.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
            for (ms, msg, ev, em, lm) in slow.iter().take(8) {
                println!("  {ms:7.1} ms  evals {ev} ({em:.0} ms)  learn {lm:.0} ms  {msg}");
            }
            let p95 = percentile(&r.think_ms, 95.0);
            println!("budget (95th percentile under 100 ms): {}", if p95 < 100.0 { "met" } else { "NOT met" });
        }
        Err(e) => {
            eprintln!("bench failed: {e}");
            std::process::exit(1);
        }
    }
}

/// Lines that do not parse, per kind of file.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Integrity {
    pub log_lines: usize,
    pub bad_log_lines: Vec<String>,
    pub vault_errors: Vec<String>,
    pub events: usize,
    pub bad_events: usize,
    pub traces: usize,
    pub bad_traces: usize,
    pub cache_lines: usize,
    pub bad_cache_lines: usize,
}

impl Integrity {
    pub fn corrupted(&self) -> usize {
        self.bad_log_lines.len() + self.vault_errors.len() + self.bad_events + self.bad_traces + self.bad_cache_lines
    }
}

/// Check every line the vault, the event stream and the traces contain.
pub fn check_integrity(vault: &Path, sessions: &Path) -> Integrity {
    let mut r = Integrity::default();
    if let Ok(rd) = std::fs::read_dir(vault.join("log")) {
        for e in rd.flatten() {
            for l in std::fs::read_to_string(e.path()).unwrap_or_default().lines() {
                if l.starts_with("- ") {
                    r.log_lines += 1;
                    let p: Vec<&str> = l[2..].splitn(4, " | ").collect();
                    if p.len() != 4 || Timestamp::parse(p[0]).is_none() {
                        r.bad_log_lines.push(l.to_string());
                    }
                } else if !(l.is_empty() || l.starts_with("# Change log")) {
                    r.bad_log_lines.push(l.to_string());
                }
            }
        }
    }
    match Brain::open(vault) {
        Ok(b) => r.vault_errors = b.kb.vault.errors().into_iter().map(|(f, l, e)| format!("{f}: {l} ({e})")).collect(),
        Err(e) => r.vault_errors.push(e.to_string()),
    }
    if let Ok(rd) = std::fs::read_dir(sessions) {
        for e in rd.flatten() {
            let p = e.path();
            let text = std::fs::read_to_string(&p).unwrap_or_default();
            let is_events = p.file_name().and_then(|n| n.to_str()) == Some("events.jsonl");
            for l in text.lines() {
                if is_events {
                    r.events += 1;
                    if serde_json::from_str::<sla_agent::events::Event>(l).is_err() {
                        r.bad_events += 1;
                    }
                } else {
                    r.traces += 1;
                    if serde_json::from_str::<sla_agent::TurnTrace>(l).is_err() {
                        r.bad_traces += 1;
                    }
                }
            }
        }
    }
    for l in std::fs::read_to_string(vault.join("llm_cache.tsv")).unwrap_or_default().lines() {
        r.cache_lines += 1;
        if l.split('\t').count() != 2 {
            r.bad_cache_lines += 1;
        }
    }
    r
}

/// Play every model role with a mock: teachers say the regression set's
/// messages, the translator answers with the pattern parser's reading, the
/// reviewer approves.
pub fn install_mock_models(root: &Path) -> std::io::Result<()> {
    let msgs: Arc<Vec<String>> = Arc::new(load_turns(&root.join("data").join("regression").join("turns.jsonl"))?.into_iter().map(|t| t.user).collect());
    let next = Arc::new(AtomicUsize::new(0));
    sla_parser::claude::set_global_mock(Arc::new(move |role, prompt| match role {
        "teacher" => {
            let i = next.fetch_add(1, Ordering::SeqCst);
            let m = &msgs[i % msgs.len()];
            Ok(if i % 9 == 8 { format!("{m} [done]") } else { m.clone() })
        }
        "translator" => {
            let text = prompt.rsplit("Message: ").next().unwrap_or("");
            Ok(sla_parser::parse(text, &sla_parser::Context { speaker: "friend".into(), last_entity: None }).canonical())
        }
        "reviewer" => Ok("1: OK".into()),
        _ => Err(format!("no mock for role {role}")),
    }));
    Ok(())
}

#[derive(Debug, Clone, Default)]
pub struct StressResult {
    /// Runs started (each at most 1000 conversations) and wall-clock seconds.
    pub runs: usize,
    pub seconds: f64,
    pub conversations: usize,
    pub turns: usize,
    pub think_ms: Vec<f64>,
    pub integrity: Integrity,
}

/// Run the trainer with `teachers` parallel teachers for `secs` seconds on
/// mock models, then check every file.
pub fn stress(root: &Path, base: &Path, teachers: usize, secs: u64) -> std::io::Result<StressResult> {
    install_mock_models(root)?;
    let vault = base.join("vault");
    let trainer = Trainer::new(vault.clone());
    let started = std::time::Instant::now();
    let until = started + std::time::Duration::from_secs(secs);
    let mut res = StressResult::default();
    // A run holds at most 1000 conversations, and mock teachers are fast:
    // start run after run on the same vault until the time is used up.
    loop {
        trainer
            .start(TrainConfig { teachers, turns: 10, conversations: 1000, model: "mock".into(), review: true })
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        loop {
            let s = trainer.snapshot();
            if !s.running {
                break;
            }
            if std::time::Instant::now() >= until {
                trainer.request_stop();
                trainer.wait();
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        let s = trainer.snapshot();
        res.conversations += s.completed;
        res.turns += s.turns;
        res.think_ms.extend(s.think_ms);
        res.runs += 1;
        if std::time::Instant::now() >= until {
            break;
        }
    }
    res.seconds = started.elapsed().as_secs_f64();
    sla_parser::claude::clear_mocks();
    res.integrity = check_integrity(&vault, &trainer.sessions);
    Ok(res)
}

pub fn stress_command(root: &Path, args: &[String]) {
    let teachers = arg(args, "--teachers").and_then(|v| v.parse().ok()).unwrap_or(16);
    let minutes: f64 = arg(args, "--minutes").and_then(|v| v.parse().ok()).unwrap_or(10.0);
    let base = temp("stress");
    println!("stress: {teachers} teachers for {minutes} minutes on mock models, vault {}", base.join("vault").display());
    match stress(root, &base, teachers, (minutes * 60.0) as u64) {
        Ok(r) => {
            let i = &r.integrity;
            println!(
                "{:.0} s, {} runs, {} conversations, {} turns; agent's own work median {:.1} ms, 95th percentile {:.1} ms",
                r.seconds,
                r.runs,
                r.conversations,
                r.turns,
                percentile(&r.think_ms, 50.0),
                percentile(&r.think_ms, 95.0)
            );
            println!(
                "checked {} log lines, {} events, {} trace lines, {} cache lines: {} corrupted",
                i.log_lines,
                i.events,
                i.traces,
                i.cache_lines,
                i.corrupted()
            );
            for l in i.bad_log_lines.iter().take(5) {
                println!("  bad log line: {l}");
            }
            for l in i.vault_errors.iter().take(5) {
                println!("  vault error: {l}");
            }
            let _ = std::fs::remove_dir_all(&base);
            if i.corrupted() > 0 {
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("stress failed: {e}");
            std::process::exit(1);
        }
    }
}
