//! `sla3`: Charlotte 3.0 runs.
//!
//!   sla3 calibrate                          set θ on dev (no model calls)
//!   sla3 run --set dev|train|both [--arm function|rules|none] [--live] [--no-fallback]
//!            [--no-teach-variants] [--no-seeds] [--name NAME]
//!
//! Runs write data/training/runs3/<name>/{encounters.jsonl, vault.json, summary.json} and the
//! lab event log data/training/lab3/<name>/events.jsonl.

use agent3::Arm;
use eval3::{calibrate, curriculum, load, root, summary, RunOpts, Runner};
use parse3::Models;
use serde_json::json;

fn arg(args: &[String], k: &str) -> Option<String> {
    args.iter().position(|a| a == k).and_then(|i| args.get(i + 1).cloned())
}

fn theta() -> f64 {
    let p = root().join("data/training/parser3/models/theta.json");
    std::fs::read_to_string(p).ok().and_then(|t| serde_json::from_str::<serde_json::Value>(&t).ok()).and_then(|v| v["theta"].as_f64()).unwrap_or(f64::INFINITY)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("calibrate") => {
            let m = Models::load(&Models::default_path()).unwrap();
            let (t, cover, n) = calibrate(&m, &load("data/curriculum3/dev"));
            let v = json!({"theta": if t.is_finite() { t } else { 1e9 }, "coverage_at_95": cover, "dev_step_lines": n});
            std::fs::write(root().join("data/training/parser3/models/theta.json"), v.to_string()).unwrap();
            println!("{v}");
        }
        Some("run") => {
            let set = arg(&args, "--set").unwrap_or_else(|| "dev".into());
            let arm = match arg(&args, "--arm").as_deref() {
                Some("rules") => Arm::Rules,
                Some("none") => Arm::None,
                _ => Arm::Function,
            };
            let opts = RunOpts {
                name: arg(&args, "--name").unwrap_or_else(|| format!("c1_{set}")),
                arm,
                live: args.iter().any(|a| a == "--live"),
                use_fallback: !args.iter().any(|a| a == "--no-fallback"),
                teach_variants: !args.iter().any(|a| a == "--no-teach-variants"),
                seeds: !args.iter().any(|a| a == "--no-seeds"),
                vault: arg(&args, "--vault").map(Into::into),
                c2: args.iter().any(|a| a == "--c2"),
            };
            let mut scs = Vec::new();
            if set == "train" || set == "both" {
                scs.extend(load("data/curriculum3/train"));
            }
            if set == "dev" || set == "both" {
                scs.extend(load("data/curriculum3/dev"));
            }
            let mut r = Runner::new(&opts, theta());
            let calls0 = teach3::Teacher::live_calls();
            curriculum(&mut r, &scs, opts.teach_variants);
            let dir = root().join("data/training/runs3").join(&opts.name);
            r.save(&dir);
            let st = &r.agent.stats;
            let s = json!({"name": opts.name, "set": set, "arm": format!("{:?}", opts.arm), "theta": theta(),
                "ingest": {"lines": st.lines, "learned": st.learned, "fallback": st.fallback, "fallback_failed": st.fallback_failed, "rejected": st.rejected},
                "live_calls": teach3::Teacher::live_calls() - calls0, "by_kind": summary(&r.encounters)});
            std::fs::write(dir.join("summary.json"), serde_json::to_string_pretty(&s).unwrap()).unwrap();
            println!("{}", serde_json::to_string_pretty(&s).unwrap());
        }
        Some("b2") => {
            // baseline B2 on H3 (a sealed reading counts as part of the reading it is run with)
            let t = teach3::Teacher::open(args.iter().any(|a| a == "--live"));
            let calls0 = teach3::Teacher::live_calls();
            let es = eval3::b2_h3(&t);
            let name = arg(&args, "--name").unwrap_or_else(|| "b2_h3".into());
            let dir = root().join("data/training/runs3").join(&name);
            let _ = std::fs::create_dir_all(&dir);
            let body: String = es.iter().map(|e| serde_json::to_string(e).unwrap() + "\n").collect();
            std::fs::write(dir.join("encounters.jsonl"), body).unwrap();
            let s = json!({"name": name, "live_calls": teach3::Teacher::live_calls() - calls0, "by_kind": summary(&es)});
            std::fs::write(dir.join("summary.json"), serde_json::to_string_pretty(&s).unwrap()).unwrap();
            println!("{}", serde_json::to_string_pretty(&s).unwrap());
        }
        Some("split") => {
            // algorithmic tier: parser trained without the split's test tasks; teach the rest,
            // then teach and measure the test tasks (as tier T)
            let k = arg(&args, "--split").expect("--split K");
            let sp = root().join("data/training/parser3/splits");
            let info: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(sp.join(format!("split{k}.json"))).unwrap()).unwrap();
            let test: std::collections::BTreeSet<String> = info["test"].as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_string()).collect();
            let name = format!("split{k}");
            let opts = RunOpts { name: name.clone(), arm: Arm::Function, live: args.iter().any(|a| a == "--live"), use_fallback: true, teach_variants: true, seeds: true, vault: None, c2: false };
            let mut r = Runner::new(&opts, theta());
            r.agent.models = Models::load(&sp.join(format!("split{k}_models.json"))).unwrap();
            let mut all = load("data/curriculum3/train");
            all.extend(load("data/curriculum3/dev"));
            let (te, tr): (Vec<_>, Vec<_>) = all.into_iter().partition(|s| test.contains(&s.task));
            let calls0 = teach3::Teacher::live_calls();
            curriculum(&mut r, &tr, true);
            r.encounters.clear();
            let stored = eval3::tier_taught(&mut r, &te, "R", true);
            let dir = root().join("data/training/runs3").join(&name);
            r.save(&dir);
            let s = json!({"name": name, "test_tasks": te.len(), "stored_pass": stored.iter().filter(|c| c.strict_pass).count(),
                "live_calls": teach3::Teacher::live_calls() - calls0, "by_kind": summary(&r.encounters)});
            std::fs::write(dir.join("summary.json"), serde_json::to_string_pretty(&s).unwrap()).unwrap();
            println!("{}", serde_json::to_string_pretty(&s).unwrap());
        }
        Some("rank") => {
            let v = kb3::Vault::load(std::path::Path::new(&arg(&args, "--vault").expect("--vault"))).unwrap();
            let q = arg(&args, "--q").expect("--q");
            for (t, sc) in agent3::identity::rank(&v, &q).into_iter().take(5) {
                println!("{sc:8.3} {t}");
            }
            let words = agent3::identity::words(&q);
            println!("request words: {words:?}");
            for t in ["make_couscous", "bake_banana_bread"] {
                let a: Vec<String> = v.live().filter(|f| f.pred == "task_alias" && f.arg(0) == t).map(|f| f.arg(1).to_string()).collect();
                println!("{t}: {a:?}");
            }
        }
        Some("rescore") => {
            // re-check saved answers with the current check (zero calls)
            let name = arg(&args, "--name").expect("--name");
            let dir = root().join("data/training/runs3").join(&name);
            let mut all = Vec::new();
            for d in ["data/curriculum3/train", "data/curriculum3/dev", "data/heldout3/S", "data/heldout3/T", "data/heldout3/G", "data/curriculum3/heldout_domain"] {
                all.extend(load(d));
            }
            let by: std::collections::BTreeMap<String, kb3::Scenario> = all.into_iter().map(|s| (s.id.clone(), s)).collect();
            let text = std::fs::read_to_string(dir.join("encounters.jsonl")).unwrap();
            let mut es: Vec<eval3::Encounter> = text.lines().map(|l| serde_json::from_str(l).unwrap()).collect();
            for e in es.iter_mut() {
                let Some(sc) = by.get(&e.scenario) else { continue };
                let mut p = kb3::lines::parse(&e.answer).procedure;
                p.task = sc.task.clone();
                let g = kb3::gold(sc, e.variant);
                let lacks = e.variant.map(|k| kb3::lacked(&sc.variants[k])).unwrap_or_default();
                let r = kb3::check(&p, &g, &kb3::Aliases::from_facts(&sc.aliases), &lacks);
                e.strict_pass = e.identity_ok && r.strict_pass;
                e.recall = r.recall();
                e.core_recall = r.core_recall();
                e.precision = r.precision();
                e.wrong = r.wrong.clone();
            }
            let body: String = es.iter().map(|e| serde_json::to_string(e).unwrap() + "\n").collect();
            std::fs::write(dir.join("encounters.jsonl"), body).unwrap();
            let mut s: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(dir.join("summary.json")).unwrap()).unwrap();
            s["by_kind"] = summary(&es);
            s["rescored"] = json!(true);
            std::fs::write(dir.join("summary.json"), serde_json::to_string_pretty(&s).unwrap()).unwrap();
            println!("{}", serde_json::to_string_pretty(&s["by_kind"]).unwrap());
        }
        Some("lab") => {
            let port = arg(&args, "--port").and_then(|p| p.parse().ok()).unwrap_or(8733);
            eval3::lab::serve(port).expect("lab server");
        }
        Some("h3") => {
            // a sealed reading: counts only on screen and in the lab; transcripts stay on disk
            let reading = arg(&args, "--reading").expect("--reading N");
            let arm = match arg(&args, "--arm").as_deref() {
                Some("rules") => Arm::Rules,
                Some("none") => Arm::None,
                _ => Arm::Function,
            };
            let name = arg(&args, "--name").unwrap_or_else(|| format!("h3_r{reading}"));
            let opts = RunOpts { name: name.clone(), arm, live: args.iter().any(|a| a == "--live"), use_fallback: !args.iter().any(|a| a == "--no-fallback"),
                teach_variants: !args.iter().any(|a| a == "--no-teach-variants"), seeds: true, vault: arg(&args, "--vault").map(Into::into), c2: args.iter().any(|a| a == "--c2") };
            let mut r = Runner::new(&opts, theta());
            r.hidden = true;
            let calls0 = teach3::Teacher::live_calls();
            eval3::tier_s(&mut r, &load("data/heldout3/S"));
            let st_t = eval3::tier_taught(&mut r, &load("data/heldout3/T"), "T", opts.teach_variants);
            let mut g = load("data/heldout3/G");
            g.extend(load("data/curriculum3/heldout_domain").into_iter().filter(|s| !s.teach.is_empty()));
            let st_g = eval3::tier_taught(&mut r, &g, "G", opts.teach_variants);
            let dir = root().join("data/training/runs3").join(&name);
            r.save(&dir);
            let stored = |v: &[kb3::CheckResult]| json!({"n": v.len(), "pass": v.iter().filter(|c| c.strict_pass).count(),
                "mean_core_recall": eval3::stats::round3(v.iter().map(|c| c.core_recall()).sum::<f64>() / v.len().max(1) as f64)});
            let st = &r.agent.stats;
            let s = json!({"name": name, "reading": reading, "arm": format!("{:?}", arm), "theta": theta(),
                "ingest": {"lines": st.lines, "learned": st.learned, "fallback": st.fallback, "fallback_failed": st.fallback_failed},
                "stored_T": stored(&st_t), "stored_G": stored(&st_g),
                "live_calls": teach3::Teacher::live_calls() - calls0, "by_kind": summary(&r.encounters)});
            std::fs::write(dir.join("summary.json"), serde_json::to_string_pretty(&s).unwrap()).unwrap();
            println!("{}", serde_json::to_string_pretty(&s).unwrap());
        }
        _ => eprintln!("usage: sla3 calibrate | run --set dev|train|both [--arm function|rules|none] [--live] [--no-fallback]"),
    }
}
