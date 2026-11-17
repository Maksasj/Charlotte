//! Milestone 18: curriculum training with the teacher protocol and the four
//! encounters (Design 2.0, "Training and curiosity").
//!
//! A scenario (`data/curriculum/scenarios/<id>.md`) has three request
//! wordings, what the user says when asked, a ground-truth procedure, teach
//! sentences in the protocol's fixed frames, a goal checklist and possibly a
//! variant. The teacher is scripted from the ground truth: its user turns are
//! the scenario's requests and situation lines, its teach blocks are the
//! scenario's teach sentences, sent on a separate teacher channel (their own
//! session, tagged as teacher knowledge). It never asks for formatting or
//! read-backs (protocol rule 4) and answers only from the ground truth (rule 6).
//!
//! Encounters, in order over all scenarios:
//! 1. Teach: the request; on failure, the teach blocks, then the same request
//!    once more in the same words (rule 5): "taught, learned" or "taught, not
//!    learned".
//! 2. Repeat: a fresh session after the other scenarios, new wording, no teaching.
//! 3. Transfer: the variant, no teaching.
//! 4. Retention: encounter 2 again, other wording, after everything else.
//!
//! A scenario that fails encounter 2 goes into a spaced queue: it is taught
//! again after the other scenarios and repeated once more (counted apart).

use crate::outcome::{Label, Labeller, TurnToLabel};
use serde::Serialize;
use sla_agent::{Agent, SharedBrain};
use sla_core::{parse_ground, GroundAtom};
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Default)]
pub struct Variant {
    pub change: String,
    pub request: String,
    pub situation: String,
    pub expect: String,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct Scenario {
    pub id: String,
    pub stage: String,
    pub task: String,
    pub requests: Vec<String>,
    pub situation: Vec<String>,
    #[serde(skip)]
    pub truth: Vec<GroundAtom>,
    pub teach: Vec<String>,
    pub checklist: Vec<String>,
    pub variant: Option<Variant>,
}

/// Parse one scenario file.
pub fn parse_scenario(src: &str) -> Scenario {
    let mut s = Scenario::default();
    let mut section = String::new();
    let mut in_front = false;
    let mut in_code = false;
    let mut v = Variant::default();
    for (i, line) in src.lines().enumerate() {
        let t = line.trim();
        if i == 0 && t == "---" {
            in_front = true;
            continue;
        }
        if in_front {
            if t == "---" {
                in_front = false;
            } else if let Some((k, val)) = t.split_once(':') {
                let val = val.trim().to_string();
                match k.trim() {
                    "id" => s.id = val,
                    "stage" => s.stage = val,
                    "task" => s.task = val,
                    _ => {}
                }
            }
            continue;
        }
        if t.starts_with("```") {
            in_code = !in_code;
            continue;
        }
        if let Some(h) = t.strip_prefix("## ") {
            section = h.trim().to_lowercase();
            continue;
        }
        if in_code {
            if section == "ground truth" {
                if let Ok(g) = parse_ground(t.trim_end_matches('.')) {
                    s.truth.push(g);
                }
            }
            continue;
        }
        let Some(item) = t.strip_prefix("- ") else { continue };
        let item = item.trim().to_string();
        match section.as_str() {
            "requests" => s.requests.push(item),
            "situation" => s.situation.push(item.trim_matches('"').to_string()),
            "teach" => s.teach.push(item),
            "checklist" => s.checklist.push(item),
            "variant" => {
                if let Some((k, val)) = item.split_once(':') {
                    let val = val.trim().trim_matches('"').to_string();
                    match k.trim() {
                        "change" => v.change = val,
                        "request" => v.request = val,
                        "situation" => v.situation = val,
                        "expect" => v.expect = val,
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    if !v.request.is_empty() {
        s.variant = Some(v);
    }
    s
}

pub fn load_scenarios(dir: &Path) -> Vec<Scenario> {
    let mut files: Vec<PathBuf> = std::fs::read_dir(dir).map(|d| d.flatten().map(|e| e.path()).filter(|p| p.extension().map_or(false, |x| x == "md")).collect()).unwrap_or_default();
    files.sort();
    files.iter().filter_map(|f| std::fs::read_to_string(f).ok()).map(|s| parse_scenario(&s)).filter(|s| !s.id.is_empty() && !s.requests.is_empty()).collect()
}

/// Seed procedures enter as hypotheses (they are reviewed by a second teacher).
pub fn install_seeds(brain: &SharedBrain, dir: &Path) -> usize {
    let mut b = sla_agent::brain::lock(brain);
    let mut n = 0;
    for e in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let p = e.path();
        if p.extension().map_or(true, |x| x != "md") {
            continue;
        }
        let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string();
        let Ok(src) = std::fs::read_to_string(&p) else { continue };
        let pf = sla_core::ProcedureFile::parse(&src, &stem);
        b.kb.vault.put_procedure(sla_core::ProcedureKind::Procedure, pf);
        n += 1;
    }
    let _ = b.save();
    n
}

/// One user turn and the agent's reply.
#[derive(Debug, Clone, Serialize)]
pub struct Turn {
    pub user: String,
    pub agent: String,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EncounterResult {
    pub scenario: String,
    /// "teach", "teach_retry", "repeat", "transfer", "retention", "spaced_repeat".
    pub kind: String,
    pub passed: bool,
    pub turns: Vec<Turn>,
    pub teach_blocks: usize,
    pub judge_note: String,
}

/// Judges an encounter against the scenario's goal checklist.
pub trait CheckJudge: Send + Sync {
    fn passes(&self, s: &Scenario, variant: bool, turns: &[Turn]) -> (bool, String);
}

/// The checklist judged by a model.
pub struct ModelJudge {
    pub model: String,
}

impl CheckJudge for ModelJudge {
    fn passes(&self, s: &Scenario, variant: bool, turns: &[Turn]) -> (bool, String) {
        let transcript: String = turns.iter().map(|t| format!("User: {}\nAgent: {}\n", t.user, t.agent)).collect();
        let mut checks = s.checklist.clone();
        if let (true, Some(v)) = (variant, &s.variant) {
            checks.push(format!("takes the change into account ({}): {}", v.change, v.expect));
        }
        let list: String = checks.iter().enumerate().map(|(i, c)| format!("{}. {c}\n", i + 1)).collect();
        let truth: String = s.truth.iter().map(|g| format!("{g}\n")).collect();
        let prompt = format!(
            "A user asked a small assistant for help. Check the assistant's replies against a goal checklist.\n\n\
             Ground truth the assistant was taught (Datalog, for reference):\n{truth}\nConversation:\n{transcript}\n\
             Checklist:\n{list}\nA check passes when the assistant's replies do it correctly; wording and small omissions of detail do not matter, \
             but wrong facts do. Reply with one line per check, `N: yes` or `N: no`, then a last line `VERDICT: pass` if every check passes, else `VERDICT: fail`."
        );
        match crate::claude_as("curriculum_judge", &prompt, &self.model) {
            Ok(out) => (out.lines().any(|l| l.trim().eq_ignore_ascii_case("VERDICT: pass")), out.trim().replace('\n', " | ")),
            Err(e) => (false, format!("judge failed: {e}")),
        }
    }
}

pub struct CurriculumConfig {
    /// Makes a user-side agent session on the shared brain.
    pub user_agent: Box<dyn Fn(&SharedBrain, &str) -> Agent + Send + Sync>,
    /// Makes the teacher channel's session.
    pub teacher_agent: Box<dyn Fn(&SharedBrain) -> Agent + Send + Sync>,
    pub judge: Box<dyn CheckJudge>,
    /// Labels every user turn (helpful, clarify, not understood ...); `None` skips labels.
    pub labeller: Option<Labeller>,
    pub max_turns: usize,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct CurriculumReport {
    pub scenarios: usize,
    pub seeds: usize,
    pub encounters: Vec<EncounterResult>,
    pub teach_first_try: usize,
    pub taught_learned: usize,
    pub taught_not_learned: usize,
    pub repeat_passed: usize,
    pub repeat_total: usize,
    pub transfer_passed: usize,
    pub transfer_total: usize,
    pub retention_passed: usize,
    pub retention_total: usize,
    pub spaced_passed: usize,
    pub spaced_total: usize,
    pub user_turns: usize,
    pub not_understood: usize,
    pub helpful_or_clarify: usize,
    pub labelled: usize,
    pub stored_facts: usize,
    pub contradicting: Vec<String>,
    pub teacher_meta_requests: usize,
    pub hygiene: Option<sla_agent::hygiene::HygieneReport>,
}

const NOT_UNDERSTOOD: &str = "Sorry, I didn't understand that.";
const META: &[&str] = &["say it back", "repeat after", "format", "as a list", "bullet", "numbered", "sorry", "apolog", "as an ai", "assistant"];

/// The user's side of one encounter: the request, then what they have when asked.
fn converse(a: &mut Agent, first: &str, situation: &[String], max_turns: usize) -> Vec<Turn> {
    let mut turns = Vec::new();
    let mut user = first.to_string();
    let mut sit = situation.iter();
    for _ in 0..max_turns {
        let reply = a.respond(&user).text;
        let asked = reply.trim_end().ends_with('?');
        turns.push(Turn { user: user.clone(), agent: reply, label: None });
        if !asked {
            break;
        }
        match sit.next() {
            Some(s) => user = s.clone(),
            None => break,
        }
    }
    // A real user closes the conversation.
    let reply = a.respond("Thanks!").text;
    turns.push(Turn { user: "Thanks!".into(), agent: reply, label: None });
    turns
}

impl CurriculumConfig {
    fn encounter(&self, brain: &SharedBrain, s: &Scenario, kind: &str, request: &str, situation: &[String], variant: bool, n: usize) -> EncounterResult {
        let mut a = (self.user_agent)(brain, &format!("user{n}"));
        let turns = converse(&mut a, request, situation, self.max_turns);
        let (passed, note) = self.judge.passes(s, variant, &turns);
        EncounterResult { scenario: s.id.clone(), kind: kind.into(), passed, turns, teach_blocks: 0, judge_note: note }
    }

    fn teach(&self, brain: &SharedBrain, s: &Scenario) -> usize {
        let mut t = (self.teacher_agent)(brain);
        for line in &s.teach {
            t.respond(line);
        }
        s.teach.len()
    }
}

/// Run the curriculum over `scenarios` on `brain`.
pub fn run(cfg: &mut CurriculumConfig, brain: &SharedBrain, scenarios: &[Scenario], seeds: usize) -> CurriculumReport {
    let mut rep = CurriculumReport { scenarios: scenarios.len(), seeds, ..Default::default() };
    let mut n = 0;
    let mut next = || {
        n += 1;
        n
    };
    // 1. Teach.
    for s in scenarios {
        let e = cfg.encounter(brain, s, "teach", &s.requests[0], &s.situation, false, next());
        let first = e.passed;
        rep.encounters.push(e);
        if first {
            rep.teach_first_try += 1;
            continue;
        }
        let blocks = cfg.teach(brain, s);
        rep.teacher_meta_requests += s.teach.iter().filter(|l| META.iter().any(|m| l.to_lowercase().contains(m))).count();
        let mut e = cfg.encounter(brain, s, "teach_retry", &s.requests[0], &s.situation, false, next());
        e.teach_blocks = blocks;
        if e.passed {
            rep.taught_learned += 1;
        } else {
            rep.taught_not_learned += 1;
        }
        rep.encounters.push(e);
    }
    // 2. Repeat, after all the others, new wording, no teaching.
    let mut queue = Vec::new();
    for s in scenarios {
        let w = s.requests.get(1).unwrap_or(&s.requests[0]);
        let e = cfg.encounter(brain, s, "repeat", w, &s.situation, false, next());
        rep.repeat_total += 1;
        if e.passed {
            rep.repeat_passed += 1;
        } else {
            queue.push(s);
        }
        rep.encounters.push(e);
    }
    // 3. Transfer.
    for s in scenarios {
        let Some(v) = &s.variant else { continue };
        let e = cfg.encounter(brain, s, "transfer", &v.request, &[v.situation.clone()], true, next());
        rep.transfer_total += 1;
        rep.transfer_passed += e.passed as usize;
        rep.encounters.push(e);
    }
    // The spaced queue: taught again, repeated once more.
    for s in &queue {
        cfg.teach(brain, s);
    }
    for s in &queue {
        let w = s.requests.get(1).unwrap_or(&s.requests[0]);
        let e = cfg.encounter(brain, s, "spaced_repeat", w, &s.situation, false, next());
        rep.spaced_total += 1;
        rep.spaced_passed += e.passed as usize;
        rep.encounters.push(e);
    }
    // 4. Retention.
    for s in scenarios {
        let w = s.requests.get(2).or(s.requests.get(1)).unwrap_or(&s.requests[0]);
        let e = cfg.encounter(brain, s, "retention", w, &s.situation, false, next());
        rep.retention_total += 1;
        rep.retention_passed += e.passed as usize;
        rep.encounters.push(e);
    }
    // Turn labels.
    let scen_of = |id: &str| scenarios.iter().find(|s| s.id == id).map(|s| s.requests[0].clone()).unwrap_or_default();
    let mut to_label = Vec::new();
    for (ei, e) in rep.encounters.iter().enumerate() {
        let mut hist = Vec::new();
        for (ti, t) in e.turns.iter().enumerate() {
            rep.user_turns += 1;
            rep.not_understood += (t.agent.trim() == NOT_UNDERSTOOD) as usize;
            to_label.push((ei, ti, TurnToLabel { id: format!("{}-{}-{ei}-{ti}", e.scenario, e.kind), scenario: scen_of(&e.scenario), history: hist.clone(), user: t.user.clone(), reply: t.agent.clone() }));
            hist.push((t.user.clone(), t.agent.clone()));
        }
    }
    if let Some(l) = cfg.labeller.as_mut() {
        let turns: Vec<TurnToLabel> = to_label.iter().map(|x| x.2.clone()).collect();
        l.prefetch(&turns, 8);
        for (ei, ti, t) in &to_label {
            let (label, src) = l.label(t);
            if src != crate::outcome::LabelSource::None {
                rep.labelled += 1;
                if matches!(label, Label::Helpful | Label::Clarify) {
                    rep.helpful_or_clarify += 1;
                }
                rep.encounters[*ei].turns[*ti].label = Some(format!("{label:?}").to_lowercase());
            }
        }
    }
    // Stored procedure facts against the ground truth.
    let b = sla_agent::brain::lock(brain);
    for s in scenarios {
        let Some(pf) = b.kb.vault.procedures.get(&sla_core::vault::file_stem(&s.task)) else { continue };
        let stored: Vec<GroundAtom> = pf.facts().iter().map(|f| f.atom.clone()).collect();
        rep.stored_facts += stored.len();
        rep.contradicting.extend(contradictions(&stored, &s.truth, &s.task).into_iter().map(|c| format!("{}: {c}", s.id)));
    }
    drop(b);
    let mut b = sla_agent::brain::lock(brain);
    rep.hygiene = Some(sla_agent::hygiene::run(&mut b, sla_core::Timestamp::now()));
    rep
}

/// Stored facts about a task that the ground truth contradicts: a need it does
/// not have, a different total time, a step whose action differs at the same
/// position.
pub fn contradictions(stored: &[GroundAtom], truth: &[GroundAtom], task: &str) -> Vec<String> {
    let sym = |g: &GroundAtom, i: usize| g.args.get(i).and_then(|v| v.as_sym()).unwrap_or("").to_string();
    let mut out = Vec::new();
    let truth_items: Vec<String> = truth.iter().filter(|g| matches!(g.pred.as_str(), "needs" | "uses" | "object_of")).map(|g| sym(g, 1)).collect();
    for g in stored.iter().filter(|g| g.pred == "needs" && sym(g, 0) == task) {
        if !truth_items.contains(&sym(g, 1)) {
            out.push(g.to_string());
        }
    }
    let tt = |set: &[GroundAtom]| set.iter().find(|g| g.pred == "takes_time" && sym(g, 0) == task).cloned();
    if let (Some(a), Some(b)) = (tt(stored), tt(truth)) {
        if a.args != b.args {
            out.push(a.to_string());
        }
    }
    // Steps by position (the order they were listed in).
    let steps = |set: &[GroundAtom]| -> Vec<String> { set.iter().filter(|g| g.pred == "step" && sym(g, 0) == task).map(|g| sym(g, 1)).collect() };
    let action = |set: &[GroundAtom], st: &str| set.iter().find(|g| g.pred == "action" && sym(g, 0) == st).map(|g| sym(g, 1));
    for (a, b) in steps(stored).iter().zip(steps(truth).iter()) {
        if let (Some(x), Some(y)) = (action(stored, a), action(truth, b)) {
            if x != y {
                out.push(format!("action({a}, {x})"));
            }
        }
    }
    out
}

pub fn markdown(r: &CurriculumReport) -> String {
    let pct = |a: usize, b: usize| if b == 0 { 0.0 } else { a as f64 * 100.0 / b as f64 };
    let mut s = format!(
        "# Curriculum run\n\n{} scenarios, {} seed procedures.\n\n\
         | Measure | Result | Target |\n|---|---|---|\n\
         | Encounter 1 solved before teaching | {} of {} | |\n\
         | Taught, learned / taught, not learned | {} / {} | |\n\
         | **Encounter 2 solved without teaching** | **{} of {} ({:.1}%)** | 70% |\n\
         | Encounter 3, transfer | {} of {} ({:.1}%) | 40% |\n\
         | Encounter 4, retention | {} of {} ({:.1}%) | |\n\
         | Spaced queue, repeated after reteaching | {} of {} | |\n\
         | Not understood | {} of {} turns ({:.1}%) | 10% or less |\n\
         | Helpful + clarify | {} of {} labelled turns ({:.1}%) | 60% |\n\
         | Stored facts contradicting ground truth | {} of {} ({:.1}%) | 1% or less |\n\
         | Teacher meta requests | {} | 0 |\n",
        r.scenarios,
        r.seeds,
        r.teach_first_try,
        r.scenarios,
        r.taught_learned,
        r.taught_not_learned,
        r.repeat_passed,
        r.repeat_total,
        pct(r.repeat_passed, r.repeat_total),
        r.transfer_passed,
        r.transfer_total,
        pct(r.transfer_passed, r.transfer_total),
        r.retention_passed,
        r.retention_total,
        pct(r.retention_passed, r.retention_total),
        r.spaced_passed,
        r.spaced_total,
        r.not_understood,
        r.user_turns,
        pct(r.not_understood, r.user_turns),
        r.helpful_or_clarify,
        r.labelled,
        pct(r.helpful_or_clarify, r.labelled),
        r.contradicting.len(),
        r.stored_facts,
        pct(r.contradicting.len(), r.stored_facts),
        r.teacher_meta_requests
    );
    if let Some(h) = &r.hygiene {
        s.push_str(&format!("| Duplicate entity pairs after the hygiene pass | {} | 0 |\n", h.duplicates_after));
    }
    if !r.contradicting.is_empty() {
        s.push_str("\n## Contradicting facts\n\n");
        for c in &r.contradicting {
            s.push_str(&format!("- {c}\n"));
        }
    }
    s.push_str("\n## Encounters\n");
    for e in &r.encounters {
        s.push_str(&format!("\n### {} ({}): {}\n\n", e.scenario, e.kind, if e.passed { "pass" } else { "fail" }));
        for t in &e.turns {
            s.push_str(&format!("- user: {}\n  - agent: {}{}\n", t.user, t.agent.replace('\n', " / "), t.label.as_ref().map(|l| format!(" [{l}]")).unwrap_or_default()));
        }
        if !e.judge_note.is_empty() {
            s.push_str(&format!("- judge: {}\n", e.judge_note));
        }
    }
    s
}

/// Make a user-side session with the given translator model.
pub fn llm_user(model: String) -> Box<dyn Fn(&SharedBrain, &str) -> Agent + Send + Sync> {
    Box::new(move |b, speaker| {
        let mut a = Agent::with_brain(Arc::clone(b), speaker);
        a.ask_in_chat = false;
        a.enable_llm(&model);
        a
    })
}

/// The teacher channel: its own session, tagged as teacher knowledge.
pub fn llm_teacher(model: String) -> Box<dyn Fn(&SharedBrain) -> Agent + Send + Sync> {
    Box::new(move |b| {
        let mut a = Agent::with_brain(Arc::clone(b), "teacher");
        a.source = "teacher:curriculum".into();
        a.ask_in_chat = false;
        a.enable_llm(&model);
        a
    })
}
