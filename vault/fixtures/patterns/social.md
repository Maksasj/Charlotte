# Social & Persona

## p1: greet-known-user
- status: active
- kind: reply
- trust: 0.95
- support: 5
- provenance: seed

### when
- text: hello
- fact: user_name(value=?u)
- fact: persona_name(value=?me)

### then
- reply: Hello ?u! I'm ?me. What shall we work on?

## p2: greet-default
- status: active
- kind: reply
- trust: 0.9
- support: 5
- provenance: seed

### when
- text: hello
- not_fact: user_name()
- fact: persona_name(value=?me)

### then
- reply: Hello! I'm ?me. Ask me for weather, math, definitions, notes, or teach me something new.

## p3: greet-hi
- status: active
- kind: reply
- trust: 0.9
- support: 4
- provenance: seed

### when
- text: hi
- fact: persona_name(value=?me)

### then
- reply: Hi! I'm ?me — ready when you are.

## p4: thanks
- status: active
- kind: reply
- trust: 0.85
- support: 4
- provenance: seed

### when
- contains: thank

### then
- reply: You're welcome! Let me know if there's anything else.

## p5: goodbye
- status: active
- kind: reply
- trust: 0.9
- support: 3
- provenance: seed

### when
- text: bye

### then
- reply: Goodbye! All long-term facts and patterns stay saved.

## p6: who-are-you
- status: active
- kind: reply
- trust: 0.95
- support: 5
- provenance: seed

### when
- text: who are you
- fact: persona_name(value=?me)
- fact: persona_role(value=?role)
- fact: persona_tone(value=?tone)

### then
- reply: I'm ?me, ?role. My style is ?tone.
