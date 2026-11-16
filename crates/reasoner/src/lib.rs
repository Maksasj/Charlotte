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
use sla_core::{Atom, CmpOp, Fact, GroundAtom, KnowledgeBase, Literal, Rule, Term, Timestamp, Value};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    /// A variable in the head, a negation or a comparison does not occur in a positive body atom.
    Unsafe { rule: String, var: String },
    /// Negation inside a recursive cycle.
    Unstratifiable { pred: String },
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::Unsafe { rule, var } => write!(f, "unsafe variable {var} in rule {rule}"),
            EvalError::Unstratifiable { pred } => write!(f, "negation through recursion involving {pred}"),
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
enum CLit {
    Pos { pred: String, args: Vec<CTerm> },
    Neg { pred: String, args: Vec<CTerm>, text: String },
    Cmp(CmpOp, CTerm, CTerm),
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

fn compile(head: &Atom, body: &[Literal], conf: f64, label: &str) -> Result<CRule, EvalError> {
    let mut slots = HashMap::new();
    let mut names = Vec::new();

    // Safety: named variables must occur in a positive body atom.
    let mut positive_vars: HashSet<&str> = HashSet::new();
    for l in body {
        if let Literal::Pos(a) = l {
            positive_vars.extend(a.vars().filter(|v| *v != "_"));
        }
    }
    let check = |v: &str| -> Result<(), EvalError> {
        if v != "_" && !positive_vars.contains(v) {
            Err(EvalError::Unsafe { rule: label.to_string(), var: v.to_string() })
        } else {
            Ok(())
        }
    };
    for v in head.vars() {
        if v == "_" {
            return Err(EvalError::Unsafe { rule: label.to_string(), var: "_".into() });
        }
        check(v)?;
    }
    for l in body {
        match l {
            Literal::Pos(_) => {}
            Literal::Neg(a) => {
                for v in a.vars() {
                    check(v)?;
                }
            }
            Literal::Cmp(_, x, y) => {
                for t in [x, y] {
                    if let Term::Var(v) = t {
                        if v == "_" {
                            return Err(EvalError::Unsafe { rule: label.to_string(), var: "_".into() });
                        }
                        check(v)?;
                    }
                }
            }
        }
    }

    // Order: positive atoms as written; each filter right after its variables are bound.
    let pos: Vec<&Atom> = body.iter().filter_map(|l| if let Literal::Pos(a) = l { Some(a) } else { None }).collect();
    let filters: Vec<&Literal> = body.iter().filter(|l| !matches!(l, Literal::Pos(_))).collect();
    let filter_vars = |l: &Literal| -> Vec<String> {
        match l {
            Literal::Neg(a) => a.vars().filter(|v| *v != "_").map(String::from).collect(),
            Literal::Cmp(_, x, y) => [x, y]
                .iter()
                .filter_map(|t| if let Term::Var(v) = t { Some(v.clone()) } else { None })
                .collect(),
            Literal::Pos(_) => Vec::new(),
        }
    };
    // place[k] = number of positive atoms that must come before filter k.
    let mut place = Vec::new();
    for f in &filters {
        let need = filter_vars(*f);
        let mut bound: HashSet<&str> = HashSet::new();
        let mut k = 0;
        while !need.iter().all(|v| bound.contains(v.as_str())) && k < pos.len() {
            bound.extend(pos[k].vars());
            k += 1;
        }
        place.push(k);
    }

    let mut cbody = Vec::new();
    let emit_filters = |k: usize, cbody: &mut Vec<CLit>, slots: &mut HashMap<String, usize>, names: &mut Vec<String>| {
        for (fi, f) in filters.iter().enumerate() {
            if place[fi] != k {
                continue;
            }
            match f {
                Literal::Neg(a) => cbody.push(CLit::Neg {
                    pred: a.pred.clone(),
                    args: a.args.iter().map(|t| compile_term(t, slots, names)).collect(),
                    text: a.to_string(),
                }),
                Literal::Cmp(op, x, y) => {
                    let cx = compile_term(x, slots, names);
                    let cy = compile_term(y, slots, names);
                    cbody.push(CLit::Cmp(*op, cx, cy));
                }
                Literal::Pos(_) => {}
            }
        }
    };
    emit_filters(0, &mut cbody, &mut slots, &mut names);
    for (k, a) in pos.iter().enumerate() {
        cbody.push(CLit::Pos {
            pred: a.pred.clone(),
            args: a.args.iter().map(|t| compile_term(t, &mut slots, &mut names)).collect(),
        });
        emit_filters(k + 1, &mut cbody, &mut slots, &mut names);
    }
    let chead = head.args.iter().map(|t| compile_term(t, &mut slots, &mut names)).collect();
    Ok(CRule { head_pred: head.pred.clone(), head: chead, body: cbody, nvars: names.len(), conf, names })
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
        self.join(&cr, 0, None, &HashMap::new(), &mut b, &mut trail, &mut prem, &mut absent, &mut |b: &Bindings, prem: &[GroundAtom], absent: &[String]| {
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
        r: &CRule,
        li: usize,
        delta_at: Option<usize>,
        delta: &HashMap<String, Rel>,
        b: &mut Bindings,
        trail: &mut Vec<usize>,
        prem: &mut Vec<GroundAtom>,
        absent: &mut Vec<String>,
        emit: &mut dyn FnMut(&Bindings, &[GroundAtom], &[String]),
    ) {
        if li == r.body.len() {
            emit(b, prem, absent);
            return;
        }
        match &r.body[li] {
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
                        self.join(r, li + 1, delta_at, delta, b, trail, prem, absent, emit);
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
                    self.join(r, li + 1, delta_at, delta, b, trail, prem, absent, emit);
                    absent.pop();
                }
            }
            CLit::Cmp(op, x, y) => {
                if let (Some(vx), Some(vy)) = (resolve(x, b), resolve(y, b)) {
                    if op.eval(vx, vy) {
                        self.join(r, li + 1, delta_at, delta, b, trail, prem, absent, emit);
                    }
                }
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

/// Assign each derived predicate a stratum; negated dependencies must be lower.
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
        self.join(r, 0, delta_at, delta, &mut b, &mut trail, &mut prem, &mut absent, &mut |b: &Bindings, prem: &[GroundAtom], absent: &[String]| {
            let g = head_atom(r, b);
            if self.contains(&g) {
                return;
            }
            let conf = prem.iter().filter_map(|p| self.conf(p)).fold(r.conf, f64::min);
            out.push((g, Derivation::Rule { rule: ri, premises: prem.to_vec(), absent: absent.to_vec(), conf }));
        });
    }
}

// ---------------------------------------------------------------- KB helpers

/// Facts valid at `at`, or every fact ever stored if `at` is `None` (history mode).
pub fn facts_for(facts: &[Fact], at: Option<Timestamp>) -> Vec<FactInput> {
    facts.iter().filter(|f| at.map_or(true, |t| f.valid_at(t))).map(FactInput::from).collect()
}

/// Evaluate the knowledge base at a time (or over all history).
pub fn evaluate_kb(kb: &KnowledgeBase, at: Option<Timestamp>, include_hypotheses: bool) -> Result<Model, EvalError> {
    let facts = facts_for(&kb.all_facts(), at);
    let rules: Vec<RuleInput> = kb.rules(include_hypotheses).iter().map(RuleInput::from).collect();
    evaluate(&facts, &rules)
}

/// Parse a program text: one statement per line; ground statements without a
/// body become given facts, the rest rules (ids `r1`, `r2`, ...).
pub fn parse_program(src: &str) -> Result<(Vec<FactInput>, Vec<RuleInput>), String> {
    let mut facts = Vec::new();
    let mut rules = Vec::new();
    for (n, line) in src.lines().enumerate() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('%') {
            continue;
        }
        let r = sla_core::parse_rule(t).map_err(|e| format!("line {}: {e}", n + 1))?;
        if r.body.is_empty() && r.head.is_ground() {
            facts.push(FactInput { atom: r.head.to_ground().unwrap(), conf: 1.0, source: "program".into() });
        } else {
            rules.push(RuleInput::new(&format!("r{}", rules.len() + 1), r));
        }
    }
    Ok((facts, rules))
}

/// Parse and evaluate a program text.
pub fn run(src: &str) -> Result<Model, String> {
    let (f, r) = parse_program(src)?;
    evaluate(&f, &r).map_err(|e| e.to_string())
}
