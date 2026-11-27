# Knowledge questions

Wordings of knowledge questions, turned into meanings for `core/knowledge/`.

## Verbs that name a relation (claude-opus-5.5, 2026-11-27)

Used by the verb rules in `core/knowledge/relations.md`: "Google created Go",
"Go was made by Google", "who made Go".

```karin
relation_word(word="created", pred="created")
relation_word(word="made", pred="created")
relation_word(word="invented", pred="created")
relation_word(word="developed", pred="created")
relation_word(word="designed", pred="created")
relation_word(word="built", pred="created")
relation_word(word="founded", pred="founded")
relation_word(word="wrote", pred="wrote")
relation_word(word="written", pred="wrote")
relation_word(word="discovered", pred="discovered")
relation_word(word="painted", pred="painted")
relation_word(word="directed", pred="directed")
relation_word(word="composed", pred="composed")

inverse(pred="founded", inv="founded_by")
inverse(pred="founded_by", inv="founded")
inverse(pred="wrote", inv="written_by")
inverse(pred="written_by", inv="wrote")
inverse(pred="discovered", inv="discovered_by")
inverse(pred="discovered_by", inv="discovered")

rewrite(from="who is the creator of", to="who created")
rewrite(from="who is creator of", to="who created")
rewrite(from="who creator of", to="who created")
rewrite(from="is the creator of", to="created")
rewrite(from="is creator of", to="created")
rewrite(from="is a creator of", to="created")
rewrite(from="who is the author of", to="who wrote")
rewrite(from="is the author of", to="wrote")
```

## How a relation is said

Used when Karin tells what she knows about a thing (`core/knowledge/about.md`).
Read from subject to object: `rel(Paris, located_in, France)` -> "Paris is in France".

```karin
relation_phrase(pred="located_in", text="is in")
relation_phrase(pred="contains", text="contains")
relation_phrase(pred="part_of", text="is part of")
relation_phrase(pred="has_part", text="has the part")
relation_phrase(pred="created", text="created")
relation_phrase(pred="created_by", text="was created by")
relation_phrase(pred="founded", text="founded")
relation_phrase(pred="founded_by", text="was founded by")
relation_phrase(pred="wrote", text="wrote")
relation_phrase(pred="written_by", text="was written by")
relation_phrase(pred="discovered", text="discovered")
relation_phrase(pred="discovered_by", text="was discovered by")
relation_phrase(pred="painted", text="painted")
relation_phrase(pred="directed", text="directed")
relation_phrase(pred="composed", text="composed")
relation_phrase(pred="orbits", text="orbits")
relation_phrase(pred="orbited_by", text="is orbited by")
relation_phrase(pred="opposite_of", text="is the opposite of")
relation_phrase(pred="teaches", text="teaches")
relation_phrase(pred="taught_by", text="is taught by")
```

## Words that are never a thing

So remarks ("this is a test", "it is a bit bugged", "I made a mistake") are not
learned as facts.

```karin
not_thing(word="this")
not_thing(word="that")
not_thing(word="it")
not_thing(word="there")
not_thing(word="here")
not_thing(word="i")
not_thing(word="you")
not_thing(word="we")
not_thing(word="they")
not_thing(word="he")
not_thing(word="she")
not_thing(word="what")
not_thing(word="who")
not_thing(word="which")
not_thing(word="everything")
not_thing(word="nothing")
not_thing(word="something")
```

## Asking what Karin knows about a thing

```karin
rewrite(from="tell me about", to="what do you know about")
rewrite(from="what can you tell me about", to="what do you know about")
rewrite(from="what do you know of", to="what do you know about")
rewrite(from="describe", to="what do you know about")
```
