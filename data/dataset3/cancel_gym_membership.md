---
id: cancel_gym_membership
stage: procedures
task: cancel_gym_membership
schema: v3
---
# Cancel a gym membership

## requests
- How do I cancel my gym membership? I want it done properly so they can't keep charging me.
- What's the process for ending a gym membership by post?
- Can you talk me through quitting my gym? I need to write to them.

## situation
- I have my gym membership contract, a pen, paper, an envelope and a stamp.
- I have about 30 minutes free this evening.

## ground truth
```datalog
task(cancel_gym_membership).
domain(cancel_gym_membership, errands).
goal(cancel_gym_membership, cancel_membership).
needs(cancel_gym_membership, membership_contract).
needs(cancel_gym_membership, pen).
needs(cancel_gym_membership, paper).
needs(cancel_gym_membership, envelope).
needs(cancel_gym_membership, stamp).
takes_time(cancel_gym_membership, 15, 30, minute).
step(cancel_gym_membership, cancel_gym_membership_s1).
action(cancel_gym_membership_s1, read).
object_of(cancel_gym_membership_s1, membership_contract).
purpose(cancel_gym_membership_s1, notice_period_known).
takes_time(cancel_gym_membership_s1, 5, 10, minute).
step(cancel_gym_membership, cancel_gym_membership_s2).
action(cancel_gym_membership_s2, write).
object_of(cancel_gym_membership_s2, cancellation_letter).
uses(cancel_gym_membership_s2, pen).
role(cancel_gym_membership_s2, pen, write).
uses(cancel_gym_membership_s2, paper).
role(cancel_gym_membership_s2, paper, hold).
takes_time(cancel_gym_membership_s2, 5, 10, minute).
step(cancel_gym_membership, cancel_gym_membership_s3).
action(cancel_gym_membership_s3, sign).
object_of(cancel_gym_membership_s3, cancellation_letter).
uses(cancel_gym_membership_s3, pen).
role(cancel_gym_membership_s3, pen, write).
takes_time(cancel_gym_membership_s3, 1, 2, minute).
step(cancel_gym_membership, cancel_gym_membership_s4).
action(cancel_gym_membership_s4, insert).
object_of(cancel_gym_membership_s4, cancellation_letter).
into(cancel_gym_membership_s4, envelope).
uses(cancel_gym_membership_s4, envelope).
role(cancel_gym_membership_s4, envelope, contain).
takes_time(cancel_gym_membership_s4, 1, 2, minute).
step(cancel_gym_membership, cancel_gym_membership_s5).
action(cancel_gym_membership_s5, post).
object_of(cancel_gym_membership_s5, envelope).
uses(cancel_gym_membership_s5, stamp).
role(cancel_gym_membership_s5, stamp, pay).
check(cancel_gym_membership_s5, proof_of_posting).
warning(cancel_gym_membership_s5, no_proof_of_posting).
takes_time(cancel_gym_membership_s5, 2, 3, minute).
before(cancel_gym_membership_s1, cancel_gym_membership_s2).
before(cancel_gym_membership_s2, cancel_gym_membership_s3).
before(cancel_gym_membership_s3, cancel_gym_membership_s4).
before(cancel_gym_membership_s4, cancel_gym_membership_s5).
```

## aliases
```datalog
task_alias(cancel_gym_membership, "cancel gym membership").
task_alias(cancel_gym_membership, "end my gym membership").
task_alias(cancel_gym_membership, "quit the gym").
alias(membership_contract, "gym contract").
alias(stamp, "postage stamp").
alias(pen, "biro").
alias(proof_of_posting, "proof of posting").
alias(notice_period_known, "notice period").
alias(no_proof_of_posting, "no proof of posting").
```

## teach
- To cancel a gym membership you need a membership contract, a pen, paper, an envelope and a stamp.
- Cancelling a gym membership takes 15 to 30 minutes.
- Step 1 of cancelling a gym membership: read the membership contract to find the notice period.
- Step 2 of cancelling a gym membership: write a cancellation letter on the paper with the pen.
- Step 3 of cancelling a gym membership: sign the cancellation letter with the pen.
- Step 4 of cancelling a gym membership: put the cancellation letter into the envelope.
- Step 5 of cancelling a gym membership: post the envelope with a stamp, using recorded delivery so you have proof it was sent.

## checklist
- names what is needed, including a membership contract, a pen, paper, an envelope and a stamp
- gives the steps in order: read the contract, write the letter, sign it, put it in the envelope, post it
- says to check the notice period in the contract before writing
- warns to use recorded delivery so there is proof the letter was posted
- gives a total time of about 15 to 30 minutes

## variant
- change: you only have a pencil, not a pen, so the letter is written and signed in pencil
- request: I only have a pencil, no pen. Can I still write the cancellation letter?
- situation: I have my membership contract, a pencil, paper, an envelope and a stamp.
- expect: The reply uses the pencil in place of the pen for writing and signing the letter, and keeps the other steps the same.

```datalog situation
has(user, membership_contract).
has(user, pencil).
has(user, paper).
has(user, envelope).
has(user, stamp).
lacks(user, pen).
```

```datalog difference
swap(cancel_gym_membership_s2, pen, pencil).
swap(cancel_gym_membership_s3, pen, pencil).
extra_need(pencil).
no_need(pen).
```

## variant
- change: you have no stamp, so you hand the sealed envelope in at the front desk instead of posting it
- request: I've run out of stamps and I'm at the gym anyway. Can I hand in the cancellation there instead?
- situation: I have my membership contract, a pen, paper and an envelope, I have no stamp, and I'm at the gym today.
- expect: The reply skips the posting step, has you hand the envelope in at the front desk instead, and drops the stamp from the list.

```datalog situation
has(user, membership_contract).
has(user, pen).
has(user, paper).
has(user, envelope).
lacks(user, stamp).
```

```datalog difference
drop(cancel_gym_membership_s5).
no_need(stamp).
add_step(cancel_gym_membership_s9, cancel_gym_membership_s4).
step(cancel_gym_membership, cancel_gym_membership_s9).
action(cancel_gym_membership_s9, hand_in).
object_of(cancel_gym_membership_s9, envelope).
into(cancel_gym_membership_s9, front_desk).
takes_time(cancel_gym_membership_s9, 5, 10, minute).
before(cancel_gym_membership_s4, cancel_gym_membership_s9).
change(cancel_gym_membership, takes_time, 15_30, 10_20).
```
