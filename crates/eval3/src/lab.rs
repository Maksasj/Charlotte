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

// ---------------------------------------------------------------------------- live server
//
// `sla3 lab [--port 8733]`: binds 127.0.0.1 only. GET /api/login?token=T sets an HttpOnly,
// SameSite=Strict cookie (EventSource cannot send headers); the token is in
// data/training/lab3/token (created on first start). GET /api/runs lists runs;
// GET /api/stream?run=R streams events as Server-Sent Events, resuming after Last-Event-ID,
// pushing new events as the runner appends them. H3 runs only ever log counts.

use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};

fn token() -> String {
    let p = dir().join("token");
    if let Ok(t) = std::fs::read_to_string(&p) {
        return t.trim().to_string();
    }
    let seed = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0);
    let t = format!("{:x}{:x}", seed, std::process::id() as u128 * 0x9e3779b97f4a7c15);
    let _ = std::fs::create_dir_all(dir());
    let _ = std::fs::write(&p, &t);
    t
}

fn reply(s: &mut TcpStream, status: &str, ctype: &str, extra: &str, body: &str) {
    let _ = write!(s, "HTTP/1.1 {status}\r\nContent-Type: {ctype}\r\nContent-Length: {}\r\n{extra}Connection: close\r\n\r\n{body}", body.len());
}

fn query(path: &str, k: &str) -> Option<String> {
    path.split_once('?')?.1.split('&').find_map(|kv| kv.split_once('=').filter(|(a, _)| *a == k).map(|(_, v)| v.to_string()))
}

fn handle(mut s: TcpStream, tok: &str) {
    let mut rd = BufReader::new(s.try_clone().unwrap());
    let mut first = String::new();
    if rd.read_line(&mut first).is_err() {
        return;
    }
    let mut cookie_ok = false;
    let mut last_id: Option<u64> = None;
    loop {
        let mut h = String::new();
        if rd.read_line(&mut h).is_err() || h.trim().is_empty() {
            break;
        }
        let low = h.to_ascii_lowercase();
        if low.starts_with("cookie:") && h.contains(&format!("lab3={tok}")) {
            cookie_ok = true;
        }
        if low.starts_with("last-event-id:") {
            last_id = h.split_once(':').and_then(|(_, v)| v.trim().parse().ok());
        }
    }
    let path = first.split_whitespace().nth(1).unwrap_or("/").to_string();
    let route = path.split('?').next().unwrap_or("/");
    match route {
        "/" => reply(&mut s, "200 OK", "text/html; charset=utf-8", "", PAGE),
        "/api/login" => {
            if query(&path, "token").as_deref() == Some(tok) {
                let c = format!("Set-Cookie: lab3={tok}; HttpOnly; SameSite=Strict; Path=/\r\n");
                reply(&mut s, "200 OK", "text/plain", &c, "ok");
            } else {
                reply(&mut s, "403 Forbidden", "text/plain", "", "bad token");
            }
        }
        _ if !cookie_ok => reply(&mut s, "401 Unauthorized", "text/plain", "", "login first"),
        "/api/runs" => {
            let mut runs: Vec<String> = std::fs::read_dir(dir()).map(|r| r.flatten().filter(|e| e.path().is_dir()).map(|e| e.file_name().to_string_lossy().to_string()).collect()).unwrap_or_default();
            runs.sort();
            reply(&mut s, "200 OK", "application/json", "", &serde_json::to_string(&runs).unwrap());
        }
        "/api/stream" => {
            let run = query(&path, "run").unwrap_or_default();
            if run.is_empty() || run.contains('/') || run.contains("..") {
                return reply(&mut s, "400 Bad Request", "text/plain", "", "run?");
            }
            let after = last_id.or_else(|| query(&path, "after").and_then(|v| v.parse().ok()));
            let _ = write!(s, "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nCache-Control: no-cache\r\nConnection: keep-alive\r\n\r\n");
            let file = dir().join(&run).join("events.jsonl");
            let mut sent: usize = after.map(|a| a as usize + 1).unwrap_or(0);
            loop {
                let lines: Vec<String> = std::fs::read_to_string(&file).map(|t| t.lines().map(String::from).collect()).unwrap_or_default();
                for (k, l) in lines.iter().enumerate().skip(sent) {
                    if write!(s, "id: {k}\ndata: {l}\n\n").is_err() {
                        return;
                    }
                }
                sent = sent.max(lines.len());
                if s.flush().is_err() || write!(s, ": keep-alive\n\n").is_err() {
                    return;
                }
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        }
        _ => reply(&mut s, "404 Not Found", "text/plain", "", "no"),
    }
}

pub fn serve(port: u16) -> std::io::Result<()> {
    let tok = token();
    let l = TcpListener::bind(("127.0.0.1", port))?;
    eprintln!("lab3: http://127.0.0.1:{port}/  (login: /api/login?token={tok})");
    for s in l.incoming().flatten() {
        let t = tok.clone();
        std::thread::spawn(move || handle(s, &t));
    }
    Ok(())
}

const PAGE: &str = r#"<!doctype html><meta charset="utf-8"><title>Charlotte 3.0 lab</title>
<style>body{font:14px system-ui;margin:16px;background:#fafafa;color:#222}table{border-collapse:collapse}td,th{padding:3px 8px;border-bottom:1px solid #ddd;text-align:right}th:first-child,td:first-child{text-align:left}#log{font:12px monospace;white-space:pre;max-height:50vh;overflow:auto;background:#fff;border:1px solid #ddd;padding:6px}</style>
<h1>Charlotte 3.0 training lab</h1><p>Run: <select id="run"></select> <span id="st"></span></p>
<table id="t"><thead><tr><th>stage / kind</th><th>n</th><th>pass</th><th>rate</th></tr></thead><tbody></tbody></table><h3>Events</h3><div id="log"></div>
<script>
const counts={};let es;
function row(){const tb=document.querySelector('#t tbody');tb.innerHTML='';for(const[k,v]of Object.entries(counts).sort()){tb.insertAdjacentHTML('beforeend',`<tr><td>${k}</td><td>${v.n}</td><td>${v.p}</td><td>${(v.p/v.n*100).toFixed(1)}%</td></tr>`)}}
function open_(r){if(es)es.close();for(const k in counts)delete counts[k];document.getElementById('log').textContent='';row();
 es=new EventSource('/api/stream?run='+encodeURIComponent(r));es.onopen=()=>st.textContent='live';es.onerror=()=>st.textContent='reconnecting';
 es.onmessage=m=>{const e=JSON.parse(m.data);const p=e.payload||{};const k=e.stage+(p.kind?' / '+p.kind:'');const c=counts[k]||(counts[k]={n:0,p:0});c.n++;if(p.pass)c.p++;row();
  const L=document.getElementById('log');L.textContent+=JSON.stringify(e)+'\n';L.scrollTop=L.scrollHeight}}
fetch('/api/runs').then(r=>r.json()).then(rs=>{const s=document.getElementById('run');rs.forEach(r=>s.insertAdjacentHTML('beforeend',`<option>${r}</option>`));s.onchange=()=>open_(s.value);if(rs.length){s.value=rs[rs.length-1];open_(s.value)}}).catch(()=>st.textContent='log in first: /api/login?token=...')
</script>"#;
