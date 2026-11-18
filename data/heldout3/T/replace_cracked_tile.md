---
id: replace_cracked_tile
stage: tools
task: replace_cracked_tile
schema: v3
tier: T
---
# Replace a cracked wall tile

## requests
- One tile above my sink is cracked. How do I swap it for a new one?
- How do you replace a single broken wall tile?
- There's a split tile in the bathroom. Can I change just that one without retiling everything?

## situation
- I have a grout saw, a hammer, a chisel, safety goggles, tile adhesive, a notched trowel, a spare tile, grout and a sponge.

## ground truth
```datalog
task(replace_cracked_tile).
domain(replace_cracked_tile, repair).
goal(replace_cracked_tile, tile_replaced).
needs(replace_cracked_tile, grout_saw).
needs(replace_cracked_tile, hammer).
needs(replace_cracked_tile, chisel).
needs(replace_cracked_tile, safety_goggles).
needs(replace_cracked_tile, tile_adhesive).
needs(replace_cracked_tile, notched_trowel).
needs(replace_cracked_tile, new_tile).
needs(replace_cracked_tile, grout).
needs(replace_cracked_tile, sponge).
takes_time(replace_cracked_tile, 24, 26, hour).
step(replace_cracked_tile, replace_cracked_tile_s1).
action(replace_cracked_tile_s1, remove).
object_of(replace_cracked_tile_s1, grout).
uses(replace_cracked_tile_s1, grout_saw).
role(replace_cracked_tile_s1, grout_saw, cut).
step(replace_cracked_tile, replace_cracked_tile_s2).
action(replace_cracked_tile_s2, break).
object_of(replace_cracked_tile_s2, old_tile).
uses(replace_cracked_tile_s2, hammer).
role(replace_cracked_tile_s2, hammer, strike).
uses(replace_cracked_tile_s2, chisel).
role(replace_cracked_tile_s2, chisel, cut).
uses(replace_cracked_tile_s2, safety_goggles).
role(replace_cracked_tile_s2, safety_goggles, protect).
step(replace_cracked_tile, replace_cracked_tile_s3).
action(replace_cracked_tile_s3, scrape).
object_of(replace_cracked_tile_s3, adhesive).
uses(replace_cracked_tile_s3, chisel).
role(replace_cracked_tile_s3, chisel, lift).
achieves(replace_cracked_tile_s3, flat).
step(replace_cracked_tile, replace_cracked_tile_s4).
action(replace_cracked_tile_s4, spread).
object_of(replace_cracked_tile_s4, tile_adhesive).
onto(replace_cracked_tile_s4, new_tile).
uses(replace_cracked_tile_s4, notched_trowel).
role(replace_cracked_tile_s4, notched_trowel, spread).
step(replace_cracked_tile, replace_cracked_tile_s5).
action(replace_cracked_tile_s5, press).
object_of(replace_cracked_tile_s5, new_tile).
onto(replace_cracked_tile_s5, wall).
takes_time(replace_cracked_tile_s5, 24, 24, hour).
achieves(replace_cracked_tile_s5, adhesive_set).
step(replace_cracked_tile, replace_cracked_tile_s6).
action(replace_cracked_tile_s6, fill).
object_of(replace_cracked_tile_s6, joint).
uses(replace_cracked_tile_s6, grout).
role(replace_cracked_tile_s6, grout, fill).
requires_state(replace_cracked_tile_s6, adhesive_set).
step(replace_cracked_tile, replace_cracked_tile_s7).
action(replace_cracked_tile_s7, wipe).
object_of(replace_cracked_tile_s7, tile).
uses(replace_cracked_tile_s7, sponge).
role(replace_cracked_tile_s7, sponge, wipe).
purpose(replace_cracked_tile_s7, excess_grout_gone).
before(replace_cracked_tile_s1, replace_cracked_tile_s2).
before(replace_cracked_tile_s2, replace_cracked_tile_s3).
before(replace_cracked_tile_s3, replace_cracked_tile_s4).
before(replace_cracked_tile_s4, replace_cracked_tile_s5).
before(replace_cracked_tile_s5, replace_cracked_tile_s6).
before(replace_cracked_tile_s6, replace_cracked_tile_s7).
```

## aliases
```datalog
task_alias(replace_cracked_tile, "replacing a broken tile").
task_alias(replace_cracked_tile, "swapping a cracked tile").
task_alias(replace_cracked_tile, "fixing a cracked tile").
alias(old_tile, "cracked tile").
alias(new_tile, "spare tile").
alias(grout_saw, "grout rake").
alias(safety_goggles, "goggles").
alias(joint, "the gaps around the tile").
alias(adhesive_set, "the adhesive has set").
```

## teach
- To replace a cracked wall tile you need a grout saw, a hammer, a chisel, safety goggles, tile adhesive, a notched trowel, a new tile, grout and a sponge.
- Replacing a cracked wall tile takes 24 to 26 hours, mostly waiting for the adhesive.
- Step 1 of replacing a cracked tile: remove the grout round the tile with the grout saw.
- Step 2 of replacing a cracked tile: break the old tile with the hammer and chisel, wearing safety goggles.
- Step 3 of replacing a cracked tile: scrape the old adhesive off with the chisel until the wall is flat.
- Step 4 of replacing a cracked tile: spread tile adhesive onto the back of the new tile with the notched trowel.
- Step 5 of replacing a cracked tile: press the new tile onto the wall and leave it 24 hours until the adhesive has set.
- Step 6 of replacing a cracked tile: once the adhesive has set, fill the joints with grout.
- Step 7 of replacing a cracked tile: wipe the tile with the sponge to remove extra grout.

## checklist
- names what is needed, including a grout saw, hammer, chisel, goggles, adhesive and a new tile
- gives the steps in order: remove grout, break tile, scrape, spread adhesive, press on, grout, wipe
- says to wear goggles and to wait for the adhesive before grouting

## variant
- change: no notched trowel: spread the adhesive with a putty knife
- request: I don't have a notched trowel for the tile adhesive. Can I use a putty knife?
- situation: I have a grout saw, hammer, chisel, goggles, tile adhesive, a putty knife, a new tile, grout and a sponge.
- expect: The reply spreads the adhesive onto the tile with the putty knife instead of the notched trowel.

```datalog situation
has(user, grout_saw).
has(user, hammer).
has(user, chisel).
has(user, safety_goggles).
has(user, tile_adhesive).
has(user, putty_knife).
has(user, new_tile).
has(user, grout).
has(user, sponge).
lacks(user, notched_trowel).
```

```datalog difference
swap(replace_cracked_tile_s4, notched_trowel, putty_knife).
extra_need(putty_knife).
no_need(notched_trowel).
```

## variant
- change: hammer head came off: tap the chisel with a rubber mallet
- request: The head flew off my hammer. How do I break out the cracked tile now?
- situation: I have a grout saw, a chisel, goggles, adhesive, a trowel, a new tile, grout, a sponge and a rubber mallet; my hammer is broken.
- expect: The reply taps the chisel with the rubber mallet instead of the hammer to break the old tile.

```datalog situation
has(user, grout_saw).
has(user, chisel).
has(user, safety_goggles).
has(user, tile_adhesive).
has(user, notched_trowel).
has(user, new_tile).
has(user, grout).
has(user, sponge).
has(user, rubber_mallet).
lacks(user, hammer).
```

```datalog difference
swap(replace_cracked_tile_s2, hammer, rubber_mallet).
extra_need(rubber_mallet).
no_need(hammer).
```
