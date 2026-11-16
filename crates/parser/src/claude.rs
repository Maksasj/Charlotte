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
}

fn record(role: &str, u: &Usage) {
    GLOBAL.lock().unwrap().entry(role.to_string()).or_default().add(u);
    LOCAL.with(|l| l.borrow_mut().entry(role.to_string()).or_default().add(u));
}

/// Usage of all calls so far, by role.
pub fn global_usage() -> BTreeMap<String, Usage> {
    GLOBAL.lock().unwrap().clone()
}

/// Usage of this thread's calls since the last take, by role (and reset).
pub fn take_thread_usage() -> BTreeMap<String, Usage> {
    LOCAL.with(|l| std::mem::take(&mut *l.borrow_mut()))
}

const SYSTEM: &str = "You are a careful assistant inside a training program. Follow the user's instructions exactly and output only what they ask for.";

/// One model call. Returns the text of the answer.
pub fn call(role: &str, model: &str, prompt: &str) -> Result<String, String> {
    let start = std::time::Instant::now();
    let child = Command::new("claude")
        .args(["-p", "--model", model, "--output-format", "json", "--system-prompt", SYSTEM, "--tools", "", "--no-session-persistence", "--strict-mcp-config"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();
    let out = child.and_then(|mut c| {
        c.stdin.take().unwrap().write_all(prompt.as_bytes())?;
        c.wait_with_output()
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
