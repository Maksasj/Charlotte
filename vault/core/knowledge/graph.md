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
when: "$child is a $parent", $child != $parent, $child != "what", $child != "who", $child != "is", !not_thing(word=$child)
then:
  ++is_a(child=$child, parent=$parent)
  => "Learned: $child is a $parent."

rule graph-teach-is-an
when: "$child is an $parent", $child != $parent, $child != "what", $child != "who", $child != "is", !not_thing(word=$child)
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
then: => "I don't know the $attr of $orig yet. Teach me with \"the $attr of $orig is ...\"."
```

## Entity Summaries & Comparisons

```karin
rule graph-what-is-entity
when: "what is $entity", is_a(child=$entity, parent=$parent)
then: => "$entity is a $parent."

rule graph-compare-attr
when: "compare $attr of $a and $b", $a != $b, prop(entity=$a, attr=$attr, value=$va), prop(entity=$b, attr=$attr, value=$vb)
then: => "$a ($attr: $va) vs $b ($attr: $vb)."

rule graph-compare-weather
when: "compare $a and $b", $a != $b, weather_city(city=$a, condition=$ca, temp=$ta), weather_city(city=$b, condition=$cb, temp=$tb)
then: => "$a: $ca, $ta vs $b: $cb, $tb."

rule graph-compare-is-a
when: "compare $a and $b", $a != $b, is_a(child=$a, parent=$pa), is_a(child=$b, parent=$pb)
then: => "$a is a $pa; $b is a $pb."
```


## "What is X used for"

```karin
rule graph-used-for
when: "what is $x used for", !query_prop()
then: +query_prop(cur=$x, attr="purpose", orig=$x)
```


## Articles: "a husky" is "husky" (claude-opus-5.5, 2026-11-27)

A thing is stored without its article, so "a husky is a dog", "the husky is a
dog" and "husky is a dog" are one fact, and "what is a husky" finds it.

```karin
rule graph-teach-a-is-a
when: "a $child is a $parent", $child != $parent, !not_thing(word=$child)
then:
  ++is_a(child=$child, parent=$parent)
  => "Learned: a $child is a $parent."

rule graph-teach-a-is-an
when: "a $child is an $parent", $child != $parent, !not_thing(word=$child)
then:
  ++is_a(child=$child, parent=$parent)
  => "Learned: a $child is an $parent."

rule graph-teach-an-is-a
when: "an $child is a $parent", $child != $parent, !not_thing(word=$child)
then:
  ++is_a(child=$child, parent=$parent)
  => "Learned: an $child is a $parent."

rule graph-teach-the-is-a
when: "the $child is a $parent", $child != $parent, !not_thing(word=$child)
then:
  ++is_a(child=$child, parent=$parent)
  => "Learned: the $child is a $parent."

rule graph-ask-a-is-a
when: "is a $a a $b", !query_is_a()
then: +query_is_a(cur=$a, target=$b, orig=$a)

rule graph-ask-a-is-an
when: "is a $a an $b", !query_is_a()
then: +query_is_a(cur=$a, target=$b, orig=$a)

rule graph-ask-is-an
when: "is $a an $b", !query_is_a()
then: +query_is_a(cur=$a, target=$b, orig=$a)

rule graph-ask-the-is-a
when: "is the $a a $b", !query_is_a()
then: +query_is_a(cur=$a, target=$b, orig=$a)

rule graph-what-is-a-entity
when: "what is a $entity", is_a(child=$entity, parent=$parent)
then: => "A $entity is a $parent."

rule graph-what-is-an-entity
when: "what is an $entity", is_a(child=$entity, parent=$parent)
then: => "An $entity is a $parent."

rule graph-what-is-the-entity
when: "what is the $entity", is_a(child=$entity, parent=$parent)
then: => "The $entity is a $parent."
```

