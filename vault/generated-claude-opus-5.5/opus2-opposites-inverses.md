# Opposites & Teachable Inverses (`claude-opus-5.5` round 2)

Symmetric opposite pairs and live teaching of new inverse relation pairs that plug directly into `rel-match-inverse`.

```karin
rel(subj="hot", pred="opposite_of", obj="cold")
rel(subj="cold", pred="opposite_of", obj="hot")
rel(subj="light", pred="opposite_of", obj="dark")
rel(subj="dark", pred="opposite_of", obj="light")
rel(subj="true", pred="opposite_of", obj="false")
rel(subj="false", pred="opposite_of", obj="true")
inverse(pred="teaches", inv="taught_by")
inverse(pred="taught_by", inv="teaches")
inverse(pred="orbits", inv="orbited_by")
inverse(pred="orbited_by", inv="orbits")

rule opus2-learn-opposite
when: "$a is the opposite of $b", $a != $b
then:
  ++rel(subj=$a, pred="opposite_of", obj=$b)
  ++rel(subj=$b, pred="opposite_of", obj=$a)
  => "Learned: $a and $b are opposites (stored both ways)."

rule opus2-ask-opposite
when: "opposite of $x", !collected(tag="opus2_opp"), !collected_empty(tag="opus2_opp")
then:
  +opus2_ctx(kind="opp", key=$x)
  @collect(rel="rel", field="obj", tag="opus2_opp", subj=$x, pred="opposite_of", sep=" or ")

rule opus2-reply-opposite
when: collected(tag="opus2_opp", list=$list), opus2_ctx(kind="opp", key=$x)
then: => "The opposite of $x is $list."

rule opus2-reply-opposite-empty
when: collected_empty(tag="opus2_opp"), opus2_ctx(kind="opp", key=$x)
then: => "I don't know the opposite of $x. Teach me with \"$x is the opposite of ...\"."

rule opus2-learn-inverse
when: "$p is the inverse relation of $q", $p != $q
then:
  ++inverse(pred=$p, inv=$q)
  ++inverse(pred=$q, inv=$p)
  => "Learned: $p ⇄ $q are inverse relations."

rule opus2-list-inverses
when: "list known relations", !collected(tag="opus2_inv"), !collected_empty(tag="opus2_inv")
then: @collect(rel="inverse", field="pred", tag="opus2_inv")

rule opus2-reply-inverses
when: collected(tag="opus2_inv", list=$list)
then: => "Relations with known inverses: $list."

rule opus2-reply-inverses-empty
when: collected_empty(tag="opus2_inv")
then: => "No inverse relations known yet."
```
