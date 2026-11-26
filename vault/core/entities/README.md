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
speaker(entity="guest12")                   chat memory: who is talking now
```

- Every chat starts with a fresh, anonymous **guest** (`chat_seed` with
  `@new:guest`). Karin does not know who is talking until told: "who am I"
  asks for a name, and personal statements are kept on the guest.
- When the guest names themselves: a new name makes the guest that person;
  a known name ("I'm Maxim") merges the guest into that person
  (`@merge_entities`), so whatever was said before introducing moves along.
- `user` is the id of the person known longest (older memory used it).
  New entities get their name as id ("Oscar").
- "I / me / my" in these rules means the current speaker; "you / Karin"
  means the entity `karin`.
- Once known, "I am Oscar" when Oscar is someone else Karin knows asks whether
  Oscar is speaking now; a yes (yeah, "yes it's Oscar") switches the speaker
  for this chat only.

`vault/core` is unchanged: where a core rule handles the same sentence, the
version here also checks `speaker(...)` and so wins. Rules from the earlier
nlu and understanding packs that stored people as plain strings were moved
here and removed there.
