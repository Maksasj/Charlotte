//! A small runtime Datalog interpreter.
//!
//! - Rules are data, loaded at runtime (from the vault).
//! - Semi-naive bottom-up evaluation, stratum by stratum.
//! - Stratified negation as failure (`not p(X)`); strong negation is just a
//!   predicate named `-p`.
//! - Comparisons `= != < <= > >=` on bound values.
//! - Every derived fact keeps the first derivation found, so each answer can
//!   print a proof tree back to given facts.
//! - Derived confidence is the minimum of the rule's and the premises'.

use sla_core::kb::LoadedRule;
use sla_core::term::{AggOp, ArithOp, Expr};
use sla_core::{Atom, CmpOp, Fact, GroundAtom, KnowledgeBase, Literal, Rule, Term, Timestamp, Value};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    /// A variable in the head, a negation or a comparison does not occur in a positive body atom.
    Unsafe { rule: String, var: String },
    /// Negation (or an aggregate) inside a recursive cycle.
    Unstratifiable { pred: String },
    /// Arithmetic inside a recursive cycle (it could create values forever).
    ArithmeticInRecursion { pred: String },
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::Unsafe { rule, var } => write!(f, "unsafe variable {var} in rule {rule}"),
            EvalError::Unstratifiable { pred } => write!(f, "negation through recursion involving {pred}"),
            EvalError::ArithmeticInRecursion { pred } => write!(f, "arithmetic inside recursion involving {pred}"),
        }
    }
}

impl std::error::Error for EvalError {}

#[derive(Debug, Clone, PartialEq)]
pub struct FactInput {
    pub atom: GroundAtom,
    pub conf: f64,
    pub source: String,
}

impl FactInput {
    pub fn new(atom: GroundAtom) -> Self {
        FactInput { atom, conf: 1.0, source: "given".into() }
    }
}

impl From<&Fact> for FactInput {
    fn from(f: &Fact) -> Self {
        FactInput { atom: f.atom.clone(), conf: f.conf, source: f.source.clone() }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuleInput {
    pub id: String,
    pub rule: Rule,
    pub conf: f64,
}

impl RuleInput {
    pub fn new(id: &str, rule: Rule) -> Self {
        RuleInput { id: id.into(), rule, conf: 1.0 }
    }
}

impl From<&LoadedRule> for RuleInput {
    fn from(r: &LoadedRule) -> Self {
        RuleInput { id: r.id.clone(), rule: r.rule.clone(), conf: r.conf }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Derivation {
    Given { conf: f64, source: String },
    Rule { rule: usize, premises: Vec<GroundAtom>, absent: Vec<String>, conf: f64 },
}

impl Derivation {
    pub fn conf(&self) -> f64 {
        match self {
            Derivation::Given { conf, .. } | Derivation::Rule { conf, .. } => *conf,
        }
    }
}

// ---------------------------------------------------------------- compile

#[derive(Debug, Clone)]
enum CTerm {
    Var(usize),
    Val(Value),
}

#[derive(Debug, Clone)]
enum CExpr {
    Term(CTerm),
    Bin(ArithOp, Box<CExpr>, Box<CExpr>),
}

#[derive(Debug, Clone)]
enum CLit {
    Pos { pred: String, args: Vec<CTerm> },
    Neg { pred: String, args: Vec<CTerm>, text: String },
    Cmp(CmpOp, CTerm, CTerm),
    Arith(CTerm, CExpr),
    /// `local`: slots of the aggregate's own variables; distinct bindings of
    /// them are what is counted or summed.
    Agg { op: AggOp, result: CTerm, value: Option<CTerm>, body: Vec<CLit>, local: Vec<usize> },
}

#[derive(Debug, Clone)]
struct CRule {
    head_pred: String,
    head: Vec<CTerm>,
    body: Vec<CLit>,
    nvars: usize,
    conf: f64,
    /// Variable names by slot (for query answers).
    names: Vec<String>,
    /// Predicates read inside aggregates (they must be in a lower stratum).
    agg_preds: Vec<String>,
    /// The rule computes new values with arithmetic.
    arith: bool,
}

fn compile_term(t: &Term, slots: &mut HashMap<String, usize>, names: &mut Vec<String>) -> CTerm {
    match t {
        Term::Val(v) => CTerm::Val(v.clone()),
        Term::Var(v) => {
            if v == "_" {
                names.push("_".into());
                return CTerm::Var(names.len() - 1);
            }
            if let Some(&i) = slots.get(v) {
                return CTerm::Var(i);
            }
            names.push(v.clone());
            let i = names.len() - 1;
            slots.insert(v.clone(), i);
            CTerm::Var(i)
        }
    }
}

fn compile_expr(e: &Expr, slots: &mut HashMap<String, usize>, names: &mut Vec<String>) -> CExpr {
    match e {
        Expr::Term(t) => CExpr::Term(compile_term(t, slots, names)),
        Expr::Bin(op, a, b) => CExpr::Bin(*op, Box::new(compile_expr(a, slots, names)), Box::new(compile_expr(b, slots, names))),
    }
}

/// Named variables of a term list.
fn named<'a>(ts: impl IntoIterator<Item = &'a Term>) -> Vec<String> {
    ts.into_iter().filter_map(|t| if let Term::Var(v) = t { (v != "_").then(|| v.clone()) } else { None }).collect()
}

/// All named variables in an aggregate (body, value, not the result).
fn agg_vars(value: &Option<Term>, body: &[Literal]) -> HashSet<String> {
    let mut out: HashSet<String> = body.iter().flat_map(|l| l.vars()).filter(|v| *v != "_").map(String::from).collect();
    out.extend(named(value.iter()));
    out
}

/// What a non-atom literal needs bound before it can run, and what it binds.
fn needs_binds(l: &Literal, outside: &HashSet<String>) -> (Vec<String>, Vec<String>) {
    match l {
        Literal::Pos(a) => (Vec::new(), named(a.args.iter())),
        Literal::Neg(a) => (named(a.args.iter()), Vec::new()),
        Literal::Cmp(_, x, y) => (named([x, y]), Vec::new()),
        Literal::Arith(t, e) => (e.vars().into_iter().filter(|v| *v != "_").map(String::from).collect(), named(std::iter::once(t))),
        Literal::Agg { result, value, body, .. } => {
            let shared: Vec<String> = agg_vars(value, body).into_iter().filter(|v| outside.contains(v) && Some(v) != named(std::iter::once(result)).first()).collect();
            (shared, named(std::iter::once(result)))
        }
    }
}

/// Order a body: positive atoms as written; every other literal right after
/// the variables it needs are bound (by atoms or by earlier literals' results).
/// `bound` holds the variables bound before the body starts (an aggregate's
/// shared variables). Returns the ordered literals and the variables bound at the end.
fn order_body<'a>(body: &'a [Literal], outside: &HashSet<String>, mut bound: HashSet<String>, label: &str) -> Result<(Vec<&'a Literal>, HashSet<String>), EvalError> {
    let mut out: Vec<&Literal> = Vec::new();
    let mut pending: Vec<&Literal> = body.iter().filter(|l| !matches!(l, Literal::Pos(_))).collect();
    let place_ready = |bound: &mut HashSet<String>, out: &mut Vec<&'a Literal>, pending: &mut Vec<&'a Literal>| loop {
        let ready = pending.iter().position(|l| needs_binds(l, outside).0.iter().all(|v| bound.contains(v)));
        match ready {
            Some(i) => {
                let l = pending.remove(i);
                bound.extend(needs_binds(l, outside).1);
                out.push(l);
            }
            None => break,
        }
    };
    place_ready(&mut bound, &mut out, &mut pending);
    for l in body.iter().filter(|l| matches!(l, Literal::Pos(_))) {
        out.push(l);
        bound.extend(needs_binds(l, outside).1);
        place_ready(&mut bound, &mut out, &mut pending);
    }
    if let Some(l) = pending.first() {
        let missing = needs_binds(l, outside).0.into_iter().find(|v| !bound.contains(v)).unwrap_or_default();
        return Err(EvalError::Unsafe { rule: label.to_string(), var: missing });
    }
    Ok((out, bound))
}

fn compile_lits(lits: &[&Literal], bound: &mut HashSet<String>, slots: &mut HashMap<String, usize>, names: &mut Vec<String>, agg_preds: &mut Vec<String>, label: &str) -> Result<Vec<CLit>, EvalError> {
    let mut out = Vec::new();
    for l in lits {
        out.push(match l {
            Literal::Pos(a) => {
                bound.extend(named(a.args.iter()));
                CLit::Pos { pred: a.pred.clone(), args: a.args.iter().map(|t| compile_term(t, slots, names)).collect() }
            }
            Literal::Neg(a) => CLit::Neg { pred: a.pred.clone(), args: a.args.iter().map(|t| compile_term(t, slots, names)).collect(), text: a.to_string() },
            Literal::Cmp(op, x, y) => {
                for t in [x, y] {
                    if matches!(t, Term::Var(v) if v == "_") {
                        return Err(EvalError::Unsafe { rule: label.to_string(), var: "_".into() });
                    }
                }
                let cx = compile_term(x, slots, names);
                let cy = compile_term(y, slots, names);
                CLit::Cmp(*op, cx, cy)
            }
            Literal::Arith(t, e) => {
                if matches!(t, Term::Var(v) if v == "_") || e.vars().contains(&"_") {
                    return Err(EvalError::Unsafe { rule: label.to_string(), var: "_".into() });
                }
                let ce = compile_expr(e, slots, names);
                bound.extend(named(std::iter::once(t)));
                CLit::Arith(compile_term(t, slots, names), ce)
            }
            Literal::Agg { result, op, value, body } => {
                // Variables bound before the aggregate are shared (they group it);
                // the rest are its own, and their distinct bindings are aggregated.
                let shared: HashSet<String> = bound.clone();
                let (ordered, _) = order_body(body, &shared, shared.clone(), label)?;
                for l in body {
                    if let Literal::Pos(a) | Literal::Neg(a) = l {
                        agg_preds.push(a.pred.clone());
                    }
                }
                if let Some(Term::Var(v)) = value {
                    if v == "_" || !body.iter().any(|l| matches!(l, Literal::Pos(_)) && l.vars().contains(&v.as_str())) {
                        return Err(EvalError::Unsafe { rule: label.to_string(), var: v.clone() });
                    }
                }
                let mut inner_bound = shared.clone();
                let cbody = compile_lits(&ordered, &mut inner_bound, slots, names, agg_preds, label)?;
                let value_c = value.as_ref().map(|v| compile_term(v, slots, names));
                let mut own: Vec<String> = agg_vars(value, body).into_iter().filter(|v| !shared.contains(v)).collect();
                own.sort();
                let local: Vec<usize> = own.iter().filter_map(|v| slots.get(v).copied()).collect();
                let result_c = compile_term(result, slots, names);
                bound.extend(named(std::iter::once(result)));
                CLit::Agg { op: *op, result: result_c, value: value_c, body: cbody, local }
            }
        });
    }
    Ok(out)
}

fn compile(head: &Atom, body: &[Literal], conf: f64, label: &str) -> Result<CRule, EvalError> {
    for v in head.vars() {
        if v == "_" {
            return Err(EvalError::Unsafe { rule: label.to_string(), var: "_".into() });
        }
    }
    // Variables used outside each aggregate: the head and every top-level literal.
    let mut outside: HashSet<String> = head.vars().map(String::from).collect();
    for l in body {
        if !matches!(l, Literal::Agg { .. }) {
            outside.extend(l.vars().into_iter().filter(|v| *v != "_").map(String::from));
        } else if let Literal::Agg { result: Term::Var(r), .. } = l {
            outside.insert(r.clone());
        }
    }
    // An aggregate's shared variables are those also used outside it.
    let (ordered, bound) = order_body(body, &outside, HashSet::new(), label)?;
    for v in head.vars() {
        if !bound.contains(v) {
            return Err(EvalError::Unsafe { rule: label.to_string(), var: v.to_string() });
        }
    }
    let mut slots = HashMap::new();
    let mut names = Vec::new();
    let mut agg_preds = Vec::new();
    let mut bound_now = HashSet::new();
    let cbody = compile_lits(&ordered, &mut bound_now, &mut slots, &mut names, &mut agg_preds, label)?;
    let chead = head.args.iter().map(|t| compile_term(t, &mut slots, &mut names)).collect();
    let arith = body.iter().any(|l| matches!(l, Literal::Arith(..)));
    Ok(CRule { head_pred: head.pred.clone(), head: chead, body: cbody, nvars: names.len(), conf, names, agg_preds, arith })
}

// ---------------------------------------------------------------- relations

#[derive(Debug, Default, Clone)]
struct Rel {
    tuples: Vec<Vec<Value>>,
    set: HashSet<Vec<Value>>,
    /// First-argument index.
    by_first: HashMap<Value, Vec<usize>>,
}

impl Rel {
    fn insert(&mut self, t: Vec<Value>) -> bool {
        if self.set.contains(&t) {
            return false;
        }
        if let Some(f) = t.first() {
            self.by_first.entry(f.clone()).or_default().push(self.tuples.len());
        }
        self.set.insert(t.clone());
        self.tuples.push(t);
        true
    }

    fn candidates<'a>(&'a self, first: Option<&Value>) -> Box<dyn Iterator<Item = &'a Vec<Value>> + 'a> {
        match first {
            Some(v) => match self.by_first.get(v) {
                Some(ix) => Box::new(ix.iter().map(move |&i| &self.tuples[i])),
                None => Box::new(std::iter::empty()),
            },
            None => Box::new(self.tuples.iter()),
        }
    }
}

type Bindings = Vec<Option<Value>>;

fn resolve<'a>(t: &'a CTerm, b: &'a Bindings) -> Option<&'a Value> {
    match t {
        CTerm::Val(v) => Some(v),
        CTerm::Var(i) => b[*i].as_ref(),
    }
}

fn unify(args: &[CTerm], tuple: &[Value], b: &mut Bindings, trail: &mut Vec<usize>) -> bool {
    if args.len() != tuple.len() {
        return false;
    }
    for (a, v) in args.iter().zip(tuple) {
        match a {
            CTerm::Val(c) => {
                if c != v {
                    return false;
                }
            }
            CTerm::Var(i) => match &b[*i] {
                Some(x) => {
                    if x != v {
                        return false;
                    }
                }
                None => {
                    b[*i] = Some(v.clone());
                    trail.push(*i);
                }
            },
        }
    }
    true
}

fn undo(b: &mut Bindings, trail: &mut Vec<usize>, mark: usize) {
    while trail.len() > mark {
        let i = trail.pop().unwrap();
        b[i] = None;
    }
}

// ---------------------------------------------------------------- model

/// The result of evaluation: all facts true at the evaluated time, with derivations.
#[derive(Debug, Clone, Default)]
pub struct Model {
    rels: HashMap<String, Rel>,
    derivs: HashMap<GroundAtom, Derivation>,
    rules: Vec<RuleInput>,
}

/// One answer to a conjunctive query.
#[derive(Debug, Clone, PartialEq)]
pub struct Answer {
    /// Named variables and their values, sorted by name.
    pub bindings: BTreeMap<String, Value>,
    /// The positive atoms that matched, in body order.
    pub premises: Vec<GroundAtom>,
    /// The negated atoms that were checked absent.
    pub absent: Vec<String>,
}

impl Answer {
    pub fn get(&self, var: &str) -> Option<&Value> {
        self.bindings.get(var)
    }
}

/// Proof tree for a fact.
#[derive(Debug, Clone, PartialEq)]
pub struct Proof {
    pub atom: GroundAtom,
    pub conf: f64,
    pub step: ProofStep,
    pub children: Vec<Proof>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProofStep {
    Given { source: String },
    Rule { id: String, rule: Rule, absent: Vec<String> },
}

impl Proof {
    /// Indented, human-readable proof.
    pub fn render(&self) -> String {
        let mut s = String::new();
        self.render_into(0, &mut s);
        s
    }

    fn render_into(&self, depth: usize, out: &mut String) {
        let pad = "  ".repeat(depth);
        let conf = sla_core::term::fmt_conf(self.conf);
        match &self.step {
            ProofStep::Given { source } => {
                out.push_str(&format!("{pad}{} [conf {conf}] given by {source}\n", self.atom));
            }
            ProofStep::Rule { id, rule, absent } => {
                out.push_str(&format!("{pad}{} [conf {conf}] by rule {id}: {rule}\n", self.atom));
                for c in &self.children {
                    c.render_into(depth + 1, out);
                }
                for a in absent {
                    out.push_str(&format!("{pad}  not {a} (no such fact known)\n"));
                }
            }
        }
    }

    /// Given facts at the leaves.
    pub fn leaves(&self) -> Vec<GroundAtom> {
        let mut out = Vec::new();
        self.collect_leaves(&mut out);
        out
    }

    fn collect_leaves(&self, out: &mut Vec<GroundAtom>) {
        match self.step {
            ProofStep::Given { .. } => {
                if !out.contains(&self.atom) {
                    out.push(self.atom.clone())
                }
            }
            ProofStep::Rule { .. } => self.children.iter().for_each(|c| c.collect_leaves(out)),
        }
    }

    /// Rule ids used anywhere in the proof.
    pub fn rules_used(&self) -> Vec<String> {
        let mut out = Vec::new();
        self.collect_rules(&mut out);
        out
    }

    fn collect_rules(&self, out: &mut Vec<String>) {
        if let ProofStep::Rule { id, .. } = &self.step {
            if !out.contains(id) {
                out.push(id.clone());
            }
        }
        self.children.iter().for_each(|c| c.collect_rules(out));
    }

    pub fn depth(&self) -> usize {
        1 + self.children.iter().map(|c| c.depth()).max().unwrap_or(0)
    }
}

impl Model {
    pub fn contains(&self, g: &GroundAtom) -> bool {
        self.rels.get(&g.pred).map_or(false, |r| r.set.contains(&g.args))
    }

    pub fn conf(&self, g: &GroundAtom) -> Option<f64> {
        self.derivs.get(g).map(|d| d.conf())
    }

    pub fn derivation(&self, g: &GroundAtom) -> Option<&Derivation> {
        self.derivs.get(g)
    }

    pub fn is_given(&self, g: &GroundAtom) -> bool {
        matches!(self.derivs.get(g), Some(Derivation::Given { .. }))
    }

    /// All facts of a predicate.
    pub fn facts_of(&self, pred: &str) -> Vec<GroundAtom> {
        self.rels
            .get(pred)
            .map(|r| r.tuples.iter().map(|t| GroundAtom { pred: pred.to_string(), args: t.clone() }).collect())
            .unwrap_or_default()
    }

    pub fn all_facts(&self) -> Vec<GroundAtom> {
        let mut preds: Vec<&String> = self.rels.keys().collect();
        preds.sort();
        preds.into_iter().flat_map(|p| self.facts_of(p)).collect()
    }

    pub fn predicates(&self) -> Vec<String> {
        let mut v: Vec<String> = self.rels.keys().cloned().collect();
        v.sort();
        v
    }

    pub fn len(&self) -> usize {
        self.derivs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.derivs.is_empty()
    }

    /// Ground atoms matching a single atom pattern.
    pub fn query_atom(&self, pattern: &Atom) -> Vec<GroundAtom> {
        let Some(rel) = self.rels.get(&pattern.pred) else { return Vec::new() };
        rel.tuples
            .iter()
            .map(|t| GroundAtom { pred: pattern.pred.clone(), args: t.clone() })
            .filter(|g| sla_core::kb::matches(pattern, g))
            .collect()
    }

    /// Answer a conjunctive query (positive atoms, `not` atoms, comparisons).
    pub fn query(&self, body: &[Literal]) -> Result<Vec<Answer>, EvalError> {
        let head = Atom { pred: "__query".into(), args: Vec::new() };
        let label = body.iter().map(|l| l.to_string()).collect::<Vec<_>>().join(", ");
        let cr = compile(&head, body, 1.0, &label)?;
        let mut raw = Vec::new();
        let mut b: Bindings = vec![None; cr.nvars];
        let mut trail = Vec::new();
        let mut prem = Vec::new();
        let mut absent = Vec::new();
        self.join(&cr.body, 0, None, &HashMap::new(), &mut b, &mut trail, &mut prem, &mut absent, &mut |b: &Bindings, prem: &[GroundAtom], absent: &[String]| {
            raw.push((b.clone(), prem.to_vec(), absent.to_vec()));
        });
        let mut seen = HashSet::new();
        let mut out = Vec::new();
        for (b, premises, absent) in raw {
            let mut bindings = BTreeMap::new();
            for (i, name) in cr.names.iter().enumerate() {
                if name != "_" {
                    if let Some(v) = &b[i] {
                        bindings.insert(name.clone(), v.clone());
                    }
                }
            }
            if seen.insert(bindings.clone()) {
                out.push(Answer { bindings, premises, absent });
            }
        }
        Ok(out)
    }

    /// Parse and answer a query string, e.g. `ancestor(ann, X)`.
    pub fn ask(&self, q: &str) -> Result<Vec<Answer>, String> {
        let body = sla_core::parse_query(q).map_err(|e| e.to_string())?;
        self.query(&body).map_err(|e| e.to_string())
    }

    pub fn proof(&self, g: &GroundAtom) -> Option<Proof> {
        let d = self.derivs.get(g)?;
        Some(match d {
            Derivation::Given { conf, source } => Proof {
                atom: g.clone(),
                conf: *conf,
                step: ProofStep::Given { source: source.clone() },
                children: Vec::new(),
            },
            Derivation::Rule { rule, premises, absent, conf } => {
                let r = &self.rules[*rule];
                Proof {
                    atom: g.clone(),
                    conf: *conf,
                    step: ProofStep::Rule { id: r.id.clone(), rule: r.rule.clone(), absent: absent.clone() },
                    children: premises.iter().filter_map(|p| self.proof(p)).collect(),
                }
            }
        })
    }

    /// Pairs `p(..)` / `-p(..)` that are both true.
    pub fn contradictions(&self) -> Vec<(GroundAtom, GroundAtom)> {
        let mut out = Vec::new();
        for (pred, rel) in &self.rels {
            if let Some(base) = pred.strip_prefix('-') {
                if let Some(pos) = self.rels.get(base) {
                    for t in &rel.tuples {
                        if pos.set.contains(t) {
                            out.push((
                                GroundAtom { pred: base.to_string(), args: t.clone() },
                                GroundAtom { pred: pred.clone(), args: t.clone() },
                            ));
                        }
                    }
                }
            }
        }
        out.sort();
        out
    }

    #[allow(clippy::too_many_arguments)]
    fn join(
        &self,
        body: &[CLit],
        li: usize,
        delta_at: Option<usize>,
        delta: &HashMap<String, Rel>,
        b: &mut Bindings,
        trail: &mut Vec<usize>,
        prem: &mut Vec<GroundAtom>,
        absent: &mut Vec<String>,
        emit: &mut dyn FnMut(&Bindings, &[GroundAtom], &[String]),
    ) {
        if li == body.len() {
            emit(b, prem, absent);
            return;
        }
        match &body[li] {
            CLit::Pos { pred, args } => {
                let rel = if delta_at == Some(li) { delta.get(pred) } else { self.rels.get(pred) };
                let Some(rel) = rel else { return };
                let first = args.first().and_then(|a| resolve(a, b)).cloned();
                // Collect candidates first: `b` is mutated while iterating.
                let cands: Vec<&Vec<Value>> = rel.candidates(first.as_ref()).collect();
                for t in cands {
                    let mark = trail.len();
                    if unify(args, t, b, trail) {
                        prem.push(GroundAtom { pred: pred.clone(), args: t.clone() });
                        self.join(body, li + 1, delta_at, delta, b, trail, prem, absent, emit);
                        prem.pop();
                    }
                    undo(b, trail, mark);
                }
            }
            CLit::Neg { pred, args, text } => {
                let exists = match self.rels.get(pred) {
                    None => false,
                    Some(rel) => {
                        let first = args.first().and_then(|a| resolve(a, b)).cloned();
                        let mut found = false;
                        for t in rel.candidates(first.as_ref()) {
                            let mark = trail.len();
                            let ok = unify(args, t, b, trail);
                            undo(b, trail, mark);
                            if ok {
                                found = true;
                                break;
                            }
                        }
                        found
                    }
                };
                if !exists {
                    // Record the checked atom with bound values filled in.
                    let shown = ground_text(pred, args, b).unwrap_or_else(|| text.clone());
                    absent.push(shown);
                    self.join(body, li + 1, delta_at, delta, b, trail, prem, absent, emit);
                    absent.pop();
                }
            }
            CLit::Cmp(op, x, y) => {
                if let (Some(vx), Some(vy)) = (resolve(x, b), resolve(y, b)) {
                    if op.eval(vx, vy) {
                        self.join(body, li + 1, delta_at, delta, b, trail, prem, absent, emit);
                    }
                }
            }
            CLit::Arith(t, e) => {
                let Some(v) = eval_expr(e, b) else { return };
                let mark = trail.len();
                if unify(std::slice::from_ref(t), std::slice::from_ref(&v), b, trail) {
                    self.join(body, li + 1, delta_at, delta, b, trail, prem, absent, emit);
                }
                undo(b, trail, mark);
            }
            CLit::Agg { op, result, value, body: inner, local } => {
                // Distinct bindings of the aggregate's own variables, with the premises behind them.
                let mut seen: HashSet<Vec<Option<Value>>> = HashSet::new();
                let mut values: Vec<Value> = Vec::new();
                let mut used: Vec<GroundAtom> = Vec::new();
                {
                    let mut sub_prem = Vec::new();
                    let mut sub_abs = Vec::new();
                    let none: HashMap<String, Rel> = HashMap::new();
                    let mut collect = |b: &Bindings, p: &[GroundAtom], _: &[String]| {
                        let key: Vec<Option<Value>> = local.iter().map(|i| b[*i].clone()).collect();
                        if seen.insert(key) {
                            if let Some(v) = value.as_ref().and_then(|t| resolve(t, b)) {
                                values.push(v.clone());
                            }
                            for g in p {
                                if !used.contains(g) {
                                    used.push(g.clone());
                                }
                            }
                        }
                    };
                    self.join(inner, 0, None, &none, b, trail, &mut sub_prem, &mut sub_abs, &mut collect);
                }
                let out = match op {
                    AggOp::Count => Value::number(seen.len() as i128 * sla_core::term::DEC_SCALE as i128),
                    AggOp::Sum => values.iter().try_fold(Value::Int(0), |acc, v| ArithOp::Add.apply(&acc, v)),
                    AggOp::Min => values.iter().filter(|v| v.is_number()).min().cloned(),
                    AggOp::Max => values.iter().filter(|v| v.is_number()).max().cloned(),
                };
                let Some(out) = out else { return };
                let mark = trail.len();
                if unify(std::slice::from_ref(result), std::slice::from_ref(&out), b, trail) {
                    let n = prem.len();
                    for g in used {
                        if !prem.contains(&g) {
                            prem.push(g);
                        }
                    }
                    self.join(body, li + 1, delta_at, delta, b, trail, prem, absent, emit);
                    prem.truncate(n);
                }
                undo(b, trail, mark);
            }
        }
    }

    fn insert(&mut self, g: GroundAtom, d: Derivation) -> bool {
        let rel = self.rels.entry(g.pred.clone()).or_default();
        if rel.insert(g.args.clone()) {
            self.derivs.insert(g, d);
            true
        } else {
            false
        }
    }
}

fn eval_expr(e: &CExpr, b: &Bindings) -> Option<Value> {
    match e {
        CExpr::Term(t) => resolve(t, b).filter(|v| v.is_number()).cloned(),
        CExpr::Bin(op, x, y) => op.apply(&eval_expr(x, b)?, &eval_expr(y, b)?),
    }
}

fn ground_text(pred: &str, args: &[CTerm], b: &Bindings) -> Option<String> {
    let mut parts = Vec::new();
    for a in args {
        parts.push(match resolve(a, b) {
            Some(v) => v.to_string(),
            None => "_".to_string(),
        });
    }
    if parts.is_empty() {
        Some(pred.to_string())
    } else {
        Some(format!("{pred}({})", parts.join(", ")))
    }
}

// ---------------------------------------------------------------- evaluation

/// Assign each derived predicate a stratum; negated dependencies and
/// predicates read inside aggregates must be lower. Arithmetic may not take
/// part in recursion, so evaluation cannot create endless new values.
fn stratify(rules: &[CRule]) -> Result<Vec<Vec<usize>>, EvalError> {
    let idb: HashSet<&str> = rules.iter().map(|r| r.head_pred.as_str()).collect();
    let mut st: HashMap<&str, usize> = idb.iter().map(|p| (*p, 0)).collect();
    let limit = idb.len() + 1;
    loop {
        let mut changed = false;
        for r in rules {
            let h = r.head_pred.as_str();
            let mut need = st[h];
            for l in &r.body {
                match l {
                    CLit::Pos { pred, .. } if idb.contains(pred.as_str()) => need = need.max(st[pred.as_str()]),
                    CLit::Neg { pred, .. } if idb.contains(pred.as_str()) => need = need.max(st[pred.as_str()] + 1),
                    _ => {}
                }
            }
            for p in &r.agg_preds {
                if idb.contains(p.as_str()) {
                    need = need.max(st[p.as_str()] + 1);
                }
            }
            if need > st[h] {
                if need > limit {
                    return Err(EvalError::Unstratifiable { pred: h.to_string() });
                }
                st.insert(h, need);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    // Arithmetic in recursion: a rule computing values whose head can reach
    // one of its own positive body predicates.
    let mut uses: HashMap<&str, HashSet<&str>> = HashMap::new();
    for r in rules {
        for l in &r.body {
            if let CLit::Pos { pred, .. } = l {
                uses.entry(r.head_pred.as_str()).or_default().insert(pred.as_str());
            }
        }
    }
    let reaches = |from: &str, to: &str| -> bool {
        let mut stack = vec![from];
        let mut seen: HashSet<&str> = HashSet::new();
        while let Some(p) = stack.pop() {
            if p == to {
                return true;
            }
            if seen.insert(p) {
                if let Some(n) = uses.get(p) {
                    stack.extend(n.iter().copied());
                }
            }
        }
        false
    };
    for r in rules.iter().filter(|r| r.arith) {
        for l in &r.body {
            if let CLit::Pos { pred, .. } = l {
                if idb.contains(pred.as_str()) && reaches(pred, &r.head_pred) {
                    return Err(EvalError::ArithmeticInRecursion { pred: r.head_pred.clone() });
                }
            }
        }
    }
    let max = st.values().copied().max().unwrap_or(0);
    let mut strata = vec![Vec::new(); max + 1];
    for (i, r) in rules.iter().enumerate() {
        strata[st[r.head_pred.as_str()]].push(i);
    }
    Ok(strata)
}

fn head_atom(r: &CRule, b: &Bindings) -> GroundAtom {
    GroundAtom {
        pred: r.head_pred.clone(),
        args: r.head.iter().map(|t| resolve(t, b).cloned().expect("safe rule binds head")).collect(),
    }
}

/// Evaluate rules over facts to a fixpoint.
pub fn evaluate(facts: &[FactInput], rules: &[RuleInput]) -> Result<Model, EvalError> {
    let compiled: Vec<CRule> = rules
        .iter()
        .map(|r| compile(&r.rule.head, &r.rule.body, r.conf, &r.id))
        .collect::<Result<_, _>>()?;
    let strata = stratify(&compiled)?;

    let mut m = Model { rules: rules.to_vec(), ..Default::default() };
    // Given facts; on duplicates keep the most confident.
    for f in facts {
        match m.derivs.get(&f.atom) {
            Some(d) if d.conf() >= f.conf => {}
            Some(_) => {
                m.derivs.insert(f.atom.clone(), Derivation::Given { conf: f.conf, source: f.source.clone() });
            }
            None => {
                m.insert(f.atom.clone(), Derivation::Given { conf: f.conf, source: f.source.clone() });
            }
        }
    }

    for stratum in &strata {
        let preds: HashSet<&str> = stratum.iter().map(|&i| compiled[i].head_pred.as_str()).collect();
        // Round 0: every rule against the full model.
        let mut delta = HashMap::new();
        let mut found = Vec::new();
        for &ri in stratum {
            m.fire(&compiled[ri], ri, None, &delta, &mut found);
        }
        let mut next = commit(&mut m, found);
        // Semi-naive rounds: at least one premise must come from the last delta.
        while !next.is_empty() {
            delta = next;
            let mut found = Vec::new();
            for &ri in stratum {
                let r = &compiled[ri];
                for (li, l) in r.body.iter().enumerate() {
                    if let CLit::Pos { pred, .. } = l {
                        if preds.contains(pred.as_str()) && delta.contains_key(pred) {
                            m.fire(r, ri, Some(li), &delta, &mut found);
                        }
                    }
                }
            }
            next = commit(&mut m, found);
        }
    }
    Ok(m)
}

/// Insert new facts; return them as the next delta.
fn commit(m: &mut Model, found: Vec<(GroundAtom, Derivation)>) -> HashMap<String, Rel> {
    let mut delta: HashMap<String, Rel> = HashMap::new();
    for (g, d) in found {
        if m.insert(g.clone(), d) {
            delta.entry(g.pred).or_default().insert(g.args);
        }
    }
    delta
}

impl Model {
    fn fire(&self, r: &CRule, ri: usize, delta_at: Option<usize>, delta: &HashMap<String, Rel>, out: &mut Vec<(GroundAtom, Derivation)>) {
        let mut b: Bindings = vec![None; r.nvars];
        let mut trail = Vec::new();
        let mut prem = Vec::new();
        let mut absent = Vec::new();
        self.join(&r.body, 0, delta_at, delta, &mut b, &mut trail, &mut prem, &mut absent, &mut |b: &Bindings, prem: &[GroundAtom], absent: &[String]| {
            let g = head_atom(r, b);
            if self.contains(&g) {
                return;
            }
            let conf = prem.iter().filter_map(|p| self.conf(p)).fold(r.conf, f64::min);
            out.push((g, Derivation::Rule { rule: ri, premises: prem.to_vec(), absent: absent.to_vec(), conf }));
        });
    }
}

/// Check that rules can be evaluated together (safe and stratifiable) without
/// evaluating them.
pub fn check(rules: &[RuleInput]) -> Result<(), EvalError> {
    let compiled: Vec<CRule> = rules
        .iter()
        .map(|r| compile(&r.rule.head, &r.rule.body, r.conf, &r.id))
        .collect::<Result<_, _>>()?;
    stratify(&compiled).map(|_| ())
}

/// Split rules into the ones that can run together and the ones that cannot.
/// Rules with an id starting `core/` are always kept (if core itself is
/// broken, the error is returned). Learned rules are added one at a time in
/// order; a rule that makes the program unsafe or unstratifiable is set aside
/// with its error.
pub fn quarantine(rules: &[RuleInput]) -> Result<(Vec<RuleInput>, Vec<(RuleInput, EvalError)>), EvalError> {
    let (mut kept, learned): (Vec<RuleInput>, Vec<RuleInput>) = rules.iter().cloned().partition(|r| r.id.starts_with("core/"));
    check(&kept)?;
    let mut bad = Vec::new();
    for r in learned {
        kept.push(r);
        if let Err(e) = check(&kept) {
            let r = kept.pop().unwrap();
            bad.push((r, e));
        }
    }
    Ok((kept, bad))
}

/// Predicates whose new facts can make a derived fact false: those that a
/// negated body atom depends on (directly or through rules).
pub fn naf_sensitive(rules: &[RuleInput]) -> HashSet<String> {
    let mut feeds: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut out: HashSet<String> = HashSet::new();
    let mut stack: Vec<String> = Vec::new();
    for r in rules {
        for l in &r.rule.body {
            match l {
                Literal::Pos(a) => feeds.entry(r.rule.head.pred.as_str()).or_default().push(a.pred.as_str()),
                Literal::Neg(a) => stack.push(a.pred.clone()),
                // A count or sum can go down when facts are added, like negation.
                Literal::Agg { body, .. } => {
                    for x in body {
                        if let Literal::Pos(a) | Literal::Neg(a) = x {
                            stack.push(a.pred.clone());
                        }
                    }
                }
                Literal::Cmp(..) | Literal::Arith(..) => {}
            }
        }
    }
    while let Some(p) = stack.pop() {
        if !out.insert(p.clone()) {
            continue;
        }
        if let Some(srcs) = feeds.get(p.as_str()) {
            stack.extend(srcs.iter().map(|s| s.to_string()));
        }
    }
    out
}

/// The predicates whose facts can affect `preds`: themselves and every
/// predicate in the body (positive or negated) of a rule deriving one of them,
/// transitively.
pub fn depends_on(rules: &[RuleInput], preds: &[&str]) -> HashSet<String> {
    let mut by_head: HashMap<&str, Vec<&str>> = HashMap::new();
    for r in rules {
        for l in &r.rule.body {
            let inner: Vec<&Literal> = match l {
                Literal::Agg { body, .. } => body.iter().collect(),
                other => vec![other],
            };
            for x in inner {
                if let Literal::Pos(a) | Literal::Neg(a) = x {
                    by_head.entry(r.rule.head.pred.as_str()).or_default().push(a.pred.as_str());
                }
            }
        }
    }
    let mut out: HashSet<String> = HashSet::new();
    let mut stack: Vec<String> = preds.iter().map(|p| p.to_string()).collect();
    while let Some(p) = stack.pop() {
        if !out.insert(p.clone()) {
            continue;
        }
        if let Some(b) = by_head.get(p.as_str()) {
            stack.extend(b.iter().map(|x| x.to_string()));
        }
    }
    out
}

/// The moments at which the past must be evaluated so that every fact that
/// was ever true (given or derived) is true in at least one of them, newest
/// first, at most `cap`. Facts are only ever joined at one moment, so a rule
/// never combines premises that were not true together.
///
/// A derived fact stays true until one of its premises ends, or (through
/// negation) until a new fact of a sensitive predicate starts, so the moment
/// just before every such boundary is enough.
pub fn past_moments(facts: &[Fact], rules: &[RuleInput], now: Timestamp, cap: usize) -> Vec<Timestamp> {
    let sensitive = naf_sensitive(rules);
    let mut ts: Vec<i64> = Vec::new();
    for f in facts {
        if let Some(u) = f.until {
            if u.0 <= now.0 {
                ts.push(u.0 - 1);
            }
        }
        if let Some(fr) = f.from {
            if fr.0 <= now.0 && sensitive.contains(f.atom.pred.as_str()) {
                ts.push(fr.0 - 1);
            }
        }
    }
    ts.sort_unstable();
    ts.dedup();
    ts.reverse();
    ts.truncate(cap);
    ts.into_iter().map(Timestamp).collect()
}

// ---------------------------------------------------------------- KB helpers

/// Facts valid at `at`, or every fact ever stored if `at` is `None` (history mode).
pub fn facts_for(facts: &[Fact], at: Option<Timestamp>) -> Vec<FactInput> {
    facts.iter().filter(|f| at.map_or(true, |t| f.valid_at(t))).map(FactInput::from).collect()
}

/// Evaluate the knowledge base at a time (or over all history).
pub fn evaluate_kb(kb: &KnowledgeBase, at: Option<Timestamp>, include_hypotheses: bool) -> Result<Model, EvalError> {
    let facts = facts_for(&kb.reasoning_facts(), at);
    let rules: Vec<RuleInput> = kb.rules(include_hypotheses).iter().map(RuleInput::from).collect();
    evaluate(&facts, &rules)
}

/// Parse a program text: one statement per line; ground statements without a
/// body become given facts, the rest rules (ids `r1`, `r2`, ...).
pub fn parse_program(src: &str) -> Result<(Vec<FactInput>, Vec<RuleInput>), String> {
    let mut facts = Vec::new();
    let mut rules = Vec::new();
    for (n, line) in src.lines().enumerate() {
        // A line may hold several statements: `p(a). q(b). r(X) :- p(X).`
        let mut rest = line.trim();
        while !rest.is_empty() && !rest.starts_with('%') {
            let end = sla_core::syntax::statement_end(rest).ok_or_else(|| format!("line {}: missing '.' at end of statement", n + 1))?;
            let stmt = &rest[..=end];
            rest = rest[end + 1..].trim();
            let r = sla_core::parse_rule(stmt).map_err(|e| format!("line {}: {e}", n + 1))?;
            if r.body.is_empty() && r.head.is_ground() {
                facts.push(FactInput { atom: r.head.to_ground().unwrap(), conf: 1.0, source: "program".into() });
            } else {
                rules.push(RuleInput::new(&format!("r{}", rules.len() + 1), r));
            }
        }
    }
    Ok((facts, rules))
}

/// Parse and evaluate a program text.
pub fn run(src: &str) -> Result<Model, String> {
    let (f, r) = parse_program(src)?;
    evaluate(&f, &r).map_err(|e| e.to_string())
}
