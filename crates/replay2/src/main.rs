//! `sla_replay2 [--tiers S,T] [--limit N]`: baseline B3, the v2.0.0 agent on sealed H3 tiers.
//! Counts only on screen; transcripts in data/training/runs3/b3_replay/.

use kb3::{check, gold, lacked, Aliases, Procedure, Scenario};
use parse3::{tokenize, LineKind, Models};
use serde_json::json;
use std::collections::BTreeSet;
use std::sync::Arc;

fn converse(a: &mut sla_agent::Agent, first: &str, situation: &[String]) -> String {
    let mut user = first.to_string();
    let mut sit = situation.iter();
    let mut last = String::new();
    for _ in 0..4 {
        last = a.respond(&user).text;
        if !last.trim_end().ends_with('?') {
            break;
        }
        match sit.next() {
            Some(s) => user = s.clone(),
            None => break,
        }
    }
    let _ = a.respond("Thanks!");
    last
}

/// Read a 2.0 reply back into a procedure with the 3.0 learned parser (lossy: X6).
fn read_back(m: &Models, task: &str, reply: &str) -> Procedure {
    let mut v = kb3::Vault::default();
    let phrase = task.replace('_', " ");
    let mut known = BTreeSet::new();
    for l in reply.lines() {
        if let Some(seg) = l.to_lowercase().split_once("you need").map(|(_, s)| s.split('.').next().unwrap_or("").to_string()) {
            let line = format!("To {phrase} you need{seg}.");
            known = m.known_words(&line);
            for f in m.parse_teach(task, &line, &BTreeSet::new()).facts {
                v.add(f, kb3::Provenance::new(kb3::Source::Teach, "b3", 0, "learned", 1.0));
            }
        }
    }
    let mut n = 0;
    for l in reply.lines() {
        let t = l.trim();
        let Some((num, body)) = t.split_once(". ") else { continue };
        if num.parse::<usize>().is_err() {
            continue;
        }
        n += 1;
        let line = format!("Step {n} of {phrase}: {body}");
        let p = m.parse_teach(task, &line, &known);
        if p.kind == LineKind::Step || !p.facts.is_empty() {
            for f in p.facts {
                v.add(f, kb3::Provenance::new(kb3::Source::Teach, "b3", 0, "learned", 1.0));
            }
        }
    }
    let _ = tokenize;
    let mut p = v.procedure(task);
    // 2.0 lists steps in order; make that order explicit
    for w in p.steps.clone().windows(2) {
        p.before.insert((w[0].id.clone(), w[1].id.clone()));
    }
    p
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let arg = |k: &str| args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).cloned();
    let tiers = arg("--tiers").unwrap_or_else(|| "S,T".into());
    let limit: usize = arg("--limit").and_then(|v| v.parse().ok()).unwrap_or(usize::MAX);
    let root = kb3::repo_root();
    let models = Models::load(&Models::default_path()).unwrap();
    let ts = std::process::id();
    let vault = std::env::temp_dir().join(format!("sla_replay2_{ts}"));
    let brain = sla_agent::Brain::shared(&vault).expect("vault");
    sla_eval::curriculum::install_seeds(&brain, &root.join("data/curriculum/seed"));
    sla_agent::brain::lock(&brain).install(&root.join("data/scenarios/dinner")).ok();
    let cache = Some(root.join("eval").join("translate_cache.jsonl"));
    let user = sla_eval::curriculum::llm_user("haiku".into(), cache.clone());
    let teacher = sla_eval::curriculum::llm_teacher("haiku".into(), cache);
    let calls0 = sla_parser::claude::global_usage().values().map(|u| u.calls).sum::<u64>();
    if teach3::Teacher::live_calls() >= 4500 {
        eprintln!("budget: ledger at 4,500; B3 skipped");
        return;
    }
    let mut out: Vec<eval3::Encounter> = Vec::new();
    let mut n = 0;
    let record = |sc: &Scenario, kind: &str, variant: Option<usize>, request: &str, reply: &str, out: &mut Vec<eval3::Encounter>| {
        let p = read_back(&models, &sc.task, reply);
        let r = check(&p, &gold(sc, variant), &Aliases::from_facts(&sc.aliases), &variant.map(|k| lacked(&sc.variants[k])).unwrap_or_default());
        out.push(eval3::Encounter { scenario: sc.id.clone(), kind: kind.into(), variant, request: request.into(), identified: None, identity_ok: true,
            strict_pass: r.strict_pass, core_recall: r.core_recall(), recall: r.recall(), precision: r.precision(), wrong: r.wrong.clone(), gaps: 0, asked: false, answer: reply.into(), missing: r.missing.clone() });
    };
    for tier in tiers.split(',') {
        let scs = Scenario::load_dir(&root.join("data/heldout3").join(tier)).unwrap_or_default();
        for sc in scs.iter().take(limit) {
            n += 1;
            let mut t = teacher(&brain);
            for line in &sc.teach {
                let _ = t.respond(line);
            }
            if tier == "S" {
                if let Some(v) = sc.variants.first() {
                    let mut a = user(&brain, &format!("user{n}"));
                    let reply = converse(&mut a, &v.request, &[v.situation_text.clone()]);
                    record(sc, "S_first", Some(0), &v.request, &reply, &mut out);
                }
                continue;
            }
            if let Some(q) = sc.requests.get(1) {
                let mut a = user(&brain, &format!("user{n}"));
                let reply = converse(&mut a, q, &sc.situation);
                record(sc, &format!("{tier}_second"), None, q, &reply, &mut out);
            }
            for (k, v) in sc.variants.iter().enumerate() {
                let mut a = user(&brain, &format!("user{n}v{k}"));
                let reply = converse(&mut a, &v.request, &[v.situation_text.clone()]);
                record(sc, &format!("{tier}_variant"), Some(k), &v.request, &reply, &mut out);
            }
        }
    }
    let calls = sla_parser::claude::global_usage().values().map(|u| u.calls).sum::<u64>() - calls0;
    // count the 2.0 translator's live calls against the 3.0 budget ledger
    let ledger = root.join("data/training/calls3/ledger.jsonl");
    let mut text = std::fs::read_to_string(&ledger).unwrap_or_default();
    for _ in 0..calls {
        text.push_str(&json!({"unix": 0, "tag": "b3_replay_2_0_translator", "ok": true}).to_string());
        text.push('\n');
    }
    std::fs::write(&ledger, text).unwrap();
    let dir = root.join("data/training/runs3/b3_replay");
    let _ = std::fs::create_dir_all(&dir);
    let body: String = out.iter().map(|e| serde_json::to_string(e).unwrap() + "\n").collect();
    std::fs::write(dir.join("encounters.jsonl"), body).unwrap();
    let s = json!({"name": "b3_replay", "tiers": tiers, "live_calls_2_0": calls, "by_kind": eval3::summary(&out)});
    std::fs::write(dir.join("summary.json"), serde_json::to_string_pretty(&s).unwrap()).unwrap();
    println!("{}", serde_json::to_string_pretty(&s).unwrap());
    let _ = Arc::strong_count(&brain);
}
