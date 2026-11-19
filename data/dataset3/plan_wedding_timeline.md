---
id: plan_wedding_timeline
stage: procedures
task: plan_wedding_timeline
schema: v3
---
# Plan a wedding timeline

## requests
- How should I lay out the steps for planning a wedding, from setting the date to confirming the vendors?
- What order do I do the wedding planning jobs in? I want a timeline.
- Can you give me a schedule for organising a wedding?

## situation
- I have a calendar and a phone, and I'm planning a wedding for next autumn.
- I have a few months before the date.

## ground truth
```datalog
task(plan_wedding_timeline).
domain(plan_wedding_timeline, planning).
goal(plan_wedding_timeline, wedding_plan).
needs(plan_wedding_timeline, calendar).
needs(plan_wedding_timeline, phone).
takes_time(plan_wedding_timeline, 120, 180, minute).
step(plan_wedding_timeline, plan_wedding_timeline_s1).
action(plan_wedding_timeline_s1, set).
object_of(plan_wedding_timeline_s1, date).
uses(plan_wedding_timeline_s1, calendar).
role(plan_wedding_timeline_s1, calendar, hold).
step(plan_wedding_timeline, plan_wedding_timeline_s2).
action(plan_wedding_timeline_s2, book).
object_of(plan_wedding_timeline_s2, venue).
step(plan_wedding_timeline, plan_wedding_timeline_s3).
action(plan_wedding_timeline_s3, send).
object_of(plan_wedding_timeline_s3, invitation).
step(plan_wedding_timeline, plan_wedding_timeline_s4).
action(plan_wedding_timeline_s4, confirm).
object_of(plan_wedding_timeline_s4, guest_list).
overlaps(plan_wedding_timeline_s4, plan_wedding_timeline_s3).
step(plan_wedding_timeline, plan_wedding_timeline_s5).
action(plan_wedding_timeline_s5, confirm).
object_of(plan_wedding_timeline_s5, vendors).
before(plan_wedding_timeline_s1, plan_wedding_timeline_s2).
before(plan_wedding_timeline_s2, plan_wedding_timeline_s3).
before(plan_wedding_timeline_s4, plan_wedding_timeline_s5).
```

## aliases
```datalog
task_alias(plan_wedding_timeline, "wedding timeline").
task_alias(plan_wedding_timeline, "wedding schedule").
task_alias(plan_wedding_timeline, "planning a wedding").
alias(calendar, "wedding calendar").
alias(venue, "reception venue").
alias(invitation, "invites").
alias(guest_list, "guest list").
alias(vendors, "suppliers").
alias(wedding_plan, "wedding plan").
```

## teach
- To plan a wedding timeline you need a calendar and a phone.
- Planning a wedding timeline takes 120 to 180 minutes.
- Step 1 of plan a wedding timeline: set the wedding date on the calendar.
- Step 2 of plan a wedding timeline: book the reception venue.
- Step 3 of plan a wedding timeline: send the invitations six weeks before the date.
- While step 3 runs, step 4 of plan a wedding timeline: confirm the guest list.
- Step 5 of plan a wedding timeline: confirm the vendors two weeks before the date.

## checklist
- names what is needed, including a calendar and a phone
- gives the steps in order: set the date, book the venue, send invitations, confirm vendors
- says the guest list can be confirmed while the invitations go out
- gives a total time of about 120 to 180 minutes

## variant
- change: only an hour: skip the guest list check and send the invitations straight away
- request: I only have an hour this weekend to plan the wedding timeline. What's the quickest version?
- situation: I have a calendar and a phone, and one hour.
- expect: The reply sets the date, books the venue, sends invitations and confirms vendors, leaving out the guest list check, in about 60 to 90 minutes.

```datalog situation
has(user, calendar).
has(user, phone).
time_limit(user, 60, minute).
```

```datalog difference
drop(plan_wedding_timeline_s4).
change(plan_wedding_timeline, takes_time, 120_180, 60_90).
```

## variant
- change: they want a caterer too: book the caterer right after the venue, before the invitations go out
- request: Can you fit booking a caterer into the wedding timeline as well?
- situation: I have a calendar and a phone, and I want a caterer for the wedding.
- expect: The reply adds booking a caterer as a new step right after booking the venue and before sending invitations.

```datalog situation
has(user, calendar).
has(user, phone).
wants(user, caterer).
```

```datalog difference
add_step(plan_wedding_timeline_s9, plan_wedding_timeline_s2).
step(plan_wedding_timeline, plan_wedding_timeline_s9).
action(plan_wedding_timeline_s9, book).
object_of(plan_wedding_timeline_s9, caterer).
before(plan_wedding_timeline_s2, plan_wedding_timeline_s9).
before(plan_wedding_timeline_s9, plan_wedding_timeline_s3).
extra_need(caterer).
```
