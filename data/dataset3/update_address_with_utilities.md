---
id: update_address_with_utilities
stage: procedures
task: update_address_with_utilities
schema: v3
---
# Update your address with utilities

## requests
- I'm moving house next week. What's the right order to change my address with the gas, water and electric companies?
- How do I update my address with my utility providers when I move?
- Walk me through telling the utilities about my new address so nothing gets cut off.

## situation
- I have my utility bills, my phone and a folder to keep papers in.
- I have the new address and about 40 minutes free.

## ground truth
```datalog
task(update_address_with_utilities).
domain(update_address_with_utilities, errands).
goal(update_address_with_utilities, address_changed).
needs(update_address_with_utilities, utility_bills).
needs(update_address_with_utilities, phone).
needs(update_address_with_utilities, folder).
takes_time(update_address_with_utilities, 30, 40, minute).
step(update_address_with_utilities, update_address_with_utilities_s1).
action(update_address_with_utilities_s1, gather).
object_of(update_address_with_utilities_s1, account_number).
from(update_address_with_utilities_s1, utility_bills).
step(update_address_with_utilities, update_address_with_utilities_s2).
action(update_address_with_utilities_s2, call).
object_of(update_address_with_utilities_s2, utility_company).
uses(update_address_with_utilities_s2, phone).
role(update_address_with_utilities_s2, phone, hold).
step(update_address_with_utilities, update_address_with_utilities_s3).
action(update_address_with_utilities_s3, give).
object_of(update_address_with_utilities_s3, new_address).
step(update_address_with_utilities, update_address_with_utilities_s4).
action(update_address_with_utilities_s4, confirm).
object_of(update_address_with_utilities_s4, move_in_date).
optional(update_address_with_utilities_s4).
step(update_address_with_utilities, update_address_with_utilities_s5).
action(update_address_with_utilities_s5, keep).
object_of(update_address_with_utilities_s5, confirmation).
uses(update_address_with_utilities_s5, folder).
role(update_address_with_utilities_s5, folder, contain).
into(update_address_with_utilities_s5, folder).
before(update_address_with_utilities_s1, update_address_with_utilities_s2).
before(update_address_with_utilities_s2, update_address_with_utilities_s3).
before(update_address_with_utilities_s3, update_address_with_utilities_s4).
before(update_address_with_utilities_s4, update_address_with_utilities_s5).
```

## aliases
```datalog
task_alias(update_address_with_utilities, "moving utilities").
task_alias(update_address_with_utilities, "change utility address").
task_alias(update_address_with_utilities, "updating utilities after a move").
alias(account_number, "account number").
alias(utility_bills, "utility bills").
alias(utility_company, "utility company").
alias(move_in_date, "moving date").
alias(confirmation, "confirmation number").
alias(new_address, "new address").
alias(computer, "laptop").
alias(address_changed, "address changed").
```

## teach
- To update your address with utilities you need your utility bills, a phone and a folder.
- Updating your address with utilities takes 30 to 40 minutes.
- Step 1 of updating your address with utilities: gather the account numbers from your utility bills.
- Step 2 of updating your address with utilities: call each utility company on the phone.
- Step 3 of updating your address with utilities: give them your new address.
- Step 4 of updating your address with utilities: confirm the moving date, which you can skip if you are short on time.
- Step 5 of updating your address with utilities: keep the confirmation number in the folder.

## checklist
- names the utility bills, a phone and a folder as needed
- gives the steps in order: gather account numbers, call each company, give the new address, confirm the moving date, keep the confirmation number
- says the moving-date confirmation is optional
- gives a total time of about 30 to 40 minutes

## variant
- change: the phone is broken, so the user changes the address through each company's website on a computer instead of calling
- request: My phone is broken and I'm moving. Can I update my utilities from my computer instead?
- situation: I have my utility bills, a folder, a computer and the new address, but my phone is broken.
- expect: The reply keeps the same order and gathers the account numbers, then submits the new address on each company's website from the computer instead of calling, and keeps the confirmation number in the folder.

```datalog situation
lacks(user, phone).
has(user, computer).
has(user, utility_bills).
has(user, folder).
```

```datalog difference
extra_need(computer).
no_need(phone).
swap(update_address_with_utilities_s2, phone, computer).
change(update_address_with_utilities_s2, action, call, submit).
```

- change: only 20 minutes, so the optional moving-date confirmation is dropped and the total time is cut
- request: I'm short on time and need to update my utilities in 20 minutes. What's the quick way?
- situation: I have my phone, my utility bills and a folder, and only 20 minutes.
- expect: The reply gives the account numbers, the calls with the new address and the confirmation to keep, in order, and says to skip the moving-date confirmation to fit in 20 minutes.

```datalog situation
has(user, phone).
has(user, utility_bills).
has(user, folder).
time_limit(user, 20, minute).
```

```datalog difference
drop(update_address_with_utilities_s4).
change(update_address_with_utilities, takes_time, 30_40, 15_20).
```
