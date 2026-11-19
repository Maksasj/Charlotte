---
id: clean_light_fixture
stage: procedures
task: clean_light_fixture
schema: v3
---
# Clean a light fixture

## requests
- What's the way to clean a light fixture without damaging it?
- Can you tell me how to wipe down a ceiling light and put the shade back?
- I need to clean the glass shade on my hallway light. What are the steps?

## situation
- I have a stepladder, soapy water and a cloth.
- I have about half an hour.

## ground truth
```datalog
task(clean_light_fixture).
domain(clean_light_fixture, cleaning).
goal(clean_light_fixture, clean_light).
needs(clean_light_fixture, stepladder).
needs(clean_light_fixture, soapy_water).
needs(clean_light_fixture, cloth).
takes_time(clean_light_fixture, 20, 30, minute).
step(clean_light_fixture, clean_light_fixture_s1).
action(clean_light_fixture_s1, switch_off).
object_of(clean_light_fixture_s1, light_fixture).
step(clean_light_fixture, clean_light_fixture_s2).
action(clean_light_fixture_s2, remove).
object_of(clean_light_fixture_s2, glass_shade).
uses(clean_light_fixture_s2, stepladder).
role(clean_light_fixture_s2, stepladder, lift).
step(clean_light_fixture, clean_light_fixture_s3).
action(clean_light_fixture_s3, wash).
object_of(clean_light_fixture_s3, glass_shade).
uses(clean_light_fixture_s3, soapy_water).
role(clean_light_fixture_s3, soapy_water, clean).
step(clean_light_fixture, clean_light_fixture_s4).
action(clean_light_fixture_s4, dry).
object_of(clean_light_fixture_s4, glass_shade).
uses(clean_light_fixture_s4, cloth).
role(clean_light_fixture_s4, cloth, dry).
purpose(clean_light_fixture_s4, no_streaks).
step(clean_light_fixture, clean_light_fixture_s5).
action(clean_light_fixture_s5, refit).
object_of(clean_light_fixture_s5, glass_shade).
step(clean_light_fixture, clean_light_fixture_s6).
action(clean_light_fixture_s6, switch_on).
object_of(clean_light_fixture_s6, light_fixture).
before(clean_light_fixture_s1, clean_light_fixture_s2).
before(clean_light_fixture_s2, clean_light_fixture_s3).
before(clean_light_fixture_s3, clean_light_fixture_s4).
before(clean_light_fixture_s4, clean_light_fixture_s5).
before(clean_light_fixture_s5, clean_light_fixture_s6).
```

## aliases
```datalog
task_alias(clean_light_fixture, "clean a light fixture").
task_alias(clean_light_fixture, "clean a ceiling light").
alias(stepladder, "step ladder").
alias(soapy_water, "soapy water").
alias(glass_shade, "glass shade").
alias(light_fixture, "light fixture").
alias(switch_off, "turn off").
alias(switch_on, "turn back on").
alias(no_streaks, "no streaks").
```

## teach
- To clean a light fixture you need a stepladder, soapy water and a cloth.
- Cleaning a light fixture takes 20 to 30 minutes.
- Step 1 of cleaning a light fixture: switch off the light at the wall switch.
- Step 2 of cleaning a light fixture: stand on the stepladder and remove the glass shade.
- Step 3 of cleaning a light fixture: wash the glass shade in soapy water.
- Step 4 of cleaning a light fixture: dry the glass shade with the cloth so there are no streaks.
- Step 5 of cleaning a light fixture: refit the glass shade.
- Step 6 of cleaning a light fixture: switch the light back on.

## checklist
- names the stepladder, soapy water and cloth
- gives the steps in order: switch off, remove the shade, wash it, dry it, refit it, switch on
- gives a total time of about 20 to 30 minutes
- says the shade is dried with the cloth so it has no streaks

## variant
- change: no cloth: dry the glass shade with paper towel instead
- request: I want to clean my light fixture but I don't have a cloth. Can I use something else?
- situation: I have a stepladder, soapy water, paper towel and about 30 minutes, but no cloth.
- expect: The reply keeps the same steps and says to dry the glass shade with paper towel in place of the cloth.

```datalog situation
lacks(user, cloth).
has(user, stepladder).
has(user, soapy_water).
has(user, paper_towel).
time_limit(user, 30, minute).
```

```datalog difference
swap(clean_light_fixture_s4, cloth, paper_towel).
extra_need(paper_towel).
no_need(cloth).
```

## variant
- change: also wipe the fixture body above the bulb before drying the shade
- request: I'd like to clean the whole light fixture, including the metal part above the shade. What do I do?
- situation: I have a stepladder, soapy water and a cloth, and I want the fixture body cleaned too.
- expect: The reply adds a step after washing to wipe the fixture body with the cloth, then continues with drying and refitting.

```datalog situation
has(user, stepladder).
has(user, soapy_water).
has(user, cloth).
wants(user, fixture_body).
```

```datalog difference
add_step(clean_light_fixture_s9, clean_light_fixture_s3).
step(clean_light_fixture, clean_light_fixture_s9).
action(clean_light_fixture_s9, wipe).
object_of(clean_light_fixture_s9, fixture_body).
uses(clean_light_fixture_s9, cloth).
role(clean_light_fixture_s9, cloth, wipe).
before(clean_light_fixture_s3, clean_light_fixture_s9).
before(clean_light_fixture_s9, clean_light_fixture_s4).
```
