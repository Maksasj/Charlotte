---
id: start_compost_heap
stage: household
task: start_compost_heap
schema: v3
tier: G
---
# Start a compost heap

## requests
- I'd like to stop binning my veg peelings and make my own compost. How do I set it up?
- Just got a compost bin delivered. What's the way to get it going?
- How do I start composting at the bottom of the garden?

## situation
- I have a compost bin with a lid, some twigs, kitchen scraps and grass cuttings, dry leaves and cardboard, a watering can and water.

## ground truth
```datalog
task(start_compost_heap).
domain(start_compost_heap, gardening).
goal(start_compost_heap, compost).
needs(start_compost_heap, compost_bin).
needs(start_compost_heap, twig).
needs(start_compost_heap, green_waste).
needs(start_compost_heap, brown_waste).
needs(start_compost_heap, watering_can).
needs(start_compost_heap, water).
needs(start_compost_heap, lid).
takes_time(start_compost_heap, 20, 30, minute).
step(start_compost_heap, start_compost_heap_s1).
action(start_compost_heap_s1, place).
object_of(start_compost_heap_s1, compost_bin).
onto(start_compost_heap_s1, soil).
purpose(start_compost_heap_s1, worms_enter).
step(start_compost_heap, start_compost_heap_s2).
action(start_compost_heap_s2, put).
object_of(start_compost_heap_s2, twig).
into(start_compost_heap_s2, compost_bin).
purpose(start_compost_heap_s2, air_flow).
step(start_compost_heap, start_compost_heap_s3).
action(start_compost_heap_s3, add).
object_of(start_compost_heap_s3, green_waste).
into(start_compost_heap_s3, compost_bin).
step(start_compost_heap, start_compost_heap_s4).
action(start_compost_heap_s4, add).
object_of(start_compost_heap_s4, brown_waste).
into(start_compost_heap_s4, compost_bin).
purpose(start_compost_heap_s4, balance_mix).
step(start_compost_heap, start_compost_heap_s5).
action(start_compost_heap_s5, water).
object_of(start_compost_heap_s5, heap).
uses(start_compost_heap_s5, watering_can).
role(start_compost_heap_s5, watering_can, pour).
uses(start_compost_heap_s5, water).
role(start_compost_heap_s5, water, wet).
until(start_compost_heap_s5, damp).
step(start_compost_heap, start_compost_heap_s6).
action(start_compost_heap_s6, cover).
object_of(start_compost_heap_s6, compost_bin).
uses(start_compost_heap_s6, lid).
role(start_compost_heap_s6, lid, cover).
purpose(start_compost_heap_s6, keeps_warm).
before(start_compost_heap_s1, start_compost_heap_s2).
before(start_compost_heap_s2, start_compost_heap_s3).
before(start_compost_heap_s3, start_compost_heap_s4).
before(start_compost_heap_s4, start_compost_heap_s5).
before(start_compost_heap_s5, start_compost_heap_s6).
```

## aliases
```datalog
task_alias(start_compost_heap, "starting a compost heap").
task_alias(start_compost_heap, "start composting").
task_alias(start_compost_heap, "make my own compost").
alias(compost_bin, "compost bin").
alias(twig, "twigs").
alias(green_waste, "kitchen scraps and grass cuttings").
alias(green_waste, "veg peelings").
alias(brown_waste, "dry leaves and cardboard").
alias(heap, "the heap").
alias(damp, "damp like a wrung-out sponge").
alias(old_carpet, "piece of old carpet").
alias(compost_worm, "compost worms").
alias(compost_worm, "brandling worms").
```

## teach
- To start a compost heap you need a compost bin with a lid, twigs, green waste such as kitchen scraps and grass cuttings, brown waste such as dry leaves and cardboard, a watering can and water.
- Starting a compost heap takes 20 to 30 minutes.
- Step 1 of starting a compost heap: place the compost bin on bare soil so worms can get in.
- Step 2 of starting a compost heap: put a layer of twigs in the bottom of the bin so air can flow.
- Step 3 of starting a compost heap: add a layer of green waste.
- Step 4 of starting a compost heap: add a layer of brown waste to balance the mix.
- Step 5 of starting a compost heap: water the heap with the watering can until it is damp.
- Step 6 of starting a compost heap: cover the bin with its lid to keep the heat in.

## checklist
- names what is needed: a compost bin, twigs, green waste, brown waste and water
- gives the steps in order: bin on soil, twigs, green layer, brown layer, water, cover
- says to put the bin on bare soil and keep the heap damp
- gives a total time of about 20 to 30 minutes

## variant
- change: no lid: cover the heap with an old piece of carpet
- request: My compost bin came without a lid. I've got a scrap of old carpet in the shed, will that work?
- situation: I have a compost bin with no lid, twigs, green and brown waste, a watering can, water and an old piece of carpet.
- expect: The reply covers the heap with the old carpet instead of a lid and keeps the other steps.

```datalog situation
lacks(user, lid).
has(user, old_carpet).
has(user, compost_bin).
has(user, twig).
has(user, green_waste).
has(user, brown_waste).
has(user, watering_can).
has(user, water).
```

```datalog difference
no_need(lid).
extra_need(old_carpet).
swap(start_compost_heap_s6, lid, old_carpet).
```

## variant
- change: wants worms: add compost worms after watering
- request: A friend gave me a tub of compost worms. How do I set up my new heap and get them in there?
- situation: I have a compost bin with a lid, twigs, green and brown waste, a watering can, water and a tub of compost worms.
- expect: The reply adds a step to tip the compost worms onto the damp heap before putting the lid on.

```datalog situation
has(user, compost_bin).
has(user, lid).
has(user, twig).
has(user, green_waste).
has(user, brown_waste).
has(user, watering_can).
has(user, water).
has(user, compost_worm).
wants(user, compost_worm).
```

```datalog difference
extra_need(compost_worm).
add_step(start_compost_heap_s9, start_compost_heap_s5).
step(start_compost_heap, start_compost_heap_s9).
action(start_compost_heap_s9, add).
object_of(start_compost_heap_s9, compost_worm).
into(start_compost_heap_s9, compost_bin).
before(start_compost_heap_s5, start_compost_heap_s9).
before(start_compost_heap_s9, start_compost_heap_s6).
```
