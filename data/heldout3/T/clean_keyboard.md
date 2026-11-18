---
id: clean_keyboard
stage: household
task: clean_keyboard
schema: v3
tier: T
---
# Clean a computer keyboard

## requests
- My keyboard is full of crumbs and the keys feel sticky. How do I clean it?
- What's the safe way to clean a laptop keyboard?
- Some of my keys are greasy and grimy. How do I get them clean without damaging anything?

## situation
- I have a can of compressed air, a microfibre cloth and rubbing alcohol.

## ground truth
```datalog
task(clean_keyboard).
domain(clean_keyboard, cleaning).
goal(clean_keyboard, clean_keyboard).
needs(clean_keyboard, compressed_air).
needs(clean_keyboard, cloth).
needs(clean_keyboard, rubbing_alcohol).
takes_time(clean_keyboard, 10, 15, minute).
step(clean_keyboard, clean_keyboard_s1).
action(clean_keyboard_s1, unplug).
object_of(clean_keyboard_s1, keyboard).
purpose(clean_keyboard_s1, safe).
step(clean_keyboard, clean_keyboard_s2).
action(clean_keyboard_s2, shake).
object_of(clean_keyboard_s2, keyboard).
purpose(clean_keyboard_s2, crumbs_out).
step(clean_keyboard, clean_keyboard_s3).
action(clean_keyboard_s3, blow).
object_of(clean_keyboard_s3, keyboard).
with_part(clean_keyboard_s3, gap).
uses(clean_keyboard_s3, compressed_air).
role(clean_keyboard_s3, compressed_air, clean).
step(clean_keyboard, clean_keyboard_s4).
action(clean_keyboard_s4, wipe).
object_of(clean_keyboard_s4, keyboard).
with_part(clean_keyboard_s4, key).
uses(clean_keyboard_s4, cloth).
role(clean_keyboard_s4, cloth, wipe).
uses(clean_keyboard_s4, rubbing_alcohol).
role(clean_keyboard_s4, rubbing_alcohol, clean).
avoid(clean_keyboard_s4, soak, cloth).
step(clean_keyboard, clean_keyboard_s5).
action(clean_keyboard_s5, dry).
object_of(clean_keyboard_s5, keyboard).
takes_time(clean_keyboard_s5, 2, 3, minute).
check(clean_keyboard_s5, dry).
before(clean_keyboard_s1, clean_keyboard_s2).
before(clean_keyboard_s2, clean_keyboard_s3).
before(clean_keyboard_s3, clean_keyboard_s4).
before(clean_keyboard_s4, clean_keyboard_s5).
```

## aliases
```datalog
task_alias(clean_keyboard, "cleaning a keyboard").
task_alias(clean_keyboard, "getting crumbs out of a keyboard").
task_alias(clean_keyboard, "cleaning computer keys").
alias(compressed_air, "can of air").
alias(cloth, "microfibre cloth").
alias(rubbing_alcohol, "isopropyl alcohol").
alias(gap, "between the keys").
alias(shake, "turn it upside down and tap it").
```

## teach
- To clean a computer keyboard you need a can of compressed air, a microfibre cloth and rubbing alcohol.
- Cleaning a computer keyboard takes 10 to 15 minutes.
- Step 1 of cleaning a computer keyboard: unplug the keyboard or turn off the laptop to be safe.
- Step 2 of cleaning a computer keyboard: shake the keyboard upside down to get the crumbs out.
- Step 3 of cleaning a computer keyboard: blow between the keys with the compressed air.
- Step 4 of cleaning a computer keyboard: wipe the keys with the cloth dampened with rubbing alcohol; do not soak the cloth.
- Step 5 of cleaning a computer keyboard: let the keyboard dry for 2 to 3 minutes before using it.

## checklist
- names what is needed: compressed air, a cloth and rubbing alcohol
- gives the steps in order: unplug, shake out, blow, wipe keys, dry
- warns not to soak the cloth

## variant
- change: no compressed air: shake and wipe only
- request: I've no can of air. Can I still clean my keyboard?
- situation: I have a microfibre cloth and rubbing alcohol.
- expect: The reply skips the compressed air step, shaking the crumbs out and wiping the keys.

```datalog situation
has(user, cloth).
has(user, rubbing_alcohol).
lacks(user, compressed_air).
```

```datalog difference
drop(clean_keyboard_s3).
no_need(compressed_air).
```

## variant
- change: deep clean: pull off the keycaps first
- request: I want to give my mechanical keyboard a really deep clean, keycaps and all. What changes?
- situation: I have compressed air, a cloth, rubbing alcohol and a keycap puller.
- expect: The reply adds pulling off the keycaps with the keycap puller before blowing out the keyboard, and pressing them back on at the end.

```datalog situation
has(user, compressed_air).
has(user, cloth).
has(user, rubbing_alcohol).
has(user, keycap_puller).
wants(user, deep_clean).
```

```datalog difference
extra_need(keycap_puller).
add_step(clean_keyboard_s9, clean_keyboard_s2).
step(clean_keyboard, clean_keyboard_s9).
action(clean_keyboard_s9, remove).
object_of(clean_keyboard_s9, keycap).
uses(clean_keyboard_s9, keycap_puller).
role(clean_keyboard_s9, keycap_puller, lift).
before(clean_keyboard_s2, clean_keyboard_s9).
before(clean_keyboard_s9, clean_keyboard_s3).
add_step(clean_keyboard_s10, clean_keyboard_s5).
step(clean_keyboard, clean_keyboard_s10).
action(clean_keyboard_s10, press).
object_of(clean_keyboard_s10, keycap).
onto(clean_keyboard_s10, keyboard).
before(clean_keyboard_s5, clean_keyboard_s10).
```
