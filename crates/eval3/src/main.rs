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
                teach_variants: !args.iter().any(|a| a == "--no-teach-variants"), seeds: true, vault: arg(&args, "--vault").map(Into::into) };
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
