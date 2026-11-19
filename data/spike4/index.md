# Design 4.0 spike fixture: planted-case index

Test-only fixture (decision D4.3, Appendix A of `research/Symbolic Learning Agent – Design 4.0.md`). Written by a sub-agent from the design documents only, without sight of `crates/`; **to be reviewed by the user before any run.** `MANIFEST.sha256` is not written yet: the main session seals the fixture after review.

Line references are `file:line` relative to `data/spike4/`. Regenerate the counts and re-check everything with `python3 -I tools/design4/check_fixture.py`.

## Files

| File | Content |
|---|---|
| `teach/baking.msgs`, `cooking.msgs`, `cleaning.msgs`, `repair.msgs` | Item relations, in curriculum order (kinds, materials and properties, functions, places, parts, actions and states, names). Each starts with a `topic <domain>/vocabulary.` batch of `new(sym, sort).` declarations for the symbols first used in that file |
| `teach/procedures_fixture.msgs` | 8 procedures (2 per domain), **marked fixture**: v3 ground-truth core facts copied from `data/curriculum3`, plus `demands` and a few `fixture addition` lines; then 4 taught swap cases |
| `script/answers.jsonl` | Oracle teacher answers keyed by canonical question (naming, conflicts, diamonds, confirming questions). Any other question gets `unknown.` |
| `bench/questions.jsonl` | The 100 spike questions |
| `bench/deny.txt` | Every gold atom, intermediate proof node, withheld piece and `swap` form (291 atoms); none is taught |

Delivery order: baking, cooking, cleaning, repair, procedures_fixture; oracle answers are delivered when asked.

## Conventions the fixture assumes (please check)

1. **Role symbols come from the v3 ground truth.** The loaf tin's role in `bake_banana_bread_s4` is `contain` (as in `data/curriculum3`), not `hold` as in the design's E4. Item relations therefore use `contain` (`can_do_on(baking_tin, contain, batter)`), and the demand is `demands(bake_banana_bread_s4, contain, can_go_in, oven)`.
2. **Procedures come from the v3 ground truth, not from archive3 readings.** No cached Haiku reading was opened (no calls, and the sub-agent did not inspect `data/training/`). Only the predicates task, step, action, object_of, uses, role, before, requires_state, achieves and needs are copied; settings, times, purposes, checks, `into` and `overlaps` are left out. `bake_banana_bread` and `make_apple_crumble` have `domain(..., cooking)` in v3 and are used as baking here (as eval §5.1 suggests).
3. **Fixture additions** to the ground truth (each marked in the file): `achieves`/`requires_state` pairs for the four STEP questions, one `before(make_omelette_s2, make_omelette_s4)` (B8 reads only direct `before`), and 14 `demands`.
4. **Taught swap cases are topic-scoped.** The grammar has no case wrapper, so each case is a `topic <domain>/case_<name>.` batch holding `lacks(user, I). has(user, J). swap(S, I, J).`. The `lacks`/`has` lines are the case's situation only and must not enter the global situation; the checker treats them so. If agent4 cannot scope them, these 4 batches need another form.
5. **Hops** follow section 7.4: an inheritance down n `is_a` links counts n; B5, the B6 role bridge, B7, B8 count 1; the `used_for`/`can_do_on` → `can_do` bridges, reification and leaves count 0; a B7 proof sums its `can_do` proof and every demand proof, plus 1. A transitive `is_a(X, K)` over n links counts n − 1; a `has_part` chain of n links counts n − 1 plus any inheritance. **`same_as` (B4) counts 0** because it is canonicalisation at entry; NAME questions therefore have at least one real hop besides the renaming, otherwise the gold would equal a taught fact under the deny list's `same_as` closure (D4.8 level L1).
6. **Query form.** `query: <goal>.` with an optional `given: <facts>.`; variables are upper case. A help request is `request(<task>). <situation facts>.`. Golds name the user's symbol (e.g. `egg_whisk`); compare modulo `same_as`.
7. **Extra gold fields** beyond `kind/atoms/proof_lines`: `excluded` (SUB look-alikes that must not be claimed), `missing` and `completes` (GAP: the withheld pieces, and what they would make derivable), `rule` and `min_support` (IND), and top-level `ind_case` (`support`, `counterexample`, `below_threshold`) and `note`. For a request, a `gap(S, I)` gold atom is the B9 conclusion the answer must state.
8. **Induction support counts inherited subjects** (section 7.1: taught or inherited). The planted clusters have 3-4 *taught* examples; with subkinds the support sets are larger (listed below).

## Sort errors (exactly 2, must be rejected)

- `made_of(whisk, beat)` teach/baking.msgs:148: beat is action/function, not material
- `has_property(abrasive, scourer)` teach/cleaning.msgs:115: abrasive is property, not item

## Specificity exceptions (9)

| # | General | Exception | Question |
|---|---|---|---|
| 1 | `can_go_in(dish, oven)` teach/cooking.msgs:196 | `not can_go_in(plastic_dish, oven)` teach/cooking.msgs:197 | q055 `not can_go_in(takeaway_box, oven)`; with the **redundant link** `is_a(takeaway_box, dish)` teach/cooking.msgs:118 |
| 2 | `not can_go_in(plastic_dish, oven)` teach/cooking.msgs:197 | `can_go_in(ovenproof_plastic_tray, oven)` teach/cooking.msgs:198 | q062 `can_go_in(lasagne_tray, oven)`; **exception to an exception** |
| 3 | `can_go_in(pan, dishwasher)` teach/cooking.msgs:204 | `not can_go_in(cast_iron_pan, dishwasher)` teach/cooking.msgs:205 | q059 `not can_go_in(camp_skillet, dishwasher)` |
| 4 | `can_go_in(cutlery, dishwasher)` teach/cooking.msgs:202 | `not can_go_in(kitchen_knife, dishwasher)` teach/cooking.msgs:203 | q060 `not can_go_in(santoku_knife, dishwasher)` |
| 5 | `can_go_in(cleaning_cloth, washing_machine)` teach/cleaning.msgs:142 | `not can_go_in(chamois, washing_machine)` teach/cleaning.msgs:143 | q058 `not can_go_in(car_chamois, washing_machine)` |
| 6 | `has_property(scourer, abrasive)` teach/cleaning.msgs:113 | `not has_property(non_scratch_scourer, abrasive)` teach/cleaning.msgs:114 | q061 `not has_property(glass_hob_scourer, abrasive)` |
| 7 | `made_of(hammer, metal)` teach/repair.msgs:115 | `not made_of(rubber_mallet, metal)` teach/repair.msgs:116 | q057 `not made_of(tile_mallet, metal)` |
| 8 | `has_property(flour, contains_gluten)` teach/baking.msgs:150 | `not has_property(rice_flour, contains_gluten)` teach/baking.msgs:151 | q056 `not has_property(brown_rice_flour, contains_gluten)` |
| 9 | `has_property(screw_fixing, magnetic)` teach/repair.msgs:121 | `not has_property(brass_screw, magnetic)` teach/repair.msgs:122 | q064 `not has_property(antique_hinge_screw, magnetic)` |

Also `not can_go_in(kids_bowl, oven)` (q063) is 3 links below `plastic_dish`. Each exception's taught line raises an E3-style conflict question while teaching; the oracle answers `exception` (script/answers.jsonl, `conflict:` keys).

## Diamonds (3)

- `enamel_dish`: `can_go_in(ceramic_dish, microwave)` teach/cooking.msgs:200 vs `not can_go_in(metal_container, microwave)` teach/cooking.msgs:201, through `is_a(enamel_dish, ceramic_dish)` teach/cooking.msgs:113; `is_a(enamel_dish, metal_container)` teach/cooking.msgs:115. Oracle: script/answers.jsonl:21 → `no`. No question.
- `insulated_pliers`: `has_property(metal_tool, conductive)` teach/repair.msgs:119 (2 links) vs `not has_property(insulated_tool, conductive)` teach/repair.msgs:120 (1 link): incomparable kinds, so a conflict although the distances differ. Oracle script/answers.jsonl:22 → `no`; then q008 `has_property(electricians_pliers, conductive)` is answered `no` in 1 hop.
- `wool_duster`: `can_go_in(duster, washing_machine)` teach/cleaning.msgs:144 vs `not can_go_in(wool_item, washing_machine)` teach/cleaning.msgs:145. Oracle script/answers.jsonl:23 → `unknown` (left open on purpose); q091 expects a gap.

## Induction clusters (4, support from at least 2 domains; gold `hyp`)

- `not can_go_in(X, oven) :- made_of(X, plastic)`. Taught examples: `made_of(plastic_jug, plastic)` teach/baking.msgs:144; `not can_go_in(plastic_jug, oven)` teach/baking.msgs:180; `made_of(plastic_tub, plastic)` teach/cooking.msgs:159; `not can_go_in(plastic_tub, oven)` teach/cooking.msgs:199; `made_of(plastic_bucket, plastic)` teach/cleaning.msgs:108; `not can_go_in(plastic_bucket, oven)` teach/cleaning.msgs:146. Support with inheritance: 4 (plastic_bucket, plastic_jug, plastic_tub, spray_bottle), 4 kinds, counter none. Question q075 `not can_go_in(food_bag, oven)`. Confirming question script/answers.jsonl:24 (`no`; spray_bottle teach/cleaning.msgs:109).
- `can_do(X, dissolve_limescale) :- has_property(X, acidic)`. Taught examples: `has_property(acid, acidic)` teach/cooking.msgs:173; `can_do(acid, dissolve_limescale)` teach/cleaning.msgs:130; `is_a(vinegar, acid)` teach/cooking.msgs:152; `is_a(citric_acid, acid)` teach/cleaning.msgs:97; `has_property(cola, acidic)` teach/cooking.msgs:174; `can_do(cola, dissolve_limescale)` teach/cooking.msgs:192. Support with inheritance: 8 (acid, citric_acid, citrus_juice, cola, lemon_juice, lime, vinegar, white_vinegar), 6 kinds, counter none. Question q076 `can_do(lemon, dissolve_limescale)`. Confirming question script/answers.jsonl:25 (`yes`).
- `can_do(X, cut) :- has_property(X, sharp)`. Taught examples: `has_property(knife, sharp)` teach/cooking.msgs:170; `can_do(knife, cut)` teach/cooking.msgs:178; `has_property(scissors, sharp)` teach/cooking.msgs:171; `can_do(scissors, cut)` teach/cooking.msgs:179; `has_property(saw, sharp)` teach/repair.msgs:125; `can_do(saw, cut)` teach/repair.msgs:134. Support with inheritance: 12 (bread_knife, butter_knife, chef_knife, hacksaw, junior_hacksaw, kitchen_knife, kitchen_scissors, knife, santoku_knife, saw, scissors, wire_cutter), 8 kinds, counter none. Question q077 `can_do(pizza_wheel, cut)`. Confirming question script/answers.jsonl:26 (`yes`).
- `can_do(X, soak_up) :- has_property(X, absorbent)`. Taught examples: `has_property(sponge, absorbent)` teach/cleaning.msgs:116; `can_do(sponge, soak_up)` teach/cleaning.msgs:128; `has_property(paper_towel, absorbent)` teach/cleaning.msgs:117; `can_do(paper_towel, soak_up)` teach/cleaning.msgs:129; `has_property(tea_towel, absorbent)` teach/cooking.msgs:175; `can_do(tea_towel, soak_up)` teach/cooking.msgs:191. Support with inheritance: 4 (cotton_rag, paper_towel, sponge, tea_towel), 3 kinds, counter none. Question q078 `can_do(sawdust, soak_up)`. Confirming question script/answers.jsonl:27 (`yes`).

The material-based cluster is plastic → not oven; the property-based one is acidic → can dissolve limescale (`dissolve_limescale` is the function symbol). The deny list holds each question's prediction, so its confirming question is about another prediction.

## Traps with support 2 (must not induce at k = 3; gold `gap`)

- `can_go_in(X, oven) :- made_of(X, glass)`: `made_of(pyrex_jug, glass)` teach/baking.msgs:145; `can_go_in(pyrex_jug, oven)` teach/baking.msgs:179; `made_of(glass_dish, glass)` teach/cooking.msgs:167; support 2 (glass_dish, pyrex_jug). Questions: q082 `can_go_in(glass_jar, oven)`.
- `not can_go_in(X, dishwasher) :- made_of(X, wood)`: `made_of(wooden_spoon, wood)` teach/cooking.msgs:168; `not can_go_in(wooden_spoon, dishwasher)` teach/cooking.msgs:207; `made_of(chopping_board, wood)` teach/cooking.msgs:169; `not can_go_in(chopping_board, dishwasher)` teach/cooking.msgs:208; support 2 (chopping_board, wooden_spoon). Questions: q083 `can_go_in(rolling_pin, dishwasher)`, q084 `can_go_in(bread_board, dishwasher)`.
  (`glass_dish` supports through `can_go_in(dish, oven)` teach/cooking.msgs:196.)

## Clusters with counterexamples (must not induce, or must specialise; gold `gap`)

- **Taught counterexample:** `can_go_in(X, dishwasher) :- made_of(X, metal)`. Support 5 (butter_knife, fork, knife, metal_colander, spoon) from `can_go_in(cutlery, dishwasher)` teach/cooking.msgs:202; `made_of(fork, metal)` teach/cooking.msgs:161, `can_go_in(metal_colander, dishwasher)` teach/cooking.msgs:206. Counterexamples taught: `not can_go_in(kitchen_knife, dishwasher)` teach/cooking.msgs:203; `made_of(cast_iron_pan, metal)` teach/cooking.msgs:166; `not can_go_in(cast_iron_pan, dishwasher)` teach/cooking.msgs:205; with subkinds: bread_knife, camp_skillet, cast_iron_pan, cast_iron_skillet, chef_knife, kitchen_knife, santoku_knife. Questions q079 `can_go_in(whisk, dishwasher)`, q080 `can_go_in(grater, dishwasher)`.
- **Inherited counterexample (the silicone cases):** `not can_go_in(X, oven) :- made_of(X, rubber)`. Support 4 (rubber_glove, rubber_mallet, rubber_seal, tile_mallet) from `not can_go_in(rubber_glove, oven)` teach/cleaning.msgs:147; `not can_go_in(rubber_mallet, oven)` teach/repair.msgs:143; `not can_go_in(rubber_seal, oven)` teach/repair.msgs:144. Counterexamples by inheritance: silicone_mould, silicone_tray. Question q081 `can_go_in(rubber_plug, oven)`.

A specialisation (for example `not isa(X, baking_tin)`, or an instance exception) could turn either rule into a hypothesis that predicts the question atom; the golds stay `gap` as Appendix A requires. Please confirm this reading.

## Silicone-tray cases (2): kind inheritance against a material pattern

- `is_a(silicone_tray, baking_tin)` teach/baking.msgs:108; `made_of(silicone_tray, rubber)` teach/baking.msgs:141, `can_go_in(baking_tin, oven)` teach/baking.msgs:176. q003 expects `yes` by inheritance (taught and inherited beat learned).
- `is_a(silicone_mould, cake_tin)` teach/baking.msgs:109; `made_of(silicone_mould, rubber)` teach/baking.msgs:142. q009 expects `yes` (2 links).

## Decoy kinds (2)

- `plastic_tub` (teach/cooking.msgs:126, ancestor `container` shared with `baking_tin` teach/baking.msgs:102) fails `demands(bake_banana_bread_s4, contain, can_go_in, oven)` teach/procedures_fixture.msgs:148 by `not can_go_in(plastic_tub, oven)` teach/cooking.msgs:199. q039 excludes it.
- `flathead_screwdriver` (teach/repair.msgs:86, sibling of `phillips_screwdriver`) fails `demands(replace_door_handle_s1, loosen, has_property, cross_tip)` teach/procedures_fixture.msgs:434 by `not has_property(flathead_screwdriver, cross_tip)` teach/repair.msgs:124. q038 excludes it.

Excluded look-alike, not a decoy kind: `dish_soap` in q043 (can clean, `not has_property(dish_soap, acidic)` teach/cleaning.msgs:123).

## Swap cases for part B (4 cases, 4 tasks, 3 domains)

- `swap(make_omelette_s2, fork, chopsticks)` teach/procedures_fixture.msgs:441
- `swap(boil_egg_s2, spoon, ladle)` teach/procedures_fixture.msgs:445
- `swap(clean_window_s2, sponge, dish_cloth)` teach/procedures_fixture.msgs:449
- `swap(fix_dripping_tap_s2, screwdriver, butter_knife)` teach/procedures_fixture.msgs:453

The lgg of these four is the B7 body without demands (logic §4.3, example B). None of the cases is on a question's (step, item, stand-in) triple.

## Cross-domain links (12): kind taught in one domain, used in another

| # | Taught (domain) | Used in | Question |
|---|---|---|---|
| 1 | `is_a(tea_towel, cloth)` teach/cooking.msgs:146 (cooking) | cleaning (clean_window_s4) | q035 |
| 2 | `is_a(white_vinegar, vinegar)` teach/cooking.msgs:153 (cooking) | cleaning (clean_microwave_s1, with teach/cleaning.msgs:140) | q043 |
| 3 | `is_a(vinegar, acid)` teach/cooking.msgs:152 (cooking) | cleaning (teach/cleaning.msgs:130) | q014 |
| 4 | `is_a(lemon, citrus_fruit)` teach/cooking.msgs:148 (cooking) | cleaning (acidic rule) | q076 |
| 5 | `is_a(lime, citrus_fruit)` teach/cooking.msgs:149 (cooking) | cleaning (clean_microwave_s1 demand acidic) | q089 |
| 6 | `is_a(kitchen_scissors, scissors)` teach/cooking.msgs:138 (cooking) | gardening | q045 |
| 7 | `is_a(kitchen_knife, knife)` teach/cooking.msgs:132 (cooking) | gardening | q051 |
| 8 | `is_a(butter_knife, knife)` teach/cooking.msgs:136 (cooking) | repair (teach/repair.msgs:138) | q090 |
| 9 | `is_a(casserole_dish, dish)` teach/cooking.msgs:111 (cooking) | baking (make_apple_crumble_s5) | q029 |
| 10 | `is_a(bowl, container)` teach/baking.msgs:130 (baking) | gardening | q047 |
| 11 | `is_a(bucket, container)` teach/cleaning.msgs:92 (cleaning) | gardening | q048 |
| 12 | `is_a(rubber_glove, glove)` teach/cleaning.msgs:95 (cleaning) | gardening | q049 |
| 13 | `is_a(claw_hammer, hammer)` teach/repair.msgs:91 (repair) | gardening | q050 |
| 14 | `is_a(hacksaw, saw)` teach/repair.msgs:99 (repair) | gardening | q052 |
| 15 | `is_a(pyrex_jug, measuring_jug)` teach/baking.msgs:121 (baking) | gardening | q054 |

Gardening is never taught; its procedure fragments appear only in `given:`.

## Naming (3 drift pairs, plus taught `same_as`)

- `can_do(egg_whisk, beat)` teach/baking.msgs:164 → `ask: same_as(egg_whisk, whisk)? or is_a(egg_whisk, whisk)?` → `same_as(egg_whisk, whisk).` (script/answers.jsonl:1). Questions: q070.
- `used_for(cutting_board, chop)` teach/cooking.msgs:193 → `ask: same_as(cutting_board, chopping_board)? or is_a(cutting_board, chopping_board)?` → `same_as(cutting_board, chopping_board).` (script/answers.jsonl:3). Questions: q069.
- `can_do(adjustable_spanner, grip)` teach/repair.msgs:137 → `ask: same_as(adjustable_spanner, adjustable_wrench)? or is_a(adjustable_spanner, adjustable_wrench)?` → `same_as(adjustable_spanner, adjustable_wrench).` (script/answers.jsonl:4). Questions: q071.
- `whisk_attachment` (teach/baking.msgs:165) is the E2 negative: the oracle answers `not same_as(whisk_attachment, whisk)` (script/answers.jsonl:2).
- Taught `same_as`: `same_as(bicarbonate_of_soda, baking_soda)` teach/baking.msgs:194; `same_as(hob, stove_top)` teach/cooking.msgs:225; `same_as(washing_up_liquid, dish_soap)` teach/cleaning.msgs:161; `same_as(kitchen_roll, paper_towel)` teach/cleaning.msgs:162; `same_as(allen_key, hex_key)` teach/repair.msgs:168.
- `script/answers.jsonl` also answers likely false alarms of the head-token detector with `not same_as` (bread_board, balloon_whisk, white_vinegar, plastic_bucket, kitchen_mixer_tap, adjustable_wrench/spanner).

## Procedures (8, marked fixture)

| Domain | Task | Source | Demands |
|---|---|---|---|
| baking | `bake_banana_bread` (teach/procedures_fixture.msgs:96) | `data/curriculum3/train/bake_banana_bread.md` | `demands(bake_banana_bread_s4, contain, can_go_in, oven)` teach/procedures_fixture.msgs:148 |
| baking | `make_apple_crumble` (teach/procedures_fixture.msgs:151) | `data/curriculum3/train/make_apple_crumble.md` | `demands(make_apple_crumble_s5, contain, can_go_in, oven)` teach/procedures_fixture.msgs:202; `demands(make_apple_crumble_s2, cut, has_property, sharp)` teach/procedures_fixture.msgs:203; `demands(make_apple_crumble_s3, cut, has_property, sharp)` teach/procedures_fixture.msgs:204 |
| cooking | `make_omelette` (teach/procedures_fixture.msgs:207) | `data/curriculum3/dev/make_omelette.md` | `demands(make_omelette_s3, heat, can_go_in, hob)` teach/procedures_fixture.msgs:247; `demands(make_omelette_s4, heat, can_go_in, hob)` teach/procedures_fixture.msgs:248 |
| cooking | `boil_egg` (teach/procedures_fixture.msgs:251) | `data/curriculum3/dev/boil_egg.md` | `demands(boil_egg_s1, heat, can_go_in, hob)` teach/procedures_fixture.msgs:280; `demands(boil_egg_s4, contain, has_property, watertight)` teach/procedures_fixture.msgs:281 |
| cleaning | `clean_window` (teach/procedures_fixture.msgs:284) | `data/curriculum3/dev/clean_window.md` | `demands(clean_window_s1, contain, has_property, watertight)` teach/procedures_fixture.msgs:319; `demands(clean_window_s3, dry, has_property, lint_free)` teach/procedures_fixture.msgs:320 |
| cleaning | `clean_microwave` (teach/procedures_fixture.msgs:323) | `data/curriculum3/train/clean_microwave.md` | `demands(clean_microwave_s1, clean, has_property, acidic)` teach/procedures_fixture.msgs:353 |
| repair | `fix_dripping_tap` (teach/procedures_fixture.msgs:356) | `data/curriculum3/train/fix_dripping_tap.md` | `demands(fix_dripping_tap_s3, loosen, can_do, grip)` teach/procedures_fixture.msgs:402 |
| repair | `replace_door_handle` (teach/procedures_fixture.msgs:405) | `data/curriculum3/train/replace_door_handle.md` | `demands(replace_door_handle_s1, loosen, has_property, cross_tip)` teach/procedures_fixture.msgs:434; `demands(replace_door_handle_s4, fasten, has_property, cross_tip)` teach/procedures_fixture.msgs:435 |

## Questions

| Type | n | 1-hop | 2-hop | 3-hop | ids |
|---|---|---|---|---|---|
| K | 24 | 8 | 10 | 6 | q001-q024 |
| SUB | 20 | 4 | 10 | 6 | q025-q044 |
| X | 10 | 0 | 6 | 4 | q045-q054 |
| EXC | 10 | 4 | 4 | 2 | q055-q064 |
| STEP | 4 | 2 | 2 | 0 | q065-q068 |
| NAME | 6 | 2 | 4 | 0 | q069-q074 |
| IND | 10 | 0 | 0 | 0 | q075-q084 |
| GAP | 16 | 0 | 0 | 0 | q085-q100 |

IND: 4 `hyp` (q075-q078), 3 with a counterexample (q079-q081), 3 below the threshold (q082-q084). GAP: 16 (q085-q100), near-miss q085-q090.

## Counts per relation per domain

Accepted statements in `teach/` (procedure facts counted in their task's domain; `new` and `topic` lines not counted). From `check_fixture.py`:

```
fixture: data/spike4
statements: 724 accepted, 2 rejected (sort errors: teach/baking.msgs:148 made_of(whisk, beat), teach/cleaning.msgs:115 has_property(abrasive, scourer))
  teach/baking.msgs                    90
  teach/cooking.msgs                  120
  teach/cleaning.msgs                  84
  teach/repair.msgs                    83
  teach/procedures_fixture.msgs       330
  script/answers.jsonl                 17
  relation                 baking  cooking cleaning   repair
  is_a                         44       60       37       36
  same_as                       1        1        2        1
  has_part                      4        4        4       13
  made_of                       8       11        5        3
  has_property                  7        7        7        7
  can_do                        5       10        7        5
  can_do_on                     2        0        0        1
  used_for                      8        7        9        6
  can_go_in                     4       10        2        1
  effect                        3        2        3        2
  precondition                  1        1        1        2
  not made_of                   0        0        0        1
  not has_property              2        0        3        3
  not can_go_in                 1        7        4        2
  task                          2        2        2        2
  step                         12        9        8       13
  action                       12        9        8       13
  object_of                    12        9        8       13
  uses                         15        9        9        7
  role                         15        9        9        7
  before                       10        8        6       11
  requires_state                4        1        1        1
  achieves                      1        1        1        1
  needs                        18       10       11        5
  demands                       4        4        3        3
  lacks                         0        2        1        1
  has                           0        2        1        1
  swap                          0        2        1        1
```

Against Appendix A's per-domain targets: `is_a` is above target (34-60 per domain against about 20), because the 3-hop chains, exception chains and gardening stand-ins each need their own kinds; cooking's `can_go_in` (17 with negatives) is above about 6 because most exceptions are about the oven and the dishwasher; procedure facts are about 75 per domain against about 30, because whole ground-truth procedures are copied. The relation files alone are 83-120 statements each.
