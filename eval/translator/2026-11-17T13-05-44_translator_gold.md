# Translator on `translator_gold`

Atom-level precision 0.888, recall 0.796, F1 **0.839**; act accuracy **86.5%**; sentences recovered 205/235 (87.2%); ungrounded constants 0; translation time median 2371 ms, 95th percentile 4945 ms; model calls 180.

## Items that differ from the gold

### g004 (acts differ)

> Hey, could you help me make pancakes for breakfast?

- missing `request`
- missing `request|make_pancake`

### g011 (acts differ)

> Hi there, can you help me iron a shirt?

- missing `request`
- missing `request|iron_shirt`

### g014 (acts differ)

> Hey, I need help organizing my closet.

- missing `greet`

### g019 (acts differ)

> Thanks a lot! Could you help me clean the bathroom next?

- missing `request`
- missing `request|clean_bathroom`
- extra `ask_next`

### g028 (acts differ)

> Hi! I need help preparing lunch for my kids.

- missing `request`
- missing `request|prepare_lunch`
- extra `inform`
- extra `inform|has(maksim, kid)`

### g048 

> How long does step 2 take?

- missing `ask_duration|make_tea_s2`
- extra `ask_duration|step_2`

### g049 

> How do I brew coffee?

- missing `ask_how|brew_coffee`
- extra `ask_how|make_coffee`

### g067 

> My faucet is leaking. Can you help me fix it?

- missing `inform|leak(my_faucet)`
- extra `inform|has(my_faucet, leak)`

### g068 

> I have a hammer, nails and some wood.

- missing `inform|available(hammer)`
- missing `inform|available(nail)`
- missing `inform|available(wood)`
- extra `inform|has(maksim, hammer)`
- extra `inform|has(maksim, nail)`
- extra `inform|has(maksim, wood)`

### g075 

> I'm at home and I have about 90 minutes.

- missing `inform|located_in(maksim, home)`
- not understood: I'm at home: not understood

### g076 

> I have a bike pump and a patch kit.

- missing `inform|available(bike_pump)`
- missing `inform|available(patch_kit)`
- extra `inform|has(my_bike, bike_pump)`
- extra `inform|has(my_bike, patch_kit)`

### g079 

> I have one pan and a small pot.

- extra `inform|has(maksim, pan)`

### g080 

> I'm in the kitchen and I have flour, yeast and water.

- missing `inform|located_in(maksim, kitchen)`

### g082 

> My oven is small, but I have a big frying pan.

- missing `inform|available(frying_pan)`
- missing `inform|property(my_oven, size, small)`
- extra `inform|has(maksim, frying_pan)`
- extra `inform|has(maksim, oven)`
- extra `inform|property(frying_pan, size, big)`
- extra `inform|property(oven, size, small)`

### g084 

> Hi! I have oats, bananas and milk.

- missing `inform|available(oat)`
- extra `inform|available(oats)`

### g107 

> Baking bread takes about an hour.

- missing `tell|takes_time(bake_bread, 1, 1, hour)`
- extra `tell|take_about(baking_bread, hour)`

### g110 (acts differ)

> To make pancakes you need flour, eggs and milk.

- missing `tell`
- missing `tell|needs(make_pancake, egg)`
- missing `tell|needs(make_pancake, flour)`
- missing `tell|needs(make_pancake, milk)`
- extra `ask_need`
- extra `ask_need|make_pancake`

### g111 

> You can use butter instead of oil.

- missing `tell|substitute(oil, butter)`
- extra `tell|substitute(butter, oil)`

### g112 

> Salt is in almost every home.

- missing `tell|staple(salt)`
- extra `tell|located_in(salt, almost_home)`

### g118 

> I'm vegetarian.

- missing `tell|is_a(maksim, vegetarian)`
- extra `tell|has(maksim, vegetarian)`

### g119 (acts differ)

> I used to live in Berlin.

- missing `tell`
- missing `tell|@past`
- missing `tell|located_in(maksim, berlin)`
- extra `inform`
- extra `inform|-located_in(maksim, berlin)`

### g120 

> Anna is my sister.

- missing `tell|property(maksim, sister, anna)`
- extra `tell|is_a(anna, sister)`

### g121 

> My mother lives in Madrid.

- missing `tell|located_in(my_mother, madrid)`
- extra `tell|is_a(mother, my_mother)`

### g128 (acts differ)

> Nina is 12.

- missing `tell`
- missing `tell|property(nina, age, 12)`
- extra `inform`
- extra `inform|property(nina, age, 12)`

### g135 (acts differ)

> Bob lived in Rome. Now he lives in Oslo.

- missing `tell`
- missing `tell|@past`
- missing `tell|located_in(bob, oslo)`
- missing `tell|located_in(bob, rome)`
- extra `inform`
- extra `inform|located_in(bob, oslo)`
- not understood: Bob lived in Rome.: lived_in(bob, rome): unknown predicate lived_in (it needs review before facts can use it)

### g136 

> First chop the onions, then fry them in butter for 5 minutes.

- missing `tell|action(make_soup_s1, chop)`
- missing `tell|action(make_soup_s2, fry)`
- missing `tell|before(make_soup_s1, make_soup_s2)`
- missing `tell|object_of(make_soup_s1, onion)`
- missing `tell|object_of(make_soup_s2, onion)`
- missing `tell|step(make_soup, make_soup_s1)`
- missing `tell|step(make_soup, make_soup_s2)`
- missing `tell|takes_time(make_soup_s2, 5, 5, minute)`
- missing `tell|uses(make_soup_s2, butter)`
- extra `tell|chop(first, onion)`
- not understood: then fry them in butter for 5 minutes: `onion` is not grounded: "onion" is not in the message or the conversation; step(make_soup, make_soup_s2) does not say what the sentence says; object_of(make_soup_s2, onion) does not say what the sentence says; takes_time(make_soup_s2, 5, 5, minute) does not say what the sentence says; before(make_soup_s1, make_soup_s2) does not say what the sentence says

### g137 

> Boil the water. Then steep the tea bag for 3 minutes.

- missing `tell|action(make_tea_s1, boil)`
- missing `tell|object_of(make_tea_s1, water)`
- missing `tell|step(make_tea, make_tea_s1)`
- extra `tell|water(boil)`

### g139 

> Drain the pasta in a colander.

- extra `tell|before(make_pasta_s2, make_pasta_s3)`

### g141 (acts differ)

> Cook the carrots until they are soft.

- missing `tell`
- missing `tell|action(make_soup_s1, cook)`
- missing `tell|object_of(make_soup_s1, carrot)`
- missing `tell|step(make_soup, make_soup_s1)`
- missing `tell|until(make_soup_s1, soft)`
- not understood: Cook the carrots until they are soft.: step(make_soup, make_soup_s1) does not say what the sentence says

### g142 

> Boil the pasta. Meanwhile, chop the garlic.

- extra `tell|during(make_pasta_s2, make_pasta_s1)`

### g143 (acts differ)

> First wipe the counters with a cloth, then mop the floor.

- missing `tell`
- missing `tell|action(clean_kitchen_s1, wipe)`
- missing `tell|action(clean_kitchen_s2, mop)`
- missing `tell|before(clean_kitchen_s1, clean_kitchen_s2)`
- missing `tell|object_of(clean_kitchen_s1, counter)`
- missing `tell|object_of(clean_kitchen_s2, floor)`
- missing `tell|step(clean_kitchen, clean_kitchen_s1)`
- missing `tell|step(clean_kitchen, clean_kitchen_s2)`
- missing `tell|uses(clean_kitchen_s1, cloth)`
- not understood: First wipe the counters with a cloth, then mop the floor.: step(clean_kitchen, clean_kitchen_s1) does not say what the sentence says; step(clean_kitchen, clean_kitchen_s2) does not say what the sentence says; before(clean_kitchen_s1, clean_kitchen_s2) does not say what the sentence says

### g144 (acts differ)

> Remove the wheel. After that, patch the tube.

- missing `tell`
- missing `tell|action(fix_tire_s1, remove)`
- missing `tell|action(fix_tire_s2, patch)`
- missing `tell|before(fix_tire_s1, fix_tire_s2)`
- missing `tell|object_of(fix_tire_s1, wheel)`
- missing `tell|object_of(fix_tire_s2, tube)`
- missing `tell|step(fix_tire, fix_tire_s1)`
- missing `tell|step(fix_tire, fix_tire_s2)`
- not understood: Remove the wheel.: step(fix_tire, fix_tire_s1) does not say what the sentence says
- not understood: After that, patch the tube.: step(fix_tire, fix_tire_s2) does not say what the sentence says; before(fix_tire_s1, fix_tire_s2) does not say what the sentence says

### g146 (acts differ)

> Whisk the eggs. At the same time, heat the pan.

- missing `tell`
- missing `tell|action(make_omelette_s1, whisk)`
- missing `tell|action(make_omelette_s2, heat)`
- missing `tell|object_of(make_omelette_s1, egg)`
- missing `tell|object_of(make_omelette_s2, pan)`
- missing `tell|overlaps(make_omelette_s2, make_omelette_s1)`
- missing `tell|step(make_omelette, make_omelette_s1)`
- missing `tell|step(make_omelette, make_omelette_s2)`
- not understood: Whisk the eggs.: step(make_omelette, make_omelette_s1) does not say what the sentence says
- not understood: At the same time, heat the pan.: step(make_omelette, make_omelette_s2) does not say what the sentence says; overlaps(make_omelette_s2, make_omelette_s1) does not say what the sentence says

### g147 (acts differ)

> Whisk the batter, then fry it in a pan for 2 minutes.

- missing `tell`
- missing `tell|action(make_pancake_s1, whisk)`
- missing `tell|action(make_pancake_s2, fry)`
- missing `tell|before(make_pancake_s1, make_pancake_s2)`
- missing `tell|object_of(make_pancake_s1, batter)`
- missing `tell|object_of(make_pancake_s2, batter)`
- missing `tell|step(make_pancake, make_pancake_s1)`
- missing `tell|step(make_pancake, make_pancake_s2)`
- missing `tell|takes_time(make_pancake_s2, 2, 2, minute)`
- missing `tell|uses(make_pancake_s2, pan)`
- not understood: Whisk the batter, then fry it in a pan for 2 minutes.: step(make_pancake, make_pancake_s1) does not say what the sentence says; step(make_pancake, make_pancake_s2) does not say what the sentence says; takes_time(make_pancake_s2, 2, 2, minute) does not say what the sentence says; before(make_pancake_s1, make_pancake_s2) does not say what the sentence says

### g154 (acts differ)

> That's not right, Nina is 12.

- missing `correct`
- missing `correct|property(nina, age, 12)`
- extra `feedback`
- extra `feedback|negative`

### g155 (acts differ)

> Wrong, a tomato is a fruit.

- missing `correct`
- missing `correct|is_a(tomato, fruit)`
- extra `feedback`
- extra `feedback|negative`

### g161 (acts differ)

> Done! What's next?

- missing `ask_next`
- extra `ask`
- extra `ask|is_a(next, W)`

### g167 

> I finished step 2.

- missing `done|make_tea_s2`
- extra `done|step_2`

### g168 (acts differ)

> I'm out of butter.

- missing `lack`
- missing `lack|butter`
- extra `inform`
- extra `inform|-available(butter)`

### g170 (acts differ)

> Perfect, that's exactly what I needed!

- extra `social`
- extra `social|reaction`

### g173 (acts differ)

> The onions are soft now. What's next?

- missing `ask_next`
- missing `done`
- missing `done|last`
- extra `ask`
- extra `ask|is_a(next, W)`
- extra `tell`
- extra `tell|property(onion, texture, soft)`

### g175 (acts differ)

> Great, that works for me.

- extra `social`
- extra `social|reaction`

### g178 (acts differ)

> Hmm, I don't have eggs. What else could I use?

- missing `lack`
- missing `lack|egg`
- extra `feedback`
- extra `feedback|negative`

### g182 (acts differ)

> Hey, how are you?

- missing `greet`

### g192 (acts differ)

> Thanks, bye!

- missing `bye`

### g193 (acts differ)

> Have you ever tried sushi?

- missing `about_you`
- missing `about_you|sushi`
- not understood: Have you ever tried sushi?: `...` is not grounded: "..." is not in the message or the conversation

### g195 (acts differ)

> Goodbye, and thanks for the help!

- missing `bye`
- missing `social`
- missing `social|thanks`
- extra `greet`

### g200 (acts differ)

> Life's funny like that, I guess.

- extra `social`
- extra `social|reaction`

