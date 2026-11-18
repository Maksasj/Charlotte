---
id: core_rules
status: confirmed
conf: 1
evidence: 0
counter: 0
module: core
source: builtin
---
# Core rules

Transitivity of categories, parts and time order; symmetry of sameness;
inheritance of properties and parts through categories unless an exception
is known. Persons are things.

```datalog
is_a(X, Z) :- is_a(X, Y), is_a(Y, Z).
part_of(X, Z) :- part_of(X, Y), part_of(Y, Z).
before(A, C) :- before(A, B), before(B, C).
same(X, Y) :- same(Y, X).
thing(X) :- person(X).
person(X) :- is_a(X, person).
place(X) :- is_a(X, place).
has(X, Y) :- is_a(X, C), has(C, Y), not -has(X, Y).
property(X, P, V) :- is_a(X, C), property(C, P, V), not -property(X, P, V).
```
