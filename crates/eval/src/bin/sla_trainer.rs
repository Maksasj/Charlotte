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
    println!("Training app for {}\nOpen http://localhost:{port}  (training starts only when you press Start)", vault.display());
    if let Err(e) = serve(Trainer::new(vault), ui, port) {
        eprintln!("cannot serve on port {port}: {e}");
        std::process::exit(1);
    }
}
