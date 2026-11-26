mod handlers;
mod incognito;
mod state;
mod vault_watch;

use axum::{
    Router,
    routing::{delete, get, post},
};
use karin_core::{
    Engine,
    store::{load_state, next_chat_id},
    vault::default_vault_dir,
};
use state::AppState;
use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let port = read_port();
    let state_path = read_state_path();
    let vault_dir = default_vault_dir();
    let chats_dir = vault_dir
        .parent()
        .unwrap_or_else(|| Path::new("../vault"))
        .join(".chats");
    let chat_id = next_chat_id(&chats_dir);
    println!("Using vault core from {}", vault_dir.display());
    println!("Saving chats under {}", chats_dir.display());

    let engine = load_or_init_engine(&state_path, &vault_dir);
    let shared = Arc::new(AppState {
        engine: Mutex::new(engine),
        state_path,
        vault_dir,
        chats_dir,
        chat_id: Mutex::new(chat_id),
        incognito: Default::default(),
        vault_status: Default::default(),
    });

    // The saved state may predate vault edits: merge the current vault in now,
    // then keep watching it for changes.
    vault_watch::reload(&shared, "startup");
    vault_watch::spawn(shared.clone());

    let app = build_router(shared);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Karin Rust API listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn read_port() -> u16 {
    std::env::var("PORT")
        .ok()
        .and_then(|val| val.parse().ok())
        .unwrap_or(8080)
}

fn read_state_path() -> PathBuf {
    PathBuf::from(std::env::var("STATE_PATH").unwrap_or_else(|_| "../vault/state.json".to_string()))
}

fn load_or_init_engine(state_path: &Path, vault_dir: &Path) -> Engine {
    match load_state(state_path) {
        Ok(Some(saved)) => {
            println!("Loaded state from {}", state_path.display());
            Engine::from_state(saved)
        }
        Ok(None) => {
            println!(
                "Initialized fresh symbolic core from {}",
                vault_dir.display()
            );
            Engine::from_vault_dir(vault_dir).expect("failed to load vault core")
        }
        Err(err) => {
            eprintln!("Failed to load state ({err}); starting from vault core");
            Engine::from_vault_dir(vault_dir).expect("failed to load vault core")
        }
    }
}

fn build_router(shared: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/health", get(handlers::health))
        .route("/api/state", get(handlers::get_state))
        .route("/api/chat", post(handlers::chat))
        .route("/api/chat/new", post(handlers::new_chat))
        .route("/api/teach", post(handlers::teach))
        .route("/api/feedback", post(handlers::feedback))
        .route("/api/facts", post(handlers::add_fact))
        .route("/api/facts/{id}", delete(handlers::delete_fact))
        .route("/api/patterns", post(handlers::add_pattern))
        .route(
            "/api/patterns/{id}/promote",
            post(handlers::promote_pattern),
        )
        .route("/api/patterns/{id}", delete(handlers::delete_pattern))
        .route("/api/meta/generalize", post(handlers::meta_generalize))
        .route("/api/meta/chunk", post(handlers::meta_chunk))
        .route("/api/reset", post(handlers::reset))
        .route("/api/vault/status", get(vault_watch::status))
        .route("/api/vault/reload", post(vault_watch::reload_now))
        .route("/api/incognito", post(incognito::create))
        .route("/api/incognito/{id}", delete(incognito::delete))
        .route("/api/incognito/{id}/chat", post(incognito::chat))
        .route("/api/incognito/{id}/new-chat", post(incognito::new_chat))
        .route("/api/incognito/{id}/state", get(incognito::state))
        .layer(cors)
        .with_state(shared)
}
