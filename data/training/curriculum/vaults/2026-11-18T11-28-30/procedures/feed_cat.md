---
id: feed_cat
status: hypothesis
source: seed
uses: 0
successes: 0
---
# Feed the cat

Give the cat a clean bowl of food and fresh water.

```datalog
task(feed_cat).
needs(feed_cat, cat_food).
needs(feed_cat, food_bowl).
needs(feed_cat, water_bowl).
needs(feed_cat, water).
takes_time(feed_cat, 3, 5, minute).
step(feed_cat, feed_cat_s1).
action(feed_cat_s1, wash).
object_of(feed_cat_s1, food_bowl).
takes_time(feed_cat_s1, 1, 2, minute).
until(feed_cat_s1, clean).
step(feed_cat, feed_cat_s2).
action(feed_cat_s2, fill).
object_of(feed_cat_s2, food_bowl).
uses(feed_cat_s2, cat_food).
takes_time(feed_cat_s2, 1, 1, minute).
step(feed_cat, feed_cat_s3).
action(feed_cat_s3, fill).
object_of(feed_cat_s3, water_bowl).
uses(feed_cat_s3, water).
takes_time(feed_cat_s3, 1, 1, minute).
step(feed_cat, feed_cat_s4).
action(feed_cat_s4, place).
object_of(feed_cat_s4, food_bowl).
before(feed_cat_s1, feed_cat_s2).
before(feed_cat_s2, feed_cat_s3).
before(feed_cat_s3, feed_cat_s4).
```
