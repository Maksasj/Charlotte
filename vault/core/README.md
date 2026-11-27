# Core: how Karin reasons

Language-independent mechanisms every other module builds on (see
`../README.md` for the layers).

| folder | what it holds |
|--------|---------------|
| `entities/` | People and Karin as entities, and who is speaking: every chat starts with an anonymous guest; `me.md` (the speaker: name, likes, city, work, goal, projects), `people.md` (other people and relations), `likes.md`, `karin.md` (Karin's own relations). See `entities/README.md`. The wordings for these meanings are in `language/people.md`. |
| `knowledge/` | The knowledge graph: `graph.md` (is-a and inheritance), `relations.md` (typed relations and inverses), `reasoning.md` (multi-hop place questions), `lists.md` (list members, parts, creations), `opposites.md`. |
| `dialogue/` | Questions that span turns: `frames.md` (ask for a missing detail, capture the answer, cancel) and `workflows.md` (the multi-step goal engine that reminders, meetings, flashcards and shopping use). |
| `learning/` | Learning from corrections and from Karin's own chains of rules. |

## How a sentence is read (engine)

1. **Canonical form** - the language layer's aliases, rewrites and fillers.
2. **Whole and nested readings** - the sentence is matched as a whole; when a
   `wrap` fits ("sorry $act", "do you know $act"; see `language/wrappers.md`),
   the request inside is also read as a sentence of its own, recursively (up
   to 3 deep). The reading whose rules matched more of the words wins; a tie
   keeps the whole reading.
3. **Clauses** - "Oscar is my friend and he lives in Venice" is read clause by
   clause (each clause again whole and nested) when the whole is not understood
   or every clause is.
4. **Exact wording** - replies taught on that exact text.
