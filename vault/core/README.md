# Core: how Karin reasons

Language-independent mechanisms every other module builds on (see
`../README.md` for the layers).

| folder | what it holds |
|--------|---------------|
| `entities/` | People and Karin as entities, and who is speaking: every chat starts with an anonymous guest; `me.md` (the speaker: name, likes, city, work, goal, projects), `people.md` (other people and relations), `likes.md`, `karin.md` (Karin's own relations). See `entities/README.md`. The wordings for these meanings are in `language/people.md`. |
| `knowledge/` | The knowledge graph: `graph.md` (is-a and inheritance), `relations.md` (typed relations and inverses), `reasoning.md` (multi-hop place questions), `lists.md` (list members, parts, creations), `opposites.md`. |
| `dialogue/` | Questions that span turns: `frames.md` (ask for a missing detail, capture the answer, cancel) and `workflows.md` (the multi-step goal engine that reminders, meetings, flashcards and shopping use). |
| `learning/` | Learning from corrections and from Karin's own chains of rules. |
