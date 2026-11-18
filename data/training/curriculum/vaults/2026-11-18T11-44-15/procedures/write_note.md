---
id: write_note
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# write note

Learned in conversation.

```datalog
task(write_note). @conf 1 @source ""
needs(write_note, card). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
needs(write_note, pen). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
needs(write_note, envelope). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
needs(write_note, stamp). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
takes_time(write_note, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
step(write_note, write_note_s1). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
action(write_note_s1, write). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
object_of(write_note_s1, greeting). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
uses(write_note_s1, card). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
uses(write_note_s1, pen). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
step(write_note, write_note_s2). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
action(write_note_s2, thank). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
object_of(write_note_s2, gift). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
before(write_note_s1, write_note_s2). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
step(write_note, write_note_s3). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
action(write_note_s3, describe). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
object_of(write_note_s3, gift). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
before(write_note_s2, write_note_s3). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
step(write_note, write_note_s4). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
action(write_note_s4, sign). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
object_of(write_note_s4, card). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
uses(write_note_s4, pen). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
before(write_note_s3, write_note_s4). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
step(write_note, write_note_s5). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
action(write_note_s5, seal). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
object_of(write_note_s5, envelope). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
step(write_note, write_note_s6). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
action(write_note_s6, stick). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
object_of(write_note_s6, stamp). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
uses(write_note_s6, envelope). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
before(write_note_s5, write_note_s6). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
before(write_note_s4, write_note_s5). @conf 1 @source teacher:curriculum @conv s1795002270399-760026-77
```

## Taught wording

- task: write a thank-you note
- write_note_s1: write a greeting with the person's name on the card with the pen
- write_note_s2: thank them for the gift by name
- write_note_s3: describe how you will use or enjoy the gift
- write_note_s4: sign the card with the pen
- write_note_s5: seal the envelope and stick the stamp on it
