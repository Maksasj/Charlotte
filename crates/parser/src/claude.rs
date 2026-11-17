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
    let mocked = mock().is_some();
    let out = call_system_inner(role, model, system, prompt);
    if !mocked {
        log_call(role, model, system, prompt, &out, start.elapsed().as_secs_f64());
    }
    out
}

static CALL_LOG: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);

/// Log every real model call (role, model, the full prompt and answer, time)
/// as one JSON line in `dir/<date>.jsonl`. System prompts are stored once, by
/// hash, in `dir/systems.jsonl`.
pub fn set_call_log(dir: std::path::PathBuf) {
    let _ = std::fs::create_dir_all(&dir);
    *CALL_LOG.lock().unwrap_or_else(|e| e.into_inner()) = Some(dir);
}

fn fnv(s: &str) -> String {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in s.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("{h:016x}")
}

fn log_call(role: &str, model: &str, system: &str, prompt: &str, out: &Result<String, String>, seconds: f64) {
    let guard = CALL_LOG.lock().unwrap_or_else(|e| e.into_inner());
    let Some(dir) = guard.as_ref() else { return };
    use std::io::Write;
    let sys_hash = fnv(system);
    let sys_file = dir.join("systems.jsonl");
    let known = std::fs::read_to_string(&sys_file).map_or(false, |t| t.contains(&sys_hash));
    if !known {
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&sys_file) {
            let _ = writeln!(f, "{}", serde_json::json!({"hash": sys_hash, "system": system}));
        }
    }
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let day = {
        // Civil date from Unix days (UTC).
        let z = (now / 86400) as i64 + 719468;
        let era = z.div_euclid(146097);
        let doe = z - era * 146097;
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = doy - (153 * mp + 2) / 5 + 1;
        let m = if mp < 10 { mp + 3 } else { mp - 9 };
        let y = yoe + era * 400 + (m <= 2) as i64;
        format!("{y:04}-{m:02}-{d:02}")
    };
    let line = serde_json::json!({
        "unix": now, "role": role, "model": model, "system": sys_hash, "prompt": prompt,
        "ok": out.is_ok(), "answer": out.as_ref().map(String::as_str).unwrap_or_else(|e| e.as_str()), "seconds": seconds,
    });
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(dir.join(format!("{day}.jsonl"))) {
        let _ = writeln!(f, "{line}");
    }
}

fn call_system_inner(role: &str, model: &str, system: &str, prompt: &str) -> Result<String, String> {
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
