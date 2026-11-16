//! CLI chat with the agent.
//!
//! ```text
//! sla [--vault DIR] [--speaker NAME] [--script FILE]
//! ```
//! Commands: `:facts`, `:rules`, `:log`, `:reload`, `:quit`.
//! With `--script`, each non-empty line of FILE is sent as a message and the
//! transcript is printed (used by teacher and judge sub-agents).

use sla_agent::Agent;
use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    let mut vault = "vault".to_string();
    let mut speaker = "maksim".to_string();
    let mut script: Option<String> = None;
    let mut source: Option<String> = None;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--vault" => vault = args.next().expect("--vault DIR"),
            "--speaker" => speaker = args.next().expect("--speaker NAME"),
            "--script" => script = Some(args.next().expect("--script FILE")),
            "--source" => source = Some(args.next().expect("--source LABEL")),
            "-h" | "--help" => {
                println!("sla [--vault DIR] [--speaker NAME] [--source LABEL] [--script FILE]");
                return Ok(());
            }
            other => {
                eprintln!("unknown argument {other}");
                std::process::exit(2);
            }
        }
    }
    let mut agent = Agent::open(&vault, &speaker)?;
    if let Some(s) = source {
        agent.source = s;
    }
    for (file, line, err) in agent.kb.vault.errors() {
        eprintln!("warning: {file}: cannot read `{line}`: {err}");
    }

    if let Some(path) = script {
        let text = std::fs::read_to_string(path)?;
        for line in text.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')) {
            println!("> {line}");
            println!("{}", reply_or_command(&mut agent, line));
        }
        return Ok(());
    }

    println!("Symbolic learning agent. Vault: {vault}. Type :quit to leave.");
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut line = String::new();
        if stdin.lock().read_line(&mut line)? == 0 {
            break;
        }
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line == ":quit" || line == ":q" {
            break;
        }
        println!("{}", reply_or_command(&mut agent, line));
    }
    agent.save()
}

fn reply_or_command(agent: &mut Agent, line: &str) -> String {
    match line {
        ":facts" => agent.dump_facts().join("\n"),
        ":rules" => agent.dump_rules().join("\n"),
        ":reload" => match agent.kb.reload() {
            Ok(()) => "Reloaded the vault.".into(),
            Err(e) => format!("Reload failed: {e}"),
        },
        ":log" => match agent.kb.vault.read_log() {
            Ok(entries) => entries.iter().rev().take(20).map(|(t, a, s, r)| format!("{t} {a} {s} ({r})")).collect::<Vec<_>>().join("\n"),
            Err(e) => format!("Cannot read log: {e}"),
        },
        _ if line.starts_with(":demo ") => demo(agent, &line[6..]),
        _ => agent.respond(line).text,
    }
}

/// `:demo fill(maksim, cup1) | before: is_a(cup1, cup); property(cup1, quality, empty) | after: is_a(cup1, cup); property(cup1, quality, full)`
fn demo(agent: &mut Agent, spec: &str) -> String {
    let parts: Vec<&str> = spec.split('|').map(str::trim).collect();
    let atoms = |s: &str, key: &str| -> Result<Vec<sla_core::GroundAtom>, String> {
        let body = s.strip_prefix(key).ok_or(format!("expected `{key}`"))?;
        body.split(';').map(str::trim).filter(|x| !x.is_empty()).map(|x| sla_core::parse_ground(x).map_err(|e| e.to_string())).collect()
    };
    let parsed = (|| -> Result<sla_learner::corrections::Demonstration, String> {
        if parts.len() != 3 {
            return Err("use `:demo ACTION | before: A; B | after: C; D`".into());
        }
        Ok(sla_learner::corrections::Demonstration {
            action: sla_core::parse_ground(parts[0]).map_err(|e| e.to_string())?,
            before: atoms(parts[1], "before:")?,
            after: atoms(parts[2], "after:")?,
        })
    })();
    match parsed {
        Ok(d) => {
            let rules = agent.demonstrate(&d);
            if rules.is_empty() {
                "I didn't see any change I could learn from.".into()
            } else {
                format!("I learned what {} does ({} rule(s): {}).", d.action.pred, rules.len(), rules.join(", "))
            }
        }
        Err(e) => format!("Bad demonstration: {e}"),
    }
}
