# Likes by category (`claude-opus-5.5`, 2026-11-26)

`vault/core/persona/memory.md` stores "I like black tea" without a category and
"my favorite drink is black tea" with one. Connecting the two ("black tea is a
drink" → it counts as a drink you like) is deliberately *not* hard-coded here:
the core learns that rule from a correction ("what about black tea?" →
"approve"), see `vault/core/learning/correction.md`.

## Categories people talk about

```karin
topic_word(word="music")
topic_word(word="food")
topic_word(word="drink")
topic_word(word="movie")
topic_word(word="film")
topic_word(word="book")
topic_word(word="game")
topic_word(word="song")
topic_word(word="color")
topic_word(word="sport")
topic_word(word="animal")
```

## More ways to ask


## More ways to tell

