---
id: switch_mobile_provider
stage: procedures
task: switch_mobile_provider
schema: v3
---
# Switch mobile provider

## requests
- I want to change my phone plan to a cheaper mobile provider. What do I need to do?
- How do I switch to a new mobile network without losing my number?
- Can you take me through moving my phone to a different provider step by step?

## situation
- I have a phone, a laptop and a new SIM card from my new provider.
- I'd like to lower my monthly bill and have about 45 minutes.

## ground truth
```datalog
task(switch_mobile_provider).
domain(switch_mobile_provider, errands).
goal(switch_mobile_provider, lower_bill).
needs(switch_mobile_provider, laptop).
needs(switch_mobile_provider, phone).
needs(switch_mobile_provider, new_sim).
takes_time(switch_mobile_provider, 30, 45, minute).
step(switch_mobile_provider, switch_mobile_provider_s1).
action(switch_mobile_provider_s1, compare).
object_of(switch_mobile_provider_s1, plans).
uses(switch_mobile_provider_s1, laptop).
role(switch_mobile_provider_s1, laptop, search).
purpose(switch_mobile_provider_s1, lower_bill).
takes_time(switch_mobile_provider_s1, 10, 15, minute).
step(switch_mobile_provider, switch_mobile_provider_s2).
action(switch_mobile_provider_s2, call).
object_of(switch_mobile_provider_s2, provider).
uses(switch_mobile_provider_s2, phone).
role(switch_mobile_provider_s2, phone, contact).
overlaps(switch_mobile_provider_s2, switch_mobile_provider_s1).
takes_time(switch_mobile_provider_s2, 5, 10, minute).
step(switch_mobile_provider, switch_mobile_provider_s3).
action(switch_mobile_provider_s3, insert).
object_of(switch_mobile_provider_s3, new_sim).
into(switch_mobile_provider_s3, phone).
uses(switch_mobile_provider_s3, new_sim).
role(switch_mobile_provider_s3, new_sim, fit).
warning(switch_mobile_provider_s3, pins_bent).
takes_time(switch_mobile_provider_s3, 2, 5, minute).
step(switch_mobile_provider, switch_mobile_provider_s4).
action(switch_mobile_provider_s4, restart).
object_of(switch_mobile_provider_s4, phone).
check(switch_mobile_provider_s4, network_active).
takes_time(switch_mobile_provider_s4, 2, 5, minute).
before(switch_mobile_provider_s2, switch_mobile_provider_s3).
before(switch_mobile_provider_s3, switch_mobile_provider_s4).
```

## aliases
```datalog
task_alias(switch_mobile_provider, "change mobile provider").
task_alias(switch_mobile_provider, "switch phone plan").
alias(laptop, "computer").
alias(new_sim, "new SIM card").
alias(provider, "mobile provider").
alias(lower_bill, "lower bill").
alias(pins_bent, "bent pins").
alias(network_active, "network is active").
alias(restart, "reboot").
```

## teach
- To switch mobile provider you need a laptop, a phone and a new SIM card.
- Switching mobile provider takes 30 to 45 minutes.
- Step 1 of switch mobile provider: compare the plans on the laptop to lower your bill, for 10 to 15 minutes.
- While step 1 runs, step 2 of switch mobile provider: call your provider on the phone to confirm the new plan, for 5 to 10 minutes.
- Step 3 of switch mobile provider: insert the new SIM card into the phone, for 2 to 5 minutes, taking care not to bend the pins.
- Step 4 of switch mobile provider: restart the phone for 2 to 5 minutes and check that the network is active.

## checklist
- names what is needed: a laptop, a phone and a new SIM card
- gives the steps in order: compare plans, call the provider, insert the SIM, restart the phone
- says the call can be made while the plans are being compared
- warns not to bend the SIM pins when inserting it
- gives a total time of about 30 to 45 minutes

## variant
- change: no laptop: compare the plans on the phone instead, which takes a little longer
- request: I want to change provider, but I only have my phone, not a laptop. How do I compare plans?
- situation: I have a phone and a new SIM card, but no laptop.
- expect: The reply compares the plans on the phone instead of the laptop, then calls the provider, inserts the new SIM and restarts the phone, in about 35 to 50 minutes.

```datalog situation
lacks(user, laptop).
has(user, phone).
has(user, new_sim).
```

```datalog difference
swap(switch_mobile_provider_s1, laptop, phone).
no_need(laptop).
change(switch_mobile_provider_s1, takes_time, 10_15, 15_20).
```

## variant
- change: only 20 minutes: skip comparing plans, call the provider for their best offer, then insert the SIM and restart
- request: I've only got 20 minutes. Can I still switch provider quickly?
- situation: I have a laptop, a phone and a new SIM card, and 20 minutes.
- expect: The reply drops the plan comparison, calls the provider to ask for their best offer, then inserts the new SIM and restarts the phone, in about 15 to 20 minutes.

```datalog situation
time_limit(user, 20, minute).
has(user, laptop).
has(user, phone).
has(user, new_sim).
```

```datalog difference
drop(switch_mobile_provider_s1).
no_need(laptop).
change(switch_mobile_provider, takes_time, 30_45, 15_20).
```
