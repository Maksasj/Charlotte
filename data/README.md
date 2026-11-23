# Teaching data

Conversations the engine learns from. One file per batch, one conversation per
line of JSON:

```json
{"id": "c000123", "domain": "retail", "outcome": "resolved", "turns": [
  {"role": "user",  "text": "can I send back order 7741? got it 5 days ago",
   "intent": "refund", "facts": {"subject": "7741", "object": "order", "days": 5}},
  {"role": "agent", "text": "Is it still in the original packaging?",
   "acts": [{"op": "ask", "arg": "packaging"}]},
  {"role": "user",  "text": "yes, never opened", "facts": {"packaging": "original"}},
  {"role": "agent", "text": "Refund approved for 7741.", "acts": [{"op": "grant"}]}
]}
```

Only `role` and `text` are required. The labels decide what can be learned:

| Label | On | Needed to learn |
|---|---|---|
| `acts` (or `act` and `arg`) | agent turns | Patterns |
| `intent`, `facts` | user turns | Wordings for the lexicon; also lets patterns be learned without relying on the reader |
| `signal` (`done` or `failed`) | user turns | Progress through a procedure |

Intent, slot and operation names must be ones the vault knows (`vault/lexicon/`);
conversations about anything else are read but teach nothing. The full
description is under "Learning" in
[`software/docs/design.md`](../software/docs/design.md).

```sh
software/target/release/charlotte learn data/your-batch.jsonl      # from the repository root
```

## What is here

| File | What |
|---|---|
| `pretrain-80-per-shape.jsonl` | The 240 simulated conversations the vault was pretrained on |
| `paraphrases.yaml` | Extra wordings the simulated user draws on (`charlotte simulate --paraphrases`) |
