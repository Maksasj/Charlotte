# Dialogue: Fruit and vegetables
domain: food

## script
- Apples are fruits.
- Carrots are vegetables.
- Anna likes apples.
- Anna hates carrots.
- Bob likes ice cream.
- Is an apple a fruit? => yes
- Does Anna like carrots? => no
- Does Anna like apples? => yes
- Who likes ice cream? => contains: bob
- kind of, sort of
- If someone likes apples then they want fruit.
- Does Anna want fruit? => yes
- Why? => contains: because
- Does Bob like carrots? => unknown
- What do you know about Anna? => contains: apple

## expect facts
- is_a(apple, fruit)
- is_a(carrot, vegetable)
- likes(anna, apple)
- -likes(anna, carrot)
- likes(bob, ice_cream)
- wants(anna, fruit)

## expect rules
- wants(A, fruit) :- likes(A, apple).

## ground truth
- is_a(apple, fruit)
- is_a(carrot, vegetable)
- likes(anna, apple)
- -likes(anna, carrot)
- likes(bob, ice_cream)
- wants(anna, fruit)
