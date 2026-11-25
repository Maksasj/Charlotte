# Language Intents

Decouples surface phrasings from skill execution by mapping natural user utterances into canonical `intent(...)` working facts.

## Weather Intents

```karin
rule intent-weather-city
when: "weather in $city", !intent(name="weather")
then: +intent(name="weather", city=$city)

rule intent-weather-whats
when: "what's the weather in $city", !intent(name="weather")
then: +intent(name="weather", city=$city)

rule intent-weather-what-is
when: "what is the weather in $city", !intent(name="weather")
then: +intent(name="weather", city=$city)

rule intent-weather-forecast-city
when: "forecast for $city", !intent(name="weather")
then: +intent(name="weather", city=$city)

rule intent-weather-how-hot
when: "how hot is $city", !intent(name="weather")
then: +intent(name="weather", city=$city)

rule intent-weather-how-is
when: "how is the weather in $city", !intent(name="weather")
then: +intent(name="weather", city=$city)

rule intent-weather-bare
when: "weather", !intent(name="weather_missing_city"), !intent(name="weather")
then: +intent(name="weather_missing_city")

rule intent-forecast-bare
when: "forecast", !intent(name="weather_missing_city"), !intent(name="weather")
then: +intent(name="weather_missing_city")
```

## Calculator Intents

```karin
rule intent-calc-calculate
when: "calculate $expr", !intent(name="calc")
then: +intent(name="calc", expr=$expr)

rule intent-calc-compute
when: "compute $expr", !intent(name="calc")
then: +intent(name="calc", expr=$expr)

rule intent-calc-eval
when: "eval $expr", !intent(name="calc")
then: +intent(name="calc", expr=$expr)

rule intent-calc-solve
when: "solve $expr", !intent(name="calc")
then: +intent(name="calc", expr=$expr)
```

## Definition Intents

```karin
rule intent-define
when: "define $topic", !intent(name="define")
then: +intent(name="define", topic=$topic)

rule intent-define-meaning
when: "what does $topic mean", !intent(name="define")
then: +intent(name="define", topic=$topic)

rule intent-define-explain
when: "explain $topic", !intent(name="define")
then: +intent(name="define", topic=$topic)

rule intent-define-could-explain
when: "could you explain $topic", !intent(name="define")
then: +intent(name="define", topic=$topic)

rule intent-define-bare
when: "define", !intent(name="define_missing_topic"), !intent(name="define")
then: +intent(name="define_missing_topic")
```

## Note Intents

```karin
rule intent-note-remember
when: "remember $text", !intent(name="save_note")
then: +intent(name="save_note", text=$text)

rule intent-note-make
when: "make a note $text", !intent(name="save_note")
then: +intent(name="save_note", text=$text)

rule intent-note-save
when: "save note $text", !intent(name="save_note")
then: +intent(name="save_note", text=$text)

rule intent-note-list-short
when: "my notes", !intent(name="list_notes")
then: +intent(name="list_notes")

rule intent-note-list-show
when: "show my notes", !intent(name="list_notes")
then: +intent(name="list_notes")

rule intent-note-list-all
when: "list my notes", !intent(name="list_notes")
then: +intent(name="list_notes")
```

## Unit Conversion Intents

```karin
rule intent-convert-explicit
when: "convert $value $from to $to", !intent(name="convert")
then: +intent(name="convert", value=$value, from=$from, to=$to)

rule intent-convert-short
when: "$value $from in $to", !intent(name="convert")
then: +intent(name="convert", value=$value, from=$from, to=$to)
```
