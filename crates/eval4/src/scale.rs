//! Size stress (milestone 32): the fixture copied N times with item, place, task and step
//! symbols renamed per copy (functions, materials, properties and states stay shared, so the
//! copies are cross-linked through them), plus an is_a link from every 20th item to its
//! original. Measures full-closure time, model size and proof depth.

use crate::{deliver, Fixture, Opts};
use serde_json::{json, Value as J};
use std::time::Instant;

pub fn stress(fx: &Fixture, copies: usize) -> J {
    let base = deliver(fx, &Opts { learn: false, naming: false, prefix: 1.0, min_support: 3, ..Default::default() });
    let sorts = base.agent.sorts.clone();
    let rename = |line: &str, k: usize| -> String {
        let mut out = String::new();
        let mut tok = String::new();
        let flush = |tok: &mut String, out: &mut String| {
            if !tok.is_empty() {
                let s = sorts.get(tok.as_str()).map(String::as_str);
                if matches!(s, Some("item" | "place" | "task" | "step")) {
                    out.push_str(&format!("{tok}_c{k}"));
                } else if tok.contains("_s") && sorts.get(tok.split("_s").next().unwrap_or("")).map(String::as_str) == Some("task") {
                    out.push_str(&format!("{tok}_c{k}"));
                } else {
                    out.push_str(tok);
                }
                tok.clear();
            }
        };
        for ch in line.chars() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                tok.push(ch);
            } else {
                flush(&mut tok, &mut out);
                out.push(ch);
            }
        }
        flush(&mut tok, &mut out);
        out
    };
    let mut teach = Vec::new();
    for k in 0..copies {
        for l in &fx.teach {
            let mut l2 = l.clone();
            l2.text = if k == 0 { l.text.clone() } else { rename(&l.text, k) };
            teach.push(l2);
        }
        if k > 0 {
            for (n, (s, sort)) in sorts.iter().enumerate() {
                if sort == "item" && n % 20 == 0 {
                    teach.push(crate::Line { file: "scale".into(), no: 0, text: format!("is_a({s}_c{k}, {s}).") });
                }
            }
        }
    }
    let big = Fixture { teach, answers: fx.answers.clone(), questions: vec![], deny: fx.deny.clone() };
    let t0 = Instant::now();
    let mut d = deliver(&big, &Opts { learn: false, naming: false, prefix: 1.0, min_support: 3, ..Default::default() });
    let teach_ms = t0.elapsed().as_millis();
    let facts = d.agent.engine_facts(&[]).len();
    d.agent.reset_model();
    let t1 = Instant::now();
    let m = d.agent.model();
    let closure_ms = t1.elapsed().as_millis();
    let size = m.len();
    // answer the original questions on the big knowledge base: do the answers stay the same?
    let mut same = 0;
    let mut n = 0;
    let mut small = deliver(fx, &Opts { learn: false, naming: false, prefix: 1.0, min_support: 3, ..Default::default() });
    let t2 = Instant::now();
    for q in &fx.questions {
        let a = small.agent.respond(&q.query);
        let b = d.agent.respond(&q.query);
        n += 1;
        let ca: Vec<&String> = a.claims.iter().map(|c| &c.0).collect();
        let cb: Vec<&String> = b.claims.iter().map(|c| &c.0).collect();
        same += usize::from(ca == cb);
    }
    let answer_ms = t2.elapsed().as_millis();
    json!({"copies": copies, "taught_facts": facts, "model_facts": size, "teach_ms": teach_ms, "closure_ms": closure_ms,
           "answer_ms_all_questions": answer_ms, "questions": n, "same_answers_as_x1": same})
}
