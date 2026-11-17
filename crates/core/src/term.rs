//! Terms, atoms, literals, rules and facts: the Datalog data model.
//!
//! A predicate name that starts with `-` is the *strong negation* of the
//! predicate without it: `-likes(maksim, tea)` states that maksim does not
//! like tea. `not p(X)` in a rule body is negation as failure.

use crate::time::Timestamp;
use std::fmt;

/// Decimals are fixed-point: `Dec(n)` is `n / DEC_SCALE` (six places).
pub const DEC_SCALE: i64 = 1_000_000;

/// A ground value. Symbols that are not plain identifiers are written quoted.
///
/// A number is `Int` when it is whole and `Dec` otherwise; [`Value::number`]
/// keeps that invariant, so equal numbers are always equal values. Numbers
/// order by size, and all numbers come before all symbols.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Int(i64),
    /// A decimal that is not whole, in millionths.
    Dec(i64),
    Sym(String),
}

impl Value {
    pub fn sym(s: impl Into<String>) -> Self {
        Value::Sym(s.into())
    }

    pub fn as_sym(&self) -> Option<&str> {
        match self {
            Value::Sym(s) => Some(s),
            Value::Int(_) | Value::Dec(_) => None,
        }
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Value::Int(_) | Value::Dec(_))
    }

    /// The number in millionths.
    pub fn scaled(&self) -> Option<i128> {
        match self {
            Value::Int(i) => Some(*i as i128 * DEC_SCALE as i128),
            Value::Dec(d) => Some(*d as i128),
            Value::Sym(_) => None,
        }
    }

    /// A number from millionths: `Int` if whole, else `Dec`; `None` on overflow.
    pub fn number(scaled: i128) -> Option<Value> {
        if scaled % DEC_SCALE as i128 == 0 {
            i64::try_from(scaled / DEC_SCALE as i128).ok().map(Value::Int)
        } else {
            i64::try_from(scaled).ok().map(Value::Dec)
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        self.scaled().map(|s| s as f64 / DEC_SCALE as f64)
    }

    /// Parse `12`, `-3`, `2.5`, `0.125` (at most six decimals).
    pub fn parse_number(s: &str) -> Option<Value> {
        let (neg, body) = match s.strip_prefix('-') {
            Some(b) => (true, b),
            None => (false, s),
        };
        let (int, frac) = body.split_once('.').unwrap_or((body, ""));
        if int.is_empty() || !int.chars().all(|c| c.is_ascii_digit()) || !frac.chars().all(|c| c.is_ascii_digit()) || frac.len() > 6 || (body.contains('.') && frac.is_empty()) {
            return None;
        }
        let mut scaled: i128 = int.parse::<i128>().ok()?.checked_mul(DEC_SCALE as i128)?;
        if !frac.is_empty() {
            let f: i128 = format!("{frac:0<6}").parse().ok()?;
            scaled += f;
        }
        Value::number(if neg { -scaled } else { scaled })
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.scaled(), other.scaled()) {
            (Some(a), Some(b)) => a.cmp(&b),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => self.as_sym().cmp(&other.as_sym()),
        }
    }
}

/// Arithmetic operators in rule bodies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArithOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl ArithOp {
    pub fn as_str(self) -> &'static str {
        match self {
            ArithOp::Add => "+",
            ArithOp::Sub => "-",
            ArithOp::Mul => "*",
            ArithOp::Div => "/",
        }
    }

    /// Apply to two numbers; `None` for symbols, division by zero or overflow.
    /// Division keeps six decimals.
    pub fn apply(self, a: &Value, b: &Value) -> Option<Value> {
        let (x, y) = (a.scaled()?, b.scaled()?);
        let s = DEC_SCALE as i128;
        let r = match self {
            ArithOp::Add => x.checked_add(y)?,
            ArithOp::Sub => x.checked_sub(y)?,
            ArithOp::Mul => x.checked_mul(y)? / s,
            ArithOp::Div => {
                if y == 0 {
                    return None;
                }
                x.checked_mul(s)? / y
            }
        };
        Value::number(r)
    }
}

/// An arithmetic expression over terms.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Term(Term),
    Bin(ArithOp, Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn vars(&self) -> Vec<&str> {
        match self {
            Expr::Term(Term::Var(v)) => vec![v.as_str()],
            Expr::Term(Term::Val(_)) => Vec::new(),
            Expr::Bin(_, a, b) => {
                let mut v = a.vars();
                v.extend(b.vars());
                v
            }
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Term(t) => write!(f, "{t}"),
            Expr::Bin(op, a, b) => {
                // Parenthesize nested sums inside products, so the text reads back the same.
                let wrap = |e: &Expr, f: &mut fmt::Formatter<'_>| -> fmt::Result {
                    match e {
                        Expr::Bin(inner, ..) if matches!(op, ArithOp::Mul | ArithOp::Div) && matches!(inner, ArithOp::Add | ArithOp::Sub) => write!(f, "({e})"),
                        _ => write!(f, "{e}"),
                    }
                };
                wrap(a, f)?;
                write!(f, " {} ", op.as_str())?;
                match &**b {
                    // Right operands of - and / with an operator of the same strength need brackets.
                    Expr::Bin(inner, ..) if matches!((op, inner), (ArithOp::Sub, ArithOp::Add | ArithOp::Sub) | (ArithOp::Div, ArithOp::Mul | ArithOp::Div) | (ArithOp::Mul | ArithOp::Div, ArithOp::Add | ArithOp::Sub)) => write!(f, "({b})"),
                    _ => write!(f, "{b}"),
                }
            }
        }
    }
}

/// Aggregates: `N = count { body }`, `S = sum X { body }`, `M = min X { body }`, `M = max X { body }`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AggOp {
    Count,
    Sum,
    Min,
    Max,
}

impl AggOp {
    pub fn as_str(self) -> &'static str {
        match self {
            AggOp::Count => "count",
            AggOp::Sum => "sum",
            AggOp::Min => "min",
            AggOp::Max => "max",
        }
    }

    pub fn parse(s: &str) -> Option<AggOp> {
        match s {
            "count" => Some(AggOp::Count),
            "sum" => Some(AggOp::Sum),
            "min" => Some(AggOp::Min),
            "max" => Some(AggOp::Max),
            _ => None,
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
            Value::Dec(d) => {
                let neg = *d < 0;
                let a = (*d as i128).abs();
                let s = DEC_SCALE as i128;
                let frac = format!("{:06}", a % s);
                write!(f, "{}{}.{}", if neg { "-" } else { "" }, a / s, frac.trim_end_matches('0'))
            }
            Value::Sym(s) if is_plain_ident(s) => write!(f, "{s}"),
            Value::Sym(s) => write!(f, "{}", quote(s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
    /// `T = X + Y * 2`: computes the expression (all its variables bound) and
    /// binds or checks `T`. Not allowed in recursion (no endless new values).
    Arith(Term, Expr),
    /// `S = sum D { step(T, X), takes_time(X, D, _, minute) }`: aggregates
    /// over the distinct bindings of the body's own variables, grouped by the
    /// variables it shares with the rest of the rule. Its predicates must be
    /// in a lower stratum (as for negation).
    Agg { result: Term, op: AggOp, value: Option<Term>, body: Vec<Literal> },
}

impl Literal {
    /// Variables the literal uses (for an aggregate: the shared ones are the
    /// caller's business, this lists every variable in it).
    pub fn vars(&self) -> Vec<&str> {
        match self {
            Literal::Pos(a) | Literal::Neg(a) => a.vars().collect(),
            Literal::Cmp(_, x, y) => [x, y].into_iter().filter_map(|t| if let Term::Var(v) = t { Some(v.as_str()) } else { None }).collect(),
            Literal::Arith(t, e) => {
                let mut v = e.vars();
                if let Term::Var(x) = t {
                    v.push(x);
                }
                v
            }
            Literal::Agg { result, value, body, .. } => {
                let mut v: Vec<&str> = body.iter().flat_map(|l| l.vars()).collect();
                for t in [Some(result), value.as_ref()].into_iter().flatten() {
                    if let Term::Var(x) = t {
                        v.push(x);
                    }
                }
                v
            }
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Pos(a) => write!(f, "{a}"),
            Literal::Neg(a) => write!(f, "not {a}"),
            Literal::Cmp(op, l, r) => write!(f, "{l} {} {r}", op.as_str()),
            Literal::Arith(t, e) => write!(f, "{t} = {e}"),
            Literal::Agg { result, op, value, body } => {
                write!(f, "{result} = {}", op.as_str())?;
                if let Some(v) = value {
                    write!(f, " {v}")?;
                }
                write!(f, " {{ ")?;
                for (i, l) in body.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{l}")?;
                }
                write!(f, " }}")
            }
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
    /// Stable id (`f1042`), given when the fact is stored. Other facts can
    /// refer to it (`knows(anna, f1042)`), and a reviewer closes exactly it.
    pub id: Option<String>,
    /// The conversation (session id) the fact came from.
    pub conv: Option<String>,
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
            id: None,
            conv: None,
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

    /// The conversation the fact came from.
    pub fn with_conv(mut self, session: &str) -> Self {
        self.conv = Some(session.to_string());
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
        if let Some(id) = &self.id {
            s.push_str(&format!(" @id {id}"));
        }
        if let Some(c) = &self.conv {
            s.push_str(&format!(" @conv {}", if c.contains(char::is_whitespace) || c.contains('"') { quote(c) } else { c.clone() }));
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
    fn numbers() {
        let v = |s: &str| Value::parse_number(s).unwrap();
        assert_eq!(v("12"), Value::Int(12));
        assert_eq!(v("2.50"), Value::Dec(2_500_000));
        assert_eq!(v("3.0"), Value::Int(3), "whole decimals are integers");
        assert_eq!(v("-0.125").to_string(), "-0.125");
        assert_eq!(v("2.5").to_string(), "2.5");
        assert!(Value::parse_number("2.").is_none() && Value::parse_number("1.1234567").is_none() && Value::parse_number("x").is_none());
        assert!(v("2.5") < Value::Int(3) && Value::Int(2) < v("2.5") && v("99") < Value::sym("a"));
        assert_eq!(ArithOp::Add.apply(&v("2.5"), &v("0.5")), Some(Value::Int(3)));
        assert_eq!(ArithOp::Mul.apply(&v("1.5"), &v("4")), Some(Value::Int(6)));
        assert_eq!(ArithOp::Div.apply(&v("1"), &v("4")), Some(v("0.25")));
        assert_eq!(ArithOp::Div.apply(&v("1"), &v("0")), None);
        assert_eq!(ArithOp::Sub.apply(&Value::sym("a"), &v("1")), None);
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
