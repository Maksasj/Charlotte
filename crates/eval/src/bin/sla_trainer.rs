//! Training app: serves the training UI and runs Haiku teachers on demand.
//!
//! ```text
//! cargo run --release -p sla_eval --bin sla_trainer -- [--vault DIR] [--port 8770] [--ui FILE] [--mock]
//! ```
//! Then open http://localhost:8770 and press Start. Nothing trains until then.

use sla_eval::trainer::{serve, Trainer};

fn main() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..");
    let root = root.canonicalize().unwrap_or(root);
    sla_parser::claude::set_call_log(root.join("data").join("training").join("calls"));
    let mut vault = root.join("vault");
    let mut port: u16 = 8770;
    let mut ui = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("ui").join("trainer.html");
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--vault" => vault = args.next().expect("--vault DIR").into(),
            "--port" => port = args.next().and_then(|p| p.parse().ok()).expect("--port N"),
            "--ui" => ui = args.next().expect("--ui FILE").into(),
            // Every model role is played by a mock (no plan limits spent): for trying the dashboard.
            "--mock" => {
                if let Err(e) = sla_eval::ops::install_mock_models(&root) {
                    eprintln!("cannot install the mock models: {e}");
                    std::process::exit(1);
                }
                println!("mock models: teachers replay the regression set, the translator is the pattern parser");
            }
            _ => {
                eprintln!("usage: sla_trainer [--vault DIR] [--port N] [--ui FILE] [--mock]");
                std::process::exit(2);
            }
        }
    }
    let mut trainer = Trainer::new(vault);
    // Keep every real training conversation in the repository's archive.
    if let Some(t) = std::sync::Arc::get_mut(&mut trainer) {
        t.archive = Some(sla_eval::archive::dir(&root));
    }
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
