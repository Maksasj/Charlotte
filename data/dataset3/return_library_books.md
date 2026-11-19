---
id: return_library_books
stage: procedures
task: return_library_books
schema: v3
---
# Return library books

## requests
- I have some library books due back today. How do I get them returned?
- What's the process for dropping off library books at the library?
- Can you tell me how to return my library books, step by step?

## situation
- I have a few library books and a tote bag.
- The library is a short walk from home.

## ground truth
```datalog
task(return_library_books).
domain(return_library_books, errands).
goal(return_library_books, returned_books).
needs(return_library_books, book).
needs(return_library_books, tote_bag).
takes_time(return_library_books, 15, 25, minute).
step(return_library_books, return_library_books_s1).
action(return_library_books_s1, pack).
object_of(return_library_books_s1, book).
uses(return_library_books_s1, tote_bag).
role(return_library_books_s1, tote_bag, contain).
into(return_library_books_s1, tote_bag).
takes_time(return_library_books_s1, 5, 10, minute).
step(return_library_books, return_library_books_s2).
action(return_library_books_s2, walk).
object_of(return_library_books_s2, library).
takes_time(return_library_books_s2, 10, 15, minute).
step(return_library_books, return_library_books_s3).
action(return_library_books_s3, drop).
object_of(return_library_books_s3, book).
into(return_library_books_s3, return_slot).
purpose(return_library_books_s3, books_counted_as_returned).
step(return_library_books, return_library_books_s4).
action(return_library_books_s4, keep).
object_of(return_library_books_s4, receipt).
purpose(return_library_books_s4, proof_of_return).
before(return_library_books_s1, return_library_books_s2).
before(return_library_books_s2, return_library_books_s3).
before(return_library_books_s3, return_library_books_s4).
```

## aliases
```datalog
task_alias(return_library_books, "returning library books").
task_alias(return_library_books, "return library books").
alias(book, "library book").
alias(tote_bag, "tote bag").
alias(return_slot, "book drop").
alias(library, "the library").
alias(receipt, "receipt").
alias(proof_of_return, "proof of return").
alias(books_counted_as_returned, "counted as returned").
```

## teach
- To return library books you need books and a tote bag.
- Returning library books takes 15 to 25 minutes.
- Step 1 of returning library books: pack the books into the tote bag, which takes 5 to 10 minutes.
- Step 2 of returning library books: walk to the library, which takes 10 to 15 minutes.
- Step 3 of returning library books: drop the books into the book drop so they are counted as returned.
- Step 4 of returning library books: keep the receipt as proof of return.

## checklist
- names what is needed, including books and a tote bag
- gives the steps in order: pack the books, walk to the library, drop them in the book drop, keep the receipt
- gives a total time of about 15 to 25 minutes
- says the receipt is kept as proof that the books were returned

## variant
- change: no tote bag, so the books go into a plastic bag instead
- request: I don't have a tote bag. Can I still return my library books?
- situation: I have some library books and a plastic bag, but no tote bag.
- expect: The reply says to put the books into the plastic bag instead of the tote bag, then walk to the library, drop them in the book drop and keep the receipt.

```datalog situation
has(user, book).
has(user, plastic_bag).
lacks(user, tote_bag).
```

```datalog difference
swap(return_library_books_s1, tote_bag, plastic_bag).
no_need(tote_bag).
extra_need(plastic_bag).
```

## variant
- change: only 10 minutes, so the receipt step is dropped
- request: I only have ten minutes before I leave. What's the quickest way to get these library books returned?
- situation: I have some library books and a tote bag, and I have about ten minutes.
- expect: The reply says to pack the books, walk to the library and drop them in the book drop, skipping the receipt, which takes about 8 to 12 minutes.

```datalog situation
has(user, book).
has(user, tote_bag).
time_limit(user, 10, minute).
```

```datalog difference
drop(return_library_books_s4).
change(return_library_books, takes_time, 15_25, 8_12).
```
