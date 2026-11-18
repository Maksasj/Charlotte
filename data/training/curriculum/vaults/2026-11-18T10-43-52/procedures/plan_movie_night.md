---
id: plan_movie_night
status: hypothesis
source: seed
uses: 0
successes: 0
---
# Plan a movie night

Pick a film, make popcorn, set up the sofa and start the movie.

```datalog
task(plan_movie_night).
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
takes_time(plan_movie_night_s1, 10, 15, minute).
step(plan_movie_night, plan_movie_night_s2).
action(plan_movie_night_s2, make).
object_of(plan_movie_night_s2, popcorn).
uses(plan_movie_night_s2, microwave).
takes_time(plan_movie_night_s2, 3, 4, minute).
step(plan_movie_night, plan_movie_night_s3).
action(plan_movie_night_s3, arrange).
object_of(plan_movie_night_s3, sofa).
uses(plan_movie_night_s3, blanket).
takes_time(plan_movie_night_s3, 3, 5, minute).
step(plan_movie_night, plan_movie_night_s4).
action(plan_movie_night_s4, dim).
object_of(plan_movie_night_s4, light).
step(plan_movie_night, plan_movie_night_s5).
action(plan_movie_night_s5, start).
object_of(plan_movie_night_s5, movie).
uses(plan_movie_night_s5, tv).
uses(plan_movie_night_s5, streaming_service).
takes_time(plan_movie_night_s5, 1, 1, minute).
before(plan_movie_night_s1, plan_movie_night_s2).
before(plan_movie_night_s2, plan_movie_night_s3).
before(plan_movie_night_s3, plan_movie_night_s4).
before(plan_movie_night_s4, plan_movie_night_s5).
```
