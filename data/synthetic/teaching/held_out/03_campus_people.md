# Dialogue: Campus people
domain: people

## script
- Mei is a student.
- Omar is a professor.
- If someone is a student then they read books.
- Does Mei read books? => yes
- Why? => contains: student
- If someone is a professor then they are a teacher.
- Is Omar a teacher? => yes
- erm
- Omar knows Mei.
- Who knows Mei? => contains: omar
- Mei is nervous.
- Is Mei nervous? => yes
- Is Omar nervous? => unknown
- Mei studies at Kingsford.
- What do you know about Mei? => contains: student

## expect facts
- is_a(mei, student)
- is_a(omar, professor)
- read(mei, book)
- is_a(omar, teacher)
- knows(omar, mei)
- feels(mei, nervous)
- study_at(mei, kingsford)

## expect rules
- read(A, book) :- is_a(A, student).
- is_a(A, teacher) :- is_a(A, professor).

## ground truth
- is_a(mei, student)
- is_a(mei, person)
- is_a(omar, professor)
- is_a(omar, teacher)
- is_a(omar, person)
- read(mei, book)
- knows(omar, mei)
- feels(mei, nervous)
- study_at(mei, kingsford)
