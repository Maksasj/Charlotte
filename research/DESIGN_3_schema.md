# Design 3.0 knowledge schema (v3)

Phase A, milestone 19. This is the schema of the agent's knowledge, of the ground truth (decision D7) and of the relation-line answers (D5). It replaces the ten-predicate 2.0 ground truth (`task`, `step`, `needs`, `action`, `object_of`, `uses`, `before`, `overlaps`, `until`, `takes_time`; REPORT_DESIGN_2 and DESIGN_3_alternatives section 1.5).

What 2.0 could not express:
- why a step is done;
- when a step applies;
- what must be avoided;
- settings and amounts;
- what an item does in a step;
- compound steps (103 of 361 step lines held two actions but stored one);
- situations and the changes they cause;
- where a fact came from.

## 1. Principles

1. **Symbols, not English.** Every argument is a lower-case symbol (`baking_tray`), a number, or a unit symbol. No free text in facts. Surface words live only in `alias` facts, which are data, learned or authored, never code.
2. **One action per step.** A taught line that does two things becomes two steps with an order fact between them. Steps get ids `<task>_s<N>` in taught order; a split step gets the next free numbers, and order facts keep the sequence.
3. **Every content fact is checkable.** The answer to a help request is a set of facts in this schema (section 5). The fact-and-relation check compares that set with the ground truth.
4. **Situations are facts too.** A request's situation (what the user lacks, has, wants, how long they have) and the change it causes are written in the same schema, so adaptation can be learned and checked.
5. **Provenance is kept beside facts, not inside them.** The agent's vault stores for every fact who taught it, when, and how sure the agent is. The ground truth needs none of this.

## 2. Entities

| Predicate | Meaning |
|---|---|
| `task(T)` | T is a task (a procedure that can be asked for) |
| `step(T, S)` | S is a step of T |
| `goal(T, G)` | T is a way to reach goal G (`goal(make_tea, warm_drink)`); replaces `solves`. A task may have several goals (`goal(make_toast, toast)`, `goal(make_toast, plan_meal)`) |
| `domain(T, D)` | T's domain: `cooking`, `cleaning`, `repair`, `planning`, `errands`, `personal_care`, `gardening`, `travel`, `crafts`. Used to build the held-out split; not shown in answers |

Items are not declared; any symbol in an item position is an item.

## 3. Facts

### 3.1 Task level

| Predicate | Meaning | Example |
|---|---|---|
| `needs(T, I)` | T needs item I | `needs(bake_potato, fork)` |
| `takes_time(T, Min, Max, U)` | total time | `takes_time(bake_potato, 60, 70, minute)` |
| `optional_need(T, I)` | I helps but is not required | `optional_need(make_salad, salad_spinner)` |
| `serves(T, N)` | portions | `serves(make_omelette, 1)` |

### 3.2 Step core

| Predicate | Meaning | Example |
|---|---|---|
| `action(S, A)` | the one action of S | `action(bake_potato_s3, prick)` |
| `object_of(S, O)` | what is acted on | `object_of(bake_potato_s3, potato)` |
| `uses(S, I)` | an item used in S (tool, ingredient, surface, container) | `uses(bake_potato_s3, fork)` |
| `role(S, I, R)` | the function item I serves in S. R is an action-like symbol for the function: `beat`, `cut`, `hold`, `heat`, `contain`, `pierce`, `spread`, `cover`, `measure`, `stir`, `wipe` ... | `role(bake_potato_s3, fork, pierce)` |
| `into(S, I)` / `onto(S, I)` / `from(S, I)` | destination or source of the object | `into(make_tea_s2, mug)` |
| `with_part(S, P)` | the part of the object acted on | `with_part(clean_window_s2, frame)` |

Every `uses` item gets a `role` fact. `role` is what makes substitution by function possible (section 4).

### 3.3 Settings, amounts and time

| Predicate | Meaning | Example |
|---|---|---|
| `takes_time(S, Min, Max, U)` | step duration | `takes_time(bake_potato_s5, 50, 60, minute)` |
| `setting(S, Attr, Value, U)` | a setting of a tool or the environment | `setting(bake_potato_s1, temperature, 200, celsius)` |
| `amount(S, I, Q, U)` | quantity of an item | `amount(make_pancake_s1, flour, 200, gram)` |
| `until(S, State)` | do S until State holds | `until(bake_potato_s5, soft)` |
| `repeat(S, N)` | do S N times | `repeat(make_bed_s3, 2)` |

A non-numeric setting uses the unit `level`: `setting(scrambled_eggs_s3, heat, low, level)`.

Units are singular symbols: `minute`, `hour`, `second`, `day`, `week`, `gram`, `millilitre`, `litre`, `cup`, `tablespoon`, `teaspoon`, `celsius`, `piece`, `pinch`, `squirt`, `level`.

### 3.4 Order

| Predicate | Meaning |
|---|---|
| `before(S1, S2)` | S1 must finish before S2 starts |
| `overlaps(S2, S1)` | S2 is done while S1 runs |
| `optional(S)` | S may be skipped |
| `alternative(S1, S2)` | S1 and S2 are two ways to do the same thing; one is enough |

`before` facts are stated for every consecutive pair that must stay in order, not only the transitive minimum. A split compound step always gets `before(first_part, second_part)`.

### 3.5 Causes, conditions, warnings

| Predicate | Meaning | Example |
|---|---|---|
| `purpose(S, Effect)` | S is done so that Effect holds | `purpose(bake_potato_s3, steam_escapes)` |
| `achieves(S, State)` | S leaves its object in State | `achieves(wash_dishes_s4, dry)` |
| `requires_state(S, State)` | S needs State to hold first | `requires_state(iron_shirt_s3, iron_hot)` |
| `condition(S, Cond)` | do S only when Cond holds | `condition(do_laundry_s2, clothes_stained)` |
| `warning(S, Hazard)` | a danger or mistake to avoid in S | `warning(pan_fry_salmon_s2, oil_spits)` |
| `avoid(S, A, I)` | do not do action A with or to I in S | `avoid(clean_window_s2, use, paper_towel)` |
| `check(S, State)` | how to tell S is done or correct | `check(boil_egg_s3, yolk_set)` |

States, effects, hazards and conditions are compact symbols (`steam_escapes`, `yolk_set`, `oil_spits`). They name a meaning, not a sentence.

### 3.6 Situations and adaptation (variants)

A situation is written in facts about the user (`user`):

| Predicate | Meaning | Example |
|---|---|---|
| `lacks(user, I)` | the user does not have I, or it is broken | `lacks(user, whisk)` |
| `has(user, I)` | the user has I | `has(user, fork)` |
| `wants(user, X)` | a preference or extra | `wants(user, cheese)` |
| `excludes(user, X)` | something to leave out (diet, allergy) | `excludes(user, meat)` |
| `time_limit(user, N, U)` | the time the user has | `time_limit(user, 25, minute)` |
| `count(user, X, N)` | how many (guests, portions) | `count(user, guest, 8)` |

The change a situation causes to the base procedure is written in difference facts:

| Predicate | Meaning | Example |
|---|---|---|
| `swap(S, Old, New)` | in S use New instead of Old | `swap(make_omelette_s2, whisk, fork)` |
| `drop(S)` | leave step S out | `drop(make_salad_s3)` |
| `add_step(S_new, After)` | a new step after step After (`start` for the first); its content is given with the normal step facts on S_new | `add_step(make_omelette_s9, make_omelette_s3)` |
| `change(S, Pred, Old, New)` | one fact of S changes; S may be the task itself for task-level facts (`change(boil_egg, takes_time, 10_13, 8_10)`) | `change(bake_potato_s5, takes_time, 50_60, 5_8)` |
| `extra_need(I)` / `no_need(I)` | the needs list grows or shrinks | `extra_need(cheese)` |
| `cannot(Reason)` | the request cannot be met; the answer must say so | `cannot(no_heat_source)` |

New steps use ids from `_s9` upward so they never clash with base steps.

### 3.7 Words (aliases)

| Predicate | Meaning | Example |
|---|---|---|
| `alias(Sym, "words")` | a surface form of a symbol | `alias(baking_tray, "baking sheet")` |
| `task_alias(T, "words")` | a way to name the task | `task_alias(bake_potato, "jacket potato")` |

Aliases are the only place English appears. In the ground truth they are authored. In the agent they are learned from teaching and stored as data. The content check uses ground-truth aliases only to accept a symbol that names the same thing (`new_pot` against `bigger_pot`); it never uses them to score wording.

## 4. Inference the schema supports (to be decided by experiment X4)

- **Substitution by function.** If the user lacks I, I plays role R in step S, and an item J the user has is known to play R somewhere (`role(_, J, R)`), then propose `swap(S, I, J)`. Known roles come from teaching, never from a list in code.
- **Dropping.** If the user lacks I, I is used only in optional step S, then `drop(S)`.
- **Adding.** If the user wants an extra item X that fills a role in some other task's step, add a step modelled on that step (`add_step`).
- **Case transfer (A4).** Map a stored variant case (situation + difference) onto the new task by structure (same role, same position in order) and transfer the difference.
- **Induced rules (A3).** Rules over these predicates learned offline from the variant cases, e.g. `swap(S, I, J) :- lacks(user, I), role(S, I, R), has(user, J), can_play(J, R).`

Which of these are hand-written and which are learned is decided in Phase A (decision D6 makes case transfer and induced rules two arms).

## 5. Relation-line answers (D5)

An answer is a list of lines. One line is one fact, written `predicate arg arg ...` with step ids replaced by step numbers in the answer's own order:

```
task bake_potato
need potato
need olive_oil
time 60 70 minute
step 1 do heat oven
step 1 setting temperature 200 celsius
step 1 time 10 minute
step 2 do wash potato
step 2 overlaps 1
step 3 do prick potato
step 3 uses fork pierce
step 3 purpose steam_escapes
step 5 do bake potato
step 5 until soft
before 4 5
```

There is no English realiser. A short telegraphic gloss may be printed after the lines for a human reader, but it is never scored.

## 6. The fact-and-relation check

Given the answer's facts A and the ground truth facts G for the request (base procedure plus the variant's difference facts applied):

1. **Align steps.** Map answer step numbers to ground-truth step ids by best match on (action, object), in order. Unmatched answer steps are extra; unmatched ground-truth steps are missing.
2. **Match facts.** A fact matches if its predicate and aligned arguments are equal, or equal through a ground-truth `alias` of the same symbol.
3. **Score.** Recall = matched G facts / |G|. Precision = matched A facts / |A|. A **wrong claim** is an answer fact that contradicts G: a different value for a functional slot (action, object, time, setting, amount), an order fact reversed, or a swap to an item the situation says the user lacks.
4. **Pass** for a request = recall of the core facts (needs, step actions and objects, order, adaptation differences) is 1.0 and there are no wrong claims. Graded recall over all facts and the precision are reported beside it, so dumping every fact is visible as low precision.

The exact rules, tiers and statistics are fixed in the Design 3.0 document after experiment X6.

## 7. Provenance (agent vault only)

Every fact in the agent's vault carries a provenance record kept in a side table, keyed by the fact:

| Field | Meaning |
|---|---|
| `source` | `teach` (dialogue), `dataset`, `derived` (inferred by a rule or case), `seed` |
| `session`, `turn` | where it was learned |
| `parser` | `learned` or `fallback` (Haiku) |
| `confidence` | 0-1, from the parser or the inference |
| `uses`, `confirmed` | how often it was used in an answer and confirmed by the teacher |
| `retracted_by` | the correction that replaced it, if any |

Corrections never delete a fact: they mark it retracted and add the new one, so the vault keeps its history.

## 8. Ground-truth file format (v3)

Re-authored scenarios live in `data/curriculum3/` and keep the 2.0 section layout:
- `## requests`
- `## situation`
- `## ground truth` (v3 facts)
- `## aliases`
- `## teach`
- `## checklist`
- `## variant`

Each `## variant` keeps its English lines and gains two blocks: `situation facts` and `difference facts`.

The teach lines are kept and may gain a sentence where the ground truth needs a fact the teaching never stated (for example a reason). Every ground-truth fact must be stated or clearly implied by a teach line, so the agent can learn it from teaching alone. The validator `tools/design3/check_gt.py` checks the syntax, arity, references, order cycles, `role` for every `uses`, and step coverage of the teach lines.
