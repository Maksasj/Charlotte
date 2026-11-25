use crate::fixtures::{initial_state, load_fixtures};
use crate::tools::{NewFact, run_world_tool};
use crate::types::{
    Action, AgentState, ChainRecord, ChatMessage, CompetingMatch, Condition, Episode, Fact,
    Pattern, PatternKind, PatternStatus, Scope, SelfTestResult, StepTrace, TurnTrace,
};
use crate::unify::{Bindings, is_var, match_conditions, normalize, subst, subst_args};
use std::collections::{BTreeMap, HashSet};
use std::path::Path;

pub struct Engine {
    pub state: AgentState,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            state: initial_state(),
        }
    }

    pub fn from_fixtures_dir(dir: &Path) -> Result<Self, String> {
        Ok(Self {
            state: load_fixtures(dir)?,
        })
    }

    pub fn from_state(state: AgentState) -> Self {
        Self { state }
    }

    pub fn reset(&mut self) {
        self.state = initial_state();
    }

    pub fn reset_from_fixtures_dir(&mut self, dir: &Path) -> Result<(), String> {
        self.state = load_fixtures(dir)?;
        Ok(())
    }

    fn alloc_id(&mut self, prefix: &str) -> String {
        let id = format!("{prefix}{}", self.state.next_id);
        self.state.next_id += 1;
        id
    }

    pub fn add_fact(&mut self, rel: String, args: BTreeMap<String, String>, scope: Scope, provenance: String) -> Fact {
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
        self.state.facts.retain(|f| f.id != id);
        self.state.facts.len() < before
    }

    pub fn add_pattern(&mut self, mut p: Pattern) -> Pattern {
        if p.id.is_empty() {
            p.id = self.alloc_id("p");
        }
        self.state.patterns.push(p.clone());
        p
    }

    pub fn promote_pattern(&mut self, id: &str) -> bool {
        if let Some(p) = self.state.patterns.iter_mut().find(|p| p.id == id) {
            p.status = PatternStatus::Active;
            true
        } else {
            false
        }
    }

    pub fn delete_pattern(&mut self, id: &str) -> bool {
        let before = self.state.patterns.len();
        self.state.patterns.retain(|p| p.id != id);
        self.state.patterns.len() < before
    }

    /// Run one full Perceive -> Match -> Select -> Fire turn.
    pub fn turn(&mut self, input: &str) -> TurnTrace {
        let trimmed = input.trim().to_string();
        self.state.turn += 1;
        let turn = self.state.turn;

        // Clear transient working facts except carry-over slot-asking state
        self.state
            .facts
            .retain(|f| f.scope == Scope::LongTerm || f.rel == "asked_slot");

        // Perceive: assert current user input as a working fact
        let mut inp_args = BTreeMap::new();
        inp_args.insert("text".into(), trimmed.clone());
        self.add_fact("user_text".into(), inp_args, Scope::Working, "perceive".into());

        let mut steps: Vec<StepTrace> = Vec::new();
        let mut fired_ids: Vec<String> = Vec::new();
        let mut fired_keys: HashSet<String> = HashSet::new();
        let mut merged_bindings: Bindings = Bindings::new();
        let mut final_reply: Option<String> = None;

        for step_num in 1..=12 {
            // Match all active patterns
            let mut candidates: Vec<(Pattern, Bindings, f64)> = Vec::new();
            for pat in &self.state.patterns {
                if pat.status != PatternStatus::Active {
                    continue;
                }
                if let Some(bindings) = match_conditions(&pat.when, &self.state.facts, &trimmed) {
                    let key = format!("{}::{:?}", pat.id, bindings);
                    if fired_keys.contains(&key) {
                        continue;
                    }
                    let score = score_pattern(pat);
                    candidates.push((pat.clone(), bindings, score));
                }
            }

            if candidates.is_empty() {
                break;
            }

            // Select highest-scoring match (tie-break by support, then id)
            candidates.sort_by(|a, b| {
                b.2.partial_cmp(&a.2)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| b.0.support.cmp(&a.0.support))
                    .then_with(|| a.0.id.cmp(&b.0.id))
            });

            let competing: Vec<CompetingMatch> = candidates
                .iter()
                .take(5)
                .map(|(p, b, s)| CompetingMatch {
                    pattern_id: p.id.clone(),
                    name: p.name.clone(),
                    score: (*s * 100.0).round() / 100.0,
                    bindings: b.clone(),
                })
                .collect();

            let (winner, bindings, _) = candidates.remove(0);
            let key = format!("{}::{:?}", winner.id, bindings);
            fired_keys.insert(key);
            fired_ids.push(winner.id.clone());
            for (k, v) in &bindings {
                merged_bindings.insert(k.clone(), v.clone());
            }

            // Fire winner's actions
            let mut effects: Vec<String> = Vec::new();
            for action in &winner.then {
                match action {
                    Action::Assert { rel, args, scope } => {
                        let resolved = subst_args(args, &bindings);
                        effects.push(format!("assert {rel}({:?})", resolved));
                        self.add_fact(
                            rel.clone(),
                            resolved,
                            scope.clone(),
                            format!("pattern:{}", winner.id),
                        );
                    }
                    Action::Retract { rel, args } => {
                        let resolved = args.as_ref().map(|a| subst_args(a, &bindings));
                        self.retract_matching(rel, resolved.as_ref());
                        effects.push(format!("retract {rel}"));
                    }
                    Action::Tool { name, args } => {
                        let resolved = subst_args(args, &bindings);
                        let summary = self.invoke_tool(name, &resolved);
                        effects.push(format!("tool {summary}"));
                    }
                    Action::Reply { template } => {
                        let text = subst(template, &bindings);
                        effects.push(format!("reply \"{text}\""));
                        final_reply = Some(text);
                    }
                }
            }

            steps.push(StepTrace {
                step: step_num,
                competing,
                winner_id: winner.id.clone(),
                winner_name: winner.name.clone(),
                bindings,
                effects,
            });

            if final_reply.is_some() {
                break;
            }
        }

        let impasse = final_reply.is_none();
        let reply = final_reply.unwrap_or_else(|| {
            "I don't know how to handle that yet (impasse). Teach me a reply or pattern and I'll learn it.".into()
        });

        // Record multi-step chains for chunking
        if !impasse && fired_ids.len() >= 2 {
            let chain = ChainRecord {
                id: self.alloc_id("c"),
                turn,
                input: trimmed.clone(),
                pattern_ids: fired_ids.clone(),
                bindings: merged_bindings,
                chunked: false,
            };
            self.state.chains.push(chain);
        }

        let trace = TurnTrace {
            turn,
            input: trimmed.clone(),
            reply: reply.clone(),
            impasse,
            steps,
            fired_pattern_ids: fired_ids,
            chunked_pattern_id: None,
        };

        let user_msg = ChatMessage {
            id: self.alloc_id("m"),
            role: "user".into(),
            text: trimmed,
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
        self.state.facts.retain(|f| {
            if f.rel != rel {
                return true;
            }
            match filter {
                None => false,
                Some(req) => !req.iter().all(|(k, v)| {
                    f.args
                        .get(k)
                        .map(|fv| normalize(fv) == normalize(v))
                        .unwrap_or(false)
                }),
            }
        });
    }

    fn invoke_tool(&mut self, name: &str, args: &BTreeMap<String, String>) -> String {
        match name {
            "generalize_episodes" => {
                let (created, summary) = self.generalize_episodes(false);
                let mut fargs = BTreeMap::new();
                fargs.insert("summary".into(), summary.clone());
                fargs.insert("count".into(), created.len().to_string());
                self.add_fact("meta_result".into(), fargs, Scope::Working, "meta:generalize".into());
                summary
            }
            "chunk_last_chain" => {
                let (_, summary) = self.chunk_last_chain(false);
                let mut fargs = BTreeMap::new();
                fargs.insert("summary".into(), summary.clone());
                self.add_fact("meta_result".into(), fargs, Scope::Working, "meta:chunk".into());
                summary
            }
            other => {
                if let Some(outcome) = run_world_tool(other, args, &self.state.facts) {
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
                } else {
                    format!("unknown tool '{other}'")
                }
            }
        }
    }

    /// Teach a concrete example (adds Episode + candidate or active pattern).
    pub fn teach_reply(&mut self, input: &str, reply: &str, promote: bool) -> (Episode, Pattern) {
        let ep = Episode {
            id: self.alloc_id("e"),
            input: input.trim().to_string(),
            reply: reply.trim().to_string(),
            source: "taught".into(),
            generalized: false,
        };
        self.state.episodes.push(ep.clone());

        let pat = Pattern {
            id: self.alloc_id("p"),
            name: format!("taught-{}", slug(input)),
            status: if promote {
                PatternStatus::Active
            } else {
                PatternStatus::Candidate
            },
            when: vec![Condition::Text {
                template: input.trim().to_string(),
            }],
            then: vec![Action::Reply {
                template: reply.trim().to_string(),
            }],
            trust: 0.9,
            support: 1,
            provenance: format!("taught:{}", ep.id),
            kind: PatternKind::Reply,
        };
        self.state.patterns.push(pat.clone());
        (ep, pat)
    }

    /// Adjust trust of patterns that fired on a given turn.
    pub fn feedback(&mut self, turn: usize, positive: bool) -> bool {
        let msg = self
            .state
            .messages
            .iter_mut()
            .find(|m| m.role == "agent" && m.turn == turn);
        let Some(msg) = msg else {
            return false;
        };
        msg.feedback = Some(if positive { "up".into() } else { "down".into() });
        let fired = msg
            .trace
            .as_ref()
            .map(|t| t.fired_pattern_ids.clone())
            .unwrap_or_default();
        for pid in fired {
            if let Some(p) = self.state.patterns.iter_mut().find(|p| p.id == pid) {
                if positive {
                    p.trust = (p.trust + 0.05).min(1.0);
                    p.support += 1;
                } else {
                    p.trust = (p.trust - 0.15).max(0.1);
                }
            }
        }
        true
    }

    /// Anti-unify un-generalized episodes that share a token skeleton and coherent reply substitution.
    pub fn generalize_episodes(&mut self, auto_promote: bool) -> (Vec<Pattern>, String) {
        let ungen_indices: Vec<usize> = self
            .state
            .episodes
            .iter()
            .enumerate()
            .filter(|(_, e)| !e.generalized)
            .map(|(i, _)| i)
            .collect();

        let mut used: HashSet<usize> = HashSet::new();
        let mut created: Vec<Pattern> = Vec::new();

        for idx_pos in 0..ungen_indices.len() {
            let i = ungen_indices[idx_pos];
            if used.contains(&i) {
                continue;
            }
            let base_tokens: Vec<String> = self.state.episodes[i]
                .input
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            if base_tokens.len() < 2 {
                continue;
            }

            // Try to find peers with same length differing at exactly 1 token index
            for diff_idx in 0..base_tokens.len() {
                let mut group = vec![i];
                for &j in &ungen_indices[(idx_pos + 1)..] {
                    if used.contains(&j) {
                        continue;
                    }
                    let other_tokens: Vec<String> = self.state.episodes[j]
                        .input
                        .split_whitespace()
                        .map(|s| s.to_string())
                        .collect();
                    if other_tokens.len() != base_tokens.len() {
                        continue;
                    }
                    let same_skeleton = (0..base_tokens.len()).all(|k| {
                        if k == diff_idx {
                            normalize(&other_tokens[k]) != normalize(&base_tokens[k])
                        } else {
                            normalize(&other_tokens[k]) == normalize(&base_tokens[k])
                        }
                    });
                    if same_skeleton {
                        group.push(j);
                    }
                }

                if group.len() < 2 {
                    continue;
                }

                // Check reply coherence: replacing each episode's varying word with ?x must produce the same reply template!
                let mut reply_templates: HashSet<String> = HashSet::new();
                let mut first_reply_template = String::new();
                let mut all_valid = true;

                for &gi in &group {
                    let ep = &self.state.episodes[gi];
                    let toks: Vec<&str> = ep.input.split_whitespace().collect();
                    let var_val = toks[diff_idx]
                        .trim_matches(|c: char| matches!(c, '?' | '!' | '.' | ','));
                    let replaced = replace_word_case_insensitive(&ep.reply, var_val, "?x");
                    if first_reply_template.is_empty() {
                        first_reply_template = replaced.clone();
                    }
                    reply_templates.insert(normalize(&replaced));
                    if reply_templates.len() > 1 {
                        all_valid = false;
                        break;
                    }
                }

                if !all_valid || first_reply_template.is_empty() {
                    continue;
                }

                // Build generalized input template
                let mut tmpl_tokens = base_tokens.clone();
                tmpl_tokens[diff_idx] = "?x".into();
                let input_template = tmpl_tokens.join(" ");

                let ep_ids: Vec<String> = group
                    .iter()
                    .map(|&gi| self.state.episodes[gi].id.clone())
                    .collect();

                let pat = Pattern {
                    id: self.alloc_id("p"),
                    name: format!("induced-{}", slug(&input_template)),
                    status: if auto_promote {
                        PatternStatus::Active
                    } else {
                        PatternStatus::Candidate
                    },
                    when: vec![Condition::Text {
                        template: input_template.clone(),
                    }],
                    then: vec![Action::Reply {
                        template: first_reply_template.clone(),
                    }],
                    trust: 0.88,
                    support: group.len(),
                    provenance: format!("anti_unify({})", ep_ids.join(",")),
                    kind: PatternKind::Induced,
                };

                self.state.patterns.push(pat.clone());
                created.push(pat);

                for &gi in &group {
                    used.insert(gi);
                    self.state.episodes[gi].generalized = true;
                }
                break;
            }
        }

        let summary = if created.is_empty() {
            "No new generalizable episode groups found.".into()
        } else {
            let names: Vec<String> = created.iter().map(|p| p.name.clone()).collect();
            format!(
                "Induced {} pattern(s) via anti-unification: {} (review & promote in Patterns).",
                created.len(),
                names.join(", ")
            )
        };
        (created, summary)
    }

    /// Compile the most recent un-chunked multi-step chain into a single composite pattern.
    pub fn chunk_last_chain(&mut self, auto_promote: bool) -> (Option<Pattern>, String) {
        let Some(chain_idx) = self
            .state
            .chains
            .iter()
            .rposition(|c| !c.chunked && c.pattern_ids.len() >= 2)
        else {
            return (None, "No un-chunked multi-step chain available yet. Try asking for weather or a calculation first.".into());
        };

        let chain = self.state.chains[chain_idx].clone();
        let chain_patterns: Vec<Pattern> = chain
            .pattern_ids
            .iter()
            .filter_map(|pid| self.state.patterns.iter().find(|p| p.id == *pid).cloned())
            .collect();

        if chain_patterns.len() < 2 {
            return (None, "Chain patterns no longer exist.".into());
        }

        let mut produced_working_rels: HashSet<String> = HashSet::new();
        let mut produced_arg_map: BTreeMap<(String, String), String> = BTreeMap::new();
        let mut var_remap: BTreeMap<String, String> = BTreeMap::new();

        let mut external_when: Vec<Condition> = Vec::new();
        let mut composed_then: Vec<Action> = Vec::new();
        let mut saw_tool = false;
        let mut folded_ids: Vec<String> = Vec::new();

        for pat in &chain_patterns {
            // Once a tool runs, subsequent patterns read tool-produced facts,
            // so stop folding here and let the existing reply pattern handle the tool output.
            if saw_tool {
                break;
            }
            folded_ids.push(pat.id.clone());

            // Map any Fact condition that consumes an earlier working assert
            for cond in &pat.when {
                match cond {
                    Condition::Fact { rel, args } if produced_working_rels.contains(rel) => {
                        for (k, v) in args {
                            if is_var(v) {
                                if let Some(origin_var) = produced_arg_map.get(&(rel.clone(), k.clone())) {
                                    var_remap.insert(v.clone(), origin_var.clone());
                                }
                            }
                        }
                    }
                    Condition::NotFact { rel, .. } if produced_working_rels.contains(rel) => {
                        // Guard on intermediate working fact — skip in composite
                    }
                    Condition::Text { template } => {
                        external_when.push(Condition::Text {
                            template: remap_str(template, &var_remap),
                        });
                    }
                    Condition::Contains { phrase } => {
                        external_when.push(Condition::Contains {
                            phrase: phrase.clone(),
                        });
                    }
                    Condition::Fact { rel, args } => {
                        external_when.push(Condition::Fact {
                            rel: rel.clone(),
                            args: remap_args(args, &var_remap),
                        });
                    }
                    Condition::NotFact { rel, args } => {
                        external_when.push(Condition::NotFact {
                            rel: rel.clone(),
                            args: remap_args(args, &var_remap),
                        });
                    }
                }
            }

            // Collect actions
            for act in &pat.then {
                match act {
                    Action::Assert { rel, args, scope } if *scope == Scope::Working => {
                        produced_working_rels.insert(rel.clone());
                        for (k, v) in args {
                            produced_arg_map
                                .insert((rel.clone(), k.clone()), remap_str(v, &var_remap));
                        }
                    }
                    Action::Assert { rel, args, scope } => {
                        composed_then.push(Action::Assert {
                            rel: rel.clone(),
                            args: remap_args(args, &var_remap),
                            scope: scope.clone(),
                        });
                    }
                    Action::Retract { rel, args } => {
                        composed_then.push(Action::Retract {
                            rel: rel.clone(),
                            args: args.as_ref().map(|a| remap_args(a, &var_remap)),
                        });
                    }
                    Action::Tool { name, args } => {
                        saw_tool = true;
                        composed_then.push(Action::Tool {
                            name: name.clone(),
                            args: remap_args(args, &var_remap),
                        });
                    }
                    Action::Reply { template } => {
                        composed_then.push(Action::Reply {
                            template: remap_str(template, &var_remap),
                        });
                    }
                }
            }
        }

        if folded_ids.len() < 2 {
            return (None, "Chain cannot be folded further (tool is already in step 1).".into());
        }

        let pat = Pattern {
            id: self.alloc_id("p"),
            name: format!("chunk-{}", folded_ids.join("-")),
            status: if auto_promote {
                PatternStatus::Active
            } else {
                PatternStatus::Candidate
            },
            when: external_when,
            then: composed_then,
            trust: 0.94,
            support: 1,
            provenance: format!("chunk({})", folded_ids.join("->")),
            kind: PatternKind::Composite,
        };

        self.state.chains[chain_idx].chunked = true;
        self.state.patterns.push(pat.clone());

        let summary = format!(
            "Chunked chain [{}] into composite pattern `{}` (review & promote in Patterns).",
            chain.pattern_ids.join(" -> "),
            pat.name
        );
        (Some(pat), summary)
    }

    /// Run 4 deterministic end-to-end tests on a fresh engine instance.
    pub fn run_selftests() -> Vec<SelfTestResult> {
        let mut results = Vec::new();

        // Test 1: Persona + multi-turn working memory
        {
            let mut eng = Engine::new();
            let t1 = eng.turn("who are you");
            let t2 = eng.turn("my name is Alex");
            let t3 = eng.turn("hello");
            let t4 = eng.turn("weather");
            let t5 = eng.turn("Paris");
            let ok = t1.reply.contains("Karin")
                && t2.reply.contains("Alex")
                && t3.reply.contains("Alex")
                && t4.reply.contains("Which city")
                && t5.reply.contains("partly cloudy");
            results.push(SelfTestResult {
                name: "Persona & multi-turn slot context".into(),
                passed: ok,
                detail: format!(
                    "persona='{}' | multi-turn weather='{}'",
                    t1.reply, t5.reply
                ),
            });
        }

        // Test 2: Impasse -> Teach -> Succeed
        {
            let mut eng = Engine::new();
            let t1 = eng.turn("launch rockets");
            let (_, pat) = eng.teach_reply("launch rockets", "Ignition sequence started!", true);
            let t2 = eng.turn("launch rockets");
            let ok = t1.impasse && !t2.impasse && t2.reply == "Ignition sequence started!";
            results.push(SelfTestResult {
                name: "Impasse detection & live teaching".into(),
                passed: ok,
                detail: format!("impasse={} -> taught {} -> '{}'", t1.impasse, pat.id, t2.reply),
            });
        }

        // Test 3: Anti-unification generalizes unseen input
        {
            let mut eng = Engine::new();
            let before = eng.turn("greet Clara");
            let (created, _) = eng.generalize_episodes(true);
            let after = eng.turn("greet Clara");
            let ok = before.impasse
                && !created.is_empty()
                && !after.impasse
                && after.reply.contains("Clara");
            results.push(SelfTestResult {
                name: "Anti-unification generalization".into(),
                passed: ok,
                detail: format!(
                    "induced {} pattern(s) -> unseen 'greet Clara' => '{}'",
                    created.len(),
                    after.reply
                ),
            });
        }

        // Test 4: Chain chunking reduces steps on next turn
        {
            let mut eng = Engine::new();
            let t1 = eng.turn("calculate 6 * 7");
            let (chunked, _) = eng.chunk_last_chain(true);
            let t2 = eng.turn("calculate 9 * 9");
            let ok = t1.reply.contains("42")
                && t1.steps.len() == 3
                && chunked.is_some()
                && t2.reply.contains("81")
                && t2.steps.len() < t1.steps.len();
            results.push(SelfTestResult {
                name: "Chain chunking & composition".into(),
                passed: ok,
                detail: format!(
                    "before={} steps ('{}') -> after={} steps ('{}')",
                    t1.steps.len(),
                    t1.reply,
                    t2.steps.len(),
                    t2.reply
                ),
            });
        }

        results
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

fn score_pattern(p: &Pattern) -> f64 {
    let mut score = p.trust * 2.0;
    for c in &p.when {
        match c {
            Condition::Text { template } => {
                let literals = template
                    .split_whitespace()
                    .filter(|t| !is_var(t))
                    .count();
                score += 3.0 + (literals as f64) * 0.6;
            }
            Condition::Contains { .. } => score += 1.0,
            Condition::Fact { .. } => score += 2.0,
            Condition::NotFact { .. } => score += 0.4,
        }
    }
    if p.kind == PatternKind::Composite {
        score += 1.2;
    }
    if p.then.iter().any(|a| matches!(a, Action::Reply { .. })) {
        score += 2.5;
    }
    score
}

fn slug(s: &str) -> String {
    let mut out = String::new();
    for ch in s.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else if !out.ends_with('-') && !out.is_empty() {
            out.push('-');
        }
        if out.len() >= 24 {
            break;
        }
    }
    out.trim_matches('-').to_string()
}

fn replace_word_case_insensitive(text: &str, needle: &str, replacement: &str) -> String {
    if needle.is_empty() {
        return text.to_string();
    }
    let lower_text = text.to_ascii_lowercase();
    let lower_needle = needle.to_ascii_lowercase();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    while i < text.len() {
        if lower_text[i..].starts_with(&lower_needle) {
            out.push_str(replacement);
            i += lower_needle.len();
        } else {
            let ch = text[i..].chars().next().unwrap();
            out.push(ch);
            i += ch.len_utf8();
        }
    }
    out
}

fn remap_str(s: &str, remap: &BTreeMap<String, String>) -> String {
    let mut out = s.to_string();
    for (from, to) in remap {
        out = out.replace(from.as_str(), to.as_str());
    }
    out
}

fn remap_args(args: &BTreeMap<String, String>, remap: &BTreeMap<String, String>) -> BTreeMap<String, String> {
    args.iter()
        .map(|(k, v)| (k.clone(), remap_str(v, remap)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_selftests_pass() {
        let results = Engine::run_selftests();
        assert_eq!(results.len(), 4);
        for r in &results {
            assert!(r.passed, "selftest failed: {} — {}", r.name, r.detail);
        }
    }
}
