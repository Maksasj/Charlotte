# Definitions

definition(topic="symbolic ai", text="AI built from explicit facts, rules, and reasoning steps rather than neural weights.")
definition(topic="anti-unification", text="Finding the most specific general template that covers two or more concrete examples.")
definition(topic="chunking", text="Compiling a successful multi-step chain of patterns into one reusable composite pattern.")

## parse-define
when: "define ?topic", !definition_result(), !definition_unknown()
then: @define(topic=?topic)

## reply-definition
when: definition_result(topic=?topic, text=?text)
then: => "?topic: ?text"

## reply-definition-unknown
when: definition_unknown(topic=?topic)
then: => "I don't have a definition for \"?topic\" yet. Add a definition fact or teach me!"
