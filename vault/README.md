# Karin's vault

Karin's brain is plain Markdown: facts and rules in ```karin blocks. Every
folder here is a module; the engine loads `core` first, then every other
folder beside it in name order, so a new module is just a new folder.

| folder | layer | what it holds |
|--------|-------|---------------|
| `core/` | reasoning | How the brain works, independent of wording: entities and who is speaking (`entities/`), the knowledge graph (`knowledge/`), questions that span turns and the workflow engine (`dialogue/`), learning from corrections (`learning/`). |
| `language/` | understanding | Text to meaning: the lexicon (aliases, rewrites, fillers, how Karin's name gets misheard), and every wording that becomes a meaning (`intent(...)`) for core and the skills. Add a phrasing here and every rule that uses the meaning gains it. |
| `persona/` | character | Who Karin is: her name and help text, talking about herself, greetings and small talk. |
| `skills/` | tools | One folder per skill, like an agent's skills: what the skill does with a meaning, which tool it calls, how it replies. `calculator`, `units`, `weather`, `definitions`, `notes`, `reminders`, `meetings`, `flashcards`, `decide`, `contacts`, `shopping`. |
| `knowledge/` | facts | Things to know (e.g. `science.md`); they plug into the knowledge graph in core. |
| `.chats/`, `.episodes/` | runtime | Chat logs and learned episodes (not modules). |

`state.json` is Karin's saved memory (what she learned while talking).

## How a sentence flows

1. `language/lexicon.md` cleans it up ("Karen" -> "karin", fillers dropped).
2. A wording rule in `language/` turns it into a meaning, e.g.
   `intent(name="speaker_is", value="Oscar")` or `intent(name="weather", city="Paris")`.
3. `core/` or a skill decides what that meaning does and replies.

Replies are still written in English inside core and skill rules; only the
understanding side lives in `language/`.

## History

Until 2026-11-26 everything sat in `core/` plus per-model `generated-*` packs.
That day the packs were folded in and the vault split into these layers (see
git history for each file's origin).
