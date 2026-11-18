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
            // The same task worded differently goes to the same procedure:
            // rename it (and its step ids) in the fact.
            let (task, g) = if b.kb.vault.procedures.contains_key(&sla_core::vault::file_stem(&task)) {
                (task, g)
            } else {
                match crate::tasks::similar_task(&task, b.kb.vault.procedures.keys().map(String::as_str)) {
                    Some(k) => {
                        let rename = |v: &Value| -> Value {
                            match v.as_sym() {
                                Some(s) if s == task => Value::sym(k.as_str()),
                                Some(s) if s.starts_with(&format!("{task}_s")) => Value::sym(format!("{k}{}", &s[task.len()..]).as_str()),
                                _ => v.clone(),
                            }
                        };
                        let g2 = GroundAtom::new(&g.pred, g.args.iter().map(rename).collect());
                        (k, g2)
                    }
                    None => (task, g),
                }
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
        // A numbered step told without an order comes after the step before
        // it (and after what that step runs alongside).
        let new_steps: Vec<(String, String)> = learned.iter().filter(|g| g.pred == "step").filter_map(|g| Some((g.args[0].as_sym()?.to_string(), g.args.get(1)?.as_sym()?.to_string()))).collect();
        for (task, st) in new_steps {
            let ordered = learned.iter().any(|g| (g.pred == "before" && g.args.get(1).and_then(|v| v.as_sym()) == Some(&st)) || (g.pred == "overlaps" && g.args.first().and_then(|v| v.as_sym()) == Some(&st)));
            let num = |s: &str| s.rsplit_once("_s").and_then(|(_, n)| n.parse::<u32>().ok());
            let (Some(k), false) = (num(&st), ordered) else { continue };
            let stem = sla_core::vault::file_stem(&task);
            let Some(pf) = b.kb.vault.procedures.get(&stem) else { continue };
            let facts: Vec<GroundAtom> = pf.facts().iter().map(|f| f.atom.clone()).collect();
            let prev = facts.iter().filter(|g| g.pred == "step" && g.args[0].as_sym() == Some(&task)).filter_map(|g| g.args.get(1)?.as_sym().map(String::from)).filter(|s| num(s).map_or(false, |n| n < k)).max_by_key(|s| num(s));
            let Some(prev) = prev else { continue };
            let mut add = vec![prev.clone()];
            add.extend(facts.iter().filter(|g| g.pred == "overlaps" && g.args[0].as_sym() == Some(&prev)).filter_map(|g| g.args.get(1)?.as_sym().map(String::from)));
            let pf = b.kb.vault.procedures.get_mut(&stem).unwrap();
            for a in add {
                let bf = GroundAtom::new("before", vec![Value::sym(a.as_str()), Value::sym(st.as_str())]);
                if !pf.facts().iter().any(|f| f.atom == bf) {
                    pf.doc.push(sla_core::doc::Item::Fact(Fact::new(bf.clone(), &self.source).with_conv(&self.session_id)));
                    learned.push(bf);
                }
            }
            b.kb.vault.mark_procedure_dirty(ProcedureKind::Procedure, &stem);
        }
        self.keep_wording(b, &learned);
        (learned, rest)
    }

    /// "Step 3 of watering a plant: pour the water slowly onto the soil ...":
    /// the taught wording of a step, kept in the procedure's notes so the
    /// help can say it the way it was taught (the facts still carry the claims).
    fn keep_wording(&mut self, b: &mut Brain, learned: &[GroundAtom]) {
        let last = self.heard_text.rsplit('\n').next().unwrap_or("").to_string();
        // "To pack for a trip you need ...": how the task itself is said.
        if let Some(g) = learned.iter().find(|g| g.pred == "needs") {
            if let (Some(task), Some(rest)) = (g.args[0].as_sym(), last.strip_prefix("to ")) {
                if let Some((phrase, _)) = rest.split_once(" you need") {
                    let first = phrase.split_whitespace().next().unwrap_or("");
                    if task.split('_').next() == Some(first) || task.contains(first) {
                        let stem = sla_core::vault::file_stem(task);
                        if let Some(pf) = b.kb.vault.procedures.get_mut(&stem) {
                            set_wording(&mut pf.doc, "task", phrase.trim());
                            b.kb.vault.mark_procedure_dirty(ProcedureKind::Procedure, &stem);
                        }
                    }
                }
            }
        }
        for g in learned.iter().filter(|g| g.pred == "action") {
            let (Some(st), Some(action)) = (g.args[0].as_sym(), g.args.get(1).and_then(|v| v.as_sym())) else { continue };
            let Some((task, n)) = st.rsplit_once("_s") else { continue };
            if n.is_empty() || !n.chars().all(|c| c.is_ascii_digit()) {
                continue;
            }
            let Some(text) = step_wording(&last, n, action) else { continue };
            let stem = sla_core::vault::file_stem(task);
            let Some(pf) = b.kb.vault.procedures.get_mut(&stem) else { continue };
            set_wording(&mut pf.doc, st, &text);
            b.kb.vault.mark_procedure_dirty(ProcedureKind::Procedure, &stem);
        }
    }
}

const WORDING_HEAD: &str = "## Taught wording";

/// The instruction after "step N of ...:" in a message, without its duration,
/// when it starts with (a form of) the step's action.
pub(crate) fn step_wording(message: &str, n: &str, action: &str) -> Option<String> {
    let key = format!("step {n} of ");
    let at = message.find(&key)?;
    let rest = &message[at..];
    let colon = rest.find(':')?;
    let body = rest[colon + 1..].trim();
    let body = body.split(". ").next().unwrap_or(body).trim().trim_end_matches('.');
    // Without the duration ("for 3 to 5 minutes", ", for 1 minute").
    let mut text = body.to_string();
    if let Some(i) = text.rfind(" for ") {
        let tail = &text[i + 5..];
        if tail.split_whitespace().next().map_or(false, |w| w.chars().all(|c| c.is_ascii_digit()) || matches!(w, "a" | "an" | "about" | "around" | "one" | "two" | "three" | "five" | "ten")) {
            text.truncate(i);
        }
    }
    let text = text.trim().trim_end_matches(',').trim().to_string();
    let first = text.split_whitespace().next()?;
    (first.starts_with(&action[..action.len().min(4)]) && text.split_whitespace().count() >= 2).then_some(text)
}

/// Store (or replace) a step's wording in the document's notes.
pub(crate) fn set_wording(doc: &mut sla_core::doc::Document, step: &str, text: &str) {
    use sla_core::doc::Segment;
    let line = format!("- {step}: {text}");
    let prefix = format!("- {step}: ");
    for seg in doc.segments.iter_mut() {
        if let Segment::Text(t) = seg {
            if t.trim_start().starts_with(WORDING_HEAD) {
                let mut lines: Vec<String> = t.lines().filter(|l| !l.starts_with(&prefix)).map(String::from).collect();
                lines.push(line);
                *t = lines.join("\n");
                return;
            }
        }
    }
    doc.segments.push(Segment::Text(format!("\n{WORDING_HEAD}\n\n{line}")));
}

/// The taught wording of each step of a procedure document.
pub(crate) fn wordings(doc: &sla_core::doc::Document) -> std::collections::HashMap<String, String> {
    use sla_core::doc::Segment;
    let mut out = std::collections::HashMap::new();
    for seg in &doc.segments {
        if let Segment::Text(t) = seg {
            if t.trim_start().starts_with(WORDING_HEAD) {
                for l in t.lines() {
                    if let Some((st, text)) = l.strip_prefix("- ").and_then(|r| r.split_once(": ")) {
                        out.insert(st.to_string(), text.to_string());
                    }
                }
            }
        }
    }
    out
}
