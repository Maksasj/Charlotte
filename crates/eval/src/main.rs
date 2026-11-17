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

/// `sla_eval curriculum [--model haiku] [--judge haiku] [--limit N] [--no-seeds] [--vault DIR] [--train]`
/// (`--train`: first the curriculum over data/curriculum/training, then the held-out run.)
/// The four encounters over the held-out scenarios on a fresh vault (seeded
/// with the 30 seed procedures), scripted teacher, model translator and judge.
fn curriculum_command(args: &[String]) {
    let arg = |k: &str| args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).cloned();
    let model = arg("--model").unwrap_or_else(|| "haiku".into());
    let judge = arg("--judge").unwrap_or_else(|| "haiku".into());
    let limit: usize = arg("--limit").and_then(|v| v.parse().ok()).unwrap_or(usize::MAX);
    let root = root();
    let ts = sla_core::Timestamp::now().to_string().replace(':', "-");
    let vault = arg("--vault").map(std::path::PathBuf::from).unwrap_or_else(|| std::env::temp_dir().join(format!("sla_curriculum_{ts}")));
    let brain = sla_agent::Brain::shared(&vault).expect("vault");
    let seeds = if args.iter().any(|a| a == "--no-seeds") { 0 } else { sla_eval::curriculum::install_seeds(&brain, &root.join("data/curriculum/seed")) };
    sla_agent::brain::lock(&brain).install(&root.join("data/scenarios/dinner")).ok();
    let scenarios: Vec<_> = sla_eval::curriculum::load_scenarios(&root.join("data/curriculum/scenarios")).into_iter().take(limit).collect();
    let mut cfg = sla_eval::curriculum::CurriculumConfig {
        user_agent: sla_eval::curriculum::llm_user(model.clone(), Some(root.join("eval").join("translate_cache.jsonl"))),
        teacher_agent: sla_eval::curriculum::llm_teacher(model.clone(), Some(root.join("eval").join("translate_cache.jsonl"))),
        judge: Box::new(sla_eval::curriculum::ModelJudge::new(judge.clone(), Some(root.join("eval").join("curriculum_judge_cache.jsonl")))),
        labeller: Some(sla_eval::outcome::Labeller::new(Some(judge.clone()), Some(root.join("eval").join("label_cache.jsonl")))),
        max_turns: 4,
        teacher_model: None,
    };
    let live = root.join("data").join("training").join("curriculum").join("live").join(format!("{ts}.jsonl"));
    sla_eval::curriculum::set_live(live.clone());
    sla_eval::curriculum::live_vault(&vault);
    eprintln!("live transcript: {} (watch: python tools/curriculum_live.py)", live.display());
    // Training mode: the curriculum over the training scenarios first (never the
    // held-out ones), on the same vault; then the held-out measurement below.
    if args.iter().any(|a| a == "--train") {
        let train = sla_eval::curriculum::load_scenarios(&root.join("data/curriculum/training"));
        eprintln!("training on {} scenarios first", train.len());
        // In training the in-role teacher (Haiku by default) also teaches variants.
        cfg.teacher_model = Some(arg("--teacher").unwrap_or_else(|| "haiku".into()));
        let t = sla_eval::curriculum::run(&mut cfg, &brain, &train, seeds);
        cfg.teacher_model = None;
        let tmd = sla_eval::curriculum::markdown(&t);
        let dir = root.join("eval").join("curriculum");
        let _ = std::fs::create_dir_all(&dir);
        let _ = std::fs::write(dir.join(format!("{ts}-training.md")), &tmd);
        let _ = sla_eval::archive::append(&sla_eval::archive::dir(&root), "curriculum_training", &t);
        println!("training stage:");
        for l in tmd.lines().skip(4).take(14) {
            println!("{l}");
        }
        println!("held-out stage:");
    }
    let r = sla_eval::curriculum::run(&mut cfg, &brain, &scenarios, seeds);
    let failed: u64 = sla_parser::claude::global_usage().values().map(|u| u.errors).sum();
    let calls: u64 = sla_parser::claude::global_usage().values().map(|u| u.calls).sum();
    let md = sla_eval::curriculum::markdown(&r);
    for l in md.lines().skip(4).take(14) {
        println!("{l}");
    }
    let dir = root.join("eval").join("curriculum");
    let _ = std::fs::create_dir_all(&dir);
    let file = dir.join(format!("{ts}.md"));
    let _ = std::fs::write(&file, &md);
    // Every conversation is training data: keep it.
    let _ = sla_eval::archive::append(&sla_eval::archive::dir(&root), "curriculum", &r);
    // Keep what the agent learned: a copy of the run's vault.
    let keep = root.join("data").join("training").join("curriculum").join("vaults").join(&ts);
    drop(brain);
    if let Err(e) = copy_dir(&vault, &keep) {
        eprintln!("could not keep the vault: {e}");
    }
    println!("model calls: {calls} ({failed} failed); vault {}; wrote {}", vault.display(), file.display());
    if calls > 0 && failed * 10 > calls {
        eprintln!("WARNING: many model calls failed; treat this run as invalid.");
    }
}

fn copy_dir(from: &std::path::Path, to: &std::path::Path) -> std::io::Result<()> {
    std::fs::create_dir_all(to)?;
    for e in std::fs::read_dir(from)? {
        let e = e?;
        let p = e.path();
        let name = e.file_name();
        // The translation cache is shared (eval/); the vault keeps knowledge only.
        if name == "translate_cache.jsonl" {
            continue;
        }
        if p.is_dir() {
            copy_dir(&p, &to.join(&name))?;
        } else {
            std::fs::copy(&p, to.join(&name))?;
        }
    }
    Ok(())
}

/// `sla_eval calibrate`: re-grade every recorded full curriculum run with the
/// symbolic checklist judge and compare with the model judge.
fn calibrate_command() {
    let root = root();
    let scenarios = sla_eval::curriculum::load_scenarios(&root.join("data/curriculum/scenarios"));
    let mut files: Vec<_> = std::fs::read_dir(root.join("data/training/curriculum"))
        .map(|d| d.flatten().map(|e| e.path()).filter(|p| p.extension().map_or(false, |x| x == "jsonl")).collect())
        .unwrap_or_default();
    files.sort();
    let mut md = String::from(
        "# Checklist judge calibration\n\nThe model (Haiku) checklist verdicts against a symbolic check from the ground truth \
         (needs named, step actions in order, total time), on encounters without a variant.\n\n\
         | Run | Encounters | Agree | Model pass, symbolic fail | Model fail, symbolic pass | Second encounter (model) | Second encounter (symbolic) |\n\
         |---|---|---|---|---|---|---|\n",
    );
    let mut notes = String::new();
    for f in files {
        for (i, line) in std::fs::read_to_string(&f).unwrap_or_default().lines().enumerate() {
            let Ok(v) = serde_json::from_str::<serde_json::Value>(line) else { continue };
            if v["scenarios"].as_u64() != Some(20) {
                continue;
            }
            let c = sla_eval::curriculum::calibrate(&v, &scenarios);
            let name = format!("{} #{}", f.file_stem().and_then(|s| s.to_str()).unwrap_or(""), i + 1);
            let row = format!(
                "| {name} | {} | {} | {} | {} | {} of {} | {} of {} |\n",
                c.encounters,
                c.both_pass + c.both_fail,
                c.model_only,
                c.symbolic_only,
                c.repeat_model,
                c.repeat_total,
                c.repeat_symbolic,
                c.repeat_total
            );
            print!("{row}");
            md.push_str(&row);
            notes.push_str(&format!("\n## {name}\n\n"));
            for d in &c.disagreements {
                notes.push_str(&format!("- {d}\n"));
            }
        }
    }
    // Hand labels (eval/curriculum/hand_labels.md, from tools/make_hand_labels.py)
    // against the last recorded run that has both verdicts.
    let hand = std::fs::read_to_string(root.join("eval").join("curriculum").join("hand_labels.md")).unwrap_or_default();
    let mut labels: Vec<(String, String, bool)> = Vec::new();
    let mut cur: Option<(String, String)> = None;
    for l in hand.lines() {
        if let Some(h) = l.strip_prefix("## ") {
            // "N. scenario (kind)"
            let rest = h.split_once(". ").map(|x| x.1).unwrap_or(h);
            if let Some((sc, k)) = rest.split_once(" (") {
                cur = Some((sc.trim().to_string(), k.trim_end_matches(')').to_string()));
            }
        } else if let Some(v) = l.strip_prefix("label:") {
            let v = v.trim().to_lowercase();
            if let (Some((sc, k)), true) = (cur.take(), v.starts_with("pass") || v.starts_with("fail")) {
                labels.push((sc, k, v.starts_with("pass")));
            }
        }
    }
    if !labels.is_empty() {
        let mut last: Option<serde_json::Value> = None;
        for f in std::fs::read_dir(root.join("data/training/curriculum")).map(|d| d.flatten().map(|e| e.path()).collect::<Vec<_>>()).unwrap_or_default() {
            if f.extension().map_or(true, |x| x != "jsonl") {
                continue;
            }
            for line in std::fs::read_to_string(&f).unwrap_or_default().lines() {
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(line) {
                    if v["scenarios"].as_u64() == Some(20) && v["encounters"].as_array().map_or(false, |a| a.iter().any(|e| e.get("symbolic_pass").is_some())) {
                        last = Some(v);
                    }
                }
            }
        }
        if let Some(run) = last {
            let (mut n, mut model_ok, mut sym_ok) = (0, 0, 0);
            for (sc, k, hand_pass) in &labels {
                let Some(e) = run["encounters"].as_array().into_iter().flatten().find(|e| e["scenario"].as_str() == Some(sc) && e["kind"].as_str() == Some(k)) else { continue };
                n += 1;
                model_ok += (e["passed"].as_bool() == Some(*hand_pass)) as usize;
                sym_ok += (e["symbolic_pass"].as_bool() == Some(*hand_pass)) as usize;
            }
            let row = format!("\nAgainst {n} hand labels: the model judge agrees on {model_ok}, the symbolic check on {sym_ok}.\n");
            print!("{row}");
            md.push_str(&row);
        }
    }
    md.push_str(&notes);
    let file = root.join("eval").join("curriculum").join("calibration.md");
    let _ = std::fs::write(&file, md);
    println!("wrote {}", file.display());
}

/// `sla_eval rate [--judge haiku]`: the judge's 1-10 score for the
/// second-encounter and retention conversations of the latest full curriculum run.
fn rate_command(args: &[String]) {
    let judge = args.iter().position(|a| a == "--judge").and_then(|i| args.get(i + 1)).cloned().unwrap_or_else(|| "haiku".into());
    let root = root();
    let mut last: Option<serde_json::Value> = None;
    let mut files: Vec<_> = std::fs::read_dir(root.join("data/training/curriculum")).map(|d| d.flatten().map(|e| e.path()).filter(|p| p.extension().map_or(false, |x| x == "jsonl")).collect()).unwrap_or_default();
    files.sort();
    for f in files {
        for line in std::fs::read_to_string(&f).unwrap_or_default().lines() {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(line) {
                if v["scenarios"].as_u64() == Some(20) {
                    last = Some(v);
                }
            }
        }
    }
    let Some(run) = last else {
        eprintln!("no full curriculum run recorded");
        return;
    };
    let j = sla_eval::curriculum::ModelJudge::new(judge, Some(root.join("eval").join("curriculum_judge_cache.jsonl")));
    let scores = sla_eval::curriculum::rate_conversations(&run, &j);
    let all: Vec<f64> = scores.iter().map(|x| x.2).collect();
    let (m, lo, hi) = sla_eval::curriculum::mean_interval(&all);
    let rep: Vec<f64> = scores.iter().filter(|x| x.1 == "repeat").map(|x| x.2).collect();
    let (rm, rlo, rhi) = sla_eval::curriculum::mean_interval(&rep);
    let mut md = format!(
        "# Judge score\n\n{} conversations (second encounter and retention) of the latest full curriculum run, rated 1 to 10.\n\n\
         - all: mean **{m:.2}** (95% interval {lo:.2} to {hi:.2}); target 6.0 or more, interval above v1's best 4.3\n\
         - second encounter only: mean {rm:.2} ({rlo:.2} to {rhi:.2})\n\n| Scenario | Encounter | Score |\n|---|---|---|\n",
        all.len()
    );
    for (s, k, x) in &scores {
        md.push_str(&format!("| {s} | {k} | {x} |\n"));
    }
    println!("{} conversations: mean {m:.2} (95% interval {lo:.2} to {hi:.2}); second encounter {rm:.2}", all.len());
    let _ = std::fs::write(root.join("eval").join("curriculum").join("judge_score.md"), md);
}

/// `sla_eval pairwise [--judge haiku]`: the latest full curriculum run's
/// second-encounter replies against the milestone 12 build's replies to the same
/// requests after the same teaching (eval/curriculum/m12_baseline.json, from
/// tools/m12_baseline.py), judged pairwise in alternating order.
fn pairwise_command(args: &[String]) {
    let judge = args.iter().position(|a| a == "--judge").and_then(|i| args.get(i + 1)).cloned().unwrap_or_else(|| "haiku".into());
    let root = root();
    let base: serde_json::Value = match std::fs::read_to_string(root.join("eval/curriculum/m12_baseline.json")).ok().and_then(|t| serde_json::from_str(&t).ok()) {
        Some(v) => v,
        None => {
            eprintln!("no eval/curriculum/m12_baseline.json: run tools/m12_baseline.py first");
            return;
        }
    };
    let mut last: Option<serde_json::Value> = None;
    let mut files: Vec<_> = std::fs::read_dir(root.join("data/training/curriculum")).map(|d| d.flatten().map(|e| e.path()).filter(|p| p.extension().map_or(false, |x| x == "jsonl")).collect()).unwrap_or_default();
    files.sort();
    for f in files {
        for line in std::fs::read_to_string(&f).unwrap_or_default().lines() {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(line) {
                if v["scenarios"].as_u64() == Some(20) {
                    last = Some(v);
                }
            }
        }
    }
    let Some(run) = last else { return };
    let j = sla_eval::curriculum::ModelJudge::new(judge, Some(root.join("eval").join("curriculum_judge_cache.jsonl")));
    let (mut wins, mut n) = (0usize, 0usize);
    let mut md = String::from("# Pairwise: this build against the milestone 12 build\n\nSame teaching, same second-encounter request; the judge sees both replies in alternating order.\n\n");
    let mut rows = String::new();
    for (i, e) in run["encounters"].as_array().into_iter().flatten().filter(|e| e["kind"] == "repeat").enumerate() {
        let sid = e["scenario"].as_str().unwrap_or("");
        let Some(old) = base[sid]["reply"].as_str().filter(|r| !r.is_empty()) else { continue };
        let Some(turn) = e["turns"].as_array().and_then(|t| t.first()) else { continue };
        let request = turn["user"].as_str().unwrap_or("");
        let new = turn["agent"].as_str().unwrap_or("");
        let (a, b, new_is_a) = if i % 2 == 0 { (new, old, true) } else { (old, new, false) };
        let prompt = format!(
            "A user asked an assistant: \"{request}\"\n\nReply A: {a}\n\nReply B: {b}\n\nWhich reply helps the user more (correct, complete and clear)? Answer with exactly one letter: A or B."
        );
        let Ok(out) = j.ask(&prompt) else { continue };
        let pick = out.trim().trim_matches(|c: char| !c.is_ascii_alphabetic()).chars().next().map(|c| c.to_ascii_uppercase());
        let Some(p) = pick.filter(|c| *c == 'A' || *c == 'B') else { continue };
        n += 1;
        let new_won = (p == 'A') == new_is_a;
        wins += new_won as usize;
        rows.push_str(&format!("- {sid}: {}\n  - this build: {}\n  - milestone 12: {}\n", if new_won { "this build" } else { "milestone 12" }, new.replace('\n', " / "), old.replace('\n', " / ")));
    }
    let (pr, lo, hi) = sla_eval::outcome::wilson(wins, n);
    let line = format!("This build preferred in {wins} of {n} pairs ({:.1}%, 95% interval {:.1} to {:.1}); target 70% or more.", pr * 100.0, lo * 100.0, hi * 100.0);
    println!("{line}");
    md.push_str(&line);
    md.push_str("\n\n");
    md.push_str(&rows);
    let _ = std::fs::write(root.join("eval/curriculum/pairwise_m12.md"), md);
}

/// `sla_eval consolidate --vault DIR`: the offline consolidation pass (rule
/// search over the whole vault, hygiene, steps shared by procedures).
fn consolidate_command(args: &[String]) {
    let Some(dir) = args.iter().position(|a| a == "--vault").and_then(|i| args.get(i + 1)) else {
        eprintln!("usage: sla_eval consolidate --vault DIR");
        std::process::exit(2);
    };
    let brain = sla_agent::Brain::shared(dir).expect("open vault");
    let mut b = sla_agent::brain::lock(&brain);
    let r = sla_agent::consolidate::run(&mut b, sla_core::Timestamp::now());
    println!("rules proposed {}, changed {}; duplicates linked {}; {:.0} ms", r.rules_proposed, r.rules_changed, r.hygiene.linked, r.ms);
    for (a, o, tasks) in &r.shared_steps {
        println!("  shared step: {a} {o} in {}", tasks.join(", "));
    }
}

/// `sla_eval hygiene --vault DIR`: the hygiene pass on a vault (run it on a copy).
fn hygiene_command(args: &[String]) {
    let Some(dir) = args.iter().position(|a| a == "--vault").and_then(|i| args.get(i + 1)) else {
        eprintln!("usage: sla_eval hygiene --vault DIR");
        std::process::exit(2);
    };
    let brain = sla_agent::Brain::shared(dir).expect("open vault");
    let mut b = sla_agent::brain::lock(&brain);
    let r = sla_agent::hygiene::run(&mut b, sla_core::Timestamp::now());
    println!("entities {}, predicates {} ({} used once), entities with no category or task {}", r.entities, r.predicates, r.predicates_used_once, r.unlinked_entities);
    println!("duplicate pairs before {}, linked {}, after {}", r.duplicates_before.len(), r.linked, r.duplicates_after);
    for (a, o) in &r.duplicates_before {
        println!("  same({a}, {o})");
    }
}

/// `sla_eval patterns [--model haiku] [--teach 300]`
fn patterns_command(args: &[String]) {
    let model = args.iter().position(|a| a == "--model").and_then(|i| args.get(i + 1)).cloned().unwrap_or_else(|| "haiku".into());
    let teach = args.iter().position(|a| a == "--teach").and_then(|i| args.get(i + 1)).and_then(|v| v.parse().ok()).unwrap_or(300);
    let root = root();
    let r = sla_eval::patterns::run(&root, &model, teach, 8);
    let failed: u64 = sla_parser::claude::global_usage().values().map(|u| u.errors).sum();
    let calls: u64 = sla_parser::claude::global_usage().values().map(|u| u.calls).sum();
    if calls > 0 && failed * 10 > calls {
        eprintln!("WARNING: {failed} of {calls} model calls failed; nothing recorded.");
        std::process::exit(1);
    }
    let md = sla_eval::patterns::markdown(&r);
    for l in md.lines().skip(2).take(5).filter(|l| !l.is_empty()) {
        println!("{l}");
    }
    let ts = sla_core::Timestamp::now().to_string().replace(':', "-");
    let dir = root.join("eval").join("patterns");
    let _ = std::fs::create_dir_all(&dir);
    let file = dir.join(format!("{ts}.md"));
    let _ = std::fs::write(&file, md);
    let _ = sla_eval::archive::append(&sla_eval::archive::dir(&root), "patterns", &r);
    println!("model calls: {calls}; wrote {}", file.display());
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
    // Every real model call (prompt and answer) is kept as training data.
    sla_parser::claude::set_call_log(root().join("data").join("training").join("calls"));
    let all: Vec<String> = std::env::args().skip(1).collect();
    match all.first().map(String::as_str) {
        Some("outcomes") => return outcomes(&all[1..]),
        Some("bench") => return sla_eval::ops::bench_command(&root(), &all[1..]),
        Some("stress") => return sla_eval::ops::stress_command(&root(), &all[1..]),
        Some("gold") => return gold_command(&all[1..]),
        Some("wording") => return wording_command(&all[1..]),
        Some("patterns") => return patterns_command(&all[1..]),
        Some("hygiene") => return hygiene_command(&all[1..]),
        Some("curriculum") => return curriculum_command(&all[1..]),
        Some("calibrate") => return calibrate_command(),
        Some("rate") => return rate_command(&all[1..]),
        Some("pairwise") => return pairwise_command(&all[1..]),
        Some("consolidate") => return consolidate_command(&all[1..]),
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
