//! Replay saved voice/text chats through a fresh engine and measure how well
//! Karin understands them.
//!
//! Usage (from `software/`):
//!   cargo run -p karin-core --example replay -- <chats_dir> <out_prefix> [baseline.jsonl]
//!
//! Chats are replayed in chronological order (file names are timestamps) on a
//! single engine, calling `new_chat` between files, so long-term memory carries
//! over exactly as it would in real use. Writes `<out_prefix>.jsonl` (one line
//! per turn) and `<out_prefix>.md` (summary + every turn, with the baseline
//! reply next to the new one when a baseline is given).

use karin_core::engine::Engine;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Clone)]
struct Row {
    chat: String,
    turn: usize,
    input: String,
    reply: String,
    impasse: bool,
    fired: Vec<String>,
}

fn read_inputs(path: &Path) -> Vec<String> {
    let raw = fs::read_to_string(path).unwrap_or_default();
    raw.lines()
        .filter_map(|line| line.strip_prefix("**User:**"))
        .map(|rest| rest.trim().to_string())
        .collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: replay <chats_dir> <out_prefix> [baseline.jsonl]");
        std::process::exit(2);
    }
    let chats_dir = PathBuf::from(&args[1]);
    let out_prefix = PathBuf::from(&args[2]);
    let baseline: HashMap<(String, usize), Row> = args
        .get(3)
        .and_then(|p| fs::read_to_string(p).ok())
        .map(|raw| {
            raw.lines()
                .filter_map(|l| serde_json::from_str::<Row>(l).ok())
                .map(|r| ((r.chat.clone(), r.turn), r))
                .collect()
        })
        .unwrap_or_default();

    let vault = karin_core::vault::default_vault_dir();
    let mut engine = Engine::from_vault_dir(&vault).expect("load vault");

    let mut files: Vec<PathBuf> = fs::read_dir(&chats_dir)
        .expect("read chats dir")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("md"))
        .collect();
    files.sort();

    let mut rows = Vec::new();
    for file in &files {
        let chat = file.file_stem().unwrap().to_string_lossy().to_string();
        engine.new_chat();
        for (i, input) in read_inputs(file).into_iter().enumerate() {
            let trace = engine.turn(&input);
            rows.push(Row {
                chat: chat.clone(),
                turn: i + 1,
                input,
                reply: trace.reply,
                impasse: trace.impasse,
                fired: trace.fired_pattern_ids,
            });
        }
    }

    let total = rows.len();
    let impasses = rows.iter().filter(|r| r.impasse).count();
    let mut md = String::new();
    md.push_str("# Chat replay\n\n");
    md.push_str(&format!(
        "- Turns: {total}\n- Impasses: {impasses} ({:.1}%)\n- Answered: {} ({:.1}%)\n",
        pct(impasses, total),
        total - impasses,
        pct(total - impasses, total)
    ));

    if !baseline.is_empty() {
        let base_imp = rows
            .iter()
            .filter(|r| baseline.get(&(r.chat.clone(), r.turn)).is_some_and(|b| b.impasse))
            .count();
        let fixed = rows
            .iter()
            .filter(|r| !r.impasse && baseline.get(&(r.chat.clone(), r.turn)).is_some_and(|b| b.impasse))
            .count();
        let broke = rows
            .iter()
            .filter(|r| r.impasse && baseline.get(&(r.chat.clone(), r.turn)).is_some_and(|b| !b.impasse))
            .count();
        let changed = rows
            .iter()
            .filter(|r| {
                !r.impasse
                    && baseline
                        .get(&(r.chat.clone(), r.turn))
                        .is_some_and(|b| !b.impasse && b.reply != r.reply)
            })
            .count();
        md.push_str(&format!(
            "- Baseline impasses: {base_imp} ({:.1}%)\n- Impasse → answered: {fixed}\n- Answered → impasse: {broke}\n- Answered both, reply changed: {changed}\n",
            pct(base_imp, total)
        ));
    }

    let mut current_chat = String::new();
    for r in &rows {
        if r.chat != current_chat {
            current_chat = r.chat.clone();
            md.push_str(&format!("\n## {current_chat}\n"));
        }
        let mark = if r.impasse { "✗" } else { "✓" };
        md.push_str(&format!("\n**{}. {mark} User:** {}\n", r.turn, r.input));
        if let Some(b) = baseline.get(&(r.chat.clone(), r.turn)) {
            let bmark = if b.impasse { "✗" } else { "✓" };
            if b.reply != r.reply {
                md.push_str(&format!("- before {bmark}: {}\n", b.reply));
            }
        }
        md.push_str(&format!("- now {mark}: {}\n", r.reply));
    }

    if let Some(parent) = out_prefix.parent() {
        fs::create_dir_all(parent).ok();
    }
    let jsonl: String = rows
        .iter()
        .map(|r| serde_json::to_string(r).unwrap() + "\n")
        .collect();
    fs::write(out_prefix.with_extension("jsonl"), jsonl).expect("write jsonl");
    fs::write(out_prefix.with_extension("md"), md).expect("write md");
    println!(
        "turns={total} impasses={impasses} ({:.1}%) -> {}",
        pct(impasses, total),
        out_prefix.with_extension("md").display()
    );
}

fn pct(n: usize, d: usize) -> f64 {
    if d == 0 { 0.0 } else { n as f64 * 100.0 / d as f64 }
}
