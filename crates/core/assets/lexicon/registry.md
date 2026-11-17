---
id: registry
status: confirmed
conf: 1
evidence: 0
counter: 0
module: lexicon
source: builtin
---
# Predicate registry

Every predicate the agent may store or ask about, with its argument types,
a definition, sentence frames (`{1}` is the first argument), synonyms and flags.
Argument types: entity, category, event, task, step, number, unit, fact (a fact id
such as `f12`), text (a quoted string), symbol (a plain word), value (entity-like
word or number), any. Flags: askable, rule_head, single_valued, open_world.
New predicates come in through the review queue, never silently. Edit freely;
the agent reloads it like any vault file.

```datalog
pred(thing, "entity", "X is a thing").
frame(thing, "{1} is a thing").
pred(same, "entity entity", "X and Y are the same thing").
frame(same, "{1} is the same as {2}").
pred(is_a, "entity category", "X is a kind of Y, or an instance of Y").
frame(is_a, "{1} is a {2}").
flag(is_a, askable).
flag(is_a, rule_head).
synonym(kind_of, is_a).
synonym(type_of, is_a).
pred(part_of, "entity entity", "X is part of Y").
frame(part_of, "{1} is part of {2}").
flag(part_of, askable).
flag(part_of, rule_head).
pred(has, "entity entity", "X has Y").
frame(has, "{1} has {2}").
flag(has, askable).
flag(has, rule_head).
flag(has, open_world).
synonym(own, has).
synonym(possess, has).
pred(property, "entity symbol value", "X has property P with value V").
frame(property, "the {2} of {1} is {3}").
flag(property, askable).
flag(property, rule_head).
pred(before, "any any", "A happens before B").
frame(before, "{1} comes before {2}").
flag(before, askable).
flag(before, rule_head).
pred(during, "any any", "A happens during T").
frame(during, "{1} happens during {2}").
flag(during, askable).
pred(event, "event", "E is an event").
frame(event, "{1} is an event").
pred(agent_of, "event entity", "X is the one who acts in event E").
frame(agent_of, "{2} does {1}").
pred(does, "entity symbol", "X does action A").
frame(does, "{1} does {2}").
flag(does, askable).
pred(causes, "any any", "A causes B").
frame(causes, "{1} causes {2}").
flag(causes, askable).
flag(causes, rule_head).
synonym(lead_to, causes).
pred(wants, "entity any", "X wants Y").
frame(wants, "{1} wants {2}").
flag(wants, askable).
flag(wants, rule_head).
flag(wants, open_world).
synonym(want, wants).
pred(knows, "entity any", "X knows fact or thing F").
frame(knows, "{1} knows {2}").
flag(knows, askable).
flag(knows, open_world).
pred(believes, "entity any", "X believes fact F").
frame(believes, "{1} believes {2}").
flag(believes, askable).
flag(believes, open_world).
pred(feels, "entity symbol", "X feels F").
frame(feels, "{1} feels {2}").
flag(feels, askable).
pred(likes, "entity any", "X likes Y").
frame(likes, "{1} likes {2}").
flag(likes, askable).
flag(likes, rule_head).
synonym(enjoy, likes).
synonym(love, likes).
pred(person, "entity", "X is a person").
frame(person, "{1} is a person").
flag(person, rule_head).
pred(place, "entity", "X is a place").
frame(place, "{1} is a place").
flag(place, rule_head).
pred(located_in, "entity entity", "X is in place Y").
frame(located_in, "{1} is in {2}").
flag(located_in, askable).
flag(located_in, rule_head).
flag(located_in, single_valued).
synonym(live_in, located_in).
pred(says, "entity any", "X says fact F").
frame(says, "{1} says {2}").
pred(proper_name, "entity", "X is written as a name").
pred(work_at, "entity entity", "X works at Y").
frame(work_at, "{1} works at {2}").
flag(work_at, askable).
flag(work_at, rule_head).
pred(task, "task", "T is a task someone can do").
frame(task, "{1} is something you can do").
flag(task, rule_head).
pred(step, "task step", "S is a step of task T").
frame(step, "{2} is a step of {1}").
pred(action, "any symbol", "the action of step or event E is A").
pred(object_of, "any entity", "the thing acted on in E is X").
pred(recipient, "event entity", "the one who receives in event E is X").
pred(needs, "task entity", "task T needs item I").
frame(needs, "{1} needs {2}").
flag(needs, askable).
flag(needs, rule_head).
synonym(require, needs).
pred(uses, "step entity", "step S uses item I").
frame(uses, "{1} uses {2}").
pred(takes_time, "any number number unit", "X takes Lo to Hi of unit U").
frame(takes_time, "{1} takes {2} to {3} {4}s").
flag(takes_time, askable).
flag(takes_time, single_valued).
synonym(last, takes_time).
pred(until, "step symbol", "step S goes on until state C").
frame(until, "until {2}").
pred(overlaps, "step step", "step S runs while step R runs").
frame(overlaps, "while {2}").
pred(method, "task task", "task T can be done by doing task M").
pred(subtask, "task task", "task T includes task U").
frame(subtask, "{1} includes {2}").
pred(done_when, "task symbol", "task T is done when state C holds").
frame(done_when, "{1} is done when {2}").
pred(solves, "task symbol", "task T is a way to reach goal G").
frame(solves, "{1} is a way to {2}").
flag(solves, askable).
pred(recommended_for, "task any", "option T is good for situation X").
frame(recommended_for, "{1} is good for {2}").
flag(recommended_for, askable).
pred(available, "entity", "the user has item I").
frame(available, "you have {1}").
pred(time_available, "number unit", "the user has N units of time").
frame(time_available, "you have {1} {2}").
flag(time_available, single_valued).
pred(staple, "entity", "item I is in almost every home").
frame(staple, "{1} is in almost every home").
pred(substitute, "entity entity", "item J can replace item I").
frame(substitute, "{2} can replace {1}").
flag(substitute, askable).
pred(portion, "entity number unit", "one person needs N of item I").
frame(portion, "one person needs {2} {3} of {1}").
flag(portion, single_valued).
pred(active_task, "task", "the conversation is about task T").
pred(at_step, "step", "the user is at step S").
pred(slot, "symbol any", "slot N of the active task has value V").
pred(open_question, "any", "the agent is waiting for an answer to Q").
pred(grounded, "fact", "fact F was confirmed in this conversation").
pred(said, "fact", "the agent already said fact F").
pred(done, "step", "the user has done step S").
pred(same_pred, "symbol symbol", "predicates P and Q mean the same").
pred(surface, "entity text", "X is written Text").
pred(plural_of, "entity text", "the plural of X is written Text").
pred(frame, "symbol text", "predicate P is said with sentence frame Text").
pred(can, "symbol", "the agent can do X").
frame(can, "I can {1}").
pred(cannot, "symbol", "the agent cannot do X").
frame(cannot, "I cannot {1}").
pred(knows_task, "task", "the agent knows how to do task T").
frame(knows_task, "I know how to do {1}").
flag(knows_task, askable).
pred(needs_all, "task entity", "everything task T needs, through steps and sub-tasks").
frame(needs_all, "{1} needs {2}").
flag(needs_all, askable).
pred(missing, "task entity", "item I is needed for T and the user lacks it").
frame(missing, "you are missing {2} for {1}").
flag(missing, askable).
pred(covered, "entity", "a substitute for I is at hand").
frame(covered, "something can replace {1}").
pred(lacking, "task", "something needed for T is missing").
frame(lacking, "something for {1} is missing").
pred(feasible, "task", "the user can do T with what they have").
frame(feasible, "you can make {1}").
flag(feasible, askable).
pred(option_for, "symbol task", "a feasible way to reach goal G").
frame(option_for, "{2} is a way to {1}").
flag(option_for, askable).
pred(waiting, "step", "a step that must wait for an earlier one").
frame(waiting, "{1} must wait").
pred(next_step, "task step", "the next step of T to do").
flag(next_step, askable).
pred(minutes, "any number number", "X takes Lo to Hi minutes").
frame(minutes, "{1} takes {2} to {3} minutes").
flag(minutes, askable).
pred(adds_time, "task step", "step S adds to the time of T").
frame(adds_time, "{2} adds to the time of {1}").
pred(side_by_side, "step", "step S runs during another step").
frame(side_by_side, "{1} runs while another step runs").
pred(total_minutes, "task number number", "task T takes Lo to Hi minutes in all").
frame(total_minutes, "{1} takes {2} to {3} minutes in all").
flag(total_minutes, askable).
pred(step_count, "task number", "task T has N steps").
frame(step_count, "{1} has {2} steps").
flag(step_count, askable).
pred(task_slot, "task symbol symbol", "task T has slot S, required or optional").
pred(slot_question, "task symbol text", "the agent asks Text to fill slot S of task T").
pred(slot_type, "task symbol symbol", "slot S of task T holds things of kind K").
pred(closing, "task text", "the line that closes task T").
pred(slot_fills, "task symbol symbol", "slot S of task T is filled by facts of predicate P").
pred(heard, "symbol", "the message held act A").
pred(requested, "task", "the person asked for help with G").
pred(open_frame, "task", "the conversation has an open task frame for G").
pred(filled, "task symbol", "slot S of frame G is filled").
pred(active, "task", "procedure T is under way").
pred(chosen, "task", "option T was already offered").
pred(known_goal, "task", "the agent knows goal or task G").
pred(missing_slot, "task symbol", "slot S of frame G is still open").
pred(asking, "task", "frame G still has open slots").
pred(budget, "number", "the person has N minutes").
pred(too_long, "task", "T takes longer than the time the person has").
pred(candidate, "task task", "T is an option for frame G").
pred(good_option, "task task", "T is a feasible option for G that fits the time").
pred(has_good, "task", "frame G has a good option").
pred(offered, "task", "an option for G was already offered").
pred(has_next, "task", "procedure T has a next step").
pred(any_active, "", "some procedure is under way").
pred(any_frame, "", "some frame is open").
pred(dm_move, "symbol number", "the dialogue move M with priority P").
pred(fact_id, "fact any any", "fact F is about these values").
% Verbs from the pattern parser: verb(V) means V(X) or V(X, Y) with an entity first.
verb(like).
verb(love).
verb(enjoy).
verb(adore).
verb(dislike).
verb(hate).
verb(detest).
verb(have).
verb(own).
verb(possess).
verb(want).
verb(need).
verb(know).
verb(feel).
verb(live).
verb(cause).
verb(drink).
verb(eat).
verb(play).
verb(work).
verb(read).
verb(write).
verb(drive).
verb(cook).
verb(fly).
verb(swim).
verb(run).
verb(walk).
verb(speak).
verb(teach).
verb(study).
verb(visit).
verb(use).
verb(wear).
verb(grow).
verb(build).
verb(sell).
verb(buy).
verb(make).
verb(watch).
verb(sing).
verb(dance).
verb(paint).
verb(fix).
verb(repair).
verb(carry).
verb(contain).
verb(produce).
verb(bark).
verb(sleep).
verb(chase).
verb(hunt).
verb(bite).
verb(climb).
verb(ride).
verb(travel).
verb(draw).
verb(collect).
verb(prefer).
verb(bake).
verb(brew).
verb(catch).
verb(code).
verb(design).
verb(manage).
verb(lead).
verb(learn).
verb(plant).
verb(water).
verb(feed).
verb(wash).
verb(clean).
verb(open).
verb(close).
verb(attend).
verb(host).
verb(meet).
verb(help).
verb(support).
verb(follow).
verb(lose).
verb(win).
verb(kick).
verb(throw).
verb(hit).
verb(require).
verb(include).
verb(serve).
verb(deliver).
verb(smell).
verb(taste).
verb(hear).
verb(see).
verb(go).
verb(come).
verb(stay).
verb(sit).
verb(stand).
verb(jump).
verb(fall).
verb(rain).
verb(snow).
verb(shine).
verb(bloom).
verb(melt).
verb(boil).
verb(freeze).
verb(burn).
verb(break).
verb(bring).
verb(take).
verb(give).
verb(send).
verb(keep).
verb(hold).
verb(belong).
verb(happen).
verb(start).
verb(end).
verb(begin).
verb(finish).
verb(call).
verb(fish).
verb(hike).
verb(ski).
verb(surf).
verb(jog).
verb(knit).
verb(sew).
verb(fight).
verb(cry).
verb(laugh).
verb(smile).
verb(talk).
verb(listen).
verb(think).
verb(believe).
verb(remember).
verb(forget).
verb(understand).
verb(translate).
verb(program).
verb(type).
verb(print).
verb(ship).
verb(store).
verb(power).
verb(heat).
verb(cool).
verb(light).
verb(connect).
verb(protect).
verb(cover).
verb(fill).
verb(pull).
verb(push).
verb(lift).
verb(cut).
verb(chop).
verb(slice).
verb(grill).
verb(fry).
verb(roast).
verb(steam).
verb(mix).
verb(pour).
verb(order).
verb(pay).
verb(cost).
verb(earn).
verb(save).
verb(spend).
verb(borrow).
verb(lend).
verb(rent).
verb(share).
verb(join).
verb(leave).
verb(enter).
verb(exit).
verb(move).
verb(stop).
verb(wait).
verb(explore).
verb(admire).
verb(trust).
verb(fear).
verb(miss).
verb(invent).
verb(discover).
verb(inspect).
verb(test).
verb(measure).
verb(weigh).
verb(count).
verb(coach).
verb(train).
verb(guard).
verb(graze).
verb(hop).
verb(crawl).
verb(dig).
verb(nest).
verb(migrate).
verb(purr).
verb(howl).
verb(roar).
verb(chirp).
verb(squeak).
verb(hum).
verb(glow).
verb(float).
verb(sink).
verb(orbit).
verb(flow).
verb(shrink).
verb(rise).
verb(set).
verb(attract).
verb(repel).
verb(dissolve).
verb(conduct).
verb(absorb).
verb(reflect).
verb(emit).
verb(smoke).
verb(vote).
verb(drown).
verb(mow).
verb(brush).
verb(comb).
verb(shave).
verb(dress).
verb(iron).
verb(fold).
verb(pack).
verb(unpack).
verb(volunteer).
verb(practice).
verb(practise).
verb(perform).
verb(compose).
verb(record).
verb(film).
verb(edit).
verb(review).
verb(publish).
verb(report).
verb(research).
verb(analyze).
verb(analyse).
verb(operate).
verb(maintain).
verb(farm).
verb(harvest).
verb(milk).
verb(herd).
verb(sail).
verb(row).
verb(paddle).
verb(skate).
verb(box).
verb(wrestle).
verb(golf).
verb(bowl).
verb(dive).
verb(camp).
verb(check).
verb(invite).
verb(hang).
verb(replace).
verb(defrost).
verb(unclog).
verb(tighten).
verb(reset).
verb(wrap).
verb(vacuum).
verb(exchange).
verb(organise).
verb(organize).
verb(plan).
verb(prepare).
% Prepositions: a verb and a preposition make a predicate (talk_to, sleep_on, study_at).
preposition(in).
preposition(at).
preposition(on).
preposition(with).
preposition(for).
preposition(to).
preposition(from).
preposition(about).
preposition(into).
preposition(near).
preposition(by).
preposition(under).
preposition(over).
preposition(after).
preposition(before).
preposition(through).
preposition(around).
preposition(across).
preposition(along).
preposition(behind).
% Units: unit(Name, Dimension, size in the dimension's base unit).
unit(second, time, 1).
unit(minute, time, 60).
unit(hour, time, 3600).
unit(day, time, 86400).
unit(week, time, 604800).
unit(gram, mass, 1).
unit(kilogram, mass, 1000).
unit(millilitre, volume, 1).
unit(litre, volume, 1000).
unit(cup, volume, 240).
unit(spoon, volume, 15).
unit(teaspoon, volume, 5).
unit(tablespoon, volume, 15).
unit(piece, count, 1).
unit(person, count, 1).
unit(slice, count, 1).
unit(degree, temperature, 1).
unit(metre, length, 1).
unit(kilometre, length, 1000).
unit(centimetre, length, 0.01).
% Words the validator never accepts as an entity, and words that glue a phrase into one constant.
function_word(here).
function_word(there).
function_word(this).
function_word(that).
function_word(these).
function_word(those).
function_word(it).
function_word(its).
function_word(they).
function_word(them).
function_word(he).
function_word(she).
function_word(him).
function_word(her).
function_word(we).
function_word(us).
function_word(you).
function_word(i).
function_word(me).
function_word(my).
function_word(your).
function_word(our).
function_word(their).
function_word(a).
function_word(an).
function_word(the).
function_word(and).
function_word(or).
function_word(but).
function_word(so).
function_word(because).
function_word(if).
function_word(then).
function_word(than).
function_word(as).
function_word(of).
function_word(to).
function_word(for).
function_word(from).
function_word(with).
function_word(without).
function_word(in).
function_word(on).
function_word(at).
function_word(by).
function_word(into).
function_word(onto).
function_word(about).
function_word(over).
function_word(under).
function_word(between).
function_word(through).
function_word(during).
function_word(before).
function_word(after).
function_word(while).
function_word(until).
function_word(when).
function_word(where).
function_word(which).
function_word(who).
function_word(whom).
function_word(whose).
function_word(what).
function_word(how).
function_word(why).
function_word(not).
function_word(no).
function_word(yes).
function_word(all).
function_word(some).
function_word(any).
function_word(each).
function_word(every).
function_word(many).
function_word(much).
function_word(more).
function_word(most).
function_word(few).
function_word(less).
function_word(least).
function_word(very).
function_word(too).
function_word(also).
function_word(just).
function_word(only).
function_word(even).
function_word(still).
function_word(first).
function_word(second).
function_word(next).
function_word(finally).
function_word(afterwards).
function_word(meanwhile).
number_word(zero).
number_word(one).
number_word(two).
number_word(three).
number_word(four).
number_word(five).
number_word(six).
number_word(seven).
number_word(eight).
number_word(nine).
number_word(ten).
number_word(eleven).
number_word(twelve).
number_word(thirteen).
number_word(fourteen).
number_word(fifteen).
number_word(sixteen).
number_word(seventeen).
number_word(eighteen).
number_word(nineteen).
number_word(twenty).
number_word(thirty).
number_word(forty).
number_word(fifty).
number_word(sixty).
number_word(seventy).
number_word(eighty).
number_word(ninety).
number_word(hundred).
number_word(thousand).
number_word(million).
number_word(half).
number_word(quarter).
number_word(dozen).
glue_word(and).
glue_word(or).
glue_word(but).
glue_word(to).
glue_word(of).
glue_word(for).
glue_word(from).
glue_word(with).
glue_word(that).
glue_word(which).
glue_word(who).
glue_word(where).
glue_word(when).
glue_word(than).
glue_word(as).
glue_word(by).
glue_word(about).
glue_word(into).
glue_word(onto).
glue_word(like).
glue_word(is).
glue_word(are).
glue_word(was).
glue_word(were).
glue_word(be).
glue_word(been).
glue_word(being).
glue_word(it).
glue_word(this).
glue_word(these).
glue_word(those).
glue_word(here).
glue_word(there).
property_name(colour).
property_name(size).
property_name(age).
property_name(job).
property_name(name).
property_name(taste).
property_name(weight).
property_name(height).
property_name(price).
property_name(temperature).
property_name(shape).
property_name(material).
property_name(smell).
property_name(sound).
property_name(speed).
property_name(difficulty).
property_name(texture).
property_name(state).
property_name(length).
property_name(width).
property_name(capacity).
property_name(flavour).
property_name(role).
property_name(value).
property_name(order).
property_name(hobby).
property_name(favourite_food).
property_name(nationality).
property_name(language).
property_name(appeal).
property_name(beauty).
property_name(brightness).
property_name(character).
property_name(clarity).
property_name(depth).
property_name(diet).
property_name(health).
property_name(loudness).
property_name(power).
property_name(safety).
property_name(strength).
property_name(wealth).
% Adjectives: adjective(Word, Property) - "Anna is lazy" is property(anna, character, lazy).
adjective(hot, temperature).
adjective(cold, temperature).
adjective(warm, temperature).
adjective(cool, temperature).
adjective(freezing, temperature).
adjective(boiling, temperature).
adjective(icy, temperature).
adjective(chilly, temperature).
adjective(heavy, weight).
adjective(light, weight).
adjective(fast, speed).
adjective(slow, speed).
adjective(quick, speed).
adjective(old, age).
adjective(young, age).
adjective(new, age).
adjective(ancient, age).
adjective(modern, age).
adjective(soft, texture).
adjective(hard, texture).
adjective(smooth, texture).
adjective(rough, texture).
adjective(sticky, texture).
adjective(crunchy, texture).
adjective(creamy, texture).
adjective(fluffy, texture).
adjective(sweet, taste).
adjective(sour, taste).
adjective(bitter, taste).
adjective(salty, taste).
adjective(spicy, taste).
adjective(tasty, taste).
adjective(delicious, taste).
adjective(bland, taste).
adjective(broken, state).
adjective(ripe, state).
adjective(fresh, state).
adjective(rotten, state).
adjective(clean, state).
adjective(dirty, state).
adjective(empty, state).
adjective(full, state).
adjective(open, state).
adjective(closed, state).
adjective(wet, state).
adjective(dry, state).
adjective(ready, state).
adjective(cooked, state).
adjective(raw, state).
adjective(burnt, state).
adjective(flat, state).
adjective(busy, state).
adjective(easy, difficulty).
adjective(difficult, difficulty).
adjective(simple, difficulty).
adjective(tricky, difficulty).
adjective(deep, depth).
adjective(shallow, depth).
adjective(long, length).
adjective(electric, power).
adjective(solar, power).
adjective(manual, power).
adjective(vegan, diet).
adjective(vegetarian, diet).
adjective(lazy, character).
adjective(kind, character).
adjective(friendly, character).
adjective(shy, character).
adjective(brave, character).
adjective(funny, character).
adjective(smart, character).
adjective(clever, character).
adjective(polite, character).
adjective(rude, character).
adjective(calm, character).
adjective(patient, character).
adjective(honest, character).
adjective(fun, appeal).
adjective(boring, appeal).
adjective(exciting, appeal).
adjective(interesting, appeal).
adjective(popular, appeal).
adjective(clear, clarity).
adjective(cloudy, clarity).
adjective(loud, loudness).
adjective(quiet, loudness).
adjective(noisy, loudness).
adjective(bright, brightness).
adjective(dark, brightness).
adjective(cheap, price).
adjective(expensive, price).
adjective(free, price).
adjective(healthy, health).
adjective(sick, health).
adjective(ill, health).
adjective(round, shape).
adjective(square, shape).
adjective(beautiful, beauty).
adjective(pretty, beauty).
adjective(ugly, beauty).
adjective(safe, safety).
adjective(dangerous, safety).
adjective(strong, strength).
adjective(weak, strength).
adjective(rich, wealth).
adjective(poor, wealth).
adjective(high, height).
adjective(low, height).
forbid(property, 2, quality, "a quality needs a named property (colour, size, taste, difficulty) or its own predicate").
```
