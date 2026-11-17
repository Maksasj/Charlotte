//! The dialogue manager (Design 2.0, milestone 15).
//!
//! It sits between understanding and reasoning. Each conversation keeps its
//! own memory (never written to the vault): the open task frame, the slots
//! filled, the procedure under way and its step, the options already offered,
//! the speaker's situation. Each turn:
//!
//! 1. the message's dialogue acts change the session (`inform` adds situation
//!    facts, `done` marks a step, a `request` opens a frame from `vault/tasks/`
//!    or starts a known procedure);
//! 2. the facts of the moment (`heard(..)`, `open_frame(..)`, `filled(..)`,
//!    `active(..)`, `chosen(..)`) are reasoned over together with the vault and
//!    the policy rules (`core/policy.md`);
//! 3. the move with the lowest priority is made, and its proof is kept in the
//!    turn trace.

use crate::{Agent, Brain};
use sla_core::{GroundAtom, Value};
use sla_nlg::Reply;
use sla_parser::Intent;
use sla_reasoner::Model;

fn g(pred: &str, args: &[&str]) -> GroundAtom {
    GroundAtom::new(pred, args.iter().map(|a| Value::sym(*a)).collect())
}

fn words(s: &str) -> String {
    s.replace('_', " ")
}

fn first_arg<'a>(a: &'a GroundAtom) -> Option<&'a str> {
    a.args.first().and_then(|v| v.as_sym())
}

/// Acts the dialogue manager handles (the rest go to the v1 handlers).
pub fn is_dialogue_act(i: &Intent) -> bool {
    matches!(
        i,
        Intent::Request(_) | Intent::AskHow(_) | Intent::AskNeed(_) | Intent::AskDuration(_) | Intent::AskNext | Intent::Done(_) | Intent::Lack(_) | Intent::Inform(_) | Intent::Accept | Intent::Reject
    )
}

impl Agent {
    /// A task schema for `t`: itself, or one it is the same as.
    fn schema_for(&self, m: &Model, t: &str) -> Option<String> {
        let has_schema = |x: &str| m.facts_of("task_slot").iter().any(|a| first_arg(a) == Some(x));
        if has_schema(t) {
            return Some(t.to_string());
        }
        m.facts_of("same").into_iter().filter(|a| first_arg(a) == Some(t)).filter_map(|a| a.args.get(1).and_then(|v| v.as_sym()).map(String::from)).find(|o| has_schema(o))
    }

    /// The facts of the moment for the policy.
    fn moment(&self, m: &Model, heard: &[&str], requested: &Option<String>) -> Vec<GroundAtom> {
        let mut out: Vec<GroundAtom> = heard.iter().map(|h| g("heard", &[h])).collect();
        if let Some(r) = requested {
            out.push(g("requested", &[r]));
        }
        if let Some(f) = &self.frame {
            out.push(g("open_frame", &[f]));
            // A slot is filled when the conversation has a fact of the predicate that fills it.
            for a in m.facts_of("slot_fills").into_iter().filter(|a| first_arg(a) == Some(f.as_str())) {
                if let (Some(slot), Some(pred)) = (a.args.get(1).and_then(|v| v.as_sym()), a.args.get(2).and_then(|v| v.as_sym())) {
                    if self.session_facts.iter().any(|s| s.pred == pred) {
                        out.push(g("filled", &[f, slot]));
                    }
                }
            }
        }
        if let Some(t) = &self.active_task {
            out.push(g("active", &[t]));
        }
        for c in &self.chosen {
            out.push(g("chosen", &[c]));
        }
        out
    }

    /// Handle all dialogue acts of one message together; one main move.
    pub(crate) fn dialogue(&mut self, b: &mut Brain, acts: Vec<Intent>) -> Reply {
        self.dialogue_ran = true;
        let now = self.now();
        let base = b.model_at(now);
        let mut heard: Vec<&str> = Vec::new();
        let mut requested: Option<String> = None;
        let mut direct: Option<Reply> = None;
        let mut informed = false;
        for a in &acts {
            heard.push(a.name());
            match a {
                Intent::Inform(atoms) => {
                    informed = true;
                    for x in atoms {
                        if let Some(gr) = x.to_ground() {
                            if !self.session_facts.contains(&gr) {
                                self.session_facts.push(gr);
                            }
                        }
                    }
                }
                Intent::Lack(items) => {
                    for i in items {
                        self.session_facts.retain(|f| !(f.pred == "available" && first_arg(f) == Some(i.as_str())));
                        let neg = g("-available", &[i]);
                        if !self.session_facts.contains(&neg) {
                            self.session_facts.push(neg);
                        }
                    }
                }
                Intent::Done(step) => {
                    if let Some(s) = step.clone().or_else(|| self.last_step.clone()) {
                        let d = g("done", &[&s]);
                        if !self.session_facts.contains(&d) {
                            self.session_facts.push(d);
                        }
                    }
                }
                Intent::Request(t) | Intent::AskHow(t) => match self.schema_for(&base, t) {
                    Some(schema) => {
                        if self.frame.as_deref() != Some(schema.as_str()) {
                            self.frame = Some(schema.clone());
                            self.chosen.clear();
                        }
                        requested = Some(schema);
                    }
                    None => match self.resolve_task(&base, t) {
                        Some(task) => requested = Some(task),
                        None => requested = Some(t.clone()),
                    },
                },
                Intent::AskNeed(t) => direct = Some(self.task_needs(b, t)),
                Intent::AskDuration(t) => direct = Some(self.task_duration(b, t)),
                Intent::Reject => {}
                _ => {}
            }
        }
        if let Some(r) = direct {
            self.trace_move("answer");
            return r;
        }
        // The policy decides.
        let mut facts = self.session_facts.clone();
        facts.extend(self.moment(&base, &heard, &requested));
        let m = b.model_with(now, &facts);
        let mut moves: Vec<(i64, String)> = m
            .facts_of("dm_move")
            .into_iter()
            .filter_map(|a| Some((match a.args.get(1)? { Value::Int(p) => *p, _ => 99 }, a.args.first()?.as_sym()?.to_string())))
            .collect();
        moves.sort();
        let Some((_, mv)) = moves.first().cloned() else {
            // No move: the turn only told the situation.
            if informed {
                self.trace_move("acknowledge");
                return Reply { text: "Okay.".into(), intent: "acknowledge", claims: Vec::new() };
            }
            if heard.contains(&"lack") {
                return Reply { text: "Okay, noted.".into(), intent: "acknowledge", claims: Vec::new() };
            }
            if heard.contains(&"accept") {
                return Reply { text: "Great!".into(), intent: "social", claims: Vec::new() };
            }
            if heard.contains(&"reject") {
                return Reply { text: "Okay. What would you like instead?".into(), intent: "clarify", claims: Vec::new() };
            }
            return Reply { text: "Okay.".into(), intent: "acknowledge", claims: Vec::new() };
        };
        // Keep why this move was made: its proof from the policy rules.
        let prio = moves[0].0;
        if let Some(p) = m.proof(&GroundAtom::new("dm_move", vec![Value::sym(mv.as_str()), Value::Int(prio)])) {
            if let Some(t) = self.cur.as_mut() {
                t.move_proof = Some(p.render());
            }
        }
        self.trace_move(&mv);
        let frame = self.frame.clone().unwrap_or_default();
        match mv.as_str() {
            "ask_slots" => {
                let text = slot_questions(&m, &frame);
                let lead = if heard.contains(&"request") || heard.contains(&"ask_how") { "Sure! " } else { "" };
                Reply { text: format!("{lead}{text}?"), intent: "clarify", claims: Vec::new() }
            }
            "recommend" | "other_option" => {
                let mut opts: Vec<String> = m.facts_of("good_option").into_iter().filter(|a| first_arg(a) == Some(frame.as_str())).filter_map(|a| a.args.get(1).and_then(|v| v.as_sym()).map(String::from)).filter(|t| !self.chosen.contains(t)).collect();
                // Rank: more successes first, then shorter, then by name.
                let successes = |t: &str| b.kb.vault.procedures.get(t).map_or(0, |p| p.meta().successes);
                let hi = |t: &str| m.facts_of("takes_time").into_iter().find(|a| first_arg(a) == Some(t)).and_then(|a| a.args.get(2).and_then(|v| v.as_f64())).unwrap_or(f64::MAX);
                opts.sort_by(|x, y| successes(y).cmp(&successes(x)).then(hi(x).partial_cmp(&hi(y)).unwrap_or(std::cmp::Ordering::Equal)).then(x.cmp(y)));
                match opts.first().cloned() {
                    Some(t) => {
                        self.chosen.push(t.clone());
                        let on_trial = b.kb.vault.procedures.get(&t).map_or(false, |p| p.meta().status == sla_core::RuleStatus::Hypothesis);
                        let mut r = self.task_help(b, &t);
                        let lead = if on_trial { format!("You could make {} (I was taught this once).", words(&t)) } else { format!("You could make {}.", words(&t)) };
                        r.text = r.text.replacen(&format!("Here is how to {}:", self.task_phrase(&t)), &lead, 1);
                        r.intent = "recommend";
                        r
                    }
                    None => Reply { text: format!("That's every idea I have for {}. What would you like instead?", words(&frame)), intent: "clarify", claims: Vec::new() },
                }
            }
            "closest" => {
                // The option missing the fewest things, and what it lacks.
                let mut opts: Vec<(usize, String)> = m
                    .facts_of("candidate")
                    .into_iter()
                    .filter(|a| first_arg(a) == Some(frame.as_str()))
                    .filter_map(|a| a.args.get(1).and_then(|v| v.as_sym()).map(String::from))
                    .map(|t| (m.facts_of("missing").iter().filter(|x| first_arg(x) == Some(t.as_str())).count(), t))
                    .collect();
                opts.sort();
                let Some((_, t)) = opts.first().cloned() else { return Reply { text: "I don't know a dish for that yet. What do you like to cook?".into(), intent: "teach_me", claims: Vec::new() } };
                let missing: Vec<String> = m.facts_of("missing").into_iter().filter(|x| first_arg(x) == Some(t.as_str())).filter_map(|x| x.args.get(1).and_then(|v| v.as_sym()).map(words)).collect();
                self.chosen.push(t.clone());
                let claims = m.facts_of("missing").into_iter().filter(|x| first_arg(x) == Some(t.as_str())).collect();
                let need = if missing.is_empty() { "a bit more time".to_string() } else { list(&missing) };
                Reply { text: format!("The closest I know is {}, but you would need {need}. Shall I tell you how to make it?", words(&t)), intent: "recommend", claims }
            }
            "close" => {
                let closing = m.facts_of("closing").into_iter().find(|a| first_arg(a) == Some(frame.as_str())).and_then(|a| a.args.get(1).and_then(|v| v.as_sym()).map(String::from));
                // The outcome is evidence for the learner: the offered option worked.
                if let Some(t) = self.chosen.last().cloned() {
                    b.kb.log_change(now, "task_outcome", &format!("{frame}: {t}"), "accepted");
                    if let Some(pf) = b.kb.vault.procedures.get_mut(&t) {
                        let mut meta = pf.meta();
                        meta.uses += 1;
                        meta.successes += 1;
                        // A procedure on trial is confirmed by a successful use.
                        let confirmed = meta.status == sla_core::RuleStatus::Hypothesis;
                        if confirmed {
                            meta.status = sla_core::RuleStatus::Confirmed;
                        }
                        pf.set_meta(&meta);
                        b.kb.vault.mark_procedure_dirty(sla_core::ProcedureKind::Procedure, &t);
                        if confirmed {
                            b.kb.log_change(now, "confirm_procedure", &t, "the user accepted it");
                        }
                    }
                }
                self.frame = None;
                Reply { text: closing.unwrap_or_else(|| "Great, enjoy!".into()), intent: "social", claims: Vec::new() }
            }
            "next_step" | "task_finished" => self.task_next(b),
            "steps" => {
                let t = requested.clone().unwrap_or_default();
                self.task_help(b, &t)
            }
            "teach_me" => {
                let t = requested.clone().unwrap_or_default();
                Reply { text: format!("I don't know how to {} yet. What would a good answer include?", words(&t)), intent: "teach_me", claims: Vec::new() }
            }
            "ask_task" => Reply { text: "Next in what? Tell me what you are working on.".into(), intent: "clarify", claims: Vec::new() },
            other => Reply { text: format!("({other})"), intent: "social", claims: Vec::new() },
        }
    }
}

/// The open frame's missing slots as one question (without the final "?").
fn slot_questions(m: &Model, frame: &str) -> String {
    let missing: Vec<String> = m.facts_of("missing_slot").into_iter().filter(|a| first_arg(a) == Some(frame)).filter_map(|a| a.args.get(1).and_then(|v| v.as_sym()).map(String::from)).collect();
    let mut qs: Vec<String> = Vec::new();
    for s in &missing {
        let q = m.facts_of("slot_question").into_iter().find(|a| first_arg(a) == Some(frame) && a.args.get(1).and_then(|v| v.as_sym()) == Some(s.as_str()));
        qs.push(q.and_then(|a| a.args.get(2).and_then(|v| v.as_sym()).map(String::from)).unwrap_or_else(|| format!("what is the {}", words(s))));
    }
    let text = qs.join(", and ");
    match text.get(0..1) {
        Some(c) => c.to_uppercase() + &text[1..],
        None => text,
    }
}

impl Agent {
    /// The resume move: after a side remark, return to the open frame's question.
    pub(crate) fn resume_question(&mut self, b: &mut Brain) -> Option<String> {
        let frame = self.frame.clone()?;
        let mut facts = self.session_facts.clone();
        facts.extend(self.moment(&b.model_at(self.now()), &[], &None));
        let m = b.model_with(self.now(), &facts);
        let q = slot_questions(&m, &frame);
        if q.is_empty() {
            return None;
        }
        Some(format!("Back to your plan: {}{}?", q.get(0..1).map(|c| c.to_lowercase()).unwrap_or_default(), q.get(1..).unwrap_or("")))
    }
}

fn list(items: &[String]) -> String {
    match items.len() {
        0 => String::new(),
        1 => items[0].clone(),
        n => format!("{} and {}", items[..n - 1].join(", "), items[n - 1]),
    }
}
