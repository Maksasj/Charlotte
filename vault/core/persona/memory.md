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
  -prop(entity="user", attr="favorite")
  ++prop(entity="user", attr="favorite", value=$thing)
  => "Noted — I'll remember you like $thing."

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
