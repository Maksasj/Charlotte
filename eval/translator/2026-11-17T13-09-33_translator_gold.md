# Translator on `translator_gold`

Atom-level precision 0.929, recall 0.951, F1 **0.940**; act accuracy **90.5%**; sentences recovered 226/235 (96.2%); ungrounded constants 0; translation time median 2361 ms, 95th percentile 4794 ms; model calls 176.

## Items that differ from the gold

### g011 (acts differ)

> Hi there, can you help me iron a shirt?

- missing `greet`

### g014 (acts differ)

> Hey, I need help organizing my closet.

- missing `greet`

### g019 (acts differ)

> Thanks a lot! Could you help me clean the bathroom next?

- extra `ask_next`

### g022 (acts differ)

> I need help with packing my suitcase for Monday.

- extra `inform`
- extra `inform|has(my_suitcase, pack)`

### g027 

> Could you help me make pizza? I've got about 1 hour.

- missing `inform|time_available(1, hour)`
- extra `inform|available(hour)`

### g066 

> My dog is hungry, and I only have rice and chicken.

- missing `inform|feels(my_dog, hungry)`
- extra `inform|feels(maksim, hungry)`

### g067 

> My faucet is leaking. Can you help me fix it?

- missing `inform|leak(my_faucet)`
- extra `inform|has(my_faucet, leak)`

### g076 

> I have a bike pump and a patch kit.

- missing `inform|available(bike_pump)`
- missing `inform|available(patch_kit)`
- extra `inform|has(my_bike, bike_pump)`
- extra `inform|has(my_bike, patch_kit)`

### g082 (acts differ)

> My oven is small, but I have a big frying pan.

- missing `inform|property(my_oven, size, small)`
- extra `inform|-property(oven, size, big)`
- extra `inform|property(oven, size, small)`

### g107 

> Baking bread takes about an hour.

- missing `tell|takes_time(bake_bread, 1, 1, hour)`
- extra `tell|take_about(baking_bread, hour)`

### g109 

> Whales are not fish.

- extra `tell|@usually`

### g111 

> You can use butter instead of oil.

- missing `tell|substitute(oil, butter)`
- extra `tell|substitute(butter, oil)`

### g112 

> Salt is in almost every home.

- missing `tell|staple(salt)`
- extra `tell|located_in(salt, almost_home)`

### g116 

> Some apples are green.

- missing `tell|@sometimes`
- missing `tell|property(apple, colour, green)`
- extra `tell|is_a(apple, green)`

### g119 (acts differ)

> I used to live in Berlin.

- missing `tell`
- missing `tell|@past`
- missing `tell|located_in(maksim, berlin)`
- extra `inform`
- extra `inform|-live_in(maksim, berlin)`

### g121 

> My mother lives in Madrid.

- missing `tell|located_in(my_mother, madrid)`
- extra `tell|is_a(mother, my_mother)`

### g136 (acts differ)

> First chop the onions, then fry them in butter for 5 minutes.


### g154 (acts differ)

> That's not right, Nina is 12.

- extra `feedback`
- extra `feedback|negative`

### g155 (acts differ)

> Wrong, a tomato is a fruit.

- extra `feedback`
- extra `feedback|negative`

### g166 (acts differ)

> I did that, the water is boiling.

- missing `done`
- missing `done|last`
- extra `inform`
- extra `inform|has(my_water, boiling)`

### g167 (acts differ)

> I finished step 2.

- missing `done`
- missing `done|make_tea_s2`
- not understood: I finished step 2.: unreadable lines: done: step(make_tea, make_tea_s2)

### g170 (acts differ)

> Perfect, that's exactly what I needed!

- extra `social`
- extra `social|thanks`

### g173 (acts differ)

> The onions are soft now. What's next?

- missing `done`
- missing `done|last`
- extra `tell`
- extra `tell|property(onion, texture, soft)`

### g176 (acts differ)

> OK, done.

- extra `ask_next`

### g178 (acts differ)

> Hmm, I don't have eggs. What else could I use?

- missing `ask`
- missing `ask|substitute(egg, W)`
- extra `ask_need`
- extra `ask_need|make_pancake`
- extra `feedback`
- extra `feedback|negative`

### g183 (acts differ)

> Bye, see you tomorrow!

- missing `bye`
- not understood: Bye, see you tomorrow!: `...` is not grounded: "..." is not in the message or the conversation

### g187 (acts differ)

> Do you like soup?

- missing `about_you`
- missing `about_you|soup`
- extra `ask`
- extra `ask|likes(maksim, soup)`

### g199 (acts differ)

> Thanks! Anyway, that's just how it goes sometimes.

- extra `social|reaction`

### g200 (acts differ)

> Life's funny like that, I guess.

- extra `social`
- extra `social|reaction`

