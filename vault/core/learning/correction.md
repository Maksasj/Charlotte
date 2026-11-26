# Correction-Driven Rule Induction & Active Learning

When the user corrects Karin after a topic-likes query (e.g. `"what about caramel macchiato"`),
Karin inspects her knowledge graph via `@induce_from_correction`:

- If both `likes(item)` and `is_a(item, topic)` exist, she proposes a general Candidate deduction rule.
- If only one fact exists, she asks to confirm the missing link.
- If she has never heard of `item`, she asks what it is and whether it belongs to the topic.

```karin
rule correct-likes-what-about
when: "what about $item", dialogue_topic(kind="likes", topic=$topic), !correction_outcome()
then: @induce_from_correction(item=$item, topic=$topic)

rule correct-likes-forgot
when: "you forgot $item", dialogue_topic(kind="likes", topic=$topic), !correction_outcome()
then: @induce_from_correction(item=$item, topic=$topic)

rule correct-likes-why-not
when: "why not $item", dialogue_topic(kind="likes", topic=$topic), !correction_outcome()
then: @induce_from_correction(item=$item, topic=$topic)

rule reply-correction-proposed-rule
when: correction_outcome(status="proposed_rule", item=$item, topic=$topic, pattern_id=$pid)
then: => "Good catch! I see you like $item and $item is a $topic. I proposed candidate rule $pid to connect likes + is_a automatically - say \"approve\" (or click Promote in Patterns) to activate it."

rule reply-correction-already-active
when: correction_outcome(status="already_active", item=$item, topic=$topic)
then: => "Done - I connected $item as a $topic you like."

rule reply-correction-already-known
when: correction_outcome(status="already_known", item=$item, topic=$topic)
then: => "I already have $item listed as a $topic you like."

rule reply-correction-missing-isa
when: correction_outcome(status="missing_isa", item=$item, topic=$topic)
then: => "I know you like $item, but I didn't know it's a $topic. Is $item a $topic? Say \"yes\" to teach me."

rule reply-correction-missing-like
when: correction_outcome(status="missing_like", item=$item, topic=$topic)
then: => "I know $item is a $topic, but I didn't know you like it. Should I add $item to what you like? Say \"yes\"."

rule reply-correction-unknown-item
when: correction_outcome(status="unknown_item", item=$item, topic=$topic)
then: => "I haven't heard of $item before - what is it? Say \"yes\" if it's a $topic you like."

rule approve-pending-rule-word
when: "approve", dialogue_pending_approval(pattern_id=$pid), !approval_result()
then:
  -dialogue_pending_approval
  @approve_pattern(id=$pid)

rule approve-pending-rule-yes
when: "yes", dialogue_pending_approval(pattern_id=$pid), !approval_result()
then:
  -dialogue_pending_approval
  @approve_pattern(id=$pid)

rule reply-approval-ok
when: approval_result(ok="true", id=$pid)
then: => "Approved rule $pid! From now on, whenever you like something and I know its category, I'll connect them automatically."

rule reply-approval-fail
when: approval_result(ok="false", id=$pid)
then: => "I couldn't find candidate rule $pid to approve."

rule confirm-missing-isa-yes
when: "yes", dialogue_confirm(kind="confirm_isa", item=$item, topic=$topic)
then:
  -dialogue_confirm
  ++is_a(child=$item, parent=$topic)
  ++rel(subj="user", pred="likes", topic=$topic, obj=$item)
  => "Got it - learned that $item is a $topic and added it to your favorite $topic."

rule confirm-missing-like-yes
when: "yes", dialogue_confirm(kind="confirm_like", item=$item, topic=$topic)
then:
  -dialogue_confirm
  ++rel(subj="user", pred="likes", obj=$item)
  ++rel(subj="user", pred="likes", topic=$topic, obj=$item)
  => "Got it - added $item to your favorite $topic."

rule confirm-unknown-both-yes
when: "yes", dialogue_confirm(kind="confirm_both", item=$item, topic=$topic)
then:
  -dialogue_confirm
  ++is_a(child=$item, parent=$topic)
  ++rel(subj="user", pred="likes", obj=$item)
  ++rel(subj="user", pred="likes", topic=$topic, obj=$item)
  => "Got it - learned that $item is a $topic and remembered it as a $topic you like."

rule decline-pending-approval
when: "no", dialogue_pending_approval()
then:
  -dialogue_pending_approval
  => "Okay, I'll leave that rule as a candidate for now."

rule decline-dialogue-confirm
when: "no", dialogue_confirm()
then:
  -dialogue_confirm
  => "Okay, I won't add that."
```
