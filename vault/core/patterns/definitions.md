# Definitions

## parse-define
when: "define ?topic", !definition_result(), !definition_unknown()
then: @define(topic=?topic)

## reply-definition
when: definition_result(topic=?topic, text=?text)
then: => "?topic: ?text"

## reply-definition-unknown
when: definition_unknown(topic=?topic)
then: => "I don't have a definition for \"?topic\" yet. Add a definition fact or teach me!"
