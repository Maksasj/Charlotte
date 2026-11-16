//! Milestone 2 acceptance: >= 50 cases, including recursion, negation and
//! time-bounded facts; every answer can print its proof.

use sla_core::{parse_ground, Fact, GroundAtom, Timestamp};
use sla_reasoner::{evaluate, facts_for, parse_program, run, EvalError, FactInput, Model, RuleInput};

/// Answers rendered as "X=a,Y=b" (sorted by var name), sorted; "yes" for a
/// true ground query.
fn answers(m: &Model, q: &str) -> Vec<String> {
    let mut v: Vec<String> = m
        .ask(q)
        .unwrap_or_else(|e| panic!("query {q}: {e}"))
        .iter()
        .map(|a| {
            if a.bindings.is_empty() {
                "yes".to_string()
            } else {
                a.bindings.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join(",")
            }
        })
        .collect();
    v.sort();
    v
}

/// Every derived or given fact in the model must have a proof whose leaves are given.
fn all_proofs_ok(m: &Model) {
    for g in m.all_facts() {
        let p = m.proof(&g).unwrap_or_else(|| panic!("no proof for {g}"));
        assert!(!p.render().is_empty());
        for leaf in p.leaves() {
            assert!(m.is_given(&leaf), "leaf {leaf} of {g} is not given");
        }
    }
}

struct Case {
    name: &'static str,
    program: &'static str,
    query: &'static str,
    expect: &'static [&'static str],
}

const FAMILY: &str = "
parent(ann, bob).
parent(bob, cid).
parent(cid, dan).
parent(ann, eve).
ancestor(X, Y) :- parent(X, Y).
ancestor(X, Z) :- parent(X, Y), ancestor(Y, Z).
sibling(X, Y) :- parent(P, X), parent(P, Y), X != Y.
";

const GRAPH: &str = "
edge(a, b).
edge(b, c).
edge(c, a).
edge(c, d).
edge(e, f).
reach(X, Y) :- edge(X, Y).
reach(X, Z) :- reach(X, Y), edge(Y, Z).
node(X) :- edge(X, _).
node(Y) :- edge(_, Y).
unreachable_from_a(X) :- node(X), not reach(a, X).
cyclic(X) :- reach(X, X).
";

const BIRDS: &str = "
is_a(tweety, bird).
is_a(pingu, penguin).
is_a(penguin, bird).
is_a(X, Z) :- is_a(X, Y), is_a(Y, Z).
abnormal(X) :- is_a(X, penguin).
flies(X) :- is_a(X, bird), not abnormal(X).
-flies(X) :- is_a(X, penguin).
";

const NUMBERS: &str = "
age(ann, 34).
age(bob, 12).
age(cid, 70).
age(dan, 34).
adult(X) :- age(X, A), A >= 18.
minor(X) :- age(X, A), A < 18.
older(X, Y) :- age(X, A), age(Y, B), A > B.
same_age(X, Y) :- age(X, A), age(Y, A), X != Y.
senior(X) :- age(X, A), A > 65.
";

const FOOD: &str = "
likes(maksim, coffee).
likes(anna, tea).
likes(bob, coffee).
-likes(anna, coffee).
drink(coffee).
drink(tea).
drink(juice).
hot(coffee).
hot(tea).
wants_hot(X) :- likes(X, D), hot(D).
untried(X, D) :- person(X), drink(D), not likes(X, D), not -likes(X, D).
person(maksim).
person(anna).
person(bob).
";

const STRATA: &str = "
p(a).
p(b).
p(c).
q(a).
r(X) :- p(X), not q(X).
s(X) :- p(X), not r(X).
t(X) :- s(X), not q(X).
u :- t(_).
v :- p(X), not w(X).
w(a).
w(b).
w(c).
";

const PARTS: &str = "
part_of(wheel, car).
part_of(spoke, wheel).
part_of(engine, car).
part_of(piston, engine).
part_of(X, Z) :- part_of(X, Y), part_of(Y, Z).
has(car, wheels).
is_a(beetle, car).
has(X, Y) :- is_a(X, C), has(C, Y), not -has(X, Y).
is_a(broken_car, car).
-has(broken_car, wheels).
";

const CASES: &[Case] = &[
    // --- recursion: ancestors
    Case { name: "ancestor direct", program: FAMILY, query: "ancestor(ann, bob)", expect: &["yes"] },
    Case { name: "ancestor depth 3", program: FAMILY, query: "ancestor(ann, dan)", expect: &["yes"] },
    Case { name: "ancestor none", program: FAMILY, query: "ancestor(dan, ann)", expect: &[] },
    Case { name: "ancestor all of ann", program: FAMILY, query: "ancestor(ann, X)", expect: &["X=bob", "X=cid", "X=dan", "X=eve"] },
    Case { name: "ancestors of dan", program: FAMILY, query: "ancestor(X, dan)", expect: &["X=ann", "X=bob", "X=cid"] },
    Case { name: "sibling", program: FAMILY, query: "sibling(bob, X)", expect: &["X=eve"] },
    Case { name: "no self sibling", program: FAMILY, query: "sibling(X, X)", expect: &[] },
    Case { name: "conjunctive query", program: FAMILY, query: "parent(ann, X), parent(X, Y)", expect: &["X=bob,Y=cid"] },
    Case { name: "query with negation", program: FAMILY, query: "parent(ann, X), not parent(X, _)", expect: &["X=eve"] },
    Case { name: "count ancestor pairs", program: FAMILY, query: "ancestor(X, Y), X = ann", expect: &["X=ann,Y=bob", "X=ann,Y=cid", "X=ann,Y=dan", "X=ann,Y=eve"] },
    // --- recursion: graphs with cycles
    Case { name: "reach through cycle", program: GRAPH, query: "reach(a, d)", expect: &["yes"] },
    Case { name: "reach self in cycle", program: GRAPH, query: "cyclic(X)", expect: &["X=a", "X=b", "X=c"] },
    Case { name: "reach set", program: GRAPH, query: "reach(b, X)", expect: &["X=a", "X=b", "X=c", "X=d"] },
    Case { name: "no reach other component", program: GRAPH, query: "reach(a, e)", expect: &[] },
    Case { name: "negation over recursion", program: GRAPH, query: "unreachable_from_a(X)", expect: &["X=e", "X=f"] },
    Case { name: "nodes", program: GRAPH, query: "node(X)", expect: &["X=a", "X=b", "X=c", "X=d", "X=e", "X=f"] },
    Case { name: "d not cyclic", program: GRAPH, query: "cyclic(d)", expect: &[] },
    // --- negation and defaults
    Case { name: "tweety flies", program: BIRDS, query: "flies(tweety)", expect: &["yes"] },
    Case { name: "pingu does not fly", program: BIRDS, query: "flies(pingu)", expect: &[] },
    Case { name: "pingu is a bird", program: BIRDS, query: "is_a(pingu, bird)", expect: &["yes"] },
    Case { name: "strong negation", program: BIRDS, query: "-flies(X)", expect: &["X=pingu"] },
    Case { name: "fliers", program: BIRDS, query: "flies(X)", expect: &["X=penguin", "X=tweety"] },
    Case { name: "neither", program: BIRDS, query: "is_a(X, bird), not flies(X), not -flies(X)", expect: &[] },
    // --- comparisons
    Case { name: "adults", program: NUMBERS, query: "adult(X)", expect: &["X=ann", "X=cid", "X=dan"] },
    Case { name: "minors", program: NUMBERS, query: "minor(X)", expect: &["X=bob"] },
    Case { name: "older than ann", program: NUMBERS, query: "older(X, ann)", expect: &["X=cid"] },
    Case { name: "same age", program: NUMBERS, query: "same_age(ann, X)", expect: &["X=dan"] },
    Case { name: "senior", program: NUMBERS, query: "senior(X)", expect: &["X=cid"] },
    Case { name: "cmp in query", program: NUMBERS, query: "age(X, A), A <= 34, A > 12", expect: &["A=34,X=ann", "A=34,X=dan"] },
    Case { name: "ne const", program: NUMBERS, query: "age(X, 34), X != ann", expect: &["X=dan"] },
    Case { name: "eq in query", program: NUMBERS, query: "age(X, A), A = 12", expect: &["A=12,X=bob"] },
    // --- negation, strong negation, closed world
    Case { name: "wants hot", program: FOOD, query: "wants_hot(X)", expect: &["X=anna", "X=bob", "X=maksim"] },
    Case { name: "untried by anna", program: FOOD, query: "untried(anna, D)", expect: &["D=juice"] },
    Case { name: "untried by maksim", program: FOOD, query: "untried(maksim, D)", expect: &["D=juice", "D=tea"] },
    Case { name: "explicit dislike", program: FOOD, query: "-likes(anna, X)", expect: &["X=coffee"] },
    Case { name: "unknown is not false", program: FOOD, query: "-likes(maksim, tea)", expect: &[] },
    Case { name: "coffee drinkers", program: FOOD, query: "likes(X, coffee)", expect: &["X=bob", "X=maksim"] },
    // --- stratification
    Case { name: "stratum 1", program: STRATA, query: "r(X)", expect: &["X=b", "X=c"] },
    Case { name: "stratum 2", program: STRATA, query: "s(X)", expect: &["X=a"] },
    Case { name: "stratum 3", program: STRATA, query: "t(X)", expect: &[] },
    Case { name: "zero-arity derived false", program: STRATA, query: "u", expect: &[] },
    Case { name: "zero-arity all blocked", program: STRATA, query: "v", expect: &[] },
    Case { name: "zero-arity fact", program: "flag.\nok :- flag.", query: "ok", expect: &["yes"] },
    // --- parts and inheritance with exceptions
    Case { name: "transitive parts", program: PARTS, query: "part_of(spoke, car)", expect: &["yes"] },
    Case { name: "parts of car", program: PARTS, query: "part_of(X, car)", expect: &["X=engine", "X=piston", "X=spoke", "X=wheel"] },
    Case { name: "inherited has", program: PARTS, query: "has(beetle, wheels)", expect: &["yes"] },
    Case { name: "exception blocks inheritance", program: PARTS, query: "has(broken_car, wheels)", expect: &[] },
    // --- misc
    Case { name: "strings", program: "name(bob, \"Big Bob\").\nshort(X) :- name(X, \"Big Bob\").", query: "short(X)", expect: &["X=bob"] },
    Case { name: "duplicate facts", program: "p(a).\np(a).\nq(X) :- p(X).", query: "q(X)", expect: &["X=a"] },
    Case { name: "rule with constant head", program: "p(a).\nflag(on) :- p(_).", query: "flag(X)", expect: &["X=on"] },
    Case { name: "repeated var in atom", program: "e(a, a).\ne(a, b).\nloop(X) :- e(X, X).", query: "loop(X)", expect: &["X=a"] },
    Case { name: "join on two vars", program: "e(a, b).\ne(b, a).\ne(b, c).\nmutual(X, Y) :- e(X, Y), e(Y, X).", query: "mutual(X, Y)", expect: &["X=a,Y=b", "X=b,Y=a"] },
    Case { name: "empty relation", program: "q(X) :- p(X).", query: "q(X)", expect: &[] },
    Case { name: "negated unknown predicate", program: "p(a).\nq(X) :- p(X), not zzz(X).", query: "q(X)", expect: &["X=a"] },
    Case { name: "same chain length 5", program: "n(1, 2).\nn(2, 3).\nn(3, 4).\nn(4, 5).\nn(5, 6).\nlt(X, Y) :- n(X, Y).\nlt(X, Z) :- lt(X, Y), n(Y, Z).", query: "lt(1, X)", expect: &["X=2", "X=3", "X=4", "X=5", "X=6"] },
    Case { name: "mutual recursion even/odd", program: "zero(0).\nsucc(0, 1).\nsucc(1, 2).\nsucc(2, 3).\nsucc(3, 4).\neven(X) :- zero(X).\neven(Y) :- odd(X), succ(X, Y).\nodd(Y) :- even(X), succ(X, Y).", query: "even(X)", expect: &["X=0", "X=2", "X=4"] },
];

#[test]
fn suite() {
    assert!(CASES.len() >= 50, "only {} cases", CASES.len());
    let mut failures = Vec::new();
    for c in CASES {
        let m = run(c.program).unwrap_or_else(|e| panic!("{}: {e}", c.name));
        all_proofs_ok(&m);
        let got = answers(&m, c.query);
        let mut want: Vec<String> = c.expect.iter().map(|s| s.to_string()).collect();
        want.sort();
        if got != want {
            failures.push(format!("{}: query {} -> got {:?}, want {:?}", c.name, c.query, got, want));
        }
    }
    assert!(failures.is_empty(), "{} failures:\n{}", failures.len(), failures.join("\n"));
}

#[test]
fn proof_trace_ancestor() {
    let m = run(FAMILY).unwrap();
    let g = parse_ground("ancestor(ann, dan)").unwrap();
    let p = m.proof(&g).unwrap();
    let text = p.render();
    assert!(text.starts_with("ancestor(ann, dan)"), "{text}");
    assert!(text.contains("parent(ann, bob) [conf 1] given by program"), "{text}");
    assert!(text.contains("parent(cid, dan)"), "{text}");
    assert_eq!(p.depth(), 4);
    let mut leaves: Vec<String> = p.leaves().iter().map(|l| l.to_string()).collect();
    leaves.sort();
    assert_eq!(leaves, vec!["parent(ann, bob)", "parent(bob, cid)", "parent(cid, dan)"]);
}

#[test]
fn proof_shows_negation() {
    let m = run(BIRDS).unwrap();
    let p = m.proof(&parse_ground("flies(tweety)").unwrap()).unwrap();
    assert!(p.render().contains("not abnormal(tweety)"), "{}", p.render());
}

#[test]
fn unstratifiable_rejected() {
    let r = run("p(a).\nq(X) :- p(X), not r(X).\nr(X) :- p(X), not q(X).");
    assert!(r.unwrap_err().contains("negation through recursion"));
}

#[test]
fn unsafe_rules_rejected() {
    let (f, r) = parse_program("p(a).\nq(X, Y) :- p(X).").unwrap();
    assert!(matches!(evaluate(&f, &r), Err(EvalError::Unsafe { .. })));
    let (f, r) = parse_program("p(a).\nq(X) :- p(X), not r(Y).").unwrap();
    assert!(matches!(evaluate(&f, &r), Err(EvalError::Unsafe { .. })));
    let (f, r) = parse_program("p(a).\nq(X) :- p(X), X > Y.").unwrap();
    assert!(matches!(evaluate(&f, &r), Err(EvalError::Unsafe { .. })));
}

#[test]
fn confidence_is_minimum() {
    let facts = vec![
        FactInput { atom: parse_ground("parent(a, b)").unwrap(), conf: 0.9, source: "user".into() },
        FactInput { atom: parse_ground("parent(b, c)").unwrap(), conf: 0.6, source: "user".into() },
    ];
    let rules = vec![
        RuleInput { id: "base".into(), rule: sla_core::parse_rule("anc(X, Y) :- parent(X, Y).").unwrap(), conf: 1.0 },
        RuleInput { id: "step".into(), rule: sla_core::parse_rule("anc(X, Z) :- parent(X, Y), anc(Y, Z).").unwrap(), conf: 0.8 },
    ];
    let m = evaluate(&facts, &rules).unwrap();
    assert_eq!(m.conf(&parse_ground("anc(a, b)").unwrap()), Some(0.9));
    assert_eq!(m.conf(&parse_ground("anc(b, c)").unwrap()), Some(0.6));
    assert_eq!(m.conf(&parse_ground("anc(a, c)").unwrap()), Some(0.6));
    let rules2 = vec![RuleInput { conf: 0.5, ..rules[1].clone() }, rules[0].clone()];
    let m2 = evaluate(&facts, &rules2).unwrap();
    assert_eq!(m2.conf(&parse_ground("anc(a, c)").unwrap()), Some(0.5));
}

#[test]
fn contradictions_detected() {
    let m = run("likes(a, tea).\n-likes(a, tea).\nlikes(b, tea).").unwrap();
    let c = m.contradictions();
    assert_eq!(c.len(), 1);
    assert_eq!(c[0].0.to_string(), "likes(a, tea)");
}

#[test]
fn time_bounded_facts() {
    let t = |d: u32| Timestamp::from_ymd(2026, 1, d);
    let likes = |s: &str| Fact::new(parse_ground(s).unwrap(), "user");
    let facts = vec![
        likes("likes(maksim, coffee)").with_from(t(1)).with_until(t(10)),
        likes("likes(maksim, tea)").with_from(t(10)),
        likes("works_at(maksim, acme)").with_from(t(1)).with_until(t(5)),
        likes("works_at(maksim, initech)").with_from(t(5)),
    ];
    let rules = vec![RuleInput::new("r", sla_core::parse_rule("drinks(X, D) :- likes(X, D).").unwrap())];
    let at = |d: u32| evaluate(&facts_for(&facts, Some(t(d))), &rules).unwrap();
    assert_eq!(answers(&at(3), "drinks(maksim, X)"), vec!["X=coffee"]);
    assert_eq!(answers(&at(10), "drinks(maksim, X)"), vec!["X=tea"]);
    assert_eq!(answers(&at(3), "works_at(maksim, X)"), vec!["X=acme"]);
    assert_eq!(answers(&at(7), "works_at(maksim, X)"), vec!["X=initech"]);
    let history = evaluate(&facts_for(&facts, None), &rules).unwrap();
    assert_eq!(answers(&history, "drinks(maksim, X)"), vec!["X=coffee", "X=tea"]);
    // Before anything was known.
    assert!(at(1).contains(&GroundAtom::syms("likes", &["maksim", "coffee"])));
    let before = evaluate(&facts_for(&facts, Some(Timestamp::from_ymd(2025, 12, 31))), &rules).unwrap();
    assert!(before.is_empty());
}

#[test]
fn rules_loaded_at_runtime_from_text() {
    // The same engine evaluates rules it has never seen at compile time.
    let mut program = String::from("edge(n0, n1).\n");
    for i in 1..40 {
        program.push_str(&format!("edge(n{i}, n{}).\n", i + 1));
    }
    program.push_str("path(X, Y) :- edge(X, Y).\npath(X, Z) :- path(X, Y), edge(Y, Z).\n");
    let m = run(&program).unwrap();
    assert_eq!(m.facts_of("path").len(), 41 * 40 / 2);
}
