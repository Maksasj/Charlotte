# Entities and the current speaker (claude-opus-5.5, 2026-11-26)

## Built-in entities

`user` is the person Karin has known longest (older memory used this id);
`karin` is Karin. A chat itself starts with an unknown guest (below).

```karin
entity(id="user", kind="person")
default_speaker(entity="user")

entity(id="karin", kind="assistant")
display_name(entity="karin", value="Karin")
named(entity="karin", value="Karin")
named(entity="karin", value="Karen")
named(entity="karin", value="Karina")
```

## Who is speaking

The speaker is chat memory. Every chat starts with a **fresh guest** - an entity
Karin knows nothing about (`chat_seed` with `@new:guest`, filled in by the
engine before the first turn). When the guest says who they are, a new name
makes the guest that person; a name Karin knows merges the guest into that
person (`@merge_entities`), so whatever was said before moves along.
`user` stays the long-known person whose older memory used that id. Questions about switching
lapse after two turns.

```karin
chat_expiry(rel="dialogue_pending_speaker", turns="2")
chat_seed(rel="speaker", entity="@new:guest")
```

## Wording

```karin
rewrite(from="is the creator of you", to="is your creator")
rewrite(from="is a creator of you", to="is your creator")
rewrite(from="is creator of you", to="is your creator")
rewrite(from="is the creator of karin", to="is your creator")
rewrite(from="is creator of karin", to="is your creator")
rewrite(from="is a creator of karin", to="is your creator")
rewrite(from="is your creator", to="is your creator")
rewrite(from="who made you", to="who is your creator")
rewrite(from="who created you", to="who is your creator")
rewrite(from="who is the creator of you", to="who is your creator")
rewrite(from="who is your maker", to="who is your creator")
rewrite(from="what do you know about myself", to="what do you know about me")
rewrite(from="what do you remember about me", to="what do you know about me")
rewrite(from="tell me about myself", to="what do you know about me")
rewrite(from="do you know me", to="what do you know about me")
rewrite(from="do you know who i am", to="who am i")
rewrite(from="do you know who am i", to="who am i")
rewrite(from="what else do you know about", to="what do you know about")
rewrite(from="do you know anything about", to="what do you know about")
rewrite(from="i have another", to="i have a")
rewrite(from="i have an", to="i have a")
rewrite(from="it is me", to="it is me")
```
