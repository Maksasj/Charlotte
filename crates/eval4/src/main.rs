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
            // the fixture keeps its own raw layer; it is never merged into data/kb4 (design 8.1)
            let kbroot = arg(&args, "--kb").map(std::path::PathBuf::from).unwrap_or_else(|| root().join("data/spike4/kb4"));
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
            let mut rows = agent4::kb::read_lines(&kbroot.join("raw/sessions").join(format!("{session}.jsonl")));
            // a curve snapshot: the raw prefix through the k-th quarter block's end
            if let Some(k) = arg(&args, "--upto-block").and_then(|v| v.parse::<usize>().ok()) {
                let mut seen = 0;
                if let Some(cut) = rows.iter().position(|r| {
                    if r["speaker"] == "agent" && r["channel"] == "block_end" {
                        seen += 1;
                    }
                    seen == k
                }) {
                    rows.truncate(cut + 1);
                }
            }
            let renames = arg(&args, "--migration").map(|m| agent4::kb::load_migration(std::path::Path::new(&m))).unwrap_or_default();
            let (agent, unanswered) = agent4::kb::replay(&rows, &session, &renames, &[], !args.iter().any(|a| a == "--no-learn"), true);
            let out = arg(&args, "--out").unwrap_or_else(|| format!("{session}_replay"));
            let id = agent4::kb::write_build(&kbroot.join("builds").join(&out), &agent, rows.len()).unwrap();
            let asked = agent.questions.len();
            if args.iter().any(|a| a == "--rejects") {
                let mut by: std::collections::BTreeMap<String, usize> = Default::default();
                for (_, r) in &agent.rejects {
                    let key: String = r.split(':').nth(1).unwrap_or(r).split_whitespace().filter(|w| !w.contains('_') || w.ends_with(',')).take(6).collect::<Vec<_>>().join(" ");
                    *by.entry(key).or_insert(0) += 1;
                }
                println!("{}", json!({"rejects": agent.rejects.len(), "by_reason": by, "examples": agent.rejects.iter().take(25).collect::<Vec<_>>()}));
            }
            let under = asked > 0 && unanswered.len() * 20 > asked;
            println!("{}", json!({"build": id, "out": out, "questions": asked, "unanswered": unanswered.len(), "under_answered": under}));
        }
        Some("teach") => {
            // stage teaching with Haiku (milestone 34): --stages 0,1 --units N --cap C --session S
            let stages: Vec<u32> = arg(&args, "--stages").unwrap_or_else(|| "0".into()).split(',').filter_map(|v| v.parse().ok()).collect();
            let max_units: usize = arg(&args, "--units").and_then(|v| v.parse().ok()).unwrap_or(usize::MAX);
            let cap: usize = arg(&args, "--cap").and_then(|v| v.parse().ok()).unwrap_or(100);
            let session = arg(&args, "--session").unwrap_or_else(|| "pilot".into());
            let size = arg(&args, "--size").unwrap_or_else(|| "S".into());
            let topics = agent4::kb::read_lines(&root().join("data/curriculum4").join(&size).join("topics.jsonl"));
            let mut haiku = teach3::Teacher::open_at(root().join("data/training/calls4"), 5000, true);
            haiku.run_cap = Some(cap);
            // start from an earlier session's knowledge, replayed from raw through the live path
            let mut agent = match arg(&args, "--from") {
                Some(from) => {
                    let rows = agent4::kb::read_lines(&root().join("data/kb4/raw/sessions").join(format!("{from}.jsonl")));
                    let (mut a, _) = agent4::kb::replay(&rows, &from, &Default::default(), &[], true, true);
                    a.session = session.clone();
                    a
                }
                None => agent4::Agent::new(&session),
            };
            let only: Option<Vec<String>> = arg(&args, "--only").map(|v| v.split(',').map(String::from).collect());
            let mut queue = eval4::teach::Queue::default();
            queue.deny = agent4::deny::Deny::load(&root().join("data/bench4/deny.txt"));
            queue.deny.add_hashed(&root().join("data/bench4/deny_sealed.sha256"));
            // the H3 task names are denied at name level (design 9.2)
            let calls0 = teach3::Teacher::live_calls_at(&root().join("data/training/calls4"));
            let mut log = Vec::new();
            let mut history: Vec<String> = Vec::new();
            'outer: for stage in &stages {
                for t in topics.iter().filter(|t| only.as_ref().map(|o| o.iter().any(|u| t["unit"] == u.as_str())).unwrap_or(true)).take(max_units) {
                    let sorts: Vec<(String, String)> = t["sorts"].as_object().map(|o| o.iter().map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string())).collect()).unwrap_or_default();
                    let u = eval4::teach::Unit { sorts, text: t["text"].as_str().unwrap_or("").into(), task: t["task"].as_str().map(String::from), stage: *stage, unit: t["unit"].as_str().unwrap().into(), domain: t["domain"].as_str().unwrap().into(),
                        symbols: t["symbols"].as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_string()).collect() };
                    match eval4::teach::teach_unit(&mut agent, &haiku, &mut queue, &u, &history) {
                        Ok(rec) => {
                            eprintln!("{rec}");
                            log.push(rec);
                        }
                        Err(e) => {
                            eprintln!("stopped: {e}");
                            log.push(json!({"stage": stage, "unit": u.unit, "error": e}));
                            if e.contains("limit") || e.contains("cap") || e.contains("budget") {
                                break 'outer;
                            }
                        }
                    }
                    history = agent.raw.iter().rev().filter(|r| r["who"] == "agent" && !matches!(r["channel"].as_str(), Some("batch_end" | "block_end" | "finish"))).take(2).map(|r| r["text"].as_str().unwrap_or("").to_string()).collect();
                }
                agent.end_batch(&mut queue);
            }
            agent.finish();
            let kbroot = root().join("data/kb4");
            agent4::kb::write_raw(&kbroot, &agent, "live").unwrap();
            let id = agent4::kb::write_build(&kbroot.join("builds").join(&session), &agent, agent.raw.len()).unwrap();
            let calls = teach3::Teacher::live_calls_at(&root().join("data/training/calls4")) - calls0;
            let out = json!({"session": session, "stages": stages, "live_calls": calls, "build": id, "calls": log, "stats": agent.stats,
                "symbols": agent.sorts.len(), "aliases": agent.canon.len(), "questions_dropped": queue.dropped});
            std::fs::create_dir_all(root().join("data/training/teach4")).unwrap();
            std::fs::write(root().join("data/training/teach4").join(format!("{session}.json")), serde_json::to_string_pretty(&out).unwrap()).unwrap();
            println!("{}", json!({"session": session, "live_calls": calls, "accepted": agent.stats.accepted, "rejected": agent.stats.rejected}));
        }
        Some("bench") => {
            // answer benchmark questions from a KB rebuilt from raw (zero calls):
            // --from s1,s2 --file data/bench4/dev.jsonl [--out NAME]
            let from = arg(&args, "--from").expect("--from");
            let mut rows = Vec::new();
            for s in from.split(',') {
                rows.extend(agent4::kb::read_lines(&root().join("data/kb4/raw/sessions").join(format!("{s}.jsonl"))));
            }
            let (mut agent, _) = agent4::kb::replay(&rows, "bench", &Default::default(), &[], true, true);
            let file = arg(&args, "--file").expect("--file");
            if let Some(out) = arg(&args, "--score") {
                // per-question answers and proof records for the bench4 scorer (tools/design4/score_bench.py)
                let mut lines = String::new();
                let mut proofs = String::new();
                for q in agent4::kb::read_lines(std::path::Path::new(&file)) {
                    let query = q["query"].as_str().unwrap_or("");
                    let t0 = std::time::Instant::now();
                    let a = agent.respond(query);
                    let ms = t0.elapsed().as_millis();
                    let given: Vec<String> = match agent4::msg::parse(query) {
                        Ok(Some(agent4::msg::Msg::Query { given, .. })) | Ok(Some(agent4::msg::Msg::Request { given, .. })) => {
                            given.iter().map(|(g, n)| reason4::encode(&agent.canon_atom(g), *n).to_string()).collect()
                        }
                        _ => vec![],
                    };
                    let claims: Vec<serde_json::Value> = a.claims.iter().map(|(c, rec)| json!({"claim": c, "kind": rec["kind"], "hops": rec["hops"]})).collect();
                    let golds: Vec<String> = q["gold"]["atoms"].as_array().map(|v| v.iter().filter_map(|x| x.as_str()).map(|x| agent.canonical_goal(x)).collect()).unwrap_or_default();
                    let excluded: Vec<String> = q["gold"]["excluded"].as_array().map(|v| v.iter().filter_map(|x| x.as_str()).map(|x| agent.canonical_goal(x)).collect()).unwrap_or_default();
                    lines.push_str(&json!({"id": q["id"], "claims": claims, "gaps": a.gaps, "asks": a.asks, "ms": ms,
                        "gold_canon": golds, "excluded_canon": excluded}).to_string());
                    lines.push('\n');
                    for (c, rec) in &a.claims {
                        proofs.push_str(&json!({"q": q["id"], "claim": c, "proof": rec, "given": given}).to_string());
                        proofs.push('\n');
                    }
                }
                std::fs::write(format!("{out}.jsonl"), lines).unwrap();
                std::fs::write(format!("{out}_proofs.jsonl"), proofs).unwrap();
                let facts: Vec<String> = agent.engine_facts(&[]).iter().map(|f| f.atom.to_string()).collect();
                std::fs::write(format!("{out}_proofs.jsonl.facts"), facts.join("\n")).unwrap();
                std::fs::write(format!("{out}_canon.json"), serde_json::to_string(&agent.canon).unwrap()).unwrap();
                println!("{}", json!({"from": from, "file": file, "out": out, "facts": agent.facts.len()}));
                return;
            }
            // the agent's surface name for a stand-in is swap (design 4.3); golds say stand_in
            let norm = |s: &str| s.split_whitespace().collect::<String>().trim_end_matches('.').replacen("swap(", "stand_in(", 1);
            let (mut known, mut total) = (0usize, 0usize);
            let mut rejected_pieces = 0usize;
            let mut with_rejects: Vec<serde_json::Value> = Vec::new();
            let mut hits = Vec::new();
            for q in agent4::kb::read_lines(std::path::Path::new(&file)) {
                let kind = q["gold"]["kind"].as_str().unwrap_or("");
                if kind == "gap" {
                    continue;
                }
                total += 1;
                // --teach-pieces: a copy of the agent is taught this question's declarations and
                // pieces through the conversation path first (the ceiling: perfect teaching)
                let mut local = None;
                if args.iter().any(|a| a == "--teach-pieces") {
                    struct Mute;
                    impl agent4::Teacher for Mute {
                        fn answer(&mut self, _q: &str) -> String { "unknown".into() }
                    }
                    let (mut a, _) = agent4::kb::replay(&rows, "bench", &Default::default(), &[], true, true);
                    let rj0 = a.stats.rejected;
                    for d in q["new"].as_array().into_iter().flatten().filter_map(|v| v.as_str()) {
                        if let Some((sym, sort)) = d.split_once(':') {
                            a.hear(&format!("new({}, {}).", sym.trim(), sort.trim()), "bench", &mut Mute);
                        }
                    }
                    let trace = arg(&args, "--trace").map(|t| q["id"] == t.as_str()).unwrap_or(false);
                    for p in q["pieces"].as_array().into_iter().flatten().filter_map(|v| v.as_str()) {
                        let r = a.hear(p, "bench", &mut Mute);
                        if trace {
                            eprintln!("T: {p}");
                            eprintln!("A: {}", r.join(" | "));
                        }
                    }
                    if trace {
                        let ans = a.respond(q["query"].as_str().unwrap_or(""));
                        eprintln!("Q: {}", q["query"]);
                        eprintln!("claims {:?}", ans.claims.iter().map(|c| &c.0).collect::<Vec<_>>());
                        eprintln!("gaps {:?} asks {:?}", ans.gaps, ans.asks);
                    }
                    rejected_pieces += a.stats.rejected - rj0;
                    if a.stats.rejected > rj0 {
                        with_rejects.push(json!({"id": q["id"], "rejects": a.rejects.iter().skip(agent.rejects.len()).take(3).collect::<Vec<_>>()}));
                    }
                    local = Some(a);
                }
                let ans = match local.as_mut() { Some(a) => a.respond(q["query"].as_str().unwrap_or("")), None => agent.respond(q["query"].as_str().unwrap_or("")) };
                let claims: std::collections::BTreeSet<String> = ans.claims.iter().map(|(c, _)| norm(c)).collect();
                // golds are read through the agent's own names (same_as canonicalisation), as the spike scorer does
                let canon_of = |g: &str| match local.as_ref() { Some(a) => a.canonical_goal(g), None => agent.canonical_goal(g) };
                let golds: Vec<String> = q["gold"]["atoms"].as_array().map(|v| v.iter().filter_map(|a| a.as_str()).map(|a| norm(&canon_of(a))).collect()).unwrap_or_default();
                if !golds.is_empty() && golds.iter().all(|g| claims.contains(g)) {
                    known += 1;
                    hits.push(q["id"].clone());
                }
            }
            println!("{}", json!({"from": from, "file": file, "facts": agent.facts.len(), "symbols": agent.sorts.len(), "answerable": total, "answered_correct": known, "ids": hits,
                "rejected_pieces": rejected_pieces, "questions_with_rejects": with_rejects}));
        }
        Some("curriculum") => {
            // quarter blocks of a size's plan (milestone 36): --plan S --session S_main --from stage0_S_v3 --blocks 1,2,3,4 --cap N
            let size = arg(&args, "--plan").unwrap_or_else(|| "S".into());
            let session = arg(&args, "--session").expect("--session");
            let blocks: Vec<usize> = arg(&args, "--blocks").unwrap_or_else(|| "1,2,3,4".into()).split(',').filter_map(|v| v.parse().ok()).collect();
            let cap: usize = arg(&args, "--cap").and_then(|v| v.parse().ok()).unwrap_or(200);
            let plan = agent4::kb::read_lines(&root().join("data/curriculum4").join(&size).join("plan.jsonl"));
            // --dry: cache only, no live calls (prompt checks)
            let mut haiku = teach3::Teacher::open_at(root().join("data/training/calls4"), 5000, !args.iter().any(|a| a == "--dry"));
            haiku.run_cap = Some(cap);
            let from = arg(&args, "--from").expect("--from (the stage 0 vocabulary session)");
            let rows = agent4::kb::read_lines(&root().join("data/kb4/raw/sessions").join(format!("{from}.jsonl")));
            let (mut agent, _) = agent4::kb::replay(&rows, &from, &Default::default(), &[], true, true);
            agent.session = session.clone();
            let mut queue = eval4::teach::Queue::default();
            queue.deny = agent4::deny::Deny::load(&root().join("data/bench4/deny.txt"));
            queue.deny.add_hashed(&root().join("data/bench4/deny_sealed.sha256"));
            let calls0 = teach3::Teacher::live_calls_at(&root().join("data/training/calls4"));
            let mut log = Vec::new();
            let mut history: Vec<String> = Vec::new();
            let mut stop = false;
            let unit_of = |t: &serde_json::Value, stage: u32| eval4::teach::Unit {
                sorts: vec![], text: t["text"].as_str().unwrap_or("").into(), task: t["task"].as_str().map(String::from), stage,
                unit: t["unit"].as_str().unwrap().into(), domain: t["domain"].as_str().unwrap().into(),
                symbols: t["symbols"].as_array().map(|a| a.iter().filter_map(|v| v.as_str().map(String::from)).collect()).unwrap_or_default() };
            'blocks: for b in &blocks {
                let mine: Vec<&serde_json::Value> = plan.iter().filter(|t| t["block"].as_u64() == Some(*b as u64)).collect();
                for stage in 1..=7u32 {
                    let kinds: &[&str] = match stage { 3 => &["group", "role"], 6 => &["archive", "procedure"], 7 => &["situation"], _ => &["group"] };
                    for t in mine.iter().filter(|t| kinds.contains(&t["kind"].as_str().unwrap_or(""))) {
                        if t["kind"] == "archive" {
                            // archive3 readings replayed through the conversation path at 0 calls (D4.2)
                            let task = t["task"].as_str().unwrap();
                            for r in agent4::kb::read_lines(&root().join("data/kb4/raw/sessions").join(format!("a3-{task}.jsonl"))) {
                                if let Some(text) = r["text"].as_str() {
                                    agent.hear(text, "archive3", &mut queue);
                                }
                            }
                            log.push(json!({"block": b, "stage": stage, "unit": t["unit"], "archive": true}));
                            continue;
                        }
                        let u = unit_of(t, stage);
                        match eval4::teach::teach_unit(&mut agent, &haiku, &mut queue, &u, &history) {
                            Ok(mut rec) => {
                                rec["block"] = json!(b);
                                eprintln!("{rec}");
                                log.push(rec);
                            }
                            Err(e) => {
                                eprintln!("stopped: {e}");
                                log.push(json!({"block": b, "stage": stage, "unit": u.unit, "error": e}));
                                if e.contains("limit") || e.contains("cap") || e.contains("budget") || e.contains("leak") {
                                    stop = true;
                                    break 'blocks;
                                }
                            }
                        }
                        history = agent.raw.iter().rev().filter(|r| r["who"] == "agent" && !matches!(r["channel"].as_str(), Some("batch_end" | "block_end" | "finish"))).take(2).map(|r| r["text"].as_str().unwrap_or("").to_string()).collect();
                    }
                    agent.end_batch(&mut queue);
                }
                agent.mark_block(*b);
            }
            agent.finish();
            let kbroot = root().join("data/kb4");
            agent4::kb::write_raw(&kbroot, &agent, "live").unwrap();
            let id = agent4::kb::write_build(&kbroot.join("builds").join(&session), &agent, agent.raw.len()).unwrap();
            let calls = teach3::Teacher::live_calls_at(&root().join("data/training/calls4")) - calls0;
            let out = json!({"session": session, "plan": size, "blocks": blocks, "stopped": stop, "live_calls": calls, "build": id, "calls": log,
                "stats": agent.stats, "symbols": agent.sorts.len(), "aliases": agent.canon.len(), "facts": agent.facts.len(),
                "learned": agent.learned.len(), "questions_dropped": queue.dropped});
            std::fs::create_dir_all(root().join("data/training/teach4")).unwrap();
            std::fs::write(root().join("data/training/teach4").join(format!("{session}.json")), serde_json::to_string_pretty(&out).unwrap()).unwrap();
            println!("{}", json!({"session": session, "live_calls": calls, "stopped": stop, "facts": agent.facts.len(), "accepted": agent.stats.accepted, "rejected": agent.stats.rejected}));
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
