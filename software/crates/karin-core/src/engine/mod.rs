mod learn;
mod selftest;

use crate::tools::{NewFact, run_world_tool};
use crate::types::{
    Action, AgentState, ChainRecord, ChatMessage, CompetingMatch, Condition, Fact, Pattern,
    PatternKind, PatternStatus, Scope, StepTrace, TurnTrace,
};
use crate::unify::{Bindings, is_var, match_conditions, normalize, subst, subst_args};
use crate::vault::{initial_state, load_vault};
use std::collections::{BTreeMap, HashSet};
use std::path::Path;

const MAX_STEPS_PER_TURN: usize = 12;
const IMPASSE_REPLY: &str =
    "I don't know how to handle that yet (impasse). Teach me a reply or pattern and I'll learn it.";

pub struct Engine {
    pub state: AgentState,
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
        }
    }

    pub fn from_vault_dir(dir: &Path) -> Result<Self, String> {
        Ok(Self {
            state: load_vault(dir)?,
        })
    }

    pub fn from_state(state: AgentState) -> Self {
        Self { state }
    }

    pub fn reset(&mut self) {
        self.state = initial_state();
    }

    pub fn reset_from_vault_dir(&mut self, dir: &Path) -> Result<(), String> {
        self.state = load_vault(dir)?;
        Ok(())
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
        let trimmed = input.trim().to_string();
        let turn = self.prepare_turn(&trimmed);

        let mut steps = Vec::new();
        let mut fired_ids = Vec::new();
        let mut fired_keys = HashSet::new();
        let mut merged_bindings = Bindings::new();
        let mut final_reply = None;

        for step in 1..=MAX_STEPS_PER_TURN {
            let mut candidates = self.match_active_patterns(&trimmed, &fired_keys);
            if candidates.is_empty() {
                break;
            }

            let competing = top_competing_matches(&candidates);
            let (winner, bindings, _) = candidates.remove(0);

            fired_keys.insert(fire_key(&winner.id, &bindings));
            fired_ids.push(winner.id.clone());
            merged_bindings.extend(bindings.clone());

            let (effects, reply) = self.fire_pattern(&winner, &bindings);
            steps.push(StepTrace {
                step,
                competing,
                winner_id: winner.id,
                winner_name: winner.name,
                bindings,
                effects,
            });

            if let Some(text) = reply {
                final_reply = Some(text);
                break;
            }
        }

        self.record_turn(
            turn,
            trimmed,
            final_reply,
            steps,
            fired_ids,
            merged_bindings,
        )
    }

    fn prepare_turn(&mut self, input: &str) -> usize {
        self.state.turn += 1;
        self.state.facts.retain(|fact| {
            fact.scope == Scope::LongTerm
                || fact.rel == "asked_slot"
                || fact.rel.starts_with("dialogue_")
        });

        let args = BTreeMap::from([("text".into(), input.to_string())]);
        self.add_fact("user_text".into(), args, Scope::Working, "perceive".into());
        self.state.turn
    }

    fn match_active_patterns(
        &self,
        input: &str,
        fired_keys: &HashSet<String>,
    ) -> Vec<(Pattern, Bindings, f64)> {
        let mut candidates: Vec<(Pattern, Bindings, f64)> = self
            .state
            .patterns
            .iter()
            .filter(|pat| pat.status == PatternStatus::Active)
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
                    effects.push(format!("assert {rel}({resolved:?})"));
                    self.add_fact(
                        rel.clone(),
                        resolved,
                        scope.clone(),
                        format!("pattern:{}", winner.id),
                    );
                }
                Action::Retract { rel, args } => {
                    let resolved = args.as_ref().map(|a| subst_args(a, bindings));
                    self.retract_matching(rel, resolved.as_ref());
                    effects.push(format!("retract {rel}"));
                }
                Action::Tool { name, args } => {
                    let resolved = subst_args(args, bindings);
                    let summary = self.invoke_tool(name, &resolved);
                    effects.push(format!("tool {summary}"));
                }
                Action::Reply { template } => {
                    let text = subst(template, bindings);
                    effects.push(format!("reply \"{text}\""));
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
        steps: Vec<StepTrace>,
        fired_ids: Vec<String>,
        merged_bindings: Bindings,
    ) -> TurnTrace {
        let impasse = final_reply.is_none();
        let reply = final_reply.unwrap_or_else(|| IMPASSE_REPLY.into());

        if !impasse && fired_ids.len() >= 2 {
            let chain = ChainRecord {
                id: self.alloc_id("c"),
                turn,
                input: input.clone(),
                pattern_ids: fired_ids.clone(),
                bindings: merged_bindings,
                chunked: false,
            };
            self.state.chains.push(chain);
        }

        let trace = TurnTrace {
            turn,
            input: input.clone(),
            reply: reply.clone(),
            impasse,
            steps,
            fired_pattern_ids: fired_ids,
            chunked_pattern_id: None,
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
        })
        .collect()
}

fn score_pattern(pattern: &Pattern) -> f64 {
    let mut score = pattern.trust * 2.0;

    for cond in &pattern.when {
        score += match cond {
            Condition::Text { template } => {
                let literals = template
                    .split_whitespace()
                    .filter(|tok| !is_var(tok))
                    .count();
                3.0 + (literals as f64) * 0.6
            }
            Condition::Contains { .. } => 1.0,
            Condition::Fact { .. } => 2.0,
            Condition::NotFact { .. } | Condition::NotEq { .. } => 0.4,
        };
    }

    if pattern.kind == PatternKind::Composite {
        score += 1.2;
    }
    if pattern
        .then
        .iter()
        .any(|act| matches!(act, Action::Reply { .. }))
    {
        score += 2.5;
    }
    score
}
