//! Pattern-based input parser (ELIZA-style intents and slots).
//!
//! Turns one user message into an [`Intent`] with Datalog atoms as slots.
//! The output format is specified in `docs/parser_spec.md`. The [`Parser`]
//! trait lets an LLM translator be plugged in later.

pub mod lexicon;
pub mod llm;
mod words;

pub use words::lemma;

use lexicon::*;
use sla_core::{Atom, Literal, Rule, Term, Value};
use words::{singularize, tokenize, Tok};

/// What the user wants, with slots as atoms.
#[derive(Debug, Clone, PartialEq)]
pub enum Intent {
    /// New facts (ground atoms) or rules. `past`: "used to" (true before, not now).
    Tell { facts: Vec<Atom>, rules: Vec<Rule>, past: bool },
    /// A query; `history`: about the past rather than now.
    Ask { query: Vec<Literal>, history: bool },
    /// "What do you know about X?"
    Recall(String),
    /// "What else do you know (about X)?", "Tell me more": continue the last recall.
    RecallMore(Option<String>),
    /// Everyday conversation: thanks, reactions, questions about the agent itself.
    Social(Social),
    /// "Why ...?" about an atom, or the last answer if `None`.
    Why(Option<Atom>),
    /// A correction: the right statements (empty = "that's wrong").
    Correct(Vec<Atom>),
    Yes,
    No,
    Greet,
    /// "How are you?"
    HowAreYou,
    Bye,
    Unknown,
}

impl Intent {
    /// One-line canonical rendering used by tests: `<intent>: <payload>`.
    pub fn canonical(&self) -> String {
        let atoms = |v: &[Atom]| v.iter().map(|a| a.to_string()).collect::<Vec<_>>().join("; ");
        match self {
            Intent::Tell { facts, rules, past } => {
                let mut parts: Vec<String> = facts.iter().map(|a| a.to_string()).collect();
                parts.extend(rules.iter().map(|r| r.to_string()));
                let mut s = format!("tell: {}", parts.join("; "));
                if *past {
                    s.push_str(" @past");
                }
                s
            }
            Intent::Ask { query, history } => {
                let q = query.iter().map(|l| l.to_string()).collect::<Vec<_>>().join(", ");
                if *history {
                    format!("ask: {q} @history")
                } else {
                    format!("ask: {q}")
                }
            }
            Intent::Recall(e) => format!("recall: {}", Value::sym(e.as_str())),
            Intent::RecallMore(Some(e)) => format!("recall_more: {}", Value::sym(e.as_str())),
            Intent::RecallMore(None) => "recall_more: last".into(),
            Intent::Social(k) => format!("social: {}", k.name()),
            Intent::Why(None) => "why: last".into(),
            Intent::Why(Some(a)) => format!("why: {a}"),
            Intent::Correct(v) if v.is_empty() => "correct: none".into(),
            Intent::Correct(v) => format!("correct: {}", atoms(v)),
            Intent::Yes => "yes:".into(),
            Intent::No => "no:".into(),
            Intent::Greet => "greet:".into(),
            Intent::HowAreYou => "how_are_you:".into(),
            Intent::Bye => "bye:".into(),
            Intent::Unknown => "unknown:".into(),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Intent::Tell { .. } => "tell",
            Intent::Ask { .. } => "ask",
            Intent::Recall(_) => "recall",
            Intent::RecallMore(_) => "recall_more",
            Intent::Social(_) => "social",
            Intent::Why(_) => "why",
            Intent::Correct(_) => "correct",
            Intent::Yes => "yes",
            Intent::No => "no",
            Intent::Greet => "greet",
            Intent::HowAreYou => "how_are_you",
            Intent::Bye => "bye",
            Intent::Unknown => "unknown",
        }
    }
}

/// Small-talk moves between friends.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Social {
    /// "thanks", "thank you"
    Thanks,
    /// "cool", "nice", "really?", "I see"
    Reaction,
    /// "who are you?", "what is your name?"
    WhoAreYou,
    /// "what can you do?", "how do you work?"
    WhatCanYouDo,
    /// "and you?", "how about you?"
    AndYou,
    /// "I don't know", "no idea": a non-answer, never a fact.
    DontKnow,
}

impl Social {
    pub fn name(self) -> &'static str {
        match self {
            Social::Thanks => "thanks",
            Social::Reaction => "reaction",
            Social::WhoAreYou => "who_are_you",
            Social::WhatCanYouDo => "what_can_you_do",
            Social::AndYou => "and_you",
            Social::DontKnow => "dont_know",
        }
    }
}

/// Conversation context the parser may use.
#[derive(Debug, Clone)]
pub struct Context {
    /// Who "I"/"me"/"my" refers to.
    pub speaker: String,
    /// The last entity talked about: "he", "she", "it", "they" refer to it.
    pub last_entity: Option<String>,
}

impl Default for Context {
    fn default() -> Self {
        Context { speaker: "maksim".into(), last_entity: None }
    }
}

/// A translator from text to intents. The pattern parser is the default;
/// an LLM translator can implement this trait later.
pub trait Parser {
    fn parse(&mut self, text: &str, ctx: &Context) -> Intent;

    /// A message may carry several intents ("Anna is a teacher and she likes tea. Who likes tea?").
    fn parse_many(&mut self, text: &str, ctx: &Context) -> Vec<Intent> {
        vec![self.parse(text, ctx)]
    }

    fn name(&self) -> &str {
        "pattern"
    }
}

#[derive(Debug, Default, Clone)]
pub struct PatternParser;

impl Parser for PatternParser {
    fn parse(&mut self, text: &str, ctx: &Context) -> Intent {
        parse(text, ctx)
    }
}

/// Placeholder tokens for variables.
const VAR_W: &str = "?w";
const VAR_A: &str = "?a";

fn ws(toks: &[Tok]) -> Vec<&str> {
    toks.iter().map(|t| t.w.as_str()).collect()
}

fn starts(toks: &[Tok], pat: &[&str]) -> bool {
    toks.len() >= pat.len() && toks.iter().zip(pat).all(|(t, p)| t.w == *p)
}

fn tok(w: &str) -> Tok {
    Tok { w: w.to_string(), cap: false }
}

/// Parse one message.
pub fn parse(text: &str, ctx: &Context) -> Intent {
    let (mut toks, question) = tokenize(text);
    {
        let raw: Vec<&str> = toks.iter().map(|t| t.w.as_str()).filter(|w| *w != ",").collect();
        let raw = raw.join(" ");
        let raw = raw.trim_start_matches("oh ").trim_start_matches("well ");
        for (list, kind) in [
            (THANKS, Social::Thanks),
            (REACTIONS, Social::Reaction),
            (WHO_ARE_YOU, Social::WhoAreYou),
            (WHAT_CAN_YOU_DO, Social::WhatCanYouDo),
            (AND_YOU, Social::AndYou),
        ] {
            if list.contains(&raw) {
                return Intent::Social(kind);
            }
        }
        if MORE.contains(&raw) {
            return Intent::RecallMore(None);
        }
    }
    strip_fillers(&mut toks);
    if toks.first().map_or(false, |t| t.w != "if") {
        toks.retain(|t| t.w != ",");
    }
    if toks.is_empty() {
        return Intent::Unknown;
    }
    let w = ws(&toks);
    let joined = w.join(" ");

    // A greeting word followed by more: the rest is the message ("Hi, Anna likes tea"),
    // and a self-introduction ("Hi, I'm Maksim") is just a greeting.
    if GREET.contains(&w[0]) && w.len() > 2 && !starts(&toks, &["hey", "there"]) && !starts(&toks, &["hi", "there"]) && !starts(&toks, &["hello", "there"]) {
        let rest: Vec<&str> = w[1..].to_vec();
        if rest.len() <= 3 && (rest.starts_with(&["i", "am"]) || rest.starts_with(&["my", "name", "is"]) || rest == ["there"]) {
            return Intent::Greet;
        }
        let rest_text: Vec<String> = toks[1..].iter().map(|t| if t.cap { capitalize_word(&t.w) } else { t.w.clone() }).collect();
        let inner = parse(&format!("{}{}", rest_text.join(" "), if question { "?" } else { "" }), ctx);
        return if inner == Intent::Unknown { Intent::Greet } else { inner };
    }
    if starts(&toks, &["i", "am"]) && toks.len() == 3 && toks[2].cap && toks[2].w == ctx.speaker {
        return Intent::Greet;
    }
    // Fixed phrases.
    if GREET.contains(&joined.as_str()) || (GREET.contains(&w[0]) && w.len() <= 2) || starts(&toks, &["good", "morning"]) || starts(&toks, &["good", "evening"]) || starts(&toks, &["good", "afternoon"]) {
        return Intent::Greet;
    }
    if BYE.contains(&joined.as_str()) || starts(&toks, &["see", "you"]) || (BYE.contains(&w[0]) && w.len() <= 2) {
        return Intent::Bye;
    }
    if YES.contains(&joined.as_str()) {
        return Intent::Yes;
    }
    if NO.contains(&joined.as_str()) {
        return Intent::No;
    }
    if HOW_ARE_YOU.contains(&joined.as_str()) {
        return Intent::HowAreYou;
    }
    // "I don't know" is a non-answer, never a fact.
    if DONT_KNOW.contains(&joined.as_str()) {
        return Intent::Social(Social::DontKnow);
    }
    if WRONG.contains(&joined.as_str()) {
        return Intent::Correct(Vec::new());
    }

    // Corrections: "no, <statement>", "actually <statement>", "wrong, <statement>".
    for lead in [&["no"][..], &["actually"], &["wrong"], &["that", "is", "wrong"], &["nope"], &["no", "actually"]] {
        if starts(&toks, lead) && toks.len() > lead.len() {
            let rest = &toks[lead.len()..];
            if let Some(atoms) = statement_atoms(rest, ctx) {
                return Intent::Correct(atoms);
            }
        }
    }

    // "What else do you know about X?", "Tell me more about X".
    for lead in [
        &["what", "else", "do", "you", "know", "about"][..],
        &["what", "more", "do", "you", "know", "about"],
        &["tell", "me", "more", "about"],
        &["anything", "else", "about"],
        &["what", "else", "about"],
        &["what", "else", "can", "you", "tell", "me", "about"],
        &["more", "about"],
    ] {
        if starts(&toks, lead) && toks.len() > lead.len() {
            if let Some(Term::Val(Value::Sym(e))) = entity(&toks[lead.len()..], ctx, true) {
                return Intent::RecallMore(Some(e));
            }
        }
    }
    // Recall.
    for lead in [&["what", "do", "you", "know", "about"][..], &["tell", "me", "about"], &["what", "about"], &["tell", "me", "everything", "about"]] {
        if starts(&toks, lead) && toks.len() > lead.len() {
            if let Some(Term::Val(Value::Sym(e))) = entity(&toks[lead.len()..], ctx, true) {
                return Intent::Recall(e);
            }
        }
    }

    // Why.
    if w[0] == "why" {
        if toks.len() == 1 {
            return Intent::Why(None);
        }
        if let Some((atoms, _)) = question_atoms(&toks[1..], ctx) {
            return Intent::Why(atoms.into_iter().next());
        }
        return Intent::Why(None);
    }
    for lead in [&["how", "do", "you", "know", "that"][..], &["how", "do", "you", "know"]] {
        if starts(&toks, lead) {
            let rest = &toks[lead.len()..];
            if rest.is_empty() {
                return Intent::Why(None);
            }
            if let Some(atoms) = statement_atoms(rest, ctx) {
                return Intent::Why(atoms.into_iter().next());
            }
        }
    }

    // Rules: "if A then B".
    if w[0] == "if" {
        if let Some(rules) = if_then(&toks[1..], ctx) {
            return Intent::Tell { facts: Vec::new(), rules, past: false };
        }
        return Intent::Unknown;
    }

    // Questions.
    let qword = QWORDS.contains(&w[0]);
    let aux_start = Q_AUX.contains(&w[0]);
    if question || qword {
        if let Some((atoms, history)) = wh_question(&toks, ctx).or_else(|| question_atoms(&toks, ctx)) {
            return Intent::Ask { query: atoms.into_iter().map(Literal::Pos).collect(), history };
        }
        if question {
            return Intent::Unknown;
        }
    } else if aux_start && toks.len() >= 3 {
        if let Some((atoms, history)) = question_atoms(&toks, ctx) {
            return Intent::Ask { query: atoms.into_iter().map(Literal::Pos).collect(), history };
        }
    }

    // Statements.
    match statement(&toks, ctx) {
        Some(st) => {
            let (facts, rules): (Vec<Atom>, Vec<Atom>) = st.atoms.into_iter().partition(|a| a.is_ground());
            let rules = rules
                .into_iter()
                .filter_map(|h| st.generic.as_ref().map(|c| Rule { head: h, body: vec![Literal::Pos(Atom::new("is_a", vec![Term::var("A"), Term::sym(c)]))] }))
                .collect();
            Intent::Tell { facts, rules, past: st.past }
        }
        None => Intent::Unknown,
    }
}

fn strip_fillers(toks: &mut Vec<Tok>) {
    toks.retain(|t| !DROP_ANYWHERE.contains(&t.w.as_str()));
    // "a lot", "very much", "every day", "these days", "no longer" handled elsewhere.
    let mut i = 0;
    while i + 1 < toks.len() {
        let pair = (toks[i].w.as_str(), toks[i + 1].w.as_str());
        if pair == ("a", "lot") && toks.get(i + 2).map_or(false, |t| t.w == "of") {
            toks.drain(i..i + 3);
        } else if matches!(pair, ("a", "lot") | ("very", "much") | ("every", "day") | ("these", "days") | ("every", "morning") | ("at", "all")) {
            toks.drain(i..i + 2);
        } else {
            i += 1;
        }
    }
    loop {
        let Some(first) = toks.first() else { break };
        let w = first.w.as_str();
        if LEAD_FILLERS.contains(&w) && toks.len() > 1 {
            toks.remove(0);
        } else if (w == "i" && toks.len() > 2 && matches!(toks[1].w.as_str(), "think" | "believe" | "guess") && !question_like(&toks[2..])) || (w == "you" && toks.len() > 2 && toks[1].w == "know") {
            // "I think Anna likes tea" -> "Anna likes tea"
            toks.drain(0..2);
            if toks.first().map_or(false, |t| t.w == "that") {
                toks.remove(0);
            }
        } else {
            break;
        }
    }
}

fn question_like(toks: &[Tok]) -> bool {
    toks.is_empty() || matches!(toks[0].w.as_str(), "so" | "not" | "maybe")
}

// ------------------------------------------------------------------ entities

fn is_det(w: &str) -> bool {
    DETERMINERS.contains(&w)
}

/// A noun phrase -> constant (or variable placeholder).
fn entity(toks: &[Tok], ctx: &Context, singular: bool) -> Option<Term> {
    let content: Vec<&Tok> = toks.iter().filter(|t| !is_det(&t.w) && t.w != "'s").collect();
    if content.is_empty() {
        return None;
    }
    if content.len() == 1 {
        let w = content[0].w.as_str();
        if w == VAR_W {
            return Some(Term::var("W"));
        }
        if w == VAR_A || (VAR_PRONOUNS.contains(&w) && ctx_in_rule()) {
            return Some(Term::var("A"));
        }
        if PERSONAL_PRONOUNS.contains(&w) {
            return ctx.last_entity.as_ref().map(|e| Term::sym(e));
        }
        if SPEAKER.contains(&w) {
            return Some(Term::sym(&ctx.speaker));
        }
        if let Ok(n) = w.parse::<i64>() {
            return Some(Term::Val(Value::Int(n)));
        }
    }
    if content.iter().any(|t| BAD_IN_ENTITY.contains(&t.w.as_str())) {
        return None;
    }
    let n = content.len();
    if n > 4 {
        return None;
    }
    let parts: Vec<String> = content
        .iter()
        .enumerate()
        .map(|(i, t)| if i == n - 1 && singular && !t.cap { singularize(&t.w) } else { t.w.clone() })
        .collect();
    Some(Term::sym(&parts.join("_")))
}

thread_local! {
    static IN_RULE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

fn ctx_in_rule() -> bool {
    IN_RULE.with(|c| c.get())
}

/// Split a span on "and" / "or" into entities.
fn entities(toks: &[Tok], ctx: &Context, singular: bool) -> Option<Vec<Term>> {
    let mut out = Vec::new();
    for part in toks.split(|t| t.w == "and" || t.w == "or") {
        if part.is_empty() {
            continue;
        }
        out.push(entity(part, ctx, singular)?);
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

// ------------------------------------------------------------------ statements

#[derive(Debug, Default)]
struct Statement {
    atoms: Vec<Atom>,
    past: bool,
    /// Plural/"every" subject category: non-ground atoms become rules over it.
    generic: Option<String>,
    /// "did"/"was"/"were": about the past (for questions).
    past_tense: bool,
}

fn is_verbish(w: &str) -> bool {
    AUX.contains(&w) || VERBS.contains(&w) || VERBS.contains(&lemma(w).as_str()) || w == VAR_W
}

/// Index where the subject ends (the first verb-like token after content).
fn subject_end(toks: &[Tok]) -> Option<usize> {
    let content_before = |i: usize| toks[..i].iter().any(|t| !is_det(&t.w));
    if toks.is_empty() {
        return None;
    }
    // "my family likes ...": a possessive noun phrase, not the pronoun alone.
    let possessive_np = toks[0].w == "my" && toks.len() > 2 && !is_verbish(&toks[1].w);
    // Pronouns and placeholders are one token.
    if !possessive_np && SPEAKER.contains(&toks[0].w.as_str()) || VAR_PRONOUNS.contains(&toks[0].w.as_str()) || toks[0].w == VAR_W || toks[0].w == VAR_A {
        return if toks.len() > 1 { Some(1) } else { None };
    }
    for i in 1..toks.len() {
        if is_verbish(&toks[i].w) && content_before(i) && toks[i].w != "and" {
            return Some(i);
        }
    }
    // Fallback: [det] word, then an -s verb.
    let mut i = 0;
    while i < toks.len() && is_det(&toks[i].w) {
        i += 1;
    }
    i += 1;
    while i < toks.len() && toks[i].cap && toks[i - 1].cap {
        i += 1;
    }
    if i < toks.len() && toks[i].w.ends_with('s') && toks[i].w.len() > 2 {
        Some(i)
    } else {
        None
    }
}

fn atom(pred: &str, args: Vec<Term>, neg: bool) -> Atom {
    let p = if neg { format!("-{pred}") } else { pred.to_string() };
    Atom { pred: p, args }
}

fn statement_atoms(toks: &[Tok], ctx: &Context) -> Option<Vec<Atom>> {
    let st = statement(toks, ctx)?;
    if st.generic.is_some() && st.atoms.iter().any(|a| !a.is_ground()) {
        return None;
    }
    Some(st.atoms)
}

/// Parse a declarative clause.
fn statement(toks: &[Tok], ctx: &Context) -> Option<Statement> {
    // Possessive properties: "X's P is V", "my P is V", "the P of X is V".
    if let Some(st) = possessive(toks, ctx) {
        return Some(st);
    }
    let mut toks: Vec<Tok> = toks.to_vec();
    let mut generic: Option<String> = None;
    let mut forced_generic = false;
    if matches!(toks.first().map(|t| t.w.as_str()), Some("every") | Some("all") | Some("each")) {
        toks.remove(0);
        forced_generic = true;
    }
    let se = subject_end(&toks)?;
    let subj_toks = &toks[..se];
    let mut rest: Vec<Tok> = toks[se..].to_vec();

    // Auxiliaries, negation and tense.
    let mut neg = false;
    let mut past = false;
    let mut past_tense = false;
    loop {
        let Some(first) = rest.first() else { break };
        match first.w.as_str() {
            "does" | "do" | "will" | "can" | "would" => {
                rest.remove(0);
            }
            "did" => {
                past_tense = true;
                rest.remove(0);
            }
            "not" | "never" => {
                neg = true;
                rest.remove(0);
            }
            "no" if rest.get(1).map_or(false, |t| t.w == "longer") => {
                neg = true;
                rest.drain(0..2);
            }
            "used" | "use" if rest.get(1).map_or(false, |t| t.w == "to") => {
                past = true;
                rest.drain(0..2);
            }
            _ => break,
        }
    }
    if rest.is_empty() {
        return None;
    }
    let verb_raw = rest[0].w.clone();
    let mut obj: Vec<Tok> = rest[1..].to_vec();

    // Generic subject: "every cat", or a plural common noun with a plural verb.
    let base_verb = !verb_raw.ends_with('s') || verb_raw == "is" || verb_raw == "was" || verb_raw == "has";
    let subj_content: Vec<&Tok> = subj_toks.iter().filter(|t| !is_det(&t.w)).collect();
    let plural_noun = subj_content.len() == 1
        && !SPEAKER.contains(&subj_content[0].w.as_str())
        && !VAR_PRONOUNS.contains(&subj_content[0].w.as_str())
        && subj_content[0].w != VAR_W
        && subj_content[0].w != VAR_A
        && looks_plural(&subj_content[0].w)
        && (verb_raw == "are" || verb_raw == "were" || (base_verb && !AUX_SINGULAR.contains(&verb_raw.as_str())));
    if forced_generic || plural_noun {
        if subj_content.len() == 1 {
            generic = Some(singularize(&subj_content[0].w));
        } else {
            return None;
        }
    }
    let subjects: Vec<Term> = match &generic {
        Some(c) => vec![Term::sym(c)],
        None => entities(subj_toks, ctx, true)?,
    };

    // Verb phrase.
    let mut is_be = BE.contains(&verb_raw.as_str());
    let mut verb = lemma(&verb_raw);
    if is_be {
        if let Some(t) = obj.first() {
            if t.w == "not" {
                neg = true;
                obj.remove(0);
            }
        }
        if verb_raw == "was" || verb_raw == "were" {
            past_tense = true;
        }
        // Progressive: "is drinking tea" -> drink.
        if let Some(t) = obj.first() {
            if t.w.ends_with("ing") && t.w.len() > 4 {
                let base = lemma(&t.w);
                if VERBS.contains(&base.as_str()) {
                    verb = base;
                    is_be = false;
                    obj.remove(0);
                }
            }
        }
    }
    if obj.first().map_or(false, |t| t.w == "not") {
        neg = true;
        obj.remove(0);
    }

    let mut atoms = Vec::new();
    let push = |pred: &str, objs: &[Option<Term>], atoms: &mut Vec<Atom>, neg: bool| {
        for s in &subjects {
            for o in objs {
                let mut args = vec![s.clone()];
                if let Some(o) = o {
                    args.push(o.clone());
                }
                atoms.push(atom(pred, args, neg));
            }
        }
    };
    let objs = |toks: &[Tok]| -> Option<Vec<Option<Term>>> { Some(entities(toks, ctx, true)?.into_iter().map(Some).collect()) };

    if is_be {
        let o: Vec<&str> = obj.iter().map(|t| t.w.as_str()).collect();
        if o.is_empty() {
            return None;
        }
        if o[0] == "in" || o[0] == "at" || (o[0] == "located" && o.len() > 1) {
            let skip = if o[0] == "located" { 2 } else { 1 };
            push("located_in", &objs(&obj[skip..])?, &mut atoms, neg);
        } else if o.len() > 2 && o[0] == "part" && o[1] == "of" {
            push("part_of", &objs(&obj[2..])?, &mut atoms, neg);
        } else if matches!(o[0], "a" | "an" | "the") {
            push("is_a", &objs(&obj[1..])?, &mut atoms, neg);
        } else {
            // Adjectives (possibly several, "red and big") or a plural noun.
            let parts: Vec<&[Tok]> = obj.split(|t| t.w == "and" || t.w == "or").filter(|p| !p.is_empty()).collect();
            for p in parts {
                let words: Vec<&str> = p.iter().map(|t| t.w.as_str()).collect();
                let last = *words.last()?;
                if p.len() == 1 && EMOTIONS.contains(&last) {
                    push("feels", &[Some(Term::sym(last))], &mut atoms, neg);
                } else if p.len() == 1 && COLOURS.contains(&last) {
                    for s in &subjects {
                        atoms.push(atom("property", vec![s.clone(), Term::sym("colour"), Term::sym(last)], neg));
                    }
                } else if p.len() == 1 && SIZES.contains(&last) {
                    for s in &subjects {
                        atoms.push(atom("property", vec![s.clone(), Term::sym("size"), Term::sym(last)], neg));
                    }
                } else if p.len() == 1 && p[0].w == VAR_W {
                    push("is_a", &[Some(Term::var("W"))], &mut atoms, neg);
                } else if (generic.is_some() || subjects.len() > 1) && looks_plural(last) && !ADJ_LIKE_S.contains(&last) {
                    push("is_a", &objs(p)?, &mut atoms, neg);
                } else if p.len() == 1 {
                    // One adjective, also capitalized ones like nationalities ("Anna is British").
                    for s in &subjects {
                        atoms.push(atom("property", vec![s.clone(), Term::sym("quality"), Term::sym(last)], neg));
                    }
                } else if p.iter().all(|t| t.cap) {
                    // "Rex is Fido" - sameness is rare; treat as a name category.
                    push("is_a", &objs(p)?, &mut atoms, neg);
                } else {
                    return None;
                }
            }
        }
    } else {
        let v = verb.as_str();
        // Map to core predicates.
        let (pred, negate_more, skip): (String, bool, usize) = match v {
            "like" | "love" | "enjoy" | "adore" => ("likes".into(), false, 0),
            "dislike" | "hate" | "detest" => ("likes".into(), true, 0),
            "have" | "own" | "possess" => ("has".into(), false, 0),
            "want" | "need" => ("wants".into(), false, 0),
            "know" => ("knows".into(), false, 0),
            "feel" => ("feels".into(), false, 0),
            "cause" => ("causes".into(), false, 0),
            "live" if obj.first().map_or(false, |t| matches!(t.w.as_str(), "in" | "at")) => ("located_in".into(), false, 1),
            _ => {
                if let Some(p) = obj.first().filter(|t| PREPS.contains(&t.w.as_str())) {
                    (format!("{v}_{}", p.w), false, 1)
                } else {
                    (v.to_string(), false, 0)
                }
            }
        };
        if negate_more {
            neg = !neg;
        }
        if !is_plausible_verb(v) {
            return None;
        }
        let mut rest = &obj[skip..];
        // "likes to eat rice", "wants to visit Paris": the object after the second verb.
        if matches!(pred.as_str(), "likes" | "wants") && rest.len() >= 3 && rest[0].w == "to" && (VERBS.contains(&rest[1].w.as_str())) {
            rest = &rest[2..];
        }
        if rest.is_empty() {
            if skip > 0 {
                return None;
            }
            push(&pred, &[None], &mut atoms, neg);
        } else {
            if generic.is_some() && pred == "has" {
                push(&pred, &objs(rest)?, &mut atoms, neg);
            } else {
                push(&pred, &objs(rest)?, &mut atoms, neg);
            }
        }
    }
    if atoms.is_empty() {
        return None;
    }
    // Generic subjects: "has"/"is_a"/"property"/"feels" stay facts about the
    // category; any other verb becomes a rule over members.
    if let Some(c) = &generic {
        let keep_fact = |a: &Atom| matches!(a.pred.trim_start_matches('-'), "has" | "is_a" | "property" | "feels" | "located_in" | "part_of");
        atoms = atoms
            .into_iter()
            .map(|a| {
                if keep_fact(&a) {
                    a
                } else {
                    let mut a = a;
                    a.args[0] = Term::var("A");
                    let _ = c;
                    a
                }
            })
            .collect();
    }
    Some(Statement { atoms, past, generic, past_tense })
}

fn looks_plural(w: &str) -> bool {
    w.len() > 2 && w.ends_with('s') && !w.ends_with("ss") && !w.ends_with("us") && !w.ends_with("is")
}

fn is_plausible_verb(v: &str) -> bool {
    !v.is_empty() && v.chars().all(|c| c.is_ascii_alphabetic() || c == '_') && !NOT_VERBS.contains(&v)
}

fn possessive(toks: &[Tok], ctx: &Context) -> Option<Statement> {
    let w = ws(toks);
    let is_pos = w.iter().position(|x| BE.contains(x))?;
    if is_pos + 1 >= w.len() {
        return None;
    }
    let lhs = &toks[..is_pos];
    let mut rhs: &[Tok] = &toks[is_pos + 1..];
    let mut neg = false;
    if rhs.first().map_or(false, |t| t.w == "not") {
        neg = true;
        rhs = &rhs[1..];
    }
    let lw = ws(lhs);
    let (owner, prop): (Term, String) = if lw.len() >= 2 && lw[0] == "my" {
        (Term::sym(&ctx.speaker), lw[1..].join("_"))
    } else if let Some(k) = lw.iter().position(|x| *x == "'s") {
        if k == 0 || k + 1 >= lw.len() {
            return None;
        }
        (entity(&lhs[..k], ctx, false)?, lw[k + 1..].iter().filter(|x| !is_det(x)).cloned().collect::<Vec<_>>().join("_"))
    } else if lw.len() >= 4 && lw[0] == "the" && lw.contains(&"of") {
        let k = lw.iter().position(|x| *x == "of")?;
        (entity(&lhs[k + 1..], ctx, true)?, lw[1..k].join("_"))
    } else {
        return None;
    };
    if prop.is_empty() {
        return None;
    }
    let val = entity(rhs, ctx, false)?;
    Some(Statement { atoms: vec![atom("property", vec![owner, Term::sym(&prop), val], neg)], ..Default::default() })
}

// ------------------------------------------------------------------ questions

/// Yes/no question or declarative-with-question-mark -> atoms, history flag.
fn question_atoms(toks: &[Tok], ctx: &Context) -> Option<(Vec<Atom>, bool)> {
    let w0 = toks.first()?.w.as_str();
    if Q_AUX.contains(&w0) && toks.len() >= 2 {
        let rest = &toks[1..];
        let history = matches!(w0, "did" | "was" | "were");
        let st = if BE.contains(&w0) {
            // "is the car red" -> "the car is red"
            let se = noun_phrase_len(rest);
            if se == 0 || se >= rest.len() {
                return None;
            }
            let mut v: Vec<Tok> = rest[..se].to_vec();
            v.push(tok(w0));
            v.extend_from_slice(&rest[se..]);
            statement(&v, ctx)?
        } else if matches!(w0, "has" | "have") {
            let se = noun_phrase_len(rest);
            if se == 0 || se >= rest.len() {
                return None;
            }
            let mut v: Vec<Tok> = rest[..se].to_vec();
            v.push(tok("has"));
            v.extend_from_slice(&rest[se..]);
            statement(&v, ctx)?
        } else {
            statement(rest, ctx)?
        };
        let history = history || st.past || st.past_tense;
        return Some((st.atoms, history));
    }
    // A statement with a question mark.
    let st = statement(toks, ctx)?;
    Some((st.atoms, st.past || st.past_tense))
}

/// Length of a leading noun phrase: determiners, one word, and following capitalized words.
fn noun_phrase_len(toks: &[Tok]) -> usize {
    let mut i = 0;
    while i < toks.len() && is_det(&toks[i].w) {
        i += 1;
    }
    if i >= toks.len() {
        return 0;
    }
    i += 1;
    while i < toks.len() && toks[i].cap && toks[i - 1].cap {
        i += 1;
    }
    // Two-word common nouns before an adjective: "the ice cream cold" is rare; keep one word.
    i
}

fn wh_question(toks: &[Tok], ctx: &Context) -> Option<(Vec<Atom>, bool)> {
    let w = ws(toks);
    let q = *w.first()?;
    if !QWORDS.contains(&q) || toks.len() < 2 {
        return None;
    }
    let rest = &toks[1..];
    let r = ws(rest);
    let var = Term::var("W");
    match q {
        "where" => {
            // where is X / where does X live / where do I work
            if BE.contains(&r[0]) {
                let e = entity(&rest[1..], ctx, true)?;
                return Some((vec![atom("located_in", vec![e, var], false)], matches!(r[0], "was" | "were")));
            }
            if matches!(r[0], "does" | "do" | "did") && rest.len() >= 3 {
                let history = r[0] == "did";
                let verb = lemma(r[r.len() - 1]);
                let e = entity(&rest[1..rest.len() - 1], ctx, true)?;
                let pred = if verb == "live" { "located_in".to_string() } else { format!("{verb}_in") };
                return Some((vec![atom(&pred, vec![e, var], false)], history));
            }
            None
        }
        "who" | "what" | "which" => {
            // what colour/size is X
            if q == "what" && rest.len() >= 3 && (COLOUR_WORDS.contains(&r[0]) || r[0] == "size") && BE.contains(&r[1]) {
                let p = if r[0] == "size" { "size" } else { "colour" };
                let e = entity(&rest[2..], ctx, true)?;
                return Some((vec![atom("property", vec![e, Term::sym(p), var], false)], matches!(r[1], "was" | "were")));
            }
            if BE.contains(&r[0]) && rest.len() >= 2 {
                let history = matches!(r[0], "was" | "were");
                let tail = &rest[1..];
                // what is the P of X / what is X's P / what is my P
                let mut v: Vec<Tok> = tail.to_vec();
                v.push(tok("is"));
                v.push(tok(VAR_W));
                if let Some(st) = possessive(&v, ctx) {
                    return Some((st.atoms, history));
                }
                // what is in X? -> located_in(W, x)
                if tail[0].w == "in" {
                    let e = entity(&tail[1..], ctx, true)?;
                    return Some((vec![atom("located_in", vec![var, e], false)], history));
                }
                // what/who is X -> is_a(x, W)
                let e = entity(tail, ctx, true)?;
                return Some((vec![atom("is_a", vec![e, var], false)], history));
            }
            if matches!(r[0], "does" | "do" | "did") && rest.len() >= 3 {
                // what does X VERB -> statement "X VERB ?w"
                let mut v: Vec<Tok> = rest[1..].to_vec();
                v.push(tok(VAR_W));
                let st = statement(&v, ctx)?;
                return Some((st.atoms, r[0] == "did" || st.past || st.past_tense));
            }
            // who/what VERBs Y -> statement "?w VERBs Y"
            let mut v = vec![tok(VAR_W)];
            v.extend_from_slice(rest);
            let st = statement(&v, ctx)?;
            Some((st.atoms, st.past || st.past_tense))
        }
        _ => None,
    }
}

// ------------------------------------------------------------------ rules

fn if_then(toks: &[Tok], ctx: &Context) -> Option<Vec<Rule>> {
    let split = toks.iter().position(|t| t.w == "then").or_else(|| toks.iter().position(|t| t.w == ","))?;
    let strip = |v: &[Tok]| -> Vec<Tok> { v.iter().filter(|t| t.w != ",").cloned().collect() };
    let (cond, cons) = (strip(&toks[..split]), strip(&toks[split + 1..]));
    let (cond, cons) = (cond.as_slice(), cons.as_slice());
    IN_RULE.with(|c| c.set(true));
    let result = (|| {
        let mut body = Vec::new();
        for part in cond.split(|t| t.w == "and") {
            if part.is_empty() {
                continue;
            }
            let st = statement(part, ctx)?;
            body.extend(st.atoms.into_iter().map(Literal::Pos));
        }
        let head = statement(cons, ctx)?;
        if body.is_empty() || head.atoms.is_empty() {
            return None;
        }
        Some(head.atoms.into_iter().map(|h| Rule { head: h, body: body.clone() }).collect::<Vec<_>>())
    })();
    IN_RULE.with(|c| c.set(false));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(s: &str) -> String {
        parse(s, &Context::default()).canonical()
    }

    #[test]
    fn spec_examples() {
        let cases = [
            ("Anna likes tea.", "tell: likes(anna, tea)"),
            ("I don't drink coffee anymore", "tell: -drink(maksim, coffee)"),
            ("Anna likes tea and coffee", "tell: likes(anna, tea); likes(anna, coffee)"),
            ("Anna and Bob like tea", "tell: likes(anna, tea); likes(bob, tea)"),
            ("Bob works at Acme", "tell: work_at(bob, acme)"),
            ("Tweety flies", "tell: fly(tweety)"),
            ("I used to like tea.", "tell: likes(maksim, tea) @past"),
            ("Rex is a dog", "tell: is_a(rex, dog)"),
            ("Rex is not a cat", "tell: -is_a(rex, cat)"),
            ("Dogs are animals.", "tell: is_a(dog, animal)"),
            ("The car is red", "tell: property(car, colour, red)"),
            ("Anna is happy", "tell: feels(anna, happy)"),
            ("Anna's job is teacher", "tell: property(anna, job, teacher)"),
            ("My name is Maksim", "tell: property(maksim, name, maksim)"),
            ("I live in New York", "tell: located_in(maksim, new_york)"),
            ("Birds have wings.", "tell: has(bird, wing)"),
            ("Cats like fish.", "tell: likes(A, fish) :- is_a(A, cat)."),
            ("Every cat likes fish.", "tell: likes(A, fish) :- is_a(A, cat)."),
            ("Does Anna like tea?", "ask: likes(anna, tea)"),
            ("What does Anna like?", "ask: likes(anna, W)"),
            ("Who likes tea?", "ask: likes(W, tea)"),
            ("What is a robin?", "ask: is_a(robin, W)"),
            ("Is Rex a dog?", "ask: is_a(rex, dog)"),
            ("Is the car red?", "ask: property(car, colour, red)"),
            ("Is Anna happy?", "ask: feels(anna, happy)"),
            ("Where is Paris?", "ask: located_in(paris, W)"),
            ("Where does Anna live?", "ask: located_in(anna, W)"),
            ("What colour is the car?", "ask: property(car, colour, W)"),
            ("What is the size of the box?", "ask: property(box, size, W)"),
            ("What is Anna's job?", "ask: property(anna, job, W)"),
            ("What does Bob have?", "ask: has(bob, W)"),
            ("Does Tweety fly?", "ask: fly(tweety)"),
            ("Did Anna like tea?", "ask: likes(anna, tea) @history"),
            ("What did I use to drink?", "ask: drink(maksim, W) @history"),
            ("What do you know about Anna?", "recall: anna"),
            ("Tell me about Anna.", "recall: anna"),
            ("Why?", "why: last"),
            ("Why is Rex an animal?", "why: is_a(rex, animal)"),
            ("Why does Anna like tea?", "why: likes(anna, tea)"),
            ("How do you know that Anna likes tea?", "why: likes(anna, tea)"),
            ("No, Rex is a cat.", "correct: is_a(rex, cat)"),
            ("Actually, Rex is a cat.", "correct: is_a(rex, cat)"),
            ("That's wrong.", "correct: none"),
            ("No, Anna doesn't like tea.", "correct: -likes(anna, tea)"),
            ("yes", "yes:"),
            ("nope", "no:"),
            ("Hello!", "greet:"),
            ("goodbye", "bye:"),
            ("asdf", "unknown:"),
            ("What else do you know about birds?", "recall_more: bird"),
            ("Tell me more about Anna.", "recall_more: anna"),
            ("What else?", "recall_more: last"),
            ("Anything else?", "recall_more: last"),
            ("Really?", "social: reaction"),
            ("Oh cool", "social: reaction"),
            ("Thank you!", "social: thanks"),
            ("Who are you?", "social: who_are_you"),
            ("What can you do?", "social: what_can_you_do"),
            ("And you?", "social: and_you"),
            ("I don't know.", "social: dont_know"),
            ("No idea", "social: dont_know"),
            ("If someone is a doctor, then they are a person.", "tell: is_a(A, person) :- is_a(A, doctor)."),
            ("If something is a bird then it flies.", "tell: fly(A) :- is_a(A, bird)."),
            ("If someone likes coffee then they drink coffee.", "tell: drink(A, coffee) :- likes(A, coffee)."),
        ];
        let mut bad = Vec::new();
        for (s, want) in cases {
            let got = p(s);
            if got != want {
                bad.push(format!("{s:?}: got {got:?}, want {want:?}"));
            }
        }
        assert!(bad.is_empty(), "{}", bad.join("\n"));
    }
}

fn capitalize_word(w: &str) -> String {
    let mut c = w.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}
