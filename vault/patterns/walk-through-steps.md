---
kind: pattern
applies_to: procedure
rules:
- when:
  - not ever_user=failed
  - steps_left
  do: instruct(step_first_undone)
- when:
  - not last_user=done
  - last_agent=instruct
  do: repeat(step_first_undone)
- when:
  - not steps_left
  do: finish
- when:
  - last_user=done
  do: instruct(step_first_undone)
otherwise: escalate
support: 71
validated: 1.0
seen_with:
- '[[export-data]]'
- '[[pair-device]]'
- '[[reset-password]]'
learned_from: data/pretrain-80-per-shape.jsonl
trust: 0.8
status: trusted
---
Learned from 71 conversations. First rule that fits wins.

1. when not ever_user=failed and steps_left → `instruct(step_first_undone)`
2. when not last_user=done and last_agent=instruct → `repeat(step_first_undone)`
3. when not steps_left → `finish`
4. when last_user=done → `instruct(step_first_undone)`
5. otherwise → `escalate`
