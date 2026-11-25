# Notes

## save-note
when: "remember ?text", !note_saved()
then: @note_save(text=?text)

## reply-note-saved
when: note_saved(text=?text)
then: => "Saved to long-term memory: \"?text\"."

## list-notes
when: "my notes", !notes_listing()
then: @note_list()

## reply-notes-listing
when: notes_listing(list=?list)
then: => "Your notes: ?list"
