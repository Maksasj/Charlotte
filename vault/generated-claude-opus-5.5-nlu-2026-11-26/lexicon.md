# Lexicon (`claude-opus-5.5`, 2026-11-26)

Read by the engine's language layer before any pattern is matched. Order of
processing: `alias` → `rewrite` → strip `filler` / `vocative` → `rewrite` again.
Phrases are whole lowercase words; the longest phrase wins.

## Chat memory

These working relations live for the whole chat (replaces the old hard-coded
`dialogue_` prefix rule in Rust).

```karin
chat_memory(prefix="dialogue_")
chat_memory(rel="asked_slot")
chat_memory(rel="mention")
```

## Names and speech-recognition fixes

How the microphone heard "Karin" and other words in the 2026-11-25 voice chats.

```karin
alias(from="karen", to="karin")
alias(from="karina", to="karin")
alias(from="caren", to="karin")
alias(from="carin", to="karin")
alias(from="karine", to="karin")
alias(from="vilnus", to="Vilnius")
alias(from="hii", to="hi")
alias(from="hiya", to="hi")
alias(from="symbolic artificial intelligence", to="symbolic ai")
alias(from="a to-do list", to="a note")
alias(from="a to-do item", to="a note")
alias(from="a to-do", to="a note")
alias(from="a todo", to="a note")
alias(from="favourite", to="favorite")
```

## Contractions

```karin
rewrite(from="what's", to="what is")
rewrite(from="whats", to="what is")
rewrite(from="who's", to="who is")
rewrite(from="where's", to="where is")
rewrite(from="how's", to="how is")
rewrite(from="it's", to="it is")
rewrite(from="that's", to="that is")
rewrite(from="i'm", to="i am")
rewrite(from="im", to="i am")
rewrite(from="you're", to="you are")
rewrite(from="youre", to="you are")
rewrite(from="they're", to="they are")
rewrite(from="we're", to="we are")
rewrite(from="he's", to="he is")
rewrite(from="she's", to="she is")
rewrite(from="isn't", to="is not")
rewrite(from="aren't", to="are not")
rewrite(from="don't", to="do not")
rewrite(from="doesn't", to="does not")
rewrite(from="can't", to="cannot")
rewrite(from="won't", to="will not")
rewrite(from="i've", to="i have")
rewrite(from="i'd", to="i would")
rewrite(from="i'll", to="i will")
rewrite(from="let's", to="let us")
```

## Talking about Karin in the third person

"Karen is smart" means "you are smart"; "who is Karen" means "who are you".

```karin
rewrite(from="karin is your name", to="your name is karin")
rewrite(from="karin is a very", to="you are")
rewrite(from="karin is very", to="you are")
rewrite(from="karin is a my", to="you are my")
rewrite(from="karin is", to="you are")
rewrite(from="karin also likes", to="you like")
rewrite(from="karin likes", to="you like")
rewrite(from="what karin likes", to="what do you like")
rewrite(from="is karin", to="are you")
// Identity rewrite: keeps "your name is karin" from becoming "your name are you".
rewrite(from="name is karin", to="name is karin")
rewrite(from="you are a very", to="you are")
rewrite(from="you are very", to="you are")
rewrite(from="you are really", to="you are")
rewrite(from="you are so", to="you are")
rewrite(from="you are sometimes so", to="you are")
rewrite(from="you are sometimes", to="you are")
rewrite(from="you are not very", to="you are not")
rewrite(from="you are not so", to="you are not")
rewrite(from="can you are", to="you are")
rewrite(from="you is", to="you are")
rewrite(from="stop being so", to="stop being")
```

## Canonical wording

```karin
rewrite(from="which", to="what")
rewrite(from="i really like", to="i like")
rewrite(from="i really love", to="i like")
rewrite(from="i love", to="i like")
rewrite(from="i enjoy", to="i like")
rewrite(from="i adore", to="i like")
rewrite(from="what i like", to="what do i like")
rewrite(from="what is i like", to="what do i like")
rewrite(from="the symbolic ai", to="symbolic ai")
rewrite(from="a symbolic ai", to="symbolic ai")
rewrite(from="things do i like", to="what do i like")
rewrite(from="what things do i like", to="what do i like")
rewrite(from="what are the things i like", to="what do i like")
rewrite(from="what do you know i like", to="what do i like")
rewrite(from="is a my", to="is my")
rewrite(from="my best a friend", to="my best friend")
rewrite(from="drinks", to="drink")
rewrite(from="foods", to="food")
rewrite(from="songs", to="song")
rewrite(from="movies", to="movie")
rewrite(from="films", to="film")
rewrite(from="books", to="book")
rewrite(from="games", to="game")
rewrite(from="colors", to="color")
rewrite(from="colours", to="color")
rewrite(from="colour", to="color")
rewrite(from="sports", to="sport")
rewrite(from="listening to some", to="listening to")
rewrite(from="listening some", to="listening to")
rewrite(from="how are you feeling", to="how are you")
rewrite(from="how are you doing", to="how are you")
rewrite(from="how do you feel", to="how are you")
rewrite(from="how is it going", to="how are you")
rewrite(from="how are things", to="how are you")
rewrite(from="hey how are you", to="how are you")
rewrite(from="hi how are you", to="how are you")
rewrite(from="hello how are you", to="how are you")
rewrite(from="what tools do you have", to="what can you do")
rewrite(from="what tools you have", to="what can you do")
rewrite(from="what skills do you have", to="what can you do")
rewrite(from="what are your skills", to="what can you do")
rewrite(from="what are your tools", to="what can you do")
rewrite(from="what else can you do", to="what can you do")
rewrite(from="what are you able to do", to="what can you do")
rewrite(from="start a new chat", to="new chat")
rewrite(from="create a new chat", to="new chat")
rewrite(from="open a new chat", to="new chat")
rewrite(from="make a new chat", to="new chat")
rewrite(from="say that again", to="repeat")
rewrite(from="repeat that", to="repeat")
rewrite(from="repeat it", to="repeat")
rewrite(from="repeat after me", to="repeat")
rewrite(from="what did you say", to="repeat")
rewrite(from="what did you just say", to="repeat")
rewrite(from="what was your last message", to="repeat")
rewrite(from="what is the last message", to="repeat")
rewrite(from="what is a last message", to="repeat")
rewrite(from="what was the last message", to="repeat")
rewrite(from="what did i say", to="what did i just say")
rewrite(from="what was my last message", to="what did i just say")
```

## Notes

```karin
rewrite(from="take a note", to="make a note")
rewrite(from="create a note", to="make a note")
rewrite(from="save a note", to="make a note")
rewrite(from="add a note", to="make a note")
rewrite(from="write a note", to="make a note")
rewrite(from="make note", to="make a note")
rewrite(from="add more notes", to="make a note")
rewrite(from="make another note", to="make a note")
rewrite(from="make a note for me", to="make a note")
rewrite(from="make a note that", to="make a note")
rewrite(from="write down", to="make a note")
rewrite(from="note down", to="make a note")
rewrite(from="list all my notes", to="list my notes")
rewrite(from="show me my notes", to="list my notes")
rewrite(from="show my notes", to="list my notes")
rewrite(from="read my notes", to="list my notes")
rewrite(from="what are my notes", to="list my notes")
rewrite(from="what notes do i have", to="list my notes")
rewrite(from="remember me to", to="remember to")
```

## Calculator and units

```karin
rewrite(from="plus", to="+")
rewrite(from="minus", to="-")
rewrite(from="times", to="*")
rewrite(from="multiplied by", to="*")
rewrite(from="divided by", to="/")
rewrite(from="pounds", to="lb")
rewrite(from="pound", to="lb")
rewrite(from="lbs", to="lb")
rewrite(from="kilograms", to="kg")
rewrite(from="kilogram", to="kg")
rewrite(from="kilos", to="kg")
rewrite(from="kilo", to="kg")
rewrite(from="grams", to="g")
rewrite(from="gram", to="g")
rewrite(from="milligrams", to="mg")
rewrite(from="ounces", to="oz")
rewrite(from="ounce", to="oz")
rewrite(from="meters", to="m")
rewrite(from="meter", to="m")
rewrite(from="metres", to="m")
rewrite(from="kilometers", to="km")
rewrite(from="kilometer", to="km")
rewrite(from="kilometres", to="km")
rewrite(from="centimeters", to="cm")
rewrite(from="centimeter", to="cm")
rewrite(from="millimeters", to="mm")
rewrite(from="miles", to="mi")
rewrite(from="mile", to="mi")
rewrite(from="feet", to="ft")
rewrite(from="foot", to="ft")
rewrite(from="inches", to="in")
rewrite(from="inch", to="in")
rewrite(from="seconds", to="s")
rewrite(from="minutes", to="min")
rewrite(from="minute", to="min")
rewrite(from="hours", to="h")
rewrite(from="hour", to="h")
rewrite(from="days", to="day")
```

## Weather and definitions

```karin
rewrite(from="what is weather in", to="what is the weather in")
rewrite(from="how is weather in", to="how is the weather in")
rewrite(from="what is the weather like in", to="what is the weather in")
rewrite(from="what is the temperature in", to="what is the weather in")
rewrite(from="weather for", to="weather in")
rewrite(from="what is the definition of", to="define")
rewrite(from="what is the definition for", to="define")
rewrite(from="what is the meaning of", to="define")
rewrite(from="definition of", to="define")
rewrite(from="what is a", to="what is")
rewrite(from="what is an", to="what is")
```

## Fillers

Words that carry no meaning for Karin. `at` is `start`, `end` or `any`.

```karin
filler(text="please", at="any")
filler(text="um", at="any")
filler(text="uh", at="any")
filler(text="so", at="start")
filler(text="and", at="start")
filler(text="also", at="start")
filler(text="okay", at="start")
filler(text="ok", at="start")
filler(text="well", at="start")
filler(text="oh", at="start")
filler(text="just", at="start")
filler(text="actually", at="start")
filler(text="now", at="start")
filler(text="then", at="start")
filler(text="recently", at="start")
filler(text="by the way", at="start")
filler(text="can you please", at="start")
filler(text="could you please", at="start")
filler(text="can you", at="start")
filler(text="could you", at="start")
filler(text="would you", at="start")
filler(text="will you", at="start")
filler(text="can i", at="start")
filler(text="could i", at="start")
filler(text="i want you to", at="start")
filler(text="i would like you to", at="start")
filler(text="i need you to", at="start")
filler(text="for me", at="end")
filler(text="right now", at="end")
filler(text="now", at="end")
filler(text="today", at="end")
filler(text="currently", at="end")
filler(text="again", at="end")
filler(text="too", at="end")
filler(text="as well", at="end")
filler(text="same as me", at="end")
```

## Addressing Karin

A vocative is dropped at the start or end of a sentence, unless the next
(`side="after"`) or previous (`side="before"`) word shows Karin is the subject
or object ("karin is smart", "who is karin").

```karin
vocative(text="karin")
vocative(text="can can")
vocative(text="hey karin")
vocative(text="ok karin")

vocative_block(word="is", side="after")
vocative_block(word="was", side="after")
vocative_block(word="likes", side="after")
vocative_block(word="also", side="after")
vocative_block(word="has", side="after")
vocative_block(word="does", side="after")
vocative_block(word="'s", side="after")

vocative_block(word="is", side="before")
vocative_block(word="was", side="before")
vocative_block(word="about", side="before")
vocative_block(word="to", side="before")
vocative_block(word="with", side="before")
vocative_block(word="for", side="before")
vocative_block(word="of", side="before")
vocative_block(word="call", side="before")
vocative_block(word="called", side="before")
vocative_block(word="named", side="before")
vocative_block(word="meet", side="before")
vocative_block(word="like", side="before")
vocative_block(word="likes", side="before")

greeting(word="hi")
greeting(word="hello")
greeting(word="hey")
greeting(word="yo")
```

## Clauses

```karin
clause_joiner(word="and")
clause_joiner(word="also")
clause_joiner(word="but")
clause_joiner(word="then")

fixed_phrase(text="rock and roll")
fixed_phrase(text="black and white")
fixed_phrase(text="salt and pepper")
fixed_phrase(text="fish and chips")
fixed_phrase(text="mac and cheese")
fixed_phrase(text="bread and butter")
fixed_phrase(text="r and b")
fixed_phrase(text="pros and cons")
fixed_phrase(text="back and forth")
fixed_phrase(text="dungeons and dragons")
```

## Pronouns

Resolved against the chat's most recent `mention(entity, kind)`.

```karin
pronoun(word="he", kind="person")
pronoun(word="him", kind="person")
pronoun(word="she", kind="person")
pronoun(word="her", kind="person")
pronoun(word="there", kind="place")
```

## Hints when nothing matched

```karin
hint(word="note", text="For notes, say \"make a note <text>\" or \"list my notes\".")
hint(word="notes", text="For notes, say \"make a note <text>\" or \"list my notes\".")
hint(word="remind", text="Say \"remind me to <task> at <time>\".")
hint(word="reminder", text="Say \"remind me to <task> at <time>\".")
hint(word="weather", text="Ask \"what is the weather in <city>\" — I know Paris, Tokyo, London and Vilnius.")
hint(word="square root", text="I can do + - * / and parentheses, like \"calculate (5 + 7) / 2\". Square roots aren't supported yet.")
hint(word="calculate", text="I can do + - * / and parentheses, like \"calculate (5 + 7) / 2\".")
hint(word="divide", text="Try \"divide 5 by 7\" or \"calculate 5 / 7\".")
hint(word="convert", text="Try \"convert 5 kg to lb\".")
hint(word="friend", text="Tell me \"Oscar is my friend\", then ask \"who is my friend\".")
hint(word="favorite", text="Tell me \"my favorite drink is ...\", then ask \"what is my favorite drink\".")
hint(word="like", text="Tell me \"I like ...\", then ask \"what do I like\".")
hint(word="name", text="Say \"my name is ...\", or ask \"what is my name\".")
hint(word="app", text="I can't open or control apps yet.")
hint(word="open", text="I can't open or control apps yet.")
hint(word="chat", text="To start a new chat, use the New chat button.")
hint(word="smart", text="I'm a symbolic assistant: I understand the phrasings I've been taught, and I learn new ones when you teach me.")
hint(word="stupid", text="Sorry — I only understand the phrasings I've been taught. Teach me what you meant and I'll get it next time.")
```
