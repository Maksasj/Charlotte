# Notes

## p23: save-note
- status: active
- kind: language
- trust: 0.9
- support: 4
- provenance: seed

### when
- text: remember ?text
- not_fact: note_saved()

### then
- tool: note_save(text=?text)

## p24: reply-note-saved
- status: active
- kind: reply
- trust: 0.9
- support: 4
- provenance: seed

### when
- fact: note_saved(text=?text)

### then
- reply: Saved to long-term memory: "?text".

## p25: list-notes
- status: active
- kind: language
- trust: 0.9
- support: 4
- provenance: seed

### when
- text: my notes
- not_fact: notes_listing()

### then
- tool: note_list()

## p26: reply-notes-listing
- status: active
- kind: reply
- trust: 0.9
- support: 4
- provenance: seed

### when
- fact: notes_listing(list=?list)

### then
- reply: Your notes: ?list
