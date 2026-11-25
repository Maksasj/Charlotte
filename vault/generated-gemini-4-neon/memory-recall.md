# Generated Rules (`gemini-4-neon`)

Rules proposed and verified via the `vault-suggest` skill.

## User Workplace Recall

Adds direct recall for the `workplace` property already stored by `vault/core/persona/memory.md`.

```karin
rule recall-user-workplace
when: "where do I work", prop(entity="user", attr="workplace", value=$org)
then: => "You told me you work at $org."

rule recall-user-workplace-unknown
when: "where do I work", !prop(entity="user", attr="workplace")
then: => "I don't know where you work yet. Tell me \"I work at ...\" and I'll remember."
```
