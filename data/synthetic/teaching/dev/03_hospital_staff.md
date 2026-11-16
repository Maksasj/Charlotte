# Dialogue: Hospital staff
domain: work

## script
- Anna is a doctor.
- Bob is a teacher.
- Anna works at Mercy.
- If someone is a doctor, then they are a person.
- Is Anna a person? => yes
- Why? => contains: because; doctor
- Does Bob work at Mercy? => unknown
- If someone is a teacher then they are a person.
- Is Bob a person? => yes
- hmm
- What do you know about Anna? => contains: doctor; mercy
- Anna knows Bob.
- Does Anna know Bob? => yes
- Is Anna a doctor? => yes

## expect facts
- is_a(anna, doctor)
- is_a(bob, teacher)
- work_at(anna, mercy)
- is_a(anna, person)
- is_a(bob, person)
- knows(anna, bob)

## expect rules
- is_a(A, person) :- is_a(A, doctor).
- is_a(A, person) :- is_a(A, teacher).

## ground truth
- is_a(anna, doctor)
- is_a(bob, teacher)
- is_a(anna, person)
- is_a(bob, person)
- work_at(anna, mercy)
- knows(anna, bob)
