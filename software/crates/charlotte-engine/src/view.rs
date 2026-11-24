//! How a frame looks against the knowledge: which rule applies, the state of its conditions, and the
//! generic description of the conversation state that patterns are written over.

use charlotte_core::*;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
pub struct Gate {
    pub exists: bool,
    /// Index of the first condition a known fact breaks.
    pub violated: Option<usize>,
    /// Indexes of conditions whose fact is not known yet.
    pub unbound: Vec<usize>,
}
impl Gate {
    pub fn satisfied(&self) -> bool {
        self.exists && self.violated.is_none() && self.unbound.is_empty()
    }
}
pub fn gate(atom: &Atom, facts: &Facts) -> Gate {
    let mut g = Gate { exists: true, ..Default::default() };
    for (i, c) in atom.conditions.iter().enumerate() {
        match facts.get(&c.slot) {
            None => g.unbound.push(i),
            Some(v) if !c.holds(v) => {
                if g.violated.is_none() {
                    g.violated = Some(i)
                }
            }
            Some(_) => {}
        }
    }
    g
}

pub struct View<'a> {
    pub kind: ReqKind,
    /// The main rule for this request and the alternative considered beside it.
    pub prim: Option<&'a Atom>,
    pub alt: Option<&'a Atom>,
    pub p: Gate,
    pub a: Gate,
    pub procedure: Option<&'a Procedure>,
    pub mv: Option<&'a Move>,
    /// An intent that fetches a record, for a request to call when it needs one.
    pub record_source: Option<String>,
}

pub fn view<'a>(frame: &Frame, mem: &'a dyn Memory) -> View<'a> {
    let mut v = View { kind: ReqKind::Unknown, prim: None, alt: None, p: Gate::default(), a: Gate::default(), procedure: None, mv: None, record_source: mem.intents().into_iter().find(|i| i.lookup).map(|i| i.id.clone()) };
    let mut atoms = mem.atoms(&frame.intent);
    if !atoms.is_empty() {
        atoms.sort_by(|a, b| b.priority.cmp(&a.priority).then(a.id.cmp(&b.id)));
        // Atoms about another kind of object do not take part: an order is not a subscription.
        let wanted = frame.facts.get("object").map(|o| o.text());
        let object = match &wanted {
            Some(o) if atoms.iter().any(|a| a.object.as_deref() == Some(o.as_str())) => Some(o.clone()),
            _ => atoms[0].object.clone(),
        };
        let group: Vec<&Atom> = atoms.into_iter().filter(|a| a.object == object).collect();
        let prim = group[0];
        let others = &group[1..];
        let alt = others.iter().find(|a| gate(a, &frame.facts).violated.is_none()).or(others.first()).copied();
        v.kind = ReqKind::Gated;
        v.p = gate(prim, &frame.facts);
        v.a = alt.map(|a| gate(a, &frame.facts)).unwrap_or_default();
        v.prim = Some(prim);
        v.alt = alt;
    } else if let Some(p) = mem.procedure(&frame.intent) {
        v.kind = ReqKind::Procedure;
        v.procedure = Some(p);
    } else if mem.intent(&frame.intent).map(|i| i.lookup).unwrap_or(false) {
        v.kind = ReqKind::Lookup;
    } else if let Some(m) = mem.moves().into_iter().find(|m| m.intent.as_deref() == Some(frame.intent.as_str())) {
        v.kind = ReqKind::Move;
        v.mv = Some(m);
    }
    v
}

/// The state description. Every pattern rule is a conjunction of these names, plain or negated.
pub fn features(frame: &Frame, v: &View) -> BTreeMap<String, bool> {
    let mut f = BTreeMap::new();
    let mut set = |k: &str, b: bool| {
        f.insert(k.to_string(), b);
    };
    set("req_gated", v.kind == ReqKind::Gated);
    set("req_proc", v.kind == ReqKind::Procedure);
    set("req_status", v.kind == ReqKind::Lookup);
    set("p_unbound", v.p.exists && !v.p.unbound.is_empty());
    set("p_viol", v.p.violated.is_some());
    set("p_sat", v.p.satisfied());
    set("a_exists", v.a.exists);
    set("a_unbound", v.a.exists && !v.a.unbound.is_empty());
    set("a_viol", v.a.violated.is_some());
    set("a_sat", v.a.satisfied());
    let from = |src: Source| v.prim.map(|p| v.p.unbound.iter().any(|&i| p.conditions[i].source == src)).unwrap_or(false);
    set("p_need_record", from(Source::Record));
    set("p_need_user", from(Source::User));
    set("subject_known", frame.facts.contains_key("subject"));
    set("fetched", frame.fetched);
    set("found", frame.found);
    set("subgoal_active", frame.subgoal.is_some());
    set("returned", frame.returned_from.is_some());
    for k in [ReqKind::Gated, ReqKind::Procedure, ReqKind::Lookup] {
        set(&format!("returned_from={}", k.name()), frame.returned_from == Some(k));
    }
    set("subgoal_failed", frame.subgoal_failed);
    let steps = v.procedure.map(|p| p.steps.len()).unwrap_or(0);
    set("steps_left", v.procedure.is_some() && frame.steps_done < steps);
    set("none_done", v.procedure.is_some() && frame.steps_done == 0);
    let n = frame.agent_acts.len();
    for o in Op::LEARNABLE {
        set(&format!("last_agent={}", o.name()), n >= 1 && frame.agent_acts[n - 1] == o);
        set(&format!("prev_agent={}", o.name()), n >= 2 && frame.agent_acts[n - 2] == o);
        set(&format!("ever_agent={}", o.name()), frame.agent_acts.contains(&o));
    }
    let m = frame.user_acts.len();
    for u in UserAct::ALL {
        set(&format!("last_user={}", u.name()), m >= 1 && frame.user_acts[m - 1] == u);
        set(&format!("prev_user={}", u.name()), m >= 2 && frame.user_acts[m - 2] == u);
        set(&format!("ever_user={}", u.name()), frame.user_acts.contains(&u));
    }
    for o in [Op::Grant, Op::Offer, Op::Deny] {
        set(&format!("standing={}", o.name()), frame.standing == Some(o));
    }
    f
}

/// Ways to name an act's argument without naming any domain content.
pub fn selectors(frame: &Frame, v: &View) -> BTreeMap<String, String> {
    let mut s = BTreeMap::new();
    s.insert("subject".to_string(), "subject".to_string());
    if let Some(r) = &v.record_source {
        s.insert("record_source".to_string(), r.clone());
    }
    if let Some(p) = v.prim {
        let slot = |i: &usize| p.conditions[*i].slot.clone();
        if let Some(i) = v.p.unbound.first() {
            s.insert("p_first_unbound".into(), slot(i));
        }
        if let Some(i) = v.p.unbound.last() {
            s.insert("p_last_unbound".into(), slot(i));
        }
        if let Some(i) = v.p.unbound.iter().find(|&&i| p.conditions[i].source == Source::User) {
            s.insert("p_first_unbound_user".into(), slot(i));
        }
    }
    if let Some(a) = v.alt {
        if let Some(i) = v.a.unbound.first() {
            s.insert("a_first_unbound".into(), a.conditions[*i].slot.clone());
        }
        if let Some(i) = v.a.unbound.last() {
            s.insert("a_last_unbound".into(), a.conditions[*i].slot.clone());
        }
    }
    if let Some(p) = v.procedure {
        let n = p.steps.len();
        if frame.steps_done < n {
            s.insert("step_first_undone".into(), frame.steps_done.to_string());
        }
        if frame.steps_done > 0 {
            s.insert("step_last_done".into(), (frame.steps_done - 1).to_string());
        }
        if n > 0 {
            s.insert("step_first".into(), "0".into());
            s.insert("step_last".into(), (n - 1).to_string());
        }
    }
    s
}
