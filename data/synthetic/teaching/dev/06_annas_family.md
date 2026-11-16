# Dialogue: Anna's family
domain: family

## script
- Hi.
- Anna is a mother.
- Anna has a son.
- Anna has a daughter.
- Anna's daughter is Lisa.
- Anna loves Tom.
- If someone is a mother then they are a parent.
- Is Anna a parent? => yes
- Why? => contains: mother
- What does Anna have? => contains: son; daughter
- Does Anna love Tom? => yes
- family stuff, whatever
- What is Anna's daughter? => contains: lisa
- Tom lives in Berlin.
- Where does Tom live? => contains: berlin
- Does Tom have a sister? => unknown

## expect facts
- is_a(anna, mother)
- has(anna, son)
- has(anna, daughter)
- property(anna, daughter, lisa)
- likes(anna, tom)
- is_a(anna, parent)
- located_in(tom, berlin)

## expect rules
- is_a(A, parent) :- is_a(A, mother).

## ground truth
- is_a(anna, mother)
- is_a(anna, parent)
- is_a(anna, person)
- has(anna, son)
- has(anna, daughter)
- property(anna, daughter, lisa)
- likes(anna, tom)
- located_in(tom, berlin)
