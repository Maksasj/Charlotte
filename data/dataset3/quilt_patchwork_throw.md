---
id: quilt_patchwork_throw
stage: procedures
task: quilt_patchwork_throw
schema: v3
---
# Make a patchwork throw

## requests
- Can you show me how to make a patchwork throw from fabric scraps?
- What's the process for a quilted patchwork blanket for the sofa?
- I want to make a small throw out of patches. What steps do I follow?

## situation
- I have fabric, scissors, a needle, thread, pins, batting, backing fabric and an iron.
- I have a free weekend afternoon.

## ground truth
```datalog
task(quilt_patchwork_throw).
domain(quilt_patchwork_throw, crafts).
goal(quilt_patchwork_throw, patchwork_throw).
needs(quilt_patchwork_throw, fabric).
needs(quilt_patchwork_throw, scissors).
needs(quilt_patchwork_throw, needle).
needs(quilt_patchwork_throw, thread).
needs(quilt_patchwork_throw, pins).
needs(quilt_patchwork_throw, batting).
needs(quilt_patchwork_throw, backing_fabric).
needs(quilt_patchwork_throw, iron).
takes_time(quilt_patchwork_throw, 150, 200, minute).
step(quilt_patchwork_throw, quilt_patchwork_throw_s1).
action(quilt_patchwork_throw_s1, cut).
object_of(quilt_patchwork_throw_s1, fabric).
uses(quilt_patchwork_throw_s1, scissors).
role(quilt_patchwork_throw_s1, scissors, cut).
takes_time(quilt_patchwork_throw_s1, 30, 30, minute).
step(quilt_patchwork_throw, quilt_patchwork_throw_s2).
action(quilt_patchwork_throw_s2, sew).
object_of(quilt_patchwork_throw_s2, squares).
uses(quilt_patchwork_throw_s2, needle).
role(quilt_patchwork_throw_s2, needle, pierce).
uses(quilt_patchwork_throw_s2, thread).
role(quilt_patchwork_throw_s2, thread, fasten).
takes_time(quilt_patchwork_throw_s2, 60, 60, minute).
step(quilt_patchwork_throw, quilt_patchwork_throw_s3).
action(quilt_patchwork_throw_s3, press).
object_of(quilt_patchwork_throw_s3, seams).
uses(quilt_patchwork_throw_s3, iron).
role(quilt_patchwork_throw_s3, iron, heat).
setting(quilt_patchwork_throw_s3, temperature, 150, celsius).
purpose(quilt_patchwork_throw_s3, seams_lie_flat).
takes_time(quilt_patchwork_throw_s3, 15, 20, minute).
step(quilt_patchwork_throw, quilt_patchwork_throw_s4).
action(quilt_patchwork_throw_s4, layer).
object_of(quilt_patchwork_throw_s4, layers).
uses(quilt_patchwork_throw_s4, backing_fabric).
role(quilt_patchwork_throw_s4, backing_fabric, hold).
uses(quilt_patchwork_throw_s4, batting).
role(quilt_patchwork_throw_s4, batting, cover).
uses(quilt_patchwork_throw_s4, pins).
role(quilt_patchwork_throw_s4, pins, hold).
takes_time(quilt_patchwork_throw_s4, 20, 20, minute).
step(quilt_patchwork_throw, quilt_patchwork_throw_s5).
action(quilt_patchwork_throw_s5, stitch).
object_of(quilt_patchwork_throw_s5, layers).
uses(quilt_patchwork_throw_s5, needle).
role(quilt_patchwork_throw_s5, needle, pierce).
uses(quilt_patchwork_throw_s5, thread).
role(quilt_patchwork_throw_s5, thread, fasten).
check(quilt_patchwork_throw_s5, layers_joined).
takes_time(quilt_patchwork_throw_s5, 60, 70, minute).
before(quilt_patchwork_throw_s1, quilt_patchwork_throw_s2).
before(quilt_patchwork_throw_s2, quilt_patchwork_throw_s3).
before(quilt_patchwork_throw_s3, quilt_patchwork_throw_s4).
before(quilt_patchwork_throw_s4, quilt_patchwork_throw_s5).
```

## aliases
```datalog
task_alias(quilt_patchwork_throw, "patchwork throw").
task_alias(quilt_patchwork_throw, "patchwork blanket").
task_alias(quilt_patchwork_throw, "quilt throw").
alias(batting, "wadding").
alias(backing_fabric, "backing").
alias(layers, "quilt sandwich").
alias(seams, "seam allowances").
alias(seams_lie_flat, "seams lie flat").
alias(layers_joined, "layers joined").
alias(squares, "fabric squares").
```

## teach
- To make a patchwork throw you need fabric, scissors, a needle, thread, pins, batting, backing fabric and an iron.
- A patchwork throw takes 150 to 200 minutes.
- Step 1 of making a patchwork throw: cut the fabric into squares with the scissors, which takes 30 minutes.
- Step 2 of making a patchwork throw: sew the squares into rows with the needle and thread, which takes 60 minutes.
- Step 3 of making a patchwork throw: press the seams flat with the iron at 150 degrees, for 15 to 20 minutes, so the seams lie flat.
- Step 4 of making a patchwork throw: layer the backing fabric, then the batting, then the pieced top, and pin them together, which takes 20 minutes.
- Step 5 of making a patchwork throw: stitch the layers together with the needle and thread until the layers are joined, which takes 60 to 70 minutes.

## checklist
- names what is needed, including fabric, scissors, a needle, thread, pins, batting, backing fabric and an iron
- gives the five steps in order: cut, sew the rows, press the seams, layer and pin, stitch the layers
- says the seams are pressed flat with the iron at 150 degrees
- gives a total time of about 150 to 200 minutes

## variant
- change: no iron: press the seams under a heavy book instead, which takes longer
- request: I don't own an iron. Can I still make the patchwork throw?
- situation: I have fabric, scissors, a needle, thread, pins, batting, backing fabric and a heavy book, but no iron.
- expect: The reply presses the seams flat under a heavy book for about 30 to 40 minutes instead of ironing, and keeps the other four steps as they are.

```datalog situation
has(user, fabric).
has(user, scissors).
has(user, needle).
has(user, thread).
has(user, pins).
has(user, batting).
has(user, backing_fabric).
has(user, heavy_book).
lacks(user, iron).
```

```datalog difference
swap(quilt_patchwork_throw_s3, iron, heavy_book).
change(quilt_patchwork_throw_s3, takes_time, 15_20, 30_40).
no_need(iron).
extra_need(heavy_book).
```

## variant
- change: add a bias-bound edge: one step after stitching that binds the edge with bias binding tape
- request: I'd like the throw to have a neat bound edge. Can you add that to the steps?
- situation: I have fabric, scissors, a needle, thread, pins, batting, backing fabric and bias binding tape, and I want the edges finished with it.
- expect: The reply keeps the five base steps and adds one step after stitching: pin the bias binding around the edge and sew it down, which takes 20 to 30 minutes.

```datalog situation
has(user, fabric).
has(user, needle).
has(user, thread).
has(user, binding_tape).
wants(user, bound_edge).
```

```datalog difference
add_step(quilt_patchwork_throw_s9, quilt_patchwork_throw_s5).
step(quilt_patchwork_throw, quilt_patchwork_throw_s9).
action(quilt_patchwork_throw_s9, bind).
object_of(quilt_patchwork_throw_s9, edge).
uses(quilt_patchwork_throw_s9, binding_tape).
role(quilt_patchwork_throw_s9, binding_tape, cover).
takes_time(quilt_patchwork_throw_s9, 20, 30, minute).
before(quilt_patchwork_throw_s5, quilt_patchwork_throw_s9).
extra_need(binding_tape).
```
