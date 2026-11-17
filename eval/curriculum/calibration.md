# Checklist judge calibration

The model (Haiku) checklist verdicts against a symbolic check from the ground truth (needs named, step actions in order, total time), on encounters without a variant.

| Run | Encounters | Agree | Model pass, symbolic fail | Model fail, symbolic pass | Second encounter (model) | Second encounter (symbolic) |
|---|---|---|---|---|---|---|
| 2026-11-17 #2 | 74 | 51 | 0 | 23 | 6 of 20 | 11 of 20 |
| 2026-11-17 #3 | 71 | 54 | 0 | 17 | 9 of 20 | 12 of 20 |
| 2026-11-17 #4 | 70 | 40 | 0 | 30 | 10 of 20 | 18 of 20 |
| 2026-11-17 #5 | 70 | 43 | 0 | 27 | 10 of 20 | 18 of 20 |
| 2026-11-17 #6 | 67 | 46 | 0 | 21 | 13 of 20 | 18 of 20 |

## 2026-11-17 #2

- cook_rice (teach_retry): model fail, symbolic pass (needs 75%, steps in order 80%, total time yes); model said: 1: no | 2: no | 3: yes | 4: yes | VERDICT: fail
- iron_shirt (teach_retry): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: yes | 4: yes | VERDICT: fail
- make_omelette (teach_retry): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- make_pancake (teach_retry): model fail, symbolic pass (needs 100%, steps in order 80%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- make_smoothie (teach_retry): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- plan_picnic (teach_retry): model fail, symbolic pass (needs 100%, steps in order 80%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- wash_dishes (teach_retry): model fail, symbolic pass (needs 80%, steps in order 80%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- water_plant (teach_retry): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: no | 4: yes | VERDICT: fail
- cook_rice (repeat): model fail, symbolic pass (needs 100%, steps in order 80%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- make_omelette (repeat): model fail, symbolic pass (needs 83%, steps in order 100%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- make_pancake (repeat): model fail, symbolic pass (needs 100%, steps in order 80%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- wash_dishes (repeat): model fail, symbolic pass (needs 80%, steps in order 80%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- water_plant (repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: no | 3: no | 4: yes | VERDICT: fail
- cook_rice (spaced_repeat): model fail, symbolic pass (needs 100%, steps in order 80%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- do_laundry (spaced_repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- make_omelette (spaced_repeat): model fail, symbolic pass (needs 83%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: yes | 4: yes | VERDICT: fail
- make_pancake (spaced_repeat): model fail, symbolic pass (needs 100%, steps in order 80%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- wash_dishes (spaced_repeat): model fail, symbolic pass (needs 80%, steps in order 80%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- water_plant (spaced_repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: no | 4: yes | VERDICT: fail
- cook_rice (retention): model fail, symbolic pass (needs 100%, steps in order 80%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- iron_shirt (retention): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: yes | 4: yes | VERDICT: fail
- wash_dishes (retention): model fail, symbolic pass (needs 80%, steps in order 80%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- water_plant (retention): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: no | 4: yes | VERDICT: fail

## 2026-11-17 #3

- do_laundry (teach_retry): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: yes | 4: yes | VERDICT: fail
- iron_shirt (teach_retry): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: yes | 4: yes | VERDICT: fail
- make_smoothie (teach_retry): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- plan_picnic (teach_retry): model fail, symbolic pass (needs 100%, steps in order 80%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- wash_dishes (teach_retry): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- water_plant (teach_retry): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: yes | 2: yes | 3: no | 4: yes | VERDICT: fail
- cook_rice (repeat): model fail, symbolic pass (needs 100%, steps in order 80%, total time yes); model said: 1: no | 2: yes | 3: yes | 4: yes | VERDICT: fail
- wash_dishes (repeat): model fail, symbolic pass (needs 80%, steps in order 100%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- water_plant (repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: no | 3: no | 4: yes | VERDICT: fail
- cook_rice (spaced_repeat): model fail, symbolic pass (needs 100%, steps in order 80%, total time yes); model said: 1: no | 2: yes | 3: yes | 4: yes | VERDICT: fail
- do_laundry (spaced_repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: no | 3: yes | 4: yes | VERDICT: fail
- water_plant (spaced_repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: no | 3: no | 4: yes | VERDICT: fail
- boil_egg (retention): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: yes | 4: yes | VERDICT: fail
- cook_rice (retention): model fail, symbolic pass (needs 100%, steps in order 80%, total time yes); model said: 1: no | 2: yes | 3: yes | 4: yes | VERDICT: fail
- iron_shirt (retention): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: yes | 4: yes | VERDICT: fail
- wash_dishes (retention): model fail, symbolic pass (needs 80%, steps in order 100%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- water_plant (retention): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: no | 4: yes | VERDICT: fail

## 2026-11-17 #4

- bake_potato (teach_retry): model fail, symbolic pass (needs 100%, steps in order 80%, total time yes); model said: 1: yes | 2: no | 3: no | 4: yes | VERDICT: fail
- boil_egg (teach_retry): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: yes | 4: yes | VERDICT: fail
- do_laundry (teach_retry): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: yes | 4: yes | VERDICT: fail
- iron_shirt (teach_retry): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: yes | 4: yes | VERDICT: fail
- make_omelette (teach_retry): model fail, symbolic pass (needs 100%, steps in order 80%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- plan_birthday_party (teach_retry): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- replace_lightbulb (teach_retry): model fail, symbolic pass (needs 100%, steps in order 83%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- water_plant (teach_retry): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: no | 4: yes | VERDICT: fail
- bake_potato (repeat): model fail, symbolic pass (needs 100%, steps in order 80%, total time yes); model said: 1: yes | 2: no | 3: no | 4: yes | VERDICT: fail
- cook_rice (repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: yes | 4: yes | VERDICT: fail
- do_laundry (repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: yes | 4: yes | VERDICT: fail
- make_omelette (repeat): model fail, symbolic pass (needs 100%, steps in order 80%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- plan_birthday_party (repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- replace_lightbulb (repeat): model fail, symbolic pass (needs 100%, steps in order 83%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- water_plant (repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: yes | 2: yes | 3: no | 4: yes | VERDICT: fail
- write_thank_you_note (repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: no | 3: yes | 4: yes | VERDICT: fail
- bake_potato (spaced_repeat): model fail, symbolic pass (needs 100%, steps in order 80%, total time yes); model said: 1: yes | 2: no | 3: no | 4: yes | VERDICT: fail
- cook_rice (spaced_repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: yes | 4: yes | VERDICT: fail
- do_laundry (spaced_repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: yes | 4: yes | VERDICT: fail
- make_omelette (spaced_repeat): model fail, symbolic pass (needs 100%, steps in order 80%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- plan_birthday_party (spaced_repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- replace_lightbulb (spaced_repeat): model fail, symbolic pass (needs 100%, steps in order 83%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- water_plant (spaced_repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: yes | 2: yes | 3: no | 4: yes | VERDICT: fail
- write_thank_you_note (spaced_repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: no | 3: yes | 4: yes | VERDICT: fail
- bake_potato (retention): model fail, symbolic pass (needs 100%, steps in order 80%, total time yes); model said: 1: yes | 2: no | 3: no | 4: yes | VERDICT: fail
- cook_rice (retention): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: yes | 4: yes | VERDICT: fail
- iron_shirt (retention): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: yes | 4: yes | VERDICT: fail
- replace_lightbulb (retention): model fail, symbolic pass (needs 100%, steps in order 83%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- water_plant (retention): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: yes | 2: yes | 3: no | 4: yes | VERDICT: fail
- write_thank_you_note (retention): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: no | 3: yes | 4: yes | VERDICT: fail

## 2026-11-17 #5

- bake_potato (teach_retry): model fail, symbolic pass (needs 100%, steps in order 80%, total time yes); model said: 1: yes | 2: no | 3: no | 4: yes | VERDICT: fail
- do_laundry (teach_retry): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: yes | 4: yes | VERDICT: fail
- make_omelette (teach_retry): model fail, symbolic pass (needs 100%, steps in order 80%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- plan_birthday_party (teach_retry): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- replace_lightbulb (teach_retry): model fail, symbolic pass (needs 100%, steps in order 83%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- water_plant (teach_retry): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: yes | 2: yes | 3: no | 4: yes | VERDICT: fail
- bake_potato (repeat): model fail, symbolic pass (needs 100%, steps in order 80%, total time yes); model said: 1: yes | 2: no | 3: no | 4: yes | VERDICT: fail
- cook_rice (repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: yes | 4: yes | VERDICT: fail
- do_laundry (repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: yes | 4: yes | VERDICT: fail
- make_omelette (repeat): model fail, symbolic pass (needs 100%, steps in order 80%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- plan_birthday_party (repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- replace_lightbulb (repeat): model fail, symbolic pass (needs 100%, steps in order 83%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- water_plant (repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: yes | 2: yes | 3: no | 4: yes | VERDICT: fail
- write_thank_you_note (repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: no | 3: yes | 4: yes | VERDICT: fail
- bake_potato (spaced_repeat): model fail, symbolic pass (needs 100%, steps in order 80%, total time yes); model said: 1: yes | 2: no | 3: no | 4: yes | VERDICT: fail
- cook_rice (spaced_repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: yes | 4: yes | VERDICT: fail
- do_laundry (spaced_repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: yes | 4: yes | VERDICT: fail
- make_omelette (spaced_repeat): model fail, symbolic pass (needs 100%, steps in order 80%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- plan_birthday_party (spaced_repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- replace_lightbulb (spaced_repeat): model fail, symbolic pass (needs 100%, steps in order 83%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- water_plant (spaced_repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: yes | 2: yes | 3: no | 4: yes | VERDICT: fail
- write_thank_you_note (spaced_repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: no | 3: yes | 4: yes | VERDICT: fail
- bake_potato (retention): model fail, symbolic pass (needs 100%, steps in order 80%, total time yes); model said: 1: yes | 2: no | 3: no | 4: yes | VERDICT: fail
- pack_for_trip (retention): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: yes | 2: yes | 3: no | 4: yes | VERDICT: fail
- replace_lightbulb (retention): model fail, symbolic pass (needs 100%, steps in order 83%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- water_plant (retention): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: no | 4: yes | VERDICT: fail
- write_thank_you_note (retention): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: no | 3: yes | 4: yes | VERDICT: fail

## 2026-11-17 #6

- cook_rice (teach_retry): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- do_laundry (teach_retry): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: yes | 4: yes | VERDICT: fail
- plan_birthday_party (teach_retry): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- replace_lightbulb (teach_retry): model fail, symbolic pass (needs 100%, steps in order 83%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- water_plant (teach_retry): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: yes | 2: yes | 3: no | 4: yes | VERDICT: fail
- do_laundry (repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: yes | 4: yes | VERDICT: fail
- plan_birthday_party (repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- replace_lightbulb (repeat): model fail, symbolic pass (needs 100%, steps in order 83%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- water_plant (repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: yes | 2: yes | 3: no | 4: yes | VERDICT: fail
- write_thank_you_note (repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: no | 3: yes | 4: yes | VERDICT: fail
- do_laundry (spaced_repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: yes | 4: yes | VERDICT: fail
- plan_birthday_party (spaced_repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- replace_lightbulb (spaced_repeat): model fail, symbolic pass (needs 100%, steps in order 83%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- water_plant (spaced_repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: yes | 2: yes | 3: no | 4: yes | VERDICT: fail
- write_thank_you_note (spaced_repeat): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: no | 3: yes | 4: yes | VERDICT: fail
- do_laundry (retention): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: no | 2: yes | 3: yes | 4: yes | VERDICT: fail
- make_smoothie (retention): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: yes | 2: yes | 3: yes | 4: no | VERDICT: fail
- plan_birthday_party (retention): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- replace_lightbulb (retention): model fail, symbolic pass (needs 100%, steps in order 83%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
- water_plant (retention): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: yes | 2: yes | 3: no | 4: yes | VERDICT: fail
- write_thank_you_note (retention): model fail, symbolic pass (needs 100%, steps in order 100%, total time yes); model said: 1: yes | 2: no | 3: yes | 4: yes | VERDICT: fail
