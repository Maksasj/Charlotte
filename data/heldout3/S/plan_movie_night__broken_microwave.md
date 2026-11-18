---
id: plan_movie_night__broken_microwave
task: plan_movie_night
schema: v3
tier: S
base: plan_movie_night
---
# Plan a movie night

## requests
- Film night with my flatmates tonight, but the microwave has died and the only popcorn we've got is the microwave kind.

## ground truth
```datalog
task(plan_movie_night).
domain(plan_movie_night, planning).
goal(plan_movie_night, movie_night).
needs(plan_movie_night, tv).
needs(plan_movie_night, streaming_service).
needs(plan_movie_night, popcorn).
needs(plan_movie_night, microwave).
needs(plan_movie_night, blanket).
takes_time(plan_movie_night, 20, 30, minute).
step(plan_movie_night, plan_movie_night_s1).
action(plan_movie_night_s1, choose).
object_of(plan_movie_night_s1, movie).
uses(plan_movie_night_s1, streaming_service).
role(plan_movie_night_s1, streaming_service, access).
takes_time(plan_movie_night_s1, 10, 15, minute).
step(plan_movie_night, plan_movie_night_s2).
action(plan_movie_night_s2, make).
object_of(plan_movie_night_s2, popcorn).
uses(plan_movie_night_s2, microwave).
role(plan_movie_night_s2, microwave, heat).
takes_time(plan_movie_night_s2, 3, 4, minute).
step(plan_movie_night, plan_movie_night_s3).
action(plan_movie_night_s3, arrange).
object_of(plan_movie_night_s3, sofa).
uses(plan_movie_night_s3, blanket).
role(plan_movie_night_s3, blanket, cover).
takes_time(plan_movie_night_s3, 3, 5, minute).
step(plan_movie_night, plan_movie_night_s4).
action(plan_movie_night_s4, dim).
object_of(plan_movie_night_s4, light).
step(plan_movie_night, plan_movie_night_s5).
action(plan_movie_night_s5, start).
object_of(plan_movie_night_s5, movie).
uses(plan_movie_night_s5, tv).
role(plan_movie_night_s5, tv, display).
uses(plan_movie_night_s5, streaming_service).
role(plan_movie_night_s5, streaming_service, play).
takes_time(plan_movie_night_s5, 1, 1, minute).
before(plan_movie_night_s1, plan_movie_night_s2).
before(plan_movie_night_s2, plan_movie_night_s3).
before(plan_movie_night_s3, plan_movie_night_s4).
before(plan_movie_night_s4, plan_movie_night_s5).
```

## aliases
```datalog
task_alias(plan_movie_night, "planning a movie night").
task_alias(plan_movie_night, "movie night").
task_alias(plan_movie_night, "film night").
alias(movie, "film").
alias(tv, "TV").
alias(tv, "television").
alias(streaming_service, "streaming service").
alias(blanket, "blankets").
alias(sofa, "couch").
alias(light, "lights").
alias(dim, "turn down").
alias(choose, "pick").
alias(arrange, "set up").
```

## teach
- To plan a movie night you need a TV, a streaming service, popcorn, a microwave and blankets.
- Planning a movie night takes 20 to 30 minutes.
- Step 1 of planning a movie night: choose a movie on the streaming service for 10 to 15 minutes.
- Step 2 of planning a movie night: make the popcorn in the microwave for 3 to 4 minutes.
- Step 3 of planning a movie night: arrange the sofa with blankets for 3 to 5 minutes.
- Step 4 of planning a movie night: dim the lights.
- Step 5 of planning a movie night: start the movie on the TV with the streaming service for 1 minute.

## variant
- change: broken microwave: skip the popcorn
- request: Film night with my flatmates tonight, but the microwave has died and the only popcorn we've got is the microwave kind.
- situation: The microwave is broken. I have the TV, a streaming service, microwave popcorn bags and blankets.
- expect: The reply leaves out making popcorn and keeps choosing the film, setting up the sofa, dimming the lights and starting the movie.

```datalog situation
has(user, tv).
has(user, streaming_service).
has(user, popcorn).
has(user, blanket).
lacks(user, microwave).
```

```datalog difference
drop(plan_movie_night_s2).
no_need(microwave).
no_need(popcorn).
```
