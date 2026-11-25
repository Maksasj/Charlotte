# Social & Persona

## greet-known-user
when: "hello", user_name(value=?u), persona_name(value=?me)
then: => "Hello ?u! I'm ?me. What shall we work on?"

## greet-default
when: "hello", !user_name(), persona_name(value=?me)
then: => "Hello! I'm ?me. Ask me for weather, math, definitions, notes, or teach me something new."

## greet-hi
when: "hi", persona_name(value=?me)
then: => "Hi! I'm ?me — ready when you are."

## thanks
when: ~"thank"
then: => "You're welcome! Let me know if there's anything else."

## goodbye
when: "bye"
then: => "Goodbye! All long-term facts and patterns stay saved."

## who-are-you
when: "who are you", persona_name(value=?me), persona_role(value=?role), persona_tone(value=?tone)
then: => "I'm ?me, ?role. My style is ?tone."
