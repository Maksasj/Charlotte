# Karin language bench

Measures how well Karin understands **real voice chats**: not just whether she
replied, but whether she understood the right thing, pulled out the right
values, and declined or asked to repeat when she should.

```sh
# against a running karin-server (needs the /api/incognito routes)
python bench/run_bench.py --label my-change

# start a private server just for the run (CI); add --cargo "cargo +<toolchain>" if needed
python bench/run_bench.py --start-server --label ci

# fail the run (exit 1) if it gets worse than a threshold
python bench/run_bench.py --start-server --fail-under 0.85 --max-wrong 0.04
```

Standard library Python only. Exit codes: `0` ok, `1` a threshold failed,
`2` server unreachable or too old.

## Isolation

Each chat runs in its own **incognito session**: a fresh engine loaded from the
vault, never saved to `state.json` and never logged to `vault/.chats`. A bench
run never touches Karin's live memory, and it tests the **vault**, not whatever
the live server has learned since.

## Cases

`cases/<chat-id>.json`: one file per recorded chat, the wording exactly as the
microphone heard it. Each scored turn says what a sensible assistant should do:

```json
{"n": 11, "say": "what is the weather in a wellness",
 "intent": "weather.ask", "respond": "answer", "slots": ["Vilnius"],
 "note": "'a wellness' is how the mic heard Vilnius"}
```

| Field | Meaning |
|---|---|
| `intent` | the meaning (see `intents.json`), a list when one utterance says several things, or `noise` when nothing sensible can be recovered |
| `respond` | `answer`, `clarify` (ask a question), `decline` (honest "can't do that yet"), `repeat` (noise), or a list of acceptable ones |
| `slots` | values that must appear in what Karin did (reply, bindings, stored facts), case-insensitive |
| `also` | other intents that are also fine |
| `setup` (per chat) | unscored turns run first, for things the chat relied on from earlier chats |
| `skip` | turn or chat not scored (duplicates, test messages) |

The first labels are a draft by Claude and need a human review.

## Scoring

| Outcome | When |
|---|---|
| correct | right intent, right kind of response, expected values present |
| partial | right intent, wrong kind of response |
| value_error | right intent and kind, an expected value missing |
| **wrong** | answered with a different meaning (the worst outcome) |
| missed | in scope, but Karin got stuck |
| correct_reject | noise, and Karin got stuck or asked to repeat |
| **false_accept** | noise, but answered as if understood |

Headline: **accuracy** (correct + correct_reject over all turns) and **wrong-answer
rate** (wrong + false_accept over all turns). A change is an improvement only if
accuracy goes up and wrong answers don't.

Karin's meaning is read from the trace: the rule that produced the reply is
looked up in `intents.json` (shared dialogue rules defer to the rule before
them). Rules missing from the map are listed under "Unmapped rules" in the report.

## Results

Each run writes `data/training/bench/<time>-<label>/` with `report.md`,
`summary.json` and `results.jsonl`, and compares with the previous run
(fixed / broken turns).

## Caveat

All current cases are chats the rules were written from, so the score shows
"the fixes still work", not "Karin understands new wording". Rephrasings of
these chats come next as a separate set.
