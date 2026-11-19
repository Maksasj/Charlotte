---
id: renew_library_card
stage: procedures
task: renew_library_card
schema: v3
---
# Renew a library card

## requests
- My library card is about to expire. How do I renew it online?
- Can you tell me how to extend my library card from my computer?
- What's the process for renewing a library membership card on the website?

## situation
- I have my library card and a laptop.
- I want to renew it before I go to the library this week.

## ground truth
```datalog
task(renew_library_card).
domain(renew_library_card, errands).
goal(renew_library_card, card_renewed).
needs(renew_library_card, library_card).
needs(renew_library_card, computer).
takes_time(renew_library_card, 10, 15, minute).
step(renew_library_card, renew_library_card_s1).
action(renew_library_card_s1, open).
object_of(renew_library_card_s1, library_website).
uses(renew_library_card_s1, computer).
role(renew_library_card_s1, computer, hold).
step(renew_library_card, renew_library_card_s2).
action(renew_library_card_s2, enter).
object_of(renew_library_card_s2, card_number).
uses(renew_library_card_s2, library_card).
role(renew_library_card_s2, library_card, hold).
step(renew_library_card, renew_library_card_s3).
action(renew_library_card_s3, click).
object_of(renew_library_card_s3, renew_button).
uses(renew_library_card_s3, computer).
role(renew_library_card_s3, computer, hold).
step(renew_library_card, renew_library_card_s4).
action(renew_library_card_s4, confirm).
object_of(renew_library_card_s4, renewal).
check(renew_library_card_s4, renewal_confirmed).
before(renew_library_card_s1, renew_library_card_s2).
before(renew_library_card_s2, renew_library_card_s3).
before(renew_library_card_s3, renew_library_card_s4).
```

## aliases
```datalog
task_alias(renew_library_card, "renew library card").
task_alias(renew_library_card, "renew my library card").
task_alias(renew_library_card, "library membership card").
alias(library_card, "library card").
alias(computer, "laptop").
alias(library_website, "library website").
alias(card_number, "card number").
alias(renew_button, "renew button").
alias(renewal, "renewal").
alias(renewal_confirmed, "confirmation").
```

## teach
- To renew a library card you need a library card and a computer.
- Renewing a library card takes 10 to 15 minutes.
- Step 1 of renewing a library card: open the library website on the computer.
- Step 2 of renewing a library card: enter the card number from the library card.
- Step 3 of renewing a library card: click the renew button on the computer.
- Step 4 of renewing a library card: confirm the renewal and wait for the confirmation.

## checklist
- names what is needed, including a library card and a computer
- gives the steps in order: open the library website, enter the card number, click renew, confirm
- gives a total time of about 10 to 15 minutes
- says the renewal is done when the confirmation appears

## variant
- change: no computer, so use a phone instead
- request: I want to renew my library card but I only have my phone. Can I still do it?
- situation: I have my library card and a phone, and no computer.
- expect: The reply has the user open the library website on the phone instead of the computer, then enter the card number, tap renew and confirm.

```datalog situation
has(user, library_card).
has(user, phone).
lacks(user, computer).
```

```datalog difference
swap(renew_library_card_s1, computer, phone).
swap(renew_library_card_s3, computer, phone).
no_need(computer).
extra_need(phone).
```

## variant
- change: library card is lost, so use the card number written down instead
- request: I've lost my library card, but I wrote the number down. Can I renew with that?
- situation: I have a laptop and the card number written down, but no library card.
- expect: The reply has the user open the library website on the laptop, enter the written-down card number instead of the card, click renew and confirm.

```datalog situation
has(user, computer).
has(user, library_card_number).
lacks(user, library_card).
```

```datalog difference
swap(renew_library_card_s2, library_card, library_card_number).
no_need(library_card).
extra_need(library_card_number).
```
