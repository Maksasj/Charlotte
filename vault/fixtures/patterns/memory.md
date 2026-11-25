# User Memory

## p7: learn-user-name
- status: active
- kind: language
- trust: 0.95
- support: 5
- provenance: seed

### when
- text: my name is ?name

### then
- retract: user_name
- assert[long_term]: user_name(value=?name)
- reply: Nice to meet you, ?name! I've stored your name in long-term memory.

## p8: recall-user-name
- status: active
- kind: reply
- trust: 0.95
- support: 5
- provenance: seed

### when
- text: what is my name
- fact: user_name(value=?name)

### then
- reply: Your name is ?name.

## p9: recall-user-name-unknown
- status: active
- kind: reply
- trust: 0.9
- support: 3
- provenance: seed

### when
- text: what is my name
- not_fact: user_name()

### then
- reply: You haven't told me your name yet. Say "my name is ..." and I'll remember it.
