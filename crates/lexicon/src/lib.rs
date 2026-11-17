//! The predicate registry and word lists (Design 2.0, "Closed vocabulary,
//! one door").
//!
//! Every predicate the agent may store or ask about is in `vault/lexicon/`
//! with its argument types, a definition, sentence frames, synonyms and
//! flags. The flags replace hand lists that were spread over the v1 crates.
//! [`Registry::check`] types a fact: an unknown predicate goes to the review
//! queue instead of being stored, and ill-typed arguments are rejected:
//! `is_a(here, first_fact)`, `has(day, twenty_four_hour)`,
//! `property(pan, quality, good_for_eggs_and_frying)`.

use sla_core::{GroundAtom, RuleFile, Term, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ArgType {
    /// A thing: a plain word or short compound, never a function word, a
    /// spelled number or a glued phrase.
    Entity,
    Category,
    /// An event id (`e7`) or an entity.
    Event,
    Task,
    Step,
    Number,
    /// A unit from the unit list (minute, gram, piece, ...).
    Unit,
    /// A fact id (`f12`).
    Fact,
    /// A quoted string.
    Text,
    /// A plain identifier (an action, a property name, a state).
    Symbol,
    /// An entity-like word or a number (property values).
    Value,
    Any,
}

impl ArgType {
    pub fn parse(s: &str) -> Option<ArgType> {
        Some(match s {
            "entity" => ArgType::Entity,
            "category" => ArgType::Category,
            "event" => ArgType::Event,
            "task" => ArgType::Task,
            "step" => ArgType::Step,
            "number" => ArgType::Number,
            "unit" => ArgType::Unit,
            "fact" => ArgType::Fact,
            "text" => ArgType::Text,
            "symbol" => ArgType::Symbol,
            "value" => ArgType::Value,
            "any" => ArgType::Any,
            _ => return None,
        })
    }

    pub fn name(self) -> &'static str {
        match self {
            ArgType::Entity => "entity",
            ArgType::Category => "category",
            ArgType::Event => "event",
            ArgType::Task => "task",
            ArgType::Step => "step",
            ArgType::Number => "number",
            ArgType::Unit => "unit",
            ArgType::Fact => "fact",
            ArgType::Text => "text",
            ArgType::Symbol => "symbol",
            ArgType::Value => "value",
            ArgType::Any => "any",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PredInfo {
    pub name: String,
    pub types: Vec<ArgType>,
    pub definition: String,
    pub frames: Vec<String>,
    pub flags: BTreeSet<String>,
    /// Verbs from the parser's lexicon: arity 1 or 2, an entity first.
    pub verb: bool,
}

impl PredInfo {
    pub fn has_flag(&self, f: &str) -> bool {
        self.flags.contains(f)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeError {
    /// Not in the registry: goes to the review queue, facts using it are pending.
    UnknownPredicate(String),
    Arity { pred: String, want: String, got: usize },
    BadArgument { pred: String, pos: usize, want: ArgType, value: String, why: String },
    Forbidden { pred: String, pos: usize, value: String, why: String },
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeError::UnknownPredicate(p) => write!(f, "unknown predicate {p} (it needs review before facts can use it)"),
            TypeError::Arity { pred, want, got } => write!(f, "{pred} takes {want} argument(s), not {got}"),
            TypeError::BadArgument { pred, pos, want, value, why } => write!(f, "argument {pos} of {pred} must be a {}: `{value}` {why}", want.name()),
            TypeError::Forbidden { pred, pos, value, why } => write!(f, "`{value}` is not allowed as argument {pos} of {pred}: {why}"),
        }
    }
}

impl std::error::Error for TypeError {}

#[derive(Debug, Clone, Default)]
pub struct Registry {
    pub preds: BTreeMap<String, PredInfo>,
    /// synonym -> registered name
    pub synonyms: BTreeMap<String, String>,
    /// unit -> (dimension, size in the dimension's base unit)
    pub units: BTreeMap<String, (String, Value)>,
    pub function_words: BTreeSet<String>,
    pub number_words: BTreeSet<String>,
    pub glue_words: BTreeSet<String>,
    /// (pred, 1-based position, value) -> reason
    pub forbid: BTreeMap<(String, usize, String), String>,
    /// Lines of the registry it could not read: (statement, problem).
    pub errors: Vec<(String, String)>,
}

fn sym(t: &Term) -> Option<&str> {
    match t {
        Term::Val(Value::Sym(s)) => Some(s),
        _ => None,
    }
}

impl Registry {
    /// Read registry files (rule-file format with fact statements).
    pub fn from_files<'a>(files: impl IntoIterator<Item = &'a RuleFile>) -> Registry {
        let mut r = Registry::default();
        for rf in files {
            for rule in rf.doc.rules() {
                if !rule.body.is_empty() {
                    r.errors.push((rule.to_string(), "the registry holds facts only".into()));
                    continue;
                }
                let a = &rule.head.args;
                let arg = |i: usize| a.get(i).and_then(sym);
                match (rule.head.pred.as_str(), a.len()) {
                    ("pred", 3) => {
                        let (Some(name), Some(types), Some(def)) = (arg(0), arg(1), arg(2)) else {
                            r.errors.push((rule.to_string(), "pred(Name, \"types\", \"definition\")".into()));
                            continue;
                        };
                        let parsed: Option<Vec<ArgType>> = types.split_whitespace().map(ArgType::parse).collect();
                        let Some(types) = parsed else {
                            r.errors.push((rule.to_string(), "unknown argument type".into()));
                            continue;
                        };
                        let e = r.preds.entry(name.to_string()).or_default();
                        e.name = name.to_string();
                        e.types = types;
                        e.definition = def.to_string();
                    }
                    ("frame", 2) => {
                        if let (Some(p), Some(f)) = (arg(0), arg(1)) {
                            r.preds.entry(p.to_string()).or_default().frames.push(f.to_string());
                        }
                    }
                    ("flag", 2) => {
                        if let (Some(p), Some(f)) = (arg(0), arg(1)) {
                            r.preds.entry(p.to_string()).or_default().flags.insert(f.to_string());
                        }
                    }
                    ("synonym", 2) => {
                        if let (Some(s), Some(p)) = (arg(0), arg(1)) {
                            r.synonyms.insert(s.to_string(), p.to_string());
                        }
                    }
                    ("verb", 1) => {
                        if let Some(v) = arg(0) {
                            let e = r.preds.entry(v.to_string()).or_default();
                            if e.name.is_empty() {
                                e.name = v.to_string();
                                e.verb = true;
                                e.definition = format!("X {v}s (Y)");
                            }
                        }
                    }
                    ("unit", 3) => {
                        if let (Some(u), Some(d), Some(Term::Val(n))) = (arg(0), arg(1), a.get(2)) {
                            r.units.insert(u.to_string(), (d.to_string(), n.clone()));
                        }
                    }
                    ("function_word", 1) => r.function_words.extend(arg(0).map(String::from)),
                    ("number_word", 1) => r.number_words.extend(arg(0).map(String::from)),
                    ("glue_word", 1) => r.glue_words.extend(arg(0).map(String::from)),
                    ("forbid", 4) => {
                        if let (Some(p), Some(Term::Val(Value::Int(pos))), Some(v), Some(why)) = (arg(0), a.get(1), arg(2), arg(3)) {
                            r.forbid.insert((p.to_string(), *pos as usize, v.to_string()), why.to_string());
                        }
                    }
                    _ => r.errors.push((rule.to_string(), "not a registry statement".into())),
                }
            }
        }
        // A frame or flag for a name that has no `pred` or `verb` entry is an error, not a predicate.
        let orphans: Vec<String> = r.preds.iter().filter(|(_, p)| p.name.is_empty()).map(|(k, _)| k.clone()).collect();
        for k in orphans {
            r.preds.remove(&k);
            r.errors.push((k, "frame or flag without a pred(...) entry".into()));
        }
        r
    }

    /// The built-in registry (as seeded into new vaults).
    pub fn builtin() -> Registry {
        let files: Vec<RuleFile> = sla_core::default_lexicon().into_iter().map(|(_, rf)| rf).collect();
        Registry::from_files(files.iter())
    }

    /// The registry of a vault.
    pub fn from_vault(v: &sla_core::Vault) -> Registry {
        Registry::from_files(v.lexicon.values())
    }

    /// The registered name for `name` (itself, or what it is a synonym of).
    pub fn resolve<'a>(&'a self, name: &'a str) -> Option<&'a str> {
        let base = name.trim_start_matches('-');
        if self.preds.contains_key(base) {
            return Some(self.preds.get_key_value(base).map(|(k, _)| k.as_str()).unwrap_or(base));
        }
        self.synonyms.get(base).map(String::as_str)
    }

    pub fn get(&self, name: &str) -> Option<&PredInfo> {
        self.resolve(name).and_then(|n| self.preds.get(n))
    }

    pub fn is_unit(&self, s: &str) -> bool {
        self.units.contains_key(s)
    }

    /// Why `s` cannot name a thing, if it cannot.
    pub fn entity_problem(&self, s: &str) -> Option<String> {
        if s.is_empty() {
            return Some("is empty".into());
        }
        if self.function_words.contains(s) {
            return Some("is a function word, not a thing".into());
        }
        let parts: Vec<&str> = s.split('_').filter(|p| !p.is_empty()).collect();
        if parts.iter().any(|p| self.number_words.contains(*p)) {
            return Some("contains a number written as a word; numbers are values".into());
        }
        if parts.len() >= 2 && parts.iter().any(|p| self.function_words.contains(*p) || self.glue_words.contains(*p)) {
            return Some("glues a phrase into one name; split it into facts".into());
        }
        if parts.len() >= 4 {
            return Some("is too long for one name; split it into facts".into());
        }
        None
    }

    fn check_arg(&self, pred: &str, pos: usize, want: ArgType, v: &Value) -> Result<(), TypeError> {
        let bad = |why: &str| TypeError::BadArgument { pred: pred.to_string(), pos, want, value: v.to_string(), why: why.to_string() };
        let word = |v: &Value| -> Result<(), TypeError> {
            match v {
                Value::Sym(s) => match self.entity_problem(s) {
                    Some(p) => Err(bad(&p)),
                    None => Ok(()),
                },
                _ => Err(bad("is a number")),
            }
        };
        match want {
            ArgType::Entity | ArgType::Category | ArgType::Task | ArgType::Step => word(v),
            ArgType::Event => match v.as_sym() {
                Some(s) if s.starts_with('e') && s[1..].chars().all(|c| c.is_ascii_digit()) && s.len() > 1 => Ok(()),
                _ => word(v),
            },
            ArgType::Number => {
                if v.is_number() {
                    Ok(())
                } else {
                    Err(bad("is not a number"))
                }
            }
            ArgType::Unit => match v.as_sym() {
                Some(s) if self.is_unit(s) => Ok(()),
                _ => Err(bad("is not a known unit")),
            },
            ArgType::Fact => match v.as_sym() {
                Some(s) if s.len() > 1 && s.starts_with('f') && s[1..].chars().all(|c| c.is_ascii_digit()) => Ok(()),
                _ => Err(bad("is not a fact id")),
            },
            ArgType::Text => match v.as_sym() {
                Some(_) => Ok(()),
                None => Err(bad("is not text")),
            },
            ArgType::Symbol => match v.as_sym() {
                Some(s) if sla_core::term::is_plain_ident(s) && !self.function_words.contains(s) => {
                    if self.entity_problem(s).is_some() && s.contains('_') {
                        Err(bad(&self.entity_problem(s).unwrap()))
                    } else {
                        Ok(())
                    }
                }
                _ => Err(bad("is not a plain word")),
            },
            ArgType::Value => {
                if v.is_number() {
                    Ok(())
                } else {
                    word(v)
                }
            }
            ArgType::Any => match v {
                Value::Sym(s) if s.contains('_') => match self.entity_problem(s) {
                    Some(p) => Err(bad(&p)),
                    None => Ok(()),
                },
                Value::Sym(s) if self.function_words.contains(s) => Err(bad("is a function word, not a thing")),
                _ => Ok(()),
            },
        }
    }

    /// Type-check a fact (strong negations are checked as their base predicate).
    pub fn check(&self, g: &GroundAtom) -> Result<(), TypeError> {
        let base = g.base_pred();
        let Some(name) = self.resolve(base) else { return Err(TypeError::UnknownPredicate(base.to_string())) };
        let info = &self.preds[name];
        let types: Vec<ArgType> = if info.verb {
            match g.args.len() {
                1 => vec![ArgType::Entity],
                2 => vec![ArgType::Entity, ArgType::Any],
                n => return Err(TypeError::Arity { pred: name.to_string(), want: "1 or 2".into(), got: n }),
            }
        } else {
            if g.args.len() != info.types.len() {
                return Err(TypeError::Arity { pred: name.to_string(), want: info.types.len().to_string(), got: g.args.len() });
            }
            info.types.clone()
        };
        for (i, (v, t)) in g.args.iter().zip(&types).enumerate() {
            if let Some(why) = v.as_sym().and_then(|s| self.forbid.get(&(name.to_string(), i + 1, s.to_string()))) {
                return Err(TypeError::Forbidden { pred: name.to_string(), pos: i + 1, value: v.to_string(), why: why.clone() });
            }
            self.check_arg(name, i + 1, *t, v)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sla_core::parse_ground;

    #[test]
    fn builtin_registry_reads_cleanly() {
        let r = Registry::builtin();
        assert!(r.errors.is_empty(), "{:?}", r.errors);
        assert!(r.preds.len() > 300, "{}", r.preds.len());
        assert_eq!(r.get("takes_time").unwrap().types.len(), 4);
        assert!(r.get("located_in").unwrap().has_flag("single_valued"));
        assert_eq!(r.resolve("kind_of"), Some("is_a"), "synonyms map to the registered name");
        assert!(r.is_unit("minute") && !r.is_unit("banana"));
        // Every core predicate is registered.
        for (stem, src) in sla_core::CORE_FILES {
            for rule in sla_core::RuleFile::parse(src, stem).rules() {
                if rule.head.pred == "core_predicate" {
                    let p = rule.head.args[0].to_string();
                    assert!(r.get(&p).is_some(), "core predicate {p} is not in the registry");
                } else if rule.head.pred != "functional" {
                    assert!(r.get(&rule.head.pred).is_some(), "rule head {} is not in the registry", rule.head.pred);
                }
            }
        }
    }

    #[test]
    fn good_facts_pass() {
        let r = Registry::builtin();
        for f in [
            "is_a(rex, dog)",
            "likes(anna, green_tea)",
            "-likes(anna, coffee)",
            "takes_time(garlic_tomato_pasta, 15, 20, minute)",
            "takes_time(tea, 2.5, 4, minute)",
            "property(car, colour, red)",
            "property(nina, age, 12)",
            "located_in(anna, new_york)",
            "knows(anna, f1042)",
            "agent_of(e7, anna)",
            "step(garlic_tomato_pasta, gtp_s1)",
            "portion(pasta, 100, gram)",
            "play(anna, chess)",
            "bark(rex)",
            "surface(cherry_tomato, \"cherry tomato\")",
        ] {
            let g = parse_ground(f).unwrap();
            assert_eq!(r.check(&g), Ok(()), "{f}");
        }
    }

    #[test]
    fn bad_facts_fail_with_a_reason() {
        let r = Registry::builtin();
        let err = |f: &str| r.check(&parse_ground(f).unwrap()).unwrap_err();
        assert!(matches!(err("grow_on(moss, rock)"), TypeError::UnknownPredicate(_)));
        assert!(matches!(err("likes(anna)"), TypeError::Arity { .. }));
        assert!(matches!(err("takes_time(tea, three, five, minute)"), TypeError::BadArgument { pos: 2, .. }));
        assert!(matches!(err("takes_time(tea, 3, 5, banana)"), TypeError::BadArgument { pos: 4, .. }));
        assert!(matches!(err("knows(anna, it)"), TypeError::BadArgument { .. }));
        assert!(matches!(err("property(ant, quality, easy)"), TypeError::Forbidden { .. }));
        let e = err("is_a(here, first_fact)");
        assert!(e.to_string().contains("function word"), "{e}");
    }
}
