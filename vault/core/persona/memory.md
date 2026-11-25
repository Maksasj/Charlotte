# User Memory

Stores and recalls the user's name in long-term memory (`++user_name`), replacing any previous value (`-user_name`).

```karin
rule learn-user-name
when: "my name is $name"
then:
  -user_name
  ++user_name(value=$name)
  => "Nice to meet you, $name! I've stored your name in long-term memory."

rule recall-user-name
when: "what is my name", user_name(value=$name)
then: => "Your name is $name."

rule recall-user-name-unknown
when: "what is my name", !user_name()
then: => "You haven't told me your name yet. Say \"my name is ...\" and I'll remember it."

rule learn-user-city
when: "I live in $city"
then:
  -prop(entity="user", attr="city")
  ++prop(entity="user", attr="city", value=$city)
  => "Got it — I'll remember you live in $city."

rule learn-user-work
when: "I work at $org"
then:
  -prop(entity="user", attr="workplace")
  ++prop(entity="user", attr="workplace", value=$org)
  => "Noted — I'll remember you work at $org."

rule learn-user-like
when: "I like $thing"
then:
  ++rel(subj="user", pred="likes", obj=$thing)
  => "Noted — I'll remember you like $thing."

rule learn-user-like-topic-a
when: "my favorite $topic is a $thing"
then:
  -dialogue_topic
  +dialogue_topic(kind="likes", topic=$topic)
  ++rel(subj="user", pred="likes", topic=$topic, obj=$thing)
  => "Got it — I'll remember $thing as a $topic you like."

rule learn-user-like-topic
when: "my favorite $topic is $thing"
then:
  -dialogue_topic
  +dialogue_topic(kind="likes", topic=$topic)
  ++rel(subj="user", pred="likes", topic=$topic, obj=$thing)
  => "Got it — I'll remember $thing as a $topic you like."

rule learn-user-also-like-topic
when: "I also like $thing", dialogue_topic(kind="likes", topic=$topic)
then:
  ++rel(subj="user", pred="likes", topic=$topic, obj=$thing)
  => "Got it — added $thing to the $topic you like."

rule learn-user-also-like-general
when: "I also like $thing", !dialogue_topic(kind="likes")
then:
  ++rel(subj="user", pred="likes", obj=$thing)
  => "Got it — added $thing to what you like."

rule ask-user-likes-all
when: "what do I like", !collected(tag="likes"), !collected_empty(tag="likes")
then:
  -dialogue_topic
  @collect(rel="rel", field="obj", tag="likes", subj="user", pred="likes")

rule reply-user-likes-all
when: collected(tag="likes", list=$list)
then: => "You like $list."

rule reply-user-likes-all-empty
when: collected_empty(tag="likes")
then: => "I don't know what you like yet. Tell me \"I like ...\" and I'll remember."

rule ask-user-likes-topic-do
when: "what $topic do I like", !intent(name="likes_topic")
then: +intent(name="likes_topic", topic=$topic)

rule ask-user-likes-topic-short
when: "what $topic I like", !intent(name="likes_topic")
then: +intent(name="likes_topic", topic=$topic)

rule ask-user-likes-topic-favorite
when: "what is my favorite $topic", !intent(name="likes_topic")
then: +intent(name="likes_topic", topic=$topic)

rule run-likes-topic-collect
when: intent(name="likes_topic", topic=$topic), !collected(tag="likes_topic"), !collected_empty(tag="likes_topic")
then:
  -dialogue_topic
  +dialogue_topic(kind="likes", topic=$topic)
  @collect(rel="rel", field="obj", tag="likes_topic", subj="user", pred="likes", topic=$topic)

rule reply-user-likes-topic
when: collected(tag="likes_topic", list=$list), dialogue_topic(kind="likes", topic=$topic)
then: => "Your favorite $topic: $list."

rule reply-user-likes-topic-empty
when: collected_empty(tag="likes_topic"), dialogue_topic(kind="likes", topic=$topic)
then: => "I don't know what $topic you like yet. Say \"my favorite $topic is ...\" to tell me."

rule recall-user-profile-full
when: "what do you know about me", user_name(value=$name), prop(entity="user", attr="city", value=$city)
then: => "You're $name and you live in $city."

rule recall-user-profile-name
when: "what do you know about me", user_name(value=$name), !prop(entity="user", attr="city")
then: => "I know your name is $name. Tell me \"I live in ...\" or \"I like ...\" to teach me more!"

rule recall-user-profile-city
when: "what do you know about me", !user_name(), prop(entity="user", attr="city", value=$city)
then: => "I know you live in $city. Tell me \"my name is ...\" so I know your name too!"

rule recall-user-profile-empty
when: "what do you know about me", !user_name(), !prop(entity="user", attr="city")
then: => "I don't know much about you yet — try \"my name is ...\", \"I live in ...\", or \"I like ...\"."
```
