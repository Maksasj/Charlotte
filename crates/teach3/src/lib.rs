//! Charlotte 3.0 teacher channel: cached, budgeted Haiku calls (decisions D3, D11, D16).
//!
//! Shares its archive with `tools/design3/haiku.py`: `data/training/calls3/cache.jsonl`
//! (key = sha256 of the Python `json.dumps([model, system, prompt])`), and the ledger of live
//! calls. A cached answer costs nothing, so every call is reusable. Fallback parses are
//! appended to `data/training/parser3/pairs.jsonl` as training pairs for the learned parser.

pub mod sha256;

use kb3::Fact;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::Mutex;

pub const BUDGET: usize = 5000;

/// Set when the CLI reports a plan usage limit: later calls fail fast instead of being sent.
pub static LIMIT_HIT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub fn limit_hit() -> bool {
    LIMIT_HIT.load(std::sync::atomic::Ordering::SeqCst)
}
pub const MODEL: &str = "haiku";

/// Python's `json.dumps` of a list of strings (ASCII-escaped, ", " separators).
pub fn py_dumps(items: &[&str]) -> String {
    let mut out = String::from("[");
    for (k, s) in items.iter().enumerate() {
        if k > 0 {
            out.push_str(", ");
        }
        out.push('"');
        for c in s.chars() {
            match c {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\t' => out.push_str("\\t"),
                '\u{08}' => out.push_str("\\b"),
                '\u{0c}' => out.push_str("\\f"),
                c if (c as u32) < 0x20 || (c as u32) > 0x7e => {
                    let mut buf = [0u16; 2];
                    for u in c.encode_utf16(&mut buf) {
                        out.push_str(&format!("\\u{:04x}", u));
                    }
                }
                c => out.push(c),
            }
        }
        out.push('"');
    }
    out.push(']');
    out
}

pub fn key(model: &str, system: &str, prompt: &str) -> String {
    sha256::hex(py_dumps(&[model, system, prompt]).as_bytes())
}

fn dir() -> PathBuf {
    kb3::repo_root().join("data/training/calls3")
}

pub struct Teacher {
    cache: Mutex<HashMap<String, String>>,
    /// when false, a cache miss is an error instead of a live call (replays, tests)
    pub live: bool,
}

impl Teacher {
    pub fn open(live: bool) -> Teacher {
        let mut cache = HashMap::new();
        if let Ok(t) = std::fs::read_to_string(dir().join("cache.jsonl")) {
            for line in t.lines() {
                if let Ok(v) = serde_json::from_str::<Value>(line) {
                    if v["ok"].as_bool() == Some(true) {
                        if let (Some(k), Some(a)) = (v["key"].as_str(), v["answer"].as_str()) {
                            cache.insert(k.to_string(), a.to_string());
                        }
                    }
                }
            }
        }
        Teacher { cache: Mutex::new(cache), live }
    }

    /// Successful live calls (refusals such as plan-limit errors cost no model work).
    pub fn live_calls() -> usize {
        std::fs::read_to_string(dir().join("ledger.jsonl"))
            .map(|t| t.lines().filter(|l| serde_json::from_str::<Value>(l).ok().and_then(|v| v["ok"].as_bool()) == Some(true)).count())
            .unwrap_or(0)
    }

    pub fn cached(&self, system: &str, prompt: &str) -> Option<String> {
        self.cache.lock().unwrap().get(&key(MODEL, system, prompt)).cloned()
    }

    pub fn ask(&self, system: &str, prompt: &str, tag: &str) -> Result<String, String> {
        let k = key(MODEL, system, prompt);
        if let Some(a) = self.cache.lock().unwrap().get(&k) {
            return Ok(a.clone());
        }
        if !self.live {
            return Err("not cached and live calls are off".into());
        }
        if LIMIT_HIT.load(std::sync::atomic::Ordering::SeqCst) {
            return Err("plan usage limit reached earlier in this run".into());
        }
        if Self::live_calls() >= BUDGET {
            return Err(format!("{BUDGET} live Haiku calls used; ask the user before going past it"));
        }
        let t0 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        let start = std::time::Instant::now();
        let out = Command::new("claude")
            .args(["-p", "--model", MODEL, "--output-format", "json", "--system-prompt", system, "--tools", "", "--no-session-persistence", "--strict-mcp-config"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .and_then(|mut c| {
                c.stdin.take().unwrap().write_all(prompt.as_bytes())?;
                c.wait_with_output()
            });
        let (answer, ok) = match out {
            Ok(o) => match serde_json::from_slice::<Value>(&o.stdout) {
                Ok(v) => {
                    let a = v["result"].as_str().unwrap_or("").to_string();
                    let ok = !v["is_error"].as_bool().unwrap_or(false) && !a.is_empty();
                    (a, ok)
                }
                Err(e) => (format!("error: {e}"), false),
            },
            Err(e) => (format!("error: {e}"), false),
        };
        let rec = json!({"key": k, "unix": t0, "tag": tag, "model": MODEL, "system": system, "prompt": prompt,
                         "answer": answer, "ok": ok, "seconds": (start.elapsed().as_secs_f64() * 100.0).round() / 100.0});
        let _ = std::fs::create_dir_all(dir());
        append(&dir().join("cache.jsonl"), &rec.to_string());
        append(&dir().join("ledger.jsonl"), &json!({"unix": t0, "tag": tag, "ok": ok}).to_string());
        if ok {
            self.cache.lock().unwrap().insert(k, answer.clone());
            Ok(answer)
        } else {
            if answer.contains("limit") {
                LIMIT_HIT.store(true, std::sync::atomic::Ordering::SeqCst);
            }
            Err(answer)
        }
    }

    /// Parse one teaching line into schema facts with Haiku (the parser's fallback). Valid facts
    /// only; the pair is stored as training data for the learned parser.
    pub fn parse_line(&self, task: &str, line: &str, known_items: &[String]) -> Result<Vec<Fact>, String> {
        let prompt = format!(
            "Task id: {task}\nSteps are named {task}_s<N> where N is the step number the line gives.\nItems already known for this task: {}\nLine: {line}\n",
            if known_items.is_empty() { "-".to_string() } else { known_items.join(", ") }
        );
        let answer = self.ask(PARSE_SYSTEM, &prompt, "parse_fallback")?;
        let facts = facts_in(&answer);
        if facts.is_empty() {
            return Err(format!("no valid facts in fallback answer: {answer}"));
        }
        store_pair(task, line, &facts);
        Ok(facts)
    }

    /// The teacher answers a question about a task from the task's own facts (gap questions).
    pub fn answer(&self, task_facts: &[Fact], situation: &[Fact], question: &str) -> Result<String, String> {
        let facts: Vec<String> = task_facts.iter().chain(situation.iter()).map(|f| f.to_string()).collect();
        let prompt = format!("Facts you know:\n{}\n\nThe learner asks: {question}\n", facts.join("\n"));
        self.ask(TEACHER_SYSTEM, &prompt, "teacher_answer")
    }
}

static APPEND: Mutex<()> = Mutex::new(());

/// One append per record, in a single write (O_APPEND), serialised within the process.
fn append(path: &std::path::Path, line: &str) {
    let _g = APPEND.lock().unwrap_or_else(|e| e.into_inner());
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let mut buf = line.as_bytes().to_vec();
        buf.push(b'\n');
        let _ = f.write_all(&buf);
    }
}

/// Valid schema facts in a model answer (one per line; other text ignored).
pub fn facts_in(answer: &str) -> Vec<Fact> {
    answer
        .lines()
        .map(|l| l.trim().trim_start_matches("- ").trim_matches('`').trim())
        .filter(|l| l.ends_with(')') || l.ends_with(").") || l.ends_with(");"))
        .map(|l| format!("{}.", l.trim_end_matches(['.', ';'])))
        .filter_map(|l| Fact::parse(&l).ok())
        .filter(|f| f.valid().is_ok())
        .collect()
}

pub fn store_pair(task: &str, line: &str, facts: &[Fact]) {
    let path = kb3::repo_root().join("data/training/parser3/pairs.jsonl");
    let _ = std::fs::create_dir_all(path.parent().unwrap());
    let fs: Vec<Vec<String>> = facts.iter().map(|f| std::iter::once(f.pred.clone()).chain(f.args.iter().cloned()).collect()).collect();
    append(&path, &json!({"task": task, "line": line, "facts": fs}).to_string());
}

pub const PARSE_SYSTEM: &str = "You convert one line of household teaching into facts. Reply with facts only, one per line, no prose, no code fences. \
Use only these predicates: needs(Task, Item). takes_time(TaskOrStep, Min, Max, Unit). step(Task, Step). action(Step, Verb). object_of(Step, Thing). \
uses(Step, Item). role(Step, Item, Function). into(Step, Place). onto(Step, Place). from(Step, Place). until(Step, State). \
overlaps(Step, OtherStep). setting(Step, Attribute, Value, Unit). amount(Step, Item, Quantity, Unit). purpose(Step, Effect). \
condition(Step, Condition). warning(Step, Hazard). check(Step, State). optional(Step). \
Symbols are lower case with underscores, singular nouns, base-form verbs; numbers as digits; units singular (minute, hour, celsius, gram). \
One action per step. For an item list line, give needs facts; for a total time line, give takes_time on the task.";

pub const TEACHER_SYSTEM: &str = "You are a patient teacher of household tasks. Answer the learner's question in plain short sentences, \
using only the facts you are given. If the facts do not answer it, say what you would do instead in one or two steps, \
in the same style: 'Step N of <task>: <action> the <object> with the <item>.'";
