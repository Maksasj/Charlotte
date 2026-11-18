---
id: hem_trousers
stage: tools
task: hem_trousers
schema: v3
tier: T
---
# Hem a pair of trousers

## requests
- My new trousers are too long and drag on the floor. Can I shorten them myself?
- How do I take up the hem on a pair of trousers by hand?
- What's the method for hemming trousers without a sewing machine?

## situation
- I have tailor's chalk, scissors, an iron, a needle and matching thread.

## ground truth
```datalog
task(hem_trousers).
domain(hem_trousers, crafts).
goal(hem_trousers, shorter_trousers).
needs(hem_trousers, chalk).
needs(hem_trousers, scissors).
needs(hem_trousers, iron).
needs(hem_trousers, needle).
needs(hem_trousers, thread).
takes_time(hem_trousers, 60, 90, minute).
step(hem_trousers, hem_trousers_s1).
action(hem_trousers_s1, mark).
object_of(hem_trousers_s1, hem).
uses(hem_trousers_s1, chalk).
role(hem_trousers_s1, chalk, mark).
step(hem_trousers, hem_trousers_s2).
action(hem_trousers_s2, cut).
object_of(hem_trousers_s2, trousers).
uses(hem_trousers_s2, scissors).
role(hem_trousers_s2, scissors, cut).
setting(hem_trousers_s2, allowance, 4, centimetre).
step(hem_trousers, hem_trousers_s3).
action(hem_trousers_s3, fold).
object_of(hem_trousers_s3, hem).
step(hem_trousers, hem_trousers_s4).
action(hem_trousers_s4, press).
object_of(hem_trousers_s4, hem).
uses(hem_trousers_s4, iron).
role(hem_trousers_s4, iron, press).
step(hem_trousers, hem_trousers_s5).
action(hem_trousers_s5, sew).
object_of(hem_trousers_s5, hem).
uses(hem_trousers_s5, needle).
role(hem_trousers_s5, needle, pierce).
uses(hem_trousers_s5, thread).
role(hem_trousers_s5, thread, fasten).
step(hem_trousers, hem_trousers_s6).
action(hem_trousers_s6, check).
object_of(hem_trousers_s6, trousers).
check(hem_trousers_s6, right_length).
before(hem_trousers_s1, hem_trousers_s2).
before(hem_trousers_s2, hem_trousers_s3).
before(hem_trousers_s3, hem_trousers_s4).
before(hem_trousers_s4, hem_trousers_s5).
before(hem_trousers_s5, hem_trousers_s6).
```

## aliases
```datalog
task_alias(hem_trousers, "hemming trousers").
task_alias(hem_trousers, "shortening trousers").
task_alias(hem_trousers, "taking up trousers").
alias(chalk, "tailor's chalk").
alias(hem, "the hem").
alias(trousers, "the leg").
alias(right_length, "the right length").
alias(allowance, "a 4 centimetre allowance").
```

## teach
- To hem a pair of trousers you need chalk, scissors, an iron, a needle and thread.
- Hemming a pair of trousers takes 60 to 90 minutes.
- Step 1 of hemming trousers: mark the new hem with the chalk.
- Step 2 of hemming trousers: cut the trousers below the mark, leaving a 4 centimetre allowance, with the scissors.
- Step 3 of hemming trousers: fold the hem up.
- Step 4 of hemming trousers: press the hem with the iron.
- Step 5 of hemming trousers: sew the hem with the needle and thread.
- Step 6 of hemming trousers: try them on and check they are the right length.

## checklist
- names what is needed: chalk, scissors, an iron, a needle and thread
- gives the steps in order: mark, cut with allowance, fold, press, sew, check

## variant
- change: no chalk: mark the line with a thin sliver of soap
- request: I don't have tailor's chalk to mark my trousers. What can I use?
- situation: I have a dry sliver of soap, scissors, an iron, a needle and thread.
- expect: The reply marks the hem line with the edge of the soap sliver instead of chalk; it washes out.

```datalog situation
has(user, soap_bar).
has(user, scissors).
has(user, iron).
has(user, needle).
has(user, thread).
lacks(user, chalk).
```

```datalog difference
swap(hem_trousers_s1, chalk, soap_bar).
extra_need(soap_bar).
no_need(chalk).
```

## variant
- change: iron broken: press the fold with a hair straightener
- request: My iron's stopped heating up. How can I press the hem before sewing?
- situation: I have chalk, scissors, a hair straightener, a needle and thread; the iron is broken.
- expect: The reply presses the folded hem with a hair straightener instead of the iron.

```datalog situation
has(user, chalk).
has(user, scissors).
has(user, hair_straightener).
has(user, needle).
has(user, thread).
lacks(user, iron).
```

```datalog difference
swap(hem_trousers_s4, iron, hair_straightener).
extra_need(hair_straightener).
no_need(iron).
```
