# The current speaker (claude-opus-5.5, 2026-11-26)

Everything "I / me / my" means the entity in `speaker(entity=...)`. Every chat
starts with a fresh guest, a person Karin does not know yet: no `display_name`. When an
unknown speaker tells Karin something about themselves, she keeps it on that
entity and asks who she is talking to; the answer names the entity, or, if it
is someone she already knows, offers to switch to them.

## "My name is X", "I am X", "X is me": the meaning

Wording rules only produce `intent(name="speaker_is", value=X, how=...)`;
the behaviour rules below decide what it means for the entities.


## Wording

```karin
rewrite(from="yes it is", to="yes")
rewrite(from="yes that is me", to="yes")
rewrite(from="yes it is me", to="yes")
rewrite(from="yeah", to="yes")
rewrite(from="yep", to="yes")
rewrite(from="yup", to="yes")
rewrite(from="hey my name is", to="my name is")
rewrite(from="hi my name is", to="my name is")
rewrite(from="hello my name is", to="my name is")
rewrite(from="hey i am", to="i am")
rewrite(from="hi i am", to="i am")
rewrite(from="hello i am", to="i am")
```

## "My name is X": what it means

```karin
rule me-guest-is-known
when: intent(name="speaker_is", value=$n), speaker(entity=$me), !display_name(entity=$me), named(entity=$o, value=$n), $o != $me, $o != "karin"
trust: 3
then:
  @merge_entities(from=$me, into=$o)
  => "Welcome back, $n!"

rule me-is-already-me
when: intent(name="speaker_is", value=$n), speaker(entity=$me), named(entity=$me, value=$n)
trust: 2
then:
  -display_name(entity=$me)
  ++display_name(entity=$me, value=$n)
  => "Yes - you're $n. I remember."

rule me-is-karin
when: intent(name="speaker_is", value=$n), named(entity="karin", value=$n)
trust: 2
then: => "That's my name! What's yours?"

rule me-is-someone-known-switch
when: intent(name="speaker_is", value=$n, how=$how), $how != "i_am", speaker(entity=$me), named(entity=$o, value=$n), $o != $me, $o != "karin"
trust: 2
then:
  -dialogue_pending_speaker
  -speaker
  +chat speaker(entity=$o)
  +chat speaker_switched(entity=$o)
  -display_name(entity=$o)
  ++display_name(entity=$o, value=$n)
  => "Hi $n! I'll talk to you as $n now."

rule me-is-someone-known
when: intent(name="speaker_is", value=$n, how="i_am"), speaker(entity=$me), named(entity=$o, value=$n), $o != $me, $o != "karin"
trust: 2
then:
  -dialogue_pending_speaker
  +chat dialogue_pending_speaker(entity=$o, name=$n)
  => "I know someone called $n already. Is $n talking to me now? Say yes and I'll talk to you as $n."

rule me-is-new-name
when: intent(name="speaker_is", value=$n, how=$how), $how != "i_am", speaker(entity=$me), !named(value=$n), !display_name(entity=$me)
trust: 2
then:
  ++entity(id=$me, kind="person")
  ++display_name(entity=$me, value=$n)
  ++named(entity=$me, value=$n)
  -dialogue_slot(goal="identity")
  => "Nice to meet you, $n! I've stored your name in long-term memory."

rule me-is-new-person
when: intent(name="speaker_is", value=$n, how=$how), $how != "i_am", speaker(entity=$me), speaker_switched(entity=$me), !named(value=$n), display_name(entity=$me)
trust: 2
then:
  ++entity(id=$n, kind="person")
  ++display_name(entity=$n, value=$n)
  ++named(entity=$n, value=$n)
  -speaker
  +chat speaker(entity=$n)
  +chat speaker_switched(entity=$n)
  => "Nice to meet you, $n! I'll talk to you as $n now."

rule me-is-another-name
when: intent(name="speaker_is", value=$n, how="said"), speaker(entity=$me), !speaker_switched(entity=$me), !named(value=$n), display_name(entity=$me, value=$old)
trust: 2
then:
  -display_name(entity=$me)
  ++display_name(entity=$me, value=$n)
  ++named(entity=$me, value=$n)
  => "Got it - I'll call you $n. I'll still know you as $old too."

rule me-is-nickname
when: intent(name="speaker_is", value=$n, how="alias"), speaker(entity=$me), !named(value=$n), display_name(entity=$me, value=$main)
trust: 2
then:
  ++named(entity=$me, value=$n)
  => "Got it - you're also $n. I'll keep calling you $main."

rule me-is-maybe-name
when: intent(name="speaker_is", value=$n, how="i_am"), speaker(entity=$me), !named(value=$n)
trust: 2
then:
  -dialogue_pending_name
  +chat dialogue_pending_name(value=$n)
  => "Is \"$n\" your name? Say yes and I'll remember it."

rule me-confirm-name-yes
when: "yes", dialogue_pending_name(value=$n), speaker(entity=$me)
trust: 2
then:
  ++entity(id=$me, kind="person")
  -dialogue_pending_name
  -display_name(entity=$me)
  ++display_name(entity=$me, value=$n)
  ++named(entity=$me, value=$n)
  => "Nice to meet you, $n! I've stored your name in long-term memory."

rule me-confirm-name-no
when: "no", dialogue_pending_name(value=$n)
trust: 2
then:
  -dialogue_pending_name
  => "Okay, I won't store that as your name."
```

## Switching to another speaker

```karin
rule me-switch-yes
when: "yes", dialogue_pending_speaker(entity=$o, name=$n), speaker(entity=$me)
trust: 2
then:
  -dialogue_pending_speaker
  -speaker
  +chat speaker(entity=$o)
  +chat speaker_switched(entity=$o)
  => "Hi $n! In this chat I'll keep what you tell me as yours."

rule me-switch-yes-name
when: "yes $n", dialogue_pending_speaker(entity=$o, name=$n), speaker(entity=$me)
trust: 2
then:
  -dialogue_pending_speaker
  -speaker
  +chat speaker(entity=$o)
  +chat speaker_switched(entity=$o)
  => "Hi $n! In this chat I'll keep what you tell me as yours."

rule me-switch-yes-i-am
when: "yes i am", dialogue_pending_speaker(entity=$o, name=$n), speaker(entity=$me)
trust: 2
then:
  -dialogue_pending_speaker
  -speaker
  +chat speaker(entity=$o)
  +chat speaker_switched(entity=$o)
  => "Hi $n! In this chat I'll keep what you tell me as yours."

rule me-confirm-name-yes-i-am
when: "yes i am", dialogue_pending_name(value=$n), speaker(entity=$me)
trust: 2
then:
  -dialogue_pending_name
  -display_name(entity=$me)
  ++display_name(entity=$me, value=$n)
  ++named(entity=$me, value=$n)
  => "Nice to meet you, $n! I've stored your name in long-term memory."

rule me-switch-no
when: "no", dialogue_pending_speaker(entity=$o)
trust: 2
then:
  -dialogue_pending_speaker
  => "Okay - I'll keep talking to you as before."
```

## Asking who the speaker is

```karin
rule me-who-am-i
when: "who am i", speaker(entity=$me), display_name(entity=$me, value=$n)
then: => "You're $n."

rule me-who-am-i-unknown
when: "who am i", speaker(entity=$me), !display_name(entity=$me)
then:
  +chat dialogue_slot(goal="identity", slot="name")
  => "I don't know yet - what's your name?"

rule me-what-is-my-name
when: "what is my name", speaker(entity=$me), display_name(entity=$me, value=$n)
then: => "Your name is $n."

rule me-what-is-my-name-unknown
when: "what is my name", speaker(entity=$me), !display_name(entity=$me)
then:
  +chat dialogue_slot(goal="identity", slot="name")
  => "You haven't told me your name yet - what is it?"

rule me-greet-guest
when: "hello", speaker(entity=$me), !display_name(entity=$me), persona_name(value=$k)
trust: 2
then:
  +chat dialogue_slot(goal="identity", slot="name")
  => "Hello! I'm $k. Who am I talking to?"

rule me-greet-known
when: "hello", speaker(entity=$me), display_name(entity=$me, value=$n), persona_name(value=$k)
then: => "Hello $n! I'm $k. What shall we work on?"

rule me-sync-legacy-name
when: speaker(entity="user"), display_name(entity="user", value=$n), !user_name(value=$n)
trust: 5
then:
  -user_name
  ++user_name(value=$n)
```

## Role

```karin
rule me-role-a
when: "i am a $role", speaker(entity=$me)
then:
  -prop(entity=$me, attr="role")
  ++prop(entity=$me, attr="role", value=$role)
  => "Nice - I'll remember you're a $role."

rule me-role-an
when: "i am an $role", speaker(entity=$me)
then:
  -prop(entity=$me, attr="role")
  ++prop(entity=$me, attr="role", value=$role)
  => "Nice - I'll remember you're an $role."

rule me-what-am-i
when: "what am i", speaker(entity=$me), prop(entity=$me, attr="role", value=$role)
then: => "You told me you're a $role."
```

## Likes (the speaker's)

An unknown speaker's likes are kept, and Karin asks who they are.

```karin
rule me-like
when: "i like $thing", speaker(entity=$me), display_name(entity=$me)
then:
  ++rel(subj=$me, pred="likes", obj=$thing)
  => "Noted - I'll remember you like $thing."

rule me-like-unknown-speaker
when: "i like $thing", speaker(entity=$me), !display_name(entity=$me)
then:
  ++rel(subj=$me, pred="likes", obj=$thing)
  +chat dialogue_slot(goal="identity", slot="name")
  => "Noted - you like $thing. By the way, who am I talking to?"

rule me-like-thing-topic
when: "i like $thing $topic", topic_word(word=$topic), speaker(entity=$me)
then:
  ++rel(subj=$me, pred="likes", topic=$topic, obj=$thing)
  => "Got it - I'll remember $thing as a $topic you like."

rule me-favorite-a
when: "my favorite $topic is a $thing", speaker(entity=$me)
then:
  -dialogue_topic
  +dialogue_topic(kind="likes", topic=$topic)
  ++rel(subj=$me, pred="likes", topic=$topic, obj=$thing)
  => "Got it - I'll remember $thing as a $topic you like."

rule me-favorite
when: "my favorite $topic is $thing", speaker(entity=$me)
then:
  -dialogue_topic
  +dialogue_topic(kind="likes", topic=$topic)
  ++rel(subj=$me, pred="likes", topic=$topic, obj=$thing)
  => "Got it - I'll remember $thing as a $topic you like."

rule me-thing-is-favorite
when: "$thing is my favorite $topic", speaker(entity=$me), !intent(), $thing != "what", $thing != "who", $thing != "it", $thing != "this", $thing != "that"
then:
  ++rel(subj=$me, pred="likes", topic=$topic, obj=$thing)
  => "Got it - I'll remember $thing as a $topic you like."

rule me-also-like-topic
when: "i also like $thing", speaker(entity=$me), dialogue_topic(kind="likes", topic=$topic)
then:
  ++rel(subj=$me, pred="likes", topic=$topic, obj=$thing)
  => "Got it - added $thing to the $topic you like."

rule me-also-like
when: "i also like $thing", speaker(entity=$me), !dialogue_topic(kind="likes")
then:
  ++rel(subj=$me, pred="likes", obj=$thing)
  => "Got it - added $thing to what you like."

rule me-listening
when: "i started listening to $thing", speaker(entity=$me)
then:
  ++rel(subj=$me, pred="likes", topic="music", obj=$thing)
  => "Nice - I'll remember you listen to $thing."

rule me-listen
when: "i listen to $thing", speaker(entity=$me)
then:
  ++rel(subj=$me, pred="likes", topic="music", obj=$thing)
  => "Nice - I'll remember you listen to $thing."

rule me-ask-likes-guest
when: "what do i like", speaker(entity=$me), !display_name(entity=$me), !rel(subj=$me, pred="likes")
trust: 2
then:
  +chat dialogue_slot(goal="identity", slot="name")
  => "I don't know who you are yet - what's your name?"

rule me-ask-likes
when: "what do i like", speaker(entity=$me), !collected(tag="likes"), !collected_empty(tag="likes")
then:
  -dialogue_topic
  @collect(rel="rel", field="obj", tag="likes", subj=$me, pred="likes")

rule me-ask-likes-topic
when: intent(name="likes_topic", topic=$topic), speaker(entity=$me), !collected(tag="likes_topic"), !collected_empty(tag="likes_topic"), !collected(tag="likes"), !collected_empty(tag="likes")
trust: 0.1
then:
  -dialogue_topic
  +dialogue_topic(kind="likes", topic=$topic)
  @collect(rel="rel", field="obj", tag="likes_topic", subj=$me, pred="likes", topic=$topic)
```

Priority note: `me-ask-likes-topic` runs above the core lookup but below rules
learned from corrections (likes + is_a), which must tag likes before they are
collected.

## Where you live and work, your goal, your projects

Older rules keep these on the fixed id `user`; these keep them on whoever is
speaking (a guest's move along when the guest is merged into a known person).

```karin
rule me-live-in
when: "i live in $city", speaker(entity=$me), words($city) <= 3
trust: 2
then:
  -prop(entity=$me, attr="city")
  ++prop(entity=$me, attr="city", value=$city)
  => "Got it - I'll remember you live in $city."

rule me-where-do-i-live
when: "where do i live", speaker(entity=$me), prop(entity=$me, attr="city", value=$city)
trust: 2
then: => "You live in $city."

rule me-where-do-i-live-unknown
when: "where do i live", speaker(entity=$me), !prop(entity=$me, attr="city")
trust: 2
then: => "I don't know where you live yet. Tell me \"I live in ...\"."

rule me-work-at
when: "i work at $org", speaker(entity=$me), words($org) <= 4
trust: 2
then:
  -prop(entity=$me, attr="workplace")
  ++prop(entity=$me, attr="workplace", value=$org)
  => "Noted - I'll remember you work at $org."

rule me-where-do-i-work
when: "where do i work", speaker(entity=$me), prop(entity=$me, attr="workplace", value=$org)
trust: 2
then: => "You told me you work at $org."

rule me-where-do-i-work-unknown
when: "where do i work", speaker(entity=$me), !prop(entity=$me, attr="workplace")
trust: 2
then: => "I don't know where you work yet. Tell me \"I work at ...\" and I'll remember."

rule me-goal-set
when: "my goal is $goal", speaker(entity=$me)
trust: 2
then:
  -prop(entity=$me, attr="goal")
  ++prop(entity=$me, attr="goal", value=$goal)
  => "Goal saved: $goal. I'll keep it in mind."

rule me-goal-ask
when: "what is my goal", speaker(entity=$me), prop(entity=$me, attr="goal", value=$goal)
trust: 2
then: => "Your goal is: $goal."

rule me-goal-ask-unknown
when: "what is my goal", speaker(entity=$me), !prop(entity=$me, attr="goal")
trust: 2
then: => "You haven't set a goal yet. Say \"my goal is ...\"."

rule me-working-on
when: "i am working on $project", speaker(entity=$me)
trust: 2
then:
  ++rel(subj=$me, pred="works_on", obj=$project)
  => "Noted - you're working on $project."

rule me-projects-ask
when: "what am i working on", speaker(entity=$me), !collected(tag="my_projects"), !collected_empty(tag="my_projects")
trust: 2
then: @collect(rel="rel", field="obj", tag="my_projects", subj=$me, pred="works_on")

rule me-projects-list
when: "list my projects", speaker(entity=$me), !collected(tag="my_projects"), !collected_empty(tag="my_projects")
trust: 2
then: @collect(rel="rel", field="obj", tag="my_projects", subj=$me, pred="works_on")

rule me-projects-reply
when: collected(tag="my_projects", list=$l)
trust: 4
then: => "You're working on: $l."

rule me-projects-reply-empty
when: collected_empty(tag="my_projects")
trust: 4
then: => "No projects yet. Say \"I am working on ...\"."
```

The weather skill looks up the city of whoever is speaking, or asks for one.
Both drop the "city missing" request, so the core rules that read the fixed
id `user` don't answer for someone else afterwards.

```karin
rule me-weather-my-city
when: intent(name="weather_missing_city"), speaker(entity=$me), prop(entity=$me, attr="city", value=$city), !intent(name="weather")
trust: 2
then:
  -intent(name="weather_missing_city")
  +intent(name="weather", city=$city)

rule me-weather-ask-city
when: intent(name="weather_missing_city"), speaker(entity=$me), !prop(entity=$me, attr="city"), !need_slot(goal="weather", slot="city"), !intent(name="weather")
trust: 2
then:
  -intent(name="weather_missing_city")
  +need_slot(goal="weather", slot="city", prompt="Which city would you like the weather for? (I know Paris, Tokyo, London, and Vilnius.)")
```

## Corrections ("what about X?" -> "yes")

The core confirms these for `user`; these versions add the like to whoever is
speaking.

```karin
rule me-confirm-isa-yes
when: "yes", dialogue_confirm(kind="confirm_isa", item=$item, topic=$topic), speaker(entity=$me)
then:
  -dialogue_confirm
  ++is_a(child=$item, parent=$topic)
  ++rel(subj=$me, pred="likes", topic=$topic, obj=$item)
  => "Got it - learned that $item is a $topic and added it to your favorite $topic."

rule me-confirm-like-yes
when: "yes", dialogue_confirm(kind="confirm_like", item=$item, topic=$topic), speaker(entity=$me)
then:
  -dialogue_confirm
  ++rel(subj=$me, pred="likes", obj=$item)
  ++rel(subj=$me, pred="likes", topic=$topic, obj=$item)
  => "Got it - added $item to your favorite $topic."

rule me-confirm-both-yes
when: "yes", dialogue_confirm(kind="confirm_both", item=$item, topic=$topic), speaker(entity=$me)
then:
  -dialogue_confirm
  ++is_a(child=$item, parent=$topic)
  ++rel(subj=$me, pred="likes", obj=$item)
  ++rel(subj=$me, pred="likes", topic=$topic, obj=$item)
  => "Got it - learned that $item is a $topic and remembered it as a $topic you like."
```

## "What do you know about me"

```karin
rule me-about
when: "what do you know about me", speaker(entity=$me), !about()
trust: 3
then:
  +about(entity=$me)
  @collect(rel="rel", field="obj", tag="about", subj=$me, pred="likes")

rule me-about-reply-likes
when: collected(tag="about", list=$l), about(entity=$e), speaker(entity=$e), display_name(entity=$e, value=$n)
trust: 2
then: => "You're $n, and you like $l."

rule me-about-reply-name
when: collected_empty(tag="about"), about(entity=$e), speaker(entity=$e), display_name(entity=$e, value=$n)
trust: 2
then: => "You're $n. I don't know what you like yet - tell me!"

rule me-about-reply-likes-unknown
when: collected(tag="about", list=$l), about(entity=$e), speaker(entity=$e), !display_name(entity=$e)
trust: 2
then:
  +chat dialogue_slot(goal="identity", slot="name")
  => "I know you like $l, but not your name yet - who am I talking to?"

rule me-about-reply-nothing
when: collected_empty(tag="about"), about(entity=$e), speaker(entity=$e), !display_name(entity=$e)
trust: 2
then:
  +chat dialogue_slot(goal="identity", slot="name")
  => "I don't know anything about you yet - what's your name?"
```

## Older memory

Memory saved before entities stored the name as `user_name` and people as
plain names in `rel(subj="user", ...)`. These run once, when such facts are
found, and turn them into entities.

```karin
rule me-adopt-legacy-name
when: user_name(value=$n), !display_name(entity="user")
trust: 5
then:
  ++display_name(entity="user", value=$n)
  ++named(entity="user", value=$n)

rule me-adopt-legacy-person
when: rel(subj="user", pred=$p, obj=$x), $p != "likes", !named(value=$x)
trust: 5
then:
  ++entity(id=$x, kind="person")
  ++display_name(entity=$x, value=$x)
  ++named(entity=$x, value=$x)
```

