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
    /// The symbolic checklist check from the ground truth (see `SymbolicJudge`).
    #[serde(default)]
    pub symbolic_pass: bool,
    #[serde(default)]
    pub symbolic_note: String,
}

/// Judges an encounter against the scenario's goal checklist.
pub trait CheckJudge: Send + Sync {
    fn passes(&self, s: &Scenario, variant: bool, turns: &[Turn]) -> (bool, String);
}

/// The checklist judged by a model; verdicts are cached by the exact prompt.
pub struct ModelJudge {
    pub model: String,
    cache_path: Option<PathBuf>,
    cache: std::sync::Mutex<std::collections::HashMap<String, String>>,
}

impl ModelJudge {
    pub fn new(model: String, cache_path: Option<PathBuf>) -> Self {
        let mut cache = std::collections::HashMap::new();
        if let Some(p) = &cache_path {
            for l in std::fs::read_to_string(p).unwrap_or_default().lines() {
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(l) {
                    if let (Some(k), Some(o)) = (v["key"].as_str(), v["out"].as_str()) {
                        cache.insert(k.to_string(), o.to_string());
                    }
                }
            }
        }
        ModelJudge { model, cache_path, cache: std::sync::Mutex::new(cache) }
    }

    fn ask(&self, prompt: &str) -> Result<String, String> {
        let key = {
            let mut h: u64 = 0xcbf29ce484222325;
            for b in self.model.bytes().chain([0u8]).chain(prompt.bytes()) {
                h ^= b as u64;
                h = h.wrapping_mul(0x100000001b3);
            }
            format!("{h:016x}")
        };
        if let Some(o) = self.cache.lock().unwrap_or_else(|e| e.into_inner()).get(&key) {
            return Ok(o.clone());
        }
        let out = crate::claude_as("curriculum_judge", prompt, &self.model)?;
        if let Some(p) = &self.cache_path {
            use std::io::Write;
            if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(p) {
                let _ = writeln!(f, "{}", serde_json::json!({"key": key, "out": out}));
            }
        }
        self.cache.lock().unwrap_or_else(|e| e.into_inner()).insert(key, out.clone());
        Ok(out)
    }
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
        match self.ask(&prompt) {
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
    /// Encounter 2 passed by the symbolic check.
    #[serde(default)]
    pub repeat_symbolic: usize,
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

static LIVE: std::sync::Mutex<Option<PathBuf>> = std::sync::Mutex::new(None);

/// Write every message of the run, as it happens, to this JSON-lines file
/// (for `tools/curriculum_live.py`, and kept as training data).
pub fn set_live(path: PathBuf) {
    if let Some(d) = path.parent() {
        let _ = std::fs::create_dir_all(d);
    }
    *LIVE.lock().unwrap_or_else(|e| e.into_inner()) = Some(path);
}

/// One live event: `who` is user, agent, teacher, judge or phase.
fn live(enc: usize, kind: &str, scenario: &str, who: &str, text: &str) {
    let g = LIVE.lock().unwrap_or_else(|e| e.into_inner());
    let Some(p) = g.as_ref() else { return };
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(p) {
        let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs_f64()).unwrap_or(0.0);
        let _ = writeln!(f, "{}", serde_json::json!({"t": t, "enc": enc, "kind": kind, "scenario": scenario, "who": who, "text": text}));
    }
}
const META: &[&str] = &["say it back", "repeat after", "format", "as a list", "bullet", "numbered", "sorry", "apolog", "as an ai", "assistant"];

/// The user's side of one encounter: the request, then what they have when asked.
fn converse(a: &mut Agent, first: &str, situation: &[String], max_turns: usize, tag: (usize, &str, &str)) -> Vec<Turn> {
    let (n, kind, sid) = tag;
    let mut turns = Vec::new();
    let mut user = first.to_string();
    let mut sit = situation.iter();
    for _ in 0..max_turns {
        live(n, kind, sid, "user", &user);
        let reply = a.respond(&user).text;
        live(n, kind, sid, "agent", &reply);
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
    live(n, kind, sid, "user", "Thanks!");
    let reply = a.respond("Thanks!").text;
    live(n, kind, sid, "agent", &reply);
    turns.push(Turn { user: "Thanks!".into(), agent: reply, label: None });
    turns
}

impl CurriculumConfig {
    fn encounter(&self, brain: &SharedBrain, s: &Scenario, kind: &str, request: &str, situation: &[String], variant: bool, n: usize) -> EncounterResult {
        let mut a = (self.user_agent)(brain, &format!("user{n}"));
        live(n, kind, &s.id, "phase", &format!("encounter {n}: {kind}, {}", s.id));
        let turns = converse(&mut a, request, situation, self.max_turns, (n, kind, &s.id));
        let (passed, note) = self.judge.passes(s, variant, &turns);
        live(n, kind, &s.id, "judge", &format!("{}: {note}", if passed { "PASS" } else { "FAIL" }));
        eprintln!("[curriculum] encounter {n}: {kind:<13} {:<22} {}", s.id, if passed { "pass" } else { "fail" });
        let (symbolic_pass, symbolic_note) = SymbolicJudge.passes(s, variant, &turns);
        EncounterResult { scenario: s.id.clone(), kind: kind.into(), passed, turns, teach_blocks: 0, judge_note: note, symbolic_pass, symbolic_note }
    }

    fn teach(&self, brain: &SharedBrain, s: &Scenario) -> usize {
        let mut t = (self.teacher_agent)(brain);
        live(0, "teach_block", &s.id, "phase", &format!("teach blocks for {}", s.id));
        for line in &s.teach {
            live(0, "teach_block", &s.id, "teacher", line);
            let r = t.respond(line).text;
            live(0, "teach_block", &s.id, "agent", &r);
        }
        // Protocol rule 3: the teacher sees what was stored and fixes only what
        // is wrong: a step lost or stored with another action, or needs missing,
        // gets its teach line once more.
        let fixes = fix_lines(brain, s, &t.session_id);
        for line in &fixes {
            live(0, "teach_block", &s.id, "teacher", &format!("(fix) {line}"));
            let r = t.respond(line).text;
            live(0, "teach_block", &s.id, "agent", &r);
        }
        s.teach.len() + fixes.len()
    }
}

/// The teach lines to send again: for each step of the ground truth whose
/// action is missing or different in what this teaching session stored, its
/// "Step K of ..." line; and the needs line if a need is missing.
pub fn fix_lines(brain: &SharedBrain, s: &Scenario, session: &str) -> Vec<String> {
    let b = sla_agent::brain::lock(brain);
    // The procedure this session taught into (the agent may name it its own way).
    let Some((_, pf)) = b
        .kb
        .vault
        .procedures
        .iter()
        .map(|(k, pf)| (k, pf, pf.facts().iter().filter(|f| f.conv.as_deref() == Some(session)).count()))
        .filter(|(_, _, n)| *n > 0)
        .max_by_key(|(_, _, n)| *n)
        .map(|(k, pf, _)| (k.clone(), pf.clone()))
    else {
        return Vec::new();
    };
    let stored: Vec<GroundAtom> = pf.facts().iter().map(|f| f.atom.clone()).collect();
    drop(b);
    let sym = |g: &GroundAtom, i: usize| g.args.get(i).and_then(|v| v.as_sym()).unwrap_or("").to_string();
    let num = |st: &str| st.rsplit_once("_s").and_then(|(_, n)| n.parse::<u32>().ok());
    let action_by_num = |set: &[GroundAtom]| -> std::collections::BTreeMap<u32, String> {
        set.iter()
            .filter(|g| g.pred == "action")
            .filter_map(|g| Some((num(&sym(g, 0))?, sym(g, 1))))
            .collect()
    };
    let have = action_by_num(&stored);
    let want = action_by_num(&s.truth.iter().filter(|g| g.pred != "action" || sym(g, 0).starts_with(&s.task)).cloned().collect::<Vec<_>>());
    let mut out = Vec::new();
    for (k, a) in &want {
        if have.get(k) != Some(a) {
            let key = format!("step {k} of");
            if let Some(line) = s.teach.iter().find(|l| l.to_lowercase().contains(&key)) {
                out.push(line.clone());
            }
        }
    }
    let stored_needs: Vec<String> = stored.iter().filter(|g| g.pred == "needs").map(|g| sym(g, 1)).collect();
    let missing_need = s.truth.iter().filter(|g| g.pred == "needs").any(|g| !stored_needs.contains(&sym(g, 1)));
    if missing_need {
        if let Some(line) = s.teach.iter().find(|l| l.starts_with("To ")) {
            out.push(line.clone());
        }
    }
    out
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
        if e.symbolic_pass {
            rep.repeat_symbolic += 1;
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
    // Steps by id (`<task>_sN`: step N in both).
    let action = |set: &[GroundAtom], st: &str| set.iter().find(|g| g.pred == "action" && sym(g, 0) == st).map(|g| sym(g, 1));
    for st in stored.iter().filter(|g| g.pred == "step" && sym(g, 0) == task).map(|g| sym(g, 1)) {
        if let (Some(x), Some(y)) = (action(stored, &st), action(truth, &st)) {
            if x != y {
                out.push(format!("action({st}, {x})"));
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
    s.push_str(&format!(
        "| Encounter 2 by the symbolic check (needs, step order, time from the ground truth) | {} of {} ({:.1}%) | 70% |
",
        r.repeat_symbolic,
        r.repeat_total,
        pct(r.repeat_symbolic, r.repeat_total)
    ));
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
pub fn llm_user(model: String, cache: Option<PathBuf>) -> Box<dyn Fn(&SharedBrain, &str) -> Agent + Send + Sync> {
    Box::new(move |b, speaker| {
        let mut a = Agent::with_brain(Arc::clone(b), speaker);
        a.ask_in_chat = false;
        match &cache {
            Some(c) => a.enable_llm_cached(&model, c.clone()),
            None => a.enable_llm(&model),
        }
        a
    })
}

/// The teacher channel: its own session, tagged as teacher knowledge.
pub fn llm_teacher(model: String, cache: Option<PathBuf>) -> Box<dyn Fn(&SharedBrain) -> Agent + Send + Sync> {
    Box::new(move |b| {
        let mut a = Agent::with_brain(Arc::clone(b), "teacher");
        a.source = "teacher:curriculum".into();
        a.ask_in_chat = false;
        match &cache {
            Some(c) => a.enable_llm_cached(&model, c.clone()),
            None => a.enable_llm(&model),
        }
        a
    })
}

/// A checklist judge from the ground truth alone, with no model: the reply
/// names most of what is needed, gives most step actions in the right order,
/// and gives the total time. Used to calibrate the model judge, and for the
/// non-variant encounters (a variant needs judgement).
pub struct SymbolicJudge;

fn stem5(w: &str) -> String {
    let w = w.to_lowercase();
    let w = w.strip_suffix("ies").map(|r| format!("{r}y")).unwrap_or(w);
    let w = if w.len() > 4 { w.strip_suffix("es").or_else(|| w.strip_suffix('s')).map(String::from).unwrap_or(w) } else { w };
    w.chars().take(5).collect()
}

/// The word stems of a text, in order.
fn word_stems(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).map(stem5).collect()
}

/// Score each checklist part: (needs found / needs, steps in order / steps, time given).
pub fn symbolic_scores(s: &Scenario, turns: &[Turn]) -> (f64, f64, bool) {
    let text: String = turns.iter().map(|t| t.agent.as_str()).collect::<Vec<_>>().join(" ");
    let words = word_stems(&text);
    let sym = |g: &GroundAtom, i: usize| g.args.get(i).and_then(|v| v.as_sym()).unwrap_or("").to_string();
    let has = |symbol: &str| symbol.split('_').filter(|p| p.len() > 2).all(|p| words.contains(&stem5(p)));
    // Needs.
    let needs: Vec<String> = s.truth.iter().filter(|g| g.pred == "needs" && sym(g, 0) == s.task).map(|g| sym(g, 1)).collect();
    let need_score = if needs.is_empty() { 1.0 } else { needs.iter().filter(|n| has(n)).count() as f64 / needs.len() as f64 };
    // Steps: actions in step-number order, each found after the previous one.
    let mut steps: Vec<String> = s.truth.iter().filter(|g| g.pred == "step" && sym(g, 0) == s.task).map(|g| sym(g, 1)).collect();
    steps.sort_by_key(|st| st.rsplit_once("_s").and_then(|(_, n)| n.parse::<u32>().ok()).unwrap_or(0));
    let actions: Vec<String> = steps.iter().filter_map(|st| s.truth.iter().find(|g| g.pred == "action" && sym(g, 0) == *st).map(|g| sym(g, 1))).collect();
    let mut pos = 0;
    let mut in_order = 0;
    for a in &actions {
        let st = stem5(a.split('_').next().unwrap_or(a));
        if let Some(i) = words[pos..].iter().position(|w| *w == st) {
            pos += i + 1;
            in_order += 1;
        }
    }
    let step_score = if actions.is_empty() { 1.0 } else { in_order as f64 / actions.len() as f64 };
    // Total time: both ends of the range appear as numbers.
    let time = match s.truth.iter().find(|g| g.pred == "takes_time" && sym(g, 0) == s.task) {
        Some(g) => {
            let num = |i: usize| g.args.get(i).map(|v| v.to_string()).unwrap_or_default();
            words.contains(&num(1)) && words.contains(&num(2))
        }
        None => true,
    };
    (need_score, step_score, time)
}

impl CheckJudge for SymbolicJudge {
    fn passes(&self, s: &Scenario, _variant: bool, turns: &[Turn]) -> (bool, String) {
        let (n, st, t) = symbolic_scores(s, turns);
        let pass = n >= 0.75 && st >= 0.75 && t;
        (pass, format!("needs {:.0}%, steps in order {:.0}%, total time {}", n * 100.0, st * 100.0, if t { "yes" } else { "no" }))
    }
}

/// The model judge's verdicts on a recorded run against the symbolic judge
/// (encounters without a variant).
#[derive(Debug, Clone, Serialize, Default)]
pub struct Calibration {
    pub encounters: usize,
    pub both_pass: usize,
    pub both_fail: usize,
    pub model_only: usize,
    pub symbolic_only: usize,
    pub repeat_model: usize,
    pub repeat_symbolic: usize,
    pub repeat_total: usize,
    pub disagreements: Vec<String>,
}

pub fn calibrate(report: &serde_json::Value, scenarios: &[Scenario]) -> Calibration {
    let mut c = Calibration::default();
    for e in report["encounters"].as_array().into_iter().flatten() {
        let kind = e["kind"].as_str().unwrap_or("");
        if !matches!(kind, "teach_retry" | "repeat" | "retention" | "spaced_repeat") {
            continue;
        }
        let Some(s) = scenarios.iter().find(|s| Some(s.id.as_str()) == e["scenario"].as_str()) else { continue };
        let turns: Vec<Turn> = e["turns"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|t| Turn { user: t["user"].as_str().unwrap_or("").into(), agent: t["agent"].as_str().unwrap_or("").into(), label: None })
            .collect();
        let model = e["passed"].as_bool().unwrap_or(false);
        let (sym, note) = SymbolicJudge.passes(s, false, &turns);
        c.encounters += 1;
        match (model, sym) {
            (true, true) => c.both_pass += 1,
            (false, false) => c.both_fail += 1,
            (true, false) => c.model_only += 1,
            (false, true) => c.symbolic_only += 1,
        }
        if model != sym {
            let m = if model { "pass" } else { "fail" };
            let y = if sym { "pass" } else { "fail" };
            c.disagreements.push(format!("{} ({kind}): model {m}, symbolic {y} ({note}); model said: {}", s.id, e["judge_note"].as_str().unwrap_or("")));
        }
        if kind == "repeat" {
            c.repeat_total += 1;
            c.repeat_model += model as usize;
            c.repeat_symbolic += sym as usize;
        }
    }
    c
}
