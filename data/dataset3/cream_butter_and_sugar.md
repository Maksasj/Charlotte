---
id: cream_butter_and_sugar
stage: procedures
task: cream_butter_and_sugar
schema: v3
---
# Cream butter and sugar

## requests
- I'm making a cake and need the butter and sugar creamed first. How do I do that?
- What's the method for creaming butter and sugar until it's light and fluffy?
- Could you take me through beating butter and sugar together for a sponge?

## situation
- I have butter, sugar, a measuring cup, a mixing bowl, an electric mixer and a spatula.
- I have about 45 minutes before I need to start the cake.

## ground truth
```datalog
task(cream_butter_and_sugar).
domain(cream_butter_and_sugar, cooking).
goal(cream_butter_and_sugar, creamed_mixture).
needs(cream_butter_and_sugar, butter).
needs(cream_butter_and_sugar, sugar).
needs(cream_butter_and_sugar, measuring_cup).
needs(cream_butter_and_sugar, mixing_bowl).
needs(cream_butter_and_sugar, electric_mixer).
needs(cream_butter_and_sugar, spatula).
takes_time(cream_butter_and_sugar, 35, 45, minute).
step(cream_butter_and_sugar, cream_butter_and_sugar_s1).
action(cream_butter_and_sugar_s1, soften).
object_of(cream_butter_and_sugar_s1, butter).
takes_time(cream_butter_and_sugar_s1, 30, 40, minute).
purpose(cream_butter_and_sugar_s1, easy_to_beat).
step(cream_butter_and_sugar, cream_butter_and_sugar_s2).
action(cream_butter_and_sugar_s2, measure).
object_of(cream_butter_and_sugar_s2, sugar).
uses(cream_butter_and_sugar_s2, measuring_cup).
role(cream_butter_and_sugar_s2, measuring_cup, measure).
amount(cream_butter_and_sugar_s2, sugar, 100, gram).
overlaps(cream_butter_and_sugar_s2, cream_butter_and_sugar_s1).
step(cream_butter_and_sugar, cream_butter_and_sugar_s3).
action(cream_butter_and_sugar_s3, pour).
object_of(cream_butter_and_sugar_s3, sugar).
into(cream_butter_and_sugar_s3, mixing_bowl).
uses(cream_butter_and_sugar_s3, mixing_bowl).
role(cream_butter_and_sugar_s3, mixing_bowl, contain).
step(cream_butter_and_sugar, cream_butter_and_sugar_s4).
action(cream_butter_and_sugar_s4, beat).
object_of(cream_butter_and_sugar_s4, butter).
uses(cream_butter_and_sugar_s4, electric_mixer).
role(cream_butter_and_sugar_s4, electric_mixer, beat).
uses(cream_butter_and_sugar_s4, mixing_bowl).
role(cream_butter_and_sugar_s4, mixing_bowl, contain).
setting(cream_butter_and_sugar_s4, speed, medium, level).
until(cream_butter_and_sugar_s4, pale_and_fluffy).
takes_time(cream_butter_and_sugar_s4, 3, 5, minute).
step(cream_butter_and_sugar, cream_butter_and_sugar_s5).
action(cream_butter_and_sugar_s5, scrape).
object_of(cream_butter_and_sugar_s5, mixing_bowl).
uses(cream_butter_and_sugar_s5, spatula).
role(cream_butter_and_sugar_s5, spatula, scrape).
before(cream_butter_and_sugar_s1, cream_butter_and_sugar_s4).
before(cream_butter_and_sugar_s2, cream_butter_and_sugar_s3).
before(cream_butter_and_sugar_s3, cream_butter_and_sugar_s4).
before(cream_butter_and_sugar_s4, cream_butter_and_sugar_s5).
```

## aliases
```datalog
task_alias(cream_butter_and_sugar, "creaming butter and sugar").
task_alias(cream_butter_and_sugar, "creamed butter and sugar").
alias(butter, "softened butter").
alias(electric_mixer, "hand mixer").
alias(measuring_cup, "measuring jug").
alias(spatula, "rubber spatula").
alias(pale_and_fluffy, "light and fluffy").
alias(creamed_mixture, "creamed butter and sugar").
```

## teach
- To cream butter and sugar you need butter, sugar, a measuring cup, a mixing bowl, an electric mixer and a spatula.
- Creaming butter and sugar takes 35 to 45 minutes.
- Step 1 of cream butter and sugar: leave the butter out at room temperature to soften for 30 to 40 minutes so it is easy to beat.
- While step 1 runs, step 2 of cream butter and sugar: measure 100 grams of sugar with the measuring cup.
- Step 3 of cream butter and sugar: pour the sugar into the mixing bowl with the softened butter.
- Step 4 of cream butter and sugar: beat the butter and sugar with the electric mixer on medium speed for 3 to 5 minutes until pale and fluffy.
- Step 5 of cream butter and sugar: scrape down the sides of the mixing bowl with a spatula.

## checklist
- names what is needed, including butter, sugar, a measuring cup, a mixing bowl, an electric mixer and a spatula
- gives the steps in order: soften butter, measure sugar, pour sugar into bowl, beat on medium speed until pale and fluffy, scrape the bowl
- says the softening and measuring can be done at the same time
- gives a total time of about 35 to 45 minutes

## variant
- change: no electric mixer: beat the butter and sugar by hand with a wooden spoon for 8 to 10 minutes, and the total time becomes about 40 to 50 minutes
- request: I don't own an electric mixer. Can I still cream the butter and sugar by hand?
- situation: I have butter, sugar, a measuring cup, a mixing bowl, a spatula and a wooden spoon, but no electric mixer.
- expect: The reply beats the butter and sugar by hand with the wooden spoon for about 8 to 10 minutes until pale and fluffy, instead of using the mixer.

```datalog situation
lacks(user, electric_mixer).
has(user, wooden_spoon).
```

```datalog difference
no_need(electric_mixer).
extra_need(wooden_spoon).
swap(cream_butter_and_sugar_s4, electric_mixer, wooden_spoon).
change(cream_butter_and_sugar_s4, takes_time, 3_5, 8_10).
change(cream_butter_and_sugar, takes_time, 35_45, 40_50).
```

- change: add one beaten egg after the butter and sugar are pale and fluffy, beating it in before the bowl is scraped
- request: I want to add an egg to the creamed butter and sugar. Where does it go?
- situation: I have butter, sugar, a measuring cup, a mixing bowl, an electric mixer, a spatula and an egg.
- expect: The reply adds the egg after the butter and sugar are pale and fluffy, beats it in on the electric mixer for about a minute, and then scrapes the bowl.

```datalog situation
has(user, egg).
```

```datalog difference
add_step(cream_butter_and_sugar_s9, cream_butter_and_sugar_s4).
step(cream_butter_and_sugar, cream_butter_and_sugar_s9).
action(cream_butter_and_sugar_s9, beat).
object_of(cream_butter_and_sugar_s9, egg).
uses(cream_butter_and_sugar_s9, electric_mixer).
role(cream_butter_and_sugar_s9, electric_mixer, beat).
takes_time(cream_butter_and_sugar_s9, 1, 2, minute).
before(cream_butter_and_sugar_s4, cream_butter_and_sugar_s9).
before(cream_butter_and_sugar_s9, cream_butter_and_sugar_s5).
extra_need(egg).
```
