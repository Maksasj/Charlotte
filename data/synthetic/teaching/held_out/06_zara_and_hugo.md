# Dialogue: Zara and Hugo
domain: people

## script
- Hello.
- Zara lives in Lisbon.
- Zara knows Hugo.
- Zara paints.
- Hugo is a pilot.
- Hugo's hobby is sailing.
- What do you know about Zara? => contains: lisbon
- What do you know about Hugo? => contains: pilot
- What is Hugo's hobby? => contains: sailing
- Does Zara paint? => yes
- Why? => contains: because
- Does Hugo paint? => unknown
- anyway, never mind
- If someone is a pilot then they fly.
- Does Hugo fly? => yes
- Hugo doesn't like Lisbon.
- Does Hugo like Lisbon? => no

## expect facts
- located_in(zara, lisbon)
- knows(zara, hugo)
- paint(zara)
- is_a(hugo, pilot)
- property(hugo, hobby, sailing)
- fly(hugo)
- -likes(hugo, lisbon)

## expect rules
- fly(A) :- is_a(A, pilot).

## ground truth
- is_a(zara, person)
- is_a(hugo, person)
- located_in(zara, lisbon)
- knows(zara, hugo)
- paint(zara)
- is_a(hugo, pilot)
- property(hugo, hobby, sailing)
- fly(hugo)
- -likes(hugo, lisbon)
