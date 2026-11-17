//! The training-data archive: every real conversation the project generates
//! (trainer teachers, Haiku judges) is appended here as one JSON line, and
//! never deleted. It lives in `data/training/<kind>/<date>.jsonl` and is
//! versioned with the code, so it can be reused later (learned patterns,
//! judge calibration, regression sets). Runs on mock models are synthetic
//! and are not archived.

use serde::Serialize;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

static LOCK: Mutex<()> = Mutex::new(());

/// The archive directory of a repository root.
pub fn dir(root: &Path) -> PathBuf {
    root.join("data").join("training")
}

/// Append one record to `<archive>/<kind>/<today>.jsonl`.
pub fn append(archive: &Path, kind: &str, record: &impl Serialize) -> std::io::Result<PathBuf> {
    let day = sla_core::Timestamp::now().to_string()[..10].to_string();
    let d = archive.join(kind);
    std::fs::create_dir_all(&d)?;
    let path = d.join(format!("{day}.jsonl"));
    let mut line = serde_json::to_string(record).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    line.push('\n');
    let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    std::fs::OpenOptions::new().create(true).append(true).open(&path)?.write_all(line.as_bytes())?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    #[test]
    fn appends_lines() {
        let d = std::env::temp_dir().join(format!("sla_archive_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        let p = super::append(&d, "trainer", &serde_json::json!({"a": 1})).unwrap();
        super::append(&d, "trainer", &serde_json::json!({"a": 2})).unwrap();
        assert_eq!(std::fs::read_to_string(&p).unwrap().lines().count(), 2);
        let _ = std::fs::remove_dir_all(&d);
    }
}
