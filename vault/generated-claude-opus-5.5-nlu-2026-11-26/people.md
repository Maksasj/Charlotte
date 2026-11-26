# People and relations (`claude-opus-5.5`, 2026-11-26)

People the user mentions are stored as `rel(subj, pred, obj)` facts and, for
the current chat, as `mention(entity, kind="person")` so that "he" / "she"
in the next sentence resolves to them.

## The user

```karin
rule user-who-am-i
when: "who am i", user_name(value=$name)
then: => "You're $name."

rule user-who-am-i-unknown
when: "who am i", !user_name()
then: => "I don't know your name yet. Say \"my name is ...\"."

rule user-call-me
when: "call me $name"
then:
  -user_name
  ++user_name(value=$name)
  => "Okay, I'll call you $name."

rule user-i-am-a
when: "i am a $role"
then:
  -prop(entity="user", attr="role")
  ++prop(entity="user", attr="role", value=$role)
  => "Nice - I'll remember you're a $role."

rule user-i-am-an
when: "i am an $role"
then:
  -prop(entity="user", attr="role")
  ++prop(entity="user", attr="role", value=$role)
  => "Nice - I'll remember you're an $role."

rule user-what-am-i
when: "what am i", prop(entity="user", attr="role", value=$role)
then: => "You told me you're a $role."
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

rule user-i-am-known-name
when: "i am $name", user_name(value=$name)
then: => "Yes - you're $name. I remember."

rule user-i-am-state-negative
when: "i am $x", state_word(word=$x, mood="negative")
then: => "Sorry to hear you're $x. Is there anything I can help with?"

rule user-i-am-state-positive
when: "i am $x", state_word(word=$x, mood="positive")
then: => "Glad to hear you're $x!"

rule user-i-am-state-neutral
when: "i am $x", state_word(word=$x, mood="neutral")
then: => "Got it - you're $x."

rule user-i-am-maybe-name
when: "i am $x", !user_name(value=$x), !state_word(word=$x)
trust: 0.2
then:
  -dialogue_pending_name
  +dialogue_pending_name(value=$x)
  => "Is \"$x\" your name? Say yes and I'll remember it."

rule user-confirm-name-yes
when: "yes", dialogue_pending_name(value=$name)
then:
  -dialogue_pending_name
  -user_name
  ++user_name(value=$name)
  => "Nice to meet you, $name! I've stored your name in long-term memory."

rule user-confirm-name-no
when: "no", dialogue_pending_name(value=$x)
then:
  -dialogue_pending_name
  => "Okay, I won't store that as your name."
```

## "Oscar is my best friend", "my dad's name is Vasily"

```karin
rule people-teach-my-rel
when: "$x is my $rel", !intent(), $x != "what", $x != "who", $x != "it", $x != "this", $x != "that", $x != "you", $x != "he", $x != "she", $x != "there"
then:
  ++rel(subj="user", pred=$rel, obj=$x)
  +chat mention(entity=$x, kind="person")
  => "Got it - $x is your $rel."

rule people-teach-my-rel-name
when: "my $rel 's name is $x"
then:
  ++rel(subj="user", pred=$rel, obj=$x)
  +chat mention(entity=$x, kind="person")
  => "Got it - your $rel is $x."

rule people-teach-my-rel-is
when: "my $rel is $x", !intent(), $rel != "name", $rel != "favorite"
trust: 0.4
then:
  ++rel(subj="user", pred=$rel, obj=$x)
  +chat mention(entity=$x, kind="person")
  => "Got it - your $rel is $x."
```

## Asking about people

```karin
rule people-ask-my-rel
when: "who is my $rel", !asked_rel()
trust: 1.5
then:
  +asked_rel(rel=$rel)
  @collect(rel="rel", field="obj", tag="my_rel", subj="user", pred=$rel)

rule people-ask-what-is-my-rel
when: "what is my $rel", rel(subj="user", pred=$rel), !asked_rel()
trust: 1.5
then:
  +asked_rel(rel=$rel)
  @collect(rel="rel", field="obj", tag="my_rel", subj="user", pred=$rel)

rule people-ask-my-rel-name
when: "what is my $rel 's name", !asked_rel()
trust: 1.5
then:
  +asked_rel(rel=$rel)
  @collect(rel="rel", field="obj", tag="my_rel", subj="user", pred=$rel)

rule people-reply-my-rel
when: collected(tag="my_rel", list=$list), asked_rel(rel=$rel)
then: => "Your $rel: $list."

rule people-reply-my-rel-empty
when: collected_empty(tag="my_rel"), asked_rel(rel=$rel)
then: => "You haven't told me who your $rel is yet. Say \"<name> is my $rel\"."

rule people-who-is-known
when: "who is $x", rel(subj="user", pred=$rel, obj=$x), !asked_rel()
then:
  +chat mention(entity=$x, kind="person")
  => "$x is your $rel."

rule people-what-is-known
when: "what is $x", rel(subj="user", pred=$rel, obj=$x), $rel != "likes"
then:
  +chat mention(entity=$x, kind="person")
  => "$x is your $rel."

rule people-who-is-isa
when: "who is $x", is_a(child=$x, parent=$p), !rel(subj="user", obj=$x), !asked_rel()
then: => "$x is a $p."

rule people-who-is-unknown
when: "who is $x", !rel(subj="user", obj=$x), !is_a(child=$x), !asked_rel()
trust: 0.1
then: => "I don't know who $x is yet. Tell me, for example, \"$x is my friend\"."

rule people-is-my-rel-yes
when: "is $x my $rel", rel(subj="user", pred=$rel, obj=$x)
then: => "Yes, $x is your $rel."

rule people-is-my-rel-no
when: "is $x my $rel", !rel(subj="user", pred=$rel, obj=$x)
then: => "Not that I know of - you haven't told me $x is your $rel."
```

## What other people like and where they live

```karin
rule people-user-likes-by-name
when: "$x likes $y", user_name(value=$x)
then:
  ++rel(subj="user", pred="likes", obj=$y)
  => "Noted - I'll remember you like $y."

rule people-other-likes
when: "$x likes $y", !user_name(value=$x), $x != "who", $x != "what", $x != "everyone", $x != "he", $x != "she"
then:
  ++rel(subj=$x, pred="likes", obj=$y)
  +chat mention(entity=$x, kind="person")
  => "Noted - $x likes $y."

rule people-what-does-like
when: "what does $x like", !collected(tag="their_likes"), !collected_empty(tag="their_likes")
then:
  +asked_likes(who=$x)
  @collect(rel="rel", field="obj", tag="their_likes", subj=$x, pred="likes")

rule people-reply-their-likes
when: collected(tag="their_likes", list=$list), asked_likes(who=$x)
then: => "$x likes $list."

rule people-reply-their-likes-empty
when: collected_empty(tag="their_likes"), asked_likes(who=$x)
then: => "I don't know what $x likes yet."

rule people-lives-in
when: "$x lives in $y", $x != "who"
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
