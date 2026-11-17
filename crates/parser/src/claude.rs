//! Headless Claude CLI calls with usage accounting.
//!
//! Every call runs `claude -p` with a short system prompt of our own, no
//! tools and no session (the default Claude Code system prompt would add
//! ~25k cached tokens to each call), asks for JSON output, and records the
//! call's tokens and cost under a role ("teacher", "translator", ...), both
//! globally and for the calling thread.

use serde::Serialize;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::Write;
use std::process::{Command, Stdio};
use std::sync::Mutex;

/// Tokens and cost of one or more calls.
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct Usage {
    pub calls: u64,
    pub errors: u64,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_write_tokens: u64,
    pub cache_read_tokens: u64,
    pub cost_usd: f64,
    /// Wall-clock seconds spent waiting for the model.
    pub seconds: f64,
}

impl Usage {
    pub fn add(&mut self, o: &Usage) {
        self.calls += o.calls;
        self.errors += o.errors;
        self.input_tokens += o.input_tokens;
        self.output_tokens += o.output_tokens;
        self.cache_write_tokens += o.cache_write_tokens;
        self.cache_read_tokens += o.cache_read_tokens;
        self.cost_usd += o.cost_usd;
        self.seconds += o.seconds;
    }
}

static GLOBAL: Mutex<BTreeMap<String, Usage>> = Mutex::new(BTreeMap::new());

thread_local! {
    static LOCAL: RefCell<BTreeMap<String, Usage>> = RefCell::new(BTreeMap::new());
    /// Calls made by this thread, never reset (for per-turn counts).
    static CALLS: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
    static THREAD_MOCK: RefCell<Option<Mock>> = const { RefCell::new(None) };
}

/// A stand-in for the model in tests: (role, prompt) -> answer.
pub type Mock = std::sync::Arc<dyn Fn(&str, &str) -> Result<String, String> + Send + Sync>;

static GLOBAL_MOCK: Mutex<Option<Mock>> = Mutex::new(None);

/// Answer this thread's model calls with `f` instead of the Claude CLI.
pub fn set_thread_mock(f: Mock) {
    THREAD_MOCK.with(|m| *m.borrow_mut() = Some(f));
}

/// Answer every thread's model calls with `f` (for code that spawns threads).
/// A thread mock takes precedence.
pub fn set_global_mock(f: Mock) {
    *GLOBAL_MOCK.lock().unwrap_or_else(|e| e.into_inner()) = Some(f);
}

/// Remove this thread's mock only (a global mock stays).
pub fn clear_thread_mock() {
    THREAD_MOCK.with(|m| *m.borrow_mut() = None);
}

/// Remove this thread's mock and the global one.
pub fn clear_mocks() {
    THREAD_MOCK.with(|m| *m.borrow_mut() = None);
    *GLOBAL_MOCK.lock().unwrap_or_else(|e| e.into_inner()) = None;
}

fn mock() -> Option<Mock> {
    THREAD_MOCK.with(|m| m.borrow().clone()).or_else(|| GLOBAL_MOCK.lock().unwrap_or_else(|e| e.into_inner()).clone())
}

/// Model calls made by the current thread so far.
pub fn thread_calls() -> u64 {
    CALLS.with(|c| c.get())
}

fn record(role: &str, u: &Usage) {
    CALLS.with(|c| c.set(c.get() + u.calls));
    GLOBAL.lock().unwrap_or_else(|e| e.into_inner()).entry(role.to_string()).or_default().add(u);
    LOCAL.with(|l| l.borrow_mut().entry(role.to_string()).or_default().add(u));
}

/// Usage of all calls so far, by role.
pub fn global_usage() -> BTreeMap<String, Usage> {
    GLOBAL.lock().unwrap_or_else(|e| e.into_inner()).clone()
}

/// Usage of this thread's calls since the last take, by role (and reset).
pub fn take_thread_usage() -> BTreeMap<String, Usage> {
    LOCAL.with(|l| std::mem::take(&mut *l.borrow_mut()))
}

/// Longest wait for one model call.
pub const TIMEOUT: std::time::Duration = std::time::Duration::from_secs(90);

const SYSTEM: &str = "You are a careful assistant inside a training program. Follow the user's instructions exactly and output only what they ask for.";

/// One model call. Returns the text of the answer.
pub fn call(role: &str, model: &str, prompt: &str) -> Result<String, String> {
    call_system(role, model, SYSTEM, prompt)
}

/// One model call with a system prompt of the caller's. A long, unchanging
/// system prompt (the translator's language specification) is cached by the
/// API across calls, so only the short user part is new each time.
pub fn call_system(role: &str, model: &str, system: &str, prompt: &str) -> Result<String, String> {
    let start = std::time::Instant::now();
    if let Some(f) = mock() {
        let out = f(role, prompt);
        let u = Usage { calls: 1, errors: out.is_err() as u64, seconds: start.elapsed().as_secs_f64(), ..Default::default() };
        record(role, &u);
        return out;
    }
    let child = Command::new("claude")
        .args(["-p", "--model", model, "--output-format", "json", "--system-prompt", system, "--tools", "", "--no-session-persistence", "--strict-mcp-config"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();
    // A hung CLI process must not block a run forever: give up after TIMEOUT.
    let out = child.and_then(|mut c| {
        c.stdin.take().unwrap().write_all(prompt.as_bytes())?;
        let mut stdout = c.stdout.take().unwrap();
        let mut stderr = c.stderr.take().unwrap();
        let reader = std::thread::spawn(move || {
            use std::io::Read;
            let mut o = Vec::new();
            let mut e = Vec::new();
            let _ = stdout.read_to_end(&mut o);
            let _ = stderr.read_to_end(&mut e);
            (o, e)
        });
        let deadline = std::time::Instant::now() + TIMEOUT;
        let status = loop {
            if let Some(s) = c.try_wait()? {
                break s;
            }
            if std::time::Instant::now() >= deadline {
                let _ = c.kill();
                let _ = c.wait();
                return Err(std::io::Error::new(std::io::ErrorKind::TimedOut, format!("no answer within {} s", TIMEOUT.as_secs())));
            }
            std::thread::sleep(std::time::Duration::from_millis(20));
        };
        let (stdout, stderr) = reader.join().unwrap_or_default();
        Ok(std::process::Output { status, stdout, stderr })
    });
    let mut u = Usage { calls: 1, seconds: start.elapsed().as_secs_f64(), ..Default::default() };
    let out = match out {
        Ok(o) => o,
        Err(e) => {
            u.errors = 1;
            record(role, &u);
            return Err(format!("cannot run claude: {e}"));
        }
    };
    let text = String::from_utf8_lossy(&out.stdout);
    let v: serde_json::Value = match serde_json::from_str(text.trim()) {
        Ok(v) => v,
        Err(_) => {
            u.errors = 1;
            record(role, &u);
            let err = String::from_utf8_lossy(&out.stderr);
            return Err(format!("unreadable claude output: {} {}", text.chars().take(200).collect::<String>(), err.chars().take(200).collect::<String>()));
        }
    };
    let n = |p: &str| v.pointer(p).and_then(|x| x.as_u64()).unwrap_or(0);
    u.input_tokens = n("/usage/input_tokens");
    u.output_tokens = n("/usage/output_tokens");
    u.cache_write_tokens = n("/usage/cache_creation_input_tokens");
    u.cache_read_tokens = n("/usage/cache_read_input_tokens");
    u.cost_usd = v.get("total_cost_usd").and_then(|x| x.as_f64()).unwrap_or(0.0);
    let is_error = v.get("is_error").and_then(|x| x.as_bool()).unwrap_or(false) || !out.status.success();
    if is_error {
        u.errors = 1;
    }
    record(role, &u);
    let result = v.get("result").and_then(|x| x.as_str()).unwrap_or("").to_string();
    if is_error {
        return Err(if result.is_empty() { "claude reported an error".into() } else { result });
    }
    Ok(result)
}
