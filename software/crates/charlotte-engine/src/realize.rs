//! Layer 4, first implementation: fixed wording from the notes.

use crate::view::view;
use charlotte_core::*;

pub struct TemplateRealize;

fn fill(tpl: &str, facts: &Facts) -> String {
    let mut out = String::new();
    let mut rest = tpl;
    while let Some(i) = rest.find('{') {
        out.push_str(&rest[..i]);
        match rest[i..].find('}') {
            Some(j) => {
                let key = &rest[i + 1..i + j];
                match facts.get(key) {
                    Some(v) => out.push_str(&v.text()),
                    None if key == "subject" => out.push_str("your request"),
                    None => {}
                }
                rest = &rest[i + j + 1..];
            }
            None => {
                out.push_str(&rest[i..]);
                rest = "";
            }
        }
    }
    out.push_str(rest);
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

impl Realize for TemplateRealize {
    fn realize(&self, frame: &Frame, act: &Act, mem: &dyn Memory) -> String {
        let v = view(frame, mem);
        let subject = frame.facts.get("subject").map(|s| s.text()).unwrap_or_else(|| "that".into());
        let step = |v: &crate::view::View| act.arg.as_ref().and_then(|a| a.parse::<usize>().ok()).and_then(|i| v.procedure.and_then(|p| p.steps.get(i).map(|s| (i, p.steps.len(), s.clone()))));
        match act.op {
            Op::Ask => {
                let slot = act.arg.clone().unwrap_or_default();
                mem.slot(&slot).map(|s| s.ask.clone()).filter(|a| !a.is_empty()).unwrap_or_else(|| format!("Could you tell me the {slot}?"))
            }
            Op::Grant => v.prim.map(|a| fill(&a.payload, &frame.facts)).unwrap_or_default(),
            Op::Offer => v.alt.map(|a| fill(&a.payload, &frame.facts)).unwrap_or_default(),
            Op::Deny => {
                let Some(a) = v.prim else { return "That falls outside the policy I have on file.".into() };
                if let Some(d) = &a.deny {
                    return fill(d, &frame.facts);
                }
                match v.p.violated.map(|i| &a.conditions[i]) {
                    Some(c) => format!(
                        "I can't approve that: {} is {}, and the rule needs it {} {}.",
                        c.slot,
                        frame.facts.get(&c.slot).map(|x| x.text()).unwrap_or_default(),
                        c.op.words(),
                        c.value.text()
                    ),
                    None => "That falls outside the policy I have on file.".into(),
                }
            }
            Op::Confirm => {
                let standing = frame.standing.map(|op| self.realize(frame, &Act::new(op), mem)).unwrap_or_default();
                format!("That doesn't change the outcome. {standing}").trim().to_string()
            }
            Op::Lookup | Op::Call => String::new(),
            Op::Report => match mem.record(&subject) {
                Some(r) => format!("{} is {}. {}", r.id, r.state, r.note).trim().to_string(),
                None => format!("I have nothing on {subject}."),
            },
            Op::Instruct => step(&v).map(|(i, n, s)| format!("Step {} of {}: {}", i + 1, n, s)).unwrap_or_default(),
            Op::Repeat => step(&v).map(|(_, _, s)| format!("Let's try that step once more. {s}")).unwrap_or_default(),
            Op::Finish => "That was the last step — you're all set.".into(),
            Op::Escalate if frame.fetched && !frame.found => format!("I can't find {subject} in our records, so I'm passing this to a teammate."),
            Op::Escalate => "I'm passing this to a teammate, with what we have so far.".into(),
            Op::Say => act.arg.as_ref().and_then(|id| mem.moves().into_iter().find(|m| &m.id == id)).map(|m| fill(&m.reply, &frame.facts)).unwrap_or_default(),
            Op::TeachMe => "I don't know how to help with that yet.".into(),
        }
    }
}
