//! Stage 3: the validator. Six checks per part (Design 2.0):
//!
//! 1. shape: the line parsed (done by the reader of the model's output);
//! 2. vocabulary: predicates are registered, with the right arity and types;
//! 3. grounding: every constant comes from the message, the conversation or
//!    the registry, so the translator cannot add knowledge or slip in an answer;
//! 4. act: a question never ends up as only a statement;
//! 5. entities: glued phrases, pronouns and spelled numbers are rejected
//!    (part of the registry's type check);
//! 6. round trip: a stored fact said back in the agent's own words shares its
//!    content words with the source sentence.

use crate::TContext;
use sla_core::{Atom, GroundAtom, Literal, Term, Value};
use sla_lexicon::Registry;
use sla_parser::Intent;
use std::collections::BTreeSet;

/// Number words a person may write instead of digits.
fn number_words() -> Vec<(&'static str, i64)> {
    let mut v = vec![
        ("zero", 0), ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9), ("ten", 10),
        ("eleven", 11), ("twelve", 12), ("thirteen", 13), ("fourteen", 14), ("fifteen", 15), ("sixteen", 16), ("seventeen", 17), ("eighteen", 18), ("nineteen", 19),
        ("twenty", 20), ("thirty", 30), ("forty", 40), ("fifty", 50), ("sixty", 60), ("seventy", 70), ("eighty", 80), ("ninety", 90), ("hundred", 100),
        ("dozen", 12), ("couple", 2), ("a", 1), ("an", 1), ("single", 1), ("once", 1), ("twice", 2),
    ];
    v.push(("thousand", 1000));
    v
}

/// The words of a text, each with its singular and verb base forms.
#[derive(Debug, Clone, Default)]
pub struct Words {
    pub forms: BTreeSet<String>,
    pub numbers: BTreeSet<String>,
}

impl Words {
    pub fn of(text: &str) -> Words {
        let mut w = Words::default();
        w.add_text(text);
        w
    }

    pub fn add_text(&mut self, text: &str) {
        let low = text.to_lowercase();
        // Digits and decimals ("2.5", "10-12" gives 10 and 12).
        let mut num = String::new();
        for c in low.chars().chain(std::iter::once(' ')) {
            if c.is_ascii_digit() || (c == '.' && !num.is_empty()) {
                num.push(c);
            } else if !num.is_empty() {
                let n = num.trim_end_matches('.').to_string();
                if let Some(v) = Value::parse_number(&n) {
                    self.numbers.insert(v.to_string());
                }
                num.clear();
            }
        }
        let nw = number_words();
        let toks: Vec<String> = low.split(|c: char| !(c.is_alphanumeric() || c == '\'')).filter(|t| !t.is_empty()).map(|t| t.trim_matches('\'').to_string()).collect();
        for (k, t) in toks.iter().enumerate() {
            let t = t.strip_suffix("'s").unwrap_or(t);
            self.add_word(t);
            if let Some((_, n)) = nw.iter().find(|(w, _)| *w == t) {
                // "twenty five" -> 25, "half" handled below.
                let mut n = *n;
                if (20..100).contains(&n) && n % 10 == 0 {
                    if let Some(next) = toks.get(k + 1) {
                        if let Some((_, m)) = nw.iter().find(|(w, _)| w == next) {
                            if (1..10).contains(m) {
                                self.numbers.insert((n + m).to_string());
                            }
                        }
                    }
                }
                if t == "hundred" || t == "thousand" {
                    if let Some(prev) = k.checked_sub(1).and_then(|p| toks.get(p)).and_then(|p| nw.iter().find(|(w, _)| w == p)) {
                        n *= prev.1;
                    }
                }
                self.numbers.insert(n.to_string());
            }
            if t == "half" {
                self.numbers.insert("0.5".into());
            }
            if t == "quarter" {
                self.numbers.insert("0.25".into());
                self.numbers.insert("15".into());
            }
        }
    }

    pub fn add_word(&mut self, t: &str) {
        if t.is_empty() {
            return;
        }
        self.forms.insert(t.to_string());
        self.forms.insert(sla_parser::singularize(t));
        self.forms.insert(sla_parser::lemma(t));
        // Common irregular verbs and nouns.
        for (a, b) in [("knives", "knife"), ("leaves", "leaf"), ("children", "child"), ("people", "person"), ("men", "man"), ("women", "woman"), ("feet", "foot"), ("teeth", "tooth"), ("mice", "mouse"), ("geese", "goose")] {
            if t == a {
                self.forms.insert(b.into());
            }
        }
        for (a, b) in [("bought", "buy"), ("brought", "bring"), ("took", "take"), ("gave", "give"), ("got", "get"), ("left", "leave"), ("put", "put"), ("ran", "run"), ("saw", "see"), ("came", "come"), ("kept", "keep"), ("held", "hold"), ("sat", "sit"), ("stood", "stand"), ("fell", "fall"), ("grew", "grow"), ("knew", "know"), ("thought", "think"), ("taught", "teach"), ("caught", "catch"), ("cut", "cut"), ("set", "set"), ("fed", "feed"), ("led", "lead"), ("lost", "lose"), ("won", "win"), ("wrote", "write"), ("drove", "drive"), ("rode", "ride"), ("swam", "swim"), ("sang", "sing"), ("flew", "fly"), ("froze", "freeze"), ("broke", "break"), ("spent", "spend"), ("sent", "send"), ("lent", "lend"), ("paid", "pay"), ("said", "say"), ("told", "tell"), ("found", "find"), ("fried", "fry"), ("tried", "try")] {
            if t == a {
                self.forms.insert(b.into());
            }
        }
    }

    pub fn has(&self, w: &str) -> bool {
        self.forms.contains(w) || self.forms.contains(&sla_parser::singularize(w)) || self.forms.contains(&sla_parser::lemma(w))
    }
}

/// Words that may appear in constants without being in the message.
const FREE_PARTS: &[&str] = &["my"];

pub struct Validator<'a> {
    pub registry: &'a Registry,
}

/// Payload keywords that are not constants.
const KEYWORDS: &[&str] = &["last", "none"];

impl<'a> Validator<'a> {
    pub fn new(registry: &'a Registry) -> Self {
        Validator { registry }
    }

    /// Is the constant grounded in the message (`words`) or the context?
    fn grounded(&self, v: &Value, words: &Words, ctx: &TContext, task_like: bool) -> Result<(), String> {
        match v {
            Value::Int(_) | Value::Dec(_) => {
                let s = v.to_string();
                if words.numbers.contains(&s) {
                    Ok(())
                } else {
                    Err(format!("the number {s} is not in the message"))
                }
            }
            Value::Sym(s) => {
                if s == &ctx.speaker || KEYWORDS.contains(&s.as_str()) || self.registry.is_unit(s) && words.has(s) {
                    return Ok(());
                }
                if ctx.recent.iter().any(|r| r == s) || ctx.active_task.as_deref() == Some(s.as_str()) || ctx.known_tasks.iter().any(|t| t == s) {
                    return Ok(());
                }
                // A step of the active task: `make_tea_s3`.
                if let Some((task, n)) = s.rsplit_once("_s") {
                    if n.chars().all(|c| c.is_ascii_digit()) && !n.is_empty() && (ctx.active_task.as_deref() == Some(task) || self.grounded(&Value::sym(task), words, ctx, true).is_ok()) {
                        return Ok(());
                    }
                }
                // Ids the context knows.
                if (s.starts_with('f') || s.starts_with('e')) && s[1..].chars().all(|c| c.is_ascii_digit()) && ctx.recent.iter().any(|r| r == s) {
                    return Ok(());
                }
                let mut ctx_words = Words::default();
                for r in ctx.recent.iter().chain(ctx.active_task.iter()) {
                    for p in r.split('_') {
                        ctx_words.add_word(p);
                    }
                }
                let parts: Vec<&str> = s.split('_').filter(|p| !p.is_empty()).collect();
                for (i, p) in parts.iter().enumerate() {
                    if FREE_PARTS.contains(p) || words.has(p) || ctx_words.has(p) {
                        continue;
                    }
                    // A task's verb may be implied: "for pancakes" -> make_pancake.
                    if task_like && i == 0 && self.registry.get(p).map_or(false, |info| info.verb) {
                        continue;
                    }
                    return Err(format!("`{s}` is not grounded: \"{p}\" is not in the message or the conversation"));
                }
                Ok(())
            }
        }
    }

    fn check_fact(&self, a: &Atom, words: &Words, ctx: &TContext, problems: &mut Vec<String>) {
        let Some(g) = a.to_ground() else {
            problems.push(format!("{a} is not ground"));
            return;
        };
        if let Err(e) = self.registry.check(&g) {
            problems.push(format!("{g}: {e}"));
            return;
        }
        let info = self.registry.get(g.base_pred());
        let types: Vec<sla_lexicon::ArgType> = info.map(|i| i.types.clone()).unwrap_or_default();
        for (k, v) in g.args.iter().enumerate() {
            let ty = types.get(k).copied();
            // Property names may be implied ("The car is red" -> colour).
            if g.base_pred() == "property" && k == 1 && v.as_sym().map_or(false, |s| self.registry.property_names.contains(s)) {
                continue;
            }
            // Units may be implied by the number's context ("20 min").
            if ty == Some(sla_lexicon::ArgType::Unit) {
                continue;
            }
            let task_like = matches!(ty, Some(sla_lexicon::ArgType::Task) | Some(sla_lexicon::ArgType::Step)) || matches!(g.base_pred(), "solves");
            if let Err(e) = self.grounded(v, words, ctx, task_like) {
                problems.push(e);
            }
        }
    }

    fn check_query_atom(&self, a: &Atom, words: &Words, ctx: &TContext, problems: &mut Vec<String>) {
        let Some(info) = self.registry.get(&a.pred) else {
            problems.push(format!("unknown predicate {}", a.pred));
            return;
        };
        if !info.verb && info.types.len() != a.args.len() {
            problems.push(format!("{} takes {} argument(s)", a.pred, info.types.len()));
            return;
        }
        for (k, t) in a.args.iter().enumerate() {
            if let Term::Val(v) = t {
                if a.pred == "property" && k == 1 && v.as_sym().map_or(false, |s| self.registry.property_names.contains(s)) {
                    continue;
                }
                if info.types.get(k) == Some(&sla_lexicon::ArgType::Unit) {
                    continue;
                }
                if let Value::Sym(s) = v {
                    if let Some(p) = self.registry.entity_problem(s) {
                        problems.push(format!("`{s}` {p}"));
                        continue;
                    }
                }
                if let Err(e) = self.grounded(v, words, ctx, false) {
                    problems.push(e);
                }
            }
        }
    }

    /// Problems with one sentence's acts (empty: it passes).
    pub fn check(&self, sentence: &str, acts: &[Intent], ctx: &TContext) -> Vec<String> {
        let mut problems = Vec::new();
        if acts.is_empty() {
            problems.push("no translation".into());
            return problems;
        }
        if acts.iter().all(|i| *i == Intent::Unknown) {
            return problems;
        }
        let words = Words::of(sentence);
        // 4. A question never ends up as only a statement.
        if sentence.trim_end().ends_with('?') && acts.iter().all(|i| matches!(i, Intent::Tell { .. } | Intent::Inform(_) | Intent::Teach { .. })) {
            problems.push("a question was translated as a statement only".into());
        }
        for act in acts {
            match act {
                Intent::Tell { facts, rules, .. } | Intent::Teach { facts, rules } => {
                    for a in facts {
                        self.check_fact(a, &words, ctx, &mut problems);
                    }
                    for r in rules {
                        for l in std::iter::once(&Literal::Pos(r.head.clone())).chain(r.body.iter()) {
                            if let Literal::Pos(a) | Literal::Neg(a) = l {
                                self.check_query_atom(a, &words, ctx, &mut problems);
                            }
                        }
                    }
                    // 6. Round trip for lasting knowledge.
                    for a in facts {
                        if let Some(g) = a.to_ground() {
                            if let Some(p) = self.round_trip(&g, &words) {
                                problems.push(p);
                            }
                        }
                    }
                }
                Intent::Inform(facts) | Intent::Correct(facts) => {
                    for a in facts {
                        self.check_fact(a, &words, ctx, &mut problems);
                    }
                }
                Intent::Ask { query, .. } => {
                    for l in query {
                        if let Literal::Pos(a) | Literal::Neg(a) = l {
                            self.check_query_atom(a, &words, ctx, &mut problems);
                        }
                    }
                }
                Intent::Why(Some(a)) => self.check_query_atom(a, &words, ctx, &mut problems),
                Intent::Request(t) | Intent::AskHow(t) | Intent::AskNeed(t) | Intent::AskDuration(t) => {
                    let v = Value::sym(t.as_str());
                    if let Some(p) = self.registry.entity_problem(t) {
                        problems.push(format!("task `{t}` {p}"));
                    } else if let Err(e) = self.grounded(&v, &words, ctx, true) {
                        problems.push(e);
                    }
                }
                Intent::Recall(e) | Intent::RecallMore(Some(e)) | Intent::AboutYou(Some(e)) => {
                    if let Err(err) = self.grounded(&Value::sym(e.as_str()), &words, ctx, false) {
                        problems.push(err);
                    }
                }
                Intent::Lack(items) => {
                    for i in items {
                        if let Err(e) = self.grounded(&Value::sym(i.as_str()), &words, ctx, false) {
                            problems.push(e);
                        }
                    }
                }
                Intent::Done(Some(s)) => {
                    if let Err(e) = self.grounded(&Value::sym(s.as_str()), &words, ctx, true) {
                        problems.push(e);
                    }
                }
                Intent::NewPred { atom, .. } => {
                    if self.registry.get(&atom.pred).is_some() {
                        problems.push(format!("{} is already a registered predicate: use it", atom.pred));
                    }
                    for v in atom.args.iter().filter_map(|t| if let Term::Val(v) = t { Some(v) } else { None }) {
                        if let Err(e) = self.grounded(v, &words, ctx, false) {
                            problems.push(e);
                        }
                    }
                }
                _ => {}
            }
        }
        problems
    }

    /// The fact in the agent's own words must share content words with the
    /// sentence (a symbolic stand-in for "means the same").
    fn round_trip(&self, g: &GroundAtom, words: &Words) -> Option<String> {
        let mut said = Words::default();
        for v in &g.args {
            if let Some(s) = v.as_sym() {
                for p in s.split('_') {
                    said.add_word(p);
                }
            }
        }
        let content: Vec<&String> = said.forms.iter().filter(|w| w.len() > 2 && !self.registry.function_words.contains(*w) && !self.registry.is_unit(w)).collect();
        if content.is_empty() {
            return None;
        }
        // A verb predicate must have its verb (or a synonym) in the sentence.
        if let Some(info) = self.registry.get(g.base_pred()) {
            if info.verb && !words.has(&info.name) {
                let syn = self.registry.synonyms.iter().any(|(s, p)| p == &info.name && words.has(s));
                if !syn {
                    return Some(format!("{g}: the verb \"{}\" is not in the sentence", info.name));
                }
            }
        }
        let shared = content.iter().filter(|w| words.has(w)).count();
        if shared == 0 {
            return Some(format!("{g} does not say what the sentence says"));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sla_parser::llm::from_canonical;

    fn ctx() -> TContext {
        TContext { speaker: "maksim".into(), ..Default::default() }
    }

    fn check(sentence: &str, lines: &[&str]) -> Vec<String> {
        let r = Registry::builtin();
        let v = Validator::new(&r);
        let acts: Vec<Intent> = lines.iter().map(|l| from_canonical(l).unwrap_or_else(|| panic!("{l}"))).collect();
        v.check(sentence, &acts, &ctx())
    }

    #[test]
    fn good_translations_pass() {
        assert!(check("I have pasta, garlic and some cherry tomatoes.", &["inform: available(pasta); available(garlic); available(cherry_tomato)"]).is_empty());
        assert!(check("Maybe twenty minutes.", &["inform: time_available(20, minute)"]).is_empty());
        assert!(check("Can you help me plan dinner tonight?", &["request: plan_dinner"]).is_empty());
        assert!(check("What do I need for pancakes?", &["ask_need: make_pancake"]).is_empty(), "the task verb may be implied");
        assert!(check("Boiling an egg takes 8 to 10 minutes.", &["tell: takes_time(boil_egg, 8, 10, minute)"]).is_empty());
        assert!(check("The car is red.", &["tell: property(car, colour, red)"]).is_empty());
        assert!(check("Anna enjoys green tea.", &["tell: likes(anna, green_tea)"]).is_empty());
        assert!(check("Does Anna like tea?", &["ask: likes(anna, tea)"]).is_empty());
        assert!(!check("It takes half an hour.", &["tell: takes_time(walk, 0.5, 0.5, hour)"]).is_empty(), "walk is not grounded");
    }

    #[test]
    fn bad_translations_fail() {
        // The model added knowledge that is not in the message.
        assert!(!check("I have pasta.", &["inform: available(pasta); available(basil)"]).is_empty());
        assert!(!check("Boil the eggs for a while.", &["tell: takes_time(boil_egg, 8, 10, minute)"]).is_empty(), "numbers must be in the message");
        // A question is not a statement.
        assert!(!check("Anna likes tea?", &["tell: likes(anna, tea)"]).is_empty());
        // Invented predicates and glued constants.
        assert!(!check("Frozen means turned into ice.", &["tell: mean(frozen, ice)"]).is_empty());
        assert!(!check("A pan is good for eggs and frying.", &["tell: property(pan, use, good_for_eggs_and_frying)"]).is_empty());
        // A verb that is not in the sentence.
        assert!(!check("Anna has a cat.", &["tell: feed(anna, cat)"]).is_empty());
    }

    #[test]
    fn number_words() {
        let w = Words::of("about twenty five minutes, or half an hour, maybe 2.5 hours or 10-12");
        for n in ["25", "20", "5", "0.5", "2.5", "10", "12", "1"] {
            assert!(w.numbers.contains(n), "{n}: {:?}", w.numbers);
        }
    }
}
