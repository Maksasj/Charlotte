//! Terms, atoms, literals, rules and facts: the Datalog data model.
//!
//! A predicate name that starts with `-` is the *strong negation* of the
//! predicate without it: `-likes(maksim, tea)` states that maksim does not
//! like tea. `not p(X)` in a rule body is negation as failure.

use crate::time::Timestamp;
use std::fmt;

/// A ground value. Symbols that are not plain identifiers are written quoted.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Value {
    Int(i64),
    Sym(String),
}

impl Value {
    pub fn sym(s: impl Into<String>) -> Self {
        Value::Sym(s.into())
    }

    pub fn as_sym(&self) -> Option<&str> {
        match self {
            Value::Sym(s) => Some(s),
            Value::Int(_) => None,
        }
    }
}

/// True if `s` can be written without quotes as a constant.
pub fn is_plain_ident(s: &str) -> bool {
    let mut cs = s.chars();
    match cs.next() {
        Some(c) if c.is_ascii_lowercase() => {}
        _ => return false,
    }
    s != "not" && cs.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub fn quote(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            _ => out.push(c),
        }
    }
    out.push('"');
    out
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{i}"),
            Value::Sym(s) if is_plain_ident(s) => write!(f, "{s}"),
            Value::Sym(s) => write!(f, "{}", quote(s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Term {
    Var(String),
    Val(Value),
}

impl Term {
    pub fn var(s: &str) -> Self {
        Term::Var(s.to_string())
    }
    pub fn sym(s: &str) -> Self {
        Term::Val(Value::sym(s))
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Var(v) => write!(f, "{v}"),
            Term::Val(v) => write!(f, "{v}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Atom {
    pub pred: String,
    pub args: Vec<Term>,
}

impl Atom {
    pub fn new(pred: &str, args: Vec<Term>) -> Self {
        Atom { pred: pred.to_string(), args }
    }

    pub fn is_ground(&self) -> bool {
        self.args.iter().all(|t| matches!(t, Term::Val(_)))
    }

    pub fn to_ground(&self) -> Option<GroundAtom> {
        let mut args = Vec::with_capacity(self.args.len());
        for t in &self.args {
            match t {
                Term::Val(v) => args.push(v.clone()),
                Term::Var(_) => return None,
            }
        }
        Some(GroundAtom { pred: self.pred.clone(), args })
    }

    pub fn vars(&self) -> impl Iterator<Item = &str> {
        self.args.iter().filter_map(|t| match t {
            Term::Var(v) => Some(v.as_str()),
            Term::Val(_) => None,
        })
    }
}

fn write_args<T: fmt::Display>(f: &mut fmt::Formatter<'_>, pred: &str, args: &[T]) -> fmt::Result {
    write!(f, "{pred}")?;
    if !args.is_empty() {
        write!(f, "(")?;
        for (i, a) in args.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{a}")?;
        }
        write!(f, ")")?;
    }
    Ok(())
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_args(f, &self.pred, &self.args)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GroundAtom {
    pub pred: String,
    pub args: Vec<Value>,
}

impl GroundAtom {
    pub fn new(pred: &str, args: Vec<Value>) -> Self {
        GroundAtom { pred: pred.to_string(), args }
    }

    /// Convenience: all-symbol arguments.
    pub fn syms(pred: &str, args: &[&str]) -> Self {
        GroundAtom::new(pred, args.iter().map(|a| Value::sym(*a)).collect())
    }

    pub fn to_atom(&self) -> Atom {
        Atom {
            pred: self.pred.clone(),
            args: self.args.iter().cloned().map(Term::Val).collect(),
        }
    }

    pub fn is_strong_negation(&self) -> bool {
        self.pred.starts_with('-')
    }

    /// `p(..)` <-> `-p(..)`.
    pub fn complement(&self) -> GroundAtom {
        GroundAtom { pred: complement_pred(&self.pred), args: self.args.clone() }
    }

    /// The positive predicate name, without a leading `-`.
    pub fn base_pred(&self) -> &str {
        self.pred.trim_start_matches('-')
    }
}

pub fn complement_pred(p: &str) -> String {
    match p.strip_prefix('-') {
        Some(base) => base.to_string(),
        None => format!("-{p}"),
    }
}

impl fmt::Display for GroundAtom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_args(f, &self.pred, &self.args)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CmpOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

impl CmpOp {
    pub fn as_str(self) -> &'static str {
        match self {
            CmpOp::Eq => "=",
            CmpOp::Ne => "!=",
            CmpOp::Lt => "<",
            CmpOp::Le => "<=",
            CmpOp::Gt => ">",
            CmpOp::Ge => ">=",
        }
    }

    pub fn eval(self, a: &Value, b: &Value) -> bool {
        match self {
            CmpOp::Eq => a == b,
            CmpOp::Ne => a != b,
            CmpOp::Lt => a < b,
            CmpOp::Le => a <= b,
            CmpOp::Gt => a > b,
            CmpOp::Ge => a >= b,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Literal {
    Pos(Atom),
    /// Negation as failure: `not p(X)`.
    Neg(Atom),
    Cmp(CmpOp, Term, Term),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Pos(a) => write!(f, "{a}"),
            Literal::Neg(a) => write!(f, "not {a}"),
            Literal::Cmp(op, l, r) => write!(f, "{l} {} {r}", op.as_str()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rule {
    pub head: Atom,
    pub body: Vec<Literal>,
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.head)?;
        if !self.body.is_empty() {
            write!(f, " :- ")?;
            for (i, l) in self.body.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{l}")?;
            }
        }
        write!(f, ".")
    }
}

/// A stored fact with its metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct Fact {
    pub atom: GroundAtom,
    pub from: Option<Timestamp>,
    pub until: Option<Timestamp>,
    pub conf: f64,
    pub source: String,
    /// How many times the fact was used in an answer (for later forgetting).
    pub uses: u64,
    pub last_used: Option<Timestamp>,
}

impl Fact {
    pub fn new(atom: GroundAtom, source: &str) -> Self {
        Fact {
            atom,
            from: None,
            until: None,
            conf: 1.0,
            source: source.to_string(),
            uses: 0,
            last_used: None,
        }
    }

    pub fn with_from(mut self, t: Timestamp) -> Self {
        self.from = Some(t);
        self
    }

    pub fn with_until(mut self, t: Timestamp) -> Self {
        self.until = Some(t);
        self
    }

    pub fn with_conf(mut self, c: f64) -> Self {
        self.conf = c;
        self
    }

    /// Valid in the half-open interval `[from, until)`.
    pub fn valid_at(&self, t: Timestamp) -> bool {
        self.from.map_or(true, |f| f <= t) && self.until.map_or(true, |u| t < u)
    }

    pub fn is_open(&self) -> bool {
        self.until.is_none()
    }

    /// Canonical vault line, e.g.
    /// `likes(maksim, coffee). @from 2026-11-16 @conf 0.9 @source user`.
    pub fn to_line(&self) -> String {
        let mut s = format!("{}.", self.atom);
        if let Some(t) = self.from {
            s.push_str(&format!(" @from {t}"));
        }
        if let Some(t) = self.until {
            s.push_str(&format!(" @until {t}"));
        }
        s.push_str(&format!(" @conf {}", fmt_conf(self.conf)));
        let src = if self.source.is_empty()
            || self.source.contains(char::is_whitespace)
            || self.source.contains('"')
        {
            quote(&self.source)
        } else {
            self.source.clone()
        };
        s.push_str(&format!(" @source {src}"));
        if self.uses > 0 {
            s.push_str(&format!(" @uses {}", self.uses));
        }
        if let Some(t) = self.last_used {
            s.push_str(&format!(" @last {t}"));
        }
        s
    }
}

/// Confidence with at most 3 decimals, no trailing zeros.
pub fn fmt_conf(c: f64) -> String {
    let r = (c * 1000.0).round() / 1000.0;
    let s = format!("{r:.3}");
    let s = s.trim_end_matches('0').trim_end_matches('.');
    if s.is_empty() || s == "-" {
        "0".to_string()
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let a = GroundAtom::new("name", vec![Value::sym("bob"), Value::sym("Big Bob"), Value::Int(-3)]);
        assert_eq!(a.to_string(), "name(bob, \"Big Bob\", -3)");
        assert_eq!(a.complement().to_string(), "-name(bob, \"Big Bob\", -3)");
        assert_eq!(a.complement().complement(), a);
    }

    #[test]
    fn conf_format() {
        assert_eq!(fmt_conf(1.0), "1");
        assert_eq!(fmt_conf(0.9), "0.9");
        assert_eq!(fmt_conf(0.12345), "0.123");
        assert_eq!(fmt_conf(0.0), "0");
    }

    #[test]
    fn validity() {
        let t1 = Timestamp(10);
        let t2 = Timestamp(20);
        let f = Fact::new(GroundAtom::syms("p", &["a"]), "test").with_from(t1).with_until(t2);
        assert!(!f.valid_at(Timestamp(9)));
        assert!(f.valid_at(Timestamp(10)));
        assert!(f.valid_at(Timestamp(19)));
        assert!(!f.valid_at(Timestamp(20)));
    }
}
