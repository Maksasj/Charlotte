# Dialogue: Maksim's drinks
domain: food

## script
- Hello.
- I like tea.
- I drink coffee.
- Do I like tea? => yes
- Do I drink coffee? => yes
- Why? => contains: because
- Do I like juice? => unknown
- hmm
- I don't like tea anymore.
- Do I like tea? => no
- Did I like tea? => contains: used to
- I like cocoa.
- What do I like? => contains: cocoa

## expect facts
- -likes(maksim, tea)
- drink(maksim, coffee)
- likes(maksim, cocoa)

## expect rules

## ground truth
- is_a(maksim, person)
- -likes(maksim, tea)
- drink(maksim, coffee)
- likes(maksim, cocoa)
