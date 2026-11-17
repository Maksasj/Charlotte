//! Milestone 16 acceptance: wording.
//!
//! - **Pairwise preference**: the same content plans (`nlg/tests/common/
//!   plans_data.rs`) realized with v1's wording and with 2.0's; a Haiku judge
//!   sees what the user said and the two replies in random order, and picks
//!   the better one. Plans whose two wordings are identical carry no
//!   preference and are only counted.
//! - **Ceiling run** (evaluation only, never shipped): a model rewrites each
//!   2.0 reply into fluent text without adding or removing facts; the judge
//!   compares the symbolic reply with the fluent one. The ceiling's win rate
//!   is how much of the remaining gap is wording.

#[path = "../../nlg/tests/common/plans_data.rs"]
#[allow(dead_code)]
mod plans_data;

use serde::Serialize;
use sla_nlg::{Act, Realizer};
use std::sync::Mutex;

/// What the user said or asked, for the judge.
fn context(r: &Realizer, act: &Act) -> String {
    match act {
        Act::YesNo { atom, .. } => format!("The user asked: \"{}\"", r.yes_no_question(atom)),
        Act::Wh { pattern, .. } => format!("The user asked: \"{}\"", r.wh_question(pattern)),
        Act::Explain(p) => format!("The user asked why the agent believes that {}.", r.clause(&p.atom)),
        Act::ExplainUnknown(a) => format!("The user asked why {}.", r.clause(a)),
        Act::Recall { entity, .. } => format!("The user asked: \"What do you know about {}?\"", entity.replace('_', " ")),
        Act::Acknowledge { added, known, .. } => match added.first().or(known.first()) {
            Some(a) => format!("The user told the agent: \"{}.\"", r.clause(a)),
            None => "The user taught the agent a rule.".into(),
        },
        Act::Corrected { added, .. } => format!("The agent got something wrong and the user corrected it: \"No, {}.\"", added.first().map(|a| r.clause(a)).unwrap_or_default()),
        Act::AskYesNo(_) | Act::AskWh(_) | Act::AskRule(_) | Act::AskContradiction(..) => "The agent asks the user a question to learn more.".into(),
        Act::Greet => "The user said: \"Hi!\"".into(),
        Act::Bye => "The user said: \"Bye!\"".into(),
        Act::Thanks => "The agent thanks the user.".into(),
        Act::NotUnderstood => "The user wrote something the agent could not understand.".into(),
        _ => "The user is chatting with the agent.".into(),
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct PairResult {
    pub context: String,
    pub first: String,
    pub second: String,
    /// "first", "second" or "none" (judge failed).
    pub winner: String,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct WordingReport {
    pub plans: usize,
    pub identical: usize,
    /// New (2.0) wording vs old (v1): new wins, comparisons.
    pub new_vs_old: (usize, usize),
    pub pairs: Vec<PairResult>,
    /// Ceiling (fluent rewrite) vs 2.0: ceiling wins, comparisons.
    pub ceiling: Option<(usize, usize)>,
    pub ceiling_pairs: Vec<PairResult>,
}

fn judge(model: &str, ctx: &str, a: &str, b: &str) -> Option<char> {
    let prompt = format!(
        "{ctx}\n\nTwo possible replies from a simple assistant, with the same facts:\n\nA: {a}\n\nB: {b}\n\n\
         Which reply is better for the user: natural, clear, not repetitive, and not echoing back what the user just said? \
         Answer with exactly one letter: A or B."
    );
    let out = crate::claude_as("wording_judge", &prompt, model).ok()?;
    let t = out.trim().trim_matches(|c: char| !c.is_ascii_alphabetic());
    match t.chars().next()? {
        'A' | 'a' => Some('A'),
        'B' | 'b' => Some('B'),
        _ => None,
    }
}

fn rewrite(model: &str, ctx: &str, reply: &str) -> Option<String> {
    let prompt = format!(
        "{ctx}\n\nThe assistant's reply: {reply}\n\nRewrite the reply so it reads naturally, as a friendly person would say it. \
         Keep exactly the same facts: add nothing, leave nothing out. Output only the rewritten reply."
    );
    crate::claude_as("ceiling", &prompt, model).ok().map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
}

/// Compare (a, b) pairs; `a_is_target` says which side we count wins for.
fn compare(model: &str, items: Vec<(String, String, String)>, parallel: usize) -> Vec<(PairResult, bool)> {
    let next = std::sync::atomic::AtomicUsize::new(0);
    let out: Mutex<Vec<(usize, PairResult, bool)>> = Mutex::new(Vec::new());
    std::thread::scope(|s| {
        for _ in 0..parallel.max(1) {
            s.spawn(|| loop {
                let i = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                let Some((ctx, target, other)) = items.get(i) else { break };
                // Alternate the order so position does not decide.
                let (first, second, target_first) = if i % 2 == 0 { (target, other, true) } else { (other, target, false) };
                let w = judge(model, ctx, first, second);
                let target_won = match w {
                    Some('A') => target_first,
                    Some('B') => !target_first,
                    _ => false,
                };
                let r = PairResult { context: ctx.clone(), first: first.clone(), second: second.clone(), winner: match w { Some('A') => "first".into(), Some('B') => "second".into(), _ => "none".into() } };
                out.lock().unwrap_or_else(|e| e.into_inner()).push((i, r, target_won && w.is_some()));
            });
        }
    });
    let mut v = out.into_inner().unwrap_or_default();
    v.sort_by_key(|x| x.0);
    v.into_iter().map(|(_, r, won)| (r, won)).collect()
}

pub fn run(model: &str, ceiling: bool, parallel: usize) -> WordingReport {
    let m = plans_data::model();
    let new = plans_data::realizer();
    let mut old = plans_data::realizer();
    old.legacy = true;
    let acts = plans_data::plans(&m);
    let mut rep = WordingReport { plans: acts.len(), ..Default::default() };
    let mut items = Vec::new();
    let mut news = Vec::new();
    for act in &acts {
        let a = new.realize(act).text;
        let b = old.realize(act).text;
        let ctx = context(&new, act);
        news.push((ctx.clone(), a.clone()));
        if a == b {
            rep.identical += 1;
        } else {
            items.push((ctx, a, b));
        }
    }
    let results = compare(model, items, parallel);
    rep.new_vs_old = (results.iter().filter(|(_, w)| *w).count(), results.iter().filter(|(r, _)| r.winner != "none").count());
    rep.pairs = results.into_iter().map(|(r, _)| r).collect();
    if ceiling {
        // The ceiling for the replies that say something (not greetings).
        let sample: Vec<(String, String)> = news.into_iter().filter(|(_, t)| t.split_whitespace().count() > 3).collect();
        let next = std::sync::atomic::AtomicUsize::new(0);
        let rewritten: Mutex<Vec<(usize, String, String, String)>> = Mutex::new(Vec::new());
        std::thread::scope(|s| {
            for _ in 0..parallel.max(1) {
                s.spawn(|| loop {
                    let i = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    let Some((ctx, t)) = sample.get(i) else { break };
                    if let Some(c) = rewrite(model, ctx, t) {
                        rewritten.lock().unwrap_or_else(|e| e.into_inner()).push((i, ctx.clone(), c, t.clone()));
                    }
                });
            }
        });
        let mut rw = rewritten.into_inner().unwrap_or_default();
        rw.sort_by_key(|x| x.0);
        let items: Vec<(String, String, String)> = rw.into_iter().map(|(_, ctx, c, t)| (ctx, c, t)).collect();
        let results = compare(model, items, parallel);
        rep.ceiling = Some((results.iter().filter(|(_, w)| *w).count(), results.iter().filter(|(r, _)| r.winner != "none").count()));
        rep.ceiling_pairs = results.into_iter().map(|(r, _)| r).collect();
    }
    rep
}

pub fn markdown(r: &WordingReport) -> String {
    let (w, n) = r.new_vs_old;
    let (p, lo, hi) = crate::outcome::wilson(w, n);
    let mut s = format!(
        "# Wording\n\n{} content plans; {} worded the same both ways.\n\n2.0 wording preferred over v1 in **{w} of {n}** pairs ({:.1}%, 95% interval {:.1} to {:.1}).\n",
        r.plans,
        r.identical,
        p * 100.0,
        lo * 100.0,
        hi * 100.0
    );
    if let Some((cw, cn)) = r.ceiling {
        let (cp, clo, chi) = crate::outcome::wilson(cw, cn);
        s.push_str(&format!(
            "\nCeiling run (a model rewrites the 2.0 reply fluently, evaluation only): the fluent rewrite is preferred in **{cw} of {cn}** pairs ({:.1}%, {:.1} to {:.1}). That share is the part of the remaining gap that is wording.\n",
            cp * 100.0,
            clo * 100.0,
            chi * 100.0
        ));
    }
    s.push_str("\n## Pairs (2.0 vs v1)\n\n");
    for x in &r.pairs {
        s.push_str(&format!("- {}\n  - first: {}\n  - second: {}\n  - judge: {}\n", x.context, x.first, x.second, x.winner));
    }
    if !r.ceiling_pairs.is_empty() {
        s.push_str("\n## Pairs (fluent rewrite vs 2.0)\n\n");
        for x in &r.ceiling_pairs {
            s.push_str(&format!("- {}\n  - first: {}\n  - second: {}\n  - judge: {}\n", x.context, x.first, x.second, x.winner));
        }
    }
    s
}
