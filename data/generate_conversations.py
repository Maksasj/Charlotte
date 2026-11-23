#!/usr/bin/env python3
"""Simulate conversations with a local Ollama model, one Markdown file per topic.

Nothing is asked to "write a conversation". Each conversation is held live, turn by turn:

  - the agent is the model chatting as a friendly general assistant, seeing only what the user wrote;
  - the user is the same model in a separate chat, playing a person who texts about something on their mind.

Each file holds the conversations for one topic in this form:

    Entry 1
    U: user text
    A: agent answer
    U: ...

    Entry 2
    ...

The script walks through the topics in data/topics.txt until every topic has its share of the
total. It can be stopped and started again: it counts what each file already holds and carries on.

    python3 data/generate_conversations.py --total 20 --topics 4    # a small trial first
    python3 data/generate_conversations.py                          # 10,000 conversations

Needs only Python 3 and a running Ollama (https://ollama.com) with the model pulled.
"""

import argparse
import json
import random
import re
import sys
import time
import urllib.error
import urllib.request
from pathlib import Path

HERE = Path(__file__).resolve().parent
END = "[END]"

AGENT_SYSTEM = """You are a friendly chat companion. Someone is texting with you.

- Talk like a warm, easygoing person would in a chat: natural, kind, and to the point, usually one to three sentences.
- Help with whatever comes up: questions, advice, ideas, explanations, or just a chat. Share what you know in plain words.
- If you are not sure what they mean, ask. If you do not know something, say so.
- Show interest in the person: react to what they say, and ask something back when it fits.
- Plain text only. No lists, no markdown, no headings. Write only your reply."""

USER_SYSTEM = """You are a regular person texting with a friendly chat assistant. This is what is on your mind:

Topic: {topic}
What is going on: {situation}
How you write: {style}

- Write the way people really text: simple everyday words, short messages, one or two sentences.
- Only say what a real person would say at this point. Do not explain everything at once unless your situation says so.
- React to what the assistant says: answer its questions, ask a follow-up, agree, push back, or change your mind, as a person would.
- Make up ordinary details about your own life when you need them, and stick to them.
- Never write as the assistant, never describe what you are doing, no quotation marks.
- When you have what you wanted, or the chat has run its course, close it naturally (a thanks or a goodbye) if that feels right. Once there is nothing more to say, reply with exactly {end} and nothing else."""

# What kind of exchange the user is about to have. The assistant is not told any of this.
SITUATIONS = [
    "you open with a greeting and a bit of small talk before getting to it",
    "you have one simple question and want a quick answer",
    "you want advice and are not sure what to do",
    "you ask something and then want it explained more simply",
    "you want a few ideas or suggestions to choose from",
    "you have an opinion already and want to see if the assistant agrees",
    "you realise partway through that you said something wrong, and you correct it",
    "you want to be walked through how to do something, one step at a time",
    "you tried what was suggested and it did not work, and you say so",
    "you have two things on your mind and mention both",
    "you start on one thing and drift to a related one",
    "you are just in the mood to chat about it and have no real question",
    "you are curious why something is the way it is",
    "you are comparing two options and cannot decide",
]
STYLES = [
    "polite and a little formal, full sentences",
    "casual, lowercase, hardly any punctuation",
    "brief and matter-of-fact",
    "friendly and chatty",
    "in a hurry, very short messages",
    "relaxed and easygoing",
    "unsure and hesitant",
    "enthusiastic",
    "casual with the odd typo",
    "direct, no greeting",
]

ENTRY = re.compile(r"^\s*entry\s+\d+\s*$", re.IGNORECASE)
TURN = re.compile(r"^([UA]): (.*)$")


def slug(topic):
    return re.sub(r"[^a-z0-9]+", "-", topic.lower()).strip("-")[:60]


def read_topics(path):
    lines = [l.strip() for l in Path(path).read_text(encoding="utf-8").splitlines()]
    topics = [l for l in lines if l and not l.startswith("#")]
    if not topics:
        sys.exit(f"no topics in {path}")
    return topics


def read_file(path):
    """Conversations already in a topic file, each a list of (speaker, text)."""
    convs = []
    if path.exists():
        for line in path.read_text(encoding="utf-8").splitlines():
            if ENTRY.match(line):
                convs.append([])
            else:
                m = TURN.match(line)
                if m and convs:
                    convs[-1].append((m.group(1), m.group(2)))
    return convs


def clean(text):
    """One message on one line, without the wrapping a model sometimes adds."""
    text = re.sub(r"<think>.*?</think>", "", text, flags=re.DOTALL | re.IGNORECASE)
    text = text.split("</think>")[-1]  # a thinking block that was cut off at the start
    text = " ".join(text.split())
    text = re.sub(r"^\**\s*(?:U|A|User|Human|Person|Agent|Assistant|Bot|AI|Me)\s*\**\s*:\s*\**\s*", "", text, flags=re.IGNORECASE)
    text = text.replace("**", "").strip()
    if len(text) >= 2 and text[0] in "\"'“‘" and text[-1] in "\"'”’":
        text = text[1:-1].strip()
    return text


class Stop(Exception):
    """Ollama cannot be used; no point going on."""


def chat(args, system, turns, seed, max_tokens):
    """One call to Ollama's chat endpoint. `turns` is a list of (role, text)."""
    body = {
        "model": args.model,
        "messages": [{"role": "system", "content": system}] + [{"role": r, "content": t} for r, t in turns],
        "stream": False,
        "options": {"temperature": args.temperature, "seed": seed, "num_predict": max_tokens},
    }
    if not args.think and not args.no_think_option:
        body["think"] = False  # reasoning models: skip the thinking, it is slow and not wanted here
    last = None
    for attempt in range(3):
        req = urllib.request.Request(args.host.rstrip("/") + "/api/chat", data=json.dumps(body).encode(), headers={"Content-Type": "application/json"})
        try:
            with urllib.request.urlopen(req, timeout=args.timeout) as r:
                return json.loads(r.read().decode())["message"]["content"]
        except urllib.error.HTTPError as e:
            detail = e.read().decode(errors="replace").strip()[:300]
            if "think" in body and "think" in detail.lower():
                del body["think"]  # this model or this Ollama version does not take the option
                args.no_think_option = True
                continue
            if e.code == 404:
                raise Stop(f"Ollama does not have the model '{args.model}': {detail}\nPull it (`ollama pull {args.model}`) or pass another with --model; `ollama list` shows what is there.")
            last = f"Ollama answered {e.code}: {detail}"
        except (urllib.error.URLError, ConnectionError) as e:
            raise Stop(f"cannot reach Ollama at {args.host}: {e}\nIs it running (`ollama serve`)?")
        except TimeoutError:
            last = f"no reply within {args.timeout} seconds"
        time.sleep(2)
    raise Stop(last or "no reply from Ollama")


def simulate(args, topic, rng, recent):
    """Hold one conversation. Returns its turns, or None if it did not come out usable."""
    system = USER_SYSTEM.format(topic=topic, situation=rng.choice(SITUATIONS), style=rng.choice(STYLES), end=END)
    opening = "You have just opened the chat. Send your first message."
    if recent:
        opening += " Other people opened with the lines below; you are a different person with something else on your mind, so do not start like them:\n" + "\n".join(f"- {r}" for r in recent)
    conv = []  # (speaker, text), speaker is "U" or "A"
    for turn in range(args.max_turns):
        # The user's side of the chat: its own lines are "assistant", the agent's are "user".
        as_user = [("user", opening)] + [("assistant" if who == "U" else "user", said) for who, said in conv]
        said = clean(chat(args, system, as_user, rng.randrange(2**31), args.user_tokens))
        if END in said.upper() or not said:
            break
        conv.append(("U", said))
        # The agent's side: an ordinary chat in which it only sees what the user wrote.
        as_agent = [("user" if who == "U" else "assistant", said) for who, said in conv]
        reply = clean(chat(args, AGENT_SYSTEM, as_agent, rng.randrange(2**31), args.agent_tokens))
        if not reply:
            conv.pop()
            break
        conv.append(("A", reply))
    return conv if len(conv) >= 2 else None


def main():
    p = argparse.ArgumentParser(description="Simulate conversations with a local Ollama model, one Markdown file per topic.")
    p.add_argument("--total", type=int, default=10000, help="conversations in all, spread evenly over the topics (default 10000)")
    p.add_argument("--topics-file", default=str(HERE / "topics.txt"), help="one topic per line (default data/topics.txt)")
    p.add_argument("--topics", type=int, default=0, help="use only the first N topics (for a trial run)")
    p.add_argument("--out", default=str(HERE / "conversations"), help="folder for the Markdown files (default data/conversations)")
    p.add_argument("--model", default="qwen3.5:4b", help="Ollama model tag (default qwen3.5:4b; `ollama list` shows what is installed)")
    p.add_argument("--host", default="http://127.0.0.1:11434", help="where Ollama listens")
    p.add_argument("--max-turns", type=int, default=6, help="most user messages in one conversation (default 6)")
    p.add_argument("--temperature", type=float, default=0.9)
    p.add_argument("--user-tokens", type=int, default=120, help="limit on one user message")
    p.add_argument("--agent-tokens", type=int, default=250, help="limit on one agent reply")
    p.add_argument("--timeout", type=int, default=300, help="seconds to wait for one reply")
    p.add_argument("--seed", type=int, default=1, help="makes the situations and styles repeatable")
    p.add_argument("--think", action="store_true", help="let a reasoning model think first (slower)")
    args = p.parse_args()
    args.no_think_option = False

    topics = read_topics(args.topics_file)
    if args.topics:
        topics = topics[: args.topics]
    out = Path(args.out)
    out.mkdir(parents=True, exist_ok=True)
    # Spread the total evenly; the first few topics take the remainder.
    share, extra = divmod(args.total, len(topics))
    wanted = [share + (1 if i < extra else 0) for i in range(len(topics))]
    width = len(str(len(topics)))
    files = [out / f"{i + 1:0{width}d}-{slug(t)}.md" for i, t in enumerate(topics)]

    before = sum(min(len(read_file(f)), w) for f, w in zip(files, wanted))
    print(f"{args.total} conversations over {len(topics)} topics with {args.model}; {before} already written, in {out}")
    started, made, dropped = time.time(), 0, 0

    for ti, (topic, path) in enumerate(zip(topics, files)):
        convs = read_file(path)
        if not path.exists():
            path.write_text(f"# {topic}\n", encoding="utf-8")
        seen = {" ".join(s.lower() for _, s in c) for c in convs}
        failures = 0
        while len(convs) < wanted[ti]:
            # Seeded by topic and position, so a restarted run asks for the same situations.
            rng = random.Random(f"{args.seed}/{topic}/{len(convs)}/{failures}")
            conv = simulate(args, topic, rng, [c[0][1] for c in convs[-6:]])
            key = " ".join(s.lower() for _, s in conv) if conv else None
            if conv is None or key in seen:
                dropped += 1
                failures += 1
                if failures >= 8:
                    print(f"\n  giving up on '{topic}' after 8 unusable conversations in a row; {len(convs)} of {wanted[ti]} written")
                    break
                continue
            failures = 0
            convs.append(conv)
            seen.add(key)
            with path.open("a", encoding="utf-8") as f:
                f.write(f"\nEntry {len(convs)}\n" + "".join(f"{who}: {said}\n" for who, said in conv))
            made += 1
            rate = made / max(time.time() - started, 1e-9)
            left = (args.total - before - made) / rate
            print(f"\r  {before + made}/{args.total}  topic {ti + 1}/{len(topics)} '{topic[:36]}'  {rate * 60:.1f}/min  about {left / 3600:.1f} h left   ", end="", flush=True)

    written = sum(len(read_file(f)) for f in files)
    print(f"\n{written} conversations in {len(topics)} files under {out}" + (f"; {dropped} dropped as empty or repeated" if dropped else ""))


if __name__ == "__main__":
    try:
        main()
    except Stop as e:
        sys.exit(f"\n{e}")
    except KeyboardInterrupt:
        print("\nstopped; run the same command again to carry on")
