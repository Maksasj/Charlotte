# Other people (claude-opus-5.5, 2026-11-26)

People are entities. A name Karin has never heard becomes a new entity (its id
is the name); a known name or nickname points at the existing one.

## Relations that imply others

A best friend is also a friend: asking for one finds the other.

```karin
rel_implies(from="best friend", to="friend")
rel_implies(from="girlfriend", to="partner")
rel_implies(from="boyfriend", to="partner")
rel_implies(from="wife", to="partner")
rel_implies(from="husband", to="partner")
rel_implies(from="mom", to="mother")
rel_implies(from="dad", to="father")
```

## Telling a relation: the meaning

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
```

## Telling a relation: what it means

```karin
rule rel-tell-known
when: intent(name="rel_tell", person=$x, rel=$rel), speaker(entity=$me), named(entity=$e, value=$x), $e != $me, $e != "karin"
trust: 2
then:
  ++rel(subj=$me, pred=$rel, obj=$e)
  +chat mention(entity=$x, kind="person")
  => "Got it - $x is your $rel."

rule rel-tell-new
when: intent(name="rel_tell", person=$x, rel=$rel), speaker(entity=$me), !named(value=$x)
trust: 2
then:
  ++entity(id=$x, kind="person")
  ++display_name(entity=$x, value=$x)
  ++named(entity=$x, value=$x)
  ++rel(subj=$me, pred=$rel, obj=$x)
  +chat mention(entity=$x, kind="person")
  => "Got it - $x is your $rel."

rule rel-tell-is-me
when: intent(name="rel_tell", person=$x, rel=$rel), speaker(entity=$me), named(entity=$me, value=$x)
trust: 2
then: => "That's you! You can't be your own $rel."
```

## Asking about relations

```karin
rule rel-ask-who-is-my
when: "who is my $rel", speaker(entity=$me), !asked_rel()
trust: 4
then:
  +asked_rel(rel=$rel)
  @collect(rel="rel", field="obj", tag="my_rel", subj=$me, pred=$rel)

rule rel-ask-who-is-my-implied
when: "who is my $rel", speaker(entity=$me), !rel(subj=$me, pred=$rel), rel_implies(from=$p, to=$rel), rel(subj=$me, pred=$p), !asked_rel()
trust: 5
then:
  +asked_rel(rel=$p)
  @collect(rel="rel", field="obj", tag="my_rel", subj=$me, pred=$p)

rule rel-ask-what-is-my
when: "what is my $rel", speaker(entity=$me), rel(subj=$me, pred=$rel), !asked_rel()
trust: 4
then:
  +asked_rel(rel=$rel)
  @collect(rel="rel", field="obj", tag="my_rel", subj=$me, pred=$rel)

rule rel-ask-my-name
when: "what is my $rel 's name", speaker(entity=$me), !asked_rel()
trust: 4
then:
  +asked_rel(rel=$rel)
  @collect(rel="rel", field="obj", tag="my_rel", subj=$me, pred=$rel)

rule rel-ask-is-my-yes
when: "is $x my $rel", speaker(entity=$me), named(entity=$e, value=$x), rel(subj=$me, pred=$rel, obj=$e)
then: => "Yes, $x is your $rel."

rule rel-ask-is-my-no
when: "is $x my $rel", speaker(entity=$me), !rel(subj=$me, pred=$rel)
then: => "Not that I know of - you haven't told me $x is your $rel."
```

## "Who is X": the meaning, and what Karin knows

```karin
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

rule who-is-me
when: intent(name="who_is", value=$x), speaker(entity=$me), named(entity=$me, value=$x)
trust: 3
then: => "That's you!"

rule who-is-karin
when: intent(name="who_is", value=$x), named(entity="karin", value=$x), persona_role(value=$role)
trust: 3
then: => "That's me - I'm Karin, $role."

rule who-is-my-relation
when: intent(name="who_is", value=$x), speaker(entity=$me), named(entity=$e, value=$x), rel(subj=$me, pred=$p, obj=$e), $p != "likes"
trust: 2
then:
  +chat mention(entity=$x, kind="person")
  => "$x is your $p."

rule who-is-their-relation-to-me
when: intent(name="who_is", value=$x), speaker(entity=$me), named(entity=$e, value=$x), rel(subj=$e, pred=$p, obj=$me), $p != "likes", !rel(subj=$me, obj=$e)
trust: 2
then:
  +chat mention(entity=$x, kind="person")
  => "$x knows you - you're their $p."

rule who-is-karins-relation
when: intent(name="who_is", value=$x), named(entity=$e, value=$x), rel(subj="karin", pred=$p, obj=$e)
trust: 1
then: => "$x is my $p."

rule who-is-someones-relation
when: intent(name="who_is", value=$x), speaker(entity=$me), named(entity=$e, value=$x), rel(subj=$s, pred=$p, obj=$e), display_name(entity=$s, value=$sn), $s != $me, $s != "karin", $p != "likes"
trust: 1
then: => "$x is $sn's $p."

rule who-is-known-name
when: intent(name="who_is", value=$x), named(entity=$e, value=$x), !rel(obj=$e), $e != "karin"
trust: 0.5
then: => "I know the name $x, but not much more yet."

rule who-is-a-thing
when: intent(name="who_is", value=$x), !named(value=$x), is_a(child=$x, parent=$p)
then: => "$x is a $p."

rule who-is-unknown
when: intent(name="who_is", value=$x), !named(value=$x), !is_a(child=$x)
trust: 0.1
then: => "I don't know who $x is yet. Tell me, for example, \"$x is my friend\"."
```

## What other people like

```karin
rule likes-say-person-likes
when: "$x likes $y", !intent(), words($x) <= 2, $x != "who", $x != "what", $x != "everyone", $x != "he", $x != "she", $x != "karin"
trust: 3
then: +intent(name="person_likes", person=$x, thing=$y)

rule likes-say-person-like
when: "$x like $y", named(value=$x), !intent(), $x != "i", $x != "you", $x != "we", $x != "they"
trust: 3
then: +intent(name="person_likes", person=$x, thing=$y)

rule likes-person-is-me
when: intent(name="person_likes", person=$x, thing=$y), speaker(entity=$me), named(entity=$me, value=$x)
trust: 2
then:
  ++rel(subj=$me, pred="likes", obj=$y)
  => "Noted - I'll remember you like $y."

rule likes-person-known
when: intent(name="person_likes", person=$x, thing=$y), speaker(entity=$me), named(entity=$e, value=$x), $e != $me, $e != "karin"
trust: 2
then:
  ++rel(subj=$e, pred="likes", obj=$y)
  +chat mention(entity=$x, kind="person")
  => "Noted - $x likes $y."

rule likes-person-new
when: intent(name="person_likes", person=$x, thing=$y), !named(value=$x)
trust: 2
then:
  ++entity(id=$x, kind="person")
  ++display_name(entity=$x, value=$x)
  ++named(entity=$x, value=$x)
  ++rel(subj=$x, pred="likes", obj=$y)
  +chat mention(entity=$x, kind="person")
  => "Noted - $x likes $y."

rule likes-ask-what-does
when: "what does $x like", named(entity=$e, value=$x), !collected(tag="their_likes"), !collected_empty(tag="their_likes")
trust: 2
then:
  +asked_likes(who=$x)
  @collect(rel="rel", field="obj", tag="their_likes", subj=$e, pred="likes")
```

## "What do you know about X"

```karin
rule about-say
when: "what do you know about $x", named(entity=$e, value=$x), speaker(entity=$me), $e != $me, !about()
trust: 3
then:
  +about(entity=$e, name=$x)
  @collect(rel="rel", field="obj", tag="about", subj=$e, pred="likes")

rule about-reply-relation-likes
when: collected(tag="about", list=$l), about(entity=$e, name=$x), speaker(entity=$me), rel(subj=$me, pred=$p, obj=$e), $p != "likes"
trust: 2
then: => "$x is your $p, and likes $l."

rule about-reply-relation
when: collected_empty(tag="about"), about(entity=$e, name=$x), speaker(entity=$me), rel(subj=$me, pred=$p, obj=$e), $p != "likes"
trust: 2
then: => "$x is your $p. I don't know what they like yet."

rule about-reply-likes
when: collected(tag="about", list=$l), about(entity=$e, name=$x), speaker(entity=$me), !rel(subj=$me, obj=$e)
trust: 2
then: => "$x likes $l."

rule about-reply-little
when: collected_empty(tag="about"), about(entity=$e, name=$x), speaker(entity=$me), !rel(subj=$me, obj=$e), $e != "karin"
trust: 2
then: => "I know the name $x, but not much more yet."
```
