# Generated Rules (`gemini-4-neon`)

Rules proposed and verified via the `vault-suggest` skill.

## User Preference & Workplace Recall

Adds direct recall for `favorite` and `workplace` properties already stored by `vault/core/persona/memory.md`.

```karin
rule recall-user-favorite
when: "what do I like", prop(entity="user", attr="favorite", value=$thing)
then: => "You told me you like $thing."

rule recall-user-favorite-unknown
when: "what do I like", !prop(entity="user", attr="favorite")
then: => "I don't know your preferences yet. Tell me \"I like ...\" and I'll remember."

rule recall-user-workplace
when: "where do I work", prop(entity="user", attr="workplace", value=$org)
then: => "You told me you work at $org."

rule recall-user-workplace-unknown
when: "where do I work", !prop(entity="user", attr="workplace")
then: => "I don't know where you work yet. Tell me \"I work at ...\" and I'll remember."
```
