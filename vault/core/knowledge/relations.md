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

rule rel-teach-generic
when: "link $subj $pred $obj", $subj != $obj
then:
  ++rel(subj=$subj, pred=$pred, obj=$obj)
  => "Linked: $subj -[$pred]→ $obj."
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


## Relations named by a verb (claude-opus-5.5, 2026-11-27)

One set of rules for every relation a verb names: `relation_word(word, pred)`
says which verb names which relation ("made", "created", "invented" ->
`created`). The words are language, so they live in `language/knowledge.md`;
these rules only say what a verb sentence means. Learned the active way round
(`rel(subj=Google, pred="created", obj=Go)`); questions also find facts stored
the other way round through `inverse` ("Rust created_by Mozilla").

`not_thing(word)` marks words that are never a thing ("this", "it", "I"), so
"this is a test" or "I made a mistake" is not learned as a fact.

```karin
rule rel-teach-verb
when: "$s $v $o", relation_word(word=$v, pred=$p), $s != $o, words($s) <= 4, words($o) <= 4, !not_thing(word=$s), !not_thing(word=$o)
then:
  ++rel(subj=$s, pred=$p, obj=$o)
  => "Learned: $s $v $o."

rule rel-teach-verb-passive
when: "$o was $v by $s", relation_word(word=$v, pred=$p), $s != $o, words($s) <= 4, words($o) <= 4, !not_thing(word=$o)
then:
  ++rel(subj=$s, pred=$p, obj=$o)
  => "Learned: $o was $v by $s."

rule rel-ask-who-verb
when: "who $v $o", relation_word(word=$v, pred=$p), !query_who()
then: +query_who(pred=$p, obj=$o, word=$v)

rule rel-ask-who-verb-passive
when: "who was $o $v by", relation_word(word=$v, pred=$p), !query_who()
then: +query_who(pred=$p, obj=$o, word=$v)

rule rel-who-direct
when: query_who(pred=$p, obj=$o, word=$v), rel(subj=$s, pred=$p, obj=$o)
then: => "$s $v $o."

rule rel-who-inverse
when: query_who(pred=$p, obj=$o, word=$v), !rel(pred=$p, obj=$o), inverse(pred=$p, inv=$i), rel(subj=$o, pred=$i, obj=$s)
then: => "$s $v $o."

rule rel-who-unknown
when: query_who(pred=$p, obj=$o, word=$v), !rel(pred=$p, obj=$o), inverse(pred=$p, inv=$i), !rel(subj=$o, pred=$i)
then: => "I don't know who $v $o. If you tell me (\"... $v $o\"), I'll remember."
```

