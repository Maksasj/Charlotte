# What Karin knows about a thing (claude-opus-5.5, 2026-11-27)

"What do you know about Paris" gathers every fact about the thing - what it
is, its properties, its relations in both directions - instead of looking for
one definition. People have their own version in `entities/people.md`
(it wins for anyone with a `named` fact).

How it works, in rules only:

1. The question adds `about_thing(x=...)`.
2. High-trust rules turn each matching fact into one `about_line(text=...)`
   (trust makes them fire before the reply, one line per fact).
3. Then `@collect` joins the lines and the reply says them.

Relations are worded by `relation_phrase(pred, text)` (in
`language/knowledge.md`); a relation without a phrase is left out rather than
said as "located_in".

```karin
rule about-thing-ask
when: "what do you know about $x", $x != "me", $x != "you", $x != "yourself", !named(value=$x), !about_thing()
then: +about_thing(x=$x)

rule about-thing-ask-the
when: "what do you know about the $x", !named(value=$x), !about_thing()
then: +about_thing(x=$x)

rule about-line-is-a
trust: 3.0
when: about_thing(x=$x), is_a(child=$x, parent=$p)
then: +about_line(x=$x, text="$x is a $p")

rule about-line-kinds
trust: 3.0
when: about_thing(x=$x), is_a(child=$c, parent=$x)
then: +about_line(x=$x, text="$c is a kind of $x")

rule about-line-prop
trust: 3.0
when: about_thing(x=$x), prop(entity=$x, attr=$a, value=$v)
then: +about_line(x=$x, text="its $a is $v")

rule about-line-rel
trust: 3.0
when: about_thing(x=$x), rel(subj=$x, pred=$r, obj=$o), relation_phrase(pred=$r, text=$t)
then: +about_line(x=$x, text="$x $t $o")

rule about-line-rel-back
trust: 3.0
when: about_thing(x=$x), rel(subj=$s, pred=$r, obj=$x), relation_phrase(pred=$r, text=$t)
then: +about_line(x=$x, text="$s $t $x")

rule about-thing-gather
when: about_thing(x=$x), about_line(x=$x), !collected(tag="about_thing")
then: @collect(rel="about_line", field="text", tag="about_thing", sep="; ", x=$x)

rule about-thing-say
when: about_thing(x=$x), collected(tag="about_thing", list=$l)
then: => "Here's what I know about $x: $l."

rule about-thing-unknown
when: about_thing(x=$x), !about_line(x=$x)
then: => "I don't know anything about $x yet. Tell me something, like \"$x is a ...\"."
```
