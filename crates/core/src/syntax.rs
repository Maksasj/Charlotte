//! Text syntax for facts and rules.
//!
//! ```text
//! statement   := atom [ ":-" literal ("," literal)* ] "."  annotations?
//! literal     := "not" atom | term cmp term | term "=" expr | term "=" agg | atom
//! expr        := term (("+" | "-" | "*" | "/") term)*   (usual precedence, brackets allowed)
//! agg         := ("count" | "sum" Var | "min" Var | "max" Var) "{" literal ("," literal)* "}"
//! atom        := ["-"] ident [ "(" term ("," term)* ")" ]
//! term        := Var | ident | number | "quoted string"     (number: 12, -3, 2.5)
//! annotations := ( "@" key value )*      (values may be quoted)
//! ```
//! Variables start with an uppercase letter or `_`. `%` starts a comment.

use crate::term::{AggOp, ArithOp, Atom, CmpOp, Expr, Fact, GroundAtom, Literal, Rule, Term, Value};
use crate::time::Timestamp;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError(pub String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ParseError {}

fn err<T>(msg: impl Into<String>) -> Result<T, ParseError> {
    Err(ParseError(msg.into()))
}

#[derive(Debug, Clone, PartialEq)]
enum Tok {
    Ident(String),
    Var(String),
    Num(Value),
    Str(String),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Arith(ArithOp),
    Comma,
    ColonDash,
    Minus,
    Op(CmpOp),
}

fn lex(src: &str) -> Result<Vec<Tok>, ParseError> {
    let cs: Vec<char> = src.chars().collect();
    let mut i = 0;
    let mut out = Vec::new();
    while i < cs.len() {
        let c = cs[i];
        if c.is_whitespace() {
            i += 1;
        } else if c == '%' {
            break;
        } else if c.is_ascii_alphabetic() || c == '_' {
            let start = i;
            while i < cs.len() && (cs[i].is_ascii_alphanumeric() || cs[i] == '_') {
                i += 1;
            }
            let w: String = cs[start..i].iter().collect();
            if c.is_ascii_uppercase() || c == '_' {
                out.push(Tok::Var(w));
            } else {
                out.push(Tok::Ident(w));
            }
        } else if c.is_ascii_digit() || (c == '-' && i + 1 < cs.len() && cs[i + 1].is_ascii_digit() && !matches!(out.last(), Some(Tok::Num(_)) | Some(Tok::Var(_)) | Some(Tok::RParen))) {
            // A leading '-' is a sign only where an operand starts (`X - 1` is a subtraction).
            let start = i;
            i += 1;
            while i < cs.len() && cs[i].is_ascii_digit() {
                i += 1;
            }
            if i + 1 < cs.len() && cs[i] == '.' && cs[i + 1].is_ascii_digit() {
                i += 1;
                while i < cs.len() && cs[i].is_ascii_digit() {
                    i += 1;
                }
            }
            let w: String = cs[start..i].iter().collect();
            match Value::parse_number(&w) {
                Some(n) => out.push(Tok::Num(n)),
                None => return err(format!("bad number {w}")),
            }
        } else if c == '"' {
            i += 1;
            let mut s = String::new();
            loop {
                if i >= cs.len() {
                    return err("unterminated string");
                }
                match cs[i] {
                    '"' => {
                        i += 1;
                        break;
                    }
                    '\\' if i + 1 < cs.len() => {
                        s.push(match cs[i + 1] {
                            'n' => '\n',
                            other => other,
                        });
                        i += 2;
                    }
                    ch => {
                        s.push(ch);
                        i += 1;
                    }
                }
            }
            out.push(Tok::Str(s));
        } else {
            let next = cs.get(i + 1).copied();
            let (tok, len) = match (c, next) {
                ('(', _) => (Tok::LParen, 1),
                (')', _) => (Tok::RParen, 1),
                ('{', _) => (Tok::LBrace, 1),
                ('}', _) => (Tok::RBrace, 1),
                ('+', _) => (Tok::Arith(ArithOp::Add), 1),
                ('*', _) => (Tok::Arith(ArithOp::Mul), 1),
                ('/', _) => (Tok::Arith(ArithOp::Div), 1),
                (',', _) => (Tok::Comma, 1),
                (':', Some('-')) => (Tok::ColonDash, 2),
                ('!', Some('=')) => (Tok::Op(CmpOp::Ne), 2),
                ('\\', Some('=')) => (Tok::Op(CmpOp::Ne), 2),
                ('<', Some('=')) => (Tok::Op(CmpOp::Le), 2),
                ('>', Some('=')) => (Tok::Op(CmpOp::Ge), 2),
                ('=', _) => (Tok::Op(CmpOp::Eq), 1),
                ('<', _) => (Tok::Op(CmpOp::Lt), 1),
                ('>', _) => (Tok::Op(CmpOp::Gt), 1),
                ('-', _) => (Tok::Minus, 1),
                _ => return err(format!("unexpected character '{c}'")),
            };
            out.push(tok);
            i += len;
        }
    }
    Ok(out)
}

struct P {
    toks: Vec<Tok>,
    pos: usize,
}

impl P {
    fn peek(&self) -> Option<&Tok> {
        self.toks.get(self.pos)
    }
    fn peek_at(&self, k: usize) -> Option<&Tok> {
        self.toks.get(self.pos + k)
    }
    fn next(&mut self) -> Option<Tok> {
        let t = self.toks.get(self.pos).cloned();
        self.pos += 1;
        t
    }

    fn term(&mut self) -> Result<Term, ParseError> {
        match self.next() {
            Some(Tok::Var(v)) => Ok(Term::Var(v)),
            Some(Tok::Ident(s)) => Ok(Term::Val(Value::Sym(s))),
            Some(Tok::Str(s)) => Ok(Term::Val(Value::Sym(s))),
            Some(Tok::Num(n)) => Ok(Term::Val(n)),
            other => err(format!("expected a term, found {other:?}")),
        }
    }

    fn atom(&mut self) -> Result<Atom, ParseError> {
        let neg = if self.peek() == Some(&Tok::Minus) {
            self.pos += 1;
            true
        } else {
            false
        };
        let name = match self.next() {
            Some(Tok::Ident(s)) => s,
            other => return err(format!("expected a predicate name, found {other:?}")),
        };
        let mut args = Vec::new();
        if self.peek() == Some(&Tok::LParen) {
            self.pos += 1;
            if self.peek() != Some(&Tok::RParen) {
                loop {
                    args.push(self.term()?);
                    match self.next() {
                        Some(Tok::Comma) => continue,
                        Some(Tok::RParen) => break,
                        other => return err(format!("expected ',' or ')', found {other:?}")),
                    }
                }
            } else {
                self.pos += 1;
            }
        }
        let pred = if neg { format!("-{name}") } else { name };
        Ok(Atom { pred, args })
    }

    fn literal(&mut self) -> Result<Literal, ParseError> {
        // `not` followed by an atom start is negation as failure.
        if let Some(Tok::Ident(w)) = self.peek() {
            if w == "not" && matches!(self.peek_at(1), Some(Tok::Ident(_)) | Some(Tok::Minus)) {
                self.pos += 1;
                return Ok(Literal::Neg(self.atom()?));
            }
        }
        let is_cmp = matches!(self.peek_at(1), Some(Tok::Op(_)));
        if is_cmp {
            let l = self.term()?;
            let op = match self.next() {
                Some(Tok::Op(op)) => op,
                _ => unreachable!(),
            };
            if op == CmpOp::Eq {
                // `S = sum D { ... }` / `N = count { ... }`.
                if let Some(Tok::Ident(w)) = self.peek() {
                    if let Some(agg) = AggOp::parse(w) {
                        if matches!(self.peek_at(1), Some(Tok::LBrace) | Some(Tok::Var(_))) {
                            self.pos += 1;
                            return self.aggregate(l, agg);
                        }
                    }
                }
                // `T = X + Y`: arithmetic when the right side has an operator.
                let e = self.expr()?;
                return Ok(match e {
                    Expr::Term(r) => Literal::Cmp(CmpOp::Eq, l, r),
                    e => Literal::Arith(l, e),
                });
            }
            let r = self.term()?;
            return Ok(Literal::Cmp(op, l, r));
        }
        Ok(Literal::Pos(self.atom()?))
    }

    fn aggregate(&mut self, result: Term, op: AggOp) -> Result<Literal, ParseError> {
        let value = match self.peek() {
            Some(Tok::Var(_)) => Some(self.term()?),
            _ => None,
        };
        if op != AggOp::Count && value.is_none() {
            return err(format!("{} needs a variable to aggregate, as in `S = {} X {{ ... }}`", op.as_str(), op.as_str()));
        }
        if self.next() != Some(Tok::LBrace) {
            return err("expected '{' after the aggregate");
        }
        let mut body = Vec::new();
        loop {
            let l = self.literal()?;
            if matches!(l, Literal::Agg { .. }) {
                return err("aggregates cannot be nested");
            }
            body.push(l);
            match self.next() {
                Some(Tok::Comma) => continue,
                Some(Tok::RBrace) => break,
                other => return err(format!("expected ',' or '}}' in the aggregate, found {other:?}")),
            }
        }
        Ok(Literal::Agg { result, op, value, body })
    }

    /// expr := sum; sum := product (("+"|"-") product)*; product := factor (("*"|"/") factor)*
    fn expr(&mut self) -> Result<Expr, ParseError> {
        let mut e = self.product()?;
        loop {
            let op = match self.peek() {
                Some(Tok::Arith(ArithOp::Add)) => ArithOp::Add,
                Some(Tok::Minus) => ArithOp::Sub,
                _ => return Ok(e),
            };
            self.pos += 1;
            let r = self.product()?;
            e = Expr::Bin(op, Box::new(e), Box::new(r));
        }
    }

    fn product(&mut self) -> Result<Expr, ParseError> {
        let mut e = self.factor()?;
        loop {
            let op = match self.peek() {
                Some(Tok::Arith(ArithOp::Mul)) => ArithOp::Mul,
                Some(Tok::Arith(ArithOp::Div)) => ArithOp::Div,
                _ => return Ok(e),
            };
            self.pos += 1;
            let r = self.factor()?;
            e = Expr::Bin(op, Box::new(e), Box::new(r));
        }
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        if self.peek() == Some(&Tok::LParen) {
            self.pos += 1;
            let e = self.expr()?;
            if self.next() != Some(Tok::RParen) {
                return err("expected ')' in an expression");
            }
            return Ok(e);
        }
        Ok(Expr::Term(self.term()?))
    }

    fn rule(&mut self) -> Result<Rule, ParseError> {
        let head = self.atom()?;
        let mut body = Vec::new();
        if self.peek() == Some(&Tok::ColonDash) {
            self.pos += 1;
            loop {
                body.push(self.literal()?);
                if self.peek() == Some(&Tok::Comma) {
                    self.pos += 1;
                } else {
                    break;
                }
            }
        }
        if self.pos != self.toks.len() {
            return err(format!("unexpected trailing input: {:?}", &self.toks[self.pos..]));
        }
        Ok(Rule { head, body })
    }
}

/// Byte index of the `.` ending the statement (outside brackets and quotes,
/// and not the point of a decimal number).
pub fn statement_end(line: &str) -> Option<usize> {
    let mut depth = 0i32;
    let mut in_str = false;
    let mut escaped = false;
    let bytes = line.as_bytes();
    for (i, c) in line.char_indices() {
        if in_str {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_str = false;
            }
            continue;
        }
        match c {
            '"' => in_str = true,
            '(' | '{' => depth += 1,
            ')' | '}' => depth -= 1,
            '%' => return None,
            '.' if depth == 0 => {
                let digit_before = i > 0 && bytes[i - 1].is_ascii_digit();
                let digit_after = bytes.get(i + 1).map_or(false, |b| b.is_ascii_digit());
                if !(digit_before && digit_after) {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

/// Parse a rule or ground fact statement, without annotations: `p(X) :- q(X).`
pub fn parse_rule(src: &str) -> Result<Rule, ParseError> {
    let src = src.trim();
    let end = statement_end(src).ok_or_else(|| ParseError("missing '.' at end of statement".into()))?;
    let rest = src[end + 1..].trim();
    if !rest.is_empty() && !rest.starts_with('%') {
        return err(format!("unexpected text after '.': {rest}"));
    }
    parse_rule_body(&src[..end])
}

/// Parse a statement without its final '.'.
pub fn parse_rule_body(src: &str) -> Result<Rule, ParseError> {
    let mut p = P { toks: lex(src)?, pos: 0 };
    p.rule()
}

/// Parse a single atom (may contain variables), with or without trailing '.'.
pub fn parse_atom(src: &str) -> Result<Atom, ParseError> {
    let s = src.trim();
    let s = match statement_end(s) {
        Some(e) => &s[..e],
        None => s,
    };
    let mut p = P { toks: lex(s)?, pos: 0 };
    let a = p.atom()?;
    if p.pos != p.toks.len() {
        return err("unexpected trailing input after atom");
    }
    Ok(a)
}

pub fn parse_ground(src: &str) -> Result<GroundAtom, ParseError> {
    let a = parse_atom(src)?;
    a.to_ground().ok_or_else(|| ParseError(format!("fact must be ground: {a}")))
}

/// Parse a comma-separated conjunction of literals (a query).
pub fn parse_query(src: &str) -> Result<Vec<Literal>, ParseError> {
    let s = src.trim().trim_start_matches("?-").trim();
    let s = match statement_end(s) {
        Some(e) => &s[..e],
        None => s.trim_end_matches('?'),
    };
    let mut p = P { toks: lex(s)?, pos: 0 };
    let mut out = vec![p.literal()?];
    while p.peek() == Some(&Tok::Comma) {
        p.pos += 1;
        out.push(p.literal()?);
    }
    if p.pos != p.toks.len() {
        return err("unexpected trailing input in query");
    }
    Ok(out)
}

/// Split `@key value @key "quoted value"` into pairs.
pub fn parse_annotations(src: &str) -> Result<Vec<(String, String)>, ParseError> {
    let cs: Vec<char> = src.chars().collect();
    let mut i = 0;
    let mut out = Vec::new();
    let skip_ws = |i: &mut usize| {
        while *i < cs.len() && cs[*i].is_whitespace() {
            *i += 1;
        }
    };
    loop {
        skip_ws(&mut i);
        if i >= cs.len() || cs[i] == '%' {
            break;
        }
        if cs[i] != '@' {
            return err(format!("expected '@' annotation, found '{}'", cs[i]));
        }
        i += 1;
        let start = i;
        while i < cs.len() && (cs[i].is_ascii_alphanumeric() || cs[i] == '_') {
            i += 1;
        }
        let key: String = cs[start..i].iter().collect();
        if key.is_empty() {
            return err("empty annotation key");
        }
        skip_ws(&mut i);
        let mut val = String::new();
        if i < cs.len() && cs[i] == '"' {
            i += 1;
            loop {
                if i >= cs.len() {
                    return err("unterminated quoted annotation");
                }
                match cs[i] {
                    '"' => {
                        i += 1;
                        break;
                    }
                    '\\' if i + 1 < cs.len() => {
                        val.push(if cs[i + 1] == 'n' { '\n' } else { cs[i + 1] });
                        i += 2;
                    }
                    ch => {
                        val.push(ch);
                        i += 1;
                    }
                }
            }
        } else {
            while i < cs.len() && !cs[i].is_whitespace() {
                val.push(cs[i]);
                i += 1;
            }
        }
        out.push((key, val));
    }
    Ok(out)
}

/// Parse a vault fact line, e.g.
/// `likes(maksim, coffee). @from 2026-11-16 @conf 0.9 @source user`.
pub fn parse_fact_line(line: &str) -> Result<Fact, ParseError> {
    let line = line.trim();
    let end = statement_end(line).ok_or_else(|| ParseError("missing '.' after fact".into()))?;
    let atom = parse_ground(&line[..end])?;
    let mut fact = Fact::new(atom, "");
    for (k, v) in parse_annotations(&line[end + 1..])? {
        match k.as_str() {
            "from" => fact.from = Some(Timestamp::parse(&v).ok_or_else(|| ParseError(format!("bad @from time {v}")))?),
            "until" => fact.until = Some(Timestamp::parse(&v).ok_or_else(|| ParseError(format!("bad @until time {v}")))?),
            "conf" => {
                let c: f64 = v.parse().map_err(|_| ParseError(format!("bad @conf {v}")))?;
                if !(0.0..=1.0).contains(&c) {
                    return err(format!("@conf out of range: {v}"));
                }
                fact.conf = c;
            }
            "source" => fact.source = v,
            "id" => fact.id = Some(v),
            "conv" => fact.conv = Some(v),
            "uses" => fact.uses = v.parse().map_err(|_| ParseError(format!("bad @uses {v}")))?,
            "last" => fact.last_used = Some(Timestamp::parse(&v).ok_or_else(|| ParseError(format!("bad @last time {v}")))?),
            other => return err(format!("unknown annotation @{other}")),
        }
    }
    Ok(fact)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule_roundtrip() {
        for s in [
            "ancestor(X, Y) :- parent(X, Y).",
            "ancestor(X, Z) :- parent(X, Y), ancestor(Y, Z).",
            "lonely(X) :- person(X), not friend(X, _).",
            "older(X, Y) :- age(X, A), age(Y, B), A > B.",
            "-likes(X, tea) :- dislikes_hot(X).",
            "safe(X) :- thing(X), not -safe(X).",
            "name(bob, \"Big Bob\").",
            "flag.",
            "diff(X, Y) :- p(X), p(Y), X != Y.",
        ] {
            let r = parse_rule(s).unwrap_or_else(|e| panic!("{s}: {e}"));
            assert_eq!(r.to_string(), s);
        }
    }

    #[test]
    fn fact_line_roundtrip() {
        for s in [
            "likes(maksim, coffee). @from 2026-11-16 @conf 0.9 @source user",
            "likes(maksim, coffee). @from 2026-11-16T08:00 @until 2026-11-20 @conf 1 @source \"teacher haiku\" @uses 3 @last 2026-11-18",
            "age(bob, 42). @conf 0.5 @source synthetic",
        ] {
            let f = parse_fact_line(s).unwrap_or_else(|e| panic!("{s}: {e}"));
            assert_eq!(f.to_line(), s);
        }
    }

    #[test]
    fn errors() {
        assert!(parse_rule("p(X)").is_err());
        assert!(parse_rule("p(X :- q.").is_err());
        assert!(parse_fact_line("p(X). @conf 1").is_err());
        assert!(parse_fact_line("p(a). @conf 2").is_err());
        assert!(parse_fact_line("p(a). @bogus 1").is_err());
    }

    #[test]
    fn query() {
        let q = parse_query("?- likes(maksim, X), not hates(X, tea)?").unwrap();
        assert_eq!(q.len(), 2);
        assert!(matches!(q[1], Literal::Neg(_)));
        let q = parse_query("likes(maksim, X), X != tea").unwrap();
        assert_eq!(q.len(), 2);
    }
}
