# Parser output specification (v1)

The pattern parser turns one user message into one *intent* with *slots*. Tests
compare a canonical one-line rendering:

```
<intent>: <payload>
```

The speaker is **maksim**: `I`, `me`, `my`, `myself` refer to `maksim`.

## Normalization

- Entities and values are lowercase; multi-word names are joined with `_`
  (`New York` -> `new_york`, `ice cream` -> `ice_cream`).
- Articles and determiners `a an the some` are dropped.
- Plural common nouns are singularized: `apples` -> `apple`, `berries` -> `berry`,
  `boxes` -> `box`, `wings` -> `wing`. Capitalized names are never singularized
  (`James` stays `james`).
- Question variables are written `W` (who/what/where). Rule variables are `A`.
- Negated facts start with `-`: `-likes(anna, tea)`.
- Several atoms are separated by `; ` in the order they appear.

## Predicates

| Phrase | Atom |
|---|---|
| X like/love/enjoy Y | `likes(x, y)` |
| X dislike/hate Y, X doesn't/does not like Y | `-likes(x, y)` |
| X has/have/owns Y | `has(x, y)`; negated `-has(x, y)` |
| X wants/needs Y | `wants(x, y)` |
| X knows Y | `knows(x, y)` |
| X is a/an Y (noun) | `is_a(x, y)`; `X is not a Y` -> `-is_a(x, y)` |
| Xs are Ys (plural nouns) | `is_a(x, y)` |
| X is / feels EMOTION (happy, sad, tired, angry, excited, bored, hungry, scared, calm, nervous) | `feels(x, emotion)` |
| X is COLOUR (red, blue, green, yellow, black, white, brown, orange, purple, pink, grey, gray) | `property(x, colour, c)` |
| X is SIZE (big, small, large, tiny, huge, tall, short) | `property(x, size, s)` |
| X is ADJ (any other adjective) | `property(x, quality, adj)` |
| X's P is V; the P of X is V; my P is V | `property(x, p, v)` |
| X lives in / is in / is located in / is at Y | `located_in(x, y)` |
| X is part of Y | `part_of(x, y)` |
| X causes Y | `causes(x, y)` |
| X VERBs Y (any other verb) | `verb_lemma(x, y)`: `Anna drinks tea` -> `drink(anna, tea)` |
| X VERBs PREP Y | `verb_prep(x, y)`: `Bob works at Acme` -> `work_at(bob, acme)` |
| X VERBs (no object) | `verb_lemma(x)`: `Tweety flies` -> `fly(tweety)` |

Negating any statement (`doesn't`, `does not`, `don't`, `isn't`, `is not`, `never`)
gives the negated atom. `anymore` / `no longer` does not change the atom:
`I don't drink coffee anymore` -> `tell: -drink(maksim, coffee)`.

Coordination: `Anna likes tea and coffee` -> `tell: likes(anna, tea); likes(anna, coffee)`;
`Anna and Bob like tea` -> `tell: likes(anna, tea); likes(bob, tea)`.

## Intents

| Intent | Examples | Payload |
|---|---|---|
| `tell` | `Anna likes tea.` | atoms, or a rule |
| `tell ... @past` | `I used to like tea.` | atoms followed by ` @past` |
| `ask` | `Does Anna like tea?` -> `ask: likes(anna, tea)` | query atom(s) |
| | `What does Anna like?` -> `ask: likes(anna, W)` | |
| | `Who likes tea?` -> `ask: likes(W, tea)` | |
| | `What is a robin?` / `What is Rex?` -> `ask: is_a(robin, W)` | |
| | `Is Rex a dog?` -> `ask: is_a(rex, dog)` | |
| | `Is the car red?` -> `ask: property(car, colour, red)` | |
| | `Is Anna happy?` -> `ask: feels(anna, happy)` | |
| | `Where is Paris?` / `Where does Anna live?` -> `ask: located_in(paris, W)` | |
| | `What colour is the car?` -> `ask: property(car, colour, W)` | |
| | `What is the size of the box?` / `What is Anna's job?` -> `ask: property(box, size, W)` / `ask: property(anna, job, W)` | |
| | `What does Bob have?` -> `ask: has(bob, W)` | |
| | `Does Tweety fly?` -> `ask: fly(tweety)` | |
| `ask ... @history` | `Did Anna like tea?` -> `ask: likes(anna, tea) @history` | past tense questions |
| | `What did I use to drink?` -> `ask: drink(maksim, W) @history` | |
| `recall` | `What do you know about Anna?` / `Tell me about Anna.` | `recall: anna` |
| `why` | `Why?` -> `why: last` | atom or `last` |
| | `Why is Rex an animal?` -> `why: is_a(rex, animal)` | |
| | `Why does Anna like tea?` / `How do you know that Anna likes tea?` -> `why: likes(anna, tea)` | |
| `correct` | `No, Rex is a cat.` / `Actually, Rex is a cat.` -> `correct: is_a(rex, cat)` | atoms or `none` |
| | `That's wrong.` / `Wrong.` / `That is not right.` -> `correct: none` | |
| | `No, Anna doesn't like tea.` -> `correct: -likes(anna, tea)` | |
| `yes` | `yes`, `yeah`, `yep`, `right`, `correct`, `exactly`, `sure`, `that's right` | `yes:` (empty payload) |
| `no` | `no`, `nope`, `nah` | `no:` |
| `greet` | `hi`, `hello`, `hey`, `good morning` | `greet:` |
| `bye` | `bye`, `goodbye`, `see you` | `bye:` |
| `unknown` | anything that fits no pattern | `unknown:` |

## Rules

- `If someone is a doctor, then they are a person.` ->
  `tell: is_a(A, person) :- is_a(A, doctor).`
- `If something is a bird then it flies.` -> `tell: fly(A) :- is_a(A, bird).`
- `If someone likes coffee then they drink coffee.` ->
  `tell: drink(A, coffee) :- likes(A, coffee).`
- `Every bird has wings.` / `Birds have wings.` -> `tell: has(bird, wing)` (a fact about the category).
- `Every cat likes fish.` / `Cats like fish.` -> `tell: likes(A, fish) :- is_a(A, cat).`
  (plural or `every` subject with any verb except have/be becomes a rule.)
- `Dogs are animals.` -> `tell: is_a(dog, animal)`.

`someone`, `something`, `they`, `it`, `he`, `she` inside an if-then are the variable `A`.
