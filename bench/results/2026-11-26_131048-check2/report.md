# Karin language bench — check2

- Run: 2026-11-26_131048 · code: `117f260 + uncommitted changes` · server: http://127.0.0.1:8099
- Cases: 14 chats, 329 scored turns (283 in scope, 46 noise)
- Compared with: 2026-11-26_131037-check

## Headline

| Metric | Value | Meaning |
|---|---|---|
| **Accuracy** | **87.5%** | correct + correctly rejected noise, over all turns |
| Understanding | 88.0% | fully correct, over in-scope turns |
| **Wrong-answer rate** | **3.0%** | wrong meaning or noise taken as meaning (lower is better) |
| False-accept rate | 15.2% | noise answered as if understood |
| Missed rate | 10.2% | in-scope turns where Karin got stuck |

## Outcomes

| Outcome | Turns |
|---|---|
| correct | 251 |
| partial | 0 |
| value_error | 2 |
| wrong | 3 |
| missed | 29 |
| correct_reject | 37 |
| false_accept | 7 |

## By intent

| Intent | Turns | ✓ | partial | value err | wrong | missed | noise ok | false acc |
|---|---|---|---|---|---|---|---|---|
| noise | 46 | 2 | 0 | 0 | 0 | 0 | 37 | 7 |
| likes.ask | 30 | 29 | 0 | 0 | 0 | 1 | 0 | 0 |
| likes.tell | 22 | 19 | 0 | 0 | 1 | 2 | 0 | 0 |
| social.compliment | 22 | 21 | 0 | 0 | 1 | 0 | 0 | 0 |
| social.greet | 17 | 17 | 0 | 0 | 0 | 0 | 0 | 0 |
| persona.identity | 16 | 13 | 0 | 0 | 0 | 3 | 0 | 0 |
| note.create | 13 | 12 | 0 | 0 | 0 | 1 | 0 | 0 |
| people.rel.tell | 12 | 10 | 0 | 0 | 0 | 2 | 0 | 0 |
| calc | 10 | 9 | 0 | 1 | 0 | 0 | 0 | 0 |
| persona.name | 10 | 10 | 0 | 0 | 0 | 0 | 0 | 0 |
| social.complaint | 10 | 7 | 0 | 0 | 0 | 3 | 0 | 0 |
| user.name.ask | 10 | 10 | 0 | 0 | 0 | 0 | 0 | 0 |
| social.how_are_you | 9 | 8 | 0 | 0 | 0 | 1 | 0 | 0 |
| people.who_is | 8 | 8 | 0 | 0 | 0 | 0 | 0 | 0 |
| persona.called | 8 | 7 | 0 | 0 | 0 | 1 | 0 | 0 |
| people.rel.ask | 7 | 7 | 0 | 0 | 0 | 0 | 0 | 0 |
| user.name.tell | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 |
| chat.repeat | 5 | 1 | 0 | 0 | 0 | 4 | 0 | 0 |
| define | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 |
| fact.ask | 5 | 4 | 0 | 0 | 1 | 0 | 0 | 0 |
| persona.statement | 5 | 2 | 0 | 0 | 0 | 3 | 0 | 0 |
| unsupported.app | 5 | 3 | 0 | 0 | 0 | 2 | 0 | 0 |
| weather.ask | 5 | 4 | 0 | 1 | 0 | 0 | 0 | 0 |
| fact.teach | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 |
| help.capabilities | 4 | 3 | 0 | 0 | 0 | 1 | 0 | 0 |
| note.list | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 |
| persona.likes | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 |
| social.sorry | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 |
| social.stop | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 |
| unsupported.other | 4 | 0 | 0 | 0 | 0 | 4 | 0 | 0 |
| unsupported.new_chat | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 |
| likes.correction | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 |
| people.likes.tell | 2 | 1 | 0 | 0 | 0 | 1 | 0 | 0 |
| social.yes | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 |
| convert | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| people.rel.tell+people.city.tell | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| social.thanks | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| unsupported.how_to | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| user.role.tell | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| weather.cities | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |

## Wrong answers (3)

| Chat · turn | Said | Expected | Got | Reply |
|---|---|---|---|---|
| 619028·30 | what is a caramel macchiato | fact.ask | define | I don't have a definition for "caramel macchiato" yet. Add a definition fact or teach me! |
| 619028·102 | Ken is a smart | social.compliment (or noise) | fact.teach | Learned: Ken is a smart. |
| 620542·88 | maximum likes chicken | likes.tell (or noise) | people.likes.tell | Noted - maximum likes chicken. |

## Noise taken as meaning (7)

| Chat · turn | Said | Expected | Got | Reply |
|---|---|---|---|---|
| 619028·129 | can a roof is my best friend | noise | people.rel.tell | Got it - can a roof is your best friend. |
| 619028·165 | Karina is a Karen | noise (or persona.statement) | fact.teach | Learned: Karina is a Karen. |
| 620542·1 | Karina is a Karin | noise (or persona.statement) | fact.teach | Learned: Karina is a Karin. |
| 620542·43 | regular is a maxim | noise | fact.teach | Learned: regular is a maxim. |
| 620542·68 | what is a very smart | noise | define | I don't have a definition for "very smart" yet. Add a definition fact or teach me! |
| 620542·81 | is a smart a Karen | noise | fact.ask | I don't have facts showing that a smart is a Karen. |
| 620542·82 | is smart is a Karen | noise | fact.teach | Learned: is smart is a Karen. |

## Right meaning, missing value (2)

| Chat · turn | Said | Expected | Got | Reply |
|---|---|---|---|---|
| 619028·11 | what is the weather in a wellness | weather.ask | weather.ask — needs Vilnius | I don't have a weather fact for a wellness yet. You can add a weather_city fact in the Facts tab. |
| 620542·17 | calculate square root of 5 | calc | calc — needs 2.23 | Couldn't evaluate "square root of 5": invalid character 's'. |

## Missed (stuck on something in scope) (29)

| Chat · turn | Said | Expected | Got | Reply |
|---|---|---|---|---|
| 619028·19 | at which tools do you have | help.capabilities | stuck | I don't understand "at which tools do you have" yet. You can teach me a reply, or say "help" to hear what I… |
| 619028·20 | how many facts do you know | unsupported.other | stuck | I don't understand "how many facts do you know" yet. You can teach me a reply, or say "help" to hear what I… |
| 619028·56 | praise rust | unsupported.other (or noise) | stuck | I don't understand "praise rust" yet. You can teach me a reply, or say "help" to hear what I can do. |
| 619028·70 | can can is not a drink | persona.statement (or noise) | stuck | I don't understand "can can is not a drink" yet. You can teach me a reply, or say "help" to hear what I can… |
| 619028·80 | Oscar is best friend | people.rel.tell | stuck | I'm not sure what you meant. Tell me "Oscar is my friend", then ask "who is my friend". |
| 619028·81 | Oscar is friend | people.rel.tell | stuck | I'm not sure what you meant. Tell me "Oscar is my friend", then ask "who is my friend". |
| 619028·100 | learn much more stuff I want you to learn much much more stuff | unsupported.other | stuck | I don't understand "learn much more stuff I want you to learn much much more stuff" yet. You can teach me a… |
| 619028·107 | how many messages do we have in our chat right now | unsupported.other | stuck | I'm not sure what you meant. To start a new chat, use the New chat button. |
| 619028·111 | to draw anime | likes.tell (or noise) | stuck | I don't understand "to draw anime" yet. You can teach me a reply, or say "help" to hear what I can do. |
| 619028·113 | what do I like to draw | likes.ask | stuck | I'm not sure what you meant. Tell me "I like ...", then ask "what do I like". |
| 619028·119 | Aaron I like rock music | likes.tell | stuck | I'm not sure what you meant. Tell me "I like ...", then ask "what do I like". |
| 619028·124 | turn create a new chat in overall up under studio design | unsupported.app | stuck | I'm not sure what you meant. To start a new chat, use the New chat button. |
| 619028·125 | can open teams app | unsupported.app | stuck | I'm not sure what you meant. I can't open or control apps yet. |
| 619028·133 | Kelly you're not very smart right now | social.complaint | stuck | I'm not sure what you meant. I'm a symbolic assistant: I understand the phrasings I've been taught, and I l… |
| 619028·142 | Oscar like ice cream | people.likes.tell | stuck | I'm not sure what you meant. Tell me "I like ...", then ask "what do I like". |
| 619028·145 | can you're failing | social.complaint | stuck | I don't understand "can you're failing" yet. You can teach me a reply, or say "help" to hear what I can do. |
| 619028·146 | current how are you | social.how_are_you | stuck | I don't understand "current how are you" yet. You can teach me a reply, or say "help" to hear what I can do. |
| 619028·149 | turn who are you | persona.identity | stuck | I don't understand "turn who are you" yet. You can teach me a reply, or say "help" to hear what I can do. |
| 619028·150 | Ken who are you | persona.identity | stuck | I don't understand "Ken who are you" yet. You can teach me a reply, or say "help" to hear what I can do. |
| 620542·9 | Karen is you | persona.statement | stuck | I don't understand "Karen is you" yet. You can teach me a reply, or say "help" to hear what I can do. |
| 620542·11 | what is you | persona.identity | stuck | I don't understand "what is you" yet. You can teach me a reply, or say "help" to hear what I can do. |
| 620542·13 | can you repeat after afterwards | chat.repeat (or noise) | stuck | I don't understand "can you repeat after afterwards" yet. You can teach me a reply, or say "help" to hear w… |
| 620542·24 | no Karen is you not me | persona.statement (or noise) | stuck | I don't understand "no Karen is you not me" yet. You can teach me a reply, or say "help" to hear what I can… |
| 620542·46 | little girl you're so stupid | social.complaint | stuck | I'm not sure what you meant. Sorry - I only understand the phrasings I've been taught. Teach me what you me… |
| 620542·62 | can can you add more notes for me | note.create | stuck | I'm not sure what you meant. For notes, say "make a note <text>" or "list my notes". |
| 620542·84 | call Karen | persona.called (or noise) | stuck | I don't understand "call Karen" yet. You can teach me a reply, or say "help" to hear what I can do. |
| 620542·85 | and I just said nothing can you just repeat it | chat.repeat | stuck | I don't understand "and I just said nothing can you just repeat it" yet. You can teach me a reply, or say "… |
| 685756·3 | What I just said ? | chat.repeat | stuck | I don't understand "What I just said ?" yet. You can teach me a reply, or say "help" to hear what I can do. |
| 685963·4 | What I just said ? | chat.repeat | stuck | I don't understand "What I just said ?" yet. You can teach me a reply, or say "help" to hear what I can do. |
