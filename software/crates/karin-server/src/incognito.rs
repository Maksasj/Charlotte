//! Incognito sessions: throwaway engines loaded fresh from the vault.
//!
//! Nothing an incognito session does is saved (no state.json) or logged (no
//! vault/.chats). They exist for benchmarks and experiments that must not touch
//! the live engine. Sessions are kept in memory, capped in number; the least
//! recently used one is dropped when the cap is reached.
//!
//!   POST   /api/incognito                 -> { "id": "..." }
//!   POST   /api/incognito/{id}/chat       { "input": "..." } -> { "trace": TurnTrace }
//!   POST   /api/incognito/{id}/new-chat   -> { "ok": true }
//!   GET    /api/incognito/{id}/state      -> { "state": AgentState }
//!   DELETE /api/incognito/{id}            -> 204

use crate::state::{AppState, ChatReq};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use karin_core::Engine;
use serde_json::{Value, json};
use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    time::{Instant, SystemTime, UNIX_EPOCH},
};

const MAX_SESSIONS: usize = 32;

#[derive(Default)]
pub struct Incognito {
    sessions: Mutex<HashMap<String, (Engine, Instant)>>,
    counter: AtomicU64,
}

impl Incognito {
    fn new_id(&self) -> String {
        let n = self.counter.fetch_add(1, Ordering::Relaxed);
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(0);
        format!("inc-{n}-{nanos:08x}")
    }

    fn with<T>(&self, id: &str, f: impl FnOnce(&mut Engine) -> T) -> Option<T> {
        let mut sessions = self.sessions.lock().unwrap();
        let (engine, used) = sessions.get_mut(id)?;
        *used = Instant::now();
        Some(f(engine))
    }
}

pub async fn create(State(app): State<Arc<AppState>>) -> Result<Json<Value>, (StatusCode, String)> {
    let engine = Engine::from_vault_dir(&app.vault_dir)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("load vault: {e}")))?;
    let id = app.incognito.new_id();
    let mut sessions = app.incognito.sessions.lock().unwrap();
    if sessions.len() >= MAX_SESSIONS
        && let Some(oldest) = sessions
            .iter()
            .min_by_key(|(_, (_, used))| *used)
            .map(|(k, _)| k.clone())
    {
        sessions.remove(&oldest);
    }
    sessions.insert(id.clone(), (engine, Instant::now()));
    Ok(Json(json!({ "id": id })))
}

pub async fn chat(
    State(app): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<ChatReq>,
) -> Result<Json<Value>, StatusCode> {
    let trace = app
        .incognito
        .with(&id, |eng| eng.turn(&req.input))
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(json!({ "trace": trace })))
}

pub async fn new_chat(
    State(app): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    app.incognito
        .with(&id, Engine::new_chat)
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(json!({ "ok": true })))
}

pub async fn state(
    State(app): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let state = app
        .incognito
        .with(&id, |eng| eng.state.clone())
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(json!({ "state": state })))
}

pub async fn delete(State(app): State<Arc<AppState>>, Path(id): Path<String>) -> StatusCode {
    match app.incognito.sessions.lock().unwrap().remove(&id) {
        Some(_) => StatusCode::NO_CONTENT,
        None => StatusCode::NOT_FOUND,
    }
}
