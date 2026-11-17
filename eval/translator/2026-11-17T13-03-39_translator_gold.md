# Translator on `translator_gold`

Atom-level precision 0.864, recall 0.769, F1 **0.814**; act accuracy **84.0%**; sentences recovered 199/235 (84.7%); ungrounded constants 0; translation time median 0 ms, 95th percentile 2748 ms; model calls 35.

## Items that differ from the gold

### g004 (acts differ)

> Hey, could you help me make pancakes for breakfast?

- missing `request`
- missing `request|make_pancake`

### g006 (acts differ)

> Good morning! I'd love some help planning a party for my sister.

- missing `request`
- missing `request|plan_party`
- not understood: I'd love some help planning a party for my sister.: `birthday` is not grounded: "birthday" is not in the message or the conversation

### g011 (acts differ)

> Hi there, can you help me iron a shirt?

- missing `request`
- missing `request|iron_shirt`

### g014 (acts differ)

> Hey, I need help organizing my closet.

- missing `request`
- missing `request|organize_closet`

### g015 

> Would you help me make a salad? I have tomatoes, cucumbers and feta.

- missing `inform|available(cucumber)`
- missing `inform|available(feta)`
- missing `inform|available(tomato)`
- extra `inform|has(my_tomato, cucumber)`
- extra `inform|has(my_tomato, feta)`

### g028 (acts differ)

> Hi! I need help preparing lunch for my kids.

- extra `inform`
- extra `inform|has(my_kid, need)`

### g029 

> Can you help me fix a leaky faucet?

- missing `request|fix_faucet`
- extra `request|fix_leaky_faucet`

### g033 

> Can you help me roast vegetables? I have carrots, potatoes and onions.

- missing `inform|available(carrot)`
- missing `inform|available(onion)`
- missing `inform|available(potato)`
- extra `inform|has(maksim, carrot)`
- extra `inform|has(maksim, onion)`
- extra `inform|has(maksim, potato)`

### g043 

> Thanks! How long do I boil it?

- missing `ask_duration|boil_egg`
- extra `ask_duration|cook_egg`

### g048 (acts differ)

> How long does step 2 take?

- missing `ask_duration`
- missing `ask_duration|make_tea_s2`
- not understood: How long does step 2 take?: unreadable lines: ask_duration: step(make_tea, make_tea_s2)

### g049 

> How do I brew coffee?

- missing `ask_how|brew_coffee`
- extra `ask_how|make_coffee`

### g050 (acts differ)

> What do I need to plant a tree?

- missing `ask_need`
- missing `ask_need|plant_tree`
- extra `request`
- extra `request|plant_tree`

### g051 

> How long does it take to roast a chicken?

- missing `ask_duration|roast_chicken`
- extra `ask_duration|cook_roast_chicken`

### g052 (acts differ)

> Hi! How do I make tea?

- missing `ask_how`
- missing `ask_how|make_tea`
- extra `request`
- extra `request|make_tea`

### g053 (acts differ)

> Thanks. What do I need to clean windows?

- missing `ask_need`
- missing `ask_need|clean_window`
- extra `request`
- extra `request|clean_window`

### g064 (acts differ)

> My kitchen is small. Can you help me clean it?

- missing `inform`
- missing `inform|property(my_kitchen, size, small)`
- extra `tell`
- extra `tell|property(my_kitchen, size, small)`

### g066 

> My dog is hungry, and I only have rice and chicken.

- missing `inform|available(chicken)`
- missing `inform|available(rice)`
- missing `inform|feels(my_dog, hungry)`
- extra `inform|is_a(my_dog, dog)`

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

### g073 (acts differ)

> I'm hungry. Can you help me make lunch?

- missing `inform`
- missing `inform|feels(maksim, hungry)`
- not understood: I'm hungry.: has_state(maksim, hungry): unknown predicate has_state (it needs review before facts can use it)

### g076 

> I have a bike pump and a patch kit.

- missing `inform|available(bike_pump)`
- missing `inform|available(patch_kit)`
- extra `inform|has(maksim, bike_pump)`
- extra `inform|has(maksim, patch_kit)`

### g078 (acts differ)

> My cat has fleas. Can you help me wash my cat?

- missing `inform`
- missing `inform|has(my_cat, flea)`
- extra `tell`
- extra `tell|has(my_cat, flea)`

### g079 

> I have one pan and a small pot.

- missing `inform|available(pan)`
- missing `inform|available(pot)`
- extra `inform|has(maksim, pan)`
- extra `inform|has(maksim, pot)`

### g082 

> My oven is small, but I have a big frying pan.

- missing `inform|available(frying_pan)`
- extra `inform|available(big_frying_pan)`

### g084 

> Hi! I have oats, bananas and milk.

- missing `inform|available(banana)`
- missing `inform|available(milk)`
- missing `inform|available(oat)`
- extra `inform|has(maksim, banana)`
- extra `inform|has(maksim, milk)`
- extra `inform|has(maksim, oats)`

### g107 

> Baking bread takes about an hour.

- missing `tell|takes_time(bake_bread, 1, 1, hour)`
- extra `tell|take_about(baking_bread, hour)`

### g111 

> You can use butter instead of oil.

- missing `tell|substitute(oil, butter)`
- extra `tell|substitute(butter, oil)`

### g112 

> Salt is in almost every home.

- missing `tell|staple(salt)`
- extra `tell|located_in(salt, almost_home)`

### g113 (acts differ)

> One person needs about 100 grams of pasta.

- missing `tell`
- missing `tell|portion(pasta, 100, gram)`
- extra `inform`
- extra `inform|portion(pasta, 100, gram)`

### g115 (acts differ)

> Tomatoes are usually red.

- extra `tell|is_a(tomato, red)`

### g116 

> Some apples are green.

- missing `tell|property(apple, colour, green)`
- extra `tell|is_a(apple, green)`

### g118 

> I'm vegetarian.

- missing `tell|is_a(maksim, vegetarian)`
- extra `tell|property(maksim, diet, vegetarian)`

### g128 (acts differ)

> Nina is 12.

- missing `tell`
- missing `tell|property(nina, age, 12)`
- extra `inform`
- extra `inform|property(nina, age, 12)`

### g129 

> Honey can replace sugar.

- missing `tell|substitute(sugar, honey)`
- extra `tell|substitute(honey, sugar)`

### g133 

> Pasta usually takes about 10 minutes to cook.

- missing `tell|takes_time(cook_pasta, 10, 10, minute)`
- extra `tell|takes_time(pasta, 10, 10, minute)`

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
- not understood: then fry them in butter for 5 minutes: not understood

### g137 

> Boil the water. Then steep the tea bag for 3 minutes.

- missing `tell|action(make_tea_s1, boil)`
- missing `tell|object_of(make_tea_s1, water)`
- missing `tell|step(make_tea, make_tea_s1)`
- extra `tell|water(boil)`

### g139 (acts differ)

> Drain the pasta in a colander.

- missing `tell`
- missing `tell|action(make_pasta_s3, drain)`
- missing `tell|object_of(make_pasta_s3, pasta)`
- missing `tell|step(make_pasta, make_pasta_s3)`
- missing `tell|uses(make_pasta_s3, colander)`
- not understood: Drain the pasta in a colander.: unreadable lines: task: ... / done: make_pasta_s2 / inform: ... / tell: step(make_pasta, make_pasta_s3); action(make_pasta_s3, drain); object_of(make_pasta_s3, pasta); uses(make_pasta_s3, colander); after(make_pasta_s2, make_pasta_s3) / tell: step(make_pasta, make_pasta_s3); action(make_pasta_s3, drain); object_of(make_pasta_s3, pasta); uses(make_pasta_s3, colander); before(make_pasta_s2, make_pasta_s3)

### g140 

> Put the eggs in boiling water for 8 to 10 minutes.

- missing `tell|action(boil_egg_s1, put)`
- extra `tell|action(boil_egg_s1, boil)`

### g141 (acts differ)

> Cook the carrots until they are soft.

- missing `tell`
- missing `tell|action(make_soup_s1, cook)`
- missing `tell|object_of(make_soup_s1, carrot)`
- missing `tell|step(make_soup, make_soup_s1)`
- missing `tell|until(make_soup_s1, soft)`
- not understood: Cook the carrots until they are soft.: unreadable lines: step: make_soup_s1; action(make_soup_s1, cook); object_of(make_soup_s1, carrot); until(make_soup_s1, soft)

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
- not understood: Remove the wheel.: unreadable lines: step: ... / tell: step(fix_tire, fix_tire_s1); action(fix_tire_s1, remove); object_of(fix_tire_s1, wheel); before(fix_tire_s1, fix_tire_s2)
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
- not understood: Whisk the eggs.: unreadable lines: step: make_omelette_s1 — wait, the translation format uses acts, so: / tell: step(make_omelette, make_omelette_s1); action(make_omelette_s1, whisk); object_of(make_omelette_s1, egg)
- not understood: At the same time, heat the pan.: step(make_omelette, make_omelette_s2) does not say what the sentence says; before(make_omelette_s1, make_omelette_s2) does not say what the sentence says

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

### g152 (acts differ)

> Actually, Bob works at the library.

- missing `correct`
- missing `correct|work_at(bob, library)`
- not understood: Actually, Bob works at the library.: `library_worker` is not grounded: "worker" is not in the message or the conversation

### g161 (acts differ)

> Done! What's next?

- missing `ask_next`
- extra `ask`
- extra `ask|is_a(next, W)`

### g173 (acts differ)

> The onions are soft now. What's next?

- missing `ask_next`
- missing `done`
- missing `done|last`
- extra `ask`
- extra `ask|is_a(next, W)`
- extra `tell`
- extra `tell|property(onion, texture, soft)`

### g178 (acts differ)

> Hmm, I don't have eggs. What else could I use?

- missing `ask`
- missing `ask|substitute(egg, W)`
- extra `ask_how`
- extra `ask_how|make_pancake`

### g182 (acts differ)

> Hey, how are you?

- missing `greet`

### g192 (acts differ)

> Thanks, bye!

- missing `bye`

### g195 (acts differ)

> Goodbye, and thanks for the help!

- missing `bye`
- missing `social`
- missing `social|thanks`
- extra `greet`

### g196 (acts differ)

> Honestly, some days just feel longer than others.

- extra `tell`
- extra `tell|@sometimes`
- extra `tell|feel(day, longer)`

### g197 (acts differ)

> Whatever, it's not like it matters.

- extra `social`
- extra `social|reaction`

### g199 (acts differ)

> Thanks! Anyway, that's just how it goes sometimes.

- extra `social|reaction`

### g200 (acts differ)

> Life's funny like that, I guess.

- extra `social`
- extra `social|reaction`

