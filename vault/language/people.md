# People and who is speaking

Sentences about you, other people and Karin, turned into meanings (`speaker_is`, `rel_tell`, `who_is`, `person_likes`, `likes_topic`, `karin_rel_tell`). What each meaning does is in `core/entities/`.

## From core/entities/karin.md

```karin
rule karin-say-is-your
when: "$x is your $rel", !intent(), words($x) <= 2, $rel != "name", $x != "what", $x != "who", $x != "it", $x != "this", $x != "that"
trust: 3
then: +intent(name="karin_rel_tell", person=$x, rel=$rel)
```

## From core/entities/likes.md

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

## From core/entities/me.md

```karin
rule me-say-my-name-is
when: "my name is $n", !intent(), words($n) <= 3
trust: 3
then: +intent(name="speaker_is", value=$n, how="said")

rule me-say-call-me
when: "call me $n", !intent(), words($n) <= 3
trust: 3
then: +intent(name="speaker_is", value=$n, how="said")

rule me-say-is-me
when: "$n is me", !intent(), words($n) <= 2, $n != "it", $n != "this", $n != "that", $n != "who", $n != "what"
trust: 3
then: +intent(name="speaker_is", value=$n, how="alias")

rule me-say-hey-it-is
when: "hey it is $n", !intent(), words($n) <= 2
trust: 3
then: +intent(name="speaker_is", value=$n, how="said")

rule me-say-hey-this-is
when: "hey this is $n", !intent(), words($n) <= 2
trust: 3
then: +intent(name="speaker_is", value=$n, how="said")

rule me-say-i-am
when: "i am $n", !intent(), words($n) <= 1, !state_word(word=$n), $n != "here", $n != "back"
trust: 3
then: +intent(name="speaker_is", value=$n, how="i_am")

rule me-say-name-answer
when: slot_value(goal="identity", slot="name", value=$n), !intent(), words($n) <= 2
then: +intent(name="speaker_is", value=$n, how="said")
```

## From core/entities/people.md

```karin
rule rel-say-is-my
when: "$x is my $rel", !intent(), words($x) <= 2, $rel != "favorite", $x != "what", $x != "who", $x != "it", $x != "this", $x != "that", $x != "you", $x != "he", $x != "she", $x != "there"
trust: 3
then: +intent(name="rel_tell", person=$x, rel=$rel)

rule rel-say-my-is
when: "my $rel is $x", !intent(), words($x) <= 2, words($rel) <= 1, $rel != "name", $rel != "goal"
trust: 3
then: +intent(name="rel_tell", person=$x, rel=$rel)

rule rel-say-my-best-friend-is
when: "my best friend is $x", !intent(), words($x) <= 2
trust: 3
then: +intent(name="rel_tell", person=$x, rel="best friend")

rule rel-say-my-name-is
when: "my $rel 's name is $x", !intent(), words($x) <= 2
trust: 3
then: +intent(name="rel_tell", person=$x, rel=$rel)

rule rel-say-i-have-named
when: "i have a $rel named $x", !intent(), words($x) <= 2
trust: 3
then: +intent(name="rel_tell", person=$x, rel=$rel)

rule rel-say-i-have-called
when: "i have a $rel called $x", !intent(), words($x) <= 2
trust: 3
then: +intent(name="rel_tell", person=$x, rel=$rel)

rule rel-say-is-word
when: "$x is $rel", relation_word(word=$rel), !intent(), words($x) <= 2, $x != "who", $x != "what", $x != "it", $x != "he", $x != "she", $x != "this", $x != "that", $x != "you"
trust: 3
then: +intent(name="rel_tell", person=$x, rel=$rel)

rule who-say-who-is
when: "who is $x", !intent(), !asked_rel(), words($x) <= 3
trust: 3
then: +intent(name="who_is", value=$x)

rule who-say-do-you-know
when: "do you know $x", named(value=$x), !intent()
trust: 3
then: +intent(name="who_is", value=$x)

rule who-say-what-is-person
when: "what is $x", named(value=$x), !intent()
trust: 3
then: +intent(name="who_is", value=$x)

rule likes-say-person-likes
when: "$x likes $y", !intent(), words($x) <= 2, $x != "who", $x != "what", $x != "everyone", $x != "he", $x != "she", $x != "karin"
trust: 3
then: +intent(name="person_likes", person=$x, thing=$y)

rule likes-say-person-like
when: "$x like $y", named(value=$x), !intent(), $x != "i", $x != "you", $x != "we", $x != "they"
trust: 3
then: +intent(name="person_likes", person=$x, thing=$y)
```
