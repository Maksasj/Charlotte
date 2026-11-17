//! LLM translator: a pure translator from free text into the formal intent
//! language, behind the [`Parser`] trait. The pattern parser runs first;
//! only messages it does not understand go to the model (Claude CLI, headless).
//! The model's output must be valid canonical lines (`docs/parser_spec.md`),
//! which are parsed back and checked; anything else is rejected. The model
//! never answers, reasons or writes replies.

use crate::{parse, Context, Intent, Parser, Social};
use sla_core::{parse_atom, parse_query, parse_rule, Atom, Rule, Term, Value};
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;

const SPEC: &str = include_str!("../../../docs/parser_spec.md");

/// Serializes appends to translation cache files across agents in one process.
static CACHE_FILE: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// Atoms and rules separated by `; ` (facts must be ground).
fn facts_and_rules(body: &str) -> Option<(Vec<Atom>, Vec<Rule>)> {
    let mut facts = Vec::new();
    let mut rules: Vec<Rule> = Vec::new();
    for item in body.split("; ").map(str::trim).filter(|x| !x.is_empty()) {
        if item.contains(":-") {
            let text = if item.ends_with('.') { item.to_string() } else { format!("{item}.") };
            rules.push(parse_rule(&text).ok()?);
        } else {
            facts.push(parse_atom(item.trim_end_matches('.').trim()).ok().filter(|a| a.is_ground())?);
        }
    }
    if facts.is_empty() && rules.is_empty() {
        return None;
    }
    Some((facts, rules))
}

/// Split a numbered line `[3] request: plan_dinner` into (3, "request: plan_dinner").
pub fn numbered(line: &str) -> Option<(usize, &str)> {
    let rest = line.trim().strip_prefix('[')?;
    let (n, rest) = rest.split_once(']')?;
    Some((n.trim().parse().ok()?, rest.trim()))
}

/// Parse one canonical line (`tell: likes(anna, tea)`) back into an intent.
pub fn from_canonical(line: &str) -> Option<Intent> {
    let (kind, payload) = line.split_once(':')?;
    let payload = payload.trim();
    let sym = |s: &str| -> Option<String> {
        let s = s.trim();
        let s = s.strip_prefix('"').and_then(|x| x.strip_suffix('"')).unwrap_or(s);
        if s.is_empty() || s.contains(['(', ')', ',', ' ']) {
            None
        } else {
            Some(s.to_string())
        }
    };
    let ground = |s: &str| -> Option<Atom> { parse_atom(s.trim()).ok().filter(|a| a.is_ground()) };
    let _ = &ground;
    match kind.trim() {
        "tell" | "teach" => {
            // Trailing annotations: @past and a hedge (@usually, @often, @sometimes).
            let mut body = payload;
            let mut past = false;
            let mut hedge = None;
            loop {
                let Some((rest, tag)) = body.rsplit_once(" @").or_else(|| body.strip_prefix('@').map(|t| ("", t))) else { break };
                let tag = tag.trim();
                match tag {
                    "past" => past = true,
                    "usually" | "often" | "sometimes" => hedge = Some(tag.to_string()),
                    _ => return None,
                }
                body = rest.trim();
            }
            let (facts, rules) = facts_and_rules(body)?;
            if kind.trim() == "teach" {
                if past || hedge.is_some() {
                    return None;
                }
                return Some(Intent::Teach { facts, rules });
            }
            Some(Intent::Tell { facts, rules, past, hedge })
        }
        "inform" => {
            let (facts, rules) = facts_and_rules(payload)?;
            if !rules.is_empty() {
                return None;
            }
            Some(Intent::Inform(facts))
        }
        "request" => Some(Intent::Request(sym(payload)?)),
        "ask_how" => Some(Intent::AskHow(sym(payload)?)),
        "ask_need" => Some(Intent::AskNeed(sym(payload)?)),
        "ask_duration" => Some(Intent::AskDuration(sym(payload)?)),
        "ask_next" if payload.is_empty() => Some(Intent::AskNext),
        "done" if payload == "last" || payload.is_empty() => Some(Intent::Done(None)),
        "done" => Some(Intent::Done(Some(sym(payload)?))),
        "lack" => payload.split("; ").map(sym).collect::<Option<Vec<_>>>().filter(|v| !v.is_empty()).map(Intent::Lack),
        "accept" if payload.is_empty() => Some(Intent::Accept),
        "reject" if payload.is_empty() => Some(Intent::Reject),
        "feedback" => match payload {
            "positive" => Some(Intent::Feedback(true)),
            "negative" => Some(Intent::Feedback(false)),
            _ => None,
        },
        "new_pred" => {
            let (atom, def) = payload.split_once('|')?;
            let atom = parse_atom(atom.trim()).ok().filter(|a| a.is_ground() && !a.args.is_empty())?;
            let definition = def.trim().to_string();
            if definition.is_empty() {
                return None;
            }
            Some(Intent::NewPred { atom, definition })
        }
        "ask" => {
            let (body, history) = match payload.strip_suffix("@history") {
                Some(b) => (b.trim(), true),
                None => (payload, false),
            };
            let query = parse_query(body).ok()?;
            Some(Intent::Ask { query, history })
        }
        "recall" => Some(Intent::Recall(sym(payload)?)),
        "recall_more" if payload == "last" => Some(Intent::RecallMore(None)),
        "recall_more" => Some(Intent::RecallMore(Some(sym(payload)?))),
        "why" if payload == "last" || payload.is_empty() => Some(Intent::Why(None)),
        "why" => Some(Intent::Why(Some(ground(payload)?))),
        "correct" if payload == "none" || payload.is_empty() => Some(Intent::Correct(Vec::new())),
        "correct" => payload.split("; ").map(ground).collect::<Option<Vec<_>>>().map(Intent::Correct),
        "about_you" if payload == "none" || payload.is_empty() => Some(Intent::AboutYou(None)),
        "about_you" => Some(Intent::AboutYou(Some(sym(payload)?))),
        "yes" => Some(Intent::Yes),
        "no" => Some(Intent::No),
        "greet" => Some(Intent::Greet),
        "bye" => Some(Intent::Bye),
        "how_are_you" => Some(Intent::HowAreYou),
        "unknown" => Some(Intent::Unknown),
        "social" => Some(Intent::Social(match payload {
            "thanks" => Social::Thanks,
            "reaction" => Social::Reaction,
            "who_are_you" => Social::WhoAreYou,
            "what_can_you_do" => Social::WhatCanYouDo,
            "and_you" => Social::AndYou,
            "dont_know" => Social::DontKnow,
            _ => return None,
        })),
        _ => None,
    }
}

/// Only accept atoms the agent can store: lowercase predicate names, no stray variables.
fn acceptable(i: &Intent) -> bool {
    let ok_pred = |p: &str| {
        let p = p.trim_start_matches('-');
        !p.is_empty() && p.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') && p.len() <= 40
    };
    match i {
        Intent::Tell { facts, rules, .. } | Intent::Teach { facts, rules } => {
            facts.iter().all(|a| ok_pred(&a.pred) && !a.args.is_empty())
                && rules.iter().all(|r| ok_pred(&r.head.pred) && r.head.args.iter().any(|t| matches!(t, Term::Var(_))))
        }
        Intent::Ask { query, .. } => query.iter().all(|l| match l {
            sla_core::Literal::Pos(a) | sla_core::Literal::Neg(a) => ok_pred(&a.pred) && a.args.iter().all(|t| !matches!(t, Term::Var(v) if v != "W" && v != "_")),
            sla_core::Literal::Cmp(..) => true,
            sla_core::Literal::Arith(..) | sla_core::Literal::Agg { .. } => false,
        }),
        Intent::Correct(v) => v.iter().all(|a| ok_pred(&a.pred)),
        _ => true,
    }
}

/// Words that never belong inside an entity name: a pattern parse that put
/// them there has swallowed part of another clause ("cat_named_mia",
/// "she_enjoys_fresh_bread").
const GLUE: &[&str] = &[
    "she", "he", "it", "they", "i", "you", "we", "named", "called", "and", "or", "but", "so", "because", "who", "which", "that", "also",
    "with", "when", "where", "if", "then", "is", "are", "was", "has", "have", "does", "do", "not", "to",
];

/// Does a pattern parse look garbled?
pub fn suspicious(i: &Intent) -> bool {
    let bad_term = |t: &Term| match t {
        Term::Val(Value::Sym(s)) => {
            let parts: Vec<&str> = s.split('_').collect();
            parts.len() >= 4 || parts.iter().any(|p| GLUE.contains(p))
        }
        _ => false,
    };
    let bad_atom = |a: &Atom| a.args.iter().any(bad_term) || a.pred.split('_').count() >= 3;
    match i {
        Intent::Tell { facts, rules, .. } => facts.iter().any(bad_atom) || rules.iter().any(|r| bad_atom(&r.head)),
        Intent::Ask { query, .. } => query.iter().any(|l| matches!(l, sla_core::Literal::Pos(a) | sla_core::Literal::Neg(a) if bad_atom(a))),
        Intent::Correct(v) => v.iter().any(bad_atom),
        Intent::Recall(e) | Intent::RecallMore(Some(e)) => bad_term(&Term::Val(Value::sym(e.as_str()))),
        _ => false,
    }
}

pub struct LlmTranslator {
    pub model: String,
    /// text -> canonical lines, persisted so the same phrasing is translated once.
    cache: HashMap<String, Vec<String>>,
    cache_path: Option<PathBuf>,
    /// Model calls made (cache misses).
    pub calls: usize,
    /// Last translation: (text, canonical lines), for inspection.
    pub last: Option<(String, Vec<String>)>,
}

impl LlmTranslator {
    pub fn new(model: &str, cache_path: Option<PathBuf>) -> Self {
        let mut cache = HashMap::new();
        if let Some(p) = &cache_path {
            if let Ok(src) = std::fs::read_to_string(p) {
                for line in src.lines() {
                    if let Some((k, v)) = line.split_once('\t') {
                        cache.insert(k.to_string(), v.split('\u{1f}').map(String::from).collect());
                    }
                }
            }
        }
        LlmTranslator { model: model.into(), cache, cache_path, calls: 0, last: None }
    }

    fn key(text: &str, ctx: &Context) -> String {
        let clean = |s: &str| s.replace(['\t', '\n', '\r'], " ");
        format!("{}|{}|{}", clean(&ctx.speaker), clean(ctx.last_entity.as_deref().unwrap_or("")), clean(text.trim()))
    }

    fn remember(&mut self, key: String, lines: Vec<String>) {
        // Failures are remembered in memory only, so a failed translation is
        // never retried later in the same turn (for example inside the vault
        // lock); a new session tries again.
        if !lines.is_empty() {
            if let Some(p) = &self.cache_path {
                // One writer at a time, one write per line: parallel agents never interleave.
                let _g = CACHE_FILE.lock().unwrap_or_else(|e| e.into_inner());
                if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(p) {
                    let _ = f.write_all(format!("{key}\t{}\n", lines.join("\u{1f}")).as_bytes());
                }
            }
        }
        self.cache.insert(key, lines);
    }

    /// Ask the model for canonical lines.
    fn ask_model(&mut self, text: &str, ctx: &Context) -> Vec<String> {
        let last = ctx.last_entity.as_deref().map(|e| format!("The last thing talked about is `{e}` (he/she/it/they usually mean it).")).unwrap_or_default();
        let prompt = format!(
            "You translate chat messages into a formal language for a symbolic agent. You never answer or chat yourself.\n\
             Here is the specification of the formal language:\n\n{SPEC}\n\n\
             Context: the speaker is `{}` (\"I\" = {}). {last}\n\n\
             Translate this message. A message with several parts becomes several lines (one intent per line). \
             Use the predicates of the specification where they fit; other verbs become their lemma (`play(anna, chess)`). \
             Entities are short singular nouns (one or two words). \
             Translate statements about the world, including general ones said casually with 'usually', 'most', \
             'often' or 'you know' (state them plainly: 'Jupiter is usually easy to spot' -> `property(jupiter, quality, \
             easy_to_spot)`); leave out apologies, filler, pure opinions, hypotheticals, meta talk ('let me say it \
             simpler') and anything you are unsure about. Fewer, cleaner atoms are better than many doubtful ones. \
             A question about the world is an `ask:` (or `why:`) line. A personal question to the agent about itself \
             ('Do you like soup?', 'Have you seen the stars tonight?', 'What's your favourite planet?') is \
             `about_you: <thing>` (`about_you: soup`), or `about_you: none` without a thing. \
             If nothing in it can be expressed, output `unknown:`.\n\
             Output ONLY the canonical lines, nothing else.\n\nMessage: {text}",
            ctx.speaker, ctx.speaker
        );
        self.calls += 1;
        let Ok(out) = crate::claude::call("translator", &self.model, &prompt) else { return Vec::new() };
        out.lines()
            .map(|l| l.trim().trim_matches('`').trim().to_string())
            .filter(|l| from_canonical(l).map_or(false, |i| acceptable(&i)))
            .collect()
    }

    /// Translate text into intents (cached).
    pub fn translate(&mut self, text: &str, ctx: &Context) -> Vec<Intent> {
        let key = Self::key(text, ctx);
        let lines = match self.cache.get(&key) {
            Some(l) => l.clone(),
            None => {
                let l = self.ask_model(text, ctx);
                self.remember(key, l.clone());
                l
            }
        };
        self.last = Some((text.to_string(), lines.clone()));
        lines.iter().filter_map(|l| from_canonical(l)).filter(|i| *i != Intent::Unknown).collect()
    }
}

impl Parser for LlmTranslator {
    fn parse(&mut self, text: &str, ctx: &Context) -> Intent {
        self.parse_many(text, ctx).into_iter().next().unwrap_or(Intent::Unknown)
    }

    /// Pattern parser first; the model only for what it does not understand,
    /// or when the pattern parse looks garbled (see [`suspicious`]).
    fn parse_many(&mut self, text: &str, ctx: &Context) -> Vec<Intent> {
        // Natural speech (several sentences or long ones) goes to the model as a whole,
        // so it has the context; only short, single, clean sentences stay with the patterns.
        let words = text.split_whitespace().count();
        let sentences = text.matches(['.', '!', '?']).filter(|_| true).count();
        if words > 10 || sentences > 1 {
            let v = self.translate(text, ctx);
            return if v.is_empty() { vec![Intent::Unknown] } else { v };
        }
        let p = parse(text, ctx);
        if p != Intent::Unknown && !suspicious(&p) {
            return vec![p];
        }
        if p != Intent::Unknown {
            // Garbled pattern parse: prefer the model's reading, keep the pattern one as fallback.
            let v = self.translate(text, ctx);
            return if v.is_empty() { vec![p] } else { v };
        }
        let v = self.translate(text, ctx);
        if v.is_empty() {
            vec![Intent::Unknown]
        } else {
            v
        }
    }

    fn name(&self) -> &str {
        "pattern+llm"
    }
}

/// Convenience for building symbols in tests.
pub fn sym(s: &str) -> Value {
    Value::sym(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_round_trip() {
        for c in [
            "tell: likes(anna, tea); likes(anna, coffee)",
            "tell: likes(maksim, tea) @past",
            "tell: is_a(A, person) :- is_a(A, doctor).",
            "ask: likes(anna, W)",
            "ask: likes(anna, tea) @history",
            "recall: anna",
            "recall_more: last",
            "recall_more: bird",
            "why: last",
            "why: is_a(rex, animal)",
            "correct: none",
            "correct: -likes(anna, tea)",
            "yes:",
            "no:",
            "greet:",
            "bye:",
            "how_are_you:",
            "social: thanks",
            "unknown:",
            "about_you: soup",
            "about_you: none",
            "tell: likes(cat, fish) @usually",
            "tell: likes(maksim, tea) @past",
            "tell: takes_time(boil_egg, 8, 10.5, minute)",
            "inform: available(pasta); time_available(20, minute)",
            "request: plan_dinner",
            "ask_how: fix_tire",
            "ask_need: make_pancake",
            "ask_duration: cook_rice",
            "ask_next:",
            "done: last",
            "done: make_tea_s2",
            "lack: garlic; salt",
            "accept:",
            "reject:",
            "feedback: negative",
            "feedback: positive",
            "teach: needs(make_tea, kettle); step(make_tea, make_tea_s1)",
            "new_pred: grow_on(moss, rock) | X grows on the surface of Y",
        ] {
            let i = from_canonical(c).unwrap_or_else(|| panic!("{c}"));
            assert_eq!(i.canonical(), c);
        }
        assert!(from_canonical("tell: likes(X, tea)").is_none(), "facts must be ground");
        assert!(from_canonical("hello world").is_none());
        assert!(from_canonical("tell: Likes(anna tea").is_none());
    }

    #[test]
    fn dev_set_expectations_round_trip() {
        let path = format!("{}/../../data/synthetic/parser/dev.md", env!("CARGO_MANIFEST_DIR"));
        let src = std::fs::read_to_string(path).unwrap();
        for want in src.lines().filter_map(|l| l.rsplit_once(" => ")).map(|(_, w)| w.trim()) {
            let i = from_canonical(want).unwrap_or_else(|| panic!("cannot read back {want}"));
            assert_eq!(i.canonical(), want);
        }
    }

    #[test]
    fn garbled_parses_are_suspicious() {
        let ctx = Context::default();
        assert!(suspicious(&parse("Anna has a cat named Mia.", &ctx)));
        assert!(suspicious(&parse("My mother likes soup and she also enjoys fresh bread.", &ctx)));
        assert!(!suspicious(&parse("Anna likes tea and coffee.", &ctx)));
        assert!(!suspicious(&parse("I live in New York.", &ctx)));
        assert!(!suspicious(&parse("Anna's job is teacher.", &ctx)));
    }

    #[test]
    fn model_path_with_mock() {
        use std::sync::Arc;
        let ctx = Context::default();
        // Valid lines are kept, invented formats are dropped, results are cached.
        crate::claude::set_thread_mock(Arc::new(|role, prompt| {
            assert_eq!(role, "translator");
            assert!(prompt.contains("Message: My aunt Anna, who lives nearby, really enjoys green tea in the afternoon."));
            Ok("tell: likes(anna, green_tea)\nThis is not a canonical line\n`ask: likes(anna, W)`".into())
        }));
        let mut t = LlmTranslator::new("haiku", None);
        let text = "My aunt Anna, who lives nearby, really enjoys green tea in the afternoon.";
        let v = t.parse_many(text, &ctx);
        assert_eq!(v.iter().map(|i| i.canonical()).collect::<Vec<_>>(), vec!["tell: likes(anna, green_tea)", "ask: likes(anna, W)"]);
        assert_eq!(t.calls, 1);
        t.parse_many(text, &ctx);
        assert_eq!(t.calls, 1, "the second time comes from the cache");
        // A failed call gives `unknown` and is not repeated in the same session.
        crate::claude::set_thread_mock(Arc::new(|_, _| Err("CLI not available".into())));
        let text2 = "Something long that the pattern parser will not understand at all, sadly.";
        assert_eq!(t.parse_many(text2, &ctx), vec![Intent::Unknown]);
        assert_eq!(t.parse_many(text2, &ctx), vec![Intent::Unknown]);
        assert_eq!(t.calls, 2);
        crate::claude::clear_thread_mock();
    }

    #[test]
    fn pattern_first_no_model_call() {
        let mut t = LlmTranslator::new("haiku", None);
        let v = t.parse_many("Anna likes tea.", &Context::default());
        assert_eq!(v.len(), 1);
        assert_eq!(t.calls, 0);
    }
}
