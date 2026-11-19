---
id: replace_bicycle_chain
stage: procedures
task: replace_bicycle_chain
schema: v3
---
# Replace a bicycle chain

## requests
- My bike chain keeps slipping, so I bought a new one. How do I fit it?
- What's the process for changing a bicycle chain at home?
- Can you tell me how to put a replacement chain on my bike, step by step?

## situation
- I have a new chain, a chain breaker tool, a quick link and chain oil.
- I have a free afternoon and no bike shop nearby.

## ground truth
```datalog
task(replace_bicycle_chain).
domain(replace_bicycle_chain, repair).
goal(replace_bicycle_chain, fix_slipping_chain).
needs(replace_bicycle_chain, new_chain).
needs(replace_bicycle_chain, chain_tool).
needs(replace_bicycle_chain, quick_link).
needs(replace_bicycle_chain, chain_oil).
takes_time(replace_bicycle_chain, 30, 45, minute).
step(replace_bicycle_chain, replace_bicycle_chain_s1).
action(replace_bicycle_chain_s1, shift).
object_of(replace_bicycle_chain_s1, chain).
step(replace_bicycle_chain, replace_bicycle_chain_s2).
action(replace_bicycle_chain_s2, break).
object_of(replace_bicycle_chain_s2, chain).
uses(replace_bicycle_chain_s2, chain_tool).
role(replace_bicycle_chain_s2, chain_tool, pierce).
warning(replace_bicycle_chain_s2, pin_bent).
step(replace_bicycle_chain, replace_bicycle_chain_s3).
action(replace_bicycle_chain_s3, remove).
object_of(replace_bicycle_chain_s3, chain).
step(replace_bicycle_chain, replace_bicycle_chain_s4).
action(replace_bicycle_chain_s4, thread).
object_of(replace_bicycle_chain_s4, new_chain).
step(replace_bicycle_chain, replace_bicycle_chain_s5).
action(replace_bicycle_chain_s5, join).
object_of(replace_bicycle_chain_s5, new_chain).
uses(replace_bicycle_chain_s5, quick_link).
role(replace_bicycle_chain_s5, quick_link, fasten).
warning(replace_bicycle_chain_s5, link_not_seated).
step(replace_bicycle_chain, replace_bicycle_chain_s6).
action(replace_bicycle_chain_s6, lubricate).
object_of(replace_bicycle_chain_s6, new_chain).
uses(replace_bicycle_chain_s6, chain_oil).
role(replace_bicycle_chain_s6, chain_oil, coat).
check(replace_bicycle_chain_s6, chain_moves_freely).
before(replace_bicycle_chain_s1, replace_bicycle_chain_s2).
before(replace_bicycle_chain_s2, replace_bicycle_chain_s3).
before(replace_bicycle_chain_s3, replace_bicycle_chain_s4).
before(replace_bicycle_chain_s4, replace_bicycle_chain_s5).
before(replace_bicycle_chain_s5, replace_bicycle_chain_s6).
```

## aliases
```datalog
task_alias(replace_bicycle_chain, "change bike chain").
task_alias(replace_bicycle_chain, "fit a new bike chain").
task_alias(replace_bicycle_chain, "replace a bicycle chain").
alias(new_chain, "replacement chain").
alias(chain_tool, "chain breaker").
alias(quick_link, "master link").
alias(chain_oil, "lube").
alias(link_not_seated, "link not clicked in").
alias(chain_moves_freely, "moves freely").
```

## teach
- To replace a bicycle chain you need a new chain, a chain breaker tool, a quick link and chain oil.
- Replacing a bicycle chain takes 30 to 45 minutes.
- Step 1 of replacing a bicycle chain: shift the chain onto the smallest rear sprocket.
- Step 2 of replacing a bicycle chain: break the old chain with the chain breaker tool by pushing out a pin, without bending the pin.
- Step 3 of replacing a bicycle chain: remove the old chain from the bike.
- Step 4 of replacing a bicycle chain: thread the new chain through the derailleur and around the gears.
- Step 5 of replacing a bicycle chain: join the two ends with the quick link, making sure it clicks fully into place.
- Step 6 of replacing a bicycle chain: lubricate the new chain with chain oil so it moves freely.

## checklist
- names what is needed, including a new chain, a chain breaker, a quick link and chain oil
- gives the steps in order: shift to smallest sprocket, break old chain, remove it, thread new chain, join with quick link, lubricate
- warns not to bend the pin when breaking the chain and to check the quick link is fully clicked in
- gives a total time of about 30 to 45 minutes

## variant
- change: the old chain is already off the bike, so skip breaking and removing it; just put the new chain on, join it and oil it, in about 15 to 20 minutes
- request: My old chain already came off, I only need to put the new one on. I don't have a chain breaker. How do I do that?
- situation: I have a new chain, a quick link and chain oil, but no chain breaker, and the old chain is already off.
- expect: The reply skips breaking and removing the old chain, threads the new chain on, joins it with the quick link and oils it, in about 15 to 20 minutes.

```datalog situation
has(user, new_chain).
has(user, quick_link).
has(user, chain_oil).
lacks(user, chain_tool).
```

```datalog difference
drop(replace_bicycle_chain_s2).
drop(replace_bicycle_chain_s3).
no_need(chain_tool).
change(replace_bicycle_chain, takes_time, 30_45, 15_20).
```

## variant
- change: while the old chain is off, wipe the rear cassette clean with a rag before threading the new chain on
- request: I want to clean the back gears while the chain is off. How do I replace the chain then?
- situation: I have a rag, a new chain, a chain breaker, a quick link and chain oil, and I want the rear cassette cleaned.
- expect: The reply adds a step after the old chain is removed to wipe the rear cassette with a rag, then threads the new chain and continues as usual.

```datalog situation
has(user, rag).
has(user, new_chain).
has(user, chain_tool).
has(user, quick_link).
has(user, chain_oil).
wants(user, clean_cassette).
```

```datalog difference
add_step(replace_bicycle_chain_s9, replace_bicycle_chain_s3).
step(replace_bicycle_chain, replace_bicycle_chain_s9).
action(replace_bicycle_chain_s9, clean).
object_of(replace_bicycle_chain_s9, cassette).
uses(replace_bicycle_chain_s9, rag).
role(replace_bicycle_chain_s9, rag, wipe).
before(replace_bicycle_chain_s3, replace_bicycle_chain_s9).
before(replace_bicycle_chain_s9, replace_bicycle_chain_s4).
extra_need(rag).
```
