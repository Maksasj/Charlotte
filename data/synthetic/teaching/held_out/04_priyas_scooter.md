# Dialogue: Priya's scooter
domain: vehicles

## script
- Priya owns a scooter.
- The scooter is green.
- Priya drives a van.
- Does Priya have a scooter? => yes
- What colour is the scooter? => contains: green
- ugh, traffic
- Priya doesn't drive a van anymore.
- Does Priya drive a van? => no
- Did Priya drive a van? => contains: used to
- If someone has a scooter then they need a helmet.
- Does Priya need a helmet? => yes
- Why? => contains: scooter
- Does Priya have a boat? => unknown
- What does Priya have? => contains: scooter

## expect facts
- has(priya, scooter)
- property(scooter, colour, green)
- -drive(priya, van)
- wants(priya, helmet)

## expect rules
- wants(A, helmet) :- has(A, scooter).

## ground truth
- is_a(priya, person)
- has(priya, scooter)
- property(scooter, colour, green)
- -drive(priya, van)
- wants(priya, helmet)
