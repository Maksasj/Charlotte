//! Training app backend: N teachers hold small conversations with the agent,
//! teaching it as they go. All teachers share one [`Brain`]: one in-memory
//! vault with one writer, so parallel teachers never reload or overwrite each
//! other. A tiny HTTP server serves the dashboard, the live state and the
//! event stream; training is started and stopped from the dashboard.
//!
//! The API answers only requests that carry the token printed at start-up
//! and come from this machine's own page (an `Origin` other than
//! `http://localhost:<port>` or `http://127.0.0.1:<port>` is refused), so no
//! other web page can start a run that spends plan limits.

use crate::outcome::{template_label, user_cues, Label};
use sla_agent::brain::{self, Brain, SharedBrain};
use sla_agent::Agent;
use sla_core::GroundAtom;
use sla_parser::claude::{global_usage, take_thread_usage, Usage};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

/// Everyday situations for training conversations, in the style of a person
/// asking an assistant for help or just chatting.
pub const SCENARIOS: &[&str] = &[
    "You want help planning dinner tonight. You have pasta, garlic, olive oil and cherry tomatoes, and about 20 minutes.",
    "You are going out and it looks like rain; you want advice on what to wear.",
    "You want to make a good cup of tea and ask how.",
    "You are planning a picnic in the park this weekend and want ideas on what to bring.",
    "You want to grow some plants on a small balcony and ask which are easy.",
    "Your bicycle has a flat tire and you want to know how to fix it.",
    "You are packing for a day at the beach and want a checklist.",
    "You are just chatting about the weather and the season.",
    "You are thinking about getting a pet for a small flat and want advice.",
    "You want to cook rice properly and ask how.",
    "It is a rainy afternoon and you want ideas for what to do indoors.",
    "You want to make a quick healthy breakfast smoothie.",
    "You need to clean the kitchen quickly before guests arrive.",
    "You want tips to fall asleep earlier.",
    "You want a simple workout you can do at home without equipment.",
    "You are curious which fruit is in season in autumn.",
    "Your houseplant has yellow leaves and you want help looking after it.",
    "You are planning a small birthday party for a child.",
    "You want to boil an egg and ask how long it takes.",
    "You want to make a sandwich for lunch with what you have: bread, cheese, tomato and lettuce.",
    "You want to keep your dog happy and healthy.",
    "You are wondering which birds visit gardens and how to attract them.",
    "You want simple ways to save energy at home.",
    "You are packing for a day of hiking in the hills.",
    "You want to make pancakes and ask what you need.",
    "You feel cold at home in winter and want ways to stay warm.",
    "You have eggs, milk and cheese and want to know what you could cook.",
    "You are chatting about favourite foods and what they are made of.",
    "You want to learn the basics of looking after a vegetable garden.",
    "You want to make a simple tomato soup.",
];


fn now_s() -> f64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs_f64()).unwrap_or(0.0)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TrainConfig {
    pub teachers: usize,
    pub turns: usize,
    pub conversations: usize,
    pub model: String,
    pub review: bool,
}

impl Default for TrainConfig {
    fn default() -> Self {
        TrainConfig { teachers: 4, turns: 10, conversations: 20, model: "haiku".into(), review: true }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Msg {
    /// "teacher", "agent" or "note".
    pub role: String,
    pub text: String,
    pub at: f64,
    /// How long this message took to produce, in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ms: Option<u64>,
    /// Agent answers: time spent translating the message with the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translate_ms: Option<u64>,
    /// Agent answers: time spent by the agent itself (reasoning, learning, waiting for the vault).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub think_ms: Option<u64>,
    /// Agent answers: the outcome label (`unjudged` for content replies until a judge labels them).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Agent answers: the dialogue acts the message was translated into.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub acts: Vec<String>,
    /// Agent answers: the moves chosen.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub moves: Vec<String>,
    /// Agent answers: the session and turn of the trace, for the drill-down.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<(String, usize)>,
}

impl Msg {
    fn new(role: &str, text: &str) -> Self {
        Msg { role: role.into(), text: text.into(), at: now_s(), ms: None, translate_ms: None, think_ms: None, label: None, acts: Vec::new(), moves: Vec::new(), trace: None }
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct TeacherView {
    pub id: usize,
    /// idle, thinking, agent, reviewing, done, stopped, error
    pub status: String,
    pub scenario: String,
    pub conversation: Option<usize>,
    pub turn: usize,
    pub turns_max: usize,
    pub transcript: Vec<Msg>,
    pub done: usize,
    pub facts: usize,
    pub rules: usize,
    pub rejected: usize,
    pub usage: BTreeMap<String, Usage>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Finished {
    pub teacher: usize,
    pub conversation: usize,
    pub scenario: String,
    pub at: f64,
    pub turns: usize,
    pub understood: usize,
    pub facts: usize,
    pub rules: usize,
    pub rejected: Vec<String>,
    pub transcript: Vec<Msg>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct VaultCounts {
    pub facts: usize,
    pub closed: usize,
    pub rules_confirmed: usize,
    pub rules_hypothesis: usize,
    pub rules_rejected: usize,
    pub entities: usize,
    pub quarantined: usize,
}

/// One plan usage limit as `/usage` reports it.
#[derive(Debug, Clone, Serialize, Default)]
pub struct Limit {
    pub name: String,
    pub percent: f64,
    pub resets: String,
}

/// Read the plan's usage limits with a headless `/usage` (uses no tokens).
pub fn fetch_limits() -> Result<Vec<Limit>, String> {
    let mut child = std::process::Command::new("claude")
        .args(["-p", "--output-format", "json", "--tools", "", "--no-session-persistence", "--strict-mcp-config"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map_err(|e| format!("cannot run claude: {e}"))?;
    child.stdin.take().unwrap().write_all(b"/usage").map_err(|e| e.to_string())?;
    let out = child.wait_with_output().map_err(|e| e.to_string())?;
    let v: serde_json::Value = serde_json::from_slice(&out.stdout).map_err(|_| "unreadable /usage output".to_string())?;
    let text = v.get("result").and_then(|r| r.as_str()).unwrap_or("");
    let mut limits = Vec::new();
    for line in text.lines().map(str::trim).filter(|l| l.starts_with("Current ")) {
        // "Current session: 14% used · resets Nov 17, 3:29am (Europe/Kyiv)"
        let Some((name, rest)) = line.split_once(": ") else { continue };
        let Some((num, after)) = rest.split_once('%') else { continue };
        let Ok(percent) = num.trim().parse::<f64>() else { continue };
        let resets = after.split_once("resets ").map(|(_, r)| r.trim().to_string()).unwrap_or_default();
        limits.push(Limit { name: name.trim_start_matches("Current ").to_string(), percent, resets });
    }
    if limits.is_empty() {
        return Err(if text.is_empty() { "no answer from /usage".into() } else { text.lines().next().unwrap_or("").to_string() });
    }
    Ok(limits)
}


#[derive(Debug, Clone, Serialize, Default)]
pub struct TrainState {
    pub running: bool,
    pub stopping: bool,
    pub started_at: Option<f64>,
    pub finished_at: Option<f64>,
    pub now: f64,
    pub config: Option<TrainConfig>,
    pub planned: usize,
    pub completed: usize,
    pub turns: usize,
    pub understood: usize,
    pub facts: usize,
    pub rules: usize,
    pub rejected: usize,
    /// Outcome labels of this run's agent turns.
    pub outcomes: BTreeMap<String, usize>,
    pub teachers: Vec<TeacherView>,
    /// Most recent first.
    pub finished: Vec<Finished>,
    /// All model calls since the app started, by role.
    pub usage: BTreeMap<String, Usage>,
    pub usage_total: Usage,
    /// Model calls of the current or last run only.
    pub run_usage: Usage,
    pub vault: VaultCounts,
    pub vault_path: String,
    pub scenarios: usize,
    /// Plan usage limits from `/usage`, refreshed about once a minute.
    pub limits: Vec<Limit>,
    pub limits_checked: Option<f64>,
    pub limits_error: Option<String>,
    /// Sums over the agent's answers this run (divide by `turns` for averages).
    pub answer_ms_total: u64,
    pub translate_ms_total: u64,
    pub think_ms_total: u64,
    /// The agent's own work per turn (ms) this run, for the speed panel.
    pub think_ms: Vec<f64>,
    /// The newest event sequence number.
    pub last_event: u64,
}

pub struct Trainer {
    pub vault: PathBuf,
    /// Where turn traces are written (`sessions/` beside the vault).
    pub sessions: PathBuf,
    /// Required on every API request.
    pub token: String,
    brain: Mutex<Option<SharedBrain>>,
    state: Mutex<TrainState>,
    stop: AtomicBool,
    next: AtomicUsize,
    active: AtomicUsize,
    usage_at_start: Mutex<Usage>,
    limits: Mutex<(Vec<Limit>, Option<f64>, Option<String>)>,
}

fn total(m: &BTreeMap<String, Usage>) -> Usage {
    let mut t = Usage::default();
    for u in m.values() {
        t.add(u);
    }
    t
}

fn count_vault(b: &Brain) -> VaultCounts {
    let mut c = VaultCounts { entities: b.kb.vault.facts.len(), quarantined: b.quarantined.len(), ..Default::default() };
    for (_, f) in b.kb.vault.all_facts() {
        if f.is_open() {
            c.facts += 1;
        } else {
            c.closed += 1;
        }
    }
    for rf in b.kb.vault.rules.values() {
        match rf.meta().status {
            sla_core::RuleStatus::Confirmed => c.rules_confirmed += 1,
            sla_core::RuleStatus::Hypothesis => c.rules_hypothesis += 1,
            sla_core::RuleStatus::Rejected => c.rules_rejected += 1,
        }
    }
    c
}

/// A random token (the standard library seeds `RandomState` from the OS).
pub fn new_token() -> String {
    use std::hash::{BuildHasher, Hasher};
    let mut s = String::new();
    for i in 0..2u64 {
        let mut h = std::collections::hash_map::RandomState::new().build_hasher();
        h.write_u64(i);
        h.write_u128(SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0));
        s.push_str(&format!("{:016x}", h.finish()));
    }
    s
}

fn teacher_prompt(scenario: &str, transcript: &[Msg]) -> String {
    let so_far = if transcript.is_empty() {
        "(you start the conversation)".to_string()
    } else {
        transcript
            .iter()
            .filter(|m| m.role != "note")
            .map(|m| format!("{}: {}", if m.role == "teacher" { "You" } else { "Assistant" }, m.text))
            .collect::<Vec<_>>()
            .join("\n")
    };
    format!(
        "You are teaching a young AI assistant how to hold helpful, natural everyday conversations, by having them with it.\n\
         Play a person in this situation: {scenario}\n\n\
         How to play it:\n\
         - Write like a real person chatting: one to three short sentences per message.\n\
         - Let the conversation go the way it would with a good assistant: ask for what you need, answer its questions, react, thank it.\n\
         - When the assistant does not understand, cannot help or answers badly, teach it right there: say simply what a good \
           answer would contain, as plain true facts it can remember (what things are, what something needs, how long it takes, \
           the steps in order), then carry on with the conversation.\n\
         - Only true, general knowledge. Never invent facts about specific or private people.\n\
         - When your goal is reached or the chat has run its course, end it naturally (for example, thank it) and put [done] at \
           the very end of that message.\n\n\
         Conversation so far:\n{so_far}\n\nWrite only your next message."
    )
}


/// A fact a conversation stored: its id (to close exactly it) and its atom.
#[derive(Debug, Clone, PartialEq)]
pub struct Stored {
    pub id: Option<String>,
    pub atom: GroundAtom,
}

/// What a turn stored, read from its trace: (facts added, rule ids added).
fn learned(t: &sla_agent::TurnTrace) -> (Vec<Stored>, Vec<String>) {
    let facts = t
        .facts_added_with_ids()
        .into_iter()
        .filter_map(|(id, s)| sla_core::parse_ground(s).ok().map(|atom| Stored { id: id.map(String::from), atom }))
        .filter(|f| f.atom.pred != "proper_name")
        .collect();
    let rules = t.rules_added().into_iter().filter_map(|s| s.split(':').next().map(|x| x.trim().to_string())).collect();
    (facts, rules)
}

/// Close one stored fact: by id when it has one (never another conversation's copy).
fn close_stored(b: &mut Brain, f: &Stored, now: sla_core::Timestamp, reason: &str) {
    match &f.id {
        Some(id) => {
            b.kb.close_by_id(id, now, reason);
        }
        None => {
            b.kb.close_matching(&f.atom.to_atom(), now, reason);
        }
    }
}

/// Personal remarks of the teacher's character are conversation, not general knowledge.
fn personal(g: &GroundAtom) -> bool {
    g.args.iter().any(|v| v.as_sym().map_or(false, |x| x == "friend" || x.starts_with("my_")))
}

/// The reviewer sees what a conversation stored, in plain English, and flags
/// mistakes; flagged facts are closed by id (only this conversation's copy,
/// never a fact another teacher taught) and flagged rules rejected. The model
/// call happens without the brain lock.
pub fn review_and_fix(agent: &mut Agent, new_facts: &[Stored], new_rules: &[String], model: &str, out: &mut Vec<String>) {
    if new_facts.is_empty() && new_rules.is_empty() {
        return;
    }
    let mut items: Vec<String> = new_facts.iter().map(|f| agent.realizer.clause(&f.atom)).collect();
    {
        let b = agent.brain();
        items.extend(new_rules.iter().map(|id| b.kb.vault.rules.get(id).and_then(|rf| rf.rules().first().map(|r| agent.realizer.rule_text(r))).unwrap_or_default()));
    }
    let listing: String = items.iter().enumerate().map(|(i, t)| format!("{}. {t}\n", i + 1)).collect();
    let prompt = format!(
        "A simple learning agent stored these statements as general knowledge (the wording is mechanical; 'you' means \
         the user). Statements about a kind are general defaults: OK if they hold for typical members, even with \
         exceptions.\n\n{listing}\nFor every number, output one line 'N: OK' or 'N: WRONG <short reason>'. Mark WRONG \
         only if false for typical cases or garbled into a different meaning."
    );
    let Ok(verdicts) = crate::claude_as("reviewer", &prompt, model) else { return };
    let now = agent.now();
    let mut b = agent.brain();
    for line in verdicts.lines() {
        let Some((n, verdict)) = line.split_once(':') else { continue };
        let Ok(n) = n.trim().trim_start_matches('#').parse::<usize>() else { continue };
        let verdict = verdict.trim();
        if !verdict.to_uppercase().starts_with("WRONG") || n == 0 || n > items.len() {
            continue;
        }
        let reason = format!("reviewer: {}", verdict.get(5..).unwrap_or("").trim());
        if n <= new_facts.len() {
            close_stored(&mut b, &new_facts[n - 1], now, &reason);
        } else {
            let id = &new_rules[n - 1 - new_facts.len()];
            if let Some(rf) = b.kb.vault.rules.get_mut(id) {
                let mut m = rf.meta();
                m.status = sla_core::RuleStatus::Rejected;
                rf.set_meta(&m);
                b.kb.vault.mark_rule_dirty(id);
                b.kb.log_change(now, "reject_rule", id, &reason);
            }
        }
        out.push(format!("{} ({reason})", items[n - 1]));
    }
    let _ = b.save();
}

impl Trainer {
    pub fn new(vault: PathBuf) -> Arc<Self> {
        let sessions = vault.parent().map(|p| p.join("sessions")).unwrap_or_else(|| PathBuf::from("sessions"));
        let state = TrainState { vault_path: vault.display().to_string(), scenarios: SCENARIOS.len(), ..Default::default() };
        Arc::new(Trainer {
            vault,
            sessions,
            token: new_token(),
            brain: Mutex::new(None),
            state: Mutex::new(state),
            stop: AtomicBool::new(false),
            next: AtomicUsize::new(0),
            active: AtomicUsize::new(0),
            usage_at_start: Mutex::new(Usage::default()),
            limits: Mutex::new((Vec::new(), None, None)),
        })
    }

    /// The shared brain, opened on first use.
    pub fn brain(&self) -> std::io::Result<SharedBrain> {
        let mut g = self.brain.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(b) = g.as_ref() {
            return Ok(Arc::clone(b));
        }
        let b = Brain::shared(&self.vault)?;
        if let Err(e) = brain::lock(&b).events.attach_file(&self.sessions.join("events.jsonl")) {
            eprintln!("warning: cannot write the event file: {e}");
        }
        *g = Some(Arc::clone(&b));
        Ok(b)
    }

    /// Check the plan limits now (blocking; call from a background thread).
    pub fn refresh_limits(&self) {
        let r = fetch_limits();
        let mut l = self.limits.lock().unwrap_or_else(|e| e.into_inner());
        match r {
            Ok(v) => *l = (v, Some(now_s()), None),
            Err(e) => {
                l.1 = Some(now_s());
                l.2 = Some(e);
            }
        }
    }

    /// A snapshot of the state for the UI.
    pub fn snapshot(&self) -> TrainState {
        let (vault, last_event) = match self.brain() {
            Ok(b) => {
                let g = brain::lock(&b);
                (count_vault(&g), g.events.last_seq())
            }
            Err(_) => (VaultCounts::default(), 0),
        };
        let mut s = self.state.lock().unwrap_or_else(|e| e.into_inner()).clone();
        s.now = now_s();
        s.usage = global_usage();
        s.usage_total = total(&s.usage);
        let base = self.usage_at_start.lock().unwrap_or_else(|e| e.into_inner()).clone();
        let mut run = s.usage_total.clone();
        run.calls -= base.calls.min(run.calls);
        run.errors -= base.errors.min(run.errors);
        run.input_tokens -= base.input_tokens.min(run.input_tokens);
        run.output_tokens -= base.output_tokens.min(run.output_tokens);
        run.cache_write_tokens -= base.cache_write_tokens.min(run.cache_write_tokens);
        run.cache_read_tokens -= base.cache_read_tokens.min(run.cache_read_tokens);
        run.cost_usd -= base.cost_usd;
        run.seconds -= base.seconds;
        s.run_usage = run;
        s.vault = vault;
        s.last_event = last_event;
        let l = self.limits.lock().unwrap_or_else(|e| e.into_inner());
        s.limits = l.0.clone();
        s.limits_checked = l.1;
        s.limits_error = l.2.clone();
        s
    }

    /// Events after `after`, for the dashboard.
    pub fn events(&self, after: u64, max: usize) -> Vec<sla_agent::events::Event> {
        match self.brain() {
            Ok(b) => {
                let bus = Arc::clone(&brain::lock(&b).events);
                bus.since(after, max)
            }
            Err(_) => Vec::new(),
        }
    }

    pub fn start(self: &Arc<Self>, cfg: TrainConfig) -> Result<(), String> {
        let cfg = TrainConfig { teachers: cfg.teachers.clamp(1, 24), turns: cfg.turns.clamp(2, 30), conversations: cfg.conversations.clamp(1, 1000), ..cfg };
        let brain = self.brain().map_err(|e| format!("cannot open the vault: {e}"))?;
        {
            let mut s = self.state.lock().unwrap_or_else(|e| e.into_inner());
            if s.running {
                return Err("training is already running".into());
            }
            *s = TrainState {
                running: true,
                started_at: Some(now_s()),
                planned: cfg.conversations,
                teachers: (0..cfg.teachers).map(|id| TeacherView { id, status: "idle".into(), turns_max: cfg.turns, ..Default::default() }).collect(),
                finished: std::mem::take(&mut s.finished),
                config: Some(cfg.clone()),
                vault_path: s.vault_path.clone(),
                scenarios: SCENARIOS.len(),
                ..Default::default()
            };
        }
        brain::lock(&brain).events.publish("run_start", &cfg);
        *self.usage_at_start.lock().unwrap_or_else(|e| e.into_inner()) = total(&global_usage());
        self.stop.store(false, Ordering::SeqCst);
        self.next.store(0, Ordering::SeqCst);
        self.active.store(cfg.teachers, Ordering::SeqCst);
        for id in 0..cfg.teachers {
            let me = Arc::clone(self);
            let cfg = cfg.clone();
            let brain = Arc::clone(&brain);
            std::thread::spawn(move || {
                loop {
                    if me.stop.load(Ordering::SeqCst) {
                        break;
                    }
                    let n = me.next.fetch_add(1, Ordering::SeqCst);
                    if n >= cfg.conversations {
                        break;
                    }
                    let scenario = SCENARIOS[n % SCENARIOS.len()];
                    me.conversation(&brain, id, n, scenario, &cfg);
                }
                me.update(id, |t| t.status = if me.stop.load(Ordering::SeqCst) { "stopped".into() } else { "done".into() });
                if me.active.fetch_sub(1, Ordering::SeqCst) == 1 {
                    let mut s = me.state.lock().unwrap_or_else(|e| e.into_inner());
                    s.running = false;
                    s.stopping = false;
                    s.finished_at = Some(now_s());
                    let summary = serde_json::json!({ "completed": s.completed, "turns": s.turns, "outcomes": s.outcomes });
                    drop(s);
                    brain::lock(&brain).events.publish("run_end", summary);
                }
            });
        }
        Ok(())
    }

    /// Block until the current run has finished.
    pub fn wait(&self) {
        while self.state.lock().unwrap_or_else(|e| e.into_inner()).running {
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
    }

    pub fn request_stop(&self) {
        self.stop.store(true, Ordering::SeqCst);
        let mut s = self.state.lock().unwrap_or_else(|e| e.into_inner());
        if s.running {
            s.stopping = true;
        }
    }

    fn update(&self, id: usize, f: impl FnOnce(&mut TeacherView)) {
        let mut s = self.state.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(t) = s.teachers.get_mut(id) {
            f(t);
        }
    }

    /// Fold this thread's model usage into the teacher's counters.
    fn absorb_usage(&self, id: usize) {
        let u = take_thread_usage();
        self.update(id, |t| {
            for (role, x) in u {
                t.usage.entry(role).or_default().add(&x);
            }
        });
    }

    fn push(&self, id: usize, role: &str, text: &str) {
        self.push_msg(id, Msg::new(role, text));
    }

    fn push_msg(&self, id: usize, m: Msg) {
        self.update(id, |t| t.transcript.push(m));
    }

    /// One training step: a small conversation, then the review.
    fn conversation(&self, brain: &SharedBrain, id: usize, n: usize, scenario: &str, cfg: &TrainConfig) {
        self.update(id, |t| {
            t.status = "thinking".into();
            t.scenario = scenario.into();
            t.conversation = Some(n + 1);
            t.turn = 0;
            t.transcript.clear();
            t.last_error = None;
        });
        // The teacher is its own speaker: its personal remarks are not about the user.
        let mut agent = Agent::with_brain(Arc::clone(brain), "friend");
        agent.source = "teacher:train".into();
        agent.trace_dir = Some(self.sessions.clone());
        agent.enable_llm(&cfg.model);
        brain::lock(brain).events.publish("conversation_start", serde_json::json!({ "teacher": id, "conversation": n + 1, "scenario": scenario, "session": agent.session_id }));
        let mut new_facts: Vec<Stored> = Vec::new();
        let mut new_rules: Vec<String> = Vec::new();
        let mut transcript: Vec<Msg> = Vec::new();
        let mut understood = 0;
        let mut turns = 0;
        for turn in 0..cfg.turns {
            if self.stop.load(Ordering::SeqCst) {
                break;
            }
            self.update(id, |t| {
                t.status = "thinking".into();
                t.turn = turn + 1;
            });
            let teacher_started = Instant::now();
            let raw = match crate::claude_as("teacher", &teacher_prompt(scenario, &transcript), &cfg.model) {
                Ok(m) => m,
                Err(e) => {
                    self.absorb_usage(id);
                    self.update(id, |t| t.last_error = Some(e.clone()));
                    self.push(id, "note", &format!("Teacher call failed: {e}"));
                    break;
                }
            };
            let mut msg = raw.lines().map(str::trim).filter(|l| !l.is_empty()).collect::<Vec<_>>().join(" ");
            msg = msg.trim_start_matches("You:").trim().trim_matches('"').to_string();
            let done = msg.contains("[done]");
            msg = msg.replace("[done]", "").trim().to_string();
            if msg.is_empty() {
                break;
            }
            let teacher_msg = Msg { ms: Some(teacher_started.elapsed().as_millis() as u64), ..Msg::new("teacher", &msg) };
            self.push_msg(id, teacher_msg.clone());
            transcript.push(teacher_msg);
            self.update(id, |t| t.status = "agent".into());
            // Translation runs outside the brain lock; only the agent's own work is serialized.
            let reply = agent.respond(&msg);
            let trace = agent.last_trace.clone().unwrap_or_default();
            let (facts_added, rules_added) = learned(&trace);
            let label = template_label(&msg, &reply.text).unwrap_or(Label::Unjudged);
            turns += 1;
            if trace.understood {
                understood += 1;
            }
            let translate_ms = trace.translate_ms.round() as u64;
            let think_ms = trace.think_ms.round() as u64;
            {
                let mut s = self.state.lock().unwrap_or_else(|e| e.into_inner());
                s.turns += 1;
                if trace.understood {
                    s.understood += 1;
                }
                s.facts += facts_added.iter().filter(|f| !personal(&f.atom)).count();
                s.rules += rules_added.len();
                *s.outcomes.entry(label.name().to_string()).or_default() += 1;
                s.answer_ms_total += translate_ms + think_ms;
                s.translate_ms_total += translate_ms;
                s.think_ms_total += think_ms;
                s.think_ms.push(trace.think_ms);
            }
            self.update(id, |t| {
                t.facts += facts_added.len();
                t.rules += rules_added.len();
            });
            new_facts.extend(facts_added);
            new_rules.extend(rules_added);
            let agent_msg = Msg {
                ms: Some(translate_ms + think_ms),
                translate_ms: Some(translate_ms),
                think_ms: Some(think_ms),
                label: Some(label.name().into()),
                acts: trace.parts.iter().flat_map(|p| p.acts.clone()).collect(),
                moves: trace.moves.clone(),
                trace: Some((trace.session.clone(), trace.turn)),
                ..Msg::new("agent", &reply.text)
            };
            self.push_msg(id, agent_msg.clone());
            transcript.push(agent_msg);
            self.absorb_usage(id);
            if done {
                break;
            }
        }
        if new_facts.iter().any(|f| personal(&f.atom)) {
            let now = agent.now();
            let mut b = agent.brain();
            for f in new_facts.iter().filter(|f| personal(&f.atom)) {
                close_stored(&mut b, f, now, "personal remark in a training chat, not general knowledge");
            }
            let _ = b.save();
        }
        new_facts.retain(|f| !personal(&f.atom));
        let mut rejected = Vec::new();
        if cfg.review && !self.stop.load(Ordering::SeqCst) {
            self.update(id, |t| t.status = "reviewing".into());
            review_and_fix(&mut agent, &new_facts, &new_rules, &cfg.model, &mut rejected);
            self.absorb_usage(id);
        }
        let requests = transcript.iter().filter(|m| m.role == "teacher" && user_cues(&m.text).request).count();
        brain::lock(brain).events.publish(
            "conversation_end",
            serde_json::json!({ "teacher": id, "conversation": n + 1, "scenario": scenario, "turns": turns, "understood": understood, "requests": requests, "facts": new_facts.len(), "rules": new_rules.len(), "rejected": rejected.len() }),
        );
        let fin = Finished {
            teacher: id,
            conversation: n + 1,
            scenario: scenario.into(),
            at: now_s(),
            turns,
            understood,
            facts: new_facts.len(),
            rules: new_rules.len(),
            rejected: rejected.clone(),
            transcript,
        };
        let mut s = self.state.lock().unwrap_or_else(|e| e.into_inner());
        s.completed += 1;
        s.rejected += rejected.len();
        s.finished.insert(0, fin);
        s.finished.truncate(60);
        if let Some(t) = s.teachers.get_mut(id) {
            t.done += 1;
            t.rejected += rejected.len();
            t.status = "idle".into();
        }
    }
}

// ---------------------------------------------------------------- HTTP

fn respond(stream: &mut TcpStream, code: u16, ctype: &str, body: &[u8]) {
    let reason = match code {
        200 => "OK",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        409 => "Conflict",
        _ => "Bad Request",
    };
    let head = format!(
        "HTTP/1.1 {code} {reason}\r\nContent-Type: {ctype}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(body);
}

/// A parsed HTTP request (just what the trainer needs).
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub query: BTreeMap<String, String>,
    pub headers: BTreeMap<String, String>,
    pub body: Vec<u8>,
}

fn read_request(stream: &TcpStream) -> Option<Request> {
    let mut reader = BufReader::new(stream.try_clone().ok()?);
    let mut first = String::new();
    reader.read_line(&mut first).ok()?;
    let mut headers = BTreeMap::new();
    loop {
        let mut h = String::new();
        if reader.read_line(&mut h).is_err() || h == "\r\n" || h.is_empty() {
            break;
        }
        if let Some((k, v)) = h.split_once(':') {
            headers.insert(k.trim().to_ascii_lowercase(), v.trim().to_string());
        }
    }
    let len: usize = headers.get("content-length").and_then(|v| v.parse().ok()).unwrap_or(0);
    let mut body = vec![0u8; len.min(1 << 20)];
    let _ = reader.read_exact(&mut body);
    let mut parts = first.split_whitespace();
    let method = parts.next().unwrap_or("").to_string();
    let target = parts.next().unwrap_or("/");
    let (path, q) = target.split_once('?').unwrap_or((target, ""));
    let query = q.split('&').filter_map(|kv| kv.split_once('=')).map(|(k, v)| (k.to_string(), v.to_string())).collect();
    Some(Request { method, path: path.to_string(), query, headers, body })
}

/// Why an API request is refused, if it is: a foreign origin, or a missing or wrong token.
pub fn refuse(req: &Request, token: &str, port: u16) -> Option<(u16, &'static str)> {
    if let Some(origin) = req.headers.get("origin") {
        let ok = [format!("http://localhost:{port}"), format!("http://127.0.0.1:{port}")];
        if !ok.iter().any(|o| o == origin) {
            return Some((403, "requests from other web pages are not allowed"));
        }
    }
    let given = req.headers.get("x-trainer-token").or_else(|| req.query.get("token"));
    if given.map(String::as_str) != Some(token) {
        return Some((401, "missing or wrong token: open the address the trainer printed at start-up"));
    }
    None
}

fn handle(trainer: &Arc<Trainer>, ui: &PathBuf, port: u16, mut stream: TcpStream) {
    let Some(req) = read_request(&stream) else { return };
    if req.path.starts_with("/api/") {
        if let Some((code, why)) = refuse(&req, &trainer.token, port) {
            respond(&mut stream, code, "application/json", serde_json::json!({ "ok": false, "error": why }).to_string().as_bytes());
            return;
        }
    }
    match (req.method.as_str(), req.path.as_str()) {
        ("GET", "/") => match std::fs::read(ui) {
            Ok(page) => respond(&mut stream, 200, "text/html; charset=utf-8", &page),
            Err(e) => respond(&mut stream, 404, "text/plain", format!("UI file {} not found: {e}", ui.display()).as_bytes()),
        },
        ("GET", "/api/state") => {
            let s = serde_json::to_vec(&trainer.snapshot()).unwrap_or_default();
            respond(&mut stream, 200, "application/json", &s);
        }
        ("GET", "/api/events") => {
            let after = req.query.get("after").and_then(|v| v.parse().ok()).unwrap_or(0);
            let s = serde_json::to_vec(&trainer.events(after, 500)).unwrap_or_default();
            respond(&mut stream, 200, "application/json", &s);
        }
        ("POST", "/api/start") => {
            let cfg: TrainConfig = serde_json::from_slice(&req.body).unwrap_or_default();
            match trainer.start(cfg) {
                Ok(()) => respond(&mut stream, 200, "application/json", b"{\"ok\":true}"),
                Err(e) => respond(&mut stream, 409, "application/json", serde_json::json!({ "ok": false, "error": e }).to_string().as_bytes()),
            }
        }
        ("POST", "/api/stop") => {
            trainer.request_stop();
            respond(&mut stream, 200, "application/json", b"{\"ok\":true}");
        }
        _ => respond(&mut stream, 404, "text/plain", b"not found"),
    }
}

/// Serve the UI and API until the process ends.
pub fn serve(trainer: Arc<Trainer>, ui: PathBuf, port: u16) -> std::io::Result<()> {
    let listener = TcpListener::bind(("127.0.0.1", port))?;
    // Plan limits: check now and then about once a minute.
    {
        let t = Arc::clone(&trainer);
        std::thread::spawn(move || loop {
            t.refresh_limits();
            std::thread::sleep(std::time::Duration::from_secs(60));
        });
    }
    for stream in listener.incoming().flatten() {
        let t = Arc::clone(&trainer);
        let ui = ui.clone();
        std::thread::spawn(move || handle(&t, &ui, port, stream));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn req(origin: Option<&str>, token: Option<&str>) -> Request {
        let mut r = Request { method: "POST".into(), path: "/api/start".into(), ..Default::default() };
        if let Some(o) = origin {
            r.headers.insert("origin".into(), o.into());
        }
        if let Some(t) = token {
            r.headers.insert("x-trainer-token".into(), t.into());
        }
        r
    }

    #[test]
    fn reviewer_closes_only_its_own_fact() {
        let dir = std::env::temp_dir().join(format!("sla_review_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let brain = Brain::shared(&dir).unwrap();
        let mut a = Agent::with_brain(Arc::clone(&brain), "friend");
        a.ask_in_chat = false;
        a.respond("Anna likes tea.");
        let (facts, rules) = learned(a.last_trace.as_ref().unwrap());
        assert_eq!(facts.len(), 1);
        assert!(facts[0].id.is_some(), "the trace carries the fact id");
        // Another conversation's copy of the same atom (as v1 could store).
        {
            let mut b = brain::lock(&brain);
            let mut other = sla_core::Fact::new(sla_core::parse_ground("likes(anna, tea)").unwrap(), "teacher:other").with_conv("s-other");
            other.id = Some("f900".into());
            other.from = Some(a.now());
            b.kb.vault.add_fact(other);
        }
        sla_parser::claude::set_thread_mock(Arc::new(|role, _| {
            assert_eq!(role, "reviewer");
            Ok("1: WRONG not general knowledge".into())
        }));
        let mut rejected = Vec::new();
        review_and_fix(&mut a, &facts, &rules, "mock", &mut rejected);
        sla_parser::claude::clear_thread_mock();
        assert_eq!(rejected.len(), 1);
        let b = brain::lock(&brain);
        let open: Vec<String> = b.kb.all_facts().into_iter().filter(|f| f.atom.to_string() == "likes(anna, tea)" && f.is_open()).filter_map(|f| f.id).collect();
        assert_eq!(open, vec!["f900".to_string()], "only the reviewed conversation's fact was closed");
        drop(b);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn api_needs_token_and_local_origin() {
        let tok = "abc123";
        assert_eq!(refuse(&req(None, Some(tok)), tok, 8770), None, "curl-style local call with the token");
        assert_eq!(refuse(&req(Some("http://localhost:8770"), Some(tok)), tok, 8770), None);
        assert_eq!(refuse(&req(Some("http://127.0.0.1:8770"), Some(tok)), tok, 8770), None);
        assert_eq!(refuse(&req(Some("http://evil.example"), Some(tok)), tok, 8770).map(|r| r.0), Some(403));
        assert_eq!(refuse(&req(Some("http://localhost:9999"), Some(tok)), tok, 8770).map(|r| r.0), Some(403));
        assert_eq!(refuse(&req(None, None), tok, 8770).map(|r| r.0), Some(401));
        assert_eq!(refuse(&req(None, Some("wrong")), tok, 8770).map(|r| r.0), Some(401));
        let mut q = req(None, None);
        q.query.insert("token".into(), tok.into());
        assert_eq!(refuse(&q, tok, 8770), None, "token in the query string");
        let a = new_token();
        assert_eq!(a.len(), 32);
        assert_ne!(a, new_token());
    }
}
