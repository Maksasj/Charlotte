use super::Engine;
use crate::types::{Action, Condition, Episode, Pattern, PatternKind, PatternStatus, Scope};
use crate::unify::{is_var, normalize};
use std::collections::{BTreeMap, HashSet};

impl Engine {
    pub fn teach_reply(&mut self, input: &str, reply: &str, promote: bool) -> (Episode, Pattern) {
        let ep = Episode {
            id: self.alloc_id("e"),
            input: input.trim().to_string(),
            reply: reply.trim().to_string(),
            source: "taught".into(),
            generalized: false,
        };
        self.state.episodes.push(ep.clone());

        let pattern = Pattern {
            id: self.alloc_id("p"),
            name: format!("taught-{}", slug(input)),
            status: status_from_flag(promote),
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
        self.state.patterns.push(pattern.clone());

        (ep, pattern)
    }

    pub fn feedback(&mut self, turn: usize, positive: bool) -> bool {
        let Some(msg) = self
            .state
            .messages
            .iter_mut()
            .find(|m| m.role == "agent" && m.turn == turn)
        else {
            return false;
        };

        msg.feedback = Some(if positive { "up".into() } else { "down".into() });
        let fired = msg
            .trace
            .as_ref()
            .map(|t| t.fired_pattern_ids.clone())
            .unwrap_or_default();

        for pid in fired {
            if let Some(pat) = self.state.patterns.iter_mut().find(|p| p.id == pid) {
                if positive {
                    pat.trust = (pat.trust + 0.05).min(1.0);
                    pat.support += 1;
                } else {
                    pat.trust = (pat.trust - 0.15).max(0.1);
                }
            }
        }
        true
    }

    pub fn generalize_episodes(&mut self, auto_promote: bool) -> (Vec<Pattern>, String) {
        let candidates: Vec<usize> = self
            .state
            .episodes
            .iter()
            .enumerate()
            .filter(|(_, ep)| !ep.generalized)
            .map(|(idx, _)| idx)
            .collect();

        let mut used = HashSet::new();
        let mut created = Vec::new();

        for (pos, &base_idx) in candidates.iter().enumerate() {
            if used.contains(&base_idx) {
                continue;
            }
            let base_tokens = split_tokens(&self.state.episodes[base_idx].input);
            if base_tokens.len() < 2 {
                continue;
            }

            let remaining = &candidates[(pos + 1)..];
            let Some((diff_idx, group, reply_template)) = find_generalizable_group(
                &self.state.episodes,
                base_idx,
                &base_tokens,
                remaining,
                &used,
            ) else {
                continue;
            };

            let mut template_tokens = base_tokens;
            template_tokens[diff_idx] = "$x".into();
            let input_template = template_tokens.join(" ");

            let episode_ids: Vec<String> = group
                .iter()
                .map(|&idx| self.state.episodes[idx].id.clone())
                .collect();

            let pattern = Pattern {
                id: self.alloc_id("p"),
                name: format!("induced-{}", slug(&input_template)),
                status: status_from_flag(auto_promote),
                when: vec![Condition::Text {
                    template: input_template,
                }],
                then: vec![Action::Reply {
                    template: reply_template,
                }],
                trust: 0.88,
                support: group.len(),
                provenance: format!("anti_unify({})", episode_ids.join(",")),
                kind: PatternKind::Induced,
            };

            self.state.patterns.push(pattern.clone());
            created.push(pattern);

            for idx in group {
                used.insert(idx);
                self.state.episodes[idx].generalized = true;
            }
        }

        let summary = if created.is_empty() {
            "No new generalizable episode groups found.".into()
        } else {
            let names: Vec<&str> = created.iter().map(|p| p.name.as_str()).collect();
            format!(
                "Induced {} pattern(s) via anti-unification: {} (review & promote in Patterns).",
                created.len(),
                names.join(", ")
            )
        };
        (created, summary)
    }

    pub fn chunk_last_chain(&mut self, auto_promote: bool) -> (Option<Pattern>, String) {
        let Some(chain_idx) = self
            .state
            .chains
            .iter()
            .rposition(|c| !c.chunked && c.pattern_ids.len() >= 2)
        else {
            return (
                None,
                "No un-chunked multi-step chain available yet. Try asking for weather or a calculation first.".into(),
            );
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

        let folder = ChainFolder::fold(&chain_patterns);
        if folder.folded_ids.len() < 2 {
            return (
                None,
                "Chain cannot be folded further (tool is already in step 1).".into(),
            );
        }

        let pattern = Pattern {
            id: self.alloc_id("p"),
            name: format!("chunk-{}", folder.folded_ids.join("-")),
            status: status_from_flag(auto_promote),
            when: folder.when,
            then: folder.then,
            trust: 0.94,
            support: 1,
            provenance: format!("chunk({})", folder.folded_ids.join("->")),
            kind: PatternKind::Composite,
        };

        self.state.chains[chain_idx].chunked = true;
        self.state.patterns.push(pattern.clone());

        let summary = format!(
            "Chunked chain [{}] into composite pattern `{}` (review & promote in Patterns).",
            chain.pattern_ids.join(" -> "),
            pattern.name
        );
        (Some(pattern), summary)
    }
}

#[derive(Default)]
struct ChainFolder {
    produced_rels: HashSet<String>,
    produced_args: BTreeMap<(String, String), String>,
    var_remap: BTreeMap<String, String>,
    when: Vec<Condition>,
    then: Vec<Action>,
    folded_ids: Vec<String>,
    saw_tool: bool,
}

impl ChainFolder {
    fn fold(patterns: &[Pattern]) -> Self {
        let mut folder = Self::default();
        for pat in patterns {
            if folder.saw_tool {
                break;
            }
            folder.folded_ids.push(pat.id.clone());
            folder.fold_conditions(&pat.when);
            folder.fold_actions(&pat.then);
        }
        folder
    }

    fn fold_conditions(&mut self, conditions: &[Condition]) {
        for cond in conditions {
            match cond {
                Condition::Fact { rel, args } if self.produced_rels.contains(rel) => {
                    for (key, val) in args {
                        if is_var(val)
                            && let Some(origin) =
                                self.produced_args.get(&(rel.clone(), key.clone()))
                        {
                            self.var_remap.insert(val.clone(), origin.clone());
                        }
                    }
                }
                Condition::NotFact { rel, .. } if self.produced_rels.contains(rel) => {}
                Condition::Text { template } => self.when.push(Condition::Text {
                    template: remap_str(template, &self.var_remap),
                }),
                Condition::Contains { phrase } => self.when.push(Condition::Contains {
                    phrase: phrase.clone(),
                }),
                Condition::Fact { rel, args } => self.when.push(Condition::Fact {
                    rel: rel.clone(),
                    args: remap_args(args, &self.var_remap),
                }),
                Condition::NotFact { rel, args } => self.when.push(Condition::NotFact {
                    rel: rel.clone(),
                    args: remap_args(args, &self.var_remap),
                }),
            }
        }
    }

    fn fold_actions(&mut self, actions: &[Action]) {
        for act in actions {
            match act {
                Action::Assert { rel, args, scope } if *scope == Scope::Working => {
                    self.produced_rels.insert(rel.clone());
                    for (key, val) in args {
                        let mapped = remap_str(val, &self.var_remap);
                        self.produced_args
                            .insert((rel.clone(), key.clone()), mapped);
                    }
                }
                Action::Assert { rel, args, scope } => self.then.push(Action::Assert {
                    rel: rel.clone(),
                    args: remap_args(args, &self.var_remap),
                    scope: scope.clone(),
                }),
                Action::Retract { rel, args } => self.then.push(Action::Retract {
                    rel: rel.clone(),
                    args: args.as_ref().map(|a| remap_args(a, &self.var_remap)),
                }),
                Action::Tool { name, args } => {
                    self.saw_tool = true;
                    self.then.push(Action::Tool {
                        name: name.clone(),
                        args: remap_args(args, &self.var_remap),
                    });
                }
                Action::Reply { template } => self.then.push(Action::Reply {
                    template: remap_str(template, &self.var_remap),
                }),
            }
        }
    }
}

fn find_generalizable_group(
    episodes: &[Episode],
    base_idx: usize,
    base_tokens: &[String],
    remaining: &[usize],
    used: &HashSet<usize>,
) -> Option<(usize, Vec<usize>, String)> {
    for diff_idx in 0..base_tokens.len() {
        let mut group = vec![base_idx];

        for &peer_idx in remaining {
            if used.contains(&peer_idx) {
                continue;
            }
            let peer_tokens = split_tokens(&episodes[peer_idx].input);
            if matches_skeleton(base_tokens, &peer_tokens, diff_idx) {
                group.push(peer_idx);
            }
        }

        if group.len() < 2 {
            continue;
        }

        if let Some(reply_template) = coherent_reply_template(episodes, &group, diff_idx) {
            return Some((diff_idx, group, reply_template));
        }
    }
    None
}

fn matches_skeleton(base: &[String], other: &[String], diff_idx: usize) -> bool {
    if base.len() != other.len() {
        return false;
    }
    (0..base.len()).all(|idx| {
        if idx == diff_idx {
            normalize(&other[idx]) != normalize(&base[idx])
        } else {
            normalize(&other[idx]) == normalize(&base[idx])
        }
    })
}

fn coherent_reply_template(
    episodes: &[Episode],
    group: &[usize],
    diff_idx: usize,
) -> Option<String> {
    let mut normalized_templates = HashSet::new();
    let mut first_template = String::new();

    for &idx in group {
        let ep = &episodes[idx];
        let tokens: Vec<&str> = ep.input.split_whitespace().collect();
        let var_val = tokens[diff_idx].trim_matches(|c: char| matches!(c, '?' | '!' | '.' | ','));
        let replaced = replace_word_case_insensitive(&ep.reply, var_val, "$x");

        if first_template.is_empty() {
            first_template = replaced.clone();
        }
        normalized_templates.insert(normalize(&replaced));
        if normalized_templates.len() > 1 {
            return None;
        }
    }

    (!first_template.is_empty()).then_some(first_template)
}

fn split_tokens(input: &str) -> Vec<String> {
    input.split_whitespace().map(str::to_string).collect()
}

fn status_from_flag(active: bool) -> PatternStatus {
    if active {
        PatternStatus::Active
    } else {
        PatternStatus::Candidate
    }
}

fn slug(raw: &str) -> String {
    let mut out = String::new();
    for ch in raw.chars() {
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
    let mut idx = 0;

    while idx < text.len() {
        if lower_text[idx..].starts_with(&lower_needle) {
            out.push_str(replacement);
            idx += lower_needle.len();
        } else {
            let ch = text[idx..].chars().next().unwrap();
            out.push(ch);
            idx += ch.len_utf8();
        }
    }
    out
}

fn remap_str(raw: &str, remap: &BTreeMap<String, String>) -> String {
    let mut out = raw.to_string();
    for (from, to) in remap {
        out = out.replace(from.as_str(), to.as_str());
    }
    out
}

fn remap_args(
    args: &BTreeMap<String, String>,
    remap: &BTreeMap<String, String>,
) -> BTreeMap<String, String> {
    args.iter()
        .map(|(k, v)| (k.clone(), remap_str(v, remap)))
        .collect()
}
