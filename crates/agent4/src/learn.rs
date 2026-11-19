//! Rule learning, part A (Design 4.0 section 7.1-7.3, decision D4.7): item rules induced from
//! taught and inherited facts. A rule is a hypothesis with support, counterexamples and a
//! confidence; it is accepted only with support >= MIN_SUPPORT from >= 2 distinct kinds, no
//! unhandled counterexample and at least one new prediction; one confirming question is asked;
//! a counterexample specialises the rule (an added condition) or retracts it.
//!
//! Learned conclusions are rank L: they live in `lholds`/`lholdsn`, never override teaching,
//! and every proof that uses one carries a `learn/` rule, so it is reported as a hypothesis.

use crate::{Agent, Teacher};
use reason4::{INHERITABLE};
use serde::Serialize;
use sla_core::term::{GroundAtom, Value};
use sla_reasoner::RuleInput;
use std::collections::{BTreeMap, BTreeSet};

pub static MIN_SUPPORT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(3);

/// A literal over a subject X: `rel(X, value)` or its negation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Hash)]
pub struct Lit {
    pub rel: String,
    pub val: String,
    pub neg: bool,
}

impl Lit {
    fn show(&self) -> String {
        format!("{}{}(X, {})", if self.neg { "not " } else { "" }, self.rel, self.val)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Rule {
    pub id: String,
    pub layer: usize,
    pub head: Lit,
    pub body: Vec<Lit>,
    pub except: Vec<String>,
    pub support: Vec<String>,
    pub counter: Vec<String>,
    pub predictions: Vec<String>,
    pub status: String,
    pub confirmed_by: Option<String>,
    pub built_from: Vec<String>,
    pub specialised_from: Option<String>,
    pub conf: f64,
    #[serde(skip)]
    pub input: RuleInput,
}

impl Rule {
    pub fn show(&self) -> String {
        let mut b: Vec<String> = self.body.iter().map(Lit::show).collect();
        for e in &self.except {
            b.push(format!("X != {e}"));
        }
        format!("{} :- {}", self.head.show(), b.join(", "))
    }
}

/// Rules that let learned conclusions take part in derivation (never against teaching).
pub fn bridge_rules() -> Vec<RuleInput> {
    let src = "hh(X, P, Y) :- holds(X, P, Y).\nhh(X, P, Y) :- lholds(X, P, Y).\nhhn(X, P, Y) :- holdsn(X, P, Y).\nhhn(X, P, Y) :- lholdsn(X, P, Y).\ncan(X, F) :- lholds(X, can_do, F).\n";
    let (_, rules) = sla_reasoner::parse_program(src).unwrap();
    rules.into_iter().enumerate().map(|(k, mut r)| { r.id = format!("lbridge/{}", k + 1); r }).collect()
}

pub fn lholds_atom(a: &GroundAtom, negated: bool) -> GroundAtom {
    let sym = |s: &str| Value::Sym(s.into());
    GroundAtom::new(if negated { "lholdsn" } else { "lholds" }, vec![a.args[0].clone(), sym(&a.pred), a.args.get(1).cloned().unwrap_or(sym(""))])
}

fn sym_of(v: &Value) -> String {
    match v {
        Value::Sym(s) => s.clone(),
        o => o.to_string(),
    }
}

/// Features of every item subject: what holds about it, taught or inherited (never learned).
fn features(agent: &mut Agent) -> BTreeMap<String, BTreeSet<Lit>> {
    let items: BTreeSet<String> = agent.sorts.iter().filter(|(_, s)| *s == "item" || *s == "place").map(|(k, _)| k.clone()).collect();
    let m = agent.model();
    let mut out: BTreeMap<String, BTreeSet<Lit>> = BTreeMap::new();
    for (p, neg) in [("holds", false), ("holdsn", true)] {
        for g in m.facts_of(p) {
            let x = sym_of(&g.args[0]);
            if !items.contains(&x) {
                continue;
            }
            out.entry(x).or_default().insert(Lit { rel: sym_of(&g.args[1]), val: sym_of(&g.args[2]), neg });
        }
    }
    for x in items {
        out.entry(x).or_default();
    }
    out
}

fn parents(agent: &Agent, x: &str) -> BTreeSet<String> {
    agent.facts.iter().filter(|(f, s)| f.pred == "is_a" && s.retracted_by.is_none() && sym_of(&f.args[0]) == x).map(|(f, _)| sym_of(&f.args[1])).collect()
}

fn contrary(l: &Lit) -> Lit {
    Lit { neg: !l.neg, ..l.clone() }
}

fn make_input(id: &str, head: &Lit, body: &[Lit], except: &[String]) -> RuleInput {
    let hp = if head.neg { "lholdsn" } else { "lholds" };
    let mut b: Vec<String> = body.iter().map(|l| format!("{}(X, {}, {})", if l.neg { "hhn" } else { "hh" }, l.rel, l.val)).collect();
    b.push(format!("not holds(X, {}, {})", head.rel, head.val));
    b.push(format!("not holdsn(X, {}, {})", head.rel, head.val));
    for e in except {
        b.push(format!("X != {e}"));
    }
    let src = format!("{hp}(X, {}, {}) :- {}.", head.rel, head.val, b.join(", "));
    let (_, mut rules) = sla_reasoner::parse_program(&src).expect("learned rule parses");
    let mut r = rules.remove(0);
    r.id = id.to_string();
    r
}

struct Score {
    support: Vec<String>,
    counter: Vec<String>,
    predictions: Vec<String>,
}

fn score(feats: &BTreeMap<String, BTreeSet<Lit>>, head: &Lit, body: &[Lit], except: &[String]) -> Score {
    let mut s = Score { support: vec![], counter: vec![], predictions: vec![] };
    for (x, f) in feats {
        if except.contains(x) || !body.iter().all(|l| f.contains(l)) {
            continue;
        }
        if f.contains(head) {
            s.support.push(x.clone());
        } else if f.contains(&contrary(head)) {
            s.counter.push(x.clone());
        } else {
            s.predictions.push(x.clone());
        }
    }
    s
}

fn kinds_ok(agent: &Agent, support: &[String]) -> bool {
    let mut ks: BTreeSet<String> = BTreeSet::new();
    for x in support {
        let p = parents(agent, x);
        if p.is_empty() {
            ks.insert(format!("self:{x}"));
        }
        ks.extend(p);
    }
    ks.len() >= 2
}

/// One induction pass over the current knowledge. Returns the agent's lines.
pub fn induce(agent: &mut Agent, teacher: &mut dyn Teacher) -> Vec<String> {
    let mut out = retry_suspended(agent, false);
    out.extend(induce_cases(agent));
    let k = MIN_SUPPORT.load(std::sync::atomic::Ordering::SeqCst);
    let feats = features(agent);
    let mut heads: BTreeSet<Lit> = BTreeSet::new();
    for f in feats.values() {
        heads.extend(f.iter().filter(|l| INHERITABLE.contains(&l.rel.as_str())).cloned());
    }
    let known: BTreeSet<(Lit, Vec<Lit>)> = agent.learned.iter().map(|r| (r.head.clone(), r.body.clone())).collect();
    let mut accepted: Vec<(Lit, Vec<Lit>, Score)> = Vec::new();
    for head in &heads {
        // candidate literals: features shared by at least k subjects that have the head
        let with_head: Vec<&BTreeSet<Lit>> = feats.values().filter(|f| f.contains(head)).collect();
        if with_head.len() < k {
            continue;
        }
        let mut counts: BTreeMap<&Lit, usize> = BTreeMap::new();
        for f in &with_head {
            for l in f.iter() {
                if l.rel != head.rel || l.val != head.val {
                    *counts.entry(l).or_insert(0) += 1;
                }
            }
        }
        let cands: Vec<Lit> = counts.into_iter().filter(|(_, n)| *n >= k).map(|(l, _)| l.clone()).collect();
        let mut bodies: Vec<Vec<Lit>> = cands.iter().map(|l| vec![l.clone()]).collect();
        for i in 0..cands.len() {
            for j in i + 1..cands.len() {
                bodies.push(vec![cands[i].clone(), cands[j].clone()]);
            }
        }
        let mut best_for_head: Vec<(Vec<Lit>, Score)> = Vec::new();
        for body in bodies {
            if known.contains(&(head.clone(), body.clone())) {
                continue;
            }
            let s = score(&feats, head, &body, &[]);
            if s.support.len() >= k && s.counter.is_empty() && !s.predictions.is_empty() && kinds_ok(agent, &s.support) {
                // minimality: no accepted shorter body with an equal or larger support set
                let dominated = best_for_head.iter().any(|(b, bs)| b.len() < body.len() && body.iter().all(|l| b.contains(l) || true) && bs.support.len() >= s.support.len() && b.iter().all(|l| body.contains(l)));
                if !dominated {
                    best_for_head.push((body, s));
                }
            }
        }
        for (b, s) in best_for_head {
            accepted.push((head.clone(), b, s));
        }
    }
    for (head, body, s) in accepted {
        let n = agent.learned.len() + 1;
        let layer = 1 + agent
            .learned
            .iter()
            .filter(|r| r.status == "hypothesis" && body.iter().any(|l| l.rel == r.head.rel && l.val == r.head.val))
            .map(|r| r.layer)
            .max()
            .unwrap_or(0);
        let built_from: Vec<String> = agent.learned.iter().filter(|r| r.status == "hypothesis" && body.iter().any(|l| l.rel == r.head.rel && l.val == r.head.val)).map(|r| r.id.clone()).collect();
        let id = format!("learn/L{layer}/{n}");
        let conf = s.support.len() as f64 / (s.support.len() as f64 + 1.0);
        let mut rule = Rule {
            id: id.clone(), layer, head: head.clone(), body: body.clone(), except: vec![], support: s.support.clone(), counter: vec![],
            predictions: s.predictions.clone(), status: "hypothesis".into(), confirmed_by: None, built_from, specialised_from: None, conf,
            input: make_input(&id, &head, &body, &[]),
        };
        out.push(format!("hypothesis: {} support [{}] counter [] conf {:.2}.", rule.show(), rule.support.join(", "), conf));
        // one confirming question about its first new prediction
        let x = s.predictions[0].clone();
        let q = format!("ask: {}{}({x}, {})?", if head.neg { "not " } else { "" }, head.rel, head.val);
        let raw = agent_ask(agent, teacher, &q);
        let pred_line = format!("{}{}({x}, {})", if head.neg { "not " } else { "" }, head.rel, head.val);
        let contra_line = format!("{}{}({x}, {})", if head.neg { "" } else { "not " }, head.rel, head.val);
        let norm = |t: &str| t.chars().filter(|c| !c.is_whitespace()).collect::<String>().trim_end_matches('.').to_string();
        let a = if raw == "yes" || norm(&raw) == norm(&pred_line) { "yes".to_string() } else if raw == "no" || norm(&raw) == norm(&contra_line) { "no".to_string() } else { raw };
        if a == "yes" {
            rule.confirmed_by = Some(q.clone());
            rule.support.push(x.clone());
            let f = GroundAtom::new(&head.rel, vec![Value::Sym(x.clone()), Value::Sym(head.val.clone())]);
            agent_store(agent, f, head.neg);
        } else if a == "no" {
            rule.counter.push(x.clone());
            let f = GroundAtom::new(&head.rel, vec![Value::Sym(x.clone()), Value::Sym(head.val.clone())]);
            agent_store(agent, f, !head.neg);
            agent.learned.push(rule);
            let lines = revise(agent, agent.learned.len() - 1, &x);
            out.extend(lines);
            continue;
        }
        agent.learned.push(rule);
    }
    agent.reset_model();
    out
}

fn agent_ask(agent: &mut Agent, teacher: &mut dyn Teacher, q: &str) -> String {
    let n = agent.questions.len();
    agent.questions.push(q.to_string());
    agent.raw.push(serde_json::json!({"id": format!("{}.{}", agent.session, agent.raw.len()), "who": "agent", "channel": "question", "text": format!("ask {n}: {q}")}));
    let a = teacher.answer(q);
    agent.raw.push(serde_json::json!({"id": format!("{}.{}", agent.session, agent.raw.len()), "who": "teacher", "channel": "answer", "text": format!("answer {n}: {a}.")}));
    a
}

fn agent_store(agent: &mut Agent, f: GroundAtom, negated: bool) {
    let id = agent.raw.len().saturating_sub(1);
    agent.facts.push((f.clone(), crate::Stored { atom: f.to_string(), negated, msg: id, conf: 0.97, retracted_by: None }));
    agent.provenance.push(serde_json::json!({"fact": f.to_string(), "negated": negated, "msg": id, "conf": 0.97, "event": "stored", "via": "confirming question"}));
    agent.reset_model();
}

/// A taught fact contradicts a learned conclusion: withdraw it and revise the rule (7.2).
pub fn counterexample(agent: &mut Agent, a: &GroundAtom, negated: bool) -> Vec<String> {
    let x = sym_of(&a.args[0]);
    let val = a.args.get(1).map(sym_of).unwrap_or_default();
    let idx: Vec<usize> = agent
        .learned
        .iter()
        .enumerate()
        .filter(|(_, r)| r.status == "hypothesis" && r.head.rel == a.pred && r.head.val == val && r.head.neg != negated)
        .map(|(k, _)| k)
        .collect();
    let mut out = vec![format!("withdrawn: {}{} because taught {}{}", if negated { "" } else { "not " }, a, if negated { "not " } else { "" }, a)];
    for k in idx {
        agent.learned[k].counter.push(x.clone());
        out.extend(revise(agent, k, &x));
    }
    out
}

/// Specialise (an added literal all support shares and x lacks) or retract.
fn revise(agent: &mut Agent, k: usize, x: &str) -> Vec<String> {
    let feats = features(agent);
    let r = agent.learned[k].clone();
    let fx = feats.get(x).cloned().unwrap_or_default();
    let shared: Option<BTreeSet<Lit>> = r.support.iter().filter_map(|s| feats.get(s).cloned()).reduce(|a, b| a.intersection(&b).cloned().collect());
    let cand = shared.unwrap_or_default().into_iter().find(|l| !fx.contains(l) && !r.body.contains(l) && !(l.rel == r.head.rel && l.val == r.head.val));
    let minsup = MIN_SUPPORT.load(std::sync::atomic::Ordering::SeqCst);
    if let Some(l) = cand {
        let mut body = r.body.clone();
        body.push(l);
        let s = score(&feats, &r.head, &body, &[]);
        if s.support.len() >= minsup && s.counter.is_empty() {
            let id = format!("learn/L{}/{}", r.layer, agent.learned.len() + 1);
            agent.learned[k].status = "specialised".into();
            let nr = Rule {
                id: id.clone(), layer: r.layer, head: r.head.clone(), body: body.clone(), except: vec![], support: s.support.clone(), counter: vec![],
                predictions: s.predictions.clone(), status: "hypothesis".into(), confirmed_by: None, built_from: r.built_from.clone(),
                specialised_from: Some(r.id.clone()), conf: s.support.len() as f64 / (s.support.len() as f64 + 1.0), input: make_input(&id, &r.head, &body, &[]),
            };
            let line = format!("specialised: {} -> {} (counterexample {x})", r.show(), nr.show());
            agent.learned.push(nr);
            agent.reset_model();
            return vec![line];
        }
    }
    // nothing separates it yet: suspend (conclusions withdrawn) and retry when more is taught
    agent.learned[k].status = "suspended".into();
    agent.reset_model();
    vec![format!("ok. {} meets a counterexample: {x}.", r.id)]
}

/// Retry suspended rules (a separating feature may have been taught since); with `last`,
/// whatever is still suspended is retracted.
pub fn retry_suspended(agent: &mut Agent, last: bool) -> Vec<String> {
    let mut out = Vec::new();
    let ks: Vec<usize> = agent.learned.iter().enumerate().filter(|(_, r)| r.status == "suspended").map(|(k, _)| k).collect();
    for k in ks {
        let x = agent.learned[k].counter.last().cloned().unwrap_or_default();
        let lines = revise(agent, k, &x);
        if agent.learned[k].status == "suspended" && last {
            agent.learned[k].status = "retracted".into();
            out.push(format!("retracted: {} (counterexample {x}, nothing separates it)", agent.learned[k].show()));
        } else if agent.learned[k].status != "suspended" {
            out.extend(lines);
        }
    }
    agent.reset_model();
    out
}

/// Part B (7.1): an adaptation rule generalised from taught swap cases by least general
/// generalisation over a pool of schema literals (role, can, uses, demands met). Accepted with
/// support >= MIN_SUPPORT cases from >= 2 tasks. Active only while B7 is switched off: a learned
/// rule never shadows a deduction.
pub fn induce_cases(agent: &mut Agent) -> Vec<String> {
    let k = MIN_SUPPORT.load(std::sync::atomic::Ordering::SeqCst);
    if agent.learned.iter().any(|r| r.id.starts_with("learn/B/")) || agent.cases.is_empty() {
        return vec![];
    }
    let off: Vec<&str> = agent.off.iter().map(String::as_str).collect();
    // literal pool, evaluated per case under the case's own situation
    let mut shared: Option<BTreeSet<&'static str>> = None;
    let mut tasks: BTreeSet<String> = BTreeSet::new();
    let mut n = 0;
    for (_, sit, diff) in agent.cases.clone() {
        for d in diff.iter().filter(|d| d.pred == "swap") {
            let (s, i, j) = (sym_of(&d.args[0]), sym_of(&d.args[1]), sym_of(&d.args[2]));
            let m = reason4::run(&agent.engine_facts(&sit), &off, &agent.learned_rules()).expect("model");
            let role_f: Vec<String> = m.facts_of("role").into_iter().filter(|g| sym_of(&g.args[0]) == s && sym_of(&g.args[1]) == i).map(|g| sym_of(&g.args[2])).collect();
            let mut lits: BTreeSet<&'static str> = BTreeSet::new();
            if !role_f.is_empty() {
                lits.insert("role");
            }
            if role_f.iter().any(|f| m.contains(&GroundAtom::new("can", vec![Value::Sym(j.clone()), Value::Sym(f.clone())]))) {
                lits.insert("can");
            }
            if m.contains(&GroundAtom::new("uses", vec![Value::Sym(s.clone()), Value::Sym(i.clone())])) {
                lits.insert("uses");
            }
            let demands: Vec<GroundAtom> = m.facts_of("demands").into_iter().filter(|g| sym_of(&g.args[0]) == s).collect();
            let met = demands.iter().all(|g| m.contains(&reason4::holds_atom(&GroundAtom::new(&sym_of(&g.args[2]), vec![Value::Sym(j.clone()), g.args[3].clone()]), false)));
            if met {
                lits.insert("demands_met");
            }
            shared = Some(match shared { None => lits, Some(sh) => sh.intersection(&lits).cloned().collect() });
            tasks.insert(s.rsplit_once("_s").map(|(t, _)| t.to_string()).unwrap_or(s.clone()));
            n += 1;
        }
    }
    let shared = shared.unwrap_or_default();
    if n < k || tasks.len() < 2 || !shared.contains("role") || !shared.contains("can") {
        let t = tasks.len();
        return vec![format!("induce_cases: none; support {n}; tasks {t}; shared {shared:?}")];
    }
    let mut body = vec!["lacks(user, I)", "has(user, J)", "role(S, I, F)", "can(J, F)", "J != I"];
    let mut src = String::new();
    if shared.contains("demands_met") {
        src.push_str("lunfit(S, J) :- lacks(user, I), role(S, I, F), has(user, J), demands(S, F, P, Y), holdsn(J, P, Y).\n");
        src.push_str("lopen(S, J) :- lacks(user, I), role(S, I, F), has(user, J), demands(S, F, P, Y), not holds(J, P, Y), not holdsn(J, P, Y).\n");
        body.push("not lunfit(S, J)");
        body.push("not lopen(S, J)");
    }
    src.push_str(&format!("stand_in(S, I, J) :- {}.\n", body.join(", ")));
    let (_, rules) = sla_reasoner::parse_program(&src).expect("part B rule");
    let active = agent.off.iter().any(|o| o == "B7");
    for (q, mut r) in rules.into_iter().enumerate() {
        r.id = format!("learn/B/{}", q + 1);
        let shown = r.rule.to_string();
        agent.learned.push(Rule {
            id: r.id.clone(), layer: 1, head: Lit { rel: "stand_in".into(), val: "-".into(), neg: false }, body: vec![], except: vec![],
            support: vec![format!("{n} swap cases")], counter: vec![], predictions: vec![], status: if active { "hypothesis".into() } else { "subsumed".into() },
            confirmed_by: None, built_from: vec![], specialised_from: None, conf: n as f64 / (n as f64 + 1.0), input: r,
        });
        let _ = shown;
    }
    agent.reset_model();
    let status = if active { "hypothesis" } else { "subsumed" };
    let t = tasks.len();
    let shown = body.join(", ");
    vec![format!("{status}: stand_in(S, I, J) :- {shown}; support {n}; tasks {t}.")]
}
