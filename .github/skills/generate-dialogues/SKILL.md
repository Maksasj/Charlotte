---
name: generate-dialogues
description: Generate friendly-chat dialogues for teaching data by having two sub-agents on a Haiku model text each other live, in the same manner and format as data/generate_conversations.py, saved under data/friendly-chats-<model>/. Use when asked to generate dialogues or conversations with sub-agents, with Haiku, or "like the Ollama script but with Claude". Default is 100 dialogues.
---

# Generate dialogues with sub-agents

Produces the same teaching material as `data/generate_conversations.py`, but
the two sides are sub-agents instead of a local Ollama model. Each dialogue is
held live: two sub-agents text each other, each seeing only what the other
wrote. Nobody is ever asked to "write a dialogue".

The helper `dialogues.py` in this skill's folder decides what to generate and
saves the result, so prompts, topics, round order, cleaning and file layout
stay identical to the Ollama generator. Run it from the repository root.

## Inputs

| Input | Default | Notes |
|---|---|---|
| How many dialogues | 100 | Use the number the user gives |
| Model | `claude-haiku-4.5` | Use the Haiku model the user names; pass it as the `model` of every sub-agent and as `--model` to the helper |

The output folder follows from the model: `data/friendly-chats-<model>/`, one
Markdown file per topic, for example `data/friendly-chats-claude-haiku-4.5/`.

## Steps

1. **Plan.** Ask the helper what to generate next. It continues from what the
   folder already holds, one dialogue per topic per round.

   ```sh
   python3 .github/skills/generate-dialogues/dialogues.py plan --model claude-haiku-4.5 --count 100
   ```

   It prints a JSON list. Each item has `topic`, `entry`, `exchanges` (how many
   U/A pairs this dialogue runs for), `opener_prompt`, `second_prompt` and
   `opening_instruction`. Use the prompts exactly as given.

2. **Hold each dialogue live.** For one item:

   1. Start the **opener**: a `general-purpose` sub-agent on the Haiku model, in
      background mode so it can take further turns. Its prompt is
      `opener_prompt`, then the chat protocol below, then
      `opening_instruction`. Its reply is the first `U:` line.
   2. Start the **second** agent the same way with `second_prompt`, the chat
      protocol, and then `The other person wrote:` followed by that first
      message. Its reply is the first `A:` line.
   3. For each further exchange, send the latest `A:` line to the opener with
      `write_agent` and read its new reply (the next `U:` line), then send that
      to the second agent and read its reply (the next `A:` line). Send only the
      other person's message, nothing else.
   4. Stop after `exchanges` pairs. The dialogue ends on an `A:` line.

   The chat protocol to append to both prompts, word for word:

   ```
   This is a live text chat. From now on, every message you receive is what the other person wrote.
   Reply with only your next message, as plain text on one line. Do not use any tools, do not read
   or write files, do not add notes or explanations, and do not write the other person's lines.
   ```

3. **Save.** Pass the dialogue to the helper on standard input, one message per
   line, each starting with `U: ` or `A: `:

   ```sh
   python3 .github/skills/generate-dialogues/dialogues.py add --model claude-haiku-4.5 --topic "weekend plans" <<'EOF'
   U: ...
   A: ...
   EOF
   ```

   It cleans the messages the way the generator does, checks the form, refuses
   duplicates, and appends `Entry N` to the topic's file. If it says "not
   saved", do that dialogue again from the start.

4. **Report.** Every 10 dialogues tell the user how many are done. At the end
   run `dialogues.py status --model <model>` and give its line, the folder, and
   how many dialogues had to be redone.

## Rules

- **Do not write or edit any message yourself.** Pass each reply on exactly as
  the sub-agent gave it. The helper does the only cleaning.
- **Do not tell a sub-agent** that it is generating data, how long the chat will
  be, or anything about the other side.
- **Redo, do not repair.** If a reply is empty, uses a tool, steps out of the
  chat ("As an AI…", "Here is my message:") or writes both sides, drop the whole
  dialogue and hold it again with fresh sub-agents.
- **Work in parallel.** Hold several dialogues at once (about five) by issuing
  their tool calls together. Keep each dialogue's two agents strictly paired.
- **Keep going on failures.** After three failed attempts at the same item,
  skip it and say so in the report.
- **Do not commit, push or teach** unless the user asked for it.

## Cost

One dialogue is two sub-agents and on average about three exchanges, so 100
dialogues are about 200 sub-agents and 600 to 700 replies. Tell the user this
before starting a run of 100 or more if they have not already confirmed it.

## Afterwards

To teach the result to the engine:

```sh
python3 data/chats_to_corpus.py data/friendly-chats-<model>
software/target/release/charlotte learn data/friendly-chats-<model>.jsonl
```
