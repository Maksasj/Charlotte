mod learn;
mod reload;
mod selftest;

pub use reload::VaultReload;

use crate::nlu::Lexicon;
use crate::tools::{NewFact, run_world_tool};
use crate::types::{
    Action, AgentState, AttemptTrace, ChainRecord, ChatMessage, CompetingMatch, Condition, Fact,
    Pattern, PatternKind, PatternStatus, Scope, ScorePart, StepTrace, TurnTrace,
};
use crate::unify::{Bindings, is_var, match_conditions, normalize, subst, subst_args};
use crate::vault::{initial_state, load_vault};
use std::collections::{BTreeMap, HashSet};
use std::path::Path;

const MAX_STEPS_PER_TURN: usize = 12;
/// How many facts of one relation a chat remembers (oldest are forgotten first).
const CHAT_MEMORY_PER_RELATION: usize = 32;
const IMPASSE_REPLY: &str =
    "I don't know how to handle that yet (impasse). Teach me a reply or pattern and I'll learn it.";

pub struct Engine {
    pub state: AgentState,
    /// The readings tried during the current turn (for the trace).
    attempt_log: Vec<AttemptTrace>,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine {
    pub fn new() -> Self {
        Self {
            state: initial_state(),
            attempt_log: Vec::new(),
        }
    }

    pub fn from_vault_dir(dir: &Path) -> Result<Self, String> {
        Ok(Self {
            state: load_vault(dir)?,
            attempt_log: Vec::new(),
        })
    }

    pub fn from_state(state: AgentState) -> Self {
        Self {
            state,
            attempt_log: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.state = initial_state();
    }

    pub fn reset_from_vault_dir(&mut self, dir: &Path) -> Result<(), String> {
        self.state = load_vault(dir)?;
        Ok(())
    }

    pub fn new_chat(&mut self) {
        self.state.turn = 0;
        self.state
            .facts
            .retain(|fact| fact.scope == Scope::LongTerm);
        self.state.messages.clear();
        self.state.chains.clear();
    }

    pub(crate) fn alloc_id(&mut self, prefix: &str) -> String {
        let id = format!("{prefix}{}", self.state.next_id);
        self.state.next_id += 1;
        id
    }

    pub fn add_fact(
        &mut self,
        rel: String,
        args: BTreeMap<String, String>,
        scope: Scope,
        provenance: String,
    ) -> Fact {
        let scope = if scope == Scope::Working && self.is_chat_memory(&rel) {
            Scope::Chat
        } else {
            scope
        };
        if scope == Scope::Chat {
            // Saying it again makes it the most recent; keep each relation bounded.
            self.state
                .facts
                .retain(|f| !(f.scope == Scope::Chat && f.rel == rel && f.args == args));
            let same: Vec<usize> = (0..self.state.facts.len())
                .filter(|&i| {
                    self.state.facts[i].scope == Scope::Chat && self.state.facts[i].rel == rel
                })
                .collect();
            if same.len() >= CHAT_MEMORY_PER_RELATION {
                self.state.facts.remove(same[0]);
            }
        }
        let fact = Fact {
            id: self.alloc_id("f"),
            rel,
            args,
            scope,
            provenance,
            turn: self.state.turn,
        };
        self.state.facts.push(fact.clone());
        fact
    }

    pub fn delete_fact(&mut self, id: &str) -> bool {
        let before = self.state.facts.len();
        self.state.facts.retain(|fact| fact.id != id);
        self.state.facts.len() < before
    }

    pub fn add_pattern(&mut self, mut pattern: Pattern) -> Pattern {
        if pattern.id.is_empty() {
            pattern.id = self.alloc_id("p");
        }
        self.state.patterns.push(pattern.clone());
        pattern
    }

    pub fn promote_pattern(&mut self, id: &str) -> bool {
        match self.state.patterns.iter_mut().find(|pat| pat.id == id) {
            Some(pattern) => {
                pattern.status = PatternStatus::Active;
                true
            }
            None => false,
        }
    }

    pub fn delete_pattern(&mut self, id: &str) -> bool {
        let before = self.state.patterns.len();
        self.state.patterns.retain(|pat| pat.id != id);
        self.state.patterns.len() < before
    }

    pub fn turn(&mut self, input: &str) -> TurnTrace {
        let raw = input.trim().to_string();
        let turn = self.prepare_turn(&raw);
        self.attempt_log.clear();
        let lex = Lexicon::from_facts(&self.state.facts);
        let base = self.snapshot();

        // 1. The whole utterance, in canonical form.
        let whole_text = lex.normalize(&lex.resolve_pronouns(&raw, &self.state.facts));
        let mut chosen = self.attempt("whole", &whole_text);

        // 2. Several clauses ("Oscar is my friend and he lives in Venus"): used
        //    when every clause is understood, or when the whole was not.
        let clauses = lex.split_clauses(&raw);
        if clauses.len() > 1 {
            let after_whole = self.snapshot();
            self.restore(base.clone(), &raw);
            let split = self.attempt_clauses(&clauses, &lex);
            let all = split.missed.is_empty() && !split.replies.is_empty();
            if all || (!chosen.answered() && !split.replies.is_empty()) {
                chosen = split;
            } else {
                self.restore(after_whole, &raw);
            }
        }

        // 3. The exact text, but only for replies taught on that exact wording: the
        //    general rules already had their chance on the canonical form, and giving
        //    catch-alls ("$x is a $y") a second go at raw noise stores garbage.
        if chosen.replies.is_empty() && whole_text != raw {
            self.restore(base.clone(), &raw);
            let exact = self.attempt("exact", &raw);
            if exact.answered() {
                chosen = exact;
            }
        }

        if chosen.replies.is_empty() {
            self.restore(base, &raw);
            let hint = impasse_reply(&lex, &raw, &whole_text);
            let failed = Attempt {
                missed: vec![raw.clone()],
                ..Attempt::default()
            };
            return self.record_turn(turn, raw, None, Some(hint), failed);
        }
        let mut reply = chosen
            .replies
            .iter()
            .map(|r| {
                let r = r.trim();
                if chosen.replies.len() > 1 && !r.ends_with(['.', '!', '?', '"', ')']) {
                    format!("{r}.")
                } else {
                    r.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
        if !chosen.missed.is_empty() {
            let missed: Vec<String> = chosen.missed.iter().map(|m| format!("\"{m}\"")).collect();
            reply.push_str(&format!(" (I didn't catch {}.)", missed.join(", ")));
        }
        self.record_turn(turn, raw, Some(reply), None, chosen)
    }

    /// Run the rule loop on one canonical sentence. Rolls back on failure.
    fn attempt(&mut self, kind: &str, text: &str) -> Attempt {
        let before = self.snapshot();
        let mut fired_keys = HashSet::new();
        let mut out = Attempt::default();

        for step in 1..=MAX_STEPS_PER_TURN {
            let mut candidates = self.match_active_patterns(text, &fired_keys, kind == "exact");
            if candidates.is_empty() {
                break;
            }

            let competing = top_competing_matches(&candidates);
            let (winner, bindings, _) = candidates.remove(0);

            fired_keys.insert(fire_key(&winner.id, &bindings));
            out.fired.push(winner.id.clone());
            out.bindings.extend(bindings.clone());

            let (effects, reply) = self.fire_pattern(&winner, &bindings);
            out.steps.push(StepTrace {
                step,
                competing,
                winner_id: winner.id,
                winner_name: winner.name,
                bindings,
                effects,
                text: text.to_string(),
            });

            if let Some(text) = reply {
                out.replies.push(text);
                break;
            }
        }

        self.attempt_log.push(AttemptTrace {
            kind: kind.to_string(),
            text: text.to_string(),
            answered: !out.replies.is_empty(),
        });
        if out.replies.is_empty() {
            let user_text = self.current_user_text();
            self.restore(before, &user_text);
        } else {
            out.understood.push(text.to_string());
        }
        out
    }

    /// Each clause in turn, with pronouns resolved against what the earlier
    /// clauses just said, and ellipsis ("calculate 5 * 7 and 6 * 9") filled in
    /// from the previous clause.
    fn attempt_clauses(&mut self, clauses: &[String], lex: &Lexicon) -> Attempt {
        let mut out = Attempt::default();
        let mut previous: Option<(String, Bindings)> = None;
        let raw = self.current_user_text();

        for clause in clauses {
            self.reset_working(&raw);
            let text = lex.normalize(&lex.resolve_pronouns(clause, &self.state.facts));
            let mut got = self.attempt("clause", &text);
            if !got.answered()
                && let Some((prev_text, prev_bindings)) = &previous
                && let Some(filled) = fill_ellipsis(prev_text, prev_bindings, &text)
            {
                self.reset_working(&raw);
                got = self.attempt("ellipsis", &filled);
            }
            if got.answered() {
                let used = got.understood.last().cloned().unwrap_or(text);
                previous = Some((used, got.bindings.clone()));
                out.absorb(got);
            } else {
                out.missed.push(clause.clone());
            }
        }
        out
    }

    fn current_user_text(&self) -> String {
        self.state
            .facts
            .iter()
            .rev()
            .find(|f| f.rel == "user_text")
            .and_then(|f| f.args.get("text").cloned())
            .unwrap_or_default()
    }

    fn snapshot(&self) -> Snapshot {
        Snapshot {
            facts: self.state.facts.clone(),
            patterns: self.state.patterns.clone(),
            episodes: self.state.episodes.clone(),
            chains: self.state.chains.clone(),
        }
    }

    fn restore(&mut self, snap: Snapshot, user_text: &str) {
        self.state.facts = snap.facts;
        self.state.patterns = snap.patterns;
        self.state.episodes = snap.episodes;
        self.state.chains = snap.chains;
        self.reset_working(user_text);
    }

    /// Drop this turn's working facts (between clauses and attempts).
    fn reset_working(&mut self, user_text: &str) {
        self.state.facts.retain(|f| f.scope != Scope::Working);
        let args = BTreeMap::from([("text".into(), user_text.to_string())]);
        self.add_fact("user_text".into(), args, Scope::Working, "perceive".into());
    }

    fn prepare_turn(&mut self, input: &str) -> usize {
        self.state.turn += 1;
        self.state.facts.retain(|fact| fact.scope != Scope::Working);
        self.expire_chat_memory();
        self.seed_chat_memory();

        let args = BTreeMap::from([("text".into(), input.to_string())]);
        self.add_fact("user_text".into(), args, Scope::Working, "perceive".into());
        self.state.turn
    }

    /// Chat facts every chat starts with, declared in the vault as
    /// `chat_seed(rel=R, k=v, ...)`: when the chat has no `R` fact, `R(k=v, ...)`
    /// is added to chat memory (e.g. the default speaker).
    fn seed_chat_memory(&mut self) {
        let seeds: Vec<(String, BTreeMap<String, String>)> = self
            .state
            .facts
            .iter()
            .filter(|f| f.rel == "chat_seed")
            .filter_map(|f| {
                let rel = f.args.get("rel")?.clone();
                let mut args = f.args.clone();
                args.remove("rel");
                Some((rel, args))
            })
            .collect();
        for (rel, args) in seeds {
            let present = self
                .state
                .facts
                .iter()
                .any(|f| f.rel == rel && f.scope == Scope::Chat);
            if !present {
                self.add_fact(rel, args, Scope::Chat, "chat_seed".into());
            }
        }
    }

    /// Chat facts the vault declares short-lived with `chat_expiry(rel=..., turns=N)` are
    /// forgotten once N turns have passed since they were last said, e.g. a question
    /// Karin asked that the user moved on from.
    fn expire_chat_memory(&mut self) {
        let limits: Vec<(String, usize)> = self
            .state
            .facts
            .iter()
            .filter(|f| f.rel == "chat_expiry")
            .filter_map(|f| {
                Some((
                    f.args.get("rel")?.clone(),
                    f.args.get("turns")?.trim().parse().ok()?,
                ))
            })
            .collect();
        if limits.is_empty() {
            return;
        }
        let now = self.state.turn;
        self.state.facts.retain(|f| {
            f.scope != Scope::Chat
                || !limits
                    .iter()
                    .any(|(rel, turns)| *rel == f.rel && now.saturating_sub(f.turn) > *turns)
        });
    }

    /// Relations the vault declares as chat memory (`chat_memory(rel=...)` or
    /// `chat_memory(prefix=...)`): a working assert of one of them lives for the chat.
    fn is_chat_memory(&self, rel: &str) -> bool {
        self.state.facts.iter().any(|f| {
            f.rel == "chat_memory"
                && (f.args.get("rel").is_some_and(|r| r == rel)
                    || f.args
                        .get("prefix")
                        .is_some_and(|p| !p.is_empty() && rel.starts_with(p.as_str())))
        })
    }

    fn match_active_patterns(
        &self,
        input: &str,
        fired_keys: &HashSet<String>,
        exact_only: bool,
    ) -> Vec<(Pattern, Bindings, f64)> {
        let mut candidates: Vec<(Pattern, Bindings, f64)> = self
            .state
            .patterns
            .iter()
            .filter(|pat| pat.status == PatternStatus::Active)
            .filter(|pat| !exact_only || taught_on_exact_wording(pat))
            .filter_map(|pat| {
                let bindings = match_conditions(&pat.when, &self.state.facts, input)?;
                if fired_keys.contains(&fire_key(&pat.id, &bindings)) {
                    return None;
                }
                Some((pat.clone(), bindings, score_pattern(pat)))
            })
            .collect();

        candidates.sort_by(|a, b| {
            b.2.partial_cmp(&a.2)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| b.0.support.cmp(&a.0.support))
                .then_with(|| a.0.id.cmp(&b.0.id))
        });

        candidates
    }

    fn fire_pattern(
        &mut self,
        winner: &Pattern,
        bindings: &Bindings,
    ) -> (Vec<String>, Option<String>) {
        let mut effects = Vec::new();
        let mut reply = None;

        for action in &winner.then {
            match action {
                Action::Assert { rel, args, scope } => {
                    let resolved = subst_args(args, bindings);
                    let fact = self.add_fact(
                        rel.clone(),
                        resolved,
                        scope.clone(),
                        format!("pattern:{}", winner.id),
                    );
                    let prefix = match fact.scope {
                        Scope::Working => "+",
                        Scope::Chat => "+chat ",
                        Scope::LongTerm => "++",
                    };
                    effects.push(format!("{prefix}{}", fmt_call(rel, &fact.args)));
                }
                Action::Retract { rel, args } => {
                    let resolved = args.as_ref().map(|a| subst_args(a, bindings));
                    self.retract_matching(rel, resolved.as_ref());
                    match &resolved {
                        Some(filter) => effects.push(format!("-{}", fmt_call(rel, filter))),
                        None => effects.push(format!("-{rel}")),
                    }
                }
                Action::Tool { name, args } => {
                    let resolved = subst_args(args, bindings);
                    let summary = self.invoke_tool(name, &resolved);
                    effects.push(format!("@{} → {summary}", fmt_call(name, &resolved)));
                }
                Action::Reply { template } => {
                    let text = subst(template, bindings);
                    effects.push(format!("=> \"{text}\""));
                    reply = Some(text);
                }
            }
        }

        (effects, reply)
    }

    fn record_turn(
        &mut self,
        turn: usize,
        input: String,
        final_reply: Option<String>,
        impasse_reply: Option<String>,
        attempt: Attempt,
    ) -> TurnTrace {
        let impasse = final_reply.is_none();
        let reply = final_reply
            .or(impasse_reply)
            .unwrap_or_else(|| IMPASSE_REPLY.into());
        let Attempt {
            steps,
            fired,
            bindings,
            understood,
            missed,
            ..
        } = attempt;

        if !impasse && fired.len() >= 2 {
            let chain = ChainRecord {
                id: self.alloc_id("c"),
                turn,
                input: input.clone(),
                pattern_ids: fired.clone(),
                bindings,
                chunked: false,
            };
            self.state.chains.push(chain);
        }

        // The chat remembers what was said (bounded like all chat memory).
        for (by, text) in [("user", &input), ("karin", &reply)] {
            let args = BTreeMap::from([("by".into(), by.into()), ("text".into(), text.clone())]);
            self.add_fact("said".into(), args, Scope::Chat, "perceive".into());
        }

        let chat_memory: Vec<Fact> = self
            .state
            .facts
            .iter()
            .filter(|f| f.scope == Scope::Chat)
            .cloned()
            .collect();

        let trace = TurnTrace {
            turn,
            input: input.clone(),
            reply: reply.clone(),
            impasse,
            steps,
            fired_pattern_ids: fired,
            chunked_pattern_id: None,
            understood,
            missed,
            attempts: std::mem::take(&mut self.attempt_log),
            chat_memory,
        };

        let user_msg = ChatMessage {
            id: self.alloc_id("m"),
            role: "user".into(),
            text: input,
            turn,
            trace: None,
            feedback: None,
        };
        let agent_msg = ChatMessage {
            id: self.alloc_id("m"),
            role: "agent".into(),
            text: reply,
            turn,
            trace: Some(trace.clone()),
            feedback: None,
        };
        self.state.messages.push(user_msg);
        self.state.messages.push(agent_msg);

        trace
    }

    fn retract_matching(&mut self, rel: &str, filter: Option<&BTreeMap<String, String>>) {
        self.state.facts.retain(|fact| {
            if fact.rel != rel {
                return true;
            }
            match filter {
                None => false,
                Some(required) => !required.iter().all(|(key, val)| {
                    fact.args
                        .get(key)
                        .map(|actual| normalize(actual) == normalize(val))
                        .unwrap_or(false)
                }),
            }
        });
    }

    fn invoke_tool(&mut self, name: &str, args: &BTreeMap<String, String>) -> String {
        match name {
            "generalize_episodes" => {
                let (created, summary) = self.generalize_episodes(false);
                let fact_args = BTreeMap::from([
                    ("summary".into(), summary.clone()),
                    ("count".into(), created.len().to_string()),
                ]);
                self.add_fact(
                    "meta_result".into(),
                    fact_args,
                    Scope::Working,
                    "meta:generalize".into(),
                );
                summary
            }
            "chunk_last_chain" => {
                let (_, summary) = self.chunk_last_chain(false);
                let fact_args = BTreeMap::from([("summary".into(), summary.clone())]);
                self.add_fact(
                    "meta_result".into(),
                    fact_args,
                    Scope::Working,
                    "meta:chunk".into(),
                );
                summary
            }
            "induce_from_correction" => {
                let item = args.get("item").map(String::as_str).unwrap_or("");
                let topic = args.get("topic").map(String::as_str).unwrap_or("");
                self.induce_from_correction(item, topic)
            }
            "approve_pattern" => {
                let id = args.get("id").map(String::as_str).unwrap_or("");
                let ok = self.promote_pattern(id);
                self.add_fact(
                    "approval_result".into(),
                    BTreeMap::from([
                        ("ok".into(), if ok { "true" } else { "false" }.into()),
                        ("id".into(), id.into()),
                    ]),
                    Scope::Working,
                    "meta:approve".into(),
                );
                if ok {
                    format!("approved pattern {id}")
                } else {
                    format!("pattern {id} not found")
                }
            }
            other => match run_world_tool(other, args, &self.state.facts) {
                Some(outcome) => {
                    for NewFact {
                        rel,
                        args,
                        scope,
                        provenance,
                    } in outcome.facts
                    {
                        self.add_fact(rel, args, scope, provenance);
                    }
                    outcome.summary
                }
                None => format!("unknown tool '{other}'"),
            },
        }
    }
}

/// Patterns learned for one exact wording: taught replies and hand-added rules.
fn taught_on_exact_wording(pattern: &Pattern) -> bool {
    pattern.provenance.starts_with("taught") || pattern.provenance == "manual"
}

fn fire_key(pattern_id: &str, bindings: &Bindings) -> String {
    format!("{pattern_id}::{bindings:?}")
}

fn top_competing_matches(candidates: &[(Pattern, Bindings, f64)]) -> Vec<CompetingMatch> {
    candidates
        .iter()
        .take(5)
        .map(|(pat, bindings, score)| CompetingMatch {
            pattern_id: pat.id.clone(),
            name: pat.name.clone(),
            score: (score * 100.0).round() / 100.0,
            bindings: bindings.clone(),
            parts: score_parts(pat),
        })
        .collect()
}

fn score_pattern(pattern: &Pattern) -> f64 {
    score_parts(pattern).iter().map(|p| p.value).sum()
}

/// A pattern's score, split into named parts so the trace can show why it won.
fn score_parts(pattern: &Pattern) -> Vec<ScorePart> {
    let mut text = 0.0;
    let mut contains = 0.0;
    let mut facts = 0.0;
    let mut negations = 0.0;
    for cond in &pattern.when {
        match cond {
            Condition::Text { template } => {
                let literals = template
                    .split_whitespace()
                    .filter(|tok| !is_var(tok))
                    .count();
                text += 3.0 + (literals as f64) * 0.6;
            }
            Condition::Contains { .. } => contains += 1.0,
            Condition::Fact { .. } => facts += 2.0,
            Condition::NotFact { .. } => negations += 0.4,
            Condition::NotEq { .. } | Condition::WordCount { .. } => {}
        }
    }
    let composite = if pattern.kind == PatternKind::Composite { 1.2 } else { 0.0 };
    let reply = if pattern.then.iter().any(|act| matches!(act, Action::Reply { .. })) {
        2.5
    } else {
        0.0
    };
    [
        ("trust", pattern.trust * 2.0),
        ("text", text),
        ("contains", contains),
        ("facts", facts),
        ("negations", negations),
        ("composite", composite),
        ("reply", reply),
    ]
    .into_iter()
    .filter(|(_, v)| *v != 0.0)
    .map(|(label, value)| ScorePart {
        label: label.into(),
        value,
    })
    .collect()
}

fn fmt_call(name: &str, args: &BTreeMap<String, String>) -> String {
    let inner: Vec<String> = args.iter().map(|(k, v)| format!("{k}=\"{v}\"")).collect();
    format!("{name}({})", inner.join(", "))
}

#[derive(Clone)]
struct Snapshot {
    facts: Vec<Fact>,
    patterns: Vec<Pattern>,
    episodes: Vec<crate::types::Episode>,
    chains: Vec<ChainRecord>,
}

#[derive(Default)]
struct Attempt {
    steps: Vec<StepTrace>,
    fired: Vec<String>,
    bindings: Bindings,
    replies: Vec<String>,
    understood: Vec<String>,
    missed: Vec<String>,
}

impl Attempt {
    fn answered(&self) -> bool {
        !self.replies.is_empty() && self.missed.is_empty()
    }

    fn absorb(&mut self, other: Attempt) {
        let offset = self.steps.len();
        self.steps.extend(other.steps.into_iter().map(|mut s| {
            s.step += offset;
            s
        }));
        self.fired.extend(other.fired);
        self.bindings.extend(other.bindings);
        self.replies.extend(other.replies);
        self.understood.extend(other.understood);
    }
}

/// "calculate 5 * 7" + "6 * 9" -> "calculate 6 * 9": the previous clause's
/// trailing captured value is replaced by the new clause.
fn fill_ellipsis(prev_text: &str, prev_bindings: &Bindings, clause: &str) -> Option<String> {
    let prev_norm = normalize(prev_text);
    prev_bindings
        .values()
        .map(|v| normalize(v))
        .filter(|v| !v.is_empty() && prev_norm.ends_with(v.as_str()) && prev_norm.len() > v.len())
        .max_by_key(|v| v.len())
        .map(|v| {
            let head = &prev_norm[..prev_norm.len() - v.len()];
            format!("{} {}", head.trim_end(), clause)
        })
}

/// What to say when nothing matched: a hint if a keyword is known, a request
/// to repeat for one-word fragments, otherwise an invitation to teach.
fn impasse_reply(lex: &Lexicon, raw: &str, normalized: &str) -> String {
    if let Some(hint) = lex.hint_for(normalized) {
        return format!("I'm not sure what you meant. {hint}");
    }
    if Lexicon::token_count(normalized) == 1 {
        return format!("I only caught \"{normalized}\". Could you say the whole sentence?");
    }
    let shown: String = if raw.chars().count() > 80 {
        format!("{}...", raw.chars().take(80).collect::<String>())
    } else {
        raw.to_string()
    };
    format!(
        "I don't understand \"{shown}\" yet. You can teach me a reply, or say \"help\" to hear what I can do."
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        Engine::from_vault_dir(&crate::vault::default_vault_dir()).expect("vault")
    }

    #[test]
    fn unanswered_question_expires_after_one_turn() {
        let mut eng = engine();
        let ask = eng.turn("make a note");
        assert!(ask.reply.contains("what should the note say"), "{}", ask.reply);
        let _other = eng.turn("how do I bake bread");
        let later = eng.turn("my dad just texted me");
        assert!(
            !later.reply.contains("Saved to long-term memory"),
            "stale question captured: {}",
            later.reply
        );
    }

    #[test]
    fn answered_question_still_captured_next_turn() {
        let mut eng = engine();
        eng.turn("make a note");
        let saved = eng.turn("buy oat milk");
        assert!(saved.reply.contains("buy oat milk"), "{}", saved.reply);
    }

    #[test]
    fn name_guard_rejects_long_captures() {
        let mut eng = engine();
        let ok = eng.turn("Oscar is my friend");
        assert!(ok.reply.contains("Oscar is your friend"), "{}", ok.reply);
        let noise = eng.turn("can a roof is my best friend");
        assert!(!noise.reply.contains("your best friend"), "{}", noise.reply);
    }
}
