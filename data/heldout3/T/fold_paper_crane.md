---
id: fold_paper_crane
stage: tools
task: fold_paper_crane
schema: v3
tier: T
---
# Fold a paper crane

## requests
- How do you fold an origami crane?
- My kid wants to make one of those paper birds from Japan. Can you explain the folds?
- Teach me the steps for folding a crane out of paper.

## situation
- I have a sheet of square origami paper.

## ground truth
```datalog
task(fold_paper_crane).
domain(fold_paper_crane, crafts).
goal(fold_paper_crane, paper_crane).
goal(fold_paper_crane, decoration).
needs(fold_paper_crane, square_paper).
takes_time(fold_paper_crane, 10, 15, minute).
step(fold_paper_crane, fold_paper_crane_s1).
action(fold_paper_crane_s1, fold).
object_of(fold_paper_crane_s1, square_paper).
setting(fold_paper_crane_s1, shape, triangle, level).
step(fold_paper_crane, fold_paper_crane_s2).
action(fold_paper_crane_s2, fold).
object_of(fold_paper_crane_s2, square_base).
step(fold_paper_crane, fold_paper_crane_s3).
action(fold_paper_crane_s3, fold).
object_of(fold_paper_crane_s3, kite).
repeat(fold_paper_crane_s3, 2).
step(fold_paper_crane, fold_paper_crane_s4).
action(fold_paper_crane_s4, fold).
object_of(fold_paper_crane_s4, petal).
repeat(fold_paper_crane_s4, 2).
check(fold_paper_crane_s4, long_diamond).
step(fold_paper_crane, fold_paper_crane_s5).
action(fold_paper_crane_s5, narrow).
object_of(fold_paper_crane_s5, leg).
repeat(fold_paper_crane_s5, 2).
step(fold_paper_crane, fold_paper_crane_s6).
action(fold_paper_crane_s6, bend).
object_of(fold_paper_crane_s6, head).
before(fold_paper_crane_s1, fold_paper_crane_s2).
before(fold_paper_crane_s2, fold_paper_crane_s3).
before(fold_paper_crane_s3, fold_paper_crane_s4).
before(fold_paper_crane_s4, fold_paper_crane_s5).
before(fold_paper_crane_s5, fold_paper_crane_s6).
```

## aliases
```datalog
task_alias(fold_paper_crane, "origami crane").
task_alias(fold_paper_crane, "folding a crane").
task_alias(fold_paper_crane, "a paper crane").
alias(square_paper, "origami paper").
alias(square_base, "a square base").
alias(kite, "kite shape").
alias(petal, "petal fold").
alias(leg, "the legs").
alias(long_diamond, "a long diamond").
```

## teach
- To fold a paper crane you need a square sheet of paper.
- Folding a paper crane takes 10 to 15 minutes.
- Step 1 of folding a paper crane: fold the square paper into a triangle.
- Step 2 of folding a paper crane: fold it into a square base.
- Step 3 of folding a paper crane: fold a kite shape on both sides.
- Step 4 of folding a paper crane: make a petal fold on both sides until you have a long diamond.
- Step 5 of folding a paper crane: narrow both legs.
- Step 6 of folding a paper crane: bend one leg down to make the head.

## checklist
- names what is needed: a square sheet of paper
- gives the steps in order: triangle, square base, kite folds, petal folds, narrow legs, bend head

## variant
- change: only printer paper: cut a square from an A4 sheet first
- request: I don't have origami paper, only normal printer paper. Can I still make a crane?
- situation: I have a sheet of A4 printer paper and scissors.
- expect: The reply first folds one corner across and cuts off the strip to make a square from the A4 sheet, then folds as usual.

```datalog situation
has(user, a4_paper).
has(user, scissors).
lacks(user, square_paper).
```

```datalog difference
extra_need(a4_paper).
extra_need(scissors).
no_need(square_paper).
add_step(fold_paper_crane_s9, start).
step(fold_paper_crane, fold_paper_crane_s9).
action(fold_paper_crane_s9, cut).
object_of(fold_paper_crane_s9, a4_paper).
uses(fold_paper_crane_s9, scissors).
role(fold_paper_crane_s9, scissors, cut).
achieves(fold_paper_crane_s9, square).
before(fold_paper_crane_s9, fold_paper_crane_s1).
change(fold_paper_crane_s1, object_of, square_paper, a4_paper).
```

## variant
- change: ten cranes for a garland: repeat and string them
- request: I want to make a garland of ten paper cranes for a party. How long will that take, and how do I hang them?
- situation: I have ten sheets of square paper, a needle and thread.
- expect: The reply repeats the folding for ten cranes, about 1.5 to 2 hours, and threads them on a string with the needle.

```datalog situation
has(user, square_paper).
has(user, needle).
has(user, thread).
count(user, crane, 10).
```

```datalog difference
change(fold_paper_crane, takes_time, 10_15, 100_150).
extra_need(needle).
extra_need(thread).
add_step(fold_paper_crane_s9, fold_paper_crane_s6).
step(fold_paper_crane, fold_paper_crane_s9).
action(fold_paper_crane_s9, thread).
object_of(fold_paper_crane_s9, paper_crane).
uses(fold_paper_crane_s9, needle).
role(fold_paper_crane_s9, needle, pierce).
before(fold_paper_crane_s6, fold_paper_crane_s9).
```
