---
id: host_game_night__wants_pizza
stage: social
task: host_game_night
schema: v3
tier: S
base: host_game_night
---
# Host a game night

## requests
- Having mates over to play board games on Friday and we want pizza as well as snacks. How do I fit that into the prep?

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

## variant
- change: add ordering pizza after buying snacks and drinks
- request: Having mates over to play board games on Friday and we want pizza as well as snacks. How do I fit that into the prep?
- situation: I have a few board games, a big table, a shop nearby and a pizza place that delivers.
- expect: The reply keeps the date, invites, game choice and snack run, adds ordering pizza for the evening, then clears and sets the table.

```datalog situation
has(user, board_game).
has(user, table).
wants(user, pizza).
```

```datalog difference
extra_need(pizza).
add_step(host_game_night_s9, host_game_night_s4).
step(host_game_night, host_game_night_s9).
action(host_game_night_s9, order).
object_of(host_game_night_s9, pizza).
before(host_game_night_s4, host_game_night_s9).
before(host_game_night_s9, host_game_night_s6).
```
