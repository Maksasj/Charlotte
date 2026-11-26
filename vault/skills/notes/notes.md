# Notes

Saves and lists freeform long-term notes via `@note_save` and `@note_list`.

```karin
rule save-note
when: intent(name="save_note", text=$text), !note_saved()
then: @note_save(text=$text)

rule reply-note-saved
when: note_saved(text=$text)
then: => "Saved to long-term memory: \"$text\"."

rule list-notes
when: intent(name="list_notes"), !notes_listing()
then: @note_list()

rule reply-notes-listing
when: notes_listing(list=$list)
then: => "Your notes: $list"
```

## More ways to save notes

The lexicon turns "take / create / save / add / write a note (for me)" and
"to-do" into "make a note", and "list all my notes" / "show me my notes" into
"list my notes". Here: asking for the text when it's missing, and a glossary
entry so "what is a note" has an answer.

```karin
definition(topic="note", text="a short piece of text I keep in long-term memory for you. Say \"make a note <text>\" to add one and \"list my notes\" to hear them.")

rule note-bare
when: "make a note", !need_slot(goal="note", slot="text")
then: +need_slot(goal="note", slot="text", prompt="Sure - what should the note say?")

```
