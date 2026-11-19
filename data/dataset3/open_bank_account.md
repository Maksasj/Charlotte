---
id: open_bank_account
stage: procedures
task: open_bank_account
schema: v3
---
# Open a bank account

## requests
- I want to open a bank account at my local branch. What do I need to bring and what do I do there?
- How do you open a current account? Walk me through the steps.
- Could you explain the process for setting up a new bank account in person?

## situation
- I'm going to my local bank branch to open a new account.
- I have my passport, a proof of address letter and a pen.

## ground truth
```datalog
task(open_bank_account).
domain(open_bank_account, errands).
goal(open_bank_account, bank_account).
needs(open_bank_account, passport).
needs(open_bank_account, proof_of_address).
needs(open_bank_account, pen).
needs(open_bank_account, application_form).
takes_time(open_bank_account, 30, 45, minute).
step(open_bank_account, open_bank_account_s1).
action(open_bank_account_s1, fill).
object_of(open_bank_account_s1, application_form).
uses(open_bank_account_s1, pen).
role(open_bank_account_s1, pen, write).
takes_time(open_bank_account_s1, 10, 15, minute).
step(open_bank_account, open_bank_account_s2).
action(open_bank_account_s2, present).
object_of(open_bank_account_s2, passport).
uses(open_bank_account_s2, passport).
role(open_bank_account_s2, passport, hold).
purpose(open_bank_account_s2, confirm_identity).
step(open_bank_account, open_bank_account_s3).
action(open_bank_account_s3, present).
object_of(open_bank_account_s3, proof_of_address).
uses(open_bank_account_s3, proof_of_address).
role(open_bank_account_s3, proof_of_address, hold).
step(open_bank_account, open_bank_account_s4).
action(open_bank_account_s4, sign).
object_of(open_bank_account_s4, application_form).
uses(open_bank_account_s4, pen).
role(open_bank_account_s4, pen, write).
step(open_bank_account, open_bank_account_s5).
action(open_bank_account_s5, submit).
object_of(open_bank_account_s5, application_form).
before(open_bank_account_s1, open_bank_account_s2).
before(open_bank_account_s2, open_bank_account_s3).
before(open_bank_account_s3, open_bank_account_s4).
before(open_bank_account_s4, open_bank_account_s5).
```

## aliases
```datalog
task_alias(open_bank_account, "open a bank account").
task_alias(open_bank_account, "set up a bank account").
task_alias(open_bank_account, "open a current account").
alias(application_form, "form").
alias(application_form, "application").
alias(proof_of_address, "proof of address letter").
alias(passport, "passport").
alias(driving_licence, "driving licence").
alias(present, "show").
alias(confirm_identity, "confirm who you are").
```

## teach
- To open a bank account you need a passport, a proof of address, a pen and an application form.
- Opening a bank account takes 30 to 45 minutes.
- Step 1 of opening a bank account: fill in the application form with a pen. This takes 10 to 15 minutes.
- Step 2 of opening a bank account: show your passport so they can confirm who you are.
- Step 3 of opening a bank account: show your proof of address.
- Step 4 of opening a bank account: sign the application form with the pen.
- Step 5 of opening a bank account: submit the application form at the branch.

## checklist
- names what is needed, including a passport, a proof of address, a pen and an application form
- gives the steps in order: fill in the form, show passport, show proof of address, sign, submit
- says the passport is shown so the bank can confirm identity
- gives a total time of about 30 to 45 minutes

## variant
- change: no passport, so the driving licence is shown instead
- request: I don't have my passport, only my driving licence. Can I still open a bank account?
- situation: I have a driving licence, a proof of address letter, a pen and the form, but no passport.
- expect: The reply shows the driving licence in place of the passport when confirming identity and keeps the other steps the same.

```datalog situation
has(user, driving_licence).
has(user, proof_of_address).
has(user, pen).
has(user, application_form).
lacks(user, passport).
```

```datalog difference
swap(open_bank_account_s2, passport, driving_licence).
no_need(passport).
extra_need(driving_licence).
```

- change: joint account for two people, so the partner also shows a passport
- request: My partner and I want to open a joint bank account. What do we need to do differently?
- situation: We both have passports, a proof of address letter, a pen and the form.
- expect: The reply adds a step where the second applicant shows their passport, placed after the proof of address step, and keeps the other steps the same.

```datalog situation
count(user, applicant, 2).
has(user, passport).
has(user, proof_of_address).
has(user, pen).
has(user, application_form).
```

```datalog difference
add_step(open_bank_account_s9, open_bank_account_s3).
step(open_bank_account, open_bank_account_s9).
action(open_bank_account_s9, present).
object_of(open_bank_account_s9, passport).
uses(open_bank_account_s9, passport).
role(open_bank_account_s9, passport, hold).
purpose(open_bank_account_s9, confirm_identity).
before(open_bank_account_s3, open_bank_account_s9).
before(open_bank_account_s9, open_bank_account_s4).
```
