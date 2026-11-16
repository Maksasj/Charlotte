//! One command for the full evaluation:
//!
//! ```text
//! cargo run --release -p sla_eval -- [--judges N] [--turns N] [--teacher-questions N] [--no-claude] [--model M] [--notes TEXT]
//! ```
//! Appends one row to `eval/history.csv` and writes `eval/last_report.md`.

use sla_eval::{run, EvalConfig, EvalReport};

fn main() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..");
    let root = root.canonicalize().unwrap_or(root);
    let mut cfg = EvalConfig::new(&root);
    let mut args = std::env::args().skip(1);
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
