//! Charlotte 3.0 agent (Design 3.0 section 4): learns procedures from teaching lines, finds the
//! task a request is about, reads the situation, adapts the procedure with a gated arm and
//! answers in relation lines. Statistical parts propose; the validator and vault decide.

pub mod adapt;
pub mod identity;

use kb3::vault::{Case, Provenance, Source};
use kb3::{lines, Fact, Procedure, Vault};
use parse3::{LineKind, Models};
use serde::Serialize;
use std::collections::BTreeSet;
use teach3::Teacher;

pub use adapt::Arm;

#[derive(Debug, Clone, Default, Serialize)]
pub struct IngestStats {
    pub lines: usize,
    pub learned: usize,
    pub fallback: usize,
    pub fallback_failed: usize,
    pub facts_added: usize,
    pub rejected: usize,
    pub situation_fallback: usize,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Reply {
    pub task: Option<String>,
    pub lines: String,
    pub situation: Vec<Fact>,
    pub difference: Vec<Fact>,
    pub gaps: Vec<(String, String, String)>,
    pub asks: Vec<String>,
    /// identity score margin between the best and second-best task
    pub identity_margin: f64,
}

pub struct Agent {
    pub vault: Vault,
    pub models: Models,
    pub teacher: Teacher,
    pub session: String,
    pub turn: u32,
    /// lines whose confidence is below this go to the Haiku fallback
    pub theta: f64,
    /// situation readings below this margin go to the teacher
    pub theta_situation: f64,
    pub use_fallback: bool,
    pub arm: Arm,
    pub stats: IngestStats,
}

impl Agent {
    pub fn new(models: Models, teacher: Teacher, session: &str, theta: f64) -> Agent {
        Agent { vault: Vault::default(), models, teacher, session: session.into(), turn: 0, theta, theta_situation: f64::INFINITY, use_fallback: true, arm: Arm::Function, stats: IngestStats::default() }
    }

    fn prov(&self, source: Source, parser: &str, conf: f64) -> Provenance {
        Provenance::new(source, &self.session, self.turn, parser, conf)
    }

    /// The validator: the last gate. Facts must be in the schema, and step facts must belong
    /// to this task's steps.
    fn validate(&self, task: &str, f: &Fact) -> bool {
        if f.valid().is_err() {
            return false;
        }
        let prefix = format!("{task}_s");
        match f.pred.as_str() {
            "task" => f.arg(0) == task,
            "needs" | "optional_need" | "goal" | "task_alias" => f.arg(0) == task,
            "step" => f.arg(0) == task && f.arg(1).starts_with(&prefix),
            "takes_time" => f.arg(0) == task || f.arg(0).starts_with(&prefix),
            "alias" => true,
            "lacks" | "has" | "wants" | "excludes" | "time_limit" | "count" => false,
            "swap" | "drop" | "add_step" | "change" | "extra_need" | "no_need" | "cannot" => false,
            _ => f.arg(0).starts_with(&prefix),
        }
    }

    /// Learn a task from teaching lines (dialogue teaching, C1). `request` is the user's
    /// message that led to the teaching; it becomes a learned name of the task.
    pub fn teach(&mut self, task: &str, request: Option<&str>, teach_lines: &[String], source: Source) -> IngestStats {
        self.turn += 1;
        let mut st = IngestStats::default();
        self.add(Fact::new("task", &[task]), self.prov(source, "learned", 1.0), &mut st);
        // the needs line first: its items become known words for the step lines
        let needs_line = teach_lines.iter().find(|l| self.models.parse_teach(task, l, &BTreeSet::new()).kind == LineKind::Needs);
        let known = needs_line.map(|l| self.models.known_words(l)).unwrap_or_default();
        let known_items: Vec<String> = known.iter().cloned().collect();
        let parsed: Vec<_> = teach_lines.iter().map(|l| self.models.parse_teach(task, l, &known)).collect();
        let needs_fb: Vec<bool> = parsed.iter().map(|p| self.use_fallback && (p.confidence < self.theta || p.kind == LineKind::Other)).collect();
        // fallback calls for this task's unsure lines run in parallel (each is cached)
        let mut fb: Vec<Option<Result<Vec<Fact>, String>>> = vec![None; teach_lines.len()];
        std::thread::scope(|sc| {
            let hs: Vec<_> = teach_lines.iter().enumerate().filter(|(k, _)| needs_fb[*k]).map(|(k, line)| {
                let t = &self.teacher;
                let items = &known_items;
                (k, sc.spawn(move || t.parse_line(task, line, items)))
            }).collect();
            for (k, h) in hs {
                fb[k] = Some(h.join().unwrap_or_else(|_| Err("panic".into())));
            }
        });
        for (k, line) in teach_lines.iter().enumerate() {
            st.lines += 1;
            let p = parsed[k].clone();
            let mut facts = p.facts.clone();
            let mut parser = "learned";
            if needs_fb[k] {
                match fb[k].take().unwrap_or_else(|| Err("none".into())) {
                    Ok(fs) => {
                        facts = fs;
                        parser = "fallback";
                    }
                    Err(_) => st.fallback_failed += 1,
                }
            }
            if parser == "fallback" { st.fallback += 1 } else { st.learned += 1 }
            let conf = if parser == "fallback" { 1.0 } else { p.confidence };
            if parser == "fallback" {
                facts = facts.into_iter().map(|f| self.canonical(task, f)).collect();
            }
            // a fallback answer may restate the task line
            for f in facts {
                self.add(f, self.prov(source, parser, conf), &mut st);
            }
            // the words naming the task in this line are a learned name of it
            if let Some(name) = identity::task_phrase(&self.models, line) {
                self.add(Fact::new("task_alias", &[task, &format!("\"{name}\"")]), self.prov(source, "learned", 1.0), &mut st);
            }
        }
        if let Some(r) = request {
            // provenance "request": identity learns from these which words are request chatter
            self.add(Fact::new("task_alias", &[task, &format!("\"{}\"", identity::clean(r))]), self.prov(source, "request", 1.0), &mut st);
        }
        self.derive_roles(task, &mut st);
        self.stats.lines += st.lines;
        self.stats.learned += st.learned;
        self.stats.fallback += st.fallback;
        self.stats.fallback_failed += st.fallback_failed;
        self.stats.facts_added += st.facts_added;
        self.stats.rejected += st.rejected;
        st
    }

    fn add(&mut self, f: Fact, p: Provenance, st: &mut IngestStats) {
        let task = match f.pred.as_str() {
            "task" | "needs" | "optional_need" | "goal" | "task_alias" | "step" => f.arg(0).to_string(),
            _ => f.arg(0).rsplit_once("_s").map(|(t, _)| t.to_string()).unwrap_or_else(|| f.arg(0).to_string()),
        };
        if !self.validate(&task, &f) {
            st.rejected += 1;
            return;
        }
        // a step fact implies the step
        if f.pred == "action" {
            let step = Fact::new("step", &[&task, f.arg(0)]);
            let _ = self.vault.add(step, p.clone());
            // one action per step: a newer action replaces the older
            if self.vault.live().any(|g| g.pred == "action" && g.arg(0) == f.arg(0) && g.arg(1) != f.arg(1)) {
                self.vault.retract("action", &[f.arg(0)], &self.session.clone());
            }
        }
        if self.vault.add(f, p) {
            st.facts_added += 1;
        }
    }

    /// Teacher-written symbols in the agent's own vocabulary: words through the learned symbol
    /// forms (plural to singular), and items linked to the task's taught items that share their
    /// head word (`bananas` -> `ripe_banana` when that is what the needs line taught).
    fn canonical(&self, task: &str, mut f: Fact) -> Fact {
        let item_slot = match f.pred.as_str() {
            "needs" | "object_of" | "uses" | "into" | "onto" | "from" => Some(1),
            "role" => Some(1),
            _ => None,
        };
        let Some(k) = item_slot else { return f };
        let toks: Vec<String> = f.args[k].split('_').map(String::from).collect();
        let sym = self.models.forms.symbol(&toks);
        let needs: Vec<String> = self.vault.live().filter(|g| g.pred == "needs" && g.arg(0) == task).map(|g| g.arg(1).to_string()).collect();
        let stem = |w: &str| -> String { w.chars().take(4).collect() };
        let sw: Vec<String> = sym.split('_').filter(|w| w.len() > 2).map(stem).collect();
        let linked = if f.pred == "needs" {
            None
        } else {
            needs.iter().find(|n| n.rsplit('_').next().map(stem).map(|h| sw.contains(&h)).unwrap_or(false)).cloned()
        };
        f.args[k] = linked.unwrap_or(sym);
        f
    }

    /// A used item without a taught function plays the step's action (derived, not taught).
    fn derive_roles(&mut self, task: &str, st: &mut IngestStats) {
        let p = self.vault.procedure(task);
        for s in &p.steps {
            for (i, r) in &s.uses {
                if r.is_none() {
                    let f = Fact::new("role", &[&s.id, i, &s.action]);
                    if self.vault.add(f, self.prov(Source::Derived, "derived", 0.5)) {
                        st.facts_added += 1;
                    }
                }
            }
        }
    }

    /// Answer a request: identity, situation, adaptation, relation lines.
    pub fn answer(&mut self, request: &str, situation_text: Option<&str>) -> Reply {
        self.turn += 1;
        let mut reply = Reply::default();
        let ranked = identity::rank(&self.vault, request);
        let Some((task, best)) = ranked.first().cloned() else {
            reply.asks.push("task".into());
            reply.lines = "ask task".into();
            return reply;
        };
        if best <= 0.0 {
            reply.asks.push("task".into());
            reply.lines = "ask task".into();
            return reply;
        }
        reply.identity_margin = best - ranked.get(1).map(|r| r.1).unwrap_or(0.0);
        let base = self.vault.procedure(&task);
        let mut text = request.to_string();
        if let Some(s) = situation_text {
            text.push(' ');
            text.push_str(s);
        }
        let (mut sit, margin) = self.models.parse_situation(&text);
        if self.use_fallback && margin < self.theta_situation {
            let items: Vec<String> = base.needs.iter().cloned().collect();
            if let Ok(fs) = self.teacher.parse_situation(&text, &items) {
                sit = fs;
                self.stats.situation_fallback += 1;
            }
        }
        let sit = adapt::link_situation(&self.vault, &base, sit);
        let out = adapt::adapt(self.arm, &self.vault, &base, &sit);
        let proc = base.apply(&out.difference);
        reply.lines = lines::print(&proc, &out.gaps, &out.asks, &out.cannot);
        reply.task = Some(task);
        reply.situation = sit;
        reply.difference = out.difference;
        reply.gaps = out.gaps;
        reply.asks = out.asks;
        reply
    }

    /// The teacher showed how the situation changes the task: store it as a case. The new
    /// procedure is learned from the teacher's lines; the difference is computed against the
    /// base.
    pub fn learn_variant(&mut self, task: &str, situation: &[Fact], taught: &Procedure, source: Source) {
        let base = self.vault.procedure(task);
        let diff = adapt::difference(&base, taught);
        self.vault.add_case(Case { task: task.into(), situation: situation.to_vec(), difference: diff, source, session: self.session.clone() });
    }

    /// Parse teaching lines into a procedure without touching the vault (for a taught variant).
    pub fn parse_procedure(&mut self, task: &str, teach_lines: &[String]) -> Procedure {
        let saved = std::mem::take(&mut self.vault);
        let saved_stats = self.stats.clone();
        self.teach(task, None, teach_lines, Source::Teach);
        let p = self.vault.procedure(task);
        self.vault = saved;
        self.stats = saved_stats;
        p
    }

    /// Load seed procedures as facts (the agent's starting knowledge, as in 2.0).
    pub fn seed(&mut self, facts: &[Fact]) {
        for f in facts {
            let _ = self.vault.add(f.clone(), Provenance::new(Source::Seed, "seed", 0, "seed", 1.0));
        }
    }

    pub fn known_procedure(&self, task: &str) -> Procedure {
        self.vault.procedure(task)
    }
}
