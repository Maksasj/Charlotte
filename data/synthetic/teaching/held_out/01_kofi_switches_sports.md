# Dialogue: Kofi switches sports
domain: hobbies

## script
- Good morning.
- Kofi plays football.
- Kofi likes running.
- Kofi is a striker.
- Does Kofi play football? => yes
- Why? => contains: because
- hmm
- Kofi doesn't play football anymore.
- Does Kofi play football? => no
- Did Kofi play football? => contains: used to
- Kofi plays cricket.
- What does Kofi play? => contains: cricket
- If someone plays cricket then they have a bat.
- Does Kofi have a bat? => yes
- Does Kofi like swimming? => unknown
- Is Kofi a striker? => yes

## expect facts
- -play(kofi, football)
- play(kofi, cricket)
- likes(kofi, running)
- is_a(kofi, striker)
- has(kofi, bat)

## expect rules
- has(A, bat) :- play(A, cricket).

## ground truth
- is_a(kofi, person)
- is_a(kofi, striker)
- -play(kofi, football)
- play(kofi, cricket)
- likes(kofi, running)
- has(kofi, bat)
