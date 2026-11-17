//! The event stream: every stage of a turn, and the trainer, publish events
//! here. Subscribers (the training dashboard) read them by sequence number;
//! with a file attached, every event is also appended to disk as one JSON
//! line, so a finished run can be replayed.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Event {
    /// 1, 2, 3, ... in publication order.
    pub seq: u64,
    /// Seconds since the Unix epoch.
    pub at: f64,
    /// `turn`, `run_start`, `run_end`, `conversation_start`, `conversation_end`, `alert`, ...
    pub kind: String,
    pub data: serde_json::Value,
}

struct Inner {
    next: u64,
    ring: VecDeque<Event>,
    cap: usize,
    file: Option<File>,
}

pub struct EventBus {
    inner: Mutex<Inner>,
}

impl Default for EventBus {
    fn default() -> Self {
        EventBus::new(5000)
    }
}

pub fn now_s() -> f64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs_f64()).unwrap_or(0.0)
}

impl EventBus {
    /// Keep the last `cap` events in memory.
    pub fn new(cap: usize) -> Self {
        EventBus { inner: Mutex::new(Inner { next: 1, ring: VecDeque::new(), cap: cap.max(1), file: None }) }
    }

    /// Also append every event to `path` (JSON lines).
    pub fn attach_file(&self, path: &Path) -> std::io::Result<()> {
        if let Some(d) = path.parent() {
            std::fs::create_dir_all(d)?;
        }
        let f = std::fs::OpenOptions::new().create(true).append(true).open(path)?;
        self.lock().file = Some(f);
        Ok(())
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Inner> {
        self.inner.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub fn publish(&self, kind: &str, data: impl Serialize) -> u64 {
        let data = serde_json::to_value(data).unwrap_or(serde_json::Value::Null);
        let mut g = self.lock();
        let e = Event { seq: g.next, at: now_s(), kind: kind.to_string(), data };
        g.next += 1;
        if let Some(f) = g.file.as_mut() {
            if let Ok(mut line) = serde_json::to_string(&e) {
                line.push('\n');
                // One write per event: lines never interleave.
                let _ = f.write_all(line.as_bytes());
            }
        }
        if g.ring.len() == g.cap {
            g.ring.pop_front();
        }
        let seq = e.seq;
        g.ring.push_back(e);
        seq
    }

    /// Events with `seq > after`, oldest first, at most `max`.
    pub fn since(&self, after: u64, max: usize) -> Vec<Event> {
        self.lock().ring.iter().filter(|e| e.seq > after).take(max).cloned().collect()
    }

    /// The sequence number of the newest event (0 if none).
    pub fn last_seq(&self) -> u64 {
        self.lock().next - 1
    }
}

/// Read an event file written by an attached bus.
pub fn read_file(path: &Path) -> std::io::Result<Vec<Event>> {
    let src = std::fs::read_to_string(path)?;
    Ok(src.lines().filter_map(|l| serde_json::from_str(l).ok()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_and_file() {
        let dir = std::env::temp_dir().join(format!("sla_events_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let bus = EventBus::new(3);
        bus.attach_file(&dir.join("events.jsonl")).unwrap();
        for i in 0..5 {
            bus.publish("tick", serde_json::json!({ "i": i }));
        }
        assert_eq!(bus.last_seq(), 5);
        let recent = bus.since(0, 10);
        assert_eq!(recent.iter().map(|e| e.seq).collect::<Vec<_>>(), vec![3, 4, 5], "only the last 3 kept in memory");
        assert_eq!(bus.since(4, 10).len(), 1);
        let all = read_file(&dir.join("events.jsonl")).unwrap();
        assert_eq!(all.len(), 5, "the file keeps everything");
        assert_eq!(all[4].data["i"], 4);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
