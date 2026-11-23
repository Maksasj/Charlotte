---
kind: pattern
applies_to: gated
rules:
- when:
  - p_sat
  - not last_agent=grant
  do: grant
- when:
  - p_need_user
  - not a_sat
  do: ask(p_first_unbound)
- when:
  - a_sat
  - not last_agent=offer
  - p_viol
  do: offer
- when:
  - not p_need_record
  - not a_sat
  - not a_unbound
  do: deny
- when:
  - a_sat
  - ever_user=correct
  do: confirm
- when:
  - last_agent=ask
  do: lookup
- when:
  - not a_exists
  - not ever_agent=ask
  do: ask(subject)
- when:
  - a_unbound
  do: ask(a_first_unbound)
otherwise: ask(p_first_unbound)
support: 71
validated: 0.889
seen_with:
- '[[cancel]]'
- '[[compensation]]'
- '[[enable]]'
- '[[fix]]'
- '[[leave]]'
- '[[refund]]'
- '[[reimburse]]'
- '[[student-discount]]'
- '[[trial-extension]]'
- '[[upgrade]]'
- '[[warranty]]'
learned_from: data/pretrain-80-per-shape.jsonl
trust: 0.8
status: candidate
promotes_to: check-a-rule
---
Learned from 71 conversations. First rule that fits wins.

1. when p_sat and not last_agent=grant → `grant`
2. when p_need_user and not a_sat → `ask(p_first_unbound)`
3. when a_sat and not last_agent=offer and p_viol → `offer`
4. when not p_need_record and not a_sat and not a_unbound → `deny`
5. when a_sat and ever_user=correct → `confirm`
6. when last_agent=ask → `lookup`
7. when not a_exists and not ever_agent=ask → `ask(subject)`
8. when a_unbound → `ask(a_first_unbound)`
9. otherwise → `ask(p_first_unbound)`
