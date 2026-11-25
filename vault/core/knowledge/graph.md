# Knowledge Graph & Inheritance

Domain-independent symbolic graph using `is_a(child, parent)` and `prop(entity, attr, value)`.
Supports conversational teaching, transitive multi-hop `is_a` queries, and automatic property inheritance with child overrides.

## Seed Graph Facts

```karin
is_a(child="Rust", parent="programming language")
is_a(child="Python", parent="programming language")
is_a(child="programming language", parent="tool")

prop(entity="programming language", attr="purpose", value="writing software")
prop(entity="Rust", attr="safety", value="memory-safe without a garbage collector")
prop(entity="Paris", attr="country", value="France")
prop(entity="Tokyo", attr="country", value="Japan")
```

## Teaching Facts in Conversation

```karin
rule graph-teach-is-a
when: "$child is a $parent", $child != $parent, $child != "what", $child != "who", $child != "is"
then:
  ++is_a(child=$child, parent=$parent)
  => "Learned: $child is a $parent."

rule graph-teach-prop
when: "the $attr of $entity is $val"
then:
  -prop(entity=$entity, attr=$attr)
  ++prop(entity=$entity, attr=$attr, value=$val)
  => "Stored: the $attr of $entity is $val."
```

## Transitive `is_a` Reasoning

```karin
rule graph-ask-is-a
when: "is $a a $b", !query_is_a()
then: +query_is_a(cur=$a, target=$b, orig=$a)

rule graph-is-a-match
when: query_is_a(cur=$cur, target=$target, orig=$orig), is_a(child=$cur, parent=$target)
then: => "Yes, $orig is a $target."

rule graph-is-a-hop
when: query_is_a(cur=$cur, target=$target, orig=$orig), is_a(child=$cur, parent=$mid), $mid != $target, !is_a(child=$cur, parent=$target), !visited_is_a(node=$mid)
then:
  -query_is_a
  +visited_is_a(node=$mid)
  +query_is_a(cur=$mid, target=$target, orig=$orig)

rule graph-is-a-unknown
when: query_is_a(cur=$cur, target=$target, orig=$orig), !is_a(child=$cur)
then: => "I don't have facts showing that $orig is a $target."
```

## Property Lookup & Inheritance

```karin
rule graph-ask-prop
when: "what is the $attr of $entity", !query_prop()
then: +query_prop(cur=$entity, attr=$attr, orig=$entity)

rule graph-prop-match
when: query_prop(cur=$cur, attr=$attr, orig=$orig), prop(entity=$cur, attr=$attr, value=$val)
then: => "The $attr of $orig is $val."

rule graph-prop-inherit
when: query_prop(cur=$cur, attr=$attr, orig=$orig), !prop(entity=$cur, attr=$attr), is_a(child=$cur, parent=$parent), !visited_prop(node=$parent)
then:
  -query_prop
  +visited_prop(node=$parent)
  +query_prop(cur=$parent, attr=$attr, orig=$orig)

rule graph-prop-unknown
when: query_prop(cur=$cur, attr=$attr, orig=$orig), !prop(entity=$cur, attr=$attr), !is_a(child=$cur)
then: => "I don't know the $attr of $orig yet."
```
