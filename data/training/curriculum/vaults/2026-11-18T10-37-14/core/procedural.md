---
id: procedural
status: confirmed
conf: 1
evidence: 0
counter: 0
module: core
source: builtin
---
# Core layers for tasks (Design 2.0)

The layers added to the primes in v1's ontology, each defined once and shared
by everything the agent learns: how to do things (procedural), picking among
options (choice), where a conversation is (session), words (lexical) and the
agent itself (self). Session facts (`done`, `available`, ...) come from the
conversation, never from the vault. Read-only for the learner.

Steps are small events ordered by `before` (which is transitive), so two steps
can run side by side. Durations are ranges with a unit:
`takes_time(tea, 3, 5, minute)`.

```datalog
% Procedural layer
core_predicate(task, 1, procedural, "T is a task someone can do").
core_predicate(step, 2, procedural, "S is a step of task T").
core_predicate(action, 2, procedural, "the action of step or event E is A").
core_predicate(object_of, 2, events, "the thing acted on in event or step E is X").
core_predicate(recipient, 2, events, "the one who receives in event E is X").
core_predicate(needs, 2, procedural, "task T needs item I").
core_predicate(uses, 2, procedural, "step S uses item I").
core_predicate(takes_time, 4, procedural, "X takes Lo to Hi of unit U").
core_predicate(until, 2, procedural, "step S goes on until state C").
core_predicate(overlaps, 2, procedural, "step S runs while step R runs").
core_predicate(method, 2, procedural, "task T can be done by doing task M").
core_predicate(subtask, 2, procedural, "task T includes task U").
core_predicate(done_when, 2, procedural, "task T is done when state C holds").
% Choice layer
core_predicate(solves, 2, choice, "task T is a way to reach goal G").
core_predicate(recommended_for, 2, choice, "option T is good for situation X").
core_predicate(available, 1, choice, "the user has item I").
core_predicate(staple, 1, choice, "item I is in almost every home (salt, water)").
core_predicate(substitute, 2, choice, "item J can replace item I").
core_predicate(portion, 3, choice, "one person needs N of item I").
% Session layer
core_predicate(active_task, 1, session, "the conversation is about task T").
core_predicate(at_step, 1, session, "the user is at step S").
core_predicate(slot, 2, session, "slot N of the active task has value V").
core_predicate(open_question, 1, session, "the agent is waiting for an answer to Q").
core_predicate(grounded, 1, session, "fact F was confirmed in this conversation").
core_predicate(said, 1, session, "the agent already said fact F").
core_predicate(done, 1, session, "the user has done step S").
% Lexical layer
core_predicate(same_pred, 2, lexical, "predicates P and Q mean the same").
core_predicate(surface, 2, lexical, "X is written Text").
core_predicate(plural_of, 2, lexical, "the plural of X is written Text").
core_predicate(frame, 2, lexical, "predicate P is said with sentence frame Text").
% Self layer
core_predicate(can, 1, self, "the agent can do X").
core_predicate(cannot, 1, self, "the agent cannot do X").
core_predicate(knows_task, 1, self, "the agent knows how to do task T").

% Everything a task needs: its own needs, what its steps use, and the needs of its sub-tasks.
% A task that solves a goal solves every goal said to be the same.
solves(T, G) :- solves(T, A), same(A, G).
needs_all(T, X) :- needs(T, X).
needs_all(T, X) :- step(T, S), uses(S, X).
needs_all(T, X) :- subtask(T, U), needs_all(U, X).
% What is missing, given what the user has.
covered(X) :- substitute(X, J), available(J).
missing(T, X) :- needs_all(T, X), not available(X), not staple(X), not covered(X).
lacking(T) :- missing(T, X).
feasible(T) :- task(T), not lacking(T).
option_for(G, T) :- solves(T, G), feasible(T).
% Steps in order: a step waits while an earlier step of its task is not done.
waiting(S) :- step(T, S), step(T, P), before(P, S), not done(P).
next_step(T, S) :- step(T, S), not done(S), not waiting(S).
% Durations in minutes.
minutes(X, Lo, Hi) :- takes_time(X, Lo, Hi, minute).
minutes(X, Lo, Hi) :- takes_time(X, L, H, hour), Lo = L * 60, Hi = H * 60.
minutes(X, Lo, Hi) :- takes_time(X, L, H, second), Lo = L / 60, Hi = H / 60.
% A task's time from its steps; a step that runs while another runs adds nothing.
adds_time(T, S) :- step(T, S), minutes(S, L, H), not side_by_side(S).
side_by_side(S) :- overlaps(S, R).
total_minutes(T, Lo, Hi) :- task(T), adds_time(T, _), Lo = sum L { adds_time(T, S), minutes(S, L, H) }, Hi = sum H { adds_time(T, S), minutes(S, L, H) }.
step_count(T, N) :- task(T), N = count { step(T, S) }.
% What the agent can and cannot do (self-descriptions come from these).
can(learn_from_you).
can(reason_with_what_i_know).
can(explain_why_i_believe_something).
can(help_with_tasks_i_know).
cannot(taste_or_see_things).
cannot(look_things_up_online).
% The agent knows how to do a task it has steps for.
knows_task(T) :- task(T), step(T, S).
```
