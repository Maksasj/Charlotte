//! Training app backend: N Haiku teachers hold small conversations with the
//! agent on the main vault, teaching it as they go. A tiny HTTP server serves
//! the UI and the live state; training is started and stopped from the UI.

use crate::pretrain::review_and_fix;
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
}

pub struct Trainer {
    pub vault: PathBuf,
    state: Mutex<TrainState>,
    stop: AtomicBool,
    next: AtomicUsize,
    active: AtomicUsize,
    /// Serializes the agent's own (fast) updates to the shared vault.
    agent_lock: Mutex<()>,
    vault_cache: Mutex<(Instant, VaultCounts)>,
    usage_at_start: Mutex<Usage>,
}

fn total(m: &BTreeMap<String, Usage>) -> Usage {
    let mut t = Usage::default();
    for u in m.values() {
        t.add(u);
    }
    t
}

fn count_vault(root: &std::path::Path) -> VaultCounts {
    let mut c = VaultCounts::default();
    if let Ok(rd) = std::fs::read_dir(root.join("facts")) {
        for e in rd.flatten() {
            c.entities += 1;
            if let Ok(text) = std::fs::read_to_string(e.path()) {
                for line in text.lines().filter(|l| l.contains("@source")) {
                    if line.contains("@until") {
                        c.closed += 1;
                    } else {
                        c.facts += 1;
                    }
                }
            }
        }
    }
    if let Ok(rd) = std::fs::read_dir(root.join("rules")) {
        for e in rd.flatten() {
            if let Ok(text) = std::fs::read_to_string(e.path()) {
                if text.contains("\nstatus: confirmed") {
                    c.rules_confirmed += 1;
                } else if text.contains("\nstatus: hypothesis") {
                    c.rules_hypothesis += 1;
                } else if text.contains("\nstatus: rejected") {
                    c.rules_rejected += 1;
                }
            }
        }
    }
    c
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

impl Trainer {
    pub fn new(vault: PathBuf) -> Arc<Self> {
        let state = TrainState { vault_path: vault.display().to_string(), scenarios: SCENARIOS.len(), ..Default::default() };
        Arc::new(Trainer {
            vault,
            state: Mutex::new(state),
            stop: AtomicBool::new(false),
            next: AtomicUsize::new(0),
            active: AtomicUsize::new(0),
            agent_lock: Mutex::new(()),
            vault_cache: Mutex::new((Instant::now() - std::time::Duration::from_secs(60), VaultCounts::default())),
            usage_at_start: Mutex::new(Usage::default()),
        })
    }

    /// A snapshot of the state for the UI.
    pub fn snapshot(&self) -> TrainState {
        let vault = {
            let mut c = self.vault_cache.lock().unwrap();
            if c.0.elapsed().as_secs() >= 3 {
                *c = (Instant::now(), count_vault(&self.vault));
            }
            c.1.clone()
        };
        let mut s = self.state.lock().unwrap().clone();
        s.now = now_s();
        s.usage = global_usage();
        s.usage_total = total(&s.usage);
        let base = self.usage_at_start.lock().unwrap().clone();
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
        s
    }

    pub fn start(self: &Arc<Self>, cfg: TrainConfig) -> Result<(), String> {
        let cfg = TrainConfig { teachers: cfg.teachers.clamp(1, 24), turns: cfg.turns.clamp(2, 30), conversations: cfg.conversations.clamp(1, 1000), ..cfg };
        {
            let mut s = self.state.lock().unwrap();
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
        *self.usage_at_start.lock().unwrap() = total(&global_usage());
        self.stop.store(false, Ordering::SeqCst);
        self.next.store(0, Ordering::SeqCst);
        self.active.store(cfg.teachers, Ordering::SeqCst);
        for id in 0..cfg.teachers {
            let me = Arc::clone(self);
            let cfg = cfg.clone();
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
                    me.conversation(id, n, scenario, &cfg);
                }
                me.update(id, |t| t.status = if me.stop.load(Ordering::SeqCst) { "stopped".into() } else { "done".into() });
                if me.active.fetch_sub(1, Ordering::SeqCst) == 1 {
                    let mut s = me.state.lock().unwrap();
                    s.running = false;
                    s.stopping = false;
                    s.finished_at = Some(now_s());
                }
            });
        }
        Ok(())
    }

    pub fn request_stop(&self) {
        self.stop.store(true, Ordering::SeqCst);
        let mut s = self.state.lock().unwrap();
        if s.running {
            s.stopping = true;
        }
    }

    fn update(&self, id: usize, f: impl FnOnce(&mut TeacherView)) {
        let mut s = self.state.lock().unwrap();
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
        let m = Msg { role: role.into(), text: text.into(), at: now_s() };
        self.update(id, |t| t.transcript.push(m));
    }

    /// One training step: a small conversation, then the review.
    fn conversation(&self, id: usize, n: usize, scenario: &str, cfg: &TrainConfig) {
        self.update(id, |t| {
            t.status = "thinking".into();
            t.scenario = scenario.into();
            t.conversation = Some(n + 1);
            t.turn = 0;
            t.transcript.clear();
            t.last_error = None;
        });
        // The teacher is its own speaker: its personal remarks are not about the user.
        let mut agent = {
            let _g = self.agent_lock.lock().unwrap();
            match Agent::open(&self.vault, "friend") {
                Ok(a) => a,
                Err(e) => {
                    self.update(id, |t| {
                        t.status = "error".into();
                        t.last_error = Some(e.to_string());
                    });
                    return;
                }
            }
        };
        agent.source = "teacher:train".into();
        agent.enable_llm(&cfg.model);
        let mut new_facts: Vec<GroundAtom> = Vec::new();
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
            self.push(id, "teacher", &msg);
            transcript.push(Msg { role: "teacher".into(), text: msg.clone(), at: now_s() });
            self.update(id, |t| t.status = "agent".into());
            // Translation runs here in parallel; only the agent's own update is serialized.
            agent.prepare(&msg);
            let (reply, facts_added, rules_added) = {
                let _g = self.agent_lock.lock().unwrap();
                let _ = agent.reload_if_edited();
                let facts_before: Vec<GroundAtom> = agent.kb.all_facts().into_iter().filter(|f| f.is_open()).map(|f| f.atom).collect();
                let rules_before: Vec<String> = agent.kb.vault.rules.keys().cloned().collect();
                let reply = agent.respond(&msg);
                let fa: Vec<GroundAtom> = agent.kb.all_facts().into_iter().filter(|f| f.is_open() && f.atom.pred != "proper_name").map(|f| f.atom).filter(|a| !facts_before.contains(a)).collect();
                let ra: Vec<String> = agent.kb.vault.rules.keys().filter(|k| !rules_before.contains(k)).cloned().collect();
                (reply, fa, ra)
            };
            turns += 1;
            if agent.last_understood {
                understood += 1;
            }
            {
                let mut s = self.state.lock().unwrap();
                s.turns += 1;
                if agent.last_understood {
                    s.understood += 1;
                }
                s.facts += facts_added.len();
                s.rules += rules_added.len();
            }
            self.update(id, |t| {
                t.facts += facts_added.len();
                t.rules += rules_added.len();
            });
            new_facts.extend(facts_added);
            new_rules.extend(rules_added);
            self.push(id, "agent", &reply.text);
            transcript.push(Msg { role: "agent".into(), text: reply.text.clone(), at: now_s() });
            self.absorb_usage(id);
            if done {
                break;
            }
        }
        // Personal remarks of the teacher's character are conversation, not general knowledge.
        let personal = |g: &GroundAtom| g.args.iter().any(|v| v.as_sym().map_or(false, |x| x == "friend" || x.starts_with("my_")));
        if new_facts.iter().any(|f| personal(f)) {
            let _g = self.agent_lock.lock().unwrap();
            let _ = agent.reload_if_edited();
            let now = agent.now();
            for f in new_facts.iter().filter(|f| personal(f)) {
                agent.kb.close_matching(&f.to_atom(), now, "personal remark in a training chat, not general knowledge");
            }
            let _ = agent.save();
        }
        new_facts.retain(|f| !personal(f));
        let mut rejected = Vec::new();
        if cfg.review && !self.stop.load(Ordering::SeqCst) {
            self.update(id, |t| t.status = "reviewing".into());
            review_and_fix(&mut agent, &self.agent_lock, &new_facts, &new_rules, &cfg.model, &mut rejected);
            self.absorb_usage(id);
        }
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
        let mut s = self.state.lock().unwrap();
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

fn handle(trainer: &Arc<Trainer>, ui: &PathBuf, mut stream: TcpStream) {
    let mut reader = BufReader::new(match stream.try_clone() {
        Ok(s) => s,
        Err(_) => return,
    });
    let mut first = String::new();
    if reader.read_line(&mut first).is_err() {
        return;
    }
    let mut len = 0usize;
    loop {
        let mut h = String::new();
        if reader.read_line(&mut h).is_err() || h == "\r\n" || h.is_empty() {
            break;
        }
        if let Some(v) = h.to_ascii_lowercase().strip_prefix("content-length:") {
            len = v.trim().parse().unwrap_or(0);
        }
    }
    let mut body = vec![0u8; len.min(1 << 20)];
    let _ = reader.read_exact(&mut body);
    let mut parts = first.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("/").split('?').next().unwrap_or("/");
    match (method, path) {
        ("GET", "/") => match std::fs::read(ui) {
            Ok(page) => respond(&mut stream, 200, "text/html; charset=utf-8", &page),
            Err(e) => respond(&mut stream, 404, "text/plain", format!("UI file {} not found: {e}", ui.display()).as_bytes()),
        },
        ("GET", "/api/state") => {
            let s = serde_json::to_vec(&trainer.snapshot()).unwrap_or_default();
            respond(&mut stream, 200, "application/json", &s);
        }
        ("POST", "/api/start") => {
            let cfg: TrainConfig = serde_json::from_slice(&body).unwrap_or_default();
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
    for stream in listener.incoming().flatten() {
        let t = Arc::clone(&trainer);
        let ui = ui.clone();
        std::thread::spawn(move || handle(&t, &ui, stream));
    }
    Ok(())
}
