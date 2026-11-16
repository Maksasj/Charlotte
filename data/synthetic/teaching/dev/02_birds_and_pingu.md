# Dialogue: Birds and Pingu
domain: animals

## script
- Tweety is a bird.
- Polly is a bird.
- Pingu is a bird.
- Pingu is a penguin.
- If something is a bird then it flies.
- Does Tweety fly? => yes
- Why? => contains: because
- Does Pingu fly? => yes
- No, Pingu doesn't fly.
- Does Pingu fly? => no
- Does Tweety fly? => yes
- Does Polly fly? => yes
- whatever, you know what I mean
- If something is a penguin then it swims.
- Does Pingu swim? => yes
- Is Pingu a penguin? => yes
- Does Tweety like fish? => unknown

## expect facts
- is_a(tweety, bird)
- is_a(polly, bird)
- is_a(pingu, penguin)
- fly(tweety)
- fly(polly)
- swim(pingu)

## expect rules
- swim(A) :- is_a(A, penguin).

## ground truth
- is_a(tweety, bird)
- is_a(polly, bird)
- is_a(pingu, bird)
- is_a(pingu, penguin)
- fly(tweety)
- fly(polly)
- -fly(pingu)
- swim(pingu)
