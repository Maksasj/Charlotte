//! Learning procedures from teaching (Design 2.0, milestone 17).
//!
//! What a task is, what it needs, how long it takes, its steps and their
//! order are kept in that task's procedure file (`vault/procedures/<task>.md`),
//! not as loose facts: v1 stored a recipe taught four times as zero usable
//! facts. Steps told over several turns attach to the same task (the
//! translator names them after the active task). A new procedure starts as a
//! hypothesis; it is offered as "I was taught this once" and confirmed by a
//! successful use (or a second teacher).

use crate::{Agent, Brain};
use sla_core::{Atom, Fact, GroundAtom, ProcedureFile, ProcedureKind, ProcedureMeta, RuleStatus, Value};

/// Predicates whose first argument is a task.
const TASK_PREDS: &[&str] = &["task", "step", "needs", "solves", "subtask", "method", "done_when", "recommended_for"];
/// Predicates whose first argument is a step.
const STEP_PREDS: &[&str] = &["action", "object_of", "uses", "until", "overlaps"];

/// `make_tea_s2` -> `make_tea`.
fn task_of_step_name(s: &str) -> Option<&str> {
    let (t, n) = s.rsplit_once("_s")?;
    (!n.is_empty() && n.chars().all(|c| c.is_ascii_digit()) && !t.is_empty()).then_some(t)
}

impl Agent {
    /// The task a fact belongs to, if it is procedure knowledge.
    fn owner(&self, b: &Brain, g: &GroundAtom, batch: &[GroundAtom]) -> Option<String> {
        if g.is_strong_negation() {
            return None;
        }
        let first = g.args.first()?.as_sym()?.to_string();
        let step_task = |s: &str| -> Option<String> {
            if let Some(st) = batch.iter().find(|a| a.pred == "step" && a.args.get(1).and_then(|v| v.as_sym()) == Some(s)) {
                return st.args[0].as_sym().map(String::from);
            }
            for (t, pf) in &b.kb.vault.procedures {
                if pf.facts().iter().any(|f| f.atom.pred == "step" && f.atom.args.get(1).and_then(|v| v.as_sym()) == Some(s)) {
                    return Some(t.clone());
                }
            }
            task_of_step_name(s).map(String::from)
        };
        match g.pred.as_str() {
            p if TASK_PREDS.contains(&p) => Some(first),
            p if STEP_PREDS.contains(&p) => step_task(&first),
            "before" => step_task(&first),
            // A duration of a step belongs to its task; a duration of anything
            // else names a task ("Boiling an egg takes 8 minutes").
            "takes_time" => step_task(&first).or(Some(first)),
            _ => None,
        }
    }

    /// Put procedure knowledge into procedure files; return (what was learned
    /// that way, the facts that are not procedure knowledge).
    pub(crate) fn learn_procedures(&mut self, b: &mut Brain, facts: Vec<Atom>, conf: f64) -> (Vec<GroundAtom>, Vec<Atom>) {
        let ground: Vec<GroundAtom> = facts.iter().filter_map(|a| a.to_ground()).collect();
        let mut learned = Vec::new();
        let mut rest = Vec::new();
        let now = self.now();
        for a in facts {
            let Some(g) = a.to_ground() else {
                rest.push(a);
                continue;
            };
            let Some(task) = self.owner(b, &g, &ground) else {
                rest.push(a);
                continue;
            };
            let stem = sla_core::vault::file_stem(&task);
            let pf = b.kb.vault.procedures.entry(stem.clone()).or_insert_with(|| {
                let meta = ProcedureMeta { id: task.clone(), status: RuleStatus::Hypothesis, source: self.source.clone(), uses: 0, successes: 0 };
                ProcedureFile::new(&meta, &task.replace('_', " "), "Learned in conversation.", &[Fact::new(GroundAtom::new("task", vec![Value::sym(task.as_str())]), "")])
            });
            if !pf.facts().iter().any(|f| f.atom == g) {
                let mut f = Fact::new(g.clone(), &self.source).with_conv(&self.session_id);
                f.conf = conf;
                pf.doc.push(sla_core::doc::Item::Fact(f));
                b.kb.vault.mark_procedure_dirty(ProcedureKind::Procedure, &stem);
                b.kb.log_change(now, "learn_procedure", &format!("{task}: {g}"), &format!("{} taught it", self.source));
            }
            learned.push(g);
        }
        (learned, rest)
    }
}
