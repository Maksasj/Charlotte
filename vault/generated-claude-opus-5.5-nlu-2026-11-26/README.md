# Language layer pack - made by Claude (Opus 5.5) on 2026-11-26

Everything in this folder was written by Claude (model `claude-opus-5.5`) in one
session on 2026-11-26, on branch `nlu-layer`. Nothing in `vault/core` was changed;
remove this folder and Karin behaves as before (except that slot dialogues need
the `chat_memory` declarations in `lexicon.md`, see below).

## Why

Yesterday's voice chats (`vault/.chats/`, 318 turns) ended in an impasse 67% of
the time. Most failures were not missing knowledge but wording: speech
recognition hears "Karin" as "Karen"/"can can", adds fillers ("so", "please",
"right now"), uses contractions, and says several things in one breath.

## What is here

| file | what it holds |
|------|---------------|
| `lexicon.md` | Facts the engine's language layer (`karin-core/src/nlu.rs`) reads: aliases, rewrites, fillers, vocatives, clause joiners, fixed phrases, pronouns, hints, chat-memory declarations. Adding a word here helps every rule. |
| `persona.md` | Karin talking about herself: name, "you are ...", compliments, criticism, favourites. |
| `social.md` | Greetings with her name, thanks, sorry, stop, repeat, new chat, things she can't do yet. |
| `people.md` | People and relations: "Oscar is my best friend", "who is my best friend", "Oscar likes ice cream", "he lives in Venus". |
| `likes.md` | More ways to ask and tell what you like: bare "favorite drink", "what drink is my favorite", "I like rock music", "I started listening to punk". |
| `notes.md` | All the ways of making / listing notes, and asking for the text when it's missing. |
| `skills.md` | Calculator words, unit names, weather wording, definitions fallback. |

## Chat memory

The engine used to keep any working fact whose name started with `dialogue_`
(or was `asked_slot`) for the whole chat - a rule hard-coded in Rust. Now there
is a real `chat` scope: a rule writes `+chat rel(...)`, or the vault declares a
relation as chat memory with `chat_memory(rel=...)` / `chat_memory(prefix=...)`.
Chat facts survive turns, are cleared by "new chat", are bounded per relation
(oldest forgotten first) and re-saying one makes it the most recent. Pronouns
("he", "she", "there") resolve to the most recent `mention(entity, kind)`.
Karin also remembers what was said in the chat as `said(by, text)`.
