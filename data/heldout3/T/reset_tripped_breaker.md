---
id: reset_tripped_breaker
stage: tools
task: reset_tripped_breaker
schema: v3
tier: T
---
# Reset a tripped circuit breaker

## requests
- Half the sockets in my flat just went dead. How do I get the power back?
- How do I reset a tripped breaker?
- The lights went out in the kitchen when I switched on the toaster. What do I check?

## situation
- I have a torch, and the fuse box is in the hall cupboard.

## ground truth
```datalog
task(reset_tripped_breaker).
domain(reset_tripped_breaker, repair).
goal(reset_tripped_breaker, power_back).
needs(reset_tripped_breaker, flashlight).
takes_time(reset_tripped_breaker, 5, 15, minute).
step(reset_tripped_breaker, reset_tripped_breaker_s1).
action(reset_tripped_breaker_s1, unplug).
object_of(reset_tripped_breaker_s1, appliance).
purpose(reset_tripped_breaker_s1, fault_isolated).
step(reset_tripped_breaker, reset_tripped_breaker_s2).
action(reset_tripped_breaker_s2, find).
object_of(reset_tripped_breaker_s2, consumer_unit).
uses(reset_tripped_breaker_s2, flashlight).
role(reset_tripped_breaker_s2, flashlight, light).
step(reset_tripped_breaker, reset_tripped_breaker_s3).
action(reset_tripped_breaker_s3, find).
object_of(reset_tripped_breaker_s3, switch).
check(reset_tripped_breaker_s3, switch_down).
step(reset_tripped_breaker, reset_tripped_breaker_s4).
action(reset_tripped_breaker_s4, flip).
object_of(reset_tripped_breaker_s4, switch).
setting(reset_tripped_breaker_s4, position, on, level).
step(reset_tripped_breaker, reset_tripped_breaker_s5).
action(reset_tripped_breaker_s5, plug).
object_of(reset_tripped_breaker_s5, appliance).
condition(reset_tripped_breaker_s5, switch_stays_on).
check(reset_tripped_breaker_s5, no_trip).
before(reset_tripped_breaker_s1, reset_tripped_breaker_s2).
before(reset_tripped_breaker_s2, reset_tripped_breaker_s3).
before(reset_tripped_breaker_s3, reset_tripped_breaker_s4).
before(reset_tripped_breaker_s4, reset_tripped_breaker_s5).
```

## aliases
```datalog
task_alias(reset_tripped_breaker, "resetting a breaker").
task_alias(reset_tripped_breaker, "getting power back after a trip").
task_alias(reset_tripped_breaker, "fixing a tripped fuse").
alias(consumer_unit, "fuse box").
alias(consumer_unit, "breaker box").
alias(flashlight, "torch").
alias(switch, "tripped switch").
alias(switch_down, "the switch that is down").
alias(no_trip, "it does not trip again").
```

## teach
- To reset a tripped circuit breaker you need a torch.
- Resetting a tripped circuit breaker takes 5 to 15 minutes.
- Step 1 of resetting a tripped breaker: unplug the appliance that tripped it, to isolate the fault.
- Step 2 of resetting a tripped breaker: find the consumer unit using the torch.
- Step 3 of resetting a tripped breaker: find the tripped switch; it is the one that is down.
- Step 4 of resetting a tripped breaker: flip the switch to on.
- Step 5 of resetting a tripped breaker: if the switch stays on, plug the appliances back one at a time and check it does not trip again.

## checklist
- names what is needed: a torch
- gives the steps in order: unplug the appliance, find the fuse box, find the switch that is down, flip it on, plug back one at a time
- says to plug things back one at a time

## variant
- change: no torch: use the phone's light
- request: Power's out in half the house, it's dark and I can't find a torch. How do I get to the breaker?
- situation: I have my mobile phone; there is no torch in the house.
- expect: The reply lights the way to the consumer unit with the phone's flashlight instead of a torch.

```datalog situation
has(user, phone).
lacks(user, flashlight).
```

```datalog difference
swap(reset_tripped_breaker_s2, flashlight, phone).
extra_need(phone).
no_need(flashlight).
```

## variant
- change: consumer unit locked away: cannot reset, call the landlord
- request: The power tripped but the fuse box in my rented flat is in a locked cupboard and I've no key. What can I do?
- situation: I have a torch but no key to the cupboard where the fuse box is.
- expect: The reply says the breaker cannot be reset without reaching the consumer unit and advises contacting the landlord or building manager.

```datalog situation
has(user, flashlight).
lacks(user, cupboard_key).
```

```datalog difference
cannot(no_access_to_consumer_unit).
```
