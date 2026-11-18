---
id: make_candle
stage: tools
task: make_candle
schema: v3
tier: T
---
# Make a jar candle

## requests
- How do I make my own scented candle in a jar?
- I'd like to pour soy wax candles as Christmas presents. What's the process?
- Can you walk me through making a candle at home?

## situation
- I have soy wax flakes, a pot of water, a heatproof jug, a wick, glue dots, a glass jar, fragrance oil, a clothes peg and scissors.

## ground truth
```datalog
task(make_candle).
domain(make_candle, crafts).
goal(make_candle, candle).
goal(make_candle, gift).
needs(make_candle, soy_wax).
needs(make_candle, pot).
needs(make_candle, heatproof_jug).
needs(make_candle, wick).
needs(make_candle, glue_dot).
needs(make_candle, jar).
needs(make_candle, fragrance_oil).
needs(make_candle, clothes_peg).
needs(make_candle, scissors).
takes_time(make_candle, 24, 25, hour).
step(make_candle, make_candle_s1).
action(make_candle_s1, melt).
object_of(make_candle_s1, soy_wax).
uses(make_candle_s1, heatproof_jug).
role(make_candle_s1, heatproof_jug, contain).
uses(make_candle_s1, pot).
role(make_candle_s1, pot, heat).
takes_time(make_candle_s1, 10, 15, minute).
step(make_candle, make_candle_s2).
action(make_candle_s2, stick).
object_of(make_candle_s2, wick).
onto(make_candle_s2, jar).
uses(make_candle_s2, glue_dot).
role(make_candle_s2, glue_dot, fasten).
overlaps(make_candle_s2, make_candle_s1).
step(make_candle, make_candle_s3).
action(make_candle_s3, add).
object_of(make_candle_s3, fragrance_oil).
into(make_candle_s3, soy_wax).
amount(make_candle_s3, fragrance_oil, 1, teaspoon).
step(make_candle, make_candle_s4).
action(make_candle_s4, pour).
object_of(make_candle_s4, soy_wax).
into(make_candle_s4, jar).
step(make_candle, make_candle_s5).
action(make_candle_s5, hold).
object_of(make_candle_s5, wick).
uses(make_candle_s5, clothes_peg).
role(make_candle_s5, clothes_peg, hold).
purpose(make_candle_s5, wick_centred).
step(make_candle, make_candle_s6).
action(make_candle_s6, cool).
object_of(make_candle_s6, candle).
takes_time(make_candle_s6, 24, 24, hour).
avoid(make_candle_s6, move, jar).
step(make_candle, make_candle_s7).
action(make_candle_s7, trim).
object_of(make_candle_s7, wick).
uses(make_candle_s7, scissors).
role(make_candle_s7, scissors, cut).
setting(make_candle_s7, length, 1, centimetre).
before(make_candle_s1, make_candle_s3).
before(make_candle_s2, make_candle_s4).
before(make_candle_s3, make_candle_s4).
before(make_candle_s4, make_candle_s5).
before(make_candle_s5, make_candle_s6).
before(make_candle_s6, make_candle_s7).
```

## aliases
```datalog
task_alias(make_candle, "making a candle").
task_alias(make_candle, "a homemade candle").
task_alias(make_candle, "pouring a jar candle").
alias(soy_wax, "soy wax").
alias(soy_wax, "wax").
alias(heatproof_jug, "jug").
alias(pot, "pot of simmering water").
alias(glue_dot, "glue dots").
alias(wick_centred, "it stays in the middle").
```

## teach
- To make a jar candle you need soy wax, a pot, a heatproof jug, a wick, glue dots, a jar, fragrance oil, a clothes peg and scissors.
- Making a jar candle takes 24 to 25 hours, mostly cooling.
- Step 1 of making a jar candle: melt the soy wax in the heatproof jug set in a pot of simmering water, for 10 to 15 minutes.
- While step 1 runs, step 2 of making a jar candle: stick the wick onto the bottom of the jar with a glue dot.
- Step 3 of making a jar candle: add 1 teaspoon of fragrance oil to the melted wax.
- Step 4 of making a jar candle: pour the wax into the jar.
- Step 5 of making a jar candle: hold the wick with the clothes peg so it stays in the middle.
- Step 6 of making a jar candle: let the candle cool for 24 hours; do not move the jar.
- Step 7 of making a jar candle: trim the wick to 1 centimetre with the scissors.

## checklist
- names what is needed, including wax, a wick, a jar and fragrance oil
- gives the steps in order: melt wax, stick wick, add fragrance, pour, centre wick, cool, trim
- says the wick can be fixed while the wax melts

## variant
- change: no wax at all: a candle cannot be made
- request: I want to make a candle today but I have no wax or old candles to melt. Can I do it with what I have?
- situation: I have a pot, a jug, a wick, glue dots, a jar, fragrance oil, a peg and scissors, but no wax.
- expect: The reply says a candle cannot be made without wax and suggests buying soy wax or reusing old candle stubs.

```datalog situation
has(user, pot).
has(user, heatproof_jug).
has(user, wick).
has(user, glue_dot).
has(user, jar).
has(user, fragrance_oil).
has(user, clothes_peg).
has(user, scissors).
lacks(user, soy_wax).
```

```datalog difference
cannot(no_wax).
```

## variant
- change: asthma, no scent: leave out the fragrance oil
- request: My partner has asthma and can't stand perfumes. How do I make an unscented candle?
- situation: I have everything for the candle; we want no fragrance.
- expect: The reply leaves out the fragrance oil step and pours the plain melted wax.

```datalog situation
has(user, soy_wax).
has(user, pot).
has(user, heatproof_jug).
has(user, wick).
has(user, glue_dot).
has(user, jar).
has(user, clothes_peg).
has(user, scissors).
excludes(user, fragrance).
```

```datalog difference
drop(make_candle_s3).
no_need(fragrance_oil).
```
