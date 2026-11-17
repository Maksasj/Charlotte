//! Design 2.0, milestone 13 acceptance: at least 40 new reasoner cases over
//! the knowledge model (numbers, arithmetic, aggregates, the procedural and
//! choice layers), each answer printed with its proof. The core rules are the
//! real built-in ones (`crates/core/assets/core/*.md`).

use sla_core::{parse_ground, RuleFile};
use sla_reasoner::{evaluate, parse_program, EvalError, FactInput, Model, RuleInput};

/// Built-in core rules (without the `core_predicate` / `functional` declarations).
fn core_rules() -> Vec<RuleInput> {
    let mut out = Vec::new();
    for (stem, src) in sla_core::CORE_FILES {
        let rf = RuleFile::parse(src, stem);
        for (i, r) in rf.rules().into_iter().enumerate() {
            if r.head.pred == "core_predicate" || r.head.pred == "functional" {
                continue;
            }
            out.push(RuleInput::new(&format!("core/{stem}#{i}"), r));
        }
    }
    out
}

/// Facts and extra rules from `program`, evaluated with the core rules.
fn model(program: &str) -> Model {
    let (facts, mut rules) = parse_program(program).unwrap_or_else(|e| panic!("{e}"));
    rules.extend(core_rules());
    evaluate(&facts, &rules).unwrap_or_else(|e| panic!("{e}"))
}

fn eval_err(program: &str) -> EvalError {
    let (facts, rules) = parse_program(program).unwrap();
    evaluate(&facts, &rules).expect_err("should be refused")
}

/// Answers to `q` as "X=a,Y=b" sorted; each answer's first premise is proved
/// back to given facts, and the proof is printed.
fn ask(m: &Model, q: &str) -> Vec<String> {
    let answers = m.ask(q).unwrap_or_else(|e| panic!("{q}: {e}"));
    let mut out = Vec::new();
    for a in &answers {
        if let Some(g) = a.premises.first() {
            let p = m.proof(g).unwrap_or_else(|| panic!("no proof for {g}"));
            for leaf in p.leaves() {
                assert!(m.is_given(&leaf), "leaf {leaf} of {g} is not given");
            }
            println!("{q}  =>  {g}\n{}", p.render());
        }
        out.push(if a.bindings.is_empty() { "yes".into() } else { a.bindings.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join(",") });
    }
    out.sort();
    out
}

/// The garlic tomato pasta procedure from the design (with step times).
const PASTA: &str = "
task(garlic_tomato_pasta).
solves(garlic_tomato_pasta, plan_meal).
needs(garlic_tomato_pasta, pasta).
needs(garlic_tomato_pasta, garlic).
needs(garlic_tomato_pasta, olive_oil).
needs(garlic_tomato_pasta, cherry_tomato).
needs(garlic_tomato_pasta, salt).
takes_time(garlic_tomato_pasta, 15, 20, minute).
step(garlic_tomato_pasta, gtp_s1).
action(gtp_s1, boil).
object_of(gtp_s1, pasta).
takes_time(gtp_s1, 10, 12, minute).
step(garlic_tomato_pasta, gtp_s2).
action(gtp_s2, fry).
object_of(gtp_s2, garlic).
uses(gtp_s2, olive_oil).
uses(gtp_s2, pan).
takes_time(gtp_s2, 1, 1, minute).
overlaps(gtp_s2, gtp_s1).
step(garlic_tomato_pasta, gtp_s3).
action(gtp_s3, add).
object_of(gtp_s3, cherry_tomato).
until(gtp_s3, soft).
takes_time(gtp_s3, 5, 7, minute).
step(garlic_tomato_pasta, gtp_s4).
action(gtp_s4, mix).
object_of(gtp_s4, pasta).
takes_time(gtp_s4, 30, 60, second).
before(gtp_s2, gtp_s3).
before(gtp_s1, gtp_s4).
before(gtp_s3, gtp_s4).
staple(salt).
task(tomato_soup).
solves(tomato_soup, plan_meal).
needs(tomato_soup, tomato).
needs(tomato_soup, stock).
";

const KITCHEN: &str = "
available(pasta).
available(garlic).
available(olive_oil).
available(cherry_tomato).
available(pan).
";

// ------------------------------------------------------------ numbers and arithmetic

#[test]
fn c01_decimal_facts_and_comparison() {
    let m = model("price(tea, 2.5). price(coffee, 3). cheap(X) :- price(X, P), P < 3.");
    assert_eq!(ask(&m, "cheap(X)"), vec!["X=tea"]);
}

#[test]
fn c02_int_and_decimal_compare_by_size() {
    let m = model("w(a, 2). w(b, 2.25). w(c, 10). heavier(X, Y) :- w(X, A), w(Y, B), A > B.");
    assert_eq!(ask(&m, "heavier(b, Y)"), vec!["Y=a"]);
    assert_eq!(ask(&m, "heavier(c, Y)"), vec!["Y=a", "Y=b"]);
}

#[test]
fn c03_arithmetic_binds_a_value() {
    let m = model("w(box, 3). w(bag, 1.5). total(X, Y, S) :- w(X, A), w(Y, B), X != Y, S = A + B.");
    assert_eq!(ask(&m, "total(box, bag, S)"), vec!["S=4.5"]);
}

#[test]
fn c04_precedence_and_brackets() {
    let m = model("n(a, 2). n(b, 3). r1(X) :- n(a, A), n(b, B), X = A + B * 2. r2(X) :- n(a, A), n(b, B), X = (A + B) * 2.");
    assert_eq!(ask(&m, "r1(X)"), vec!["X=8"]);
    assert_eq!(ask(&m, "r2(X)"), vec!["X=10"]);
}

#[test]
fn c05_division_keeps_decimals() {
    let m = model("q(7, 2). half(X) :- q(A, B), X = A / B.");
    assert_eq!(ask(&m, "half(X)"), vec!["X=3.5"]);
}

#[test]
fn c06_division_by_zero_gives_nothing() {
    let m = model("q(7, 0). r(X) :- q(A, B), X = A / B.");
    assert!(ask(&m, "r(X)").is_empty());
}

#[test]
fn c07_arithmetic_as_a_check() {
    let m = model("pair(2, 4). pair(3, 5). double(X, Y) :- pair(X, Y), Y = X * 2.");
    assert_eq!(ask(&m, "double(X, Y)"), vec!["X=2,Y=4"]);
}

#[test]
fn c08_arithmetic_on_symbols_fails_quietly() {
    let m = model("v(a, tea). r(X) :- v(a, A), X = A + 1.");
    assert!(ask(&m, "r(X)").is_empty());
}

#[test]
fn c09_arithmetic_in_recursion_is_refused() {
    let e = eval_err("count_up(0). count_up(N) :- count_up(M), N = M + 1.");
    assert!(matches!(e, EvalError::ArithmeticInRecursion { .. }), "{e}");
}

#[test]
fn c10_unsafe_arithmetic_is_refused() {
    let e = eval_err("r(X) :- q(A), X = A + B.");
    assert!(matches!(e, EvalError::Unsafe { .. }), "{e}");
}

// ------------------------------------------------------------ aggregates

#[test]
fn c11_count() {
    let m = model("likes(anna, tea). likes(anna, jam). likes(bob, tea). person(anna). person(bob). n(P, N) :- person(P), N = count { likes(P, X) }.");
    assert_eq!(ask(&m, "n(P, N)"), vec!["N=1,P=bob", "N=2,P=anna"]);
}

#[test]
fn c12_count_of_nothing_is_zero() {
    let m = model("person(carl). likes(anna, tea). n(P, N) :- person(P), N = count { likes(P, X) }.");
    assert_eq!(ask(&m, "n(carl, N)"), vec!["N=0"]);
}

#[test]
fn c13_sum_with_decimals() {
    let m = model("basket(b1). item(b1, i1). item(b1, i2). item(b1, i3). price(i1, 1.25). price(i2, 2). price(i3, 0.75). total(B, S) :- basket(B), S = sum P { item(B, I), price(I, P) }.");
    assert_eq!(ask(&m, "total(b1, S)"), vec!["S=4"]);
}

#[test]
fn c14_sum_counts_distinct_items_not_values() {
    // Two items with the same price both count.
    let m = model("basket(b1). item(b1, i1). item(b1, i2). price(i1, 2). price(i2, 2). total(B, S) :- basket(B), S = sum P { item(B, I), price(I, P) }.");
    assert_eq!(ask(&m, "total(b1, S)"), vec!["S=4"]);
}

#[test]
fn c15_min_and_max() {
    let m = model("shop(s). offer(s, a, 3). offer(s, b, 1.5). offer(s, c, 7). lo(S, M) :- shop(S), M = min P { offer(S, X, P) }. hi(S, M) :- shop(S), M = max P { offer(S, X, P) }.");
    assert_eq!(ask(&m, "lo(s, M)"), vec!["M=1.5"]);
    assert_eq!(ask(&m, "hi(s, M)"), vec!["M=7"]);
}

#[test]
fn c16_min_of_nothing_gives_no_answer() {
    let m = model("shop(s). lo(S, M) :- shop(S), M = min P { offer(S, X, P) }.");
    assert!(ask(&m, "lo(s, M)").is_empty());
}

#[test]
fn c17_aggregate_over_derived_facts() {
    let m = model("parent(ann, bob). parent(bob, cid). parent(cid, dan). anc(X, Y) :- parent(X, Y). anc(X, Z) :- parent(X, Y), anc(Y, Z). person(ann). nd(P, N) :- person(P), N = count { anc(P, D) }.");
    assert_eq!(ask(&m, "nd(ann, N)"), vec!["N=3"]);
}

#[test]
fn c18_aggregate_in_recursion_is_refused() {
    let e = eval_err("p(a). big(X) :- p(X), N = count { big(Y) }, N > 0.");
    assert!(matches!(e, EvalError::Unstratifiable { .. }), "{e}");
}

#[test]
fn c19_aggregate_then_arithmetic() {
    let m = model("class(c). pupil(c, p1). pupil(c, p2). pupil(c, p3). mark(p1, 4). mark(p2, 5). mark(p3, 3). avg(C, A) :- class(C), S = sum M { pupil(C, P), mark(P, M) }, N = count { pupil(C, P) }, A = S / N.");
    assert_eq!(ask(&m, "avg(c, A)"), vec!["A=4"]);
}

#[test]
fn c20_aggregate_with_negation_inside() {
    let m = model("box(b). in(b, x). in(b, y). in(b, z). broken(y). good(B, N) :- box(B), N = count { in(B, I), not broken(I) }.");
    assert_eq!(ask(&m, "good(b, N)"), vec!["N=2"]);
}

#[test]
fn c21_aggregate_syntax_round_trips() {
    let src = "total(T, S) :- task(T), S = sum L { step(T, X), minutes(X, L, H) }.";
    let r = sla_core::parse_rule(src).unwrap();
    assert_eq!(r.to_string(), src);
    let src2 = "r(X) :- q(A, B), X = (A + B) * 2 - B / 4.";
    assert_eq!(sla_core::parse_rule(src2).unwrap().to_string(), src2);
}

// ------------------------------------------------------------ procedural layer

#[test]
fn c22_total_duration_by_sum() {
    // Steps 1, 3, 4 add up (10-12, 5-7, 0.5-1 minutes); step 2 runs while step 1 runs.
    let m = model(PASTA);
    assert_eq!(ask(&m, "total_minutes(garlic_tomato_pasta, Lo, Hi)"), vec!["Hi=20,Lo=15.5"]);
}

#[test]
fn c23_seconds_and_hours_become_minutes() {
    let m = model("takes_time(a, 90, 120, second). takes_time(b, 1, 2, hour).");
    assert_eq!(ask(&m, "minutes(a, L, H)"), vec!["H=2,L=1.5"]);
    assert_eq!(ask(&m, "minutes(b, L, H)"), vec!["H=120,L=60"]);
}

#[test]
fn c24_task_time_stated_and_computed_agree() {
    let m = model(PASTA);
    let stated = ask(&m, "takes_time(garlic_tomato_pasta, Lo, Hi, minute)");
    assert_eq!(stated, vec!["Hi=20,Lo=15"]);
    // The computed total sits inside the stated range.
    let a = m.ask("total_minutes(garlic_tomato_pasta, Lo, Hi)").unwrap();
    let lo = a[0].get("Lo").unwrap().as_f64().unwrap();
    let hi = a[0].get("Hi").unwrap().as_f64().unwrap();
    assert!(lo >= 15.0 && hi <= 20.0, "{lo} {hi}");
}

#[test]
fn c25_step_count() {
    let m = model(PASTA);
    assert_eq!(ask(&m, "step_count(garlic_tomato_pasta, N)"), vec!["N=4"]);
}

#[test]
fn c26_all_needs_include_what_steps_use() {
    let m = model(PASTA);
    assert_eq!(
        ask(&m, "needs_all(garlic_tomato_pasta, I)"),
        vec!["I=cherry_tomato", "I=garlic", "I=olive_oil", "I=pan", "I=pasta", "I=salt"]
    );
}

#[test]
fn c27_all_needs_through_sub_tasks() {
    let m = model("task(sandwich_lunch). subtask(sandwich_lunch, make_sandwich). subtask(sandwich_lunch, make_tea). needs(sandwich_lunch, plate). task(make_sandwich). needs(make_sandwich, bread). needs(make_sandwich, cheese). task(make_tea). subtask(make_tea, boil_water). needs(make_tea, tea_bag). task(boil_water). needs(boil_water, kettle).");
    assert_eq!(ask(&m, "needs_all(sandwich_lunch, I)"), vec!["I=bread", "I=cheese", "I=kettle", "I=plate", "I=tea_bag"]);
}

#[test]
fn c28_nothing_missing_with_a_full_kitchen() {
    let m = model(&format!("{PASTA}{KITCHEN}"));
    assert!(ask(&m, "missing(garlic_tomato_pasta, I)").is_empty(), "salt is a staple");
    assert_eq!(ask(&m, "feasible(garlic_tomato_pasta)"), vec!["yes"]);
}

#[test]
fn c29_missing_items_given_what_the_user_has() {
    let m = model(&format!("{PASTA}available(pasta). available(olive_oil). available(pan)."));
    assert_eq!(ask(&m, "missing(garlic_tomato_pasta, I)"), vec!["I=cherry_tomato", "I=garlic"]);
    assert!(ask(&m, "feasible(garlic_tomato_pasta)").is_empty());
}

#[test]
fn c30_a_substitute_covers_a_missing_item() {
    let m = model(&format!("{PASTA}available(pasta). available(garlic). available(olive_oil). available(pan). available(canned_tomato). substitute(cherry_tomato, canned_tomato)."));
    assert!(ask(&m, "missing(garlic_tomato_pasta, I)").is_empty());
    assert_eq!(ask(&m, "feasible(garlic_tomato_pasta)"), vec!["yes"]);
}

#[test]
fn c31_options_for_a_goal_are_the_feasible_ones() {
    let m = model(&format!("{PASTA}{KITCHEN}"));
    assert_eq!(ask(&m, "solves(T, plan_meal)"), vec!["T=garlic_tomato_pasta", "T=tomato_soup"]);
    assert_eq!(ask(&m, "option_for(plan_meal, T)"), vec!["T=garlic_tomato_pasta"]);
}

#[test]
fn c32_what_the_closest_option_lacks() {
    let m = model(&format!("{PASTA}{KITCHEN}"));
    assert_eq!(ask(&m, "missing(tomato_soup, I)"), vec!["I=stock", "I=tomato"]);
    assert_eq!(ask(&m, "lacking(T)"), vec!["T=tomato_soup"]);
}

#[test]
fn c33_first_steps_in_a_partial_order() {
    // Boiling and frying have nothing before them: both can start.
    let m = model(PASTA);
    assert_eq!(ask(&m, "next_step(garlic_tomato_pasta, S)"), vec!["S=gtp_s1", "S=gtp_s2"]);
}

#[test]
fn c34_next_step_after_some_are_done() {
    let m = model(&format!("{PASTA}done(gtp_s1). done(gtp_s2)."));
    assert_eq!(ask(&m, "next_step(garlic_tomato_pasta, S)"), vec!["S=gtp_s3"]);
}

#[test]
fn c35_the_last_step_waits_for_all_before_it() {
    // Before is transitive: mixing waits for frying through adding the tomatoes.
    let m = model(&format!("{PASTA}done(gtp_s1). done(gtp_s3)."));
    assert_eq!(ask(&m, "next_step(garlic_tomato_pasta, S)"), vec!["S=gtp_s2"]);
    assert_eq!(ask(&m, "waiting(gtp_s4)"), vec!["yes"]);
}

#[test]
fn c36_no_next_step_when_all_are_done() {
    let m = model(&format!("{PASTA}done(gtp_s1). done(gtp_s2). done(gtp_s3). done(gtp_s4)."));
    assert!(ask(&m, "next_step(garlic_tomato_pasta, S)").is_empty());
}

#[test]
fn c37_step_details_for_the_reply() {
    let m = model(PASTA);
    assert_eq!(ask(&m, "action(gtp_s3, A), object_of(gtp_s3, O), until(gtp_s3, U)"), vec!["A=add,O=cherry_tomato,U=soft"]);
}

#[test]
fn c38_the_agent_knows_a_task_with_steps() {
    let m = model(PASTA);
    assert_eq!(ask(&m, "knows_task(T)"), vec!["T=garlic_tomato_pasta"]);
}

#[test]
fn c39_events_with_many_participants() {
    let m = model("event(e7). action(e7, give). agent_of(e7, anna). object_of(e7, book). recipient(e7, bob). during(e7, christmas). gave(A, R, O) :- event(E), action(E, give), agent_of(E, A), recipient(E, R), object_of(E, O).");
    assert_eq!(ask(&m, "gave(anna, R, O)"), vec!["O=book,R=bob"]);
}

#[test]
fn c40_statements_about_facts_by_id() {
    // "Anna knows that Bob likes tea": the fact is f1042, and knows() points at it.
    let m = model("likes(bob, tea). fact_id(f1042, bob, tea). knows(anna, f1042). knows_like(A, P, X) :- knows(A, F), fact_id(F, P, X), likes(P, X).");
    assert_eq!(ask(&m, "knows_like(anna, P, X)"), vec!["P=bob,X=tea"]);
}

#[test]
fn c41_portions_scale_with_people() {
    let m = model("portion(pasta, 100, gram). party(p1). people(p1, 4). amount(P, I, N, U) :- party(P), people(P, K), portion(I, Q, U), N = Q * K.");
    assert_eq!(ask(&m, "amount(p1, pasta, N, U)"), vec!["N=400,U=gram"]);
}

#[test]
fn c42_time_left_in_a_budget() {
    let m = model(&format!("{PASTA}budget(me, 20). fits(T) :- total_minutes(T, Lo, Hi), budget(me, B), Hi <= B. spare(T, R) :- total_minutes(T, Lo, Hi), budget(me, B), R = B - Hi."));
    assert_eq!(ask(&m, "fits(T)"), vec!["T=garlic_tomato_pasta"]);
    assert_eq!(ask(&m, "spare(garlic_tomato_pasta, R)"), vec!["R=0"]);
}

#[test]
fn c43_proofs_show_the_steps_behind_a_total() {
    let m = model(PASTA);
    let g = parse_ground("total_minutes(garlic_tomato_pasta, 15.5, 20)").unwrap();
    let p = m.proof(&g).expect("derived");
    let text = p.render();
    println!("{text}");
    for s in ["gtp_s1", "gtp_s3", "gtp_s4"] {
        assert!(text.contains(s), "the proof names step {s}");
    }
    assert!(!p.leaves().iter().any(|l| l.to_string() == "takes_time(gtp_s2, 1, 1, minute)"), "the overlapping step adds nothing");
}

#[test]
fn c44_facts_given_as_input_keep_their_numbers() {
    let f = FactInput::new(parse_ground("takes_time(tea, 3, 4.5, minute)").unwrap());
    let m = evaluate(&[f], &core_rules()).unwrap();
    assert_eq!(ask(&m, "minutes(tea, L, H)"), vec!["H=4.5,L=3"]);
}
