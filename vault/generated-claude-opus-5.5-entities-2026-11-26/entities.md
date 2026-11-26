# Entities and the current speaker (claude-opus-5.5, 2026-11-26)

## Built-in entities

`user` is whoever talks to Karin when nobody said otherwise; `karin` is Karin.

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

The speaker is chat memory: every chat starts with the default speaker
(`chat_seed`, filled in by the engine before the first turn), and it changes
only when the user says someone else is talking. Questions about switching
lapse after two turns.

```karin
chat_expiry(rel="dialogue_pending_speaker", turns="2")
chat_seed(rel="speaker", entity="user")
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
