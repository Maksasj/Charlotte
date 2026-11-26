# Entities pack - made by Claude (Opus 5.5) on 2026-11-26

Experiment on branch `entity-model`: people and things become **entities** that
facts point at, and Karin keeps track of **who is speaking**.

| file | what it holds |
|------|---------------|
| `entities.md` | the entity facts (`entity`, `display_name`, `named`), the built-in entities (the default speaker `user` and Karin herself `karin`), the current speaker, and wording rewrites |
| `me.md` | everything about the current speaker: name, nicknames, "I am X", "X is me", role, likes, "what do you know about me", switching to another speaker |
| `people.md` | other people: relations to the speaker, "who is X", what they like, "what do you know about X" |
| `karin.md` | Karin's own relations ("Max is your creator", "who made you") |

## The model

```text
entity(id="oscar", kind="person")         one per person / thing
display_name(entity="oscar", value="Oscar")  how Karin says it
named(entity="oscar", value="Oscar")         every name it answers to (nicknames too)
rel(subj="user", pred="friend", obj="oscar") relations point at entity ids
speaker(entity="user")                      chat memory: who is talking now
```

- `user` is the default speaker, so older rules that say `subj="user"` keep
  working for them. New entities get their name as id ("Oscar").
- "I / me / my" in these rules means the current speaker; "you / Karin"
  means the entity `karin`.
- "I am Oscar" when Oscar is someone Karin already knows asks whether Oscar is
  speaking now; a yes switches the speaker for this chat only.

`vault/core` is unchanged: where a core rule handles the same sentence, the
version here also checks `speaker(...)` and so wins. Rules from the earlier
nlu and understanding packs that stored people as plain strings were moved
here and removed there.
