# Wrappers: requests inside other words (claude-opus-5.5, 2026-11-27)

People wrap what they want in other words: an apology ("sorry, what's the
weather in Tokyo"), a polite question ("do you know who made Rust"), a
greeting ("hey, who am I"), or an aside ("it's Max, I'm back"). A `wrap` says
which words those are; `$act` is the request inside, which Karin reads as a
sentence of its own (and that can be wrapped again: "sorry Karin, can you
tell me ...").

Karin reads the sentence both ways - as a whole, and as the wrapped request -
and keeps the reading whose rules matched more of the words. So a wrapper never
takes over a sentence that a rule already understands as a whole; it only helps
when the whole reading explains less (the apology rule matches just "sorry").

`lead` is said before the inner request's reply.

Wrappers are matched on the canonical sentence: contractions are already
expanded ("i'm" -> "i am"), "karen" is already "karin", and start fillers
("so", "oh", "okay", "can you") are already gone.

## Apologies

```karin
wrap(text="sorry $act", lead="No worries.")
wrap(text="i am sorry $act", lead="No worries.")
wrap(text="sorry about that $act", lead="No worries.")
wrap(text="my bad $act", lead="No worries.")
wrap(text="oops $act")
wrap(text="excuse me $act")
```

## Asking politely, or asking whether Karin knows

```karin
wrap(text="do you know $act")
wrap(text="do you happen to know $act")
wrap(text="tell me $act")
wrap(text="i want to know $act")
wrap(text="i would like to know $act")
wrap(text="i was wondering $act")
wrap(text="i wonder $act")
wrap(text="any idea $act")
wrap(text="do you have any idea $act")
wrap(text="do you remember $act")
```

## Greetings and agreement in front of a request

```karin
wrap(text="hey $act")
wrap(text="hi $act", lead="Hi!")
wrap(text="hello $act", lead="Hello!")
wrap(text="yes $act")
wrap(text="right $act")
wrap(text="alright $act")
```

## Asides after a request

```karin
wrap(text="$act by the way")
wrap(text="$act to be honest")
wrap(text="$act i am back")
wrap(text="$act i am here")
```

## Embedded questions

Inside "do you know ...", a question keeps statement word order ("do you know
who I am"); these put it back in question order.

```karin
rewrite(from="who i am", to="who am i")
rewrite(from="what my name is", to="what is my name")
rewrite(from="where i live", to="where do i live")
rewrite(from="where i work", to="where do i work")
rewrite(from="who you are", to="who are you")
rewrite(from="what your name is", to="what is your name")
rewrite(from="what you can do", to="what can you do")
rewrite(from="what time it is", to="what time is it")
rewrite(from="who my friends are", to="who are my friends")
```
