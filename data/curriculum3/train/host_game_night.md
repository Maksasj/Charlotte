---
id: host_game_night
stage: social
task: host_game_night
schema: v3
---
# Host a game night

## requests
- I want to have a few friends over for a board game night. How should I plan it?
- Can you help me organise a games evening at my place?
- What do I need to do to host a fun game night this Friday?

## situation
- I have a few board games, a big table and a shop nearby.
- I can spend about an hour and a half getting ready.

## ground truth
```datalog
task(host_game_night).
domain(host_game_night, planning).
goal(host_game_night, game_night).
needs(host_game_night, board_game).
needs(host_game_night, snack).
needs(host_game_night, drink).
needs(host_game_night, table).
takes_time(host_game_night, 60, 90, minute).
step(host_game_night, host_game_night_s1).
action(host_game_night_s1, choose).
object_of(host_game_night_s1, date).
check(host_game_night_s1, suits_everyone).
takes_time(host_game_night_s1, 5, 10, minute).
step(host_game_night, host_game_night_s2).
action(host_game_night_s2, invite).
object_of(host_game_night_s2, friend).
takes_time(host_game_night_s2, 10, 15, minute).
step(host_game_night, host_game_night_s3).
action(host_game_night_s3, choose).
object_of(host_game_night_s3, board_game).
check(host_game_night_s3, suits_group).
takes_time(host_game_night_s3, 5, 10, minute).
step(host_game_night, host_game_night_s4).
action(host_game_night_s4, buy).
object_of(host_game_night_s4, snack).
uses(host_game_night_s4, drink).
role(host_game_night_s4, drink, buy).
takes_time(host_game_night_s4, 30, 45, minute).
step(host_game_night, host_game_night_s5).
action(host_game_night_s5, clear).
object_of(host_game_night_s5, table).
step(host_game_night, host_game_night_s6).
action(host_game_night_s6, set).
object_of(host_game_night_s6, table).
uses(host_game_night_s6, board_game).
role(host_game_night_s6, board_game, set).
uses(host_game_night_s6, snack).
role(host_game_night_s6, snack, set).
uses(host_game_night_s6, drink).
role(host_game_night_s6, drink, set).
takes_time(host_game_night_s6, 10, 15, minute).
before(host_game_night_s1, host_game_night_s2).
before(host_game_night_s2, host_game_night_s3).
before(host_game_night_s3, host_game_night_s4).
before(host_game_night_s4, host_game_night_s5).
before(host_game_night_s5, host_game_night_s6).
```

## aliases
```datalog
task_alias(host_game_night, "hosting a game night").
task_alias(host_game_night, "board game night").
task_alias(host_game_night, "games evening").
alias(board_game, "board games").
alias(board_game, "games").
alias(friend, "friends").
alias(snack, "snacks").
alias(drink, "drinks").
alias(set, "set out").
alias(party_game, "party games").
alias(party_game, "charades").
alias(suits_everyone, "a date that suits everyone").
alias(suits_group, "suit the group").
```

## teach
- To host a game night you need board games, snacks, drinks and a table.
- Hosting a game night takes 60 to 90 minutes.
- Step 1 of hosting a game night: choose a date that suits everyone for 5 to 10 minutes.
- Step 2 of hosting a game night: invite four or five friends by message for 10 to 15 minutes.
- Step 3 of hosting a game night: choose two or three board games that suit the group for 5 to 10 minutes.
- Step 4 of hosting a game night: buy snacks and drinks for 30 to 45 minutes.
- Step 5 of hosting a game night: clear the table.
- Step 6 of hosting a game night: set out the games, the snacks and the drinks, for 10 to 15 minutes.

## checklist
- names what is needed: board games, snacks, drinks and a table
- gives the steps in order: choose date, invite friend, choose board game, buy snack, set table
- gives a total time of about 60 to 90 minutes
- closes politely

## variant
- change: big group of ten: pick party games like charades that everyone can play at once
- request: So it turns out ten people want to come to game night. Will board games still work?
- situation: Ten friends are coming. I have a few board games, a big living room and a shop nearby.
- expect: The reply swaps the board games for party games that suit ten people, such as charades, buys more snacks and drinks and keeps the invite and set-up steps.

```datalog situation
count(user, friend, 10).
has(user, board_game).
has(user, living_room).
has(user, shop).
```

```datalog difference
change(host_game_night_s3, object_of, board_game, party_game).
swap(host_game_night_s6, board_game, party_game).
change(host_game_night_s4, amount, usual, more).
extra_need(party_game).
no_need(board_game).
```
