//! Training app: serves the training UI and runs Haiku teachers on demand.
//!
//! ```text
//! cargo run --release -p sla_eval --bin sla_trainer -- [--vault DIR] [--port 8770] [--ui FILE]
//! ```
//! Then open http://localhost:8770 and press Start. Nothing trains until then.

use sla_eval::trainer::{serve, Trainer};

fn main() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..");
    let root = root.canonicalize().unwrap_or(root);
    let mut vault = root.join("vault");
    let mut port: u16 = 8770;
    let mut ui = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("ui").join("trainer.html");
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--vault" => vault = args.next().expect("--vault DIR").into(),
            "--port" => port = args.next().and_then(|p| p.parse().ok()).expect("--port N"),
            "--ui" => ui = args.next().expect("--ui FILE").into(),
            _ => {
                eprintln!("usage: sla_trainer [--vault DIR] [--port N] [--ui FILE]");
                std::process::exit(2);
            }
        }
    }
    let trainer = Trainer::new(vault);
    println!(
        "Training app for {}\nOpen http://localhost:{port}/?token={}  (the token is required; training starts only when you press Start)",
        trainer.vault.display(),
        trainer.token
    );
    if let Err(e) = serve(trainer, ui, port) {
        eprintln!("cannot serve on port {port}: {e}");
        std::process::exit(1);
    }
}
