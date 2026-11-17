# Hand labels for the checklist judges

For each encounter, read the checklist and the conversation, and write `pass`
if the assistant's replies do every check correctly (wording and small omissions
of detail do not matter, wrong facts do), otherwise `fail`. A note after the
label is welcome. Then run `sla_eval calibrate`.

## 1. water_plant (repeat)

Checklist:
- says to check the soil is dry before watering
- gives the steps in order: check soil, fill can, pour until it drains, empty saucer
- warns against leaving the plant standing in water
- closes politely

Conversation:
> **User:** How should I water a potted plant so I don't drown it?
>
> **Assistant:** Here is how to water plant: You need water and watering can. / 1. Check the soil with finger. / 2. Fill the watering can with water (1 minute). / 3. Pour the water with watering can (1 to 2 minutes). / 4. Empty the saucer. It takes 3 to 5 minutes in all.

> **User:** Thanks!
>
> **Assistant:** Glad I could help.

label: 

## 2. do_laundry (retention)

Checklist:
- says to sort lights from darks before washing
- gives the steps in order: sort, load, add detergent, run the machine, hang to dry
- gives a total time of about 75 to 115 minutes, most of it the machine cycle
- closes politely

Conversation:
> **User:** How do I wash my clothes in the washing machine without ruining them?
>
> **Assistant:** Here is how to do laundry: You need detergent, drying rack, laundry and washing machine. / 1. Sort the laundry (3 to 5 minutes). / 2. Load the laundry with washing machine (2 to 3 minutes). / 3. Add the detergent (1 minute). / 4. Run the washing machine (60 to 90 minutes). / 5. Hang the laundry with drying rack (10 to 15 minutes). It takes 75 to 115 minutes in all.

> **User:** Thanks!
>
> **Assistant:** Glad I could help.

label: 

## 3. replace_lightbulb (teach_retry)

Checklist:
- says to switch the light off and let the old bulb cool first
- gives the steps in order: switch off, wait, climb, unscrew, screw in, test
- gives a total time of about 5 to 10 minutes
- closes politely

Conversation:
> **User:** The ceiling light in my kitchen just went out. How do I change the bulb safely?
>
> **Assistant:** Here is how to replace lightbulb: You need lightbulb and step ladder. / 1. Switch off the light. / 2. Wait the old bulb until it is cool (3 to 5 minutes). / 3. Climb the step ladder. / 4. Unscrew the old bulb. / 5. Test the light. It takes 5 to 10 minutes in all. I didn't understand "The ceiling light in my kitchen just went out": what do you mean?

> **User:** I have a new bulb of the same type and a step ladder.
>
> **Assistant:** Okay.

> **User:** Thanks!
>
> **Assistant:** Glad I could help.

label: 

## 4. make_pancake (repeat)

Checklist:
- names what is needed, including flour, egg, milk and a pan
- gives the steps in order: mix batter, melt butter, pour, cook until bubbly, flip
- gives a total time of about 10 to 15 minutes
- closes politely

Conversation:
> **User:** Can you give me a simple way to make pancakes in a frying pan?
>
> **Assistant:** Here is how to make pancake: You need bowl, butter, egg, flour, frying pan, milk, spatula and whisk. / 1. Mix the batter with bowl, egg, flour, milk and whisk (2 to 3 minutes). / 2. Melt the butter with pan (1 to 2 minutes). / 3. Pour the batter with pan. / 4. Cook the batter until it is bubble appear (2 to 3 minutes). / 5. Flip the pancakes with spatula (1 to 2 minutes). It takes 10 to 15 minutes in all.

> **User:** Thanks!
>
> **Assistant:** Glad I could help.

label: 

## 5. plan_birthday_party (teach_retry)

Checklist:
- gives the steps in order: choose date, guest list, invitations, cake, balloons, decorate
- says to send the invitations well ahead of the day
- mentions a cake and decorations
- closes politely

Conversation:
> **User:** My daughter turns eight next month and I want to throw her a party at home. How do I plan it?
>
> **Assistant:** So your daughter's age is 8, right? Here is how to plan birthday party: You need balloon, cake and invitation. / 1. Choose the date (10 minutes). / 2. Write the guest list (15 to 20 minutes). / 3. Send the invitation (20 to 30 minutes). / 4. Order the cake (15 to 20 minutes). / 5. Buy the balloon (30 to 45 minutes). / 6. Decorate the room with balloon (30 to 55 minutes). It takes 120 to 180 minutes in all.

> **User:** Thanks!
>
> **Assistant:** Glad I could help.

label: 

## 6. replace_lightbulb (retention)

Checklist:
- says to switch the light off and let the old bulb cool first
- gives the steps in order: switch off, wait, climb, unscrew, screw in, test
- gives a total time of about 5 to 10 minutes
- closes politely

Conversation:
> **User:** A bulb blew in my hallway. What steps should I follow to replace it?
>
> **Assistant:** Here is how to replace lightbulb: You need lightbulb and step ladder. / 1. Switch off the light. / 2. Wait the old bulb until it is cool (3 to 5 minutes). / 3. Climb the step ladder. / 4. Unscrew the old bulb. / 5. Test the light. It takes 5 to 10 minutes in all.

> **User:** Thanks!
>
> **Assistant:** Glad I could help.

label: 

## 7. clean_window (teach_retry)

Checklist:
- names what is needed, including soapy water, a sponge and a squeegee
- gives the steps in order: mix soapy water, wash, squeegee dry, wipe frame
- gives a total time of about 10 to 15 minutes
- closes politely

Conversation:
> **User:** My windows are covered in streaks and dust. How do I get them properly clean?
>
> **Assistant:** Here is how to clean window: You need bucket, cloth, dish soap, sponge, squeegee and water. / 1. Mix the dish soap with bucket and water (1 to 2 minutes). / 2. Wash the windows with sponge (3 to 5 minutes). / 3. Dry the windows with squeegee (2 to 3 minutes). / 4. Wipe the edge with cloth (2 to 3 minutes). It takes 10 to 15 minutes in all.

> **User:** Thanks!
>
> **Assistant:** Glad I could help.

label: 

## 8. fix_flat_tire (spaced_repeat)

Checklist:
- names what is needed: tire levers, a patch kit and a pump
- gives the steps in order: remove wheel, remove tube, find hole, patch, refit tube, pump up
- gives a total time of about 20 to 30 minutes
- closes politely

Conversation:
> **User:** I got a puncture on my bicycle. Can you walk me through patching the tube?
>
> **Assistant:** Here is how to change bike tire: You need inner tube, patch kit, pump, tire lever and wrench. / 1. Remove the wheel with wrench (3 to 5 minutes). / 2. Remove the tire with tire lever (3 to 5 minutes). / 3. Find the hole with pump (3 to 5 minutes). / 4. Fit the hole with patch kit and tire lever (3 to 5 minutes). / 5. Fit the tire with pump and tire lever until it is firm (3 to 5 minutes). / 6. Mount the tire with pump and wrench until it is firm (3 to 5 minutes). It takes 20 to 40 minutes in all.

> **User:** Thanks!
>
> **Assistant:** Glad I could help.

label: 

## 9. plan_birthday_party (retention)

Checklist:
- gives the steps in order: choose date, guest list, invitations, cake, balloons, decorate
- says to send the invitations well ahead of the day
- mentions a cake and decorations
- closes politely

Conversation:
> **User:** I've never planned a birthday party before. What do I need to do and in what order?
>
> **Assistant:** Here is how to plan birthday party: You need balloon, cake and invitation. / 1. Choose the date (10 minutes). / 2. Write the guest list (15 to 20 minutes). / 3. Send the invitation (20 to 30 minutes). / 4. Order the cake (15 to 20 minutes). / 5. Buy the balloon (30 to 45 minutes). / 6. Decorate the room with balloon (30 to 55 minutes). It takes 120 to 180 minutes in all. I didn't understand "I've never planned a birthday party before": what do you mean?

> **User:** I want to have it at home with about ten guests. I can order a cake from the bakery and buy balloons.
>
> **Assistant:** Okay.

> **User:** Thanks!
>
> **Assistant:** Glad I could help.

label: 

## 10. iron_shirt (teach_retry)

Checklist:
- names what is needed, including an iron and an ironing board
- gives the steps in order: fill with water, heat, collar, sleeves, body, hang
- gives a total time of about 10 to 15 minutes
- closes politely

Conversation:
> **User:** I have a job interview tomorrow and my shirt is all creased. How do I iron it?
>
> **Assistant:** Here is how to iron shirt: You need hanger, iron, ironing board, shirt and water. / 1. Fill the iron with water. / 2. Heat the iron with cotton setting (2 to 3 minutes). / 3. Iron the collar with ironing board (1 to 2 minutes). / 4. Iron the sleeve with ironing board (3 to 4 minutes). / 5. Iron the shirt with ironing board (3 to 5 minutes). / 6. Hang the shirt with hanger. It takes 10 to 15 minutes in all.

> **User:** Thanks!
>
> **Assistant:** Glad I could help.

label: 

## 11. hang_picture (repeat)

Checklist:
- names what is needed, including a hammer, a nail and the picture
- gives the steps in order: measure, mark, hammer nail, hang, straighten
- gives a total time of about 10 to 15 minutes
- closes politely

Conversation:
> **User:** Can you tell me how to hang a picture on the wall so it's straight?
>
> **Assistant:** Here is how to hang picture: You need hammer, nail, pencil, picture and tape measure. / 1. Measure the wall with tape measure (3 to 5 minutes). / 2. Mark the spot with pencil. / 3. Hammer the nail (1 to 2 minutes). / 4. Hang the picture with nail. / 5. Straighten the picture until it is level. It takes 10 to 15 minutes in all.

> **User:** Thanks!
>
> **Assistant:** Glad I could help.

label: 

## 12. make_pancake (retention)

Checklist:
- names what is needed, including flour, egg, milk and a pan
- gives the steps in order: mix batter, melt butter, pour, cook until bubbly, flip
- gives a total time of about 10 to 15 minutes
- closes politely

Conversation:
> **User:** I feel like pancakes for breakfast. What do I need and how do I cook them?
>
> **Assistant:** For make pancake you need bowl, butter, egg, flour, frying pan, milk, pan, spatula and whisk.

> **User:** Thanks!
>
> **Assistant:** Glad I could help.

label: 

## 13. do_laundry (teach_retry)

Checklist:
- says to sort lights from darks before washing
- gives the steps in order: sort, load, add detergent, run the machine, hang to dry
- gives a total time of about 75 to 115 minutes, most of it the machine cycle
- closes politely

Conversation:
> **User:** I just moved out and have never done my own laundry. How does it work?
>
> **Assistant:** Here is how to do laundry: You need detergent, drying rack, laundry and washing machine. / 1. Sort the laundry (3 to 5 minutes). / 2. Load the laundry with washing machine (2 to 3 minutes). / 3. Add the detergent (1 minute). / 4. Run the washing machine (60 to 90 minutes). / 5. Hang the laundry with drying rack (10 to 15 minutes). It takes 75 to 115 minutes in all. I didn't understand "I just moved out and have never done my own laundry": what do you mean?

> **User:** I have a washing machine, laundry detergent, a drying rack and a full basket of clothes.
>
> **Assistant:** Okay.

> **User:** Thanks!
>
> **Assistant:** Glad I could help.

label: 

## 14. replace_lightbulb (spaced_repeat)

Checklist:
- says to switch the light off and let the old bulb cool first
- gives the steps in order: switch off, wait, climb, unscrew, screw in, test
- gives a total time of about 5 to 10 minutes
- closes politely

Conversation:
> **User:** Can you tell me the right way to swap a dead lightbulb for a new one?
>
> **Assistant:** Here is how to replace lightbulb: You need lightbulb and step ladder. / 1. Switch off the light. / 2. Wait the old bulb until it is cool (3 to 5 minutes). / 3. Climb the step ladder. / 4. Unscrew the old bulb. / 5. Test the light. It takes 5 to 10 minutes in all.

> **User:** Thanks!
>
> **Assistant:** Glad I could help.

label: 

## 15. replace_lightbulb (repeat)

Checklist:
- says to switch the light off and let the old bulb cool first
- gives the steps in order: switch off, wait, climb, unscrew, screw in, test
- gives a total time of about 5 to 10 minutes
- closes politely

Conversation:
> **User:** Can you tell me the right way to swap a dead lightbulb for a new one?
>
> **Assistant:** Here is how to replace lightbulb: You need lightbulb and step ladder. / 1. Switch off the light. / 2. Wait the old bulb until it is cool (3 to 5 minutes). / 3. Climb the step ladder. / 4. Unscrew the old bulb. / 5. Test the light. It takes 5 to 10 minutes in all.

> **User:** Thanks!
>
> **Assistant:** Glad I could help.

label: 

## 16. write_thank_you_note (retention)

Checklist:
- gives the parts in order: greeting, thanks for the gift, how you will use it, signature
- suggests naming the gift specifically
- mentions sealing the envelope and adding a stamp
- closes politely

Conversation:
> **User:** I want to send a handwritten thank-you to my neighbour for watering my plants. What should I write?
>
> **Assistant:** Okay, noted. Here is how to write note: You need card, envelope, pen and stamp. / 1. Write the greeting with card and pen. / 2. Thank the gift. / 3. Describe the gift. / 4. Sign the card with pen. / 5. Seal the envelope with stamp. It takes 10 to 15 minutes in all.

> **User:** Thanks!
>
> **Assistant:** Glad I could help.

label: 

## 17. plan_birthday_party (spaced_repeat)

Checklist:
- gives the steps in order: choose date, guest list, invitations, cake, balloons, decorate
- says to send the invitations well ahead of the day
- mentions a cake and decorations
- closes politely

Conversation:
> **User:** Can you help me organise a small birthday party for my best friend?
>
> **Assistant:** Here is how to plan birthday party: You need balloon, cake and invitation. / 1. Choose the date (10 minutes). / 2. Write the guest list (15 to 20 minutes). / 3. Send the invitation (20 to 30 minutes). / 4. Order the cake (15 to 20 minutes). / 5. Buy the balloon (30 to 45 minutes). / 6. Decorate the room with balloon (30 to 55 minutes). It takes 120 to 180 minutes in all.

> **User:** Thanks!
>
> **Assistant:** Glad I could help.

label: 

## 18. cook_rice (teach_retry)

Checklist:
- names what is needed: rice, water, a pot and a sieve
- gives the steps in order: boil water, rinse rice (while the water heats), add rice, simmer, rest
- gives a total time of about 25 to 30 minutes
- closes politely

Conversation:
> **User:** I need to cook some white rice to go with dinner. How do I do it on the stove?
>
> **Assistant:** Here is how to cook rice: You need pot, rice, sieve and water. / 1. Boil the water with pot (5 to 6 minutes). / 2. While you boil the water, rinse the rice with sieve until it is clear (1 to 2 minutes). / 3. Add the rice with water. / 4. Simmer the rice with lid (15 to 18 minutes). / 5. Rest the rice (5 to 6 minutes). It takes 25 to 30 minutes in all.

> **User:** Thanks!
>
> **Assistant:** Glad I could help.

label: 

## 19. water_plant (spaced_repeat)

Checklist:
- says to check the soil is dry before watering
- gives the steps in order: check soil, fill can, pour until it drains, empty saucer
- warns against leaving the plant standing in water
- closes politely

Conversation:
> **User:** How should I water a potted plant so I don't drown it?
>
> **Assistant:** Here is how to water plant: You need water and watering can. / 1. Check the soil with finger. / 2. Fill the watering can with water (1 minute). / 3. Pour the water with watering can (1 to 2 minutes). / 4. Empty the saucer. It takes 3 to 5 minutes in all.

> **User:** Thanks!
>
> **Assistant:** Glad I could help.

label: 

## 20. do_laundry (spaced_repeat)

Checklist:
- says to sort lights from darks before washing
- gives the steps in order: sort, load, add detergent, run the machine, hang to dry
- gives a total time of about 75 to 115 minutes, most of it the machine cycle
- closes politely

Conversation:
> **User:** My laundry basket is overflowing. Can you walk me through washing a load?
>
> **Assistant:** Here is how to do laundry: You need detergent, drying rack, laundry and washing machine. / 1. Sort the laundry (3 to 5 minutes). / 2. Load the laundry with washing machine (2 to 3 minutes). / 3. Add the detergent (1 minute). / 4. Run the washing machine (60 to 90 minutes). / 5. Hang the laundry with drying rack (10 to 15 minutes). It takes 75 to 115 minutes in all.

> **User:** Thanks!
>
> **Assistant:** Glad I could help.

label: 
