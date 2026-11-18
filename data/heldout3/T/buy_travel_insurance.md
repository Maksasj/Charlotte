---
id: buy_travel_insurance
stage: social
task: buy_travel_insurance
schema: v3
tier: T
---
# Buy travel insurance

## requests
- Do I really need travel insurance for my holiday, and how do I get it?
- How should I choose and buy a travel insurance policy?
- My parents told me never to go abroad uninsured. How do I sort cover for my trip?

## situation
- I have a laptop, my bank card and my phone.

## ground truth
```datalog
task(buy_travel_insurance).
domain(buy_travel_insurance, travel).
goal(buy_travel_insurance, insured_trip).
needs(buy_travel_insurance, laptop).
needs(buy_travel_insurance, payment_card).
needs(buy_travel_insurance, phone).
takes_time(buy_travel_insurance, 30, 60, minute).
step(buy_travel_insurance, buy_travel_insurance_s1).
action(buy_travel_insurance_s1, list).
object_of(buy_travel_insurance_s1, trip_details).
step(buy_travel_insurance, buy_travel_insurance_s2).
action(buy_travel_insurance_s2, compare).
object_of(buy_travel_insurance_s2, policy).
uses(buy_travel_insurance_s2, laptop).
role(buy_travel_insurance_s2, laptop, inform).
step(buy_travel_insurance, buy_travel_insurance_s3).
action(buy_travel_insurance_s3, check).
object_of(buy_travel_insurance_s3, cover).
purpose(buy_travel_insurance_s3, medical_and_cancellation).
step(buy_travel_insurance, buy_travel_insurance_s4).
action(buy_travel_insurance_s4, buy).
object_of(buy_travel_insurance_s4, policy).
uses(buy_travel_insurance_s4, payment_card).
role(buy_travel_insurance_s4, payment_card, pay).
condition(buy_travel_insurance_s4, before_travel).
step(buy_travel_insurance, buy_travel_insurance_s5).
action(buy_travel_insurance_s5, save).
object_of(buy_travel_insurance_s5, policy_document).
uses(buy_travel_insurance_s5, phone).
role(buy_travel_insurance_s5, phone, record).
before(buy_travel_insurance_s1, buy_travel_insurance_s2).
before(buy_travel_insurance_s2, buy_travel_insurance_s3).
before(buy_travel_insurance_s3, buy_travel_insurance_s4).
before(buy_travel_insurance_s4, buy_travel_insurance_s5).
```

## aliases
```datalog
task_alias(buy_travel_insurance, "getting travel insurance").
task_alias(buy_travel_insurance, "holiday insurance").
task_alias(buy_travel_insurance, "insuring a trip").
alias(trip_details, "the dates, destination and travellers").
alias(policy, "policies").
alias(cover, "what is covered").
alias(policy_document, "the policy document").
alias(medical_and_cancellation, "medical costs and cancellation are included").
```

## teach
- To buy travel insurance you need a laptop, a payment card and a phone.
- Buying travel insurance takes 30 to 60 minutes.
- Step 1 of buying travel insurance: list the trip details: dates, destination and travellers.
- Step 2 of buying travel insurance: compare policies on the laptop.
- Step 3 of buying travel insurance: check the cover so medical costs and cancellation are included.
- Step 4 of buying travel insurance: buy the policy with your card before you travel.
- Step 5 of buying travel insurance: save the policy document on your phone.

## checklist
- names what is needed: a laptop, a card and a phone
- gives the steps in order: list details, compare, check cover, buy, save document
- says to buy it before travelling

## variant
- change: ski trip: add winter sports cover
- request: I'm going skiing in the Alps. Does my travel insurance need anything special?
- situation: I have a laptop, my card and my phone; the trip is a ski holiday.
- expect: The reply adds checking for and buying winter sports cover as an add-on to the policy.

```datalog situation
has(user, laptop).
has(user, payment_card).
has(user, phone).
wants(user, winter_sports_cover).
```

```datalog difference
add_step(buy_travel_insurance_s9, buy_travel_insurance_s3).
step(buy_travel_insurance, buy_travel_insurance_s9).
action(buy_travel_insurance_s9, add).
object_of(buy_travel_insurance_s9, winter_sports_cover).
before(buy_travel_insurance_s3, buy_travel_insurance_s9).
before(buy_travel_insurance_s9, buy_travel_insurance_s4).
```

## variant
- change: family of four: buy one family policy
- request: We're four going on holiday, two adults and two kids. Do we each need insurance?
- situation: I have a laptop, my card and my phone; four of us are travelling.
- expect: The reply compares and buys a single family policy covering all four instead of individual policies.

```datalog situation
has(user, laptop).
has(user, payment_card).
has(user, phone).
count(user, traveller, 4).
```

```datalog difference
change(buy_travel_insurance_s4, object_of, policy, family_policy).
```
