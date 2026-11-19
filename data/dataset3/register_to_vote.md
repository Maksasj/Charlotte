---
id: register_to_vote
stage: procedures
task: register_to_vote
schema: v3
---
# Register to vote

## requests
- How do I register to vote? I've never done it before.
- What do I need to do to get on the electoral roll by post?
- Walk me through sending in my voter registration form.

## situation
- I have my ID card, a pen, an envelope and a stamp.
- The registration form is already printed out.

## ground truth
```datalog
task(register_to_vote).
domain(register_to_vote, errands).
goal(register_to_vote, registered_voter).
needs(register_to_vote, id_card).
needs(register_to_vote, registration_form).
needs(register_to_vote, pen).
needs(register_to_vote, envelope).
needs(register_to_vote, postage_stamp).
takes_time(register_to_vote, 10, 15, minute).
step(register_to_vote, register_to_vote_s1).
action(register_to_vote_s1, gather).
object_of(register_to_vote_s1, id_card).
step(register_to_vote, register_to_vote_s2).
action(register_to_vote_s2, fill).
object_of(register_to_vote_s2, registration_form).
uses(register_to_vote_s2, pen).
role(register_to_vote_s2, pen, write).
purpose(register_to_vote_s2, correct_details).
step(register_to_vote, register_to_vote_s3).
action(register_to_vote_s3, sign).
object_of(register_to_vote_s3, registration_form).
uses(register_to_vote_s3, pen).
role(register_to_vote_s3, pen, write).
step(register_to_vote, register_to_vote_s4).
action(register_to_vote_s4, put).
object_of(register_to_vote_s4, registration_form).
into(register_to_vote_s4, envelope).
uses(register_to_vote_s4, envelope).
role(register_to_vote_s4, envelope, contain).
step(register_to_vote, register_to_vote_s5).
action(register_to_vote_s5, stamp).
object_of(register_to_vote_s5, envelope).
uses(register_to_vote_s5, postage_stamp).
role(register_to_vote_s5, postage_stamp, fasten).
step(register_to_vote, register_to_vote_s6).
action(register_to_vote_s6, post).
object_of(register_to_vote_s6, envelope).
before(register_to_vote_s1, register_to_vote_s2).
before(register_to_vote_s2, register_to_vote_s3).
before(register_to_vote_s3, register_to_vote_s4).
before(register_to_vote_s4, register_to_vote_s5).
before(register_to_vote_s5, register_to_vote_s6).
```

## aliases
```datalog
task_alias(register_to_vote, "register to vote").
task_alias(register_to_vote, "voter registration").
task_alias(register_to_vote, "get on the electoral roll").
alias(id_card, "ID card").
alias(id_card, "photo ID").
alias(registration_form, "voter registration form").
alias(registration_form, "form").
alias(postage_stamp, "stamp").
alias(envelope, "envelope").
alias(correct_details, "details are correct").
```

## teach
- To register to vote you need an ID card, a registration form, a pen, an envelope and a postage stamp.
- Registering to vote takes 10 to 15 minutes.
- Step 1 of register to vote: gather your ID card.
- Step 2 of register to vote: fill in the registration form with a pen so your details are correct.
- Step 3 of register to vote: sign the registration form with a pen.
- Step 4 of register to vote: put the registration form into the envelope.
- Step 5 of register to vote: stamp the envelope with a postage stamp.
- Step 6 of register to vote: post the envelope.

## checklist
- names what is needed, including an ID card, a registration form, a pen, an envelope and a stamp
- gives the steps in order: gather ID, fill in form, sign form, put form in envelope, stamp envelope, post envelope
- says the form must be filled in and signed with the pen before it goes in the envelope
- gives a total time of about 10 to 15 minutes

## variant
- change: no pen, so a pencil is used to fill in and sign the form
- request: I want to register to vote but I only have a pencil, not a pen. Can I still do it?
- situation: I have my ID card, a pencil, an envelope, a stamp and a printed registration form.
- expect: The reply says to fill in and sign the form with the pencil in place of the pen, then put it in the envelope, stamp it and post it.

```datalog situation
lacks(user, pen).
has(user, pencil).
has(user, id_card).
has(user, envelope).
has(user, postage_stamp).
has(user, registration_form).
```

```datalog difference
swap(register_to_vote_s2, pen, pencil).
swap(register_to_vote_s3, pen, pencil).
no_need(pen).
extra_need(pencil).
```

- change: no stamp, so the envelope cannot be posted
- request: I've filled in my voter form but I don't have a stamp and can't get to a shop today. Can I still post it?
- situation: I have my ID card, a pen, an envelope and a filled-in registration form, but no stamp.
- expect: The reply says the envelope cannot be posted without a stamp and suggests buying one first, or registering online or in person instead.

```datalog situation
lacks(user, postage_stamp).
has(user, id_card).
has(user, pen).
has(user, envelope).
has(user, registration_form).
```

```datalog difference
cannot(no_postage_stamp).
```
