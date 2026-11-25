# Typed Relations & Inverses

Generic `rel(subj, pred, obj)` triples paired with `inverse(pred, inv)` declarations.
Teaching a relation in one direction automatically answers queries in both directions.

## Inverse Declarations

```karin
inverse(pred="located_in", inv="contains")
inverse(pred="contains", inv="located_in")
inverse(pred="created_by", inv="created")
inverse(pred="created", inv="created_by")
inverse(pred="part_of", inv="has_part")
inverse(pred="has_part", inv="part_of")
```

## Seed Relations

```karin
rel(subj="Paris", pred="located_in", obj="France")
rel(subj="Tokyo", pred="located_in", obj="Japan")
rel(subj="Vilnius", pred="located_in", obj="Lithuania")
rel(subj="Rust", pred="created_by", obj="Mozilla")
rel(subj="Python", pred="created_by", obj="Guido van Rossum")
```

## Teaching Relations

```karin
rule rel-teach-located-in
when: "$subj is located in $obj", $subj != $obj
then:
  ++rel(subj=$subj, pred="located_in", obj=$obj)
  => "Learned: $subj is located in $obj."

rule rel-teach-created-by
when: "$subj was created by $obj", $subj != $obj
then:
  ++rel(subj=$subj, pred="created_by", obj=$obj)
  => "Learned: $subj was created by $obj."

rule rel-teach-generic
when: "link $subj $pred $obj", $subj != $obj
then:
  ++rel(subj=$subj, pred=$pred, obj=$obj)
  => "Linked: $subj —[$pred]→ $obj."
```

## Querying Relations (Direct & Automatic Inverse)

```karin
rule rel-ask-located-in
when: "where is $subj located", !query_rel()
then: +query_rel(subj=$subj, pred="located_in")

rule rel-ask-where-is
when: "where is $subj", !query_rel()
then: +query_rel(subj=$subj, pred="located_in")

rule rel-ask-where-find
when: "where can I find $subj", !query_rel()
then: +query_rel(subj=$subj, pred="located_in")

rule rel-ask-contains
when: "what does $subj contain", !query_rel()
then: +query_rel(subj=$subj, pred="contains")

rule rel-ask-created-by
when: "who created $subj", !query_rel()
then: +query_rel(subj=$subj, pred="created_by")

rule rel-ask-created
when: "what did $subj create", !query_rel()
then: +query_rel(subj=$subj, pred="created")

rule rel-ask-generic
when: "query $subj $pred", !query_rel()
then: +query_rel(subj=$subj, pred=$pred)

rule rel-match-direct
when: query_rel(subj=$subj, pred=$pred), rel(subj=$subj, pred=$pred, obj=$obj)
then: => "$subj $pred $obj."

rule rel-match-inverse
when: query_rel(subj=$subj, pred=$pred), !rel(subj=$subj, pred=$pred), inverse(pred=$pred, inv=$inv), rel(subj=$obj, pred=$inv, obj=$subj)
then: => "$subj $pred $obj (via inverse $inv)."

rule rel-unknown-with-inverse
when: query_rel(subj=$subj, pred=$pred), !rel(subj=$subj, pred=$pred), inverse(pred=$pred, inv=$inv), !rel(pred=$inv, obj=$subj)
then: => "I don't have a $pred relation for $subj yet. Teach me with \"link $subj $pred ...\"."

rule rel-unknown-plain
when: query_rel(subj=$subj, pred=$pred), !rel(subj=$subj, pred=$pred), !inverse(pred=$pred)
then: => "I don't have a $pred relation for $subj yet. Teach me with \"link $subj $pred ...\"."
```
