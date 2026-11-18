---
id: ontology
status: confirmed
conf: 1
evidence: 0
counter: 0
module: core
source: builtin
---
# Core ontology

The built-in predicates, inspired by Wierzbicka's semantic primes. Everything
else is learned as combinations of these. Read-only for the learner.
`core_predicate(Name, Arity, Group, Description)`. `functional(P, K)` means the
first K arguments of P determine the rest, so a new value closes the old one.

```datalog
% Things
core_predicate(thing, 1, things, "X is a thing").
core_predicate(same, 2, things, "X and Y are the same thing").
% Categories
core_predicate(is_a, 2, categories, "X is a kind of Y, or an instance of Y").
% Parts and having
core_predicate(part_of, 2, parts, "X is part of Y").
core_predicate(has, 2, parts, "X has Y").
% Properties
core_predicate(property, 3, properties, "X has property P with value V").
% Time
core_predicate(before, 2, time, "A happens before B").
core_predicate(during, 2, time, "A happens during T").
% Events and actions
core_predicate(event, 1, events, "E is an event").
core_predicate(agent_of, 2, events, "X is the agent of event E").
core_predicate(does, 2, events, "X does Action").
% Cause
core_predicate(causes, 2, cause, "A causes B").
% Minds
core_predicate(wants, 2, minds, "X wants Y").
core_predicate(knows, 2, minds, "X knows fact F").
core_predicate(believes, 2, minds, "X believes fact F").
core_predicate(feels, 2, minds, "X feels F").
% Common learned-style relations kept in core so modules share them
core_predicate(likes, 2, minds, "X likes Y").
core_predicate(person, 1, things, "X is a person").
core_predicate(place, 1, things, "X is a place").
core_predicate(located_in, 2, parts, "X is in place Y").
% Sources
core_predicate(says, 2, sources, "X says fact F").
functional(property, 2).
functional(located_in, 1).
```
