//! Training lab event log (Design 3.0 section 4.1, `lab3`): append-only
//! `data/training/lab3/<run>/events.jsonl`, each event with a monotonic id. The SSE server
//! (`sla3 lab`) streams these files.

use serde_json::{json, Value};
use std::io::Write;
use std::path::PathBuf;

pub struct Log {
    path: PathBuf,
    run: String,
    next: u64,
}

pub fn dir() -> PathBuf {
    kb3::repo_root().join("data/training/lab3")
}

impl Log {
    pub fn open(run: &str) -> Log {
        let d = dir().join(run);
        let _ = std::fs::create_dir_all(&d);
        let path = d.join("events.jsonl");
        let next = std::fs::read_to_string(&path).map(|t| t.lines().count() as u64).unwrap_or(0);
        Log { path, run: run.into(), next }
    }

    pub fn event(&mut self, stage: &str, payload: Value) {
        let unix = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
        let e = json!({"id": self.next, "run": self.run, "unix_ms": unix, "stage": stage, "payload": payload});
        self.next += 1;
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&self.path) {
            let _ = writeln!(f, "{e}");
        }
    }
}
