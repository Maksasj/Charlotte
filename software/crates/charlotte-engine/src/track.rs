//! Layer 2, first implementation: one frame per request.

use crate::view::view;
use charlotte_core::*;
use std::collections::BTreeMap;

pub struct FrameTracker;

/// Merge facts into a frame; true if a fact that was already known changed.
fn merge(frame: &mut Frame, facts: &Facts) -> bool {
    let mut changed = false;
    for (k, v) in facts {
        if let Some(old) = frame.facts.get(k) {
            if !old.same(v) {
                changed = true;
            }
        }
        frame.facts.insert(k.clone(), v.clone());
    }
    frame.conflicts.retain(|slot, _| !facts.contains_key(slot));
    if frame.last_asked.as_ref().map(|s| frame.facts.contains_key(s)).unwrap_or(false) {
        frame.last_asked = None;
    }
    changed
}
/// Slots the user contradicted themselves on: forget what was known of them and ask which is right.
fn contradict(frame: &mut Frame, conflicts: &BTreeMap<String, Vec<Value>>) {
    for (slot, values) in conflicts {
        frame.facts.remove(slot);
        frame.conflicts.insert(slot.clone(), values.clone());
    }
}
fn finished(f: &Frame) -> bool {
    f.agent_acts.iter().any(|o| matches!(o, Op::Finish | Op::Escalate | Op::Report))
}

/// Fill in the subject a reference points at: the one in focus, or the one before it. When there is none,
/// the request stays as it was and the agent asks.
fn resolve(state: &State, r: &mut Request) {
    let Some(refer) = r.refer else { return };
    if r.facts.contains_key("subject") {
        return;
    }
    let mut ranked: Vec<&Entity> = state.entities.iter().collect();
    ranked.sort_by(|a, b| (b.last_turn, b.mentions).cmp(&(a.last_turn, a.mentions)));
    let pick = match refer {
        Refer::Same => ranked.first(),
        Refer::Other => ranked.get(1),
    };
    if let Some(e) = pick {
        r.facts.insert("subject".into(), Value::Str(e.id.clone()));
        if let (Some(o), false) = (&e.object, r.facts.contains_key("object")) {
            r.facts.insert("object".into(), Value::Str(o.clone()));
        }
    }
}
fn remember(state: &mut State, facts: &Facts) {
    let Some(Value::Str(id)) = facts.get("subject") else { return };
    let object = match facts.get("object") {
        Some(Value::Str(o)) => Some(o.clone()),
        _ => None,
    };
    let turn = state.turn;
    match state.entities.iter_mut().find(|e| e.id == *id) {
        Some(e) => {
            e.last_turn = turn;
            e.mentions += 1;
            e.object = object.or(e.object.take());
        }
        None => state.entities.push(Entity { id: id.clone(), object, last_turn: turn, mentions: 1 }),
    }
}

impl Track for FrameTracker {
    fn user(&self, state: &mut State, reading: &Reading, mem: &dyn Memory) -> Vec<Touch> {
        let mut out = Vec::new();
        state.turn += 1;
        for (i, r) in reading.requests.iter().enumerate() {
            let mut r = r.clone();
            resolve(state, &mut r);
            remember(state, &r.facts);
            let r = &r;
            let (mut frame, mut act) = (None, None);
            if let Some(intent) = &r.intent {
                // The same request again continues its frame, unless it is finished or about another subject.
                let other_subject = |f: &Frame| matches!((f.facts.get("subject"), r.facts.get("subject")), (Some(a), Some(b)) if !a.same(b));
                match state.frames.iter().rposition(|f| f.intent == *intent && !finished(f) && !other_subject(f)) {
                    Some(j) => {
                        act = Some(if merge(&mut state.frames[j], &r.facts) { UserAct::Correct } else { UserAct::Provide });
                        frame = Some(j);
                    }
                    None => {
                        let id = state.frames.len() as u32 + 1;
                        state.frames.push(Frame { id, intent: intent.clone(), facts: r.facts.clone(), ..Default::default() });
                        act = Some(mem.intent(intent).and_then(|i| i.act).unwrap_or(UserAct::Request));
                        frame = Some(state.frames.len() - 1);
                    }
                }
            } else if matches!(r.signal, Some(Signal::Done | Signal::Failed)) && r.facts.is_empty() {
                if let Some(j) = state.frames.iter().rposition(|f| mem.procedure(&f.intent).is_some() && !finished(f)) {
                    if r.signal == Some(Signal::Done) {
                        state.frames[j].steps_done += 1;
                        act = Some(UserAct::Done);
                    } else {
                        act = Some(UserAct::Failed);
                    }
                    frame = Some(j);
                }
            } else if !r.facts.is_empty() || !r.conflicts.is_empty() {
                // Facts without a request answer the frame that asked for them, else the latest real request.
                let asked = state.frames.iter().rposition(|f| f.last_asked.as_ref().map(|s| r.facts.contains_key(s)).unwrap_or(false));
                let real = |f: &Frame| !matches!(view(f, mem).kind, ReqKind::Move | ReqKind::Unknown);
                let latest = || state.frames.iter().rposition(|f| real(f) && !finished(f)).or_else(|| state.frames.iter().rposition(|f| real(f)));
                if let Some(j) = asked.or_else(latest) {
                    act = Some(if merge(&mut state.frames[j], &r.facts) { UserAct::Correct } else { UserAct::Provide });
                    frame = Some(j);
                }
            }
            if let (Some(j), Some(a)) = (frame, act) {
                contradict(&mut state.frames[j], &r.conflicts);
                state.frames[j].user_acts.push(a);
            }
            out.push(Touch { request: i, frame, user_act: act });
        }
        out
    }

    fn agent(&self, frame: &mut Frame, act: &Act, mem: &dyn Memory) {
        frame.agent_acts.push(act.op);
        match act.op {
            Op::Ask => frame.last_asked = act.arg.clone(),
            Op::Grant | Op::Offer | Op::Deny => frame.standing = Some(act.op),
            Op::Lookup => {
                frame.fetched = true;
                let row = frame.facts.get("subject").and_then(|s| mem.record(&s.text()));
                frame.found = row.is_some();
                if let Some(row) = row {
                    for (k, v) in &row.facts {
                        frame.facts.insert(k.clone(), v.clone());
                    }
                }
            }
            _ => {}
        }
    }

    fn call(&self, state: &mut State, parent: usize, intent: &str, _mem: &dyn Memory) -> usize {
        let id = state.frames.len() as u32 + 1;
        let mut facts = Facts::new();
        for k in ["subject", "object"] {
            if let Some(v) = state.frames[parent].facts.get(k) {
                facts.insert(k.to_string(), v.clone());
            }
        }
        state.frames[parent].subgoal = Some(id);
        let pid = state.frames[parent].id;
        state.frames.push(Frame { id, intent: intent.to_string(), facts, parent: Some(pid), ..Default::default() });
        state.frames.len() - 1
    }

    fn ret(&self, state: &mut State, child: usize, act: &Act, mem: &dyn Memory) -> Option<usize> {
        if !matches!(act.op, Op::Report | Op::Finish | Op::Escalate) {
            return None;
        }
        let pid = state.frames[child].parent?;
        let p = state.frames.iter().position(|f| f.id == pid)?;
        let c = state.frames[child].clone();
        let kind = view(&c, mem).kind;
        let parent = &mut state.frames[p];
        parent.subgoal = None;
        parent.returned_from = Some(kind);
        parent.subgoal_failed = act.op == Op::Escalate;
        if c.fetched {
            parent.fetched = true;
            parent.found = c.found;
        }
        merge(parent, &c.facts);
        Some(p)
    }
}
