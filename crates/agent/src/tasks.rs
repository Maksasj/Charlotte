//! Help with tasks (Design 2.0): the acts `request`, `ask_how`, `ask_need`,
//! `ask_duration`, `ask_next`, `done`, `lack`, `inform`, `accept`, `reject`,
//! `feedback`, `teach` and `new_pred`.
//!
//! Milestone 14 gives each act an honest reply from the procedure layer: the
//! steps of a known task, what it needs, how long it takes, the next step,
//! or "teach me" for an unknown task. Milestone 15 replaces this with the
//! dialogue manager (task frames, slots, move rules).

use crate::{Agent, Brain};
use sla_core::{Atom, GroundAtom, Value};
use sla_nlg::Reply;
use sla_reasoner::Model;

/// Words of a symbol (`cherry_tomato` -> "cherry tomato").
pub fn words(s: &str) -> String {
    s.replace('_', " ")
}

fn sym(s: &str) -> Value {
    Value::sym(s)
}

fn g(pred: &str, args: &[&str]) -> GroundAtom {
    GroundAtom::new(pred, args.iter().map(|a| sym(a)).collect())
}

fn values(m: &Model, pred: &str, first: &str, pos: usize) -> Vec<String> {
    let mut out: Vec<String> = m
        .facts_of(pred)
        .into_iter()
        .filter(|a| a.args.first().and_then(|v| v.as_sym()) == Some(first))
        .filter_map(|a| a.args.get(pos).map(|v| v.as_sym().map(String::from).unwrap_or_else(|| v.to_string())))
        .collect();
    out.sort();
    out.dedup();
    out
}

fn list(items: &[String]) -> String {
    match items.len() {
        0 => String::new(),
        1 => items[0].clone(),
        n => format!("{} and {}", items[..n - 1].join(", "), items[n - 1]),
    }
}

fn number(v: &Value) -> String {
    v.to_string()
}

/// "10 to 12 minutes", "1 minute".
fn range(lo: &Value, hi: &Value, unit: &str) -> String {
    let plural = |v: &Value| if v.as_f64() == Some(1.0) { unit.to_string() } else { format!("{unit}s") };
    if lo == hi {
        format!("{} {}", number(lo), plural(lo))
    } else {
        format!("{} to {} {}", number(lo), number(hi), plural(hi))
    }
}

impl Agent {
    /// The task a request names: itself if known, a known task that solves
    /// it (a feasible one first), or one said to be the same.
    pub(crate) fn resolve_task(&self, m: &Model, t: &str) -> Option<String> {
        if m.contains(&g("task", &[t])) {
            return Some(t.to_string());
        }
        let mut solvers: Vec<GroundAtom> = m.facts_of("solves").into_iter().filter(|a| a.args.get(1).and_then(|v| v.as_sym()) == Some(t)).collect();
        solvers.sort_by_key(|a| !m.contains(&g("feasible", &[a.args[0].as_sym().unwrap_or("")])));
        if let Some(a) = solvers.first() {
            return a.args[0].as_sym().map(String::from);
        }
        for a in m.facts_of("same") {
            if a.args.first().and_then(|v| v.as_sym()) == Some(t) {
                if let Some(o) = a.args.get(1).and_then(|v| v.as_sym()) {
                    if m.contains(&g("task", &[o])) {
                        return Some(o.to_string());
                    }
                }
            }
        }
        None
    }

    /// The model with this conversation's situation facts on top of the vault.
    pub(crate) fn session_model(&self, b: &mut Brain) -> std::sync::Arc<Model> {
        let now = self.now();
        if self.session_facts.is_empty() {
            return b.model_at(now);
        }
        b.model_with(now, &self.session_facts)
    }

    fn teach_me(&self, t: &str) -> Reply {
        Reply { text: format!("I don't know how to {} yet. What would a good answer include?", words(t)), intent: "teach_me", claims: Vec::new() }
    }

    /// One step as an instruction: "Boil the pasta (10 to 12 minutes)."
    pub(crate) fn step_text(&self, m: &Model, s: &str) -> (String, Vec<GroundAtom>) {
        let mut claims = Vec::new();
        let action = values(m, "action", s, 1).into_iter().next();
        let object = values(m, "object_of", s, 1).into_iter().next();
        // Count nouns are plural in instructions ("Add the cherry tomatoes").
        let count = |o: &str| !sla_nlg::MASS.contains(&o);
        let mut text = match (&action, &object) {
            (Some(a), Some(o)) => {
                claims.push(g("action", &[s, a]));
                claims.push(g("object_of", &[s, o]));
                let mut c = a.chars();
                let a = c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default();
                let thing = if count(o) { sla_nlg::plural(&words(o)) } else { words(o) };
                format!("{a} the {thing}")
            }
            (Some(a), None) => {
                claims.push(g("action", &[s, a]));
                let mut c = a.chars();
                c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default()
            }
            _ => words(s),
        };
        let uses = values(m, "uses", s, 1);
        if !uses.is_empty() {
            for u in &uses {
                claims.push(g("uses", &[s, u]));
            }
            text.push_str(&format!(" with {}", list(&uses.iter().map(|u| words(u)).collect::<Vec<_>>())));
        }
        if let Some(u) = values(m, "until", s, 1).into_iter().next() {
            claims.push(g("until", &[s, &u]));
            let they = object.as_deref().map_or(false, count);
            text.push_str(&format!(" until {} {}", if they { "they are" } else { "it is" }, words(&u)));
        }
        // A step that runs during another: "While you boil the pasta, fry the garlic."
        if let Some(r) = values(m, "overlaps", s, 1).into_iter().next() {
            claims.push(g("overlaps", &[s, &r]));
            let ra = values(m, "action", &r, 1).into_iter().next();
            let ro = values(m, "object_of", &r, 1).into_iter().next();
            if let Some(ra) = ra {
                let obj = ro.map(|o| format!(" the {}", if count(&o) { sla_nlg::plural(&words(&o)) } else { words(&o) })).unwrap_or_default();
                let mut c = text.chars();
                let lower = c.next().map(|f| f.to_lowercase().collect::<String>() + c.as_str()).unwrap_or_default();
                text = format!("While you {ra}{obj}, {lower}");
            }
        }
        if let Some(a) = m.facts_of("takes_time").into_iter().find(|a| a.args.first().and_then(|v| v.as_sym()) == Some(s)) {
            if let Some(unit) = a.args[3].as_sym() {
                text.push_str(&format!(" ({})", range(&a.args[1], &a.args[2], unit)));
            }
            claims.push(a);
        }
        (format!("{text}."), claims)
    }

    /// Steps of a task in an order that respects `before` (ties by name).
    fn ordered_steps(&self, m: &Model, task: &str) -> Vec<String> {
        let mut steps = values(m, "step", task, 1);
        let before = |a: &str, b: &str| m.contains(&g("before", &[a, b]));
        // Insertion sort by the partial order.
        let mut out: Vec<String> = Vec::new();
        steps.sort();
        while !steps.is_empty() {
            let i = steps.iter().position(|s| !steps.iter().any(|o| o != s && before(o, s))).unwrap_or(0);
            out.push(steps.remove(i));
        }
        out
    }

    pub(crate) fn task_help(&mut self, b: &mut Brain, t: &str) -> Reply {
        let m = self.session_model(b);
        let Some(task) = self.resolve_task(&m, t) else { return self.teach_me(t) };
        self.active_task = Some(task.clone());
        let steps = self.ordered_steps(&m, &task);
        if steps.is_empty() {
            return self.teach_me(t);
        }
        let mut claims = vec![g("task", &[&task])];
        let mut lines = Vec::new();
        let missing = values(&m, "missing", &task, 1);
        let lead = if task == t { format!("Here is how to {}:", words(&task)) } else { format!("You could make {}:", words(&task)) };
        for (i, s) in steps.iter().enumerate() {
            let (text, c) = self.step_text(&m, s);
            claims.extend(c);
            lines.push(if steps.len() > 3 { format!("{}. {text}", i + 1) } else { text });
        }
        let mut text = format!("{lead} {}", lines.join(if steps.len() > 3 { "\n" } else { " " }));
        if steps.len() > 3 {
            text = format!("{lead}\n{}", lines.join("\n"));
        }
        // A stated duration wins over the sum of the steps.
        let stated = m.facts_of("takes_time").into_iter().find(|a| a.args.first().and_then(|v| v.as_sym()) == Some(task.as_str()));
        if let Some(a) = stated {
            let unit = a.args[3].as_sym().unwrap_or("minute").to_string();
            text.push_str(&format!(" It takes {} in all.", range(&a.args[1], &a.args[2], &unit)));
            claims.push(a);
        } else if let Some(a) = m.facts_of("total_minutes").into_iter().find(|a| a.args.first().and_then(|v| v.as_sym()) == Some(task.as_str())) {
            text.push_str(&format!(" It takes about {} in all.", range(&a.args[1], &a.args[2], "minute")));
            claims.push(a);
        }
        if !missing.is_empty() && !self.session_facts.is_empty() {
            text.push_str(&format!(" You still need {}.", list(&missing.iter().map(|x| words(x)).collect::<Vec<_>>())));
            claims.extend(missing.iter().map(|x| g("missing", &[&task, x])));
        }
        self.last_step = steps.first().cloned();
        Reply { text, intent: "answer", claims }
    }

    pub(crate) fn task_needs(&mut self, b: &mut Brain, t: &str) -> Reply {
        let m = self.session_model(b);
        let Some(task) = self.resolve_task(&m, t) else { return self.teach_me(t) };
        let needs = values(&m, "needs_all", &task, 1);
        if needs.is_empty() {
            return self.teach_me(t);
        }
        let claims = needs.iter().map(|x| g("needs_all", &[&task, x])).collect();
        Reply { text: format!("For {} you need {}.", words(&task), list(&needs.iter().map(|x| words(x)).collect::<Vec<_>>())), intent: "answer", claims }
    }

    pub(crate) fn task_duration(&mut self, b: &mut Brain, t: &str) -> Reply {
        let m = self.session_model(b);
        let task = self.resolve_task(&m, t).unwrap_or_else(|| t.to_string());
        let stated = m.facts_of("takes_time").into_iter().find(|a| a.args.first().and_then(|v| v.as_sym()) == Some(task.as_str()));
        if let Some(a) = stated {
            let unit = a.args[3].as_sym().unwrap_or("minute").to_string();
            return Reply { text: format!("{} takes {}.", capitalize(&words(&task)), range(&a.args[1], &a.args[2], &unit)), intent: "answer", claims: vec![a] };
        }
        if let Some(a) = m.facts_of("total_minutes").into_iter().find(|a| a.args.first().and_then(|v| v.as_sym()) == Some(task.as_str())) {
            return Reply { text: format!("{} takes about {}.", capitalize(&words(&task)), range(&a.args[1], &a.args[2], "minute")), intent: "answer", claims: vec![a] };
        }
        Reply { text: format!("I don't know how long {} takes yet. How long does it usually take?", words(t)), intent: "teach_me", claims: Vec::new() }
    }

    pub(crate) fn task_next(&mut self, b: &mut Brain) -> Reply {
        let Some(task) = self.active_task.clone() else {
            return Reply { text: "Next in what? Tell me what you are working on.".into(), intent: "clarify", claims: Vec::new() };
        };
        let m = self.session_model(b);
        let next: Vec<String> = self.ordered_steps(&m, &task).into_iter().filter(|s| m.contains(&g("next_step", &[&task, s]))).collect();
        match next.first() {
            None => Reply { text: format!("That was the last step. {} is done!", capitalize(&words(&task))), intent: "answer", claims: Vec::new() },
            Some(s) => {
                let (text, mut claims) = self.step_text(&m, s);
                claims.push(g("next_step", &[&task, s]));
                self.last_step = Some(s.clone());
                Reply { text: format!("Next: {text}"), intent: "answer", claims }
            }
        }
    }




    pub(crate) fn feedback(&mut self, positive: bool) -> Reply {
        if positive {
            Reply { text: "Good, glad that helps.".into(), intent: "social", claims: Vec::new() }
        } else {
            Reply { text: "Sorry about that. What should I have said?".into(), intent: "clarify", claims: Vec::new() }
        }
    }

    /// A relation no registered predicate fits goes to the review queue.
    pub(crate) fn new_pred(&mut self, b: &mut Brain, atom: Atom, definition: String) -> Reply {
        let now = self.now();
        let line = format!("- {now} | new_pred | {atom} | {definition} | from {}\n", self.source);
        let dir = b.kb.vault.root.join("queue");
        if std::fs::create_dir_all(&dir).is_ok() {
            use std::io::Write;
            if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(dir.join("predicates.md")) {
                let _ = f.write_all(line.as_bytes());
            }
        }
        b.kb.log_change(now, "queue", &format!("new_pred {atom}"), &definition);
        let name = atom.pred.replace('_', " ");
        Reply { text: format!("I don't know the relation \"{name}\" yet; I'll ask a teacher about it."), intent: "teach_me", claims: Vec::new() }
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default()
}
