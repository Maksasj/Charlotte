---
id: draft_weekly_chore_rota
stage: procedures
task: draft_weekly_chore_rota
schema: v3
---
# Draft a weekly chore rota

## requests
- How do I draft a weekly chore rota for the house?
- Could you help me set up a rota that splits the household chores for the week?
- What's the easiest way to make a chore schedule for the week at home?

## situation
- I have a whiteboard and a marker at home.
- I've got about half an hour this evening.

## ground truth
```datalog
task(draft_weekly_chore_rota).
domain(draft_weekly_chore_rota, planning).
goal(draft_weekly_chore_rota, weekly_chores_planned).
needs(draft_weekly_chore_rota, whiteboard).
needs(draft_weekly_chore_rota, marker).
takes_time(draft_weekly_chore_rota, 20, 30, minute).
step(draft_weekly_chore_rota, draft_weekly_chore_rota_s1).
action(draft_weekly_chore_rota_s1, list).
object_of(draft_weekly_chore_rota_s1, chore_list).
uses(draft_weekly_chore_rota_s1, whiteboard).
role(draft_weekly_chore_rota_s1, whiteboard, hold).
step(draft_weekly_chore_rota, draft_weekly_chore_rota_s2).
action(draft_weekly_chore_rota_s2, assign).
object_of(draft_weekly_chore_rota_s2, chore).
takes_time(draft_weekly_chore_rota_s2, 5, 10, minute).
step(draft_weekly_chore_rota, draft_weekly_chore_rota_s3).
action(draft_weekly_chore_rota_s3, write).
object_of(draft_weekly_chore_rota_s3, rota).
uses(draft_weekly_chore_rota_s3, marker).
role(draft_weekly_chore_rota_s3, marker, mark).
uses(draft_weekly_chore_rota_s3, whiteboard).
role(draft_weekly_chore_rota_s3, whiteboard, hold).
onto(draft_weekly_chore_rota_s3, whiteboard).
step(draft_weekly_chore_rota, draft_weekly_chore_rota_s4).
action(draft_weekly_chore_rota_s4, check).
object_of(draft_weekly_chore_rota_s4, rota).
check(draft_weekly_chore_rota_s4, everyone_agrees).
before(draft_weekly_chore_rota_s1, draft_weekly_chore_rota_s2).
before(draft_weekly_chore_rota_s2, draft_weekly_chore_rota_s3).
before(draft_weekly_chore_rota_s3, draft_weekly_chore_rota_s4).
```

## aliases
```datalog
task_alias(draft_weekly_chore_rota, "weekly chore rota").
task_alias(draft_weekly_chore_rota, "chore rota").
task_alias(draft_weekly_chore_rota, "chore schedule").
alias(marker, "whiteboard marker").
alias(chore_list, "list of chores").
alias(everyone_agrees, "everyone agrees").
alias(phone, "mobile phone").
```

## teach
- To draft a weekly chore rota you need a whiteboard and a marker.
- Drafting a weekly chore rota takes 20 to 30 minutes.
- Step 1 of drafting a weekly chore rota: list the chores on the whiteboard.
- Step 2 of drafting a weekly chore rota: assign each chore to one person, which takes 5 to 10 minutes.
- Step 3 of drafting a weekly chore rota: write the rota onto the whiteboard with the marker.
- Step 4 of drafting a weekly chore rota: check that everyone agrees to it.

## checklist
- names what is needed, including a whiteboard and a marker
- gives the steps in order: list the chores, assign each chore, write the rota, check everyone agrees
- gives a total time of about 20 to 30 minutes
- says the rota is written on the whiteboard with the marker

## variant
- change: also send the finished rota to the family group chat from the phone
- request: Can you add a step for sharing the rota with the family from my phone?
- situation: I have a whiteboard, a marker and my phone, and I want to share the rota with everyone.
- expect: The reply gives the four usual steps and adds a final step to send the rota to the family group chat from the phone.

```datalog situation
has(user, whiteboard).
has(user, marker).
has(user, phone).
wants(user, share_rota).
```

```datalog difference
extra_need(phone).
add_step(draft_weekly_chore_rota_s9, draft_weekly_chore_rota_s4).
step(draft_weekly_chore_rota, draft_weekly_chore_rota_s9).
action(draft_weekly_chore_rota_s9, send).
object_of(draft_weekly_chore_rota_s9, rota).
uses(draft_weekly_chore_rota_s9, phone).
role(draft_weekly_chore_rota_s9, phone, hold).
takes_time(draft_weekly_chore_rota_s9, 2, 5, minute).
before(draft_weekly_chore_rota_s4, draft_weekly_chore_rota_s9).
```

## variant
- change: I have no marker, only a pencil, so the rota is written in pencil
- request: I don't have a marker, only a pencil. Can I still draft the rota on the whiteboard?
- situation: I have a whiteboard and a pencil, but no marker.
- expect: The reply gives the four usual steps and uses the pencil in place of the marker when writing the rota on the whiteboard.

```datalog situation
has(user, whiteboard).
has(user, pencil).
lacks(user, marker).
```

```datalog difference
swap(draft_weekly_chore_rota_s3, marker, pencil).
no_need(marker).
extra_need(pencil).
```
