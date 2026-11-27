# Definitions

Concept glossary facts and lookup rules for `"define <topic>"`.

## Glossary Facts

```karin
definition(topic="symbolic ai", text="AI built from explicit facts, rules, and reasoning steps rather than neural weights")
definition(topic="anti-unification", text="Finding the most specific general template that covers two or more concrete examples")
definition(topic="chunking", text="Compiling a successful multi-step chain of patterns into one reusable composite pattern")
```

## Lookup Rules

```karin
rule define-need-topic
when: intent(name="define_missing_topic"), !need_slot(goal="define", slot="topic")
then: +need_slot(goal="define", slot="topic", prompt="Which concept would you like me to define?")

rule run-define-tool
when: intent(name="define", topic=$topic), !definition_result(), !definition_unknown()
then: @define(topic=$topic)

rule reply-definition
when: definition_result(topic=$topic, text=$text)
then: => "$topic: $text."

rule reply-definition-unknown
when: definition_unknown(topic=$topic)
then: => "I don't have a definition for \"$topic\" yet. Add a definition fact or teach me!"
```


## More glossary entries

```karin
definition(topic="programming language", text="a formal language for writing instructions a computer can run, like Rust or Python")

definition(topic="rust", text="a systems programming language that is memory-safe without a garbage collector")

definition(topic="python", text="a general-purpose programming language known for readable code")
```

## Definitions in "what do you know about X"

The skill adds its definitions to the core "what Karin knows about a thing"
answer (`core/knowledge/about.md`), as one more line.

```karin
rule about-line-definition
trust: 3.0
when: about_thing(x=$x), definition(topic=$x, text=$d)
then: +about_line(x=$x, text="$x is $d")
```
