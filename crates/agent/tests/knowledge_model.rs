//! Design 2.0, milestone 13: procedures in the vault take part in reasoning
//! through the shared brain, with session facts layered on top later (M15).

use sla_agent::Brain;
use sla_core::{parse_ground, Fact, ProcedureFile, ProcedureKind, ProcedureMeta, RuleStatus, Timestamp};

#[test]
fn procedures_in_the_vault_are_reasoned_over() {
    let dir = std::env::temp_dir().join(format!("sla_m13_brain_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let mut b = Brain::open(&dir).unwrap();
    let facts: Vec<Fact> = [
        "task(make_tea)",
        "solves(make_tea, warm_drink)",
        "needs(make_tea, tea_bag)",
        "needs(make_tea, kettle)",
        "step(make_tea, mt_s1)",
        "action(mt_s1, boil)",
        "object_of(mt_s1, water)",
        "takes_time(mt_s1, 3, 4, minute)",
        "step(make_tea, mt_s2)",
        "action(mt_s2, steep)",
        "takes_time(mt_s2, 3, 5, minute)",
        "before(mt_s1, mt_s2)",
    ]
    .iter()
    .map(|s| Fact::new(parse_ground(s).unwrap(), ""))
    .collect();
    let meta = ProcedureMeta { id: "make_tea".into(), status: RuleStatus::Confirmed, source: "teacher".into(), uses: 0, successes: 0 };
    let gen = b.kb.vault.generation;
    b.kb.vault.put_procedure(ProcedureKind::Procedure, ProcedureFile::new(&meta, "make tea", "", &facts));
    assert!(b.kb.vault.generation > gen, "a procedure change invalidates the cached model");
    b.save().unwrap();
    let now = Timestamp::from_ymd(2026, 11, 17);
    let m = b.model_at(now);
    let has = |s: &str| m.contains(&parse_ground(s).unwrap());
    assert!(has("total_minutes(make_tea, 6, 9)"));
    assert!(has("next_step(make_tea, mt_s1)") && !has("next_step(make_tea, mt_s2)"));
    assert!(has("missing(make_tea, kettle)"), "nothing is available in the vault itself");
    assert!(has("knows_task(make_tea)"));
    // After a restart the procedure is still there.
    drop(b);
    let mut b2 = Brain::open(&dir).unwrap();
    assert!(b2.model_at(now).contains(&parse_ground("step_count(make_tea, 2)").unwrap()));
    let _ = std::fs::remove_dir_all(&dir);
}
