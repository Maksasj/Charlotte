//! Layer 2, first implementation: one frame per request.

use crate::view::view;
use charlotte_core::*;

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
    if frame.last_asked.as_ref().map(|s| frame.facts.contains_key(s)).unwrap_or(false) {
        frame.last_asked = None;
    }
    changed
}
fn finished(f: &Frame) -> bool {
    f.agent_acts.iter().any(|o| matches!(o, Op::Finish | Op::Escalate | Op::Report))
}

impl Track for FrameTracker {
    fn user(&self, state: &mut State, reading: &Reading, mem: &dyn Memory) -> Vec<Touch> {
        let mut out = Vec::new();
        for (i, r) in reading.requests.iter().enumerate() {
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
                        act = Some(UserAct::Request);
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
            } else if !r.facts.is_empty() {
                // Facts without a request answer the frame that asked for them, else the latest real request.
                let asked = state.frames.iter().rposition(|f| f.last_asked.as_ref().map(|s| r.facts.contains_key(s)).unwrap_or(false));
                let latest = || state.frames.iter().rposition(|f| !matches!(view(f, mem).kind, ReqKind::Move | ReqKind::Unknown));
                if let Some(j) = asked.or_else(latest) {
                    act = Some(if merge(&mut state.frames[j], &r.facts) { UserAct::Correct } else { UserAct::Provide });
                    frame = Some(j);
                }
            }
            if let (Some(j), Some(a)) = (frame, act) {
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
}
