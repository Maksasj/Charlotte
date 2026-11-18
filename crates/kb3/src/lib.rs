//! Charlotte 3.0 knowledge base: schema v3 facts, scenarios, procedures, relation-line
//! answers and the fact-and-relation check (`research/Symbolic Learning Agent – Design 3.0.md`).

pub mod check;
pub mod fact;
pub mod lines;
pub mod procedure;
pub mod scenario;
pub mod vault;

pub use check::{check, Aliases, CheckResult};
pub use fact::Fact;
pub use lines::Answer;
pub use procedure::{Procedure, Step};
pub use scenario::{Scenario, Variant};
pub use vault::{Provenance, Source, Vault};

use std::path::PathBuf;

/// The repository root (two levels above this crate).
pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

/// The gold procedure for a scenario's base request, or for one of its variants.
pub fn gold(sc: &Scenario, variant: Option<usize>) -> Procedure {
    let base = Procedure::from_facts(&sc.task, &sc.facts);
    match variant.and_then(|k| sc.variants.get(k)) {
        Some(v) => base.apply(&v.difference),
        None => base,
    }
}

/// Items the situation says the user lacks or excludes.
pub fn lacked(v: &Variant) -> Vec<String> {
    v.situation.iter().filter(|f| f.pred == "lacks" || f.pred == "excludes").map(|f| f.arg(1).to_string()).collect()
}
