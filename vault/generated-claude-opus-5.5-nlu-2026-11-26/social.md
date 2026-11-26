# Social and chat control (`claude-opus-5.5`, 2026-11-26)

Thanks, apologies, "stop", repeating, and honest answers for things Karin can't
do yet (instead of an impasse).

```karin
rule social-thanks
when: ~"thanks"
then: => "You're welcome! Let me know if there's anything else."

rule social-sorry
when: ~"sorry"
then: => "No worries at all!"

rule social-good-night
when: "good night"
then: => "Good night! Everything you told me stays saved."

rule social-good-evening
when: "good evening", persona_name(value=$me)
then: => "Good evening! I'm $me - what shall we do?"

rule social-stop
when: "stop"
then: => "Okay, I'll stop."

rule social-stop-it
when: "stop it"
then: => "Okay, I'll stop."

rule social-stop-that
when: "stop that"
then: => "Okay, I'll stop."

rule social-stop-being
when: "stop being $x"
then: => "Sorry - I'll try to do better. Tell me what you meant and I'll learn it."

rule social-yes-nothing-pending
when: "yes", !dialogue_confirm(), !dialogue_pending_approval(), !dialogue_slot(), !approval_result(), !dialogue_pending_name()
trust: 0.2
then: => "Okay! What would you like to do?"

rule social-yeah
when: "yeah", !dialogue_confirm(), !dialogue_pending_approval(), !dialogue_slot(), !approval_result(), !dialogue_pending_name()
trust: 0.2
then: => "Okay! What would you like to do?"
```

## Remembering what was said

The engine records every utterance of the chat as `said(by, text)` (chat memory).

```karin
rule social-repeat
when: "repeat", said(by="karin", text=$t)
then: => "I said: \"$t\""

rule social-repeat-nothing
when: "repeat", !said(by="karin")
then: => "I haven't said anything yet in this chat."

rule social-what-did-i-say
when: "what did i just say", said(by="user", text=$t)
then: => "You said: \"$t\""
```

## Things Karin can't do yet

```karin
rule social-new-chat
when: "new chat"
then: => "I can't start a new chat myself yet - use the New chat button."

rule social-overlord
when: ~"overlord"
then: => "I can't send tasks to Overlord yet - there's no connection between us."

rule social-open-app
when: "open $app"
trust: 0.3
then: => "I can't open or control apps yet."

rule social-start-app
when: "start $app app"
then: => "I can't open or control apps yet."

rule social-how-do-i
when: "how do i $x"
trust: 0.2
then: => "I don't know how to $x yet - I don't have recipes or how-to guides. You can teach me."
```
