# Dialogue: Workshop tools
domain: tools

## script
- I have a hammer.
- I have a saw.
- Bob has a drill.
- The hammer is heavy.
- If someone has a saw then they cut wood.
- Do I cut wood? => yes
- Why? => contains: saw
- What do I have? => contains: hammer; saw
- Does Bob have a drill? => yes
- Does Bob have a hammer? => unknown
- Is the hammer heavy? => yes
- hmm
- I don't have a saw anymore.
- Do I have a saw? => no
- Did I have a saw? => contains: used to

## expect facts
- has(maksim, hammer)
- -has(maksim, saw)
- has(bob, drill)
- property(hammer, weight, heavy)

## expect rules
- cut(A, wood) :- has(A, saw).

## ground truth
- is_a(maksim, person)
- has(maksim, hammer)
- -has(maksim, saw)
- has(bob, drill)
- property(hammer, weight, heavy)
