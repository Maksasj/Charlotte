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
Ollama: two friendly agents (the same model in two separate chats) text each
other, each seeing only what the other wrote. One is given a topic and opens;
its lines are `U:` and the other's are `A:`. Everything it produces goes into
one folder named after the model, with one Markdown file per topic:

```
data/friendly-chats-qwen3.5-4b/
  001-saying-hello-and-how-the-day-is-going.md
  002-weekend-plans.md
  ...
```

Each file looks like this:

```
Entry 1
U: first agent
A: second agent
U: ...

Entry 2
...
```

```sh
python3 data/generate_conversations.py --total 20 --topics 4    # a small trial first
python3 data/generate_conversations.py                          # 10,000 over the topics in topics.txt
```

It goes through the topics in rounds: one chat for each topic, then the next
round. Stopped early, every topic has about the same number of chats.

Messages are kept to a sentence or two in a friendly, polite tone, halfway
between casual and formal: the prompt asks for 10 to 25 words, each message is
limited to 90 model tokens, and one that still runs past 40 words is cut back
to its first sentences (`--max-tokens`, `--max-words`).

While it runs it prints every message as it is generated, and after each
conversation a line with how many are done, the time passed, the estimated
time left and the speed. `--quiet` keeps only that line.

It needs only Python 3 and a running Ollama with the model pulled (default
`qwen3.5:4b`, change with `--model`). It can be stopped and started again and
carries on where it left off. Topics are in `topics.txt`, one per line.

To teach these chats to the engine, turn them into a corpus file first:

```sh
python3 data/chats_to_corpus.py                       # writes data/friendly-chats-<model>.jsonl
software/target/release/charlotte learn data/friendly-chats-qwen3.5-4b.jsonl
```

The chats carry no labels, so no pattern or wording is learned from them. The
engine remembers each message with the reply that followed (one note per topic
in `vault/chats/`) and, when a new message is similar enough to a remembered
one, answers with that reply.

## What is here

| File | What |
|---|---|
| `pretrain-80-per-shape.jsonl` | The 240 simulated conversations the vault was pretrained on |
| `paraphrases.yaml` | Extra wordings the simulated user draws on (`charlotte simulate --paraphrases`) |
| `generate_conversations.py` | Simulates conversations with a local Ollama model |
| `topics.txt` | The 100 topics the generator walks through |
| `friendly-chats-<model>/` | The generator's output, one file per topic, once it has been run |
| `friendly-chats-handwritten-claude-fable-5.2/` | 100 sample dialogues written directly by the assistant model (not simulated) in the target style: 10 topics, 10 dialogues each, same form |
| `chats_to_corpus.py` | Turns that folder into `friendly-chats-<model>.jsonl`, ready for `charlotte learn` |

A run with another model (`--model`) gets its own folder, and other teaching
material added later sits beside these.
