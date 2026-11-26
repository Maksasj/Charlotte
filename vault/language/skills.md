# Asking for skills

Other ways to ask for a skill, turned into the same meanings the skill folders use.

## From skills/definitions/definitions.md

```karin
rule define-from-slot
when: slot_value(goal="define", slot="topic", value=$topic), !intent(name="define")
then: +intent(name="define", topic=$topic)
```

## From skills/notes/notes.md

```karin
rule note-from-slot
when: slot_value(goal="note", slot="text", value=$text), !intent(name="save_note")
then: +intent(name="save_note", text=$text)

rule note-make-colon
when: "make a note to $text", !intent(name="save_note")
then: +intent(name="save_note", text=$text)

rule note-note-that
when: "note that $text", !intent(name="save_note")
then: +intent(name="save_note", text=$text)
```

## From skills/weather/weather.md

```karin
rule weather-use-known-user-city
when: intent(name="weather_missing_city"), prop(entity="user", attr="city", value=$city), !intent(name="weather")
then: +intent(name="weather", city=$city)

rule weather-from-slot
when: slot_value(goal="weather", slot="city", value=$city), !intent(name="weather")
then: +intent(name="weather", city=$city)
```

## From core/skills/wordings.md

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

rule convert-how-many
when: "how many $to in $value $from", !intent(name="convert")
then: +intent(name="convert", value=$value, from=$from, to=$to)

rule convert-how-many-are
when: "how many $to are in $value $from", !intent(name="convert")
then: +intent(name="convert", value=$value, from=$from, to=$to)

rule define-what-is-fallback
when: "what is $topic", !is_a(child=$topic), !intent(), $topic != "you", $topic != "my", $topic != "me", $topic != "it", $topic != "this", $topic != "that"
trust: 0.1
then: +intent(name="define", topic=$topic)

rule alias-calc-how-much
when: "how much is $expr", !intent(name="calc")
then: +intent(name="calc", expr=$expr)

rule alias-define-tell-me
when: "tell me about $topic", !intent(name="define")
then: +intent(name="define", topic=$topic)

rule alias-weather-raining
when: "is it raining in $city", !intent(name="weather")
then: +intent(name="weather", city=$city)

rule alias-convert-into
when: "convert $value $from into $to", !intent(name="convert")
then: +intent(name="convert", value=$value, from=$from, to=$to)
```
