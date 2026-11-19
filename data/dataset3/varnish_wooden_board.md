---
id: varnish_wooden_board
stage: procedures
task: varnish_wooden_board
schema: v3
---
# Varnish a wooden board

## requests
- How do I varnish a wooden board without it going streaky?
- What's the process for putting varnish on a wooden board?
- Can you give me the steps for varnishing a cutting board I made?

## situation
- I have a wooden board, sandpaper, a lint-free cloth, a paintbrush and a tin of varnish.
- I can leave it to dry overnight.

## ground truth
```datalog
task(varnish_wooden_board).
domain(varnish_wooden_board, crafts).
goal(varnish_wooden_board, protected_board).
needs(varnish_wooden_board, wooden_board).
needs(varnish_wooden_board, sandpaper).
needs(varnish_wooden_board, cloth).
needs(varnish_wooden_board, varnish).
needs(varnish_wooden_board, brush).
takes_time(varnish_wooden_board, 260, 300, minute).
step(varnish_wooden_board, varnish_wooden_board_s1).
action(varnish_wooden_board_s1, sand).
object_of(varnish_wooden_board_s1, wooden_board).
uses(varnish_wooden_board_s1, sandpaper).
role(varnish_wooden_board_s1, sandpaper, scrub).
purpose(varnish_wooden_board_s1, smooth_surface).
takes_time(varnish_wooden_board_s1, 10, 15, minute).
step(varnish_wooden_board, varnish_wooden_board_s2).
action(varnish_wooden_board_s2, wipe).
object_of(varnish_wooden_board_s2, wooden_board).
uses(varnish_wooden_board_s2, cloth).
role(varnish_wooden_board_s2, cloth, wipe).
check(varnish_wooden_board_s2, no_dust).
takes_time(varnish_wooden_board_s2, 2, 5, minute).
step(varnish_wooden_board, varnish_wooden_board_s3).
action(varnish_wooden_board_s3, coat).
object_of(varnish_wooden_board_s3, wooden_board).
uses(varnish_wooden_board_s3, varnish).
role(varnish_wooden_board_s3, varnish, coat).
uses(varnish_wooden_board_s3, brush).
role(varnish_wooden_board_s3, brush, spread).
takes_time(varnish_wooden_board_s3, 15, 20, minute).
step(varnish_wooden_board, varnish_wooden_board_s4).
action(varnish_wooden_board_s4, dry).
object_of(varnish_wooden_board_s4, wooden_board).
until(varnish_wooden_board_s4, dry_to_touch).
takes_time(varnish_wooden_board_s4, 240, 240, minute).
before(varnish_wooden_board_s1, varnish_wooden_board_s2).
before(varnish_wooden_board_s2, varnish_wooden_board_s3).
before(varnish_wooden_board_s3, varnish_wooden_board_s4).
```

## aliases
```datalog
task_alias(varnish_wooden_board, "varnishing a wooden board").
task_alias(varnish_wooden_board, "varnishing a board").
task_alias(varnish_wooden_board, "varnishing a cutting board").
alias(wooden_board, "wooden board").
alias(wooden_board, "cutting board").
alias(sandpaper, "sandpaper").
alias(cloth, "lint-free cloth").
alias(varnish, "tin of varnish").
alias(brush, "paintbrush").
alias(coat, "put on a coat").
alias(dry_to_touch, "dry to the touch").
alias(smooth_surface, "smooth").
alias(no_dust, "free of dust").
```

## teach
- To varnish a wooden board you need a wooden board, sandpaper, a lint-free cloth, varnish and a paintbrush.
- Varnishing a wooden board takes 260 to 300 minutes.
- Step 1 of varnishing a wooden board: sand the board with sandpaper along the grain to make it smooth.
- Step 2 of varnishing a wooden board: wipe the board with a clean lint-free cloth to remove the dust.
- Step 3 of varnishing a wooden board: put on a coat of varnish with the paintbrush in thin, even strokes.
- Step 4 of varnishing a wooden board: leave the board to dry until it is dry to the touch, for 4 hours.

## checklist
- names what is needed, including a wooden board, sandpaper, a cloth, varnish and a paintbrush
- gives the steps in order: sand, wipe off the dust, coat with varnish, dry
- says the board must be free of dust before varnishing
- gives a total time of about 260 to 300 minutes, most of it drying

## variant
- change: no sandpaper: use fine steel wool to smooth the board instead
- request: I don't have any sandpaper. Can I use steel wool to smooth my board before varnishing?
- situation: I have a wooden board, fine steel wool, a lint-free cloth, a paintbrush and a tin of varnish.
- expect: The reply swaps the sandpaper for fine steel wool in step 1, rubbing along the grain, and keeps the other steps the same.

```datalog situation
lacks(user, sandpaper).
has(user, wooden_board).
has(user, steel_wool).
has(user, cloth).
has(user, brush).
has(user, varnish).
```

```datalog difference
swap(varnish_wooden_board_s1, sandpaper, steel_wool).
extra_need(steel_wool).
no_need(sandpaper).
```

## variant
- change: second coat for extra protection, with a further four hours of drying
- request: I'd like the board to have extra protection, so can it get a second coat of varnish?
- situation: I have everything from the base list and I want a second coat for extra protection.
- expect: The reply keeps the base steps, then after the first drying adds a second coat with the paintbrush and a further 4 hours of drying, about 515 to 560 minutes in total.

```datalog situation
has(user, wooden_board).
has(user, sandpaper).
has(user, cloth).
has(user, brush).
has(user, varnish).
wants(user, second_coat).
```

```datalog difference
add_step(varnish_wooden_board_s9, varnish_wooden_board_s4).
add_step(varnish_wooden_board_s10, varnish_wooden_board_s9).
step(varnish_wooden_board, varnish_wooden_board_s9).
action(varnish_wooden_board_s9, coat).
object_of(varnish_wooden_board_s9, wooden_board).
uses(varnish_wooden_board_s9, varnish).
role(varnish_wooden_board_s9, varnish, coat).
uses(varnish_wooden_board_s9, brush).
role(varnish_wooden_board_s9, brush, spread).
takes_time(varnish_wooden_board_s9, 15, 20, minute).
purpose(varnish_wooden_board_s9, extra_protection).
step(varnish_wooden_board, varnish_wooden_board_s10).
action(varnish_wooden_board_s10, dry).
object_of(varnish_wooden_board_s10, wooden_board).
until(varnish_wooden_board_s10, dry_to_touch).
takes_time(varnish_wooden_board_s10, 240, 240, minute).
before(varnish_wooden_board_s4, varnish_wooden_board_s9).
before(varnish_wooden_board_s9, varnish_wooden_board_s10).
change(varnish_wooden_board, takes_time, 260_300, 515_560).
```
