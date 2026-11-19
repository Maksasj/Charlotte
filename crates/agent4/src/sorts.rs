//! Sorts of the formal language (Design 4.0 section 4.2) and the type check at entry.
//! The table is the schema of v4: relation names and argument sorts, no domain content.

/// Argument sorts of each predicate the teacher may send ("any" accepts every sort).
pub const SIGNATURES: &[(&str, &[&str])] = &[
    ("is_a", &["same", "same"]),
    ("same_as", &["same", "same"]),
    ("has_part", &["item", "item"]),
    ("made_of", &["item", "material"]),
    ("has_property", &["item", "property"]),
    ("can_do", &["item", "function"]),
    ("can_do_on", &["item", "function", "item"]),
    ("used_for", &["item", "function"]),
    ("can_go_in", &["item", "place"]),
    ("effect", &["action", "item", "state"]),
    ("precondition", &["action", "item", "state"]),
    // procedures (schema v3) and the bridges
    ("task", &["task"]),
    ("step", &["task", "step"]),
    ("action", &["step", "action"]),
    ("object_of", &["step", "item"]),
    ("uses", &["step", "item"]),
    ("needs", &["task", "item"]),
    ("role", &["step", "item", "function"]),
    ("demands", &["step", "function", "relation", "any"]),
    ("before", &["step", "step"]),
    ("overlaps", &["step", "step"]),
    ("requires_state", &["step", "state"]),
    ("achieves", &["step", "state"]),
    ("into", &["step", "item"]),
    ("onto", &["step", "item"]),
    // situations (only in `given:`)
    ("has", &["agent", "item"]),
    ("lacks", &["agent", "item"]),
    ("state_of", &["item", "state"]),
    ("wants", &["agent", "item"]),
    ("excludes", &["agent", "item"]),
    // differences (only in case batches)
    ("swap", &["step", "item", "item"]),
    ("drop", &["step"]),
];

pub const SORTS: &[&str] = &["item", "place", "material", "property", "function", "state", "action", "step", "task", "agent", "relation"];

pub fn signature(pred: &str) -> Option<&'static [&'static str]> {
    SIGNATURES.iter().find(|(p, _)| *p == pred).map(|(_, s)| *s)
}

/// `place` is a subsort of `item`.
/// `function` and `action` are disjoint sorts that may share a name (`beat`, section 4.2).
pub fn compatible(declared: &str, wanted: &str) -> bool {
    declared == wanted
        || wanted == "any"
        || (wanted == "item" && declared == "place")
        || (wanted == "place" && declared == "item")
        || (wanted == "function" && declared == "action")
        || (wanted == "action" && declared == "function")
}
