//! Hot reload: watch the vault and merge changes into the live engine.
//!
//! A background thread fingerprints the vault's Markdown files (core,
//! `generated-*`, `.episodes`) once a second. When the fingerprint changes and
//! then stays the same for a moment (editors write in several steps), the vault
//! is reloaded with `Engine::reload_vault`: vault rules and facts are replaced,
//! everything learned at runtime is kept. A file that fails to parse leaves the
//! previous rules active and is reported in `/api/vault/status`.
//!
//!   GET  /api/vault/status   -> { version, ok, message, reloadedAt, addedRules, removedRules, ... }
//!   POST /api/vault/reload   -> reload now, same shape

use crate::state::AppState;
use axum::{Json, extract::State};
use karin_core::{VaultReload, store::save_state, vault::vault_fingerprint};
use serde::Serialize;
use std::{
    sync::Arc,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const POLL: Duration = Duration::from_millis(1000);
const SETTLE: Duration = Duration::from_millis(400);

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultStatus {
    /// Increases on every reload attempt (successful or not).
    pub version: u64,
    pub ok: bool,
    pub message: String,
    /// Seconds since the Unix epoch.
    pub reloaded_at: u64,
    pub rules: usize,
    pub facts: usize,
    pub added_rules: Vec<String>,
    pub removed_rules: Vec<String>,
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Reload the vault into the live engine and record the outcome.
pub fn reload(app: &AppState, reason: &str) -> VaultStatus {
    let result = {
        let mut engine = app.engine.lock().unwrap();
        let result = engine.reload_vault(&app.vault_dir);
        if result.is_ok() {
            if let Err(err) = save_state(&app.state_path, &engine.state) {
                eprintln!("warning: failed to save state after vault reload: {err}");
            }
        }
        result
    };
    let mut status = app.vault_status.lock().unwrap();
    status.version += 1;
    status.reloaded_at = now();
    match result {
        Ok(VaultReload {
            facts,
            patterns,
            added_rules,
            removed_rules,
            facts_delta,
        }) => {
            status.ok = true;
            status.message = format!(
                "{reason}: {patterns} rules, {facts} facts ({:+} rules, {:+} facts)",
                added_rules.len() as i64 - removed_rules.len() as i64,
                facts_delta
            );
            status.rules = patterns;
            status.facts = facts;
            status.added_rules = added_rules;
            status.removed_rules = removed_rules;
        }
        Err(err) => {
            status.ok = false;
            status.message = format!("{reason}: vault not reloaded, previous rules kept — {err}");
            status.added_rules.clear();
            status.removed_rules.clear();
        }
    }
    println!("vault: {}", status.message);
    status.clone()
}

/// Start the watcher thread.
pub fn spawn(app: Arc<AppState>) {
    thread::spawn(move || {
        let mut last = vault_fingerprint(&app.vault_dir);
        loop {
            thread::sleep(POLL);
            let seen = vault_fingerprint(&app.vault_dir);
            if seen == last {
                continue;
            }
            // Wait until the files stop changing before reloading.
            thread::sleep(SETTLE);
            if vault_fingerprint(&app.vault_dir) != seen {
                continue;
            }
            last = seen;
            reload(&app, "vault changed");
        }
    });
}

pub async fn status(State(app): State<Arc<AppState>>) -> Json<VaultStatus> {
    Json(app.vault_status.lock().unwrap().clone())
}

pub async fn reload_now(State(app): State<Arc<AppState>>) -> Json<VaultStatus> {
    Json(reload(&app, "reloaded on request"))
}
