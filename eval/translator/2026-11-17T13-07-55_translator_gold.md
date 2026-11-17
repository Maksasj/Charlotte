# Translator on `translator_gold`

Atom-level precision 0.927, recall 0.923, F1 **0.925**; act accuracy **90.5%**; sentences recovered 223/235 (94.9%); ungrounded constants 0; translation time median 2369 ms, 95th percentile 4699 ms; model calls 172.

## Items that differ from the gold

### g004 (acts differ)

> Hey, could you help me make pancakes for breakfast?

- missing `request`
- missing `request|make_pancake`

### g011 (acts differ)

> Hi there, can you help me iron a shirt?

- missing `greet`

### g014 (acts differ)

> Hey, I need help organizing my closet.

- missing `request`
- missing `request|organize_closet`

### g019 (acts differ)

> Thanks a lot! Could you help me clean the bathroom next?

- extra `ask_next`

### g027 

> Could you help me make pizza? I've got about 1 hour.

- extra `inform|available(hour)`

### g028 (acts differ)

> Hi! I need help preparing lunch for my kids.

- extra `inform`
- extra `inform|available(kid)`

### g048 

> How long does step 2 take?

- missing `ask_duration|make_tea_s2`
- extra `ask_duration|step_2`

### g066 

> My dog is hungry, and I only have rice and chicken.

- missing `inform|feels(my_dog, hungry)`
- extra `inform|feels(maksim, hungry)`

### g067 

> My faucet is leaking. Can you help me fix it?

- missing `inform|leak(my_faucet)`
- extra `inform|has(my_faucet, leak)`

### g071 (acts differ)

> I have butter, sugar and two lemons.


### g076 

> I have a bike pump and a patch kit.

- missing `inform|available(bike_pump)`
- missing `inform|available(patch_kit)`
- extra `inform|has(my_bike, bike_pump)`
- extra `inform|has(my_bike, patch_kit)`

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
- extra `tell|has(maksim, anna)`
- extra `tell|is_a(anna, sister)`

### g130 

> To bake bread you need flour, yeast, water and salt.

- missing `tell|needs(bake_bread, flour)`
- missing `tell|needs(bake_bread, salt)`
- missing `tell|needs(bake_bread, water)`
- missing `tell|needs(bake_bread, yeast)`
- extra `tell|needs(make_bread, flour)`
- extra `tell|needs(make_bread, salt)`
- extra `tell|needs(make_bread, water)`
- extra `tell|needs(make_bread, yeast)`

### g135 

> Bob lived in Rome. Now he lives in Oslo.

- missing `tell|located_in(bob, rome)`
- extra `tell|live_in(bob, rome)`

### g136 

> First chop the onions, then fry them in butter for 5 minutes.

- missing `tell|action(make_soup_s2, fry)`
- missing `tell|before(make_soup_s1, make_soup_s2)`
- missing `tell|object_of(make_soup_s2, onion)`
- missing `tell|step(make_soup, make_soup_s2)`
- missing `tell|takes_time(make_soup_s2, 5, 5, minute)`
- missing `tell|uses(make_soup_s2, butter)`

### g155 (acts differ)

> Wrong, a tomato is a fruit.

- missing `correct`
- missing `correct|is_a(tomato, fruit)`
- extra `feedback`
- extra `feedback|negative`

### g166 (acts differ)

> I did that, the water is boiling.

- missing `done`
- missing `done|last`
- extra `inform`
- extra `inform|available(boiling_water)`

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

### g171 (acts differ)

> No thanks, I'd rather skip that.

- missing `reject`
- extra `feedback`
- extra `feedback|negative`

### g173 (acts differ)

> The onions are soft now. What's next?

- missing `done`
- missing `done|last`
- extra `tell`
- extra `tell|property(onion, texture, soft)`

### g182 (acts differ)

> Hey, how are you?

- missing `greet`

### g187 (acts differ)

> Do you like soup?

- missing `about_you`
- missing `about_you|soup`
- not understood: Do you like soup?: `music` is not grounded: "music" is not in the message or the conversation

### g192 (acts differ)

> Thanks, bye!

- missing `bye`

### g195 (acts differ)

> Goodbye, and thanks for the help!

- missing `bye`

### g199 (acts differ)

> Thanks! Anyway, that's just how it goes sometimes.

- extra `social|reaction`

### g200 (acts differ)

> Life's funny like that, I guess.

- extra `social`
- extra `social|reaction`

