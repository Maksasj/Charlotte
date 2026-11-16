//! Reply generator: content and intent come in as an [`Act`]; grammar
//! templates realize it as English with aggregation, pronouns and variation.
//!
//! Every asserted clause is generated from an atom, and that atom is listed in
//! [`Reply::claims`]. The agent only passes atoms it believes, so the generator
//! cannot state anything the knowledge base does not hold.

use sla_core::{Atom, GroundAtom, Rule, Term, Value};
use sla_parser::lexicon::VERBS;
use sla_reasoner::{Proof, ProofStep};
use std::cell::Cell;
use std::collections::HashSet;

/// Truth of a yes/no question.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Truth {
    /// True now.
    Yes,
    /// Was true once, not now.
    WasTrue,
    /// Known false (the strong negation holds).
    No,
    Unknown,
}

/// What to say. The agent fills these from the knowledge base.
#[derive(Debug, Clone, PartialEq)]
pub enum Act {
    /// Answer a yes/no question.
    YesNo { atom: GroundAtom, truth: Truth },
    /// Answer a wh-question: `pattern` has the variable `W`; `found` are the matching facts.
    Wh { pattern: Atom, found: Vec<GroundAtom>, past: bool },
    /// Explain a conclusion from its proof.
    Explain(Proof),
    /// Asked why, but the atom is not believed.
    ExplainUnknown(GroundAtom),
    /// Everything known about an entity.
    Recall { entity: String, facts: Vec<GroundAtom> },
    /// Acknowledge a tell: new facts, facts that ended, already-known facts, new rules.
    Acknowledge { added: Vec<GroundAtom>, closed: Vec<GroundAtom>, known: Vec<GroundAtom>, rules: Vec<Rule>, past: bool },
    /// Acknowledge a correction; `rules` are rules that were refined.
    Corrected { added: Vec<GroundAtom>, retracted: Vec<GroundAtom>, rules: Vec<Rule> },
    /// Curiosity: ask whether a fact holds.
    AskYesNo(GroundAtom),
    /// Curiosity: ask for a missing value (pattern has `W`).
    AskWh(Atom),
    /// Curiosity: ask to confirm a hypothesis rule.
    AskRule(Rule),
    /// Curiosity: ask which of two contradicting facts is true.
    AskContradiction(GroundAtom, GroundAtom),
    Greet,
    Bye,
    Thanks,
    NotUnderstood,
    /// Free text from the agent itself (status messages); claims nothing.
    Note(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Reply {
    pub text: String,
    /// The reply's intent: answer, explain, ask, recall, acknowledge, social.
    pub intent: &'static str,
    /// Atoms the reply asserts as currently true (or `past` ones as history).
    pub claims: Vec<GroundAtom>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Person {
    Third,
    You,
    Plural,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tense {
    Present,
    UsedTo,
}

/// A verb phrase split for conjugation and questions.
#[derive(Debug, Clone)]
struct Vp {
    /// "be" or a base verb.
    verb: String,
    /// Words after the verb (object, complement, preposition...).
    rest: String,
}

pub struct Realizer {
    pub speaker: String,
    /// Entities written as capitalized names.
    pub names: HashSet<String>,
    turn: Cell<usize>,
}

fn third(v: &str) -> String {
    match v {
        "be" => "is".into(),
        "have" => "has".into(),
        "do" => "does".into(),
        "go" => "goes".into(),
        _ => {
            if v.ends_with('y') && !v.ends_with("ay") && !v.ends_with("ey") && !v.ends_with("oy") && !v.ends_with("uy") {
                format!("{}ies", &v[..v.len() - 1])
            } else if v.ends_with("ch") || v.ends_with("sh") || v.ends_with('x') || v.ends_with('s') || v.ends_with('z') || v.ends_with('o') {
                format!("{v}es")
            } else {
                format!("{v}s")
            }
        }
    }
}

fn article(np: &str) -> String {
    let first = np.chars().next().unwrap_or('x').to_ascii_lowercase();
    let an = matches!(first, 'a' | 'e' | 'i' | 'o') || (first == 'u' && !np.starts_with("uni") && !np.starts_with("use")) || np.starts_with("hour") || np.starts_with("honest");
    if an {
        format!("an {np}")
    } else {
        format!("a {np}")
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

/// "a", "a and b", "a, b and c"
fn list(items: &[String]) -> String {
    match items.len() {
        0 => String::new(),
        1 => items[0].clone(),
        n => format!("{} and {}", items[..n - 1].join(", "), items[n - 1]),
    }
}

fn is_verb(w: &str) -> bool {
    VERBS.contains(&w)
}


impl Realizer {
    pub fn new(speaker: &str) -> Self {
        Realizer { speaker: speaker.into(), names: HashSet::new(), turn: Cell::new(0) }
    }

    /// Pick one of `n` variants, rotating deterministically.
    fn pick(&self, n: usize) -> usize {
        let t = self.turn.get();
        self.turn.set(t + 1);
        t % n
    }

    fn is_speaker(&self, v: &Value) -> bool {
        v.as_sym() == Some(self.speaker.as_str())
    }

    fn is_person(&self, v: &Value) -> bool {
        self.is_speaker(v) || v.as_sym().map_or(false, |s| self.names.contains(s))
    }

    /// Noun phrase for a value (object position).
    fn np(&self, v: &Value) -> String {
        match v {
            Value::Int(n) => n.to_string(),
            Value::Sym(s) if s == &self.speaker => "you".into(),
            Value::Sym(s) => {
                // "my_family" was said by the user: it is "your family" to them.
                let s = &match s.strip_prefix("my_") {
                    Some(rest) => format!("your_{rest}"),
                    None => s.clone(),
                };
                let words = s.replace('_', " ");
                if self.names.contains(s) {
                    words.split(' ').map(capitalize).collect::<Vec<_>>().join(" ")
                } else {
                    words
                }
            }
        }
    }

    fn np_term(&self, t: &Term) -> String {
        match t {
            Term::Val(v) => self.np(v),
            Term::Var(v) => v.clone(),
        }
    }

    fn person(&self, v: &Value) -> Person {
        if self.is_speaker(v) {
            Person::You
        } else {
            Person::Third
        }
    }

    /// Conjugate: (verb phrase with subject agreement, negation, tense).
    fn conj(&self, vp: &Vp, p: Person, neg: bool, tense: Tense) -> String {
        let rest = if vp.rest.is_empty() { String::new() } else { format!(" {}", vp.rest) };
        let core = match (tense, vp.verb.as_str(), neg) {
            (Tense::UsedTo, v, false) => format!("used to {v}"),
            (Tense::UsedTo, v, true) => format!("did not use to {v}"),
            (Tense::Present, "be", n) => {
                let b = if p == Person::Third { "is" } else { "are" };
                if n {
                    format!("{b} not")
                } else {
                    b.to_string()
                }
            }
            (Tense::Present, v, false) => {
                if p == Person::Third {
                    third(v)
                } else {
                    v.to_string()
                }
            }
            (Tense::Present, v, true) => {
                let d = if p == Person::Third { "does" } else { "do" };
                format!("{d} not {v}")
            }
        };
        format!("{core}{rest}")
    }

    /// Verb phrase parts for an atom whose subject is its first argument.
    /// `None` for atoms realized differently (named properties).
    fn vp_terms(&self, pred: &str, args: &[Term]) -> Option<Vp> {
        let base = pred.trim_start_matches('-');
        let o = |i: usize| self.np_term(&args[i]);
        let be = |rest: String| Some(Vp { verb: "be".into(), rest });
        let verb = |v: &str, rest: String| Some(Vp { verb: v.into(), rest });
        match (base, args.len()) {
            ("likes", 2) => verb("like", o(1)),
            ("has", 2) => verb("have", o(1)),
            ("wants", 2) => verb("want", o(1)),
            ("knows", 2) => verb("know", o(1)),
            ("causes", 2) => verb("cause", o(1)),
            ("says", 2) => verb("say", o(1)),
            ("does", 2) => verb("do", o(1)),
            ("feels", 2) => be(o(1)),
            ("is_a", 2) => be(article(&o(1))),
            ("located_in", 2) => be(format!("in {}", o(1))),
            ("part_of", 2) => be(format!("part of {}", o(1))),
            ("same", 2) => be(format!("the same as {}", o(1))),
            ("before", 2) => verb("come", format!("before {}", o(1))),
            ("during", 2) => verb("happen", format!("during {}", o(1))),
            ("agent_of", 2) => be(format!("the agent of {}", o(1))),
            ("property", 3) => {
                let p = self.np_term(&args[1]);
                if matches!(p.as_str(), "colour" | "size" | "quality") {
                    be(o(2))
                } else {
                    None
                }
            }
            ("thing" | "person" | "place" | "event", 1) => be(article(base)),
            (name, n) if (1..=2).contains(&n) => {
                let words: Vec<&str> = name.split('_').collect();
                let obj = if n == 2 { format!(" {}", o(1)) } else { String::new() };
                let v0 = sla_parser::lemma(words[0]);
                if is_verb(&v0) {
                    let mid = words[1..].join(" ");
                    let rest = format!("{mid}{obj}").trim().to_string();
                    verb(&v0, rest)
                } else if n == 1 {
                    be(article(&words.join(" ")))
                } else {
                    be(format!("{} of{obj}", article(&words.join(" "))))
                }
            }
            _ => None,
        }
    }

    fn vp(&self, a: &GroundAtom) -> Option<Vp> {
        let args: Vec<Term> = a.args.iter().cloned().map(Term::Val).collect();
        self.vp_terms(&a.pred, &args)
    }

    /// Subject noun phrase (sentence-internal; capitalized later).
    fn subject(&self, v: &Value) -> String {
        self.np(v)
    }

    /// A declarative clause (no final punctuation), e.g. "Anna likes tea".
    pub fn clause(&self, a: &GroundAtom) -> String {
        self.clause_t(a, Tense::Present)
    }

    fn clause_t(&self, a: &GroundAtom, tense: Tense) -> String {
        let neg = a.is_strong_negation();
        if a.base_pred() == "property" && a.args.len() == 3 {
            if self.vp(a).is_none() {
                let owner = &a.args[0];
                let poss = if self.is_speaker(owner) { "your".to_string() } else { format!("{}'s", self.np(owner)) };
                let vp = Vp { verb: "be".into(), rest: self.np(&a.args[2]) };
                return format!("{poss} {} {}", self.np(&a.args[1]), self.conj(&vp, Person::Third, neg, tense));
            }
        }
        if a.args.is_empty() {
            let words = a.base_pred().replace('_', " ");
            return if neg { format!("it is not the case that {words}") } else { format!("{words} holds") };
        }
        match self.vp(a) {
            Some(vp) => format!("{} {}", self.subject(&a.args[0]), self.conj(&vp, self.person(&a.args[0]), neg, tense)),
            None => {
                // Arity 3+ relations: list the arguments.
                let args: Vec<String> = a.args.iter().map(|v| self.np(v)).collect();
                let words = a.base_pred().replace('_', " ");
                if neg {
                    format!("the relation {words} does not hold for {}", list(&args))
                } else {
                    format!("the relation {words} holds for {}", list(&args))
                }
            }
        }
    }

    /// Clause with a pronoun subject: "it is a dog", "they like tea", "you like tea".
    fn clause_pronoun(&self, a: &GroundAtom) -> String {
        let s = &a.args[0];
        if self.is_speaker(s) {
            return self.clause(a);
        }
        let (pron, poss, p) = if self.is_person(s) { ("they", "their", Person::Plural) } else { ("it", "its", Person::Third) };
        let neg = a.is_strong_negation();
        match self.vp(a) {
            Some(vp) => format!("{pron} {}", self.conj(&vp, p, neg, Tense::Present)),
            None if a.base_pred() == "property" && a.args.len() == 3 => {
                let vp = Vp { verb: "be".into(), rest: self.np(&a.args[2]) };
                format!("{poss} {} {}", self.np(&a.args[1]), self.conj(&vp, Person::Third, neg, Tense::Present))
            }
            None => self.clause(a),
        }
    }

    /// Aggregate atoms into sentences: same subject and verb -> joined objects;
    /// same subject -> joined verb phrases; repeated subjects become pronouns.
    fn aggregate(&self, atoms: &[GroundAtom], tense: Tense) -> String {
        // Group by subject, keep order.
        let mut subjects: Vec<Value> = Vec::new();
        for a in atoms {
            if let Some(s) = a.args.first() {
                if !subjects.contains(s) {
                    subjects.push(s.clone());
                }
            }
        }
        let mut sentences = Vec::new();
        for s in &subjects {
            let mine: Vec<&GroundAtom> = atoms.iter().filter(|a| a.args.first() == Some(s)).collect();
            // Merge same predicate (and same middle args) objects.
            let mut parts: Vec<String> = Vec::new();
            let mut done = vec![false; mine.len()];
            for i in 0..mine.len() {
                if done[i] {
                    continue;
                }
                let a = mine[i];
                let vp = self.vp(a);
                let mergeable = vp.is_some() && a.args.len() == 2;
                let mut objs = vec![a.args.get(1).cloned()];
                if mergeable {
                    for j in i + 1..mine.len() {
                        if !done[j] && mine[j].pred == a.pred && mine[j].args.len() == 2 {
                            objs.push(mine[j].args.get(1).cloned());
                            done[j] = true;
                        }
                    }
                }
                done[i] = true;
                if mergeable && objs.len() > 1 {
                    // Realize the verb phrase with a placeholder object, then list the objects.
                    let ph = GroundAtom { pred: a.pred.clone(), args: vec![a.args[0].clone(), Value::sym("\u{1}")] };
                    let full = self.clause_t(&ph, tense);
                    let subj = self.subject(&a.args[0]);
                    let vp_text = full[subj.len()..].trim_start().to_string();
                    let objs: Vec<String> = objs.iter().flatten().map(|o| self.np(o)).collect();
                    // Articles ("a dog and a cat") belong to each object.
                    let joined = if vp_text.contains("a \u{1}") || vp_text.contains("an \u{1}") {
                        let items: Vec<String> = objs.iter().map(|o| article(o)).collect();
                        vp_text.replace("an \u{1}", "\u{1}").replace("a \u{1}", "\u{1}").replace('\u{1}', &list(&items))
                    } else {
                        vp_text.replace('\u{1}', &list(&objs))
                    };
                    parts.push(joined);
                } else {
                    let full = self.clause_t(a, tense);
                    let subj = self.subject(&a.args[0]);
                    if a.base_pred() == "property" && full.contains("'s ") || full.starts_with("your ") {
                        // Possessive clauses keep their own subject.
                        parts.push(format!("\u{2}{full}"));
                    } else {
                        parts.push(full[subj.len()..].trim_start().to_string());
                    }
                }
            }
            let (own, plain): (Vec<String>, Vec<String>) = parts.into_iter().partition(|p| p.starts_with('\u{2}'));
            if !plain.is_empty() {
                sentences.push(format!("{} {}", self.subject(s), list(&plain)));
            }
            for p in own {
                sentences.push(p.trim_start_matches('\u{2}').to_string());
            }
        }
        sentences.iter().map(|s| format!("{}.", capitalize(s))).collect::<Vec<_>>().join(" ")
    }

    // ---------------------------------------------------------------- questions

    /// "Does Anna like tea?", "Is Rex a dog?", "Is your job teacher?"
    pub fn yes_no_question(&self, a: &GroundAtom) -> String {
        if a.base_pred() == "property" && a.args.len() == 3 && self.vp(a).is_none() {
            let owner = &a.args[0];
            let poss = if self.is_speaker(owner) { "your".to_string() } else { format!("{}'s", self.np(owner)) };
            return format!("Is {poss} {} {}?", self.np(&a.args[1]), self.np(&a.args[2]));
        }
        let neg = a.is_strong_negation();
        match (a.args.first(), self.vp(a)) {
            (Some(s), Some(vp)) => {
                let p = self.person(s);
                let subj = self.subject(s);
                let not = if neg { " not" } else { "" };
                let rest = if vp.rest.is_empty() { String::new() } else { format!(" {}", vp.rest) };
                if vp.verb == "be" {
                    let b = if p == Person::Third { "Is" } else { "Are" };
                    format!("{b} {subj}{not}{rest}?")
                } else {
                    let d = if p == Person::Third { "Does" } else { "Do" };
                    format!("{d} {subj}{not} {}{rest}?", vp.verb)
                }
            }
            _ => format!("Is it true that {}?", self.clause(a)),
        }
    }

    /// Wh-question for a pattern with `W`, e.g. "What does Anna like?".
    pub fn wh_question(&self, pat: &Atom) -> String {
        let (head, body) = self.wh_parts(pat);
        capitalize(&format!("{head}{body}?"))
    }

    /// ("what", " does Anna like") style split; also used for "I don't know what Anna likes".
    fn wh_parts(&self, pat: &Atom) -> (String, String) {
        let w_at = pat.args.iter().position(|t| matches!(t, Term::Var(v) if v == "W"));
        let base = pat.pred.trim_start_matches('-');
        let subj = |i: usize| self.np_term(&pat.args[i]);
        let is_person_term = |t: &Term| matches!(t, Term::Val(v) if self.is_person(v));
        let be_of = |t: &Term| if let Term::Val(v) = t { if self.is_speaker(v) { "are" } else { "is" } } else { "is" };
        match (base, w_at) {
            ("located_in", Some(1)) => ("where".into(), format!(" {} {}", be_of(&pat.args[0]), subj(0))),
            ("is_a", Some(1)) => ("what".into(), format!(" {} {}", be_of(&pat.args[0]), subj(0))),
            ("feels", Some(1)) => (format!("how {} {}", be_of(&pat.args[0]), subj(0)), " feeling".into()),
            ("property", Some(2)) => {
                let p = self.np_term(&pat.args[1]);
                if p == "colour" || p == "size" {
                    (format!("what {p}"), format!(" {} {}", be_of(&pat.args[0]), subj(0)))
                } else {
                    let owner = if matches!(&pat.args[0], Term::Val(v) if self.is_speaker(v)) { "your".to_string() } else { format!("{}'s", subj(0)) };
                    ("what".into(), format!(" is {owner} {p}"))
                }
            }
            (_, Some(0)) => {
                // Subject question: "who likes tea", "what causes headaches".
                let who = if matches!(base, "likes" | "knows" | "wants" | "feels") { "who" } else { "what" };
                let args: Vec<Term> = pat.args.clone();
                match self.vp_terms(base, &args) {
                    Some(vp) => {
                        let neg = pat.pred.starts_with('-');
                        (who.into(), format!(" {}", self.conj(&vp, Person::Third, neg, Tense::Present)))
                    }
                    None => ("what".into(), format!(" holds for {}", base.replace('_', " "))),
                }
            }
            (_, Some(1)) => {
                let who = if matches!(base, "knows") || pat.args.get(1).map_or(false, is_person_term) { "who" } else { "what" };
                // Realize with an empty object and do-support.
                let mut args = pat.args.clone();
                args[1] = Term::Val(Value::sym("\u{1}"));
                match self.vp_terms(base, &args) {
                    Some(vp) if vp.verb != "be" => {
                        let p = if let Term::Val(v) = &pat.args[0] { self.person(v) } else { Person::Third };
                        let d = if p == Person::Third { "does" } else { "do" };
                        let rest = vp.rest.replace('\u{1}', "").trim().to_string();
                        let rest = if rest.is_empty() { String::new() } else { format!(" {rest}") };
                        (who.into(), format!(" {d} {} {}{rest}", subj(0), vp.verb))
                    }
                    Some(vp) => {
                        let rest = vp.rest.replace("an \u{1}", "").replace("a \u{1}", "").replace('\u{1}', "").trim().to_string();
                        let rest = if rest.is_empty() { String::new() } else { format!(" {rest}") };
                        (who.into(), format!(" {} {}{rest}", be_of(&pat.args[0]), subj(0)))
                    }
                    None => ("what".into(), format!(" is related to {} by {}", subj(0), base.replace('_', " "))),
                }
            }
            _ => ("what".into(), format!(" holds for {}", base.replace('_', " "))),
        }
    }

    /// "what Anna likes", "where Paris is" (embedded question word order).
    fn embedded_wh(&self, pat: &Atom) -> String {
        let (head, body) = self.wh_parts(pat);
        let words: Vec<&str> = body.split_whitespace().collect();
        if words.is_empty() {
            return head;
        }
        // Subject questions keep their order.
        if matches!(pat.args.iter().position(|t| matches!(t, Term::Var(v) if v == "W")), Some(0)) {
            return format!("{head}{body}");
        }
        match words[0] {
            "does" | "do" => {
                // " does Anna like" -> " Anna likes"
                let p = if words[0] == "does" { Person::Third } else { Person::You };
                let subj_len = self.np_term(&pat.args[0]).split_whitespace().count();
                let subj = words[1..1 + subj_len].join(" ");
                let verb = words.get(1 + subj_len).copied().unwrap_or("");
                let rest = words[(2 + subj_len).min(words.len())..].join(" ");
                let v = self.conj(&Vp { verb: verb.into(), rest }, p, false, Tense::Present);
                format!("{head} {subj} {v}")
            }
            "is" | "are" => {
                let rest = words[1..].join(" ");
                format!("{head} {rest} {}", words[0])
            }
            _ => format!("{head}{body}"),
        }
    }

    /// A rule in words: "if A is a bird then A flies".
    pub fn rule_text(&self, r: &Rule) -> String {
        let lit = |l: &sla_core::Literal| -> String {
            match l {
                sla_core::Literal::Pos(a) => self.clause_terms(a),
                sla_core::Literal::Neg(a) => format!("it is not known that {}", self.clause_terms(a)),
                sla_core::Literal::Cmp(op, x, y) => format!("{} {} {}", self.np_term(x), cmp_words(*op), self.np_term(y)),
            }
        };
        let body: Vec<String> = r.body.iter().map(lit).collect();
        format!("if {} then {}", list(&body), self.clause_terms(&r.head))
    }

    /// Clause for an atom that may contain variables (rule text).
    fn clause_terms(&self, a: &Atom) -> String {
        if let Some(g) = a.to_ground() {
            return self.clause(&g);
        }
        let neg = a.pred.starts_with('-');
        if a.pred.trim_start_matches('-') == "property" && a.args.len() == 3 {
            let p = self.np_term(&a.args[1]);
            if !matches!(p.as_str(), "colour" | "size" | "quality") {
                let not = if neg { " not" } else { "" };
                return format!("the {p} of {} is{not} {}", self.np_term(&a.args[0]), self.np_term(&a.args[2]));
            }
        }
        match (a.args.first(), self.vp_terms(&a.pred, &a.args)) {
            (Some(s), Some(vp)) => {
                let p = match s {
                    Term::Val(v) => self.person(v),
                    Term::Var(_) => Person::Third,
                };
                format!("{} {}", self.np_term(s), self.conj(&vp, p, neg, Tense::Present))
            }
            _ => format!("{} holds", a),
        }
    }

    fn source_phrase(&self, source: &str) -> String {
        let s = source.to_lowercase();
        if s == "user" || s == self.speaker || s.starts_with("user") {
            ["you told me", "you said so", "you told me so"][self.pick(3)].into()
        } else if s.starts_with("teacher") {
            "my teacher told me".into()
        } else if matches!(s.as_str(), "builtin" | "program" | "core" | "given") {
            "I knew it from the start".into()
        } else if s.starts_with("learner") || s.starts_with("induction") {
            "I worked it out from examples".into()
        } else {
            format!("{} told me", self.np(&Value::sym(source)))
        }
    }

    /// "you told me" style intro for "<intro> that <clauses>".
    fn source_intro(&self, source: &str) -> String {
        let s = source.to_lowercase();
        if s == "user" || s == self.speaker || s.starts_with("user") {
            "you told me".into()
        } else if s.starts_with("teacher") {
            "my teacher told me".into()
        } else if matches!(s.as_str(), "builtin" | "program" | "core" | "given") {
            "I have always known".into()
        } else if s.starts_with("learner") || s.starts_with("induction") {
            "I worked out".into()
        } else {
            format!("{} told me", self.np(&Value::sym(source)))
        }
    }

    fn explain(&self, p: &Proof, depth: usize, out: &mut Vec<String>, claims: &mut Vec<GroundAtom>) {
        claims.push(p.atom.clone());
        let head = capitalize(&self.clause(&p.atom));
        match &p.step {
            ProofStep::Given { source } => out.push(format!("{head} because {}.", self.source_phrase(source))),
            ProofStep::Rule { rule, absent, .. } => {
                // Same subject as the conclusion: use a pronoun ("because it is a dog").
                let subj = p.atom.args.first();
                let mut reasons: Vec<String> = p
                    .children
                    .iter()
                    .map(|c| if c.atom.args.first() == subj && subj.is_some() { self.clause_pronoun(&c.atom) } else { self.clause(&c.atom) })
                    .collect();
                for a in absent {
                    if let Ok(g) = sla_core::parse_ground(a) {
                        reasons.push(format!("nothing tells me that {}", self.clause(&g)));
                    }
                }
                claims.extend(p.children.iter().map(|c| c.atom.clone()));
                let rule = self.rule_text(rule);
                if reasons.is_empty() {
                    out.push(format!("{head}, because {rule}."));
                } else {
                    out.push(format!("{head} because {}, and {rule}.", list(&reasons)));
                }
                // Where the given premises came from.
                let mut by_source: Vec<(String, Vec<String>)> = Vec::new();
                for c in &p.children {
                    if let ProofStep::Given { source } = &c.step {
                        let intro = self.source_intro(source);
                        let clause = self.clause(&c.atom);
                        match by_source.iter_mut().find(|(i, _)| *i == intro) {
                            Some((_, v)) => v.push(clause),
                            None => by_source.push((intro, vec![clause])),
                        }
                    }
                }
                for (intro, clauses) in by_source {
                    out.push(format!("{} that {}.", capitalize(&intro), list(&clauses)));
                }
                if depth > 1 {
                    for c in &p.children {
                        if matches!(c.step, ProofStep::Rule { .. }) {
                            self.explain(c, depth - 1, out, claims);
                        }
                    }
                }
            }
        }
    }

    // ---------------------------------------------------------------- acts

    pub fn realize(&self, act: &Act) -> Reply {
        let mut claims = Vec::new();
        let (intent, text) = match act {
            Act::YesNo { atom, truth } => {
                let t = match truth {
                    Truth::Yes => {
                        claims.push(atom.clone());
                        let c = self.clause(atom);
                        match self.pick(3) {
                            0 => format!("Yes, {c}."),
                            1 => format!("Yes. {}.", capitalize(&c)),
                            _ => format!("That's right: {c}."),
                        }
                    }
                    Truth::WasTrue => {
                        claims.push(atom.clone());
                        let c = self.clause_t(atom, Tense::UsedTo);
                        format!("Yes, {c}, but not anymore.")
                    }
                    Truth::No => {
                        let neg = atom.complement();
                        claims.push(neg.clone());
                        format!("No, {}.", self.clause(&neg))
                    }
                    Truth::Unknown => {
                        let q = self.clause(atom);
                        ["I don't know whether {q}.", "I'm not sure whether {q}.", "I have no idea whether {q}."][self.pick(3)].replace("{q}", &q)
                    }
                };
                ("answer", t)
            }
            Act::Wh { pattern, found, past } => {
                if found.is_empty() {
                    let e = self.embedded_wh(pattern);
                    ("answer", ["I don't know {e}.", "I don't know {e} yet.", "Nobody has told me {e}, so I don't know."][self.pick(3)].replace("{e}", &e))
                } else {
                    claims.extend(found.iter().cloned());
                    let tense = if *past { Tense::UsedTo } else { Tense::Present };
                    ("answer", self.aggregate(found, tense))
                }
            }
            Act::Explain(proof) => {
                let mut out = Vec::new();
                self.explain(proof, 3, &mut out, &mut claims);
                ("explain", out.join(" "))
            }
            Act::ExplainUnknown(a) => ("explain", format!("I don't believe that {}, so I can't say why.", self.clause(a))),
            Act::Recall { entity, facts } => {
                let name = self.np(&Value::sym(entity.as_str()));
                if facts.is_empty() {
                    ("recall", format!("I don't know anything about {name} yet."))
                } else {
                    claims.extend(facts.iter().cloned());
                    let lead = ["Here is what I know about {n}:", "About {n}:", "This is what I know about {n}:"][self.pick(3)].replace("{n}", &name);
                    ("recall", format!("{lead} {}", self.aggregate(facts, Tense::Present)))
                }
            }
            Act::Acknowledge { added, closed, known, rules, past } => {
                let mut parts = Vec::new();
                if !added.is_empty() {
                    claims.extend(added.iter().cloned());
                    let tense = if *past { Tense::UsedTo } else { Tense::Present };
                    let body = self.aggregate(added, tense);
                    let lead = ["Okay, I'll remember that.", "Got it.", "Noted."][self.pick(3)];
                    parts.push(format!("{lead} {body}"));
                }
                if !known.is_empty() {
                    claims.extend(known.iter().cloned());
                    parts.push(format!("I already knew that. {}", self.aggregate(known, Tense::Present)));
                }
                if !closed.is_empty() {
                    let ended: Vec<String> = closed.iter().map(|c| self.clause_t(c, Tense::UsedTo)).collect();
                    parts.push(format!("So {} before, but not anymore.", list(&ended)));
                }
                for r in rules {
                    parts.push(format!("I'll keep this as a rule: {}.", self.rule_text(r)));
                }
                if parts.is_empty() {
                    parts.push("Okay.".into());
                }
                ("acknowledge", parts.join(" "))
            }
            Act::Corrected { added, retracted, rules } => {
                let mut parts = vec!["Thanks for the correction.".to_string()];
                if !retracted.is_empty() {
                    let r: Vec<String> = retracted.iter().map(|c| self.clause(c)).collect();
                    parts.push(format!("I no longer believe that {}.", list(&r)));
                }
                if !added.is_empty() {
                    claims.extend(added.iter().cloned());
                    parts.push(format!("Now I know: {}", self.aggregate(added, Tense::Present)));
                }
                for r in rules {
                    parts.push(format!("I changed my rule to: {}.", self.rule_text(r)));
                }
                ("acknowledge", parts.join(" "))
            }
            Act::AskYesNo(a) => ("ask", self.yes_no_question(a)),
            Act::AskWh(p) => ("ask", self.wh_question(p)),
            Act::AskRule(r) => ("ask", format!("I noticed a pattern. Is it true that {}?", self.rule_text(r))),
            Act::AskContradiction(a, b) => (
                "ask",
                format!("I have conflicting information: {}, but also {}. Which one is true?", self.clause(a), self.clause(b)),
            ),
            Act::Greet => ("social", ["Hello!", "Hi there!", "Hello, nice to talk to you."][self.pick(3)].into()),
            Act::Bye => ("social", ["Goodbye!", "Bye, talk to you later.", "See you!"][self.pick(3)].into()),
            Act::Thanks => ("social", "Thanks!".into()),
            Act::NotUnderstood => (
                "social",
                ["Sorry, I didn't understand that.", "I'm not sure what you mean.", "Could you say that more simply?"][self.pick(3)].into(),
            ),
            Act::Note(t) => ("social", t.clone()),
        };
        Reply { text, intent, claims }
    }

}

fn cmp_words(op: sla_core::CmpOp) -> &'static str {
    use sla_core::CmpOp::*;
    match op {
        Eq => "is",
        Ne => "is not",
        Lt => "is less than",
        Le => "is at most",
        Gt => "is more than",
        Ge => "is at least",
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use sla_core::parse_ground;

    fn r() -> Realizer {
        let mut r = Realizer::new("maksim");
        r.names.insert("anna".into());
        r.names.insert("bob".into());
        r.names.insert("new_york".into());
        r
    }

    fn g(s: &str) -> GroundAtom {
        parse_ground(s).unwrap()
    }

    #[test]
    fn clauses() {
        let r = r();
        for (a, want) in [
            ("likes(anna, tea)", "Anna likes tea"),
            ("-likes(anna, tea)", "Anna does not like tea"),
            ("likes(maksim, coffee)", "you like coffee"),
            ("-likes(maksim, coffee)", "you do not like coffee"),
            ("is_a(rex, dog)", "rex is a dog"),
            ("is_a(rex, animal)", "rex is an animal"),
            ("feels(maksim, happy)", "you are happy"),
            ("property(car, colour, red)", "car is red"),
            ("property(anna, job, teacher)", "Anna's job is teacher"),
            ("property(maksim, name, maksim)", "your name is you"),
            ("located_in(anna, new_york)", "Anna is in New York"),
            ("fly(tweety)", "tweety flies"),
            ("work_at(bob, acme)", "Bob works at acme"),
            ("drink(anna, tea)", "Anna drinks tea"),
            ("ancestor(anna, bob)", "Anna is an ancestor of Bob"),
            ("watch(anna, film)", "Anna watches film"),
        ] {
            assert_eq!(r.clause(&g(a)), want, "{a}");
        }
    }

    #[test]
    fn questions() {
        let r = r();
        assert_eq!(r.yes_no_question(&g("likes(anna, tea)")), "Does Anna like tea?");
        assert_eq!(r.yes_no_question(&g("is_a(rex, dog)")), "Is rex a dog?");
        assert_eq!(r.yes_no_question(&g("likes(maksim, tea)")), "Do you like tea?");
        let pat = |s: &str| sla_core::parse_atom(s).unwrap();
        assert_eq!(r.wh_question(&pat("likes(anna, W)")), "What does Anna like?");
        assert_eq!(r.wh_question(&pat("likes(W, tea)")), "Who likes tea?");
        assert_eq!(r.wh_question(&pat("located_in(anna, W)")), "Where is Anna?");
        assert_eq!(r.wh_question(&pat("is_a(rex, W)")), "What is rex?");
        assert_eq!(r.wh_question(&pat("property(car, colour, W)")), "What colour is car?");
        assert_eq!(r.wh_question(&pat("property(anna, job, W)")), "What is Anna's job?");
        assert_eq!(r.wh_question(&pat("work_at(bob, W)")), "What does Bob work at?");
        assert_eq!(r.embedded_wh(&pat("likes(anna, W)")), "what Anna likes");
        assert_eq!(r.embedded_wh(&pat("located_in(anna, W)")), "where Anna is");
        assert_eq!(r.embedded_wh(&pat("likes(W, tea)")), "who likes tea");
    }

    #[test]
    fn aggregation() {
        let r = r();
        let facts = vec![g("likes(anna, tea)"), g("likes(anna, coffee)"), g("feels(anna, happy)"), g("is_a(anna, teacher)")];
        assert_eq!(r.aggregate(&facts, Tense::Present), "Anna likes tea and coffee, is happy and is a teacher.");
        let facts = vec![g("is_a(rex, dog)"), g("is_a(rex, pet)")];
        assert_eq!(r.aggregate(&facts, Tense::Present), "Rex is a dog and a pet.");
    }
}
