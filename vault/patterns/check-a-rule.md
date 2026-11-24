---
kind: pattern
applies_to: gated
rules:
- when:
  - p_sat
  - not standing=grant
  do: grant
- when:
  - p_need_user
  - not a_sat
  do: ask(p_first_unbound)
- when:
  - a_sat
  - last_agent=ask
  - not p_sat
  do: offer
- when:
  - a_viol
  - not p_sat
  do: deny
- when:
  - found
  do: deny
- when:
  - standing=offer
  do: confirm
- when:
  - a_sat
  - p_viol
  do: offer
- when:
  - a_unbound
  do: ask(a_first_unbound)
- when:
  - p_viol
  do: deny
- when:
  - last_agent=ask
  - not a_exists
  do: lookup
- when:
  - not subject_known
  - not a_exists
  do: ask(subject)
- when:
  - not a_exists
  - not ever_agent=lookup
  do: lookup
- when:
  - not a_exists
  do: escalate
- when:
  - not ever_agent=grant
  do: ask(p_first_unbound)
otherwise: confirm
support: 2707
validated: 1.0
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
learned_from: simulate --n 3000 --per-shape --seed 7 --paraphrases data/paraphrases.yaml
trust: 0.8
status: trusted
---
Learned from 2707 conversations. First rule that fits wins.

1. when p_sat and not standing=grant → `grant`
2. when p_need_user and not a_sat → `ask(p_first_unbound)`
3. when a_sat and last_agent=ask and not p_sat → `offer`
4. when a_viol and not p_sat → `deny`
5. when found → `deny`
6. when standing=offer → `confirm`
7. when a_sat and p_viol → `offer`
8. when a_unbound → `ask(a_first_unbound)`
9. when p_viol → `deny`
10. when last_agent=ask and not a_exists → `lookup`
11. when not subject_known and not a_exists → `ask(subject)`
12. when not a_exists and not ever_agent=lookup → `lookup`
13. when not a_exists → `escalate`
14. when not ever_agent=grant → `ask(p_first_unbound)`
15. otherwise → `confirm`
