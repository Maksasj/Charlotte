# Likes by category (`claude-opus-5.5`, 2026-11-26)

`vault/core/persona/memory.md` stores "I like black tea" without a category and
"my favorite drink is black tea" with one. Connecting the two ("black tea is a
drink" → it counts as a drink you like) is deliberately *not* hard-coded here:
the core learns that rule from a correction ("what about black tea?" →
"approve"), see `vault/core/learning/correction.md`.

## Categories people talk about

```karin
topic_word(word="music")
topic_word(word="food")
topic_word(word="drink")
topic_word(word="movie")
topic_word(word="film")
topic_word(word="book")
topic_word(word="game")
topic_word(word="song")
topic_word(word="color")
topic_word(word="sport")
topic_word(word="animal")
```

## More ways to ask

```karin
rule likes-ask-bare-my-favorite
when: "my favorite $topic", !intent(name="likes_topic")
then: +intent(name="likes_topic", topic=$topic)

rule likes-ask-bare-favorite
when: "favorite $topic", !intent(name="likes_topic")
then: +intent(name="likes_topic", topic=$topic)

rule likes-ask-what-is-favorite
when: "what $topic is my favorite", !intent(name="likes_topic")
trust: 2.0
then: +intent(name="likes_topic", topic=$topic)

rule likes-ask-what-are-favorite
when: "what are my favorite $topic", !intent(name="likes_topic")
then: +intent(name="likes_topic", topic=$topic)

rule likes-ask-what-is-do-i-like
when: "what is $topic do i like", !intent(name="likes_topic")
then: +intent(name="likes_topic", topic=$topic)

rule likes-ask-tell-favorite
when: "tell me my favorite $topic", !intent(name="likes_topic")
then: +intent(name="likes_topic", topic=$topic)
```

## More ways to tell

```karin
rule likes-teach-thing-topic
when: "i like $thing $topic", topic_word(word=$topic)
then:
  ++rel(subj="user", pred="likes", topic=$topic, obj=$thing)
  => "Got it - I'll remember $thing as a $topic you like."

rule likes-teach-is-my-favorite
when: "$thing is my favorite $topic", !intent(), $thing != "what", $thing != "who", $thing != "which", $thing != "it", $thing != "this", $thing != "that"
then:
  ++rel(subj="user", pred="likes", topic=$topic, obj=$thing)
  => "Got it - I'll remember $thing as a $topic you like."

rule likes-teach-listening
when: "i started listening to $thing"
then:
  ++rel(subj="user", pred="likes", topic="music", obj=$thing)
  => "Nice - I'll remember you listen to $thing."

rule likes-teach-listen
when: "i listen to $thing"
then:
  ++rel(subj="user", pred="likes", topic="music", obj=$thing)
  => "Nice - I'll remember you listen to $thing."
```
