---
id: make_friendship_bracelet
stage: tools
task: make_friendship_bracelet
schema: v3
tier: T
---
# Make a friendship bracelet

## requests
- How do I make one of those woven string bracelets?
- My niece wants to make friendship bracelets for her friends. Can you explain how?
- What's the method for knotting a bracelet out of embroidery thread?

## situation
- I have embroidery thread in three colours, scissors and tape.

## ground truth
```datalog
task(make_friendship_bracelet).
domain(make_friendship_bracelet, crafts).
goal(make_friendship_bracelet, bracelet).
goal(make_friendship_bracelet, gift).
needs(make_friendship_bracelet, embroidery_thread).
needs(make_friendship_bracelet, scissors).
needs(make_friendship_bracelet, tape).
takes_time(make_friendship_bracelet, 45, 60, minute).
step(make_friendship_bracelet, make_friendship_bracelet_s1).
action(make_friendship_bracelet_s1, cut).
object_of(make_friendship_bracelet_s1, embroidery_thread).
uses(make_friendship_bracelet_s1, scissors).
role(make_friendship_bracelet_s1, scissors, cut).
amount(make_friendship_bracelet_s1, embroidery_thread, 60, centimetre).
step(make_friendship_bracelet, make_friendship_bracelet_s2).
action(make_friendship_bracelet_s2, knot).
object_of(make_friendship_bracelet_s2, embroidery_thread).
with_part(make_friendship_bracelet_s2, end).
step(make_friendship_bracelet, make_friendship_bracelet_s3).
action(make_friendship_bracelet_s3, tape).
object_of(make_friendship_bracelet_s3, knot).
onto(make_friendship_bracelet_s3, table).
uses(make_friendship_bracelet_s3, tape).
role(make_friendship_bracelet_s3, tape, fasten).
step(make_friendship_bracelet, make_friendship_bracelet_s4).
action(make_friendship_bracelet_s4, tie).
object_of(make_friendship_bracelet_s4, knot).
repeat(make_friendship_bracelet_s4, 40).
purpose(make_friendship_bracelet_s4, pattern).
step(make_friendship_bracelet, make_friendship_bracelet_s5).
action(make_friendship_bracelet_s5, tie).
object_of(make_friendship_bracelet_s5, end).
step(make_friendship_bracelet, make_friendship_bracelet_s6).
action(make_friendship_bracelet_s6, trim).
object_of(make_friendship_bracelet_s6, end).
uses(make_friendship_bracelet_s6, scissors).
role(make_friendship_bracelet_s6, scissors, cut).
before(make_friendship_bracelet_s1, make_friendship_bracelet_s2).
before(make_friendship_bracelet_s2, make_friendship_bracelet_s3).
before(make_friendship_bracelet_s3, make_friendship_bracelet_s4).
before(make_friendship_bracelet_s4, make_friendship_bracelet_s5).
before(make_friendship_bracelet_s5, make_friendship_bracelet_s6).
```

## aliases
```datalog
task_alias(make_friendship_bracelet, "friendship bracelets").
task_alias(make_friendship_bracelet, "a string bracelet").
task_alias(make_friendship_bracelet, "knotting a bracelet").
alias(embroidery_thread, "thread").
alias(embroidery_thread, "string").
alias(knot, "the knot").
alias(end, "the ends").
alias(pattern, "the stripes").
```

## teach
- To make a friendship bracelet you need embroidery thread, scissors and tape.
- Making a friendship bracelet takes 45 to 60 minutes.
- Step 1 of making a friendship bracelet: cut the threads to 60 centimetres with the scissors.
- Step 2 of making a friendship bracelet: knot the ends together.
- Step 3 of making a friendship bracelet: tape the knot onto a table.
- Step 4 of making a friendship bracelet: tie knots across the threads about 40 times to build the stripes.
- Step 5 of making a friendship bracelet: tie the ends off.
- Step 6 of making a friendship bracelet: trim the ends with the scissors.

## checklist
- names what is needed: thread, scissors and tape
- gives the steps in order: cut, knot ends, tape down, tie knots, tie off, trim

## variant
- change: no tape: pin the knot to a cushion with a safety pin
- request: I've no tape to hold my bracelet down while I knot it. What else works?
- situation: I have embroidery thread, scissors, a safety pin and a cushion.
- expect: The reply pins the starting knot to a cushion with a safety pin instead of taping it to a table.

```datalog situation
has(user, embroidery_thread).
has(user, scissors).
has(user, safety_pin).
lacks(user, tape).
```

```datalog difference
swap(make_friendship_bracelet_s3, tape, safety_pin).
change(make_friendship_bracelet_s3, onto, table, cushion).
extra_need(safety_pin).
no_need(tape).
```

## variant
- change: five bracelets: cut more thread and allow about five hours
- request: My daughter wants to make bracelets for five friends. How much time and thread will she need?
- situation: I have plenty of embroidery thread, scissors and tape; five bracelets are wanted.
- expect: The reply cuts thread for five bracelets and repeats the knotting for each, about 4 to 5 hours in all.

```datalog situation
has(user, embroidery_thread).
has(user, scissors).
has(user, tape).
count(user, bracelet, 5).
```

```datalog difference
change(make_friendship_bracelet_s1, repeat, 1, 5).
change(make_friendship_bracelet, takes_time, 45_60_minute, 4_5_hour).
```
