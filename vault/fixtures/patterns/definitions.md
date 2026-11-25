# Definitions

## p20: parse-define
- status: active
- kind: language
- trust: 0.9
- support: 4
- provenance: seed

### when
- text: define ?topic
- not_fact: definition_result()
- not_fact: definition_unknown()

### then
- tool: define(topic=?topic)

## p21: reply-definition
- status: active
- kind: reply
- trust: 0.9
- support: 4
- provenance: seed

### when
- fact: definition_result(text=?text, topic=?topic)

### then
- reply: ?topic: ?text

## p22: reply-definition-unknown
- status: active
- kind: reply
- trust: 0.85
- support: 2
- provenance: seed

### when
- fact: definition_unknown(topic=?topic)

### then
- reply: I don't have a definition for "?topic" yet. Add a `definition` fact or teach me!
