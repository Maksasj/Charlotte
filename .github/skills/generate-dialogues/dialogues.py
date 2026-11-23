#!/usr/bin/env python3
"""Helper for the generate-dialogues skill.

It keeps the dialogues produced through sub-agents identical in form to those written by
data/generate_conversations.py: same prompts, same topics and round order, same cleaning, same
"Entry N" / "U:" / "A:" files, in a folder named after the model.

    dialogues.py plan   --model MODEL --count N      what to generate next, as JSON
    dialogues.py add    --model MODEL --topic TOPIC  save one dialogue read from standard input
    dialogues.py status --model MODEL                how many dialogues each topic has
"""

import argparse
import importlib.util
import json
import random
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[3]
DATA = REPO / "data"

spec = importlib.util.spec_from_file_location("generate_conversations", DATA / "generate_conversations.py")
gen = importlib.util.module_from_spec(spec)
sys.dont_write_bytecode = True
spec.loader.exec_module(gen)

MAX_WORDS = 40  # the generator's default for --max-words


def folder(model):
    return DATA / ("friendly-chats-" + re.sub(r"[^a-z0-9.]+", "-", model.lower()).strip("-"))


def layout(model):
    """Topics and their files, named as the generator names them."""
    topics = gen.read_topics(DATA / "topics.txt")
    width = len(str(len(topics)))
    out = folder(model)
    return topics, [out / f"{i + 1:0{width}d}-{gen.slug(t)}.md" for i, t in enumerate(topics)]


def plan(model, count):
    """The next `count` dialogues in round order: one per topic, then the next round; gaps first."""
    topics, files = layout(model)
    have = [gen.read_file(f) for f in files]
    todo, rnd = [], 0
    while len(todo) < count:
        for ti, topic in enumerate(topics):
            if len(todo) >= count:
                break
            planned = sum(1 for t in todo if t["topic"] == topic)
            if len(have[ti]) + planned <= rnd:
                entry = len(have[ti]) + planned + 1
                rng = random.Random(f"skill/{model}/{topic}/{entry}")
                recent = [c[0][1] for c in have[ti][-6:]]
                opening = "Send your first message."
                if recent:
                    opening += " Earlier chats on this topic opened with the lines below; start differently:\n" + "\n".join(f"- {r}" for r in recent)
                todo.append({
                    "topic": topic,
                    "entry": entry,
                    "round": rnd + 1,
                    "exchanges": rng.choice(gen.LENGTHS),
                    "opener_prompt": gen.CHAT_SYSTEM + gen.OPENER.format(topic=topic),
                    "second_prompt": gen.CHAT_SYSTEM,
                    "opening_instruction": opening,
                })
        rnd += 1
    return todo


def add(model, topic, text):
    topics, files = layout(model)
    if topic not in topics:
        sys.exit(f"unknown topic: {topic!r} (topics are the lines of data/topics.txt)")
    path = files[topics.index(topic)]
    conv = []
    for line in text.splitlines():
        m = re.match(r"^\s*([UA]):\s?(.*)$", line)
        if m:
            said = gen.shorten(gen.clean(m.group(2)), MAX_WORDS)
            conv.append((m.group(1), said))
        elif line.strip() and conv:
            # A message that ran onto another line belongs to the message before.
            who, said = conv[-1]
            conv[-1] = (who, gen.shorten(gen.clean(said + " " + line.strip()), MAX_WORDS))
    speakers = [who for who, _ in conv]
    if len(conv) < 2 or speakers[0] != "U" or speakers[-1] != "A" or any(a == b for a, b in zip(speakers, speakers[1:])) or not all(s for _, s in conv):
        sys.exit("not saved: a dialogue is lines starting with 'U: ' and 'A: ', alternating, first U and last A, none empty")
    convs = gen.read_file(path)
    if any(" ".join(s.lower() for _, s in c) == " ".join(s.lower() for _, s in conv) for c in convs):
        sys.exit("not saved: the same dialogue is already in the file")
    path.parent.mkdir(parents=True, exist_ok=True)
    if not path.exists():
        path.write_text(f"# {topic}\n", encoding="utf-8")
    with path.open("a", encoding="utf-8") as f:
        f.write(f"\nEntry {len(convs) + 1}\n" + "".join(f"{who}: {said}\n" for who, said in conv))
    print(f"saved as Entry {len(convs) + 1} in {path.relative_to(REPO)} ({len(conv)} messages)")


def status(model):
    topics, files = layout(model)
    counts = [len(gen.read_file(f)) for f in files]
    print(f"{folder(model).relative_to(REPO)}: {sum(counts)} dialogues on {sum(1 for c in counts if c)} of {len(topics)} topics"
          + (f", {min(counts)} to {max(counts)} per topic" if sum(counts) else ""))


def main():
    p = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    sub = p.add_subparsers(dest="cmd", required=True)
    for name in ("plan", "add", "status"):
        s = sub.add_parser(name)
        s.add_argument("--model", required=True, help="the model the sub-agents run on, for example claude-haiku-4.5")
        if name == "plan":
            s.add_argument("--count", type=int, default=100)
        if name == "add":
            s.add_argument("--topic", required=True)
    a = p.parse_args()
    if a.cmd == "plan":
        print(json.dumps(plan(a.model, a.count), ensure_ascii=False, indent=1))
    elif a.cmd == "add":
        add(a.model, a.topic, sys.stdin.read())
    else:
        status(a.model)


if __name__ == "__main__":
    main()
