use crate::state::{
    AddFactReq, AddPatternReq, AppState, ChatReq, FeedbackReq, MetaReq, StateResponse, TeachReq,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use karin_core::{Engine, Pattern, SelfTestResult, list_tools};
use serde_json::{Value, json};
use std::sync::Arc;

pub async fn health() -> Json<Value> {
    Json(json!({ "ok": true }))
}

pub async fn get_state(State(app): State<Arc<AppState>>) -> Json<StateResponse> {
    Json(StateResponse {
        state: app.snapshot(),
        tools: list_tools(),
    })
}

pub async fn chat(State(app): State<Arc<AppState>>, Json(req): Json<ChatReq>) -> Json<Value> {
    let (trace, state) = app.mutate(|eng| eng.turn(&req.input));
    Json(json!({ "trace": trace, "state": state }))
}

pub async fn new_chat(State(app): State<Arc<AppState>>) -> Json<Value> {
    let (_, state) = app.mutate(Engine::new_chat);
    Json(json!({ "state": state }))
}

pub async fn teach(State(app): State<Arc<AppState>>, Json(req): Json<TeachReq>) -> Json<Value> {
    let ((episode, pattern), state) =
        app.mutate(|eng| eng.teach_reply(&req.input, &req.reply, req.promote));
    Json(json!({ "episode": episode, "pattern": pattern, "state": state }))
}

pub async fn feedback(
    State(app): State<Arc<AppState>>,
    Json(req): Json<FeedbackReq>,
) -> Json<Value> {
    let (ok, state) = app.mutate(|eng| eng.feedback(req.turn, req.positive));
    Json(json!({ "ok": ok, "state": state }))
}

pub async fn add_fact(
    State(app): State<Arc<AppState>>,
    Json(req): Json<AddFactReq>,
) -> Json<Value> {
    let (fact, state) =
        app.mutate(|eng| eng.add_fact(req.rel, req.args, req.scope, req.provenance));
    Json(json!({ "fact": fact, "state": state }))
}

pub async fn delete_fact(
    State(app): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let state = app
        .try_mutate(|eng| eng.delete_fact(&id))
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(json!({ "state": state })))
}

pub async fn add_pattern(
    State(app): State<Arc<AppState>>,
    Json(req): Json<AddPatternReq>,
) -> Json<Value> {
    let (pattern, state) = app.mutate(|eng| {
        eng.add_pattern(Pattern {
            id: String::new(),
            name: req.name,
            status: req.status,
            when: req.when,
            then: req.then,
            trust: 0.9,
            support: 1,
            provenance: "manual".into(),
            kind: req.kind,
        })
    });
    Json(json!({ "pattern": pattern, "state": state }))
}

pub async fn promote_pattern(
    State(app): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let state = app
        .try_mutate(|eng| eng.promote_pattern(&id))
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(json!({ "state": state })))
}

pub async fn delete_pattern(
    State(app): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let state = app
        .try_mutate(|eng| eng.delete_pattern(&id))
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(json!({ "state": state })))
}

pub async fn meta_generalize(
    State(app): State<Arc<AppState>>,
    Json(req): Json<MetaReq>,
) -> Json<Value> {
    let ((created, summary), state) = app.mutate(|eng| eng.generalize_episodes(req.auto_promote));
    Json(json!({ "created": created, "summary": summary, "state": state }))
}

pub async fn meta_chunk(State(app): State<Arc<AppState>>, Json(req): Json<MetaReq>) -> Json<Value> {
    let ((created, summary), state) = app.mutate(|eng| eng.chunk_last_chain(req.auto_promote));
    Json(json!({ "created": created, "summary": summary, "state": state }))
}

pub async fn reset(State(app): State<Arc<AppState>>) -> Json<Value> {
    let vault_dir = app.vault_dir.clone();
    let (_, state) = app.mutate(|eng| {
        if let Err(err) = eng.reset_from_vault_dir(&vault_dir) {
            eprintln!("warning: failed to reload vault core: {err}");
        }
    });
    Json(json!({ "state": state }))
}

pub async fn selftest() -> Json<Vec<SelfTestResult>> {
    Json(Engine::run_selftests())
}
