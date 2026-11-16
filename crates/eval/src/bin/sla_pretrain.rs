//! Pretraining (cold start) with a Claude teacher and reviewer:
//!
//! ```text
//! cargo run --release -p sla_eval --bin sla_pretrain -- [--vault DIR] [--per-topic N] [--topic TEXT]... [--no-review] [--model M]
//! ```

use sla_eval::pretrain::{conversation_report, converse, pretrain, report, ConversationConfig, PretrainConfig, CHAT_TOPICS, CONVERSATION_TOPICS, DEFAULT_TOPICS};

fn main() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..");
    let root = root.canonicalize().unwrap_or(root);
    let mut cfg = PretrainConfig {
        vault: root.join("vault"),
        topics: Vec::new(),
        per_topic: 25,
        model: "haiku".into(),
        review: true,
    };
    let mut conversations = false;
    let mut natural = false;
    let mut chat = false;
    let mut rounds = 1;
    let mut parallel = 6;
    let mut turns = 14;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--vault" => cfg.vault = args.next().expect("--vault DIR").into(),
            "--per-topic" => cfg.per_topic = args.next().and_then(|v| v.parse().ok()).expect("--per-topic N"),
            "--topic" => cfg.topics.push(args.next().expect("--topic TEXT")),
            "--no-review" => cfg.review = false,
            "--conversations" => conversations = true,
            "--chat" => {
                conversations = true;
                chat = true;
            }
            "--rounds" => rounds = args.next().and_then(|v| v.parse().ok()).expect("--rounds N"),
            "--natural" => {
                conversations = true;
                natural = true;
            }
            "--parallel" => parallel = args.next().and_then(|v| v.parse().ok()).expect("--parallel N"),
            "--turns" => turns = args.next().and_then(|v| v.parse().ok()).expect("--turns N"),
            "--model" => cfg.model = args.next().expect("--model NAME"),
            "-h" | "--help" => {
                println!("sla_pretrain [--vault DIR] [--per-topic N] [--topic TEXT]... [--no-review] [--model M]
             [--conversations | --natural | --chat] [--parallel N] [--turns N] [--rounds N]");
                return;
            }
            other => {
                eprintln!("unknown argument {other}");
                std::process::exit(2);
            }
        }
    }
    if conversations {
        let base: Vec<String> = if !cfg.topics.is_empty() {
            cfg.topics.clone()
        } else if chat {
            CHAT_TOPICS.iter().map(|t| t.to_string()).collect()
        } else {
            CONVERSATION_TOPICS.iter().map(|t| t.to_string()).collect()
        };
        // --rounds N repeats the topic list; each round is a new conversation.
        let topics: Vec<String> = (0..rounds.max(1)).flat_map(|_| base.iter().cloned()).collect();
        let ccfg = ConversationConfig { vault: cfg.vault.clone(), topics, turns, parallel, model: cfg.model.clone(), review: cfg.review, natural, chat };
        println!("Teaching {} with {} conversations ({} turns each, {} in parallel).", ccfg.vault.display(), ccfg.topics.len(), turns, parallel);
        let results = converse(&ccfg, |m| println!("{m}"));
        let text = conversation_report(&results);
        let dir = root.join("eval").join("pretrain");
        let _ = std::fs::create_dir_all(&dir);
        let name = sla_core::Timestamp::now().to_string().replace(':', "-");
        let path = dir.join(format!("{name}_conversations.md"));
        let _ = std::fs::write(&path, &text);
        println!("{}", text.lines().nth(2).unwrap_or(""));
        println!("report: {}", path.display());
        return;
    }
    if cfg.topics.is_empty() {
        cfg.topics = DEFAULT_TOPICS.iter().map(|t| t.to_string()).collect();
    }
    println!("Pretraining {} with {} topic(s), {} sentences each.", cfg.vault.display(), cfg.topics.len(), cfg.per_topic);
    match pretrain(&cfg, |m| println!("{m}")) {
        Ok(results) => {
            let text = report(&results);
            let dir = root.join("eval").join("pretrain");
            let _ = std::fs::create_dir_all(&dir);
            let name = sla_core::Timestamp::now().to_string().replace(':', "-");
            let path = dir.join(format!("{name}.md"));
            let _ = std::fs::write(&path, &text);
            println!("{}", text.lines().nth(2).unwrap_or(""));
            println!("report: {}", path.display());
        }
        Err(e) => {
            eprintln!("pretraining failed: {e}");
            std::process::exit(1);
        }
    }
}
