//! `sla4`: Design 4.0 spike runs (zero model calls).
//!
//!   sla4 spike [--name N] [--off B7,B2] [--no-learn] [--no-naming] [--prefix 0.5]
//!              [--order SEED] [--k 3] [--fixture DIR] [--proofs FILE]
//!   sla4 scale --copies 10           size stress on the fixture copied with renamed symbols
//!
//! Results: data/training/spike4/<name>.json (summary, scored questions, learned rules).

use eval4::{deliver, root, score, summary, Fixture, Opts};
use serde_json::json;
use std::time::Instant;

fn arg(args: &[String], k: &str) -> Option<String> {
    args.iter().position(|a| a == k).and_then(|i| args.get(i + 1).cloned())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dir = arg(&args, "--fixture").map(std::path::PathBuf::from).unwrap_or_else(|| root().join("data/spike4"));
    let fx = Fixture::load(&dir).unwrap_or_else(|e| panic!("fixture: {e}"));
    let out_dir = root().join("data/training/spike4");
    std::fs::create_dir_all(&out_dir).unwrap();
    match args.get(1).map(String::as_str) {
        Some("spike") => {
            let o = Opts {
                off: arg(&args, "--off").map(|s| s.split(',').map(String::from).collect()).unwrap_or_default(),
                learn: !args.iter().any(|a| a == "--no-learn"),
                naming: !args.iter().any(|a| a == "--no-naming"),
                prefix: arg(&args, "--prefix").and_then(|v| v.parse().ok()).unwrap_or(1.0),
                order_seed: arg(&args, "--order").and_then(|v| v.parse().ok()),
                min_support: arg(&args, "--k").and_then(|v| v.parse().ok()).unwrap_or(3),
                bulk: false,
                session: None,
            };
            let name = arg(&args, "--name").unwrap_or_else(|| "spike".into());
            let t0 = Instant::now();
            let mut d = deliver(&fx, &o);
            let teach_ms = t0.elapsed().as_millis();
            let t1 = Instant::now();
            let scored: Vec<_> = fx.questions.iter().map(|q| score(&mut d.agent, q)).collect();
            let answer_ms = t1.elapsed().as_millis();
            let derived = d.agent.model().len();
            let rules: Vec<serde_json::Value> = d.agent.learned.iter().map(|r| json!({"id": r.id, "rule": r.show(), "status": r.status, "layer": r.layer,
                "support": r.support, "counter": r.counter, "predictions": r.predictions.len(), "confirmed": r.confirmed_by.is_some(),
                "built_from": r.built_from, "specialised_from": r.specialised_from, "conf": r.conf})).collect();
            let s = json!({
                "name": name, "off": o.off, "learn": o.learn, "naming": o.naming, "prefix": o.prefix, "order_seed": o.order_seed, "k": o.min_support,
                "delivered": d.delivered, "withheld": d.withheld, "delivered_leaks": d.leaks, "valid_run": d.leaks.is_empty(),
                "stats": d.agent.stats, "unexpected_questions": d.unexpected.len(), "questions_asked": d.agent.questions.len(),
                "learned_rules": rules, "learn_lines": d.learn_lines, "model_facts": derived, "teach_ms": teach_ms, "answer_ms": answer_ms,
                "summary": summary(&scored),
            });
            let full = json!({"run": s, "scored": scored, "unexpected": d.unexpected, "raw": d.agent.raw});
            std::fs::write(out_dir.join(format!("{name}.json")), serde_json::to_string_pretty(&full).unwrap()).unwrap();
            println!("{}", serde_json::to_string_pretty(&s["summary"]).unwrap());
            println!("delivered {} withheld {} leaks {} rules {} asked {} unexpected {}", d.delivered, d.withheld, d.leaks.len(), d.agent.learned.len(), d.agent.questions.len(), d.unexpected.len());
            if let Some(p) = arg(&args, "--proofs") {
                let mut lines = String::new();
                for q in &fx.questions {
                    let a = d.agent.respond(&q.query);
                    let given: Vec<String> = match agent4::msg::parse(&q.query) {
                        Ok(Some(agent4::msg::Msg::Query { given, .. })) | Ok(Some(agent4::msg::Msg::Request { given, .. })) => {
                            given.iter().map(|(g, n)| reason4::encode(&d.agent.canon_atom(g), *n).to_string()).collect()
                        }
                        _ => vec![],
                    };
                    for (c, rec) in a.claims {
                        lines.push_str(&json!({"q": q.id, "claim": c, "proof": rec, "given": given}).to_string());
                        lines.push('\n');
                    }
                }
                std::fs::write(&p, lines).unwrap();
                let facts: Vec<String> = d.agent.engine_facts(&[]).iter().map(|f| f.atom.to_string()).collect();
                std::fs::write(format!("{p}.facts"), facts.join("\n")).unwrap();
            }
        }
        Some("live") => {
            // teach the fixture live, then write its raw session and a build (milestone 33)
            let name = arg(&args, "--name").unwrap_or_else(|| "live".into());
            let kbroot = arg(&args, "--kb").map(std::path::PathBuf::from).unwrap_or_else(|| root().join("data/kb4"));
            let d = deliver(&fx, &Opts { learn: !args.iter().any(|a| a == "--no-learn"), naming: true, prefix: 1.0, min_support: 3, session: Some(name.clone()), ..Default::default() });
            let agent = d.agent;
            let raw = agent4::kb::write_raw(&kbroot, &agent, "fixture").unwrap();
            // lab events on 3.0's stream (data/training/lab3/<run>/events.jsonl, served by sla3 lab)
            let mut log = eval3::lab::Log::open(&format!("kb4_{name}"));
            for r in agent4::kb::raw_records(&agent, "fixture") {
                log.event("teach4", json!({"msg": r["msg_id"], "speaker": r["speaker"], "channel": r["channel"], "pass": r["speaker"] == "teacher"}));
            }
            let id = agent4::kb::write_build(&kbroot.join("builds").join(&name), &agent, agent.raw.len()).unwrap();
            println!("{}", json!({"raw": raw, "build": id, "messages": agent.raw.len()}));
        }
        Some("replay") => {
            // rebuild from raw through the live entry point; answers from raw only
            let session = arg(&args, "--session").expect("--session");
            let kbroot = arg(&args, "--kb").map(std::path::PathBuf::from).unwrap_or_else(|| root().join("data/kb4"));
            let rows = agent4::kb::read_lines(&kbroot.join("raw/sessions").join(format!("{session}.jsonl")));
            let renames = arg(&args, "--migration").map(|m| agent4::kb::load_migration(std::path::Path::new(&m))).unwrap_or_default();
            let (agent, unanswered) = agent4::kb::replay(&rows, &session, &renames, &[], !args.iter().any(|a| a == "--no-learn"), true);
            let out = arg(&args, "--out").unwrap_or_else(|| format!("{session}_replay"));
            let id = agent4::kb::write_build(&kbroot.join("builds").join(&out), &agent, rows.len()).unwrap();
            let asked = agent.questions.len();
            let under = asked > 0 && unanswered.len() * 20 > asked;
            println!("{}", json!({"build": id, "out": out, "questions": asked, "unanswered": unanswered.len(), "under_answered": under}));
        }
        Some("drift") => {
            let every: usize = arg(&args, "--every").and_then(|v| v.parse().ok()).unwrap_or(7);
            let (dfx, n) = eval4::drift(&fx, every);
            let mut out = serde_json::Map::new();
            for (label, naming) in [("naming_on", true), ("naming_off", false)] {
                let mut d = deliver(&dfx, &Opts { learn: true, naming, prefix: 1.0, min_support: 3, ..Default::default() });
                let scored: Vec<_> = dfx.questions.iter().map(|q| score(&mut d.agent, q)).collect();
                out.insert(label.into(), json!({"summary": summary(&scored), "naming_questions": d.agent.stats.naming_questions, "aliases": d.agent.canon.len()}));
            }
            out.insert("rewritten_symbols".into(), json!(n));
            std::fs::write(out_dir.join(format!("drift_every{every}.json")), serde_json::to_string_pretty(&out).unwrap()).unwrap();
            println!("{}", serde_json::to_string_pretty(&out).unwrap());
        }
        Some("scale") => {
            let copies: usize = arg(&args, "--copies").and_then(|v| v.parse().ok()).unwrap_or(10);
            let r = eval4::scale::stress(&fx, copies);
            std::fs::write(out_dir.join(format!("scale_x{copies}.json")), serde_json::to_string_pretty(&r).unwrap()).unwrap();
            println!("{}", serde_json::to_string_pretty(&r).unwrap());
        }
        _ => eprintln!("usage: sla4 spike [...] | sla4 scale --copies N"),
    }
}
