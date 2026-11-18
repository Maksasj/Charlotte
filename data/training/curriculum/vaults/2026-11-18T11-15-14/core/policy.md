---
id: policy
status: confirmed
conf: 1
evidence: 0
counter: 0
module: core
source: builtin
---
# Dialogue moves (Design 2.0)

Every reply move is chosen by these rules. Each turn the dialogue manager
adds the facts of the moment: `heard(Act)` for the acts of the message,
`requested(G)` for a request, `open_frame(G)` for the open task frame,
`filled(G, Slot)` for slots the conversation has filled, `active(T)` for the
procedure under way, `chosen(T)` for an option already offered. The move
with the lowest priority number is made; its proof says why.

```datalog
% Goals the agent knows: a task schema or a procedure.
known_goal(G) :- task_slot(G, S, R).
known_goal(T) :- task(T).
% Slots still to ask.
missing_slot(G, S) :- open_frame(G), task_slot(G, S, required), not filled(G, S).
asking(G) :- missing_slot(G, S).
% The time budget in minutes.
budget(N) :- time_available(N, minute).
budget(M) :- time_available(N, hour), M = N * 60.
too_long(T) :- budget(B), takes_time(T, L, H, minute), L > B.
too_long(T) :- budget(B), total_minutes(T, L, H), L > B.
% Options for the open frame.
candidate(G, T) :- open_frame(G), solves(T, G).
good_option(G, T) :- candidate(G, T), feasible(T), not too_long(T).
has_good(G) :- good_option(G, T).
offered(G) :- candidate(G, T), chosen(T).
has_next(T) :- active(T), next_step(T, S).

% Moves: dm_move(Name, Priority).
dm_move(close, 1) :- heard(accept), open_frame(G).
dm_move(next_step, 1) :- heard(ask_next), active(T), has_next(T).
dm_move(task_finished, 1) :- heard(ask_next), active(T), not has_next(T).
dm_move(next_step, 1) :- heard(done), active(T), has_next(T).
dm_move(task_finished, 1) :- heard(done), active(T), not has_next(T).
dm_move(recommend, 2) :- open_frame(G), not asking(G), has_good(G), not offered(G).
dm_move(closest, 3) :- open_frame(G), not asking(G), candidate(G, T), not has_good(G), not offered(G).
dm_move(ask_slots, 4) :- open_frame(G), asking(G).
dm_move(other_option, 2) :- heard(reject), open_frame(G).
dm_move(teach_me, 5) :- requested(G), not known_goal(G).
dm_move(steps, 2) :- requested(T), task(T), not open_frame(T).
any_active :- active(T).
any_frame :- open_frame(G).
dm_move(ask_task, 6) :- heard(ask_next), not any_active, not any_frame.
```
