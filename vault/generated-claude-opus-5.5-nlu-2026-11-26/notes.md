# Notes (`claude-opus-5.5`, 2026-11-26)

The lexicon turns "take / create / save / add / write a note (for me)" and
"to-do" into "make a note", and "list all my notes" / "show me my notes" into
"list my notes". Here: asking for the text when it's missing, and a glossary
entry so "what is a note" has an answer.

```karin
definition(topic="note", text="a short piece of text I keep in long-term memory for you. Say \"make a note <text>\" to add one and \"list my notes\" to hear them.")

rule note-bare
when: "make a note", !need_slot(goal="note", slot="text")
then: +need_slot(goal="note", slot="text", prompt="Sure - what should the note say?")

rule note-from-slot
when: slot_value(goal="note", slot="text", value=$text), !intent(name="save_note")
then: +intent(name="save_note", text=$text)

rule note-make-colon
when: "make a note to $text", !intent(name="save_note")
then: +intent(name="save_note", text=$text)

rule note-note-that
when: "note that $text", !intent(name="save_note")
then: +intent(name="save_note", text=$text)
```
