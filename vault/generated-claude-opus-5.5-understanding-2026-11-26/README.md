# Understanding pack - made by Claude (Opus 5.5) on 2026-11-26

Everything in this folder was written by Claude (`claude-opus-5.5`) on
2026-11-26, branch `nlu-layer`, using the `vault-suggest` skill on the language
bench's failures, then applied after the user approved it. `vault/core` is
unchanged.

`understanding.md` holds lexicon fixes (Karin as the object of a sentence,
mishearings of her name, missing rewrites), relations without "my", likes of
people Karin knows, keyword fallbacks for long noisy sentences, honest declines,
and `chat_expiry` declarations so an unanswered question lapses after one turn.

It relies on two engine features added the same day: `chat_expiry(rel, turns)`
and the `words($x) <= n` condition.
