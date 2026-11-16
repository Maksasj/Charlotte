# Dialogue: Cities and moves
domain: places

## script
- Anna lives in Paris.
- Bob lives in Rome.
- Paris is in France.
- Paris is a city.
- Rome is a city.
- Where does Anna live? => contains: paris
- Where is Paris? => contains: france
- Is Rome a city? => yes
- I live in Oslo.
- Where do I live? => contains: oslo
- and so on and so forth
- Bob doesn't live in Rome anymore.
- Does Bob live in Rome? => no
- Did Bob live in Rome? => contains: used to
- Bob lives in Milan.
- Where does Bob live? => contains: milan
- If something is a city then it is big.
- Is Rome big? => yes
- Why? => contains: city
- Where is Oslo? => unknown

## expect facts
- located_in(anna, paris)
- located_in(paris, france)
- is_a(paris, city)
- -located_in(bob, rome)
- located_in(bob, milan)
- located_in(maksim, oslo)
- property(rome, size, big)

## expect rules
- property(A, size, big) :- is_a(A, city).

## ground truth
- located_in(anna, paris)
- located_in(paris, france)
- is_a(paris, city)
- is_a(rome, city)
- -located_in(bob, rome)
- located_in(bob, milan)
- located_in(maksim, oslo)
- property(rome, size, big)
- property(paris, size, big)
