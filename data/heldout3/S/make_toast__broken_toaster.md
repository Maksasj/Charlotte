---
id: make_toast__broken_toaster
task: make_toast
schema: v3
tier: S
base: make_toast
---
# Make toast

## requests
- My toaster's dead. Is there another way to get some buttered toast for breakfast?

## ground truth
```datalog
task(make_toast).
domain(make_toast, cooking).
goal(make_toast, plan_meal).
needs(make_toast, bread).
needs(make_toast, butter).
needs(make_toast, toaster).
needs(make_toast, knife).
takes_time(make_toast, 3, 5, minute).
step(make_toast, make_toast_s1).
action(make_toast_s1, toast).
object_of(make_toast_s1, bread).
uses(make_toast_s1, toaster).
role(make_toast_s1, toaster, heat).
takes_time(make_toast_s1, 2, 3, minute).
until(make_toast_s1, golden).
step(make_toast, make_toast_s2).
action(make_toast_s2, spread).
object_of(make_toast_s2, butter).
uses(make_toast_s2, knife).
role(make_toast_s2, knife, spread).
takes_time(make_toast_s2, 1, 1, minute).
onto(make_toast_s2, toast).
step(make_toast, make_toast_s3).
action(make_toast_s3, serve).
object_of(make_toast_s3, toast).
before(make_toast_s1, make_toast_s2).
before(make_toast_s2, make_toast_s3).
```

## aliases
```datalog
task_alias(make_toast, "making toast").
task_alias(make_toast, "buttered toast").
task_alias(make_toast, "toast some bread").
alias(bread, "slice of bread").
alias(golden, "golden brown").
alias(spread, "butter").
alias(toast, "toasted bread").
```

## teach
- To make toast you need bread, butter, a toaster and a knife.
- Making toast takes 3 to 5 minutes.
- Step 1 of making toast: toast the bread in the toaster until golden, for 2 to 3 minutes.
- Step 2 of making toast: spread the butter onto the toast with the knife for 1 minute.
- Step 3 of making toast: serve the toast.

## variant
- change: broken toaster: toast the bread in a dry pan on the hob
- request: My toaster's dead. Is there another way to get some buttered toast for breakfast?
- situation: The toaster is broken. I have bread, butter, a frying pan and a knife.
- expect: The reply toasts the bread in the pan until golden instead of the toaster, then butters and serves it.

```datalog situation
has(user, bread).
has(user, butter).
has(user, pan).
has(user, knife).
lacks(user, toaster).
```

```datalog difference
swap(make_toast_s1, toaster, pan).
no_need(toaster).
extra_need(pan).
```
