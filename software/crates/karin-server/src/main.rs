use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    net::SocketAddr,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use karin_core::{
    Action, AgentState, Condition, Engine, Pattern, PatternKind, PatternStatus, Scope,
    SelfTestResult, ToolInfo, list_tools,
    store::{load_state, save_state},
};
use tower_http::cors::{Any, CorsLayer};

struct AppState {
    engine: Mutex<Engine>,
    state_path: PathBuf,
}

impl AppState {
    fn persist(&self, state: &AgentState) {
        if let Err(err) = save_state(&self.state_path, state) {
            eprintln!("warning: failed to save state: {err}");
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct StateResponse {
    state: AgentState,
    tools: Vec<ToolInfo>,
}

#[derive(Deserialize)]
struct ChatReq {
    input: String,
}

#[derive(Deserialize)]
struct TeachReq {
    input: String,
    reply: String,
    #[serde(default = "default_true")]
    promote: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Deserialize)]
struct FeedbackReq {
    turn: usize,
    positive: bool,
}

#[derive(Deserialize)]
struct AddFactReq {
    rel: String,
    #[serde(default)]
    args: BTreeMap<String, String>,
    #[serde(default = "default_long_term")]
    scope: Scope,
    #[serde(default = "default_manual")]
    provenance: String,
}

fn default_long_term() -> Scope {
    Scope::LongTerm
}

fn default_manual() -> String {
    "manual".into()
}

#[derive(Deserialize)]
struct AddPatternReq {
    name: String,
    when: Vec<Condition>,
    then: Vec<Action>,
    #[serde(default = "default_active")]
    status: PatternStatus,
    #[serde(default = "default_reply_kind")]
    kind: PatternKind,
}

fn default_active() -> PatternStatus {
    PatternStatus::Active
}

fn default_reply_kind() -> PatternKind {
    PatternKind::Reply
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct MetaReq {
    #[serde(default)]
    auto_promote: bool,
}

#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(8080);
    let state_path = PathBuf::from(
        std::env::var("STATE_PATH").unwrap_or_else(|_| "../vault/state.json".to_string()),
    );

    let engine = match load_state(&state_path) {
        Ok(Some(saved)) => {
            println!("Loaded state from {}", state_path.display());
            Engine::from_state(saved)
        }
        Ok(None) => {
            println!("Initialized fresh pretrained symbolic core");
            Engine::new()
        }
        Err(err) => {
            eprintln!("Failed to load state ({err}); starting fresh");
            Engine::new()
        }
    };

    let shared = Arc::new(AppState {
        engine: Mutex::new(engine),
        state_path,
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/state", get(get_state))
        .route("/api/chat", post(chat))
        .route("/api/teach", post(teach))
        .route("/api/feedback", post(feedback))
        .route("/api/facts", post(add_fact))
        .route("/api/facts/{id}", delete(delete_fact))
        .route("/api/patterns", post(add_pattern))
        .route("/api/patterns/{id}/promote", post(promote_pattern))
        .route("/api/patterns/{id}", delete(delete_pattern))
        .route("/api/meta/generalize", post(meta_generalize))
        .route("/api/meta/chunk", post(meta_chunk))
        .route("/api/reset", post(reset))
        .route("/api/selftest", get(selftest))
        .layer(cors)
        .with_state(shared);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Karin Rust API listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "ok": true }))
}

async fn get_state(State(app): State<Arc<AppState>>) -> Json<StateResponse> {
    let eng = app.engine.lock().unwrap();
    Json(StateResponse {
        state: eng.state.clone(),
        tools: list_tools(),
    })
}

async fn chat(
    State(app): State<Arc<AppState>>,
    Json(req): Json<ChatReq>,
) -> Json<serde_json::Value> {
    let mut eng = app.engine.lock().unwrap();
    let trace = eng.turn(&req.input);
    let snapshot = eng.state.clone();
    drop(eng);
    app.persist(&snapshot);
    Json(serde_json::json!({
        "trace": trace,
        "state": snapshot,
    }))
}

async fn teach(
    State(app): State<Arc<AppState>>,
    Json(req): Json<TeachReq>,
) -> Json<serde_json::Value> {
    let mut eng = app.engine.lock().unwrap();
    let (episode, pattern) = eng.teach_reply(&req.input, &req.reply, req.promote);
    let snapshot = eng.state.clone();
    drop(eng);
    app.persist(&snapshot);
    Json(serde_json::json!({
        "episode": episode,
        "pattern": pattern,
        "state": snapshot,
    }))
}

async fn feedback(
    State(app): State<Arc<AppState>>,
    Json(req): Json<FeedbackReq>,
) -> Json<serde_json::Value> {
    let mut eng = app.engine.lock().unwrap();
    let ok = eng.feedback(req.turn, req.positive);
    let snapshot = eng.state.clone();
    drop(eng);
    app.persist(&snapshot);
    Json(serde_json::json!({
        "ok": ok,
        "state": snapshot,
    }))
}

async fn add_fact(
    State(app): State<Arc<AppState>>,
    Json(req): Json<AddFactReq>,
) -> Json<serde_json::Value> {
    let mut eng = app.engine.lock().unwrap();
    let fact = eng.add_fact(req.rel, req.args, req.scope, req.provenance);
    let snapshot = eng.state.clone();
    drop(eng);
    app.persist(&snapshot);
    Json(serde_json::json!({
        "fact": fact,
        "state": snapshot,
    }))
}

async fn delete_fact(
    State(app): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut eng = app.engine.lock().unwrap();
    if !eng.delete_fact(&id) {
        return Err(StatusCode::NOT_FOUND);
    }
    let snapshot = eng.state.clone();
    drop(eng);
    app.persist(&snapshot);
    Ok(Json(serde_json::json!({ "state": snapshot })))
}

async fn add_pattern(
    State(app): State<Arc<AppState>>,
    Json(req): Json<AddPatternReq>,
) -> Json<serde_json::Value> {
    let mut eng = app.engine.lock().unwrap();
    let pattern = eng.add_pattern(Pattern {
        id: String::new(),
        name: req.name,
        status: req.status,
        when: req.when,
        then: req.then,
        trust: 0.9,
        support: 1,
        provenance: "manual".into(),
        kind: req.kind,
    });
    let snapshot = eng.state.clone();
    drop(eng);
    app.persist(&snapshot);
    Json(serde_json::json!({
        "pattern": pattern,
        "state": snapshot,
    }))
}

async fn promote_pattern(
    State(app): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut eng = app.engine.lock().unwrap();
    if !eng.promote_pattern(&id) {
        return Err(StatusCode::NOT_FOUND);
    }
    let snapshot = eng.state.clone();
    drop(eng);
    app.persist(&snapshot);
    Ok(Json(serde_json::json!({ "state": snapshot })))
}

async fn delete_pattern(
    State(app): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut eng = app.engine.lock().unwrap();
    if !eng.delete_pattern(&id) {
        return Err(StatusCode::NOT_FOUND);
    }
    let snapshot = eng.state.clone();
    drop(eng);
    app.persist(&snapshot);
    Ok(Json(serde_json::json!({ "state": snapshot })))
}

async fn meta_generalize(
    State(app): State<Arc<AppState>>,
    Json(req): Json<MetaReq>,
) -> Json<serde_json::Value> {
    let mut eng = app.engine.lock().unwrap();
    let (created, summary) = eng.generalize_episodes(req.auto_promote);
    let snapshot = eng.state.clone();
    drop(eng);
    app.persist(&snapshot);
    Json(serde_json::json!({
        "created": created,
        "summary": summary,
        "state": snapshot,
    }))
}

async fn meta_chunk(
    State(app): State<Arc<AppState>>,
    Json(req): Json<MetaReq>,
) -> Json<serde_json::Value> {
    let mut eng = app.engine.lock().unwrap();
    let (created, summary) = eng.chunk_last_chain(req.auto_promote);
    let snapshot = eng.state.clone();
    drop(eng);
    app.persist(&snapshot);
    Json(serde_json::json!({
        "created": created,
        "summary": summary,
        "state": snapshot,
    }))
}

async fn reset(State(app): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let mut eng = app.engine.lock().unwrap();
    eng.reset();
    let snapshot = eng.state.clone();
    drop(eng);
    app.persist(&snapshot);
    Json(serde_json::json!({ "state": snapshot }))
}

async fn selftest() -> Json<Vec<SelfTestResult>> {
    Json(Engine::run_selftests())
}
