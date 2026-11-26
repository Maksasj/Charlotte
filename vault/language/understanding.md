# Understanding pack (claude-opus-5.5, 2026-11-26)

Proposed by the vault-suggest skill from the bench failures of run
`2026-11-26_131448-moved` and applied after the user approved it. Every block is
lexicon facts or reply rules; nothing in `vault/core` changes.

## 1. Karin as the object of a sentence, not someone being addressed

"Karina is a Karin" lost its last word: "karin" after "a" was stripped as if
the user were calling her. Articles, "my" and "are" before her name mean she is
the object.

```karin
vocative_block(word="a", side="before")
vocative_block(word="an", side="before")
vocative_block(word="the", side="before")
vocative_block(word="my", side="before")
vocative_block(word="are", side="before")
```

## 2. Rewrites that ran in the wrong order or were missing

```karin
rewrite(from="can you're", to="you are")
rewrite(from="what is you", to="what are you")
rewrite(from="who is you", to="who are you")
rewrite(from="karin is you", to="you are karin")
rewrite(from="what i just said", to="what did i just say")
rewrite(from="what did i just said", to="what did i just say")
rewrite(from="you add", to="add")
```

## 3. How the microphone hears "Karin" at the start of a sentence

Only stripped at the start or end, and never before "is / was / likes …"
(so "Ken is my friend" stays a statement about Ken).

```karin
vocative(text="ken")
vocative(text="kelly")
vocative(text="aaron")
```

## 4. Relations without "my": "Oscar is best friend"

```karin
relation_word(word="friend")
relation_word(word="best friend")
relation_word(word="girlfriend")
relation_word(word="boyfriend")
relation_word(word="wife")
relation_word(word="husband")
relation_word(word="brother")
relation_word(word="sister")
relation_word(word="mom")
relation_word(word="mother")
relation_word(word="dad")
relation_word(word="father")
relation_word(word="son")
relation_word(word="daughter")
relation_word(word="cousin")
relation_word(word="colleague")
relation_word(word="boss")
relation_word(word="teacher")
relation_word(word="neighbor")

```

## 5. "Oscar like ice cream" — only for people Karin already knows

The general "$x like $y" read "Aaron I like rock music" as Aaron's taste, so it
is limited to someone stored as one of the user's relations.

```karin
```

## 6. Karin as herself

```karin
rule self-you-are-me
when: "you are $x", persona_name(value=$x), persona_name(value=$me)
then: => "Yes - I'm $me."

rule self-call-karin
when: "call karin"
then: => "I'm here - what can I do for you?"

rule self-are-you-a-relation
when: "are you a $rel", relation_word(word=$rel)
then: => "I'd like to think so - I'm always here when you need me."

rule self-are-you-my-relation
when: "are you my $rel", relation_word(word=$rel)
then: => "I'd like to think so - I'm always here when you need me."
```

## 7. Key words inside long or noisy sentences (lowest priority)

These only win when nothing more specific matched.

```karin
rule social-insult-anywhere
when: ~"stupid", !intent()
trust: 0.1
then: => "Sorry - I only understand the phrasings I've been taught, so I miss a lot. Tell me what you meant and I'll learn it."

rule social-dumb-anywhere
when: ~"dumb", !intent()
trust: 0.1
then: => "Sorry - I only understand the phrasings I've been taught, so I miss a lot. Tell me what you meant and I'll learn it."

rule social-repeat-anywhere
when: ~"repeat", said(by="karin", text=$t), !intent()
trust: 0.1
then: => "I said: \"$t\""

rule social-new-chat-anywhere
when: ~"new chat", !intent()
trust: 0.1
then: => "I can't start a new chat myself yet - use the New chat button."

rule social-app-anywhere
when: ~"app", !intent()
trust: 0.1
then: => "I can't open or control apps yet."
```

## 8. Honest answers to requests Karin can't do

```karin
rule social-how-many-do-you
when: "how many $x do you $y"
then: => "I can't count what I know yet - the Knowledge tab shows all my facts and rules."

rule social-how-many-do-we
when: "how many $x do we $y"
then: => "I can't count our messages yet - they're all in the chat on the left."

rule social-learn-more
when: "learn $x"
then: => "I learn from what you tell me: facts like \"Oscar is my friend\" or \"black tea is a drink\", and replies you teach me when I'm stuck."
```

## 9. Questions that expire

A question Karin asked lives for one more turn only. If the user moves on, the
next unrelated sentence is not taken as the answer (it used to save "my dad just
texted me" as a note two turns after "what should the note say?").

```karin
chat_expiry(rel="dialogue_slot", turns="1")
chat_expiry(rel="dialogue_pending_name", turns="1")
chat_expiry(rel="dialogue_confirm", turns="1")
```
