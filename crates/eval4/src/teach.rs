//! The teacher channel (Design 4.0 section 9, milestone 34): one Haiku call per topic batch,
//! through the Claude CLI, cached and archived in data/training/calls4/. The reply is split
//! into statements and delivered through the agent's conversation path one by one; the
//! agent's questions are queued and answered first in the next call (`answer N:`). Deny filter
//! P1 (prompt) and P2 (reply) apply to every call.

use agent4::deny::Deny;
use agent4::{Agent, Teacher as AgentTeacher};
use serde_json::{json, Value as J};
use std::collections::BTreeSet;

pub const PROMPT_VERSION: &str = "stage_v4";
const OTHERS: &str = "Other symbols already declared (keep their sorts): ";
const SEP: &str = ", ";

/// One fixed, versioned system prompt for all stages keeps cache keys stable.
pub const SYSTEM: &str = "You teach a learning agent about household things, in a formal language only. \
Reply with statements, one per line, each ending with a period, and nothing else: no English sentences, no numbering, no code fences. \
Statements: new(sym, sort). declares a symbol. Sorts: item = any physical thing, including food, ingredients and dishes; \
place = where things go or are put (oven, fridge, sink, drawer); material = a substance things are made of (metal, plastic, glass, wood, ceramic, fabric); \
property = a quality (sharp, fragile, absorbent, heat_resistant); function = what a thing can do (cut, hold, beat, wipe); \
state = a condition something reaches (hot, cleaned, melted); action = a verb a step performs (bake, chop, wash). \
Every symbol has exactly one sort for the whole course. When one word would be both an action and a state, use the verb for the action and its past participle for the state (dry and dried, clean and cleaned, set and set_firm). \
When a tool and its action share a word, the item keeps the word and the action gets a distinct verb (screw and screw_in, nail and hammer_in, whisk and whisk_up). Places are always place, never item. \
Facts: is_a(x, kind). made_of(x, material). has_property(x, property). can_do(x, function). can_do_on(x, function, item). \
used_for(x, function). can_go_in(x, place). has_part(x, part). effect(action, item, state). precondition(action, item, state). \
same_as(a, b). means two names for one thing. Any fact can be negated: not can_go_in(x, place). \
Symbols are lower case with underscores, singular nouns, base-form verbs. Use the given symbols; declare a new symbol with new(...) before its first use. \
State only what is true of ordinary household things. If the agent asked questions, answer them first, one line each: answer N: <statement>. or answer N: yes. / no. / unknown.";

/// What each stage asks for (design 9.1).
pub fn stage_task(stage: u32) -> &'static str {
    match stage {
        0 => "Stage 0, vocabulary only: declare every symbol below with new(sym, sort), using the suggested sort unless it is clearly wrong. Add same_as(a, b) for two listed names of one thing, and not same_as(a, b) for look-alike names of different things of the same sort. Add up to 10 closely related symbols that are missing, declared with new(...). Teach no other facts in this stage.",
        1 => "Stage 1, kinds: teach is_a facts for the items below, two or three levels up to general kinds, so kinds are shared across items.",
        2 => "Stage 2, materials and properties: teach made_of and has_property facts for the items below, including negatives where a natural guess would be wrong.",
        3 => "Stage 3, functions: teach can_do, can_do_on, used_for and can_go_in facts for the items below, including negatives where they matter (for example what cannot go in an oven).",
        4 => "Stage 4, parts: teach has_part facts for the items below that have parts.",
        5 => "Stage 5, actions and states: teach effect and precondition facts for the actions below, and is_a between actions.",
        _ => "Teach facts about the symbols below.",
    }
}

/// The agent's questions during a call are queued for the next call (never a call each).
#[derive(Default)]
pub struct Queue {
    pub pending: Vec<(usize, String)>,
    pub dropped: usize,
    pub deny: Deny,
    next: usize,
}

impl AgentTeacher for Queue {
    fn answer(&mut self, q: &str) -> String {
        // P3: a question about a denied atom is never sent
        if !self.deny.hits(q).is_empty() {
            self.dropped += 1;
        } else if self.pending.len() < 10 {
            self.pending.push((self.next, q.to_string()));
        } else {
            self.dropped += 1;
        }
        self.next += 1;
        "unknown".into()
    }
}

pub struct Unit {
    /// suggested sorts from the seed (stage 0 only)
    pub sorts: Vec<(String, String)>,
    pub stage: u32,
    pub unit: String,
    pub domain: String,
    pub symbols: Vec<String>,
}

/// One teacher turn: prompt, call, filter, deliver. Returns a record of the call.
pub fn teach_unit(agent: &mut Agent, haiku: &teach3::Teacher, queue: &mut Queue, u: &Unit, history: &[String]) -> Result<J, String> {
    // the registry slice with the sort each symbol already has (design 9.3), so the teacher
    // keeps one sort per name
    let listed = if u.stage == 0 && !u.sorts.is_empty() {
        u.sorts.iter().map(|(s, t)| format!("{s} ({t})")).collect::<Vec<_>>().join(", ")
    } else {
        u.symbols.iter().map(|s| match agent.sorts.get(s) { Some(t) => format!("{s} ({t})"), None => s.clone() }).collect::<Vec<_>>().join(", ")
    };
    let used_elsewhere: Vec<String> = agent.sorts.iter().filter(|(k, t)| !u.symbols.contains(k) && matches!(t.as_str(), "state" | "action" | "function" | "property" | "material" | "place")).take(60).map(|(k, t)| format!("{k} ({t})")).collect();
    let mut prompt = format!("{}\nDomain: {}. Unit: {}.\nSymbols: {listed}\n", stage_task(u.stage), u.domain, u.unit);
    if u.stage > 0 && !used_elsewhere.is_empty() {
        let shown = used_elsewhere.join(SEP);
        prompt.push_str(&format!("{OTHERS}{shown}\n"));
    }
    if !queue.pending.is_empty() {
        prompt.push_str("The agent asked:\n");
        for (n, q) in &queue.pending {
            prompt.push_str(&format!("{n}: {q}\n"));
        }
    }
    if !history.is_empty() {
        prompt.push_str("The agent said last time:\n");
        for h in history.iter().rev().take(2).rev() {
            prompt.push_str(h);
            prompt.push('\n');
        }
    }
    prompt.push_str("Teach 20-30 statements.\n");
    // P1: the prompt never carries a denied atom
    let p1 = queue.deny.hits(&prompt);
    if !p1.is_empty() {
        return Err(format!("P1 leak in prompt: {p1:?}"));
    }
    let reply = haiku.ask(SYSTEM, &prompt, &format!("teach4_stage{}_{PROMPT_VERSION}", u.stage))?;
    let (keep, withheld) = queue.deny.filter_reply(&reply);
    let before = (agent.stats.accepted, agent.stats.rejected, agent.stats.naming_questions, agent.sorts.len());
    let known: BTreeSet<String> = agent.sorts.keys().cloned().collect();
    // answer lines refer to the question numbers the agent itself used
    queue.pending.clear();
    agent.hear(&format!("topic {}.", u.unit), "live", queue);
    let mut statements = 0;
    let mut replies = Vec::new();
    for l in &keep {
        let l = l.trim().trim_start_matches("- ").to_string();
        if l.is_empty() {
            continue;
        }
        statements += 1;
        replies.extend(agent.hear(&l, "live", queue));
    }
    let after = (agent.stats.accepted, agent.stats.rejected, agent.stats.naming_questions, agent.sorts.len());
    let new_syms: Vec<String> = agent.sorts.keys().filter(|k| !known.contains(*k) && !u.symbols.contains(k)).cloned().collect();
    Ok(json!({"stage": u.stage, "unit": u.unit, "statements": statements, "withheld": withheld.len(),
        "accepted": after.0 - before.0, "rejected": after.1 - before.1, "naming_questions": after.2 - before.2,
        "new_symbols": new_syms.len(), "questions_queued": queue.pending.len(), "replies": replies.len()}))
}
