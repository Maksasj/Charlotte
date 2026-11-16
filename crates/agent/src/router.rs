//! Module router: which domain module a predicate belongs to. Learned rules
//! are filed under the module of their head; questions can be routed to the
//! modules of their predicates. Unknown predicates go to `general`.

pub fn module_for(pred: &str) -> &'static str {
    let p = pred.trim_start_matches('-');
    match p {
        "likes" | "knows" | "wants" | "feels" | "person" | "believes" | "says" => "people",
        "located_in" | "place" | "lives_in" => "places",
        "is_a" | "has" | "part_of" | "property" | "thing" | "same" => "things",
        "before" | "during" | "event" | "agent_of" | "does" | "causes" => "events",
        _ => {
            let first = p.split('_').next().unwrap_or(p);
            match first {
                "drink" | "eat" | "cook" | "bake" | "brew" | "taste" => "food",
                "work" | "manage" | "code" | "design" | "teach" | "study" | "attend" => "work",
                "use" | "fix" | "repair" | "build" | "cut" => "tools",
                "live" | "visit" | "travel" | "go" | "stay" => "places",
                _ => "general",
            }
        }
    }
}

/// Modules relevant to a set of predicates (for routing a question).
pub fn modules_for(preds: &[&str]) -> Vec<&'static str> {
    let mut v: Vec<&'static str> = preds.iter().map(|p| module_for(p)).collect();
    v.sort();
    v.dedup();
    v
}

#[cfg(test)]
mod tests {
    #[test]
    fn routes() {
        assert_eq!(super::module_for("likes"), "people");
        assert_eq!(super::module_for("-drink"), "food");
        assert_eq!(super::module_for("work_at"), "work");
        assert_eq!(super::module_for("zorble"), "general");
        assert_eq!(super::modules_for(&["likes", "knows", "drink"]), vec!["food", "people"]);
    }
}
