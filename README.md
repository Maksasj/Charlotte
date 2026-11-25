# Karin

A general-purpose, purely symbolic (non-neural) assistant built on one uniform **facts + patterns** core.

## Architecture

- **`backend/crates/core` (`karin-core`)** — Pure Rust symbolic engine (`Perceive → Match → Select → Fire`) with `?var` unification, working vs. long-term fact scopes, uniform world tools + meta-tools, impasse teaching, anti-unification generalization, chain chunking, and JSON persistence.
- **`backend/crates/server` (`karin-server`)** — Rust Axum HTTP API (`http://127.0.0.1:8080`) exposing chat, trace inspection, teaching, facts/patterns CRUD, meta-tools, and live self-tests.
- **`frontend/`** — Vite + React + TypeScript UI with hot module reload (`http://localhost:5173`), proxying `/api` to the Rust backend.

## Running Locally

### 1. Start the Rust backend
```bash
cargo run -p karin-server
```
(Listens on `127.0.0.1:8080` by default; override with `PORT=8080` or `STATE_PATH=data/state.json`.)

### 2. Start the Vite React frontend (with HMR)
```bash
cd frontend
npm install
npm run dev
```
Open `http://localhost:5173`.

### 3. Run Rust tests
```bash
cargo test
```
