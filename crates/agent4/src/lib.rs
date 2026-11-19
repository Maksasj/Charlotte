//! Charlotte 4.0 agent (Design 4.0 section 6): every piece of knowledge enters as a message in
//! a conversation and goes through parse, type check, naming check, consistency check, store
//! (raw, facts, provenance), derive, learn and gaps. Nothing is injected.

pub mod learn;
pub mod msg;
pub mod sorts;

use msg::Msg;
use reason4::{encode, holds_atom, surface};
use serde::Serialize;
use serde_json::{json, Value as J};
use sla_core::term::{GroundAtom, Value};
use sla_reasoner::{FactInput, Model, RuleInput};
use std::collections::{BTreeMap, BTreeSet};

/// The teacher side of the conversation (an oracle in the spike, Haiku later).
pub trait Teacher {
    fn answer(&mut self, question: &str) -> String;
}

#[derive(Debug, Clone, Serialize)]
pub struct Stored {
    pub atom: String,
    pub negated: bool,
    pub msg: usize,
    pub conf: f64,
    pub retracted_by: Option<usize>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Stats {
    pub messages: usize,
    pub accepted: usize,
    pub rejected: usize,
    pub naming_questions: usize,
    pub conflict_questions: usize,
    pub unknown_answers: usize,
    pub pending: usize,
}

/// The agent's reply to a query or request.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Answer {
    /// claims with their proof records
    pub claims: Vec<(String, J)>,
    pub gaps: Vec<String>,
    pub asks: Vec<String>,
}

pub struct Agent {
    pub session: String,
    pub sorts: BTreeMap<String, String>,
    pub facts: Vec<(GroundAtom, Stored)>,
    pub raw: Vec<J>,
    pub provenance: Vec<J>,
    /// alias -> canonical name (B4 by canonicalisation)
    pub canon: BTreeMap<String, String>,
    pub not_same: BTreeSet<(String, String)>,
    pub questions: Vec<String>,
    pub learned: Vec<learn::Rule>,
    pub off: Vec<String>,
    pub learning: bool,
    pub naming: bool,
    pub stats: Stats,
    model: Option<Model>,
}

fn sym_of(v: &Value) -> String {
    match v {
        Value::Sym(s) => s.clone(),
        other => other.to_string(),
    }
}

impl Agent {
    pub fn new(session: &str) -> Agent {
        Agent {
            session: session.into(), sorts: BTreeMap::new(), facts: Vec::new(), raw: Vec::new(), provenance: Vec::new(),
            canon: BTreeMap::new(), not_same: BTreeSet::new(), questions: Vec::new(), learned: Vec::new(), off: Vec::new(),
            learning: true, naming: true, stats: Stats::default(), model: None,
        }
    }

    fn log(&mut self, who: &str, channel: &str, text: &str) -> usize {
        let id = self.raw.len();
        self.raw.push(json!({"id": format!("{}.{id}", self.session), "who": who, "channel": channel, "text": text, "schema": "v4"}));
        id
    }

    fn canonical(&self, s: &str) -> String {
        let mut cur = s.to_string();
        while let Some(n) = self.canon.get(&cur) {
            if *n == cur {
                break;
            }
            cur = n.clone();
        }
        cur
    }

    fn canon_atom(&self, a: &GroundAtom) -> GroundAtom {
        GroundAtom::new(&a.pred, a.args.iter().map(|v| match v { Value::Sym(s) => Value::Sym(self.canonical(s)), o => o.clone() }).collect())
    }

    /// Type check: every argument fits its sort; first use declares the sort.
    fn type_check(&mut self, a: &GroundAtom) -> Result<(), String> {
        let sig = sorts::signature(&a.pred).ok_or_else(|| format!("unknown relation {}", a.pred))?;
        if sig.len() != a.args.len() {
            return Err(format!("{} takes {} arguments", a.pred, sig.len()));
        }
        let mut same: Option<String> = None;
        let mut assign = Vec::new();
        for (v, want) in a.args.iter().zip(sig.iter()) {
            let s = sym_of(v);
            if *want == "relation" {
                if sorts::signature(&s).is_none() {
                    return Err(format!("{s} is not a relation"));
                }
                continue;
            }
            let declared = self.sorts.get(&s).cloned();
            match (*want, declared) {
                ("same", Some(d)) => {
                    if let Some(prev) = &same {
                        if !sorts::compatible(&d, prev) && !sorts::compatible(prev, &d) {
                            return Err(format!("{s} is {d}, not {prev}"));
                        }
                    }
                    same = Some(d);
                }
                ("same", None) => assign.push((s, "same")),
                (w, Some(d)) => {
                    if !sorts::compatible(&d, w) {
                        return Err(format!("{s} is {d}, not {w}"));
                    }
                }
                (w, None) => {
                    if w != "any" {
                        assign.push((s, w));
                    }
                }
            }
        }
        for (s, w) in assign {
            let w = if w == "same" { same.clone().unwrap_or_else(|| "item".into()) } else { w.to_string() };
            self.sorts.insert(s, w);
        }
        Ok(())
    }

    fn ask(&mut self, teacher: &mut dyn Teacher, q: &str) -> String {
        let n = self.questions.len();
        self.questions.push(q.to_string());
        self.log("agent", "question", &format!("ask {n}: {q}"));
        let a = teacher.answer(q);
        self.log("teacher", "answer", &format!("answer {n}: {a}."));
        if a == "unknown" {
            self.stats.unknown_answers += 1;
        }
        a
    }

    /// Naming check (4.5): a new symbol whose last token is a known symbol of the same sort, or
    /// the other way round, is asked about before its facts are used.
    fn naming(&mut self, teacher: &mut dyn Teacher, a: &GroundAtom) -> bool {
        if !self.naming || matches!(a.pred.as_str(), "is_a" | "same_as") {
            return true;
        }
        let syms: Vec<String> = a.args.iter().map(sym_of).filter(|s| !self.facts.iter().any(|(f, _)| f.args.iter().any(|v| sym_of(v) == *s))).collect();
        for n in syms {
            let sort = self.sorts.get(&n).cloned().unwrap_or_default();
            if sort != "item" && sort != "place" {
                continue;
            }
            let head = n.rsplit('_').next().unwrap_or(&n).to_string();
            // the message's own relation and value, to compare neighbourhoods (E2)
            let here: Option<(String, String)> = (a.args.len() == 2 && sym_of(&a.args[0]) == n).then(|| (a.pred.clone(), sym_of(&a.args[1])));
            let shares = |k: &str, facts: &[(GroundAtom, Stored)]| here.as_ref().map(|(p, v)| facts.iter().any(|(f, s)| s.retracted_by.is_none() && f.pred == *p && f.args.len() == 2 && sym_of(&f.args[0]) == k && sym_of(&f.args[1]) == *v)).unwrap_or(false);
            let facts = self.facts.clone();
            let known: Option<String> = self.sorts.iter().filter(|(k, s)| **k != n && sorts::compatible(s, &sort)).map(|(k, _)| k.clone())
                .find(|k| (n.contains('_') && *k == head) || (k.contains('_') && k.rsplit('_').next() == Some(n.as_str()))
                    || (n.split('_').any(|t| t == k) && shares(k, &facts)));
            if let Some(k) = known {
                if self.not_same.contains(&(n.clone(), k.clone())) || self.facts.iter().any(|(f, _)| f.pred == "is_a" && f.args.iter().map(sym_of).collect::<Vec<_>>() == [n.clone(), k.clone()]) {
                    continue;
                }
                self.stats.naming_questions += 1;
                let ans = self.ask(teacher, &format!("same_as({n}, {k})? or is_a({n}, {k})?"));
                if ans.starts_with("same_as") || ans == "yes" {
                    self.canon.insert(n.clone(), k.clone());
                } else if ans.starts_with("is_a") {
                    let isa = GroundAtom::new("is_a", vec![Value::Sym(n.clone()), Value::Sym(k.clone())]);
                    self.store(isa, false, 0, 0.97);
                } else {
                    self.not_same.insert((n.clone(), k.clone()));
                }
            }
        }
        true
    }

    fn store(&mut self, a: GroundAtom, negated: bool, msg: usize, conf: f64) {
        if self.facts.iter().any(|(f, s)| *f == a && s.negated == negated && s.retracted_by.is_none()) {
            return;
        }
        self.provenance.push(json!({"fact": a.to_string(), "negated": negated, "msg": msg, "conf": conf, "session": self.session, "event": "stored"}));
        self.facts.push((a.clone(), Stored { atom: a.to_string(), negated, msg, conf, retracted_by: None }));
        self.stats.accepted += 1;
        self.model = None;
    }

    pub fn reset_model(&mut self) {
        self.model = None;
    }

    pub fn engine_facts(&self, extra: &[(GroundAtom, bool)]) -> Vec<FactInput> {
        let mut out: Vec<FactInput> = self
            .facts
            .iter()
            .filter(|(_, s)| s.retracted_by.is_none())
            .map(|(a, s)| FactInput { atom: encode(a, s.negated), conf: s.conf, source: format!("raw:{}.{}", self.session, s.msg) })
            .collect();
        for (a, neg) in extra {
            out.push(FactInput { atom: encode(&self.canon_atom(a), *neg), conf: 1.0, source: "given".into() });
        }
        out
    }

    pub fn learned_rules(&self) -> Vec<RuleInput> {
        if !self.learning {
            return Vec::new();
        }
        let mut v = learn::bridge_rules();
        v.extend(self.learned.iter().filter(|r| r.status == "hypothesis").map(|r| r.input.clone()));
        v
    }

    pub fn model(&mut self) -> &Model {
        if self.model.is_none() {
            let off: Vec<&str> = self.off.iter().map(String::as_str).collect();
            self.model = Some(reason4::run(&self.engine_facts(&[]), &off, &self.learned_rules()).expect("model"));
        }
        self.model.as_ref().unwrap()
    }

    fn holds_now(&mut self, a: &GroundAtom, negated: bool) -> bool {
        let h = holds_atom(a, negated);
        self.model().contains(&h)
    }

    /// Process one teacher message; returns the agent's reply lines.
    pub fn hear(&mut self, line: &str, channel: &str, teacher: &mut dyn Teacher) -> Vec<String> {
        self.stats.messages += 1;
        let id = self.log("teacher", channel, line);
        let parsed = match msg::parse(line) {
            Ok(Some(m)) => m,
            Ok(None) => return vec![],
            Err(e) => {
                self.stats.rejected += 1;
                return vec![format!("reject: {e}.")];
            }
        };
        match parsed {
            Msg::Topic(_) | Msg::Answer { .. } | Msg::Query { .. } | Msg::Request { .. } => vec![],
            Msg::New { sym, sort } => {
                if !sorts::SORTS.contains(&sort.as_str()) {
                    self.stats.rejected += 1;
                    return vec![format!("reject: unknown sort {sort}.")];
                }
                if let Some(d) = self.sorts.get(&sym) {
                    if d != &sort {
                        self.stats.rejected += 1;
                        return vec![format!("reject: {sym} is {d}, not {sort}.")];
                    }
                }
                self.sorts.insert(sym, sort);
                vec!["ok.".into()]
            }
            Msg::Retract { atom, negated } => {
                let a = self.canon_atom(&atom);
                for (f, s) in self.facts.iter_mut() {
                    if *f == a && s.negated == negated && s.retracted_by.is_none() {
                        s.retracted_by = Some(id);
                    }
                }
                self.provenance.push(json!({"fact": a.to_string(), "negated": negated, "msg": id, "event": "retracted"}));
                self.model = None;
                vec!["ok. retracted.".into()]
            }
            Msg::Fact { atom, negated } => {
                if let Err(e) = self.type_check(&atom) {
                    self.stats.rejected += 1;
                    return vec![format!("reject: {e}.")];
                }
                self.naming(teacher, &atom);
                let a = self.canon_atom(&atom);
                if a.pred == "same_as" {
                    let (x, y) = (sym_of(&a.args[0]), sym_of(&a.args[1]));
                    if negated {
                        self.not_same.insert((x, y));
                    } else if x != y {
                        self.canon.insert(x, y);
                        self.model = None;
                    }
                    return vec!["ok.".into()];
                }
                let mut out = Vec::new();
                // consistency: a contrary conclusion already holds
                let inheritable = reason4::INHERITABLE.contains(&a.pred.as_str()) || reason4::INHERITABLE3.contains(&a.pred.as_str());
                let contrary_deduced = inheritable && self.holds_now(&a, !negated);
                let contrary_learned = inheritable && !contrary_deduced && a.args.len() == 2 && self.learned_contrary(&a, negated);
                if contrary_deduced || contrary_learned {
                    let direct = self.facts.iter().any(|(f, s)| *f == a && s.negated != negated && s.retracted_by.is_none());
                    let learned_only = contrary_learned;
                    if learned_only {
                        // a learned conclusion meets teaching: a counterexample, no question
                        let n = learn::counterexample(self, &a, negated);
                        out.extend(n);
                    } else {
                        self.stats.conflict_questions += 1;
                        let shown = if negated { format!("not {a}") } else { a.to_string() };
                        let q = if direct { format!("conflict: {shown} vs taught contrary. which?") } else { format!("conflict: {shown} vs inherited contrary. exception?") };
                        let ans = self.ask(teacher, &q);
                        if direct {
                            if ans.starts_with("retract") || ans == "yes" || ans == "new" {
                                for (f, s) in self.facts.iter_mut() {
                                    if *f == a && s.negated != negated && s.retracted_by.is_none() {
                                        s.retracted_by = Some(id);
                                    }
                                }
                            } else {
                                return vec![format!("ok. kept the earlier fact.")];
                            }
                        } else if ans == "no" {
                            return vec!["ok. not stored.".into()];
                        }
                        out.push(format!("ok. exception stored for {}.", sym_of(&a.args[0])));
                    }
                }
                let before: BTreeSet<String> = self.visible();
                let conflicts_before: BTreeSet<String> = self.model().facts_of("ask_conflict").iter().map(|g| g.to_string()).collect();
                self.store(a, negated, id, if channel == "fixture" { 1.0 } else { 0.9 });
                let after = self.visible();
                let new: Vec<String> = after.difference(&before).take(3).cloned().collect();
                let shown: Vec<String> = new.iter().map(|n| format!("{n} [{}]", self.premises_of(n).join(", "))).collect();
                if out.is_empty() {
                    out.push(if shown.is_empty() { "ok.".into() } else { format!("ok. derived: {}", shown.join("; ")) });
                }
                // a new diamond: neither kind is more specific, so ask (E3)
                let conflicts_now: Vec<sla_core::term::GroundAtom> = self.model().facts_of("ask_conflict");
                for c in conflicts_now {
                    if conflicts_before.contains(&c.to_string()) || self.facts.iter().any(|(f, s)| s.retracted_by.is_none() && f.args.first() == c.args.first() && f.pred == sym_of(&c.args[1])) {
                        continue;
                    }
                    let atom = GroundAtom::new(&sym_of(&c.args[1]), vec![c.args[0].clone(), c.args[2].clone()]);
                    self.stats.conflict_questions += 1;
                    let ans = self.ask(teacher, &format!("ask: {atom}?"));
                    out.push(format!("conflict: {atom} [diamond]; neither kind is more specific. ask: {atom}?"));
                    self.take_statement_answer(&ans, &atom, id);
                }
                out
            }
        }
    }

    /// Taught leaves of a derived surface line (for "derived: ... [premises]").
    fn premises_of(&mut self, line: &str) -> Vec<String> {
        let (neg, s) = match line.strip_prefix("not ") { Some(r) => (true, r), None => (false, line) };
        let Ok(a) = reason4::parse_atom(s) else { return vec![] };
        let h = holds_atom(&a, neg);
        let m = self.model();
        m.proof(&h).map(|p| p.leaves().iter().filter(|l| l.pred != "inheritable").filter_map(|l| surface(l).map(|(x, n)| if n { format!("not {x}") } else { x.to_string() }).or_else(|| Some(l.to_string()))).collect()).unwrap_or_default()
    }

    /// An answer that is a statement about `atom` is stored as taught (yes/no included).
    pub fn take_statement_answer(&mut self, ans: &str, atom: &GroundAtom, id: usize) {
        let ans = ans.trim().trim_end_matches('.');
        let (neg, body) = match ans.strip_prefix("not ") { Some(r) => (true, r), None => (false, ans) };
        let stated = if ans == "yes" { Some((atom.clone(), false)) } else if ans == "no" { Some((atom.clone(), true)) } else { reason4::parse_atom(body).ok().map(|a| (a, neg)) };
        if let Some((a, n)) = stated {
            if self.type_check(&a).is_ok() {
                let a = self.canon_atom(&a);
                self.store(a, n, id, 0.97);
            }
        }
    }

    fn learned_contrary(&mut self, a: &GroundAtom, negated: bool) -> bool {
        let lp = learn::lholds_atom(a, !negated);
        self.model().contains(&lp)
    }

    /// Derived item knowledge, as surface lines (what "ok. derived:" shows).
    fn visible(&mut self) -> BTreeSet<String> {
        let taught: BTreeSet<String> = self.facts.iter().filter(|(_, s)| s.retracted_by.is_none()).map(|(a, s)| if s.negated { format!("not {a}") } else { a.to_string() }).collect();
        let m = self.model();
        let mut out = BTreeSet::new();
        for p in ["holds", "holdsn", "holds3", "holdsn3"] {
            for g in m.facts_of(p) {
                if !m.is_given(&g) {
                    if let Some((s, neg)) = surface(&g) {
                        let line = if neg { format!("not {s}") } else { s.to_string() };
                        if !taught.contains(&line) {
                            out.insert(line);
                        }
                    }
                }
            }
        }
        out
    }

    /// End of a teaching batch: an induction pass (7.1) with one confirming question per rule.
    pub fn end_batch(&mut self, teacher: &mut dyn Teacher) -> Vec<String> {
        if !self.learning {
            return vec![];
        }
        learn::induce(self, teacher)
    }

    /// Answer a query or request message (benchmark or user). `given:` facts are used for
    /// this answer only and never stored.
    pub fn respond(&mut self, line: &str) -> Answer {
        self.log("user", "query", line);
        let off: Vec<&str> = self.off.iter().map(String::as_str).collect();
        match msg::parse(line) {
            Ok(Some(Msg::Query { goal, given })) => {
                let given: Vec<(GroundAtom, bool)> = given.into_iter().map(|(a, n)| (self.canon_atom(&a), n)).collect();
                let m = reason4::run(&self.engine_facts(&given), &off, &self.learned_rules()).expect("model");
                answer_query(&m, &self.canonical_goal(&goal))
            }
            Ok(Some(Msg::Request { task, given })) => {
                let given: Vec<(GroundAtom, bool)> = given.into_iter().map(|(a, n)| (self.canon_atom(&a), n)).collect();
                let m = reason4::run(&self.engine_facts(&given), &off, &self.learned_rules()).expect("model");
                answer_request(&m, &self.canonical(&task))
            }
            _ => Answer { gaps: vec!["unparsed".into()], ..Default::default() },
        }
    }

    fn canonical_goal(&self, goal: &str) -> String {
        let mut out = String::new();
        let mut tok = String::new();
        for ch in goal.chars() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                tok.push(ch);
            } else {
                out.push_str(&self.canonical(&tok));
                tok.clear();
                out.push(ch);
            }
        }
        out.push_str(&self.canonical(&tok));
        out
    }
}

/// Map a surface goal (possibly with variables) to the internal pattern.
pub fn internal_goal(goal: &str) -> Result<(sla_core::Atom, bool, String), String> {
    let (neg, g) = match goal.trim().strip_prefix("not ") { Some(r) => (true, r), None => (false, goal.trim()) };
    let r = sla_core::parse_rule(&format!("{g}.")).map_err(|e| e.to_string())?;
    let a = r.head;
    let p = a.pred.clone();
    let mut args = a.args.clone();
    let lit = |s: &str| sla_core::term::Term::Val(Value::Sym(s.to_string()));
    let (pred, args) = if reason4::INHERITABLE.contains(&p.as_str()) && args.len() == 2 {
        let o = args.remove(1);
        (if neg { "holdsn" } else { "holds" }.to_string(), vec![args.remove(0), lit(&p), o])
    } else if reason4::INHERITABLE3.contains(&p.as_str()) && args.len() == 3 {
        let (x, y, z) = (args[0].clone(), args[1].clone(), args[2].clone());
        (if neg { "holdsn3" } else { "holds3" }.to_string(), vec![x, lit(&p), y, z])
    } else {
        let mapped = match p.as_str() {
            "is_a" => "isa",
            "effect" => "holds_effect",
            "precondition" => "holds_pre",
            "has_part" => "part",
            "swap" => "stand_in",
            other => other,
        };
        (mapped.to_string(), args)
    };
    Ok((sla_core::Atom::new(&pred, args), neg, p))
}

fn claim_surface(g: &GroundAtom, asked_pred: &str) -> String {
    if g.pred == "lholds" || g.pred == "lholdsn" {
        let h = GroundAtom::new(if g.pred == "lholds" { "holds" } else { "holdsn" }, g.args.clone());
        return claim_surface(&h, asked_pred);
    }
    match surface(g) {
        Some((s, neg)) => if neg { format!("not {s}") } else { s.to_string() },
        None if g.pred == "stand_in" && asked_pred == "swap" => format!("swap({})", g.args.iter().map(sym_of).collect::<Vec<_>>().join(", ")),
        None => g.to_string(),
    }
}

pub fn answer_query(m: &Model, goal: &str) -> Answer {
    let mut ans = Answer::default();
    let Ok((pat, _neg, asked)) = internal_goal(goal) else {
        ans.gaps.push(format!("unparsed {goal}"));
        return ans;
    };
    let mut hits = m.query_atom(&pat);
    if hits.is_empty() && matches!(pat.pred.as_str(), "holds" | "holdsn") {
        // no deduction: a learned conclusion (rank L) is offered as a hypothesis
        let lp = sla_core::Atom::new(if pat.pred == "holds" { "lholds" } else { "lholdsn" }, pat.args.clone());
        hits = m.query_atom(&lp);
    }
    for h in hits {
        let rec = reason4::proof::record(m, &h).unwrap_or(json!({}));
        if rec["kind"] == "given" && !matches!(h.pred.as_str(), "stand_in" | "gap" | "replace_step") {
            // a taught fact is reported as given, not as a derivation
        }
        ans.claims.push((claim_surface(&h, &asked), rec));
    }
    if ans.claims.is_empty() {
        // near misses: open fit questions or blocked conflicts about the goal's constants
        let consts: Vec<String> = pat.args.iter().filter_map(|t| if let sla_core::term::Term::Val(v) = t { Some(sym_of(v)) } else { None }).collect();
        for p in ["ask_fit", "ask_conflict"] {
            for g in m.facts_of(p) {
                if g.args.iter().any(|v| consts.contains(&sym_of(v))) {
                    ans.asks.push(g.to_string());
                }
            }
        }
        ans.gaps.push(format!("gap: {goal}"));
    }
    ans
}

pub fn answer_request(m: &Model, task: &str) -> Answer {
    let mut ans = Answer::default();
    let prefix = format!("{task}_s");
    let of_task = |g: &GroundAtom| g.args.first().map(|v| sym_of(v).starts_with(&prefix)).unwrap_or(false);
    for g in m.facts_of("stand_in").into_iter().filter(of_task) {
        let rec = reason4::proof::record(m, &g).unwrap_or(json!({}));
        ans.claims.push((format!("swap({})", g.args.iter().map(sym_of).collect::<Vec<_>>().join(", ")), rec));
    }
    for g in m.facts_of("replace_step").into_iter().filter(of_task) {
        let rec = reason4::proof::record(m, &g).unwrap_or(json!({}));
        ans.claims.push((g.to_string(), rec));
    }
    for g in m.facts_of("gap").into_iter().filter(of_task) {
        ans.gaps.push(g.to_string());
    }
    for g in m.facts_of("ask_fit").into_iter().filter(of_task) {
        ans.asks.push(g.to_string());
    }
    ans
}
