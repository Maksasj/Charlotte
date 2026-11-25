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
