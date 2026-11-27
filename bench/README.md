# Karin bench

Measures how well Karin understands **real voice chats**, by replaying them and
checking her replies. Tests are written in plain TOML and checked only against
what Karin **says** and whether she was **stuck** - never against rule or intent
names - so they keep working whatever happens to the vault's schema.

```sh
# start a private server just for the run (also what CI should do)
python bench/run.py --start-server --cargo "cargo +stable-x86_64-pc-windows-msvc" --label my-change

# against a server you already run (needs the /api/incognito routes)
python bench/run.py --label my-change

# compare with a chosen earlier run instead of the latest; one split only
python bench/run.py --start-server --label my-change --baseline main --split test

# exit 1 if the pass rate drops below a threshold
python bench/run.py --start-server --fail-under 0.80
```

Standard library Python only (3.11+ for `tomllib`). Exit codes: `0` ok, `1` a
threshold failed, `2` server unreachable or too old.

## Isolation

Every file runs in its own **incognito session**: a fresh engine loaded from the
vault, never saved to `state.json` and never logged to `vault/.chats`. A run
never touches Karin's live memory, and it tests the **vault**, not whatever the
live server has learned since.

## Test files

Every `*.toml` under `data/` is a test, in any subfolder (the run walks the
folder recursively; the file's path shows in results). `data/chats/<chat-id>.toml`
holds one file per recorded chat, the wording exactly as the microphone heard
it; new kinds of tests (documents to read, paraphrases, ...) get their own
folder next to it. `--tests <folder>` runs a different folder. The full spec is
the header of `run.py`; unknown keys are rejected.

```toml
chat   = "chat-1795695278"
split  = "test"                        # dev: rules were written from it · test: unseen
source = "vault/.chats/chat-1795695278.md"
before = ["my name is Maksim"]         # said first, not scored
# text = '''Oscar moved to Paris. He loves tea.'''   # read sentence by sentence, not scored

[[turn]]
say      = "what is your name"
mentions = ["Karin"]                   # the reply must contain each
tags     = ["persona.name"]            # free labels for the per-topic breakdown

[[turn]]
say   = "Karen I like you"
avoid = ["you like you"]               # the reply must contain none of these

[[turn]]
say    = "uh the the"
expect = "ignore"                      # understand (default) · ask · ignore · any
note   = "free text"
```

| `expect` | passes when |
|---|---|
| `understand` (default) | she is not stuck, and the reply has every `mentions` and no `avoid` |
| `ask` | she is not stuck and asks a question back |
| `ignore` | noise: she is stuck (does not act on it) |
| `any` | not scored (said for context) |

A turn with no `mentions`/`avoid` only checks that she wasn't stuck, so a
confident wrong answer there passes. When a turn's meaning matters, give it
`mentions` (what a right answer says) or `avoid` (what a wrong one says).

The `text` field means a test can also be a document followed by questions -
reading comprehension, the long-term way to test understanding of arbitrary
text without labelling any internal structure.

### Splits

- **dev**: chats rules have been written from.
- **test**: chats nobody wrote rules from (currently `chat-1795695278`). Keep
  them unseen: fix things because of dev chats, measure on test.

Labels on the test chat are a draft (see its `labels` field) - review them.

## Results

`results/<time>-<label>/` (git-ignored, kept on disk):

- `results.jsonl`: one line per turn: what was said, the check, outcome
  (`pass`/`fail`/`skip`), why it failed, the reply.
- `summary.json`: the numbers, per split and per tag.
- `report.md`: the numbers, fixed/broken turns against the baseline, every
  failing turn, and the per-tag table.

Headline numbers: **pass** (all scored turns), **understood** (not stuck on
turns she should handle), **answers** (reply content checks), **asks**,
**ignores noise**.

Runs from before 2026-11-27 used an older intent-based bench (`run_bench.py`,
`cases/*.json`, `intents.json`; in git history). Their results stay in
`results/` but are never picked as a baseline.
