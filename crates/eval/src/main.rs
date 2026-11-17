//! One command for the full evaluation:
//!
//! ```text
//! cargo run --release -p sla_eval -- [--judges N] [--turns N] [--teacher-questions N] [--no-claude] [--model M] [--notes TEXT]
//! ```
//! Appends one row to `eval/history.csv` and writes `eval/last_report.md`.
//!
//! Design 2.0 subcommands:
//!
//! ```text
//! sla_eval outcomes [--v1] [--gold] [--parser pattern|llm[:MODEL]|oracle:FILE] [--judge MODEL] [--set trainer|judge] [--notes TEXT] [--no-write]
//! sla_eval bench --vault DIR [--rounds N]
//! sla_eval stress [--teachers N] [--minutes M] [--vault DIR]
//! ```

use sla_eval::regression::{outcome_command, OutcomeConfig, ParserMode};
use sla_eval::{run, EvalConfig, EvalReport};

fn root() -> std::path::PathBuf {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..");
    let root = root.canonicalize().unwrap_or(root);
    // Windows: drop the extended-length prefix that canonicalize adds.
    let s = root.to_string_lossy().to_string();
    match s.strip_prefix(r"\\?\") {
        Some(rest) => rest.into(),
        None => root,
    }
}

fn outcomes(args: &[String]) {
    let mut cfg = OutcomeConfig::new(root());
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--v1" => cfg.v1 = true,
            "--gold" => cfg.gold = true,
            "--judge" => cfg.judge = Some(it.next().expect("--judge MODEL").clone()),
            "--set" => cfg.set = Some(it.next().expect("--set NAME").clone()),
            "--notes" => cfg.notes = it.next().expect("--notes TEXT").clone(),
            "--no-write" => cfg.write = false,
            "--parser" => {
                let p = it.next().expect("--parser pattern|llm[:MODEL]|oracle:FILE");
                cfg.parser = if p == "pattern" {
                    ParserMode::Pattern
                } else if let Some(m) = p.strip_prefix("llm") {
                    ParserMode::Llm(m.trim_start_matches(':').to_string()).clone()
                } else if let Some(f) = p.strip_prefix("oracle:") {
                    ParserMode::Oracle(f.into())
                } else {
                    eprintln!("unknown parser {p}");
                    std::process::exit(2);
                };
                if cfg.parser == ParserMode::Llm(String::new()) {
                    cfg.parser = ParserMode::Llm("haiku".into());
                }
            }
            other => {
                eprintln!("unknown argument {other}");
                std::process::exit(2);
            }
        }
    }
    if let Err(e) = outcome_command(&cfg) {
        eprintln!("outcomes failed: {e}");
        std::process::exit(1);
    }
}

/// `sla_eval gold [--set translator_gold|long_messages] [--model haiku] [--parallel 8]`
fn gold_command(args: &[String]) {
    let arg = |n: &str| args.iter().position(|a| a == n).and_then(|i| args.get(i + 1)).cloned();
    let set = arg("--set").unwrap_or_else(|| "translator_gold".into());
    let model = arg("--model").unwrap_or_else(|| "haiku".into());
    let parallel = arg("--parallel").and_then(|p| p.parse().ok()).unwrap_or(8);
    let root = root();
    let use_cache = !args.iter().any(|a| a == "--no-cache");
    match sla_eval::gold::run(&root, &set, &model, parallel, use_cache) {
        Ok(r) => {
            let md = sla_eval::gold::markdown(&r);
            println!("{}", md.lines().take(3).collect::<Vec<_>>().join("
"));
            let ts = sla_core::Timestamp::now().to_string().replace(':', "-");
            let dir = root.join("eval").join("translator");
            let _ = std::fs::create_dir_all(&dir);
            let file = dir.join(format!("{ts}_{set}.md"));
            let _ = std::fs::write(&file, md);
            // The translations are training data too.
            let _ = sla_eval::archive::append(&sla_eval::archive::dir(&root), "translator", &serde_json::json!({ "set": set, "model": model, "items": r.items }));
            println!("wrote {}", file.display());
        }
        Err(e) => {
            eprintln!("gold failed: {e}");
            std::process::exit(1);
        }
    }
}

/// `sla_eval wording [--model haiku] [--ceiling] [--parallel 8]`
fn wording_command(args: &[String]) {
    let model = args.iter().position(|a| a == "--model").and_then(|i| args.get(i + 1)).cloned().unwrap_or_else(|| "haiku".into());
    let ceiling = args.iter().any(|a| a == "--ceiling");
    let r = sla_eval::wording::run(&model, ceiling, 8);
    let failed: u64 = sla_parser::claude::global_usage().values().map(|u| u.errors).sum();
    let calls: u64 = sla_parser::claude::global_usage().values().map(|u| u.calls).sum();
    if calls > 0 && failed * 10 > calls {
        eprintln!("WARNING: {failed} of {calls} model calls failed (plan limit or rate limit?); nothing recorded.");
        std::process::exit(1);
    }
    let md = sla_eval::wording::markdown(&r);
    for l in md.lines().skip(2).take(6).filter(|l| !l.is_empty()) {
        println!("{l}");
    }
    let root = root();
    let ts = sla_core::Timestamp::now().to_string().replace(':', "-");
    let dir = root.join("eval").join("wording");
    let _ = std::fs::create_dir_all(&dir);
    let file = dir.join(format!("{ts}.md"));
    let _ = std::fs::write(&file, md);
    let _ = sla_eval::archive::append(&sla_eval::archive::dir(&root), "wording", &r);
    println!("wrote {}", file.display());
}

fn main() {
    let all: Vec<String> = std::env::args().skip(1).collect();
    match all.first().map(String::as_str) {
        Some("outcomes") => return outcomes(&all[1..]),
        Some("bench") => return sla_eval::ops::bench_command(&root(), &all[1..]),
        Some("stress") => return sla_eval::ops::stress_command(&root(), &all[1..]),
        Some("gold") => return gold_command(&all[1..]),
        Some("wording") => return wording_command(&all[1..]),
        _ => {}
    }
    let root = root();
    let mut cfg = EvalConfig::new(&root);
    let mut args = all.into_iter();
    while let Some(a) = args.next() {
        let mut num = |name: &str| -> usize { args.next().and_then(|v| v.parse().ok()).unwrap_or_else(|| panic!("{name} needs a number")) };
        match a.as_str() {
            "--judges" => cfg.judges = num("--judges"),
            "--turns" => cfg.turns = num("--turns"),
            "--teacher-questions" => cfg.teacher_questions = num("--teacher-questions"),
            "--no-claude" => cfg.use_claude = false,
            "--llm" => cfg.llm = true,
            "--seed-vault" => cfg.seed_vault = Some(args.next().expect("--seed-vault DIR").into()),
            "--model" => cfg.model = args.next().expect("--model NAME"),
            "--notes" => cfg.notes = args.next().expect("--notes TEXT"),
            "--history" => cfg.history = args.next().expect("--history PATH").into(),
            "-h" | "--help" => {
                println!("sla_eval [--judges N] [--turns N] [--teacher-questions N] [--no-claude] [--model M] [--notes TEXT] [--history PATH]");
                println!("sla_eval outcomes [--v1] [--gold] [--parser pattern|llm[:MODEL]|oracle:FILE] [--judge MODEL] [--set trainer|judge] [--notes TEXT] [--no-write]");
                println!("sla_eval bench --vault DIR [--rounds N]");
                println!("sla_eval stress [--teachers N] [--minutes M]");
                return;
            }
            other => {
                eprintln!("unknown argument {other}");
                std::process::exit(2);
            }
        }
    }
    match run(&cfg) {
        Ok(rep) => {
            let (cd, td) = EvalReport::checks(&rep.dev);
            let (ch, th) = EvalReport::checks(&rep.held_out);
            println!("parser: dev {:.1}%, held-out {:.1}%", rep.parser_dev * 100.0, rep.parser_held_out * 100.0);
            println!(
                "dialogues: dev checks {cd}/{td} (knowledge {:.1}%), held-out checks {ch}/{th} (knowledge {:.1}%)",
                EvalReport::knowledge(&rep.dev) * 100.0,
                EvalReport::knowledge(&rep.held_out) * 100.0
            );
            let (tq, tc) = rep.teacher();
            println!("live teacher: {tq} questions, {tc} consistent");
            match rep.conversation() {
                Some(c) => println!("conversation quality: {c:.2}/10 from {} judge(s)", rep.judges.iter().filter(|j| j.score.is_some()).count()),
                None => println!("conversation quality: no judge scores"),
            }
            for j in &rep.judges {
                println!("  judge [{}]: {} - {}", j.topic, j.score.map(|s| s.to_string()).unwrap_or("-".into()), j.reason);
            }
            println!("appended a row to {}", cfg.history.display());
        }
        Err(e) => {
            eprintln!("evaluation failed: {e}");
            std::process::exit(1);
        }
    }
}
