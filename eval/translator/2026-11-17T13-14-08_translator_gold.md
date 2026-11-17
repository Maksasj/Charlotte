# Translator on `translator_gold`

Atom-level precision 0.930, recall 0.961, F1 **0.945**; act accuracy **92.0%**; sentences recovered 228/235 (97.0%); ungrounded constants 0; translation time median 2389 ms, 95th percentile 4838 ms; model calls 170.

## Items that differ from the gold

### g019 (acts differ)

> Thanks a lot! Could you help me clean the bathroom next?

- extra `ask_next`

### g022 (acts differ)

> I need help with packing my suitcase for Monday.

- extra `inform`
- extra `inform|has(my_suitcase, pack)`

### g028 (acts differ)

> Hi! I need help preparing lunch for my kids.

- extra `inform`
- extra `inform|available(kid)`

### g066 (acts differ)

> My dog is hungry, and I only have rice and chicken.


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

### g082 

> My oven is small, but I have a big frying pan.

- missing `inform|available(frying_pan)`
- missing `inform|property(my_oven, size, small)`
- extra `inform|-located_in(oven, small)`
- extra `inform|located_in(oven, small)`

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

### g118 (acts differ)

> I'm vegetarian.

- missing `tell`
- missing `tell|is_a(maksim, vegetarian)`
- extra `inform`
- extra `inform|-is_a(maksim, vegetarian)`

### g120 

> Anna is my sister.

- missing `tell|property(maksim, sister, anna)`
- extra `tell|is_a(anna, sister)`

### g121 (acts differ)

> My mother lives in Madrid.

- missing `tell`
- missing `tell|located_in(my_mother, madrid)`
- extra `inform`
- extra `inform|located_in(my_mother, madrid)`

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

### g136 (acts differ)

> First chop the onions, then fry them in butter for 5 minutes.


### g158 (acts differ)

> I'd rather not cook fish today.

- extra `inform`
- extra `inform|-wants(maksim, fish)`

### g166 (acts differ)

> I did that, the water is boiling.

- missing `done|last`
- extra `done|make_tea_s3`
- extra `feedback`
- extra `feedback|positive`

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

- extra `feedback`
- extra `feedback|negative`

### g193 (acts differ)

> Have you ever tried sushi?

- missing `about_you`
- missing `about_you|sushi`
- not understood: Have you ever tried sushi?: `...` is not grounded: "..." is not in the message or the conversation

### g199 (acts differ)

> Thanks! Anyway, that's just how it goes sometimes.

- extra `social|reaction`

### g200 (acts differ)

> Life's funny like that, I guess.

- extra `social`
- extra `social|reaction`

