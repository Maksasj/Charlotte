//! The HTTP API described in software/docs/api.md.

use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use charlotte_engine::learn::{Corpus, LearnOptions, Report};
use charlotte_engine::{ConvState, Engine, Verdict};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Serialize)]
struct Job {
    id: String,
    state: String,
    stage: String,
    source: String,
    report: Option<Report>,
    error: Option<String>,
}
struct App {
    engine: Engine,
    vault: String,
    convs: Mutex<Vec<ConvState>>,
    jobs: Mutex<Vec<Job>>,
}
type S = State<Arc<App>>;
type Fail = (StatusCode, Json<Value>);
type Res<T> = Result<Json<T>, Fail>;

fn fail(code: StatusCode, msg: impl Into<String>) -> Fail {
    (code, Json(json!({ "error": msg.into() })))
}
/// A request body that is not the JSON the endpoint expects.
type Body<T> = Result<Json<T>, JsonRejection>;
fn body<T>(b: Body<T>) -> Result<T, Fail> {
    b.map(|Json(x)| x).map_err(|e| fail(StatusCode::BAD_REQUEST, e.body_text()))
}
fn bad(msg: String) -> Fail {
    let code = if msg.starts_with("no item") { StatusCode::NOT_FOUND } else { StatusCode::BAD_REQUEST };
    fail(code, msg)
}

async fn new_conv(State(app): S) -> (StatusCode, Json<Value>) {
    let mut convs = app.convs.lock().unwrap();
    let id = format!("c{}", convs.len() + 1);
    convs.push(ConvState { id: id.clone(), ..Default::default() });
    (StatusCode::CREATED, Json(json!({ "id": id })))
}
async fn list_convs(State(app): S) -> Json<Value> {
    let convs = app.convs.lock().unwrap();
    Json(Value::Array(
        convs.iter().rev().map(|c| json!({ "id": c.id, "turns": c.messages.len(), "closed": c.closed, "verdict": c.verdict, "last": c.messages.last().map(|m| m.text.clone()) })).collect(),
    ))
}
async fn get_conv(State(app): S, Path(id): Path<String>) -> Res<ConvState> {
    let convs = app.convs.lock().unwrap();
    convs.iter().find(|c| c.id == id).cloned().map(Json).ok_or_else(|| fail(StatusCode::NOT_FOUND, format!("no conversation '{id}'")))
}
#[derive(Deserialize)]
struct Say {
    text: String,
}
async fn say(State(app): S, Path(id): Path<String>, b: Body<Say>) -> Res<charlotte_core::Reply> {
    let body = body(b)?;
    let mut convs = app.convs.lock().unwrap();
    let conv = convs.iter_mut().find(|c| c.id == id).ok_or_else(|| fail(StatusCode::NOT_FOUND, format!("no conversation '{id}'")))?;
    if conv.closed {
        return Err(fail(StatusCode::CONFLICT, "the conversation is closed"));
    }
    if body.text.trim().is_empty() {
        return Err(fail(StatusCode::BAD_REQUEST, "the message is empty"));
    }
    Ok(Json(app.engine.say(conv, body.text.trim())))
}
#[derive(Deserialize)]
struct Close {
    verdict: Verdict,
}
async fn close(State(app): S, Path(id): Path<String>, b: Body<Close>) -> Res<Value> {
    let body = body(b)?;
    let mut convs = app.convs.lock().unwrap();
    let conv = convs.iter_mut().find(|c| c.id == id).ok_or_else(|| fail(StatusCode::NOT_FOUND, format!("no conversation '{id}'")))?;
    let updated = app.engine.close(conv, body.verdict).map_err(|e| fail(StatusCode::CONFLICT, e))?;
    Ok(Json(json!({ "updated": updated })))
}

async fn items(State(app): S, Query(q): Query<HashMap<String, String>>) -> Json<Value> {
    let get = |k: &str| q.get(k).map(String::as_str).filter(|s| !s.is_empty() && *s != "all");
    Json(json!(app.engine.items(get("kind"), get("status"), get("q"))))
}
async fn item(State(app): S, Path(id): Path<String>) -> Res<charlotte_engine::Item> {
    app.engine.item(&id).map(Json).ok_or_else(|| fail(StatusCode::NOT_FOUND, format!("no item '{id}'")))
}
#[derive(Deserialize)]
struct Edit {
    front: Map<String, Value>,
    body: String,
}
async fn edit(State(app): S, Path(id): Path<String>, b: Body<Edit>) -> Res<charlotte_engine::Item> {
    let body = body(b)?;
    app.engine.edit(&id, body.front, body.body).map(Json).map_err(bad)
}
async fn promote(State(app): S, Path(id): Path<String>) -> Res<charlotte_engine::Item> {
    app.engine.promote(&id).map(Json).map_err(bad)
}
async fn reject(State(app): S, Path(id): Path<String>) -> Res<Value> {
    app.engine.reject(&id).map(|_| Json(json!({ "ok": true }))).map_err(bad)
}

#[derive(Deserialize)]
struct Learn {
    path: Option<String>,
    jsonl: Option<String>,
    auto_promote: Option<bool>,
}
async fn learn(State(app): S, b: Body<Learn>) -> Result<(StatusCode, Json<Value>), Fail> {
    let body = body(b)?;
    let corpus = match (&body.path, &body.jsonl) {
        (Some(p), _) if !p.trim().is_empty() => Corpus::open(p.trim()).map_err(|e| fail(StatusCode::BAD_REQUEST, e))?,
        (_, Some(j)) if !j.trim().is_empty() => Corpus::parse("uploaded", j),
        _ => return Err(fail(StatusCode::BAD_REQUEST, "give a corpus path or JSON lines")),
    };
    if corpus.convs.is_empty() {
        return Err(fail(StatusCode::BAD_REQUEST, "no conversations could be read from that corpus"));
    }
    let id = {
        let mut jobs = app.jobs.lock().unwrap();
        let id = format!("j{}", jobs.len() + 1);
        jobs.push(Job { id: id.clone(), state: "running".into(), stage: "ingest".into(), source: corpus.source.clone(), report: None, error: None });
        id
    };
    let (app2, id2) = (app.clone(), id.clone());
    std::thread::spawn(move || {
        let update = |f: &dyn Fn(&mut Job)| {
            if let Some(j) = app2.jobs.lock().unwrap().iter_mut().find(|j| j.id == id2) {
                f(j)
            }
        };
        let opts = LearnOptions { auto_promote: body.auto_promote.unwrap_or(true), ..Default::default() };
        match app2.engine.learn(&corpus, &opts, &|stage| update(&|j| j.stage = stage.to_string())) {
            Ok(report) => update(&|j| {
                j.state = "done".into();
                j.stage = "done".into();
                j.report = Some(report.clone());
            }),
            Err(e) => update(&|j| {
                j.state = "failed".into();
                j.error = Some(e.clone());
            }),
        }
    });
    Ok((StatusCode::ACCEPTED, Json(json!({ "job": id }))))
}
async fn jobs(State(app): S) -> Json<Value> {
    Json(json!(app.jobs.lock().unwrap().iter().rev().cloned().collect::<Vec<_>>()))
}
async fn job(State(app): S, Path(id): Path<String>) -> Res<Job> {
    app.jobs.lock().unwrap().iter().find(|j| j.id == id).cloned().map(Json).ok_or_else(|| fail(StatusCode::NOT_FOUND, format!("no job '{id}'")))
}
async fn health(State(app): S) -> Json<Value> {
    let (items, candidates) = app.engine.counts();
    let vault = std::fs::canonicalize(&app.vault).map(|p| p.to_string_lossy().to_string()).unwrap_or_else(|_| app.vault.clone());
    Json(json!({ "version": env!("CARGO_PKG_VERSION"), "vault": vault, "items": items, "candidates": candidates, "conversations": app.convs.lock().unwrap().len() }))
}

pub fn serve(engine: Engine, vault: &str, port: u16, web: &str) -> Result<(), String> {
    let app = Arc::new(App { engine, vault: vault.to_string(), convs: Mutex::new(Vec::new()), jobs: Mutex::new(Vec::new()) });
    let mut router = Router::new()
        .route("/v1/conversations", post(new_conv).get(list_convs))
        .route("/v1/conversations/{id}", get(get_conv))
        .route("/v1/conversations/{id}/messages", post(say))
        .route("/v1/conversations/{id}/close", post(close))
        .route("/v1/knowledge", get(items))
        .route("/v1/knowledge/{id}", get(item).put(edit))
        .route("/v1/knowledge/{id}/promote", post(promote))
        .route("/v1/knowledge/{id}/reject", post(reject))
        .route("/v1/learn", post(learn).get(jobs))
        .route("/v1/learn/{id}", get(job))
        .route("/v1/health", get(health))
        .with_state(app);
    let has_web = std::path::Path::new(web).join("index.html").is_file();
    if has_web {
        router = router.fallback_service(tower_http::services::ServeDir::new(web));
    }
    let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
    rt.block_on(async move {
        let addr = format!("127.0.0.1:{port}");
        let listener = tokio::net::TcpListener::bind(&addr).await.map_err(|e| format!("{addr}: {e}"))?;
        println!("charlotte is serving http://{addr}  (vault: {vault}{})", if has_web { format!(", web app: {web}") } else { ", no web build found".into() });
        axum::serve(listener, router).with_graceful_shutdown(async { tokio::signal::ctrl_c().await.ok(); }).await.map_err(|e| e.to_string())
    })
}
