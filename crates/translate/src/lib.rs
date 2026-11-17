//! Understanding input (Design 2.0, milestone 14).
//!
//! No message is translated as one all-or-nothing block any more:
//!
//! 1. the [`segment`] stage splits it into sentences and list items, with no model;
//! 2. each part is tried by the fixed patterns (the v1 pattern parser), then
//!    by learned patterns (milestone 17), then by the model;
//! 3. one model call sees the whole message and the conversation, and answers
//!    sentence by sentence (`[1] greet:`, `[2] request: ...`), so one bad
//!    sentence does not hurt the others;
//! 4. every part is checked by the [`validate::Validator`];
//! 5. failed parts get one retry with the validator's problems;
//! 6. a part that still fails is cut at clause words and its clauses are
//!    tried again (depth at most 2);
//! 7. what is left is reported as not understood, so the agent can answer
//!    the rest and ask about it.
//!
//! The model never decides what is true: hedges and questions pass through
//! as they are, and every constant must be grounded in the message, the
//! conversation or the registry.

pub mod segment;
pub mod validate;

use serde::{Deserialize, Serialize};
use sla_lexicon::Registry;
use sla_parser::llm::{from_canonical, numbered, suspicious};
use sla_parser::{Context, Intent};
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use validate::Validator;

/// The conversation the translator needs (Design 2.0: "Session context").
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TContext {
    pub speaker: String,
    pub active_task: Option<String>,
    /// The number the next new step of the active task gets.
    pub next_step: usize,
    pub last_reply: Option<String>,
    /// Things recently mentioned, most recent first.
    pub recent: Vec<String>,
    /// Tasks the agent knows (procedures and task schemas).
    pub known_tasks: Vec<String>,
}

impl TContext {
    fn key(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}",
            self.speaker,
            self.active_task.as_deref().unwrap_or(""),
            self.next_step,
            self.recent.iter().take(3).cloned().collect::<Vec<_>>().join(","),
            self.last_reply.as_deref().unwrap_or("").chars().take(80).collect::<String>()
        )
    }
}

/// Which tier produced a part's acts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Tier {
    Pattern,
    Learned,
    Model,
    /// A clause of a sentence that failed whole.
    Clause,
}

impl Tier {
    pub fn name(self) -> &'static str {
        match self {
            Tier::Pattern => "pattern",
            Tier::Learned => "learned",
            Tier::Model => "model",
            Tier::Clause => "clause",
        }
    }
}

/// One part of a message and what it became.
#[derive(Debug, Clone, PartialEq)]
pub struct Part {
    pub text: String,
    pub acts: Vec<Intent>,
    pub tier: Tier,
    /// Passed the validator (or is honestly `unknown`).
    pub ok: bool,
    /// The validator's problems with the last attempt.
    pub problems: Vec<String>,
    pub retried: bool,
}

impl Part {
    /// Understood: valid, and not only `unknown`.
    pub fn understood(&self) -> bool {
        self.ok && !self.acts.iter().all(|a| *a == Intent::Unknown)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Translation {
    pub parts: Vec<Part>,
    pub model_calls: u64,
}

/// The model tier: the Claude CLI, with the language spec in the system prompt.
#[derive(Debug, Clone)]
pub struct ModelTier {
    pub model: String,
    /// Reviewed translations to pick examples from: (sentence, lines).
    pub examples: Vec<(String, Vec<String>)>,
}

pub const SPEC: &str = include_str!("../../../docs/language_2.md");
pub const SPEC_V1: &str = include_str!("../../../docs/parser_spec.md");

fn system_prompt() -> String {
    format!(
        "You translate chat messages into a formal language for a symbolic agent, sentence by sentence. \
         You never answer, chat, explain or add knowledge: every constant you write must come from the message or the context. \
         Output only numbered lines `[n] act: payload`, one or more per sentence, nothing else.\n\n\
         The language, version 2:\n\n{SPEC}\n\nThe statements and questions it extends (version 1):\n\n{SPEC_V1}"
    )
}

/// Words shared between two texts (for choosing examples).
fn overlap(a: &str, b: &str) -> usize {
    let wa: std::collections::BTreeSet<String> = a.to_lowercase().split(|c: char| !c.is_alphanumeric()).filter(|w| w.len() > 2).map(String::from).collect();
    b.to_lowercase().split(|c: char| !c.is_alphanumeric()).filter(|w| w.len() > 2 && wa.contains(*w)).count()
}

impl ModelTier {
    pub fn new(model: &str) -> Self {
        ModelTier { model: model.into(), examples: Vec::new() }
    }

    fn prompt(&self, sentences: &[String], ask: &[usize], ctx: &TContext, retry: &[(usize, Vec<String>, Vec<String>)]) -> String {
        let mut p = String::new();
        p.push_str(&format!("Context: the speaker is `{}`.", ctx.speaker));
        if let Some(t) = &ctx.active_task {
            p.push_str(&format!(" Active task: `{t}` (its next new step is number {}).", ctx.next_step.max(1)));
        }
        if !ctx.recent.is_empty() {
            p.push_str(&format!(" Recently mentioned: {}.", ctx.recent.iter().take(5).map(|r| format!("`{r}`")).collect::<Vec<_>>().join(", ")));
        }
        if let Some(r) = &ctx.last_reply {
            p.push_str(&format!(" The agent last said: \"{}\"", r.chars().take(300).collect::<String>()));
        }
        if !ctx.known_tasks.is_empty() {
            p.push_str(&format!(" Tasks the agent knows: {}.", ctx.known_tasks.iter().take(20).map(|r| format!("`{r}`")).collect::<Vec<_>>().join(", ")));
        }
        // Examples by word overlap with the message.
        let all = sentences.join(" ");
        let mut ex: Vec<&(String, Vec<String>)> = self.examples.iter().filter(|(s, _)| overlap(s, &all) > 0).collect();
        ex.sort_by_key(|(s, _)| std::cmp::Reverse(overlap(s, &all)));
        if !ex.is_empty() {
            p.push_str("\n\nTranslations of similar messages:\n");
            for (s, lines) in ex.into_iter().take(6) {
                p.push_str(&format!("Message: {s}\n{}\n", lines.join("\n")));
            }
        }
        p.push_str("\n\nMessage, numbered by sentence:\n");
        for (i, s) in sentences.iter().enumerate() {
            p.push_str(&format!("[{}] {s}\n", i + 1));
        }
        if !retry.is_empty() {
            p.push_str("\nYour earlier translation of some sentences broke the rules; translate them again:\n");
            for (n, lines, problems) in retry {
                p.push_str(&format!("[{n}] was: {} — problems: {}\n", if lines.is_empty() { "(nothing)".into() } else { lines.join(" / ") }, problems.join("; ")));
            }
        }
        let ask_list: Vec<String> = ask.iter().map(|n| format!("[{n}]")).collect();
        p.push_str(&format!("\nTranslate sentences {}. Output only their numbered lines.", ask_list.join(", ")));
        p
    }

    /// Ask the model; returns lines by sentence number.
    fn call(&self, sentences: &[String], ask: &[usize], ctx: &TContext, retry: &[(usize, Vec<String>, Vec<String>)]) -> Result<HashMap<usize, Vec<String>>, String> {
        let out = sla_parser::claude::call_system("translator", &self.model, &system_prompt(), &self.prompt(sentences, ask, ctx, retry))?;
        let mut by: HashMap<usize, Vec<String>> = HashMap::new();
        for line in out.lines() {
            let l = line.trim().trim_matches('`').trim();
            if let Some((n, rest)) = numbered(l) {
                by.entry(n).or_default().push(rest.to_string());
            }
        }
        Ok(by)
    }
}

#[derive(Serialize, Deserialize)]
struct CacheLine {
    key: String,
    lines: Vec<(usize, Vec<String>)>,
}

/// Serializes appends to the cache file across translators in one process.
static CACHE_FILE: std::sync::Mutex<()> = std::sync::Mutex::new(());

pub struct Translator {
    pub registry: std::sync::Arc<Registry>,
    pub model: Option<ModelTier>,
    /// Longest sentence the patterns may take when a model is there (longer ones go to the model).
    pub pattern_words: usize,
    cache: HashMap<String, HashMap<usize, Vec<String>>>,
    cache_path: Option<PathBuf>,
    pub calls: u64,
}

fn cache_key(sentences: &[String], ask: &[usize], ctx: &TContext) -> String {
    let mut h: u64 = 0xcbf29ce484222325;
    for part in sentences.iter().map(String::as_str).chain(std::iter::once(ctx.key().as_str())).chain(ask.iter().map(|_| "|")) {
        for b in part.as_bytes().iter().chain(std::iter::once(&0u8)) {
            h ^= *b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
    }
    format!("{h:016x}:{}", ask.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(","))
}

/// The subject a sentence talked about, for "he", "she", "it" in the next one.
fn subject_of(intents: &[Intent], speaker: &str) -> Option<String> {
    for i in intents {
        let atom = match i {
            Intent::Tell { facts, .. } | Intent::Inform(facts) | Intent::Correct(facts) => facts.first(),
            Intent::Ask { query, .. } => query.iter().find_map(|l| if let sla_core::Literal::Pos(a) = l { Some(a) } else { None }),
            _ => None,
        };
        if let Some(a) = atom {
            if let Some(sla_core::Term::Val(sla_core::Value::Sym(s))) = a.args.first() {
                if s != speaker {
                    return Some(s.clone());
                }
            }
        }
    }
    None
}

impl Translator {
    pub fn new(registry: Registry) -> Self {
        Translator { registry: std::sync::Arc::new(registry), model: None, pattern_words: 12, cache: HashMap::new(), cache_path: None, calls: 0 }
    }

    /// Keep model results in a file (one JSON line per call).
    pub fn with_cache(mut self, path: PathBuf) -> Self {
        if let Ok(src) = std::fs::read_to_string(&path) {
            for l in src.lines() {
                if let Ok(c) = serde_json::from_str::<CacheLine>(l) {
                    self.cache.insert(c.key, c.lines.into_iter().collect());
                }
            }
        }
        self.cache_path = Some(path);
        self
    }

    fn remember(&mut self, key: String, lines: HashMap<usize, Vec<String>>, persist: bool) {
        if persist {
            if let Some(p) = &self.cache_path {
                let _g = CACHE_FILE.lock().unwrap_or_else(|e| e.into_inner());
                let mut v: Vec<(usize, Vec<String>)> = lines.iter().map(|(k, v)| (*k, v.clone())).collect();
                v.sort();
                if let Ok(mut line) = serde_json::to_string(&CacheLine { key: key.clone(), lines: v }) {
                    line.push('\n');
                    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(p) {
                        let _ = f.write_all(line.as_bytes());
                    }
                }
            }
        }
        self.cache.insert(key, lines);
    }

    /// The model's lines for some sentences, from the cache or one call.
    /// A failed call is remembered for this session (never retried under a lock).
    fn model_lines(&mut self, sentences: &[String], ask: &[usize], ctx: &TContext, retry: &[(usize, Vec<String>, Vec<String>)]) -> HashMap<usize, Vec<String>> {
        let Some(tier) = self.model.clone() else { return HashMap::new() };
        let mut key = cache_key(sentences, ask, ctx);
        if !retry.is_empty() {
            key.push_str(":retry");
        }
        if let Some(l) = self.cache.get(&key) {
            return l.clone();
        }
        self.calls += 1;
        match tier.call(sentences, ask, ctx, retry) {
            Ok(l) => {
                self.remember(key, l.clone(), !l.is_empty());
                l
            }
            Err(_) => {
                self.remember(key, HashMap::new(), false);
                HashMap::new()
            }
        }
    }

    fn read(lines: &[String]) -> Option<Vec<Intent>> {
        let acts: Option<Vec<Intent>> = lines.iter().map(|l| from_canonical(l)).collect();
        acts.filter(|a| !a.is_empty())
    }

    /// The fixed-pattern tier for one sentence.
    fn pattern(&self, text: &str, ctx: &TContext, last_entity: &Option<String>) -> Option<Vec<Intent>> {
        let pctx = Context { speaker: ctx.speaker.clone(), last_entity: last_entity.clone() };
        let mut i = sla_parser::parse(text, &pctx);
        if i == Intent::Unknown || suspicious(&i) {
            return None;
        }
        self.name_properties(&mut i);
        let v = Validator::new(&self.registry);
        let acts = vec![i];
        // What a pronoun resolved to is part of the conversation, so it is grounded.
        let mut ctx2 = ctx.clone();
        if let Some(e) = last_entity {
            if !ctx2.recent.contains(e) {
                ctx2.recent.insert(0, e.clone());
            }
        }
        if v.check(text, &acts, &ctx2).is_empty() {
            Some(acts)
        } else {
            None
        }
    }

    /// v1 patterns write an adjective as `property(x, quality, adj)`; 2.0
    /// names the property from the registry's adjective list (lazy ->
    /// character). An unknown adjective stays as it is and fails validation.
    fn name_properties(&self, i: &mut Intent) {
        let fix = |a: &mut sla_core::Atom| {
            if a.pred.trim_start_matches('-') == "property" && a.args.len() == 3 {
                if let (sla_core::Term::Val(sla_core::Value::Sym(q)), sla_core::Term::Val(sla_core::Value::Sym(v))) = (&a.args[1], &a.args[2]) {
                    if q == "quality" {
                        if let Some(p) = self.registry.adjectives.get(v) {
                            a.args[1] = sla_core::Term::sym(p);
                        }
                    }
                }
            }
        };
        match i {
            Intent::Tell { facts, rules, .. } => {
                facts.iter_mut().for_each(fix);
                for r in rules {
                    fix(&mut r.head);
                    for l in &mut r.body {
                        if let sla_core::Literal::Pos(a) | sla_core::Literal::Neg(a) = l {
                            fix(a);
                        }
                    }
                }
            }
            Intent::Correct(facts) | Intent::Inform(facts) => facts.iter_mut().for_each(fix),
            Intent::Ask { query, .. } => {
                for l in query {
                    if let sla_core::Literal::Pos(a) | sla_core::Literal::Neg(a) = l {
                        fix(a);
                    }
                }
            }
            Intent::Why(Some(a)) => fix(a),
            _ => {}
        }
    }

    /// Translate a message (see the module docs for the stages).
    pub fn translate(&mut self, message: &str, ctx: &TContext) -> Translation {
        let calls_before = self.calls;
        let sentences = segment::segment(message);
        let mut parts: Vec<Option<Part>> = vec![None; sentences.len()];
        // Tier 1: fixed patterns. With a model, only short sentences; without, all.
        let mut last_entity = ctx.recent.first().cloned();
        for (i, s) in sentences.iter().enumerate() {
            let short = s.split_whitespace().count() <= self.pattern_words || self.model.is_none();
            if short {
                if let Some(acts) = self.pattern(s, ctx, &last_entity) {
                    if let Some(e) = subject_of(&acts, &ctx.speaker) {
                        last_entity = Some(e);
                    }
                    parts[i] = Some(Part { text: s.clone(), acts, tier: Tier::Pattern, ok: true, problems: Vec::new(), retried: false });
                }
            }
        }
        // Tier 2: learned patterns come with milestone 17.
        // Tier 3: one model call for every sentence still open, with the whole message as context.
        let reg = std::sync::Arc::clone(&self.registry);
        let v = Validator::new(&reg);
        let open: Vec<usize> = (0..sentences.len()).filter(|i| parts[*i].is_none()).map(|i| i + 1).collect();
        if !open.is_empty() && self.model.is_some() {
            let lines = self.model_lines(&sentences, &open, ctx, &[]);
            let mut retry: Vec<(usize, Vec<String>, Vec<String>)> = Vec::new();
            for &n in &open {
                let got = lines.get(&n).cloned().unwrap_or_default();
                let s = &sentences[n - 1];
                match Self::read(&got) {
                    Some(acts) => {
                        let problems = v.check(s, &acts, ctx);
                        let ok = problems.is_empty();
                        if !ok {
                            retry.push((n, got.clone(), problems.clone()));
                        }
                        parts[n - 1] = Some(Part { text: s.clone(), acts, tier: Tier::Model, ok, problems, retried: false });
                    }
                    None => {
                        let problems = vec![if got.is_empty() { "no translation".to_string() } else { format!("unreadable lines: {}", got.join(" / ")) }];
                        retry.push((n, got, problems.clone()));
                        parts[n - 1] = Some(Part { text: s.clone(), acts: Vec::new(), tier: Tier::Model, ok: false, problems, retried: false });
                    }
                }
            }
            // Step 5: one retry for the failed parts, with their problems
            // (not when the call itself gave nothing: that failure is remembered).
            if !retry.is_empty() && !lines.is_empty() {
                let ask: Vec<usize> = retry.iter().map(|r| r.0).collect();
                let lines = self.model_lines(&sentences, &ask, ctx, &retry);
                for &n in &ask {
                    let got = lines.get(&n).cloned().unwrap_or_default();
                    let s = &sentences[n - 1];
                    if let Some(acts) = Self::read(&got) {
                        let problems = v.check(s, &acts, ctx);
                        let ok = problems.is_empty();
                        parts[n - 1] = Some(Part { text: s.clone(), acts, tier: Tier::Model, ok, problems, retried: true });
                    } else if let Some(p) = parts[n - 1].as_mut() {
                        p.retried = true;
                    }
                }
            }
        }
        // Step 6: split what still fails at clause words and try the clauses,
        // carrying "who we are talking about" from part to part.
        let mut out: Vec<Part> = Vec::new();
        let mut running = ctx.recent.first().cloned();
        for (i, p) in parts.into_iter().enumerate() {
            let s = &sentences[i];
            let p = p.unwrap_or_else(|| Part { text: s.clone(), acts: vec![Intent::Unknown], tier: Tier::Pattern, ok: false, problems: vec!["not understood".into()], retried: false });
            if p.ok {
                if let Some(e) = subject_of(&p.acts, &ctx.speaker) {
                    running = Some(e);
                }
                out.push(p);
                continue;
            }
            let cl = segment::clauses(s);
            if cl.len() < 2 {
                out.push(p);
                continue;
            }
            let mut sub = self.clause_parts(&cl, ctx, 1, &mut running);
            if sub.iter().any(|c| c.understood()) {
                out.append(&mut sub);
            } else {
                out.push(p);
            }
        }
        Translation { parts: out, model_calls: self.calls - calls_before }
    }

    /// Clauses of a failed sentence: patterns first, then one model call; depth at most 2.
    fn clause_parts(&mut self, clauses: &[String], ctx: &TContext, depth: u8, running: &mut Option<String>) -> Vec<Part> {
        let mut parts: Vec<Option<Part>> = vec![None; clauses.len()];
        for (i, c) in clauses.iter().enumerate() {
            if let Some(acts) = self.pattern(segment::without_link(c), ctx, running) {
                if let Some(e) = subject_of(&acts, &ctx.speaker) {
                    *running = Some(e);
                }
                parts[i] = Some(Part { text: c.clone(), acts, tier: Tier::Clause, ok: true, problems: Vec::new(), retried: false });
            }
        }
        let open: Vec<usize> = (0..clauses.len()).filter(|i| parts[*i].is_none()).map(|i| i + 1).collect();
        if !open.is_empty() && self.model.is_some() {
            let lines = self.model_lines(clauses, &open, ctx, &[]);
            let reg = std::sync::Arc::clone(&self.registry);
            let v = Validator::new(&reg);
            for &n in &open {
                let got = lines.get(&n).cloned().unwrap_or_default();
                if let Some(acts) = Self::read(&got) {
                    let problems = v.check(&clauses[n - 1], &acts, ctx);
                    let ok = problems.is_empty();
                    parts[n - 1] = Some(Part { text: clauses[n - 1].clone(), acts, tier: Tier::Clause, ok, problems, retried: false });
                }
            }
        }
        let mut out = Vec::new();
        for (i, p) in parts.into_iter().enumerate() {
            match p {
                Some(p) if p.ok => out.push(p),
                other => {
                    let sub = segment::clauses(&clauses[i]);
                    if depth < 2 && sub.len() >= 2 {
                        out.extend(self.clause_parts(&sub, ctx, depth + 1, running));
                    } else {
                        out.push(other.unwrap_or_else(|| Part { text: clauses[i].clone(), acts: vec![Intent::Unknown], tier: Tier::Clause, ok: false, problems: vec!["not understood".into()], retried: false }));
                    }
                }
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn ctx() -> TContext {
        TContext { speaker: "maksim".into(), ..Default::default() }
    }

    #[test]
    fn patterns_without_a_model() {
        let mut t = Translator::new(Registry::builtin());
        let tr = t.translate("Hi! Anna likes tea. Blorf zang quux.", &ctx());
        assert_eq!(tr.parts.len(), 3);
        assert_eq!(tr.parts[0].acts, vec![Intent::Greet]);
        assert_eq!(tr.parts[1].acts[0].canonical(), "tell: likes(anna, tea)");
        assert!(!tr.parts[2].understood(), "one broken sentence does not lose the others");
        assert_eq!(tr.model_calls, 0);
    }

    #[test]
    fn one_call_per_message_and_a_retry() {
        // The mock translates sentence 2 wrongly first (an ungrounded constant), then right.
        sla_parser::claude::set_thread_mock(Arc::new(|role, prompt| {
            assert_eq!(role, "translator");
            if prompt.contains("broke the rules") {
                assert!(prompt.contains("basil"), "the retry names the problem");
                return Ok("[2] inform: available(pasta); available(garlic)".into());
            }
            assert!(prompt.contains("[1] Hey, I need some help with dinner tonight, honestly no idea what to cook."));
            Ok("[1] request: plan_dinner\n[2] inform: available(pasta); available(garlic); available(basil)\n[3] unknown:".into())
        }));
        let mut t = Translator::new(Registry::builtin());
        t.model = Some(ModelTier::new("haiku"));
        let msg = "Hey, I need some help with dinner tonight, honestly no idea what to cook. I have pasta and garlic in the cupboard right now somewhere. Whatever, the weather is weird again today honestly.";
        let tr = t.translate(msg, &ctx());
        sla_parser::claude::clear_thread_mock();
        assert_eq!(tr.model_calls, 2, "one call for the message, one retry");
        assert_eq!(tr.parts[0].acts[0].canonical(), "request: plan_dinner");
        assert!(tr.parts[1].ok && tr.parts[1].retried);
        assert_eq!(tr.parts[1].acts[0].canonical(), "inform: available(pasta); available(garlic)");
        assert!(tr.parts[2].ok && !tr.parts[2].understood(), "an honest unknown");
        // The same message again comes from the cache.
        let mut t2 = t;
        sla_parser::claude::set_thread_mock(Arc::new(|_, _| panic!("cached")));
        let tr2 = t2.translate(msg, &ctx());
        sla_parser::claude::clear_thread_mock();
        assert_eq!(tr2.model_calls, 0);
    }

    #[test]
    fn a_failed_call_is_not_repeated() {
        sla_parser::claude::set_thread_mock(Arc::new(|_, _| Err("CLI down".into())));
        let mut t = Translator::new(Registry::builtin());
        t.model = Some(ModelTier::new("haiku"));
        let msg = "Could you please walk me through repairing the puncture on my old bicycle wheel?";
        let a = t.translate(msg, &ctx());
        let b = t.translate(msg, &ctx());
        sla_parser::claude::clear_thread_mock();
        assert!(!a.parts[0].understood() && !b.parts[0].understood());
        assert_eq!(t.calls, 1, "the failure is remembered for the session");
    }
}
