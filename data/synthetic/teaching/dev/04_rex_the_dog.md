# Dialogue: Rex the dog
domain: animals

## script
- I have a dog.
- Rex is a dog.
- Every dog likes bones.
- Does Rex like bones? => yes
- Why? => contains: because
- Rex is brown.
- Is Rex brown? => yes
- What colour is Rex? => contains: brown
- ugh, long day
- Rex is hungry.
- Is Rex hungry? => yes
- Is Rex scared? => unknown
- Do I have a dog? => yes

## expect facts
- has(maksim, dog)
- is_a(rex, dog)
- likes(rex, bone)
- property(rex, colour, brown)
- feels(rex, hungry)

## expect rules
- likes(A, bone) :- is_a(A, dog).

## ground truth
- is_a(maksim, person)
- has(maksim, dog)
- is_a(rex, dog)
- likes(rex, bone)
- property(rex, colour, brown)
- feels(rex, hungry)
