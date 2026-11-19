---
id: temper_chocolate
stage: procedures
task: temper_chocolate
schema: v3
---
# Temper chocolate

## requests
- How do I temper chocolate at home so it snaps and shines?
- I want to temper dark chocolate for dipping truffles. What are the steps, and what temperatures?
- Can you explain the tempering method for chocolate? I'm not sure about the temperatures.

## situation
- I have a bar of dark chocolate, a knife, a small saucepan, a heatproof bowl, a wooden spoon and a thermometer.
- I have about 35 minutes.

## ground truth
```datalog
task(temper_chocolate).
domain(temper_chocolate, cooking).
goal(temper_chocolate, tempered_chocolate).
needs(temper_chocolate, chocolate).
needs(temper_chocolate, knife).
needs(temper_chocolate, saucepan).
needs(temper_chocolate, heatproof_bowl).
needs(temper_chocolate, spoon).
needs(temper_chocolate, thermometer).
takes_time(temper_chocolate, 25, 35, minute).
step(temper_chocolate, temper_chocolate_s1).
action(temper_chocolate_s1, chop).
object_of(temper_chocolate_s1, chocolate).
uses(temper_chocolate_s1, knife).
role(temper_chocolate_s1, knife, cut).
amount(temper_chocolate_s1, chocolate, 200, gram).
step(temper_chocolate, temper_chocolate_s2).
action(temper_chocolate_s2, heat).
object_of(temper_chocolate_s2, chocolate).
uses(temper_chocolate_s2, saucepan).
role(temper_chocolate_s2, saucepan, heat).
uses(temper_chocolate_s2, heatproof_bowl).
role(temper_chocolate_s2, heatproof_bowl, contain).
setting(temper_chocolate_s2, temperature, 45, celsius).
takes_time(temper_chocolate_s2, 10, 15, minute).
warning(temper_chocolate_s2, water_in_chocolate).
step(temper_chocolate, temper_chocolate_s3).
action(temper_chocolate_s3, stir).
object_of(temper_chocolate_s3, chocolate).
uses(temper_chocolate_s3, spoon).
role(temper_chocolate_s3, spoon, stir).
step(temper_chocolate, temper_chocolate_s4).
action(temper_chocolate_s4, cool).
object_of(temper_chocolate_s4, chocolate).
setting(temper_chocolate_s4, temperature, 27, celsius).
takes_time(temper_chocolate_s4, 5, 10, minute).
purpose(temper_chocolate_s4, stable_crystals).
step(temper_chocolate, temper_chocolate_s5).
action(temper_chocolate_s5, measure).
object_of(temper_chocolate_s5, chocolate).
uses(temper_chocolate_s5, thermometer).
role(temper_chocolate_s5, thermometer, measure).
check(temper_chocolate_s5, glossy).
overlaps(temper_chocolate_s1, temper_chocolate_s2).
before(temper_chocolate_s1, temper_chocolate_s3).
before(temper_chocolate_s2, temper_chocolate_s3).
before(temper_chocolate_s3, temper_chocolate_s4).
before(temper_chocolate_s4, temper_chocolate_s5).
```

## aliases
```datalog
task_alias(temper_chocolate, "tempering chocolate").
task_alias(temper_chocolate, "chocolate tempering").
alias(chocolate, "dark chocolate").
alias(saucepan, "pan").
alias(heatproof_bowl, "heatproof bowl").
alias(thermometer, "probe thermometer").
alias(glossy, "shiny").
alias(water_in_chocolate, "water getting in").
alias(stable_crystals, "stable crystals").
```

## teach
- To temper chocolate you need chocolate, a knife, a saucepan, a heatproof bowl, a spoon and a thermometer.
- Tempering chocolate takes 25 to 35 minutes.
- While step 2 runs, step 1 of temper chocolate: chop 200 grams of the chocolate into small pieces with the knife.
- Step 2 of temper chocolate: heat two thirds of the chocolate in the heatproof bowl over the saucepan of simmering water to 45 degrees, for 10 to 15 minutes, keeping water out of it so it does not seize.
- Step 3 of temper chocolate: stir the remaining chopped chocolate into the melted chocolate with the spoon until it melts.
- Step 4 of temper chocolate: cool the chocolate to 27 degrees, for 5 to 10 minutes, so the crystals set stable.
- Step 5 of temper chocolate: measure the chocolate with the thermometer and check that it is glossy.

## checklist
- names what is needed, including chocolate, a knife, a saucepan, a heatproof bowl, a spoon and a thermometer
- gives the steps in order: chop, melt to 45 degrees, stir in the rest, cool to 27 degrees, check it is glossy
- says water must be kept out of the chocolate
- gives a total time of about 25 to 35 minutes

## variant
- change: the chocolate is already in chips, so skip the chopping
- request: I have ready chocolate chips and no knife. How do I temper them?
- situation: I have chocolate chips, a saucepan, a heatproof bowl, a spoon and a thermometer, and no knife.
- expect: The reply skips the chopping, melts the chips to 45 degrees, stirs, cools to 27 degrees and checks the gloss.

```datalog situation
has(user, chocolate).
has(user, saucepan).
has(user, heatproof_bowl).
has(user, spoon).
has(user, thermometer).
lacks(user, knife).
```

```datalog difference
drop(temper_chocolate_s1).
no_need(knife).
```

## variant
- change: only a microwave, no saucepan, and 15 minutes: melt the chocolate in short bursts in the microwave
- request: I have only about 15 minutes and no saucepan, just a microwave. Can I still temper chocolate?
- situation: I have chocolate, a knife, a microwave, a heatproof bowl, a spoon and a thermometer, but no saucepan, and 15 minutes.
- expect: The reply melts the chocolate in the microwave bowl in short bursts to 45 degrees for 4 to 6 minutes, then stirs, cools to 27 degrees and checks the gloss, in about 15 to 20 minutes.

```datalog situation
has(user, chocolate).
has(user, knife).
has(user, microwave).
has(user, heatproof_bowl).
has(user, spoon).
has(user, thermometer).
lacks(user, saucepan).
time_limit(user, 15, minute).
```

```datalog difference
swap(temper_chocolate_s2, saucepan, microwave).
no_need(saucepan).
extra_need(microwave).
change(temper_chocolate_s2, takes_time, 10_15, 4_6).
change(temper_chocolate, takes_time, 25_35, 15_20).
```
