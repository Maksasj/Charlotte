//! Design 2.0, milestone 13 acceptance: 100 generated procedure files
//! round-trip with no unintended changes; a changed step or a changed status
//! touches exactly one line; procedure facts take part in reasoning.

use sla_core::doc::Item;
use sla_core::term::{Fact, GroundAtom, Value};
use sla_core::vault::{ProcedureFile, ProcedureKind, ProcedureMeta, RuleStatus, Vault};
use sla_core::{parse_ground, KnowledgeBase, Timestamp};
use std::fs;
use std::path::PathBuf;

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
    fn pick<'a>(&mut self, xs: &'a [&'a str]) -> &'a str {
        xs[(self.next() as usize) % xs.len()]
    }
}

const TASKS: &[&str] = &["make_tea", "fix_flat_tire", "plan_picnic", "boil_egg", "cook_rice", "clean_kitchen", "pack_beach_bag", "water_plants", "make_pancakes", "make_soup"];
const ACTIONS: &[&str] = &["boil", "fry", "add", "mix", "pour", "wait", "cut", "wash", "remove", "check", "pump", "patch", "pack", "stir"];
const ITEMS: &[&str] = &["water", "kettle", "tea_bag", "pan", "egg", "rice", "pot", "tube", "pump", "patch_kit", "towel", "flour", "milk", "salt"];
const UNITS: &[&str] = &["second", "minute", "hour"];
const STATES: &[&str] = &["soft", "golden", "boiling", "dry", "clean", "done"];

fn g(s: &str) -> GroundAtom {
    parse_ground(s).unwrap()
}

/// A procedure with needs, ordered steps (some side by side), times and states.
fn generate(r: &mut Lcg, i: usize) -> ProcedureFile {
    let task = format!("{}_{i}", r.pick(TASKS));
    let mut facts: Vec<Fact> = vec![Fact::new(g(&format!("task({task})")), "")];
    for _ in 0..1 + r.below(5) {
        facts.push(Fact::new(g(&format!("needs({task}, {})", r.pick(ITEMS))), ""));
    }
    let lo = 1 + r.below(30);
    facts.push(Fact::new(g(&format!("takes_time({task}, {lo}, {}, minute)", lo + r.below(15))), ""));
    let n = 2 + r.below(5);
    for s in 1..=n {
        let id = format!("{task}_s{s}");
        facts.push(Fact::new(g(&format!("step({task}, {id})")), ""));
        facts.push(Fact::new(g(&format!("action({id}, {})", r.pick(ACTIONS))), ""));
        facts.push(Fact::new(g(&format!("object_of({id}, {})", r.pick(ITEMS))), ""));
        // Times as integers or decimals, in any unit.
        let t = if r.below(3) == 0 { format!("{}.5", r.below(9)) } else { (1 + r.below(20)).to_string() };
        facts.push(Fact::new(g(&format!("takes_time({id}, {t}, {}, {})", 21 + r.below(10), r.pick(UNITS))), ""));
        if r.below(3) == 0 {
            facts.push(Fact::new(g(&format!("until({id}, {})", r.pick(STATES))), ""));
        }
        if s > 1 {
            if r.below(4) == 0 {
                facts.push(Fact::new(g(&format!("overlaps({id}, {task}_s{})", s - 1)), ""));
            } else {
                facts.push(Fact::new(g(&format!("before({task}_s{}, {id})", s - 1)), ""));
            }
        }
    }
    // Some facts carry their own source and confidence.
    if r.below(2) == 0 {
        facts.push(Fact::new(g(&format!("uses({task}_s1, {})", r.pick(ITEMS))), "teacher:train").with_conf(0.8));
    }
    let meta = ProcedureMeta {
        id: task.clone(),
        status: if r.below(2) == 0 { RuleStatus::Hypothesis } else { RuleStatus::Confirmed },
        source: "teacher:train".into(),
        uses: r.below(10),
        successes: r.below(5),
    };
    ProcedureFile::new(&meta, &task.replace('_', " "), "Taught in a training conversation.", &facts)
}

fn temp(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("sla_m13_{name}_{}", std::process::id()));
    let _ = fs::remove_dir_all(&d);
    d
}

fn changed(a: &str, b: &str) -> Vec<(String, String)> {
    assert_eq!(a.lines().count(), b.lines().count(), "line count changed");
    a.lines().zip(b.lines()).filter(|(x, y)| x != y).map(|(x, y)| (x.to_string(), y.to_string())).collect()
}

#[test]
fn hundred_procedure_files_round_trip() {
    let dir = temp("procs");
    let mut r = Lcg(13);
    let mut v = Vault::empty(&dir);
    let mut originals = Vec::new();
    for i in 0..100 {
        let pf = generate(&mut r, i);
        let text = pf.render();
        // Text -> file -> text is exact, and nothing fails to parse.
        let back = ProcedureFile::parse(&text, "x");
        assert_eq!(back.render(), text, "file {i}");
        assert!(back.doc.errors().is_empty(), "{:?}", back.doc.errors());
        assert_eq!(back.meta(), pf.meta());
        assert_eq!(back.facts(), pf.facts());
        originals.push((pf.meta().id, text));
        v.put_procedure(ProcedureKind::Procedure, pf);
    }
    v.save().unwrap();
    // Reloaded from disk: every file is byte for byte the same.
    let mut v2 = Vault::load(&dir).unwrap();
    assert_eq!(v2.procedures.len(), 100);
    for (id, text) in &originals {
        assert_eq!(&v2.procedures[id].render(), text);
        assert_eq!(fs::read_to_string(dir.join("procedures").join(format!("{id}.md"))).unwrap(), *text);
    }
    // One changed step time touches exactly one line.
    let (id, before) = &originals[7];
    let pf = v2.procedures.get_mut(id).unwrap();
    let n = pf.doc.update_items(|item| match item {
        Item::Fact(f) if f.atom.pred == "takes_time" && f.atom.args[0].as_sym().map_or(false, |s| s.ends_with("_s1")) => {
            f.atom.args[1] = Value::Int(99);
            true
        }
        _ => false,
    });
    assert_eq!(n, 1);
    let after = pf.render();
    let diff = changed(before, &after);
    assert_eq!(diff.len(), 1, "{diff:?}");
    assert!(diff[0].1.contains(", 99, "));
    // A status change touches exactly the status line.
    let mut m = pf.meta();
    m.status = RuleStatus::Rejected;
    pf.set_meta(&m);
    let diff = changed(&after, &pf.render());
    assert_eq!(diff.len(), 1);
    assert!(diff[0].1.starts_with("status: rejected"), "{diff:?}");
    v2.mark_procedure_dirty(ProcedureKind::Procedure, id);
    v2.save().unwrap();
    // Every other file on disk is untouched.
    let v3 = Vault::load(&dir).unwrap();
    for (other, text) in originals.iter().filter(|(o, _)| o != id) {
        assert_eq!(&v3.procedures[other].render(), text);
    }
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn procedure_and_task_facts_are_believed() {
    let dir = temp("believed");
    let mut kb = KnowledgeBase::open(&dir).unwrap();
    kb.logging = false;
    let facts: Vec<Fact> = ["task(make_tea)", "needs(make_tea, kettle)", "step(make_tea, mt_s1)", "takes_time(mt_s1, 3, 5, minute)"].iter().map(|s| Fact::new(g(s), "")).collect();
    let meta = |id: &str, status| ProcedureMeta { id: id.into(), status, source: "teacher".into(), uses: 0, successes: 0 };
    kb.vault.put_procedure(ProcedureKind::Procedure, ProcedureFile::new(&meta("make_tea", RuleStatus::Hypothesis), "make tea", "", &facts));
    let schema: Vec<Fact> = ["task_slot(plan_meal, ingredients, required)", "slot_question(plan_meal, ingredients, \"What ingredients do you have?\")", "closing(plan_meal, \"Enjoy your dinner!\")"]
        .iter()
        .map(|s| Fact::new(g(s), ""))
        .collect();
    kb.vault.put_procedure(ProcedureKind::Task, ProcedureFile::new(&meta("plan_meal", RuleStatus::Confirmed), "plan a meal", "", &schema));
    kb.vault.put_procedure(ProcedureKind::Procedure, ProcedureFile::new(&meta("bad_tea", RuleStatus::Rejected), "bad", "", &[Fact::new(g("task(bad_tea)"), "")]));
    kb.save().unwrap();
    let kb = KnowledgeBase::open(&dir).unwrap();
    let m = sla_reasoner_free_eval(&kb);
    assert!(m.iter().any(|f| f.atom.to_string() == "needs(make_tea, kettle)" && (f.conf - 0.6).abs() < 1e-9 && f.source == "procedure:make_tea"), "a procedure on trial is believed less");
    assert!(m.iter().any(|f| f.atom.to_string() == "closing(plan_meal, \"Enjoy your dinner!\")" && f.conf == 1.0));
    assert!(!m.iter().any(|f| f.atom.to_string() == "task(bad_tea)"), "a rejected procedure takes no part");
    assert!(kb.all_facts().is_empty(), "procedures are not stored facts");
    let _ = Timestamp::now();
    let _ = fs::remove_dir_all(&dir);
}

/// The reasoner's input from a knowledge base (core cannot depend on the reasoner).
fn sla_reasoner_free_eval(kb: &KnowledgeBase) -> Vec<Fact> {
    kb.reasoning_facts()
}
