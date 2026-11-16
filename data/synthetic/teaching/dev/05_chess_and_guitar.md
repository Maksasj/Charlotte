# Dialogue: Chess and guitar
domain: hobbies

## script
- I play chess.
- I play guitar.
- I enjoy hiking.
- Do I play chess? => yes
- Do I like hiking? => yes
- blah blah
- I don't play chess anymore.
- Do I play chess? => no
- Did I play chess? => contains: used to
- Do I play guitar? => yes
- Do I play tennis? => unknown
- What do I play? => contains: guitar
- If someone likes hiking then they have boots.
- Do I have boots? => yes
- Why? => contains: hiking

## expect facts
- -play(maksim, chess)
- play(maksim, guitar)
- likes(maksim, hiking)
- has(maksim, boot)

## expect rules
- has(A, boot) :- likes(A, hiking).

## ground truth
- is_a(maksim, person)
- -play(maksim, chess)
- play(maksim, guitar)
- likes(maksim, hiking)
- has(maksim, boot)
