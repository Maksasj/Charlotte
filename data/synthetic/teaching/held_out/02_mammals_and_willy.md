# Dialogue: Mammals and Willy
domain: animals

## script
- Bessie is a mammal.
- Bessie is a cow.
- Willy is a mammal.
- Willy is a whale.
- Every mammal walks.
- Does Bessie walk? => yes
- Why? => contains: mammal
- Does Willy walk? => yes
- No, Willy doesn't walk.
- Does Willy walk? => no
- Does Bessie walk? => yes
- whatever, you know what I mean
- Willy is big.
- Is Willy big? => yes
- Every whale swims.
- Does Willy swim? => yes
- Does Bessie eat grass? => unknown

## expect facts
- is_a(bessie, mammal)
- is_a(bessie, cow)
- is_a(willy, whale)
- walk(bessie)
- property(willy, size, big)
- swim(willy)

## expect rules
- swim(A) :- is_a(A, whale).

## ground truth
- is_a(bessie, mammal)
- is_a(bessie, cow)
- is_a(willy, mammal)
- is_a(willy, whale)
- walk(bessie)
- -walk(willy)
- property(willy, size, big)
- swim(willy)
