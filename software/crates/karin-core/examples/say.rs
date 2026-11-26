//! Debug a conversation: each argument is one user turn.
//!   cargo run -p karin-core --example say -- "hi karen" "what is my name"
//! Prints what the language layer understood, which rules fired, and the reply.

use karin_core::engine::Engine;

fn main() {
    let mut engine = Engine::from_vault_dir(&karin_core::vault::default_vault_dir()).expect("load vault");
    for input in std::env::args().skip(1) {
        let t = engine.turn(&input);
        let names: Vec<String> = t
            .steps
            .iter()
            .map(|s| format!("{}({})", s.winner_name, s.winner_id))
            .collect();
        println!("> {input}");
        println!("  understood: {:?}  missed: {:?}", t.understood, t.missed);
        println!("  fired: {}", names.join(" -> "));
        println!("  {} {}", if t.impasse { "✗" } else { "✓" }, t.reply);
    }
}
