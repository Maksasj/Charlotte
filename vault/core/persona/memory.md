# User Memory

## learn-user-name
when: "my name is ?name"
then:
  -user_name
  ++user_name(value=?name)
  => "Nice to meet you, ?name! I've stored your name in long-term memory."

## recall-user-name
when: "what is my name", user_name(value=?name)
then: => "Your name is ?name."

## recall-user-name-unknown
when: "what is my name", !user_name()
then: => "You haven't told me your name yet. Say \"my name is ...\" and I'll remember it."
