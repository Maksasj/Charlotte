# People and relations (`claude-opus-5.5`, 2026-11-26)

People the user mentions are stored as `rel(subj, pred, obj)` facts and, for
the current chat, as `mention(entity, kind="person")` so that "he" / "she"
in the next sentence resolves to them.

## The user

```karin
```

## "I am Maksim" vs "I am tired"

"I am X" is ambiguous: a name, or how you feel. A known name is recognised, a
known state word gets a fitting reply, and anything else is only stored as your
name after you confirm it (the question is chat memory: `dialogue_pending_name`).

```karin
state_word(word="tired", mood="negative")
state_word(word="sleepy", mood="negative")
state_word(word="sad", mood="negative")
state_word(word="bored", mood="negative")
state_word(word="hungry", mood="negative")
state_word(word="sick", mood="negative")
state_word(word="ill", mood="negative")
state_word(word="stressed", mood="negative")
state_word(word="angry", mood="negative")
state_word(word="confused", mood="negative")
state_word(word="lost", mood="negative")
state_word(word="cold", mood="negative")
state_word(word="not sure", mood="neutral")
state_word(word="not ok", mood="negative")
state_word(word="not okay", mood="negative")
state_word(word="sorry", mood="neutral")
state_word(word="busy", mood="neutral")
state_word(word="here", mood="neutral")
state_word(word="back", mood="positive")
state_word(word="home", mood="neutral")
state_word(word="done", mood="neutral")
state_word(word="ready", mood="positive")
state_word(word="listening", mood="neutral")
state_word(word="serious", mood="neutral")
state_word(word="joking", mood="neutral")
state_word(word="kidding", mood="neutral")
state_word(word="wrong", mood="neutral")
state_word(word="right", mood="neutral")
state_word(word="fine", mood="positive")
state_word(word="ok", mood="positive")
state_word(word="okay", mood="positive")
state_word(word="alright", mood="positive")
state_word(word="good", mood="positive")
state_word(word="great", mood="positive")
state_word(word="happy", mood="positive")
state_word(word="excited", mood="positive")

rule user-i-am-state-negative
when: "i am $x", state_word(word=$x, mood="negative")
then: => "Sorry to hear you're $x. Is there anything I can help with?"

rule user-i-am-state-positive
when: "i am $x", state_word(word=$x, mood="positive")
then: => "Glad to hear you're $x!"

rule user-i-am-state-neutral
when: "i am $x", state_word(word=$x, mood="neutral")
then: => "Got it - you're $x."

```

## "Oscar is my best friend", "my dad's name is Vasily"

```karin
```

## Asking about people

```karin
rule people-reply-my-rel
when: collected(tag="my_rel", list=$list), asked_rel(rel=$rel)
then: => "Your $rel: $list."

rule people-reply-my-rel-empty
when: collected_empty(tag="my_rel"), asked_rel(rel=$rel)
then: => "You haven't told me who your $rel is yet. Say \"<name> is my $rel\"."

```

## What other people like and where they live

```karin
rule people-reply-their-likes
when: collected(tag="their_likes", list=$list), asked_likes(who=$x)
then: => "$x likes $list."

rule people-reply-their-likes-empty
when: collected_empty(tag="their_likes"), asked_likes(who=$x)
then: => "I don't know what $x likes yet."

rule people-lives-in
when: "$x lives in $y", words($x) <= 2, $x != "who"
then:
  -prop(entity=$x, attr="city")
  ++prop(entity=$x, attr="city", value=$y)
  +chat mention(entity=$x, kind="person")
  +chat mention(entity=$y, kind="place")
  => "Got it - $x lives in $y."

rule people-where-lives
when: "where does $x live", prop(entity=$x, attr="city", value=$y)
then: => "$x lives in $y."

rule people-where-lives-unknown
when: "where does $x live", !prop(entity=$x, attr="city")
then: => "I don't know where $x lives yet."
```
