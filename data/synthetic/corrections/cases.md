# Correction test set

Each case: facts and rules (`id: rule`), then one or more corrections from the
teacher (`wrong:` an atom the agent concluded that is false). Corrections are
gradual (Design 2.0): one counterexample stores only an exception
(`not -head`); a second that shares a feature with an earlier one, and that the
rule's correct cases lack, adds that feature as a condition. Expected:
`changed:` the rule file ids that must change (`none` for none), `rule:` the
expected text of each changed rule after all corrections, `also_false:`
conclusions that must disappear as a consequence. `keeps:` a wrong conclusion
that stays derived because its rule is core (read-only); then `stored:` the
negative fact that records the correction (a contradiction for the curiosity
drive to ask about). Everything else the agent
believed before must still be believed afterwards.

## penguins do not fly
facts: is_a(tweety, bird); is_a(robin1, bird); is_a(pingu, bird); is_a(pingu, penguin)
rules: flies: fly(X) :- is_a(X, bird).
rules: wants: wants(X, D) :- likes(X, D).
wrong: fly(pingu)
changed: flies
rule: flies: fly(X) :- is_a(X, bird), not -fly(X).

## two exceptions in a row
facts: is_a(tweety, bird); is_a(robin1, bird); is_a(pingu, bird); is_a(pingu, penguin); is_a(ozzy, bird); is_a(ozzy, ostrich)
rules: flies: fly(X) :- is_a(X, bird).
wrong: fly(pingu)
wrong: fly(ozzy)
changed: flies
rule: flies: fly(X) :- is_a(X, bird), not -fly(X).

## no distinguishing feature gives an exception
facts: likes(anna, tea); likes(bob, tea)
rules: wants: wants(X, D) :- likes(X, D).
rules: flies: fly(X) :- is_a(X, bird).
wrong: wants(bob, tea)
changed: wants
rule: wants: wants(X, D) :- likes(X, D), not -wants(X, D).

## a condition from a place
facts: event(m1); during(m1, morning); located_in(m1, home); event(m2); during(m2, morning); located_in(m2, home); event(m3); during(m3, morning); located_in(m3, hotel)
rules: coffee: drink(X, coffee) :- during(X, morning).
wrong: drink(m3, coffee)
changed: coffee
rule: coffee: drink(X, coffee) :- during(X, morning), not -drink(X, coffee).

## core rules are read-only
facts: is_a(whale, mammal); is_a(mammal, animal); is_a(whale, sea_creature)
rules: flies: fly(X) :- is_a(X, bird).
wrong: is_a(whale, animal)
changed: none
keeps: is_a(whale, animal)
stored: -is_a(whale, animal)

## a wrong stored fact is not a rule problem
facts: likes(anna, coffee); likes(bob, tea)
rules: wants: wants(X, D) :- likes(X, D).
wrong: likes(anna, coffee)
changed: none
also_false: wants(anna, coffee)

## only the rule that made the conclusion changes
facts: has(sparrow1, wing); has(sparrow2, wing); has(dodo1, wing); is_a(dodo1, dodo)
rules: canfly: can_fly(X) :- has(X, wing).
rules: travels: travel(X) :- can_fly(X).
wrong: can_fly(dodo1)
changed: canfly
rule: canfly: can_fly(X) :- has(X, wing), not -can_fly(X).
also_false: travel(dodo1)

## the top rule of a chain is the one refined
facts: has(sparrow1, wing); has(sparrow2, wing); has(dodo1, wing); is_a(dodo1, dodo)
rules: canfly: can_fly(X) :- has(X, wing).
rules: travels: travel(X) :- can_fly(X).
wrong: travel(dodo1)
changed: travels
rule: travels: travel(X) :- can_fly(X), not -travel(X).

## a learned hypothesis is refined too
facts: event(e1); during(e1, evening); event(e2); during(e2, evening); event(e3); during(e3, evening); feels(e3, sick)
rules: hyp_tea [learner]: drink(X, tea) :- during(X, evening).
wrong: drink(e3, tea)
changed: hyp_tea
rule: hyp_tea: drink(X, tea) :- during(X, evening), not -drink(X, tea).

## properties
facts: is_a(car1, car); is_a(car2, car); is_a(car3, car); property(car3, quality, broken)
rules: drives: drive(X) :- is_a(X, car).
rules: colours: property(X, colour, red) :- is_a(X, fire_truck).
wrong: drive(car3)
changed: drives
rule: drives: drive(X) :- is_a(X, car), not -drive(X).

## two penguins change the rule
facts: is_a(tweety, bird); is_a(robin1, bird); is_a(pingu, bird); is_a(pingu, penguin); is_a(pinga, bird); is_a(pinga, penguin); is_a(pinky, bird); is_a(pinky, penguin)
rules: flies: fly(X) :- is_a(X, bird).
wrong: fly(pingu)
wrong: fly(pinga)
changed: flies
rule: flies: fly(X) :- is_a(X, bird), not -fly(X), not is_a(X, penguin).
also_false: fly(pinky)

## two broken cars change the rule
facts: is_a(car1, car); is_a(car2, car); is_a(car3, car); property(car3, quality, broken); is_a(car4, car); property(car4, quality, broken); is_a(car5, car); property(car5, quality, broken)
rules: drives: drive(X) :- is_a(X, car).
wrong: drive(car3)
wrong: drive(car4)
changed: drives
rule: drives: drive(X) :- is_a(X, car), not -drive(X), not property(X, quality, broken).
also_false: drive(car5)
