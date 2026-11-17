# The formal language, version 2 (Design 2.0)

The translator turns what a person writes into this language, one numbered
line per sentence. The agent reasons, remembers and replies only from it. It
extends the v1 language (`docs/parser_spec.md`, still valid) with dialogue acts
for asking for help, describing one's situation, following steps and reacting.

## Lines

A message is split into sentences, numbered from 1. Every sentence gets one or
more lines, each `[n] act: payload`:

```
[1] greet:
[2] request: plan_dinner
[3] inform: available(pasta); available(garlic)
```

A sentence that cannot be expressed gets `[n] unknown:`; it does not affect the
others. A sentence with several acts gets several lines with the same number.

## Acts

| Act | Use it for | Payload | Example |
|---|---|---|---|
| `greet` | hello, hi, good morning | empty | `Hi there!` -> `greet:` |
| `bye` | goodbye, see you | empty | `Bye!` -> `bye:` |
| `social` | thanks, short reactions, questions about the agent itself | `thanks`, `reaction`, `who_are_you`, `what_can_you_do`, `and_you`, `dont_know` | `Thanks a lot!` -> `social: thanks` |
| `how_are_you` | how are you | empty | |
| `about_you` | a personal question to the agent ("Do you like soup?") | thing or `none` | `about_you: soup` |
| `yes`, `no` | a plain yes or no answer | empty | |
| `accept` | agreeing to a suggestion or plan | empty | `Sounds great.` -> `accept:` |
| `reject` | turning a suggestion down | empty | `I'd rather not cook that.` -> `reject:` |
| `feedback` | a comment on the agent's last reply, not on a fact | `positive` or `negative` | `Hmm, not quite.` -> `feedback: negative`; `That helps!` -> `feedback: positive` |
| `tell` | a statement of general or lasting knowledge | atoms or a rule, `; ` between them | `Cats like fish.` -> `tell: likes(cat, fish)` |
| `inform` | the speaker's situation right now, for this conversation only | atoms | `I have pasta and garlic.` -> `inform: available(pasta); available(garlic)` |
| `ask` | a question about the world | query (as in v1) | `Does Anna like tea?` -> `ask: likes(anna, tea)` |
| `why` | why the agent believes something | atom or `last` | |
| `recall` | "What do you know about X?" | entity | |
| `correct` | the agent got a fact wrong, and here is the right one | atoms, or `none` for "that's wrong" | `No, Rex is a dog.` -> `correct: is_a(rex, dog)` |
| `request` | the person asks for help with a task | task | `Can you help me plan dinner?` -> `request: plan_dinner` |
| `ask_how` | how to do a task | task | `How do I fix a flat tire?` -> `ask_how: fix_tire` |
| `ask_need` | what a task needs | task | `What do I need to make pancakes?` -> `ask_need: make_pancake` |
| `ask_duration` | how long a task or step takes | task or step | `How long does rice take?` -> `ask_duration: cook_rice` |
| `ask_next` | what to do next in the task under way | empty | `OK, what's next?` -> `ask_next:` |
| `done` | the person did a step | step or `last` | `Done, the water is boiling.` -> `done: last` |
| `lack` | the person does not have something the task needs | items | `I don't have garlic.` -> `lack: garlic` |
| `teach` | content on the teaching channel (a `[teach]` block) | atoms or a rule | |
| `new_pred` | a relation no listed predicate fits | `name(arg1, arg2) \| definition` | `new_pred: grow_on(moss, rock) \| X grows on the surface of Y` |
| `unknown` | nothing in the sentence can be expressed | empty | |

`tell` versus `inform`: `tell` is knowledge that stays true for everyone or for a
long time ("Garlic burns quickly.", "I'm vegetarian."); `inform` is the
speaker's situation in this conversation ("I have 20 minutes.", "My bike has a
flat tire.", "I'm at step two."). When in doubt, a statement in a request for
help is `inform`.

A question mark never ends up as only a `tell` or `inform`: a question is
`ask`, `ask_how`, `ask_need`, `ask_duration`, `ask_next`, `request`, `about_you`
or `social`.

A request for help is never dropped because the sentence also thanks or greets:
`Thanks! Can you help me pack?` -> `[1] social: thanks` and `[2] request: pack_bag`.

## Atoms

Facts are written `predicate(arg, arg)`. Arguments are:

- **entities**: lowercase, singular, words joined with `_` when they name one
  thing (`olive_oil`, `cherry_tomato`, `tea_bag`, `new_york`). No articles, no
  quantities, no adjectives unless they are part of the name (`green_tea` is a
  kind of tea; `big` in "a big pot" is dropped). Never a phrase:
  `good_for_eggs_and_frying` and `place_where_land_meets_sea` are wrong, say
  them as several facts. Never a pronoun or a word like `here`, `that`, `it`:
  resolve it to what it means, or leave the sentence out.
- **numbers**: digits, decimals with a point (`2.5`); "twenty" is `20`, "half" is
  `0.5`. A number is never part of an entity (`twenty_minutes` is wrong).
- **units**: `second`, `minute`, `hour`, `day`, `week`, `gram`, `kilogram`,
  `millilitre`, `litre`, `cup`, `spoon`, `teaspoon`, `tablespoon`, `piece`,
  `person`, `slice`, `degree`, `metre`, `kilometre`, `centimetre`.
- **tasks**: the verb and the main noun of its object, lemmatized:
  `plan_dinner`, `fix_tire`, `make_tea`, `boil_egg`, `pack_bag`,
  `make_pancake`, `clean_kitchen`, `water_plant`. Not the person, not the
  adjectives, not the time ("dinner tonight" -> `plan_dinner`).
- **steps**: the task name, `_s`, and the step number: `make_tea_s1`.
- **the speaker**: `I`, `me`, `my` are the speaker (`maksim` unless told
  otherwise). The speaker's own things are `my_x` (`my_bike`), as in v1.

Hedged general statements keep their hedge as an annotation after the atoms:
`Cats usually like fish.` -> `tell: likes(cat, fish) @usually`. Hedges:
`@usually` (usually, mostly, typically, most), `@often`, `@sometimes`
(sometimes, some). A statement about the past is `@past` as in v1.

## Predicates

Use these predicates where they fit; they are the registry
(`vault/lexicon/registry.md`). Any other verb becomes its lemma, as in v1
(`play(anna, chess)`, `bark(rex)`); a relation that is not a verb and fits
nothing here is a `new_pred` line, never invented silently.

| Predicate | Meaning |
|---|---|
| `is_a(x, kind)` | x is a kind of / an instance of kind |
| `part_of(x, y)`, `has(x, y)` | parts and having |
| `property(x, name, value)` | x's colour, size, age, taste, ... is value (`property(nina, age, 12)`); never `quality` |
| `likes(x, y)`, `wants(x, y)`, `knows(x, y)`, `feels(x, feeling)` | as in v1 |
| `located_in(x, place)`, `work_at(x, place)` | where x is or works |
| `causes(a, b)`, `before(a, b)`, `during(a, t)` | cause and time |
| `task(t)` | t is a task someone can do |
| `step(task, s)`, `action(s, verb)`, `object_of(s, thing)`, `uses(s, item)` | a step, what it does, to what, with what |
| `needs(task, item)` | the task needs the item |
| `takes_time(x, lo, hi, unit)` | x takes lo to hi units (`takes_time(boil_egg, 8, 10, minute)`; one number: lo = hi) |
| `until(s, state)`, `overlaps(s, r)`, `subtask(t, u)`, `done_when(t, state)` | step details |
| `solves(task, goal)`, `recommended_for(task, situation)` | options |
| `available(item)` | (inform) the speaker has the item now |
| `time_available(n, unit)` | (inform) the speaker has n units of time |
| `staple(item)`, `substitute(item, other)`, `portion(item, n, unit)` | choosing and amounts |
| `event(e)`, `agent_of(e, x)`, `object_of(e, y)`, `recipient(e, z)`, `during(e, t)` | events with more than two participants |

## Steps told in a message

Instructions for a task are `tell` lines with step facts. The task is the one
the message names, or the active task given in the context. Steps are numbered
in the order they are told, continuing after the steps the context already has.

- "then", "after that", "next" -> `before(s1, s2)`; a numbered list or
  "first ... then ... finally" orders the steps the same way;
- "while", "meanwhile", "at the same time" -> `overlaps(s2, s1)`;
- "until they are soft" -> `until(s, soft)`;
- "for 10 minutes", "about 10 minutes" -> `takes_time(s, 10, 10, minute)`;
  "8 to 10 minutes" -> `takes_time(s, 8, 10, minute)`;
- "with a pan", "in olive oil" -> `uses(s, pan)`, `uses(s, olive_oil)`.

`Boil the pasta for about 10 minutes, then fry the garlic in olive oil.`
(active task `make_pasta`, no steps yet)
```
[1] tell: step(make_pasta, make_pasta_s1); action(make_pasta_s1, boil); object_of(make_pasta_s1, pasta); takes_time(make_pasta_s1, 10, 10, minute); step(make_pasta, make_pasta_s2); action(make_pasta_s2, fry); object_of(make_pasta_s2, garlic); uses(make_pasta_s2, olive_oil); before(make_pasta_s1, make_pasta_s2)
```

Steps told one after another are in that order even without "then": each
step is `before` the next one told, unless "while" makes them overlap. A
dashed list orders its steps like a numbered list.

## Rules for unclear cases

- A plural subject with a verb states a fact about the kind: `Cats like
  fish.` -> `tell: likes(cat, fish)` (this replaces v1's rule form for verbs;
  "If ..." and "every"/"all ... then" sentences stay rules).
- An adjective uses the named property it describes: colour, size, age,
  taste ("sour"), texture ("soft"), temperature ("hot"), state ("broken",
  "dry", "ripe"), difficulty ("easy"). If none fits, it is a `new_pred` line.
- "I don't have X": `lack: x` while a task is under way, otherwise
  `inform: -available(x)`. Several items: `lack: garlic; salt`.
- A task whose verb is only implied gets `make` for food and things made
  ("for pancakes" -> `make_pancake`), otherwise the verb the request implies
  ("help me pack" -> `pack_bag`).
- "half an hour" is `0.5, 0.5, hour`; amounts are written as said.
- "Sure", "OK" answering a yes/no question are `yes`; agreeing to a
  suggestion is `accept`.

## Context

The translator also gets the conversation so far: the speaker, the agent's last
reply, the active task and its next step number, and the things recently
mentioned. "it", "they", "that one" mean the thing most recently mentioned that
fits; "the next step" and "that" after the agent gave a step mean that step.
Without context, a sentence that only makes sense with it is `unknown:`.

## Complete examples

`Hi! Can you help me plan dinner tonight?`
```
[1] greet:
[2] request: plan_dinner
```

`I have pasta, garlic, olive oil, and some cherry tomatoes. Maybe 20 minutes.`
```
[1] inform: available(pasta); available(garlic); available(olive_oil); available(cherry_tomato)
[2] inform: time_available(20, minute)
```

`Sounds great, thanks!`
```
[1] accept:
[1] social: thanks
```

`My bike has a flat tire and I have no idea what to do. Can you help me fix it?`
```
[1] inform: has(my_bike, flat_tire)
[2] request: fix_tire
```

`Boiling an egg usually takes about 8 to 10 minutes.`
```
[1] tell: takes_time(boil_egg, 8, 10, minute) @usually
```

`Hmm, not quite. What do I need for pancakes?`
```
[1] feedback: negative
[2] ask_need: make_pancake
```

`Done! What's next?`
```
[1] done: last
[2] ask_next:
```
