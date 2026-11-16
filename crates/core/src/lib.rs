//! Core types, the Datalog text syntax, and the file-based vault.

pub mod doc;
pub mod kb;
pub mod syntax;
pub mod term;
pub mod time;
pub mod vault;

pub use kb::{AssertOutcome, KnowledgeBase};
pub use syntax::{parse_atom, parse_fact_line, parse_ground, parse_query, parse_rule, ParseError};
pub use term::{Atom, CmpOp, Fact, GroundAtom, Literal, Rule, Term, Value};
pub use time::Timestamp;
pub use vault::{RuleFile, RuleMeta, RuleStatus, Vault};

/// The built-in core files: (file stem, contents).
pub const CORE_FILES: &[(&str, &str)] = &[
    ("ontology", include_str!("../assets/core/ontology.md")),
    ("rules", include_str!("../assets/core/rules.md")),
];

/// Parsed core files, for seeding a new vault.
pub fn default_core() -> Vec<(String, RuleFile)> {
    CORE_FILES.iter().map(|(s, src)| (s.to_string(), RuleFile::parse(src, s))).collect()
}
