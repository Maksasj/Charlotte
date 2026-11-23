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

## Generating conversations with a local model

`generate_conversations.py` holds conversations live with a model served by
Ollama. The model is the agent, a friendly general chat partner that sees only
what the user wrote; the same
model in a separate chat plays a person texting about something on their mind.
It writes
one Markdown file per topic into `data/conversations/`:

```
Entry 1
U: user text
A: agent answer
U: ...

Entry 2
...
```

```sh
python3 data/generate_conversations.py --total 20 --topics 4    # a small trial first
python3 data/generate_conversations.py                          # 10,000 over the topics in topics.txt
```

It needs only Python 3 and a running Ollama with the model pulled (default
`qwen3.5:4b`, change with `--model`). It can be stopped and started again and
carries on where it left off. Topics are in `topics.txt`, one per line.

These files are plain text with no labels and are not in the JSON-lines form
above, so `charlotte learn` cannot read them as they are.

## What is here

| File | What |
|---|---|
| `pretrain-80-per-shape.jsonl` | The 240 simulated conversations the vault was pretrained on |
| `paraphrases.yaml` | Extra wordings the simulated user draws on (`charlotte simulate --paraphrases`) |
| `generate_conversations.py` | Simulates conversations with a local Ollama model |
| `topics.txt` | The 100 topics the generator walks through |
