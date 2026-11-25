# Definitions

Concept glossary facts and lookup rules for `"define <topic>"`.

## Glossary Facts

```karin
definition(topic="symbolic ai", text="AI built from explicit facts, rules, and reasoning steps rather than neural weights.")
definition(topic="anti-unification", text="Finding the most specific general template that covers two or more concrete examples.")
definition(topic="chunking", text="Compiling a successful multi-step chain of patterns into one reusable composite pattern.")
```

## Lookup Rules

```karin
rule define-need-topic
when: intent(name="define_missing_topic"), !need_slot(goal="define", slot="topic")
then: +need_slot(goal="define", slot="topic", prompt="Which concept would you like me to define?")

rule define-from-slot
when: slot_value(goal="define", slot="topic", value=$topic), !intent(name="define")
then: +intent(name="define", topic=$topic)

rule run-define-tool
when: intent(name="define", topic=$topic), !definition_result(), !definition_unknown()
then: @define(topic=$topic)

rule reply-definition
when: definition_result(topic=$topic, text=$text)
then: => "$topic: $text"

rule reply-definition-unknown
when: definition_unknown(topic=$topic)
then: => "I don't have a definition for \"$topic\" yet. Add a definition fact or teach me!"
```
