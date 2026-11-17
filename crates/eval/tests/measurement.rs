//! Design 2.0, milestone 12: honest measurement. The outcome command, its
//! repeatability, oracle mode, and the trainer end to end on mock models.

use sla_eval::outcome::Label;
use sla_eval::regression::{run_outcomes, OutcomeConfig, ParserMode};
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

/// The v1 baseline with the new labels reproduces the report's hand count:
/// 0 helpful replies in 68 help-scenario turns, 30 not understood.
#[test]
fn v1_trainer_baseline() {
    let mut cfg = OutcomeConfig::new(root());
    cfg.v1 = true;
    cfg.gold = true;
    cfg.set = Some("trainer".into());
    cfg.write = false;
    let run = run_outcomes(&cfg).unwrap();
    assert_eq!(run.mix.n, 68);
    assert_eq!(run.mix.count(Label::Helpful), 0);
    assert_eq!(run.mix.count(Label::Partial), 1);
    assert_eq!(run.mix.count(Label::NotUnderstood), 30);
    assert_eq!(run.mix.count(Label::Unjudged), 0, "every trainer turn has a hand label");
}

/// The same build on the same scenarios gives the same rates.
#[test]
fn replay_is_repeatable() {
    let mut cfg = OutcomeConfig::new(root());
    cfg.parser = ParserMode::Pattern;
    cfg.set = Some("trainer".into());
    cfg.write = false;
    let a = run_outcomes(&cfg).unwrap();
    let b = run_outcomes(&cfg).unwrap();
    assert_eq!(a.mix.n, 68);
    for l in Label::ALL {
        assert_eq!(a.mix.count(l), b.mix.count(l), "{}", l.name());
    }
    let replies_a: Vec<&str> = a.turns.iter().map(|t| t.reply.as_str()).collect();
    let replies_b: Vec<&str> = b.turns.iter().map(|t| t.reply.as_str()).collect();
    assert_eq!(replies_a, replies_b);
}

/// The oracle parser feeds gold acts, so understanding is taken out of the
/// measurement.
#[test]
fn oracle_mode() {
    let dir = std::env::temp_dir().join(format!("sla_oracle_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let gold = dir.join("gold.jsonl");
    std::fs::write(
        &gold,
        "{\"text\": \"Hi! Can you help me plan dinner tonight?\", \"acts\": [\"greet:\"]}\n{\"text\": \"Anna likes tea.\", \"acts\": [\"tell: likes(anna, tea)\"]}\n",
    )
    .unwrap();
    let mut agent = sla_agent::Agent::open(dir.join("vault"), "maksim").unwrap();
    agent.set_parser(Box::new(sla_parser::oracle::OracleParser::load(&gold).unwrap()));
    assert_eq!(agent.parser_name(), "oracle");
    assert!(agent.respond("Anna likes tea.").text.contains("Anna likes tea"));
    let r = agent.respond("Something with no gold parse.").text;
    assert!(r.contains("understand") || r.contains("not sure") || r.contains("simply"), "{r}");
    drop(agent);
    let _ = std::fs::remove_dir_all(&dir);
}

/// The trainer runs end to end on mock models: teachers, translator and
/// reviewer are mocks; every file stays whole.
#[test]
fn trainer_on_mock_models() {
    let base = std::env::temp_dir().join(format!("sla_trainer_mock_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let r = sla_eval::ops::stress(&root(), &base, 3, 3).unwrap();
    assert!(r.turns > 10, "turns: {}", r.turns);
    assert!(r.conversations >= 1);
    assert_eq!(r.integrity.corrupted(), 0, "{:?}", r.integrity);
    assert!(r.integrity.events > r.turns, "every turn is an event");
    assert_eq!(r.integrity.traces, r.turns, "one trace line per turn");
    let _ = std::fs::remove_dir_all(&base);
}
