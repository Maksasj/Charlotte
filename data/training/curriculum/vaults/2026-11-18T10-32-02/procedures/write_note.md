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
needs(write_note, card). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
needs(write_note, pen). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
needs(write_note, envelope). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
needs(write_note, stamp). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
takes_time(write_note, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
step(write_note, write_note_s1). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
action(write_note_s1, write). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
object_of(write_note_s1, greeting). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
uses(write_note_s1, card). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
uses(write_note_s1, pen). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
step(write_note, write_note_s2). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
action(write_note_s2, thank). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
object_of(write_note_s2, gift). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
before(write_note_s1, write_note_s2). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
step(write_note, write_note_s3). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
action(write_note_s3, describe). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
object_of(write_note_s3, gift). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
before(write_note_s2, write_note_s3). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
step(write_note, write_note_s4). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
action(write_note_s4, sign). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
object_of(write_note_s4, card). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
uses(write_note_s4, pen). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
before(write_note_s3, write_note_s4). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
step(write_note, write_note_s5). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
action(write_note_s5, seal). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
object_of(write_note_s5, envelope). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
step(write_note, write_note_s6). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
action(write_note_s6, stick). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
object_of(write_note_s6, stamp). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
uses(write_note_s6, envelope). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
before(write_note_s5, write_note_s6). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
before(write_note_s4, write_note_s5). @conf 1 @source teacher:curriculum @conv s1794997930961-475800-42
```
