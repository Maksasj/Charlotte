# Skills: wording extras (`claude-opus-5.5`, 2026-11-26)

The lexicon already maps "plus / times / divided by" to symbols, unit names to
their short forms, and "what is weather in" to "what is the weather in".

## Calculator

```karin
rule calc-divide
when: "divide $a by $b", !intent(name="calc")
then: +intent(name="calc", expr="$a / $b")

rule calc-multiply
when: "multiply $a by $b", !intent(name="calc")
then: +intent(name="calc", expr="$a * $b")

rule calc-add
when: "add $a and $b", !intent(name="calc")
then: +intent(name="calc", expr="$a + $b")

rule calc-subtract
when: "subtract $a from $b", !intent(name="calc")
then: +intent(name="calc", expr="$b - $a")

rule calc-what-is-plus
when: "what is $a + $b", !intent(name="calc")
then: +intent(name="calc", expr="$a + $b")

rule calc-what-is-minus
when: "what is $a - $b", !intent(name="calc")
then: +intent(name="calc", expr="$a - $b")

rule calc-what-is-times
when: "what is $a * $b", !intent(name="calc")
then: +intent(name="calc", expr="$a * $b")

rule calc-what-is-divided
when: "what is $a / $b", !intent(name="calc")
then: +intent(name="calc", expr="$a / $b")
```

## Units

```karin
rule convert-how-many
when: "how many $to in $value $from", !intent(name="convert")
then: +intent(name="convert", value=$value, from=$from, to=$to)

rule convert-how-many-are
when: "how many $to are in $value $from", !intent(name="convert")
then: +intent(name="convert", value=$value, from=$from, to=$to)

rule convert-unknown-unit
when: ~"convert", intent(name="convert", from=$from, to=$to), !unit_base(unit=$from), !temp_formula(from=$from)
then: => "I don't know the unit \"$from\" yet. I know mm, cm, m, km, in, ft, mi, mg, g, kg, lb, oz, s, min, h and day."

rule convert-unknown-target
when: ~"convert", intent(name="convert", from=$from, to=$to), unit_base(unit=$from), !unit_base(unit=$to)
then: => "I don't know the unit \"$to\" yet. I know mm, cm, m, km, in, ft, mi, mg, g, kg, lb, oz, s, min, h and day."
```

## Weather

```karin
rule weather-known-cities
when: "what cities do you know", !collected(tag="cities")
then: @collect(rel="weather_city", field="city", tag="cities")

rule weather-known-cities-alt
when: "what cities do you have", !collected(tag="cities")
then: @collect(rel="weather_city", field="city", tag="cities")

rule weather-reply-cities
when: collected(tag="cities", list=$list)
then: => "I have weather for $list."
```

## Definitions

"what is X" used to answer only when X had an `is_a` fact. Now it falls back to
the glossary, and "what is X used for" reads the inherited `purpose`.

```karin
definition(topic="programming language", text="a formal language for writing instructions a computer can run, like Rust or Python.")
definition(topic="rust", text="a systems programming language that is memory-safe without a garbage collector.")
definition(topic="python", text="a general-purpose programming language known for readable code.")

rule define-what-is-fallback
when: "what is $topic", !is_a(child=$topic), !intent(), $topic != "you", $topic != "my", $topic != "me", $topic != "it", $topic != "this", $topic != "that"
trust: 0.1
then: +intent(name="define", topic=$topic)

rule graph-used-for
when: "what is $x used for", !query_prop()
then: +query_prop(cur=$x, attr="purpose", orig=$x)
```
