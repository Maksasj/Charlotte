//! Milestone 31 acceptance for the harness: on a small inline fixture, full teaching answers
//! every answerable question and an empty agent none; a planted leak is withheld; the Rust
//! proof self-check agrees with tools/design4/ref_reason.py.

use eval4::{deliver, score, summary, Fixture, Opts};

fn mini(dir: &std::path::Path, deny: &str) {
    std::fs::create_dir_all(dir.join("teach")).unwrap();
    std::fs::create_dir_all(dir.join("bench")).unwrap();
    std::fs::create_dir_all(dir.join("script")).unwrap();
    std::fs::write(dir.join("teach/baking.msgs"), "topic baking/tins.\nnew(oven, place).\nnew(hold, function).\nis_a(loaf_tin, baking_tin).\nis_a(muffin_tray, baking_tin).\nis_a(baking_tin, tin).\ncan_do_on(baking_tin, hold, batter).\ncan_go_in(tin, oven).\nis_a(plastic_tub, tin).\nnot can_go_in(plastic_tub, oven).\nnew(bake_bread, task).\nstep(bake_bread, bake_bread_s4).\nuses(bake_bread_s4, loaf_tin).\nrole(bake_bread_s4, loaf_tin, hold).\ndemands(bake_bread_s4, hold, can_go_in, oven).\n").unwrap();
    std::fs::write(dir.join("script/answers.jsonl"), "{\"q\": \"conflict: not can_go_in(plastic_tub, oven) vs inherited contrary. exception?\", \"a\": \"exception\"}\n").unwrap();
    std::fs::write(dir.join("bench/questions.jsonl"), concat!(
        "{\"id\":\"q1\",\"type\":\"K\",\"hops\":2,\"query\":\"query: can_go_in(muffin_tray, oven).\",\"gold\":{\"kind\":\"answer\",\"atoms\":[\"can_go_in(muffin_tray, oven)\"]}}\n",
        "{\"id\":\"q2\",\"type\":\"SUB\",\"hops\":2,\"query\":\"request(bake_bread). lacks(user, loaf_tin). has(user, muffin_tray).\",\"gold\":{\"kind\":\"answer\",\"atoms\":[\"swap(bake_bread_s4, loaf_tin, muffin_tray)\"]}}\n",
        "{\"id\":\"q3\",\"type\":\"EXC\",\"hops\":1,\"query\":\"query: can_go_in(plastic_tub, oven).\",\"gold\":{\"kind\":\"gap\",\"atoms\":[]}}\n",
        "{\"id\":\"q4\",\"type\":\"GAP\",\"hops\":null,\"query\":\"request(bake_bread). lacks(user, loaf_tin). has(user, plastic_tub).\",\"gold\":{\"kind\":\"gap\",\"atoms\":[]}}\n")).unwrap();
    std::fs::write(dir.join("bench/deny.txt"), deny).unwrap();
}

fn tmp(name: &str) -> std::path::PathBuf {
    let d = std::env::temp_dir().join(format!("eval4_{name}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    d
}

fn opts(prefix: f64) -> Opts {
    Opts { learn: true, naming: true, prefix, min_support: 3, ..Default::default() }
}

#[test]
fn full_teaching_answers_all_and_an_empty_agent_none() {
    let d = tmp("gold");
    mini(&d, "can_go_in(muffin_tray, oven)\nswap(bake_bread_s4, loaf_tin, muffin_tray)\n");
    let fx = Fixture::load(&d).unwrap();
    let mut run = deliver(&fx, &opts(1.0));
    let sc: Vec<_> = fx.questions.iter().map(|q| score(&mut run.agent, q)).collect();
    assert!(sc.iter().all(|s| s.class == "correct"), "{sc:?}");
    assert!(run.leaks.is_empty());
    let mut empty = deliver(&fx, &opts(0.0));
    let sc: Vec<_> = fx.questions.iter().map(|q| score(&mut empty.agent, q)).collect();
    let s = summary(&sc);
    assert_eq!(s["g1_two_hop"]["correct"], 0);
}

#[test]
fn a_planted_leak_is_withheld_before_the_agent_sees_it() {
    let d = tmp("leak");
    // the deny list names a teaching line: it must be withheld, so the dependent answer is a gap
    mini(&d, "can_go_in(tin, oven)\n");
    let fx = Fixture::load(&d).unwrap();
    let mut run = deliver(&fx, &opts(1.0));
    assert!(run.withheld >= 1);
    assert!(run.leaks.is_empty(), "a withheld message is never stored");
    let q = fx.questions.iter().find(|q| q.id == "q1").unwrap();
    assert_eq!(score(&mut run.agent, q).class, "gap");
}

#[test]
fn rust_self_check_agrees_with_ref_reason() {
    let d = tmp("ref");
    mini(&d, "");
    let fx = Fixture::load(&d).unwrap();
    let mut run = deliver(&fx, &opts(1.0));
    let mut lines = String::new();
    for q in &fx.questions {
        let given: Vec<String> = match agent4::msg::parse(&q.query) {
            Ok(Some(agent4::msg::Msg::Query { given, .. })) | Ok(Some(agent4::msg::Msg::Request { given, .. })) => given.iter().map(|(g, n)| reason4::encode(g, *n).to_string()).collect(),
            _ => vec![],
        };
        for (c, rec) in run.agent.respond(&q.query).claims {
            lines.push_str(&serde_json::json!({"q": q.id, "claim": c, "proof": rec, "given": given}).to_string());
            lines.push('\n');
        }
    }
    let p = d.join("proofs.jsonl");
    std::fs::write(&p, lines).unwrap();
    let facts: Vec<String> = run.agent.engine_facts(&[]).iter().map(|f| f.atom.to_string()).collect();
    std::fs::write(d.join("facts.txt"), facts.join("\n")).unwrap();
    let root = eval4::root();
    let out = std::process::Command::new("python3").arg("-I").arg(root.join("tools/design4/ref_reason.py")).arg("--proofs").arg(&p).arg("--facts").arg(d.join("facts.txt")).output().unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success(), "ref_reason disagreed: {s} {}", String::from_utf8_lossy(&out.stderr));
    assert!(s.contains("\"failed\": 0"), "{s}");
}
