# Transitive Spatial & Containment Reasoning (`claude-opus-5.5`)

Adds multi-hop `located_in` / `part_of` queries, shared-region sibling lookup, extra inverse pairs, and capital facts.

```karin
inverse(pred="parent_of", inv="child_of")
inverse(pred="child_of", inv="parent_of")
inverse(pred="works_on", inv="worked_on_by")
inverse(pred="worked_on_by", inv="works_on")

rel(subj="Kaunas", pred="located_in", obj="Lithuania")
rel(subj="Lithuania", pred="located_in", obj="Europe")
rel(subj="France", pred="located_in", obj="Europe")
rel(subj="Japan", pred="located_in", obj="Asia")
rel(subj="Europe", pred="located_in", obj="Earth")
rel(subj="Asia", pred="located_in", obj="Earth")
rel(subj="piston", pred="part_of", obj="engine")
rel(subj="engine", pred="part_of", obj="car")

prop(entity="Lithuania", attr="capital", value="Vilnius")
prop(entity="France", attr="capital", value="Paris")
prop(entity="Japan", attr="capital", value="Tokyo")

rule locx-ask-located
when: "is $a located in $b", !query_loc()
then: +query_loc(cur=$a, target=$b, orig=$a, pred="located_in")

rule locx-ask-part-of
when: "is $a part of $b", !query_loc()
then: +query_loc(cur=$a, target=$b, orig=$a, pred="part_of")

rule locx-match
when: query_loc(cur=$cur, target=$t, orig=$o, pred=$p), rel(subj=$cur, pred=$p, obj=$t)
then: => "Yes - $o is $p $t."

rule locx-hop
when: query_loc(cur=$cur, target=$t, orig=$o, pred=$p), rel(subj=$cur, pred=$p, obj=$mid), $mid != $t, !rel(subj=$cur, pred=$p, obj=$t), !visited_loc(node=$mid)
then:
  -query_loc
  +visited_loc(node=$mid)
  +query_loc(cur=$mid, target=$t, orig=$o, pred=$p)

rule locx-unknown
when: query_loc(cur=$cur, target=$t, orig=$o, pred=$p), !rel(subj=$cur, pred=$p)
then: => "I can't find a $p chain from $o to $t."

rule locx-shared-region
when: "what shares a region with $a", rel(subj=$a, pred="located_in", obj=$r), rel(subj=$s, pred="located_in", obj=$r), $s != $a
then: => "$s is also located in $r, like $a."

rule locx-shared-region-none
when: "what shares a region with $a", !rel(subj=$a, pred="located_in")
then: => "I don't know where $a is yet. Teach me with \"$a is located in ...\"."
```
