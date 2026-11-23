#!/usr/bin/env python3
"""Simulate conversations with a local Ollama model, one Markdown file per topic.

Nothing is asked to "write a conversation". Each conversation is held live, turn by turn, by two
friendly agents: the same model in two separate chats, each seeing only what the other wrote. One of
them is given a topic and opens; in the files its lines are "U:" and the other's are "A:".

Everything it produces goes into one folder named after the model, for example
data/friendly-chats-qwen3.5-4b/, with one file per topic.
Each file holds the conversations for that topic in this form:

    Entry 1
    U: first agent
    A: second agent
    U: ...

    Entry 2
    ...

The script goes through the topics in data/topics.txt in rounds: one conversation for each topic,
then the next round, until the total is reached. Stopped early, every topic has about the same
number. It can be started again: it counts what each file already holds and carries on.

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

# Both agents get this. Neither is told it is talking to a model.
CHAT_SYSTEM = """You are texting with a friend on your phone. Be as natural as possible: this must read like real text messages between two people.

Keep it SHORT. Real texts are short.
- Most of your messages are 3 to 12 words. Never more than two short sentences.
- One thought per message. No stories, no long explanations, no piling up details.
- A reply can be just a few words: "lol same", "oh nice", "no idea tbh", "wait really?".

These lines only show the length and tone to aim for. Do not copy them:
hey! how's your day going
not bad, just got home. you?
ugh, long one. finally on the couch
oh nice, what did you make?
honestly no clue, never tried it
haha fair enough

- Simple everyday words and contractions. Skipping capital letters or the final full stop is fine.
- Sound like a person, not like an assistant: no "How can I help", no "Great question", no summing up.
- React to what they just said. Do not end every message with a question; ask only when you are curious.
- If you do not know something, just say so.
- Plain text only. No lists, no markdown, no dashes for effect, no quotation marks around your message. Write only your next message."""

# Added for the one who opens.
OPENER = """

You start the conversation. The topic is: {topic}. Bring it up the way a person would, with a question, a thought or something from your day."""

# How many exchanges a conversation runs for; short ones are more common.
LENGTHS = [1, 2, 2, 3, 3, 3, 4, 4, 5, 6]

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
                reply = json.loads(r.read().decode())
            text = reply["message"]["content"]
            if reply.get("done_reason") == "length":
                # Cut off by the token limit: drop the sentence that was left unfinished.
                ends = [m.end() for m in re.finditer(r"[.!?](?=\s|$)", text)]
                if ends:
                    text = text[: ends[-1]]
            return text
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


def shorten(text, max_words):
    """A message that ran long, cut back to its first sentences. Models overshoot; people texting do not."""
    if len(text.split()) <= max_words:
        return text
    kept = []
    for sentence in re.split(r"(?<=[.!?])\s+", text):
        if kept and len(" ".join(kept + [sentence]).split()) > max_words:
            break
        kept.append(sentence)
    return " ".join(kept)


def clock(seconds):
    """Seconds as h:mm:ss."""
    seconds = int(max(seconds, 0))
    return f"{seconds // 3600}:{seconds % 3600 // 60:02d}:{seconds % 60:02d}"


def simulate(args, topic, rng, recent, show):
    """Hold one conversation. Returns its turns, or None if it did not come out usable.
    `show(speaker, text)` is called with each message as soon as it arrives."""
    opening = "Send your first message."
    if recent:
        opening += " Earlier chats on this topic opened with the lines below; start differently:\n" + "\n".join(f"- {r}" for r in recent)
    conv = []  # (speaker, text): "U" opens, "A" replies
    for _ in range(min(rng.choice(LENGTHS), args.max_turns)):
        # Each agent sees its own lines as "assistant" and the other's as "user".
        as_first = [("user", opening)] + [("assistant" if who == "U" else "user", said) for who, said in conv]
        said = shorten(clean(chat(args, CHAT_SYSTEM + OPENER.format(topic=topic), as_first, rng.randrange(2**31), args.max_tokens)), args.max_words)
        if not said:
            break
        conv.append(("U", said))
        show("U", said)
        as_second = [("user" if who == "U" else "assistant", said) for who, said in conv]
        reply = shorten(clean(chat(args, CHAT_SYSTEM, as_second, rng.randrange(2**31), args.max_tokens)), args.max_words)
        if not reply:
            conv.pop()
            break
        conv.append(("A", reply))
        show("A", reply)
    return conv if len(conv) >= 2 else None


def main():
    p = argparse.ArgumentParser(description="Simulate conversations with a local Ollama model, one Markdown file per topic.")
    p.add_argument("--total", type=int, default=10000, help="conversations in all, spread evenly over the topics (default 10000)")
    p.add_argument("--topics-file", default=str(HERE / "topics.txt"), help="one topic per line (default data/topics.txt)")
    p.add_argument("--topics", type=int, default=0, help="use only the first N topics (for a trial run)")
    p.add_argument("--out", default="", help="folder for this material, one Markdown file per topic (default data/friendly-chats-<model>, for example data/friendly-chats-qwen3.5-4b)")
    p.add_argument("--model", default="qwen3.5:4b", help="Ollama model tag (default qwen3.5:4b; `ollama list` shows what is installed)")
    p.add_argument("--host", default="http://127.0.0.1:11434", help="where Ollama listens")
    p.add_argument("--max-turns", type=int, default=6, help="most exchanges in one conversation; the length is drawn at random up to this (default 6)")
    p.add_argument("--temperature", type=float, default=0.9)
    p.add_argument("--max-tokens", type=int, default=60, help="limit on one message, in model tokens (default 60)")
    p.add_argument("--max-words", type=int, default=25, help="a longer message is cut back to its first sentences (default 25)")
    p.add_argument("--timeout", type=int, default=300, help="seconds to wait for one reply")
    p.add_argument("--seed", type=int, default=1, help="makes the run repeatable as far as the model allows")
    p.add_argument("--think", action="store_true", help="let a reasoning model think first (slower)")
    p.add_argument("--quiet", action="store_true", help="show one progress line per conversation instead of every message")
    args = p.parse_args()
    args.no_think_option = False

    topics = read_topics(args.topics_file)
    if args.topics:
        topics = topics[: args.topics]
    # One folder per model, so material from different models never mixes.
    out = Path(args.out) if args.out else HERE / ("friendly-chats-" + re.sub(r"[^a-z0-9.]+", "-", args.model.lower()).strip("-"))
    out.mkdir(parents=True, exist_ok=True)
    # Spread the total evenly; the first few topics take the remainder.
    share, extra = divmod(args.total, len(topics))
    wanted = [share + (1 if i < extra else 0) for i in range(len(topics))]
    width = len(str(len(topics)))
    files = [out / f"{i + 1:0{width}d}-{slug(t)}.md" for i, t in enumerate(topics)]

    before = sum(min(len(read_file(f)), w) for f, w in zip(files, wanted))
    todo = args.total - before
    print(f"model          {args.model} at {args.host}")
    print(f"saving to      {out}")
    print(f"plan           {args.total} conversations over {len(topics)} topics in rounds: one per topic, then again, about {share} each")
    print(f"already there  {before}" + ("  (carrying on from a previous run)" if before else ""))
    print(f"to do          {todo}")
    if todo <= 0:
        print("nothing left to do")
        return
    print("Ctrl+C stops it; the same command carries on later.\n", flush=True)

    started = time.time()
    made = messages = dropped = 0

    def show(who, said):
        nonlocal messages
        messages += 1
        if not args.quiet:
            print(f"    {who}: {said}", flush=True)

    all_convs = [read_file(f) for f in files]
    seen = [{" ".join(x.lower() for _, x in c) for c in convs} for convs in all_convs]
    given_up = set()
    for path, topic in zip(files, topics):
        if not path.exists():
            path.write_text(f"# {topic}\n", encoding="utf-8")

    # Rounds: one conversation per topic, then the next round. A topic that already has its
    # conversation for a round (from an earlier run) is passed over.
    rounds = max(wanted)
    for rnd in range(rounds):
        for ti, topic in enumerate(topics):
            convs = all_convs[ti]
            if rnd >= wanted[ti] or len(convs) > rnd or ti in given_up:
                continue
            failures = 0
            while len(convs) <= rnd:
                if not args.quiet:
                    print(f"  round {rnd + 1}/{rounds}, topic {ti + 1}/{len(topics)} \"{topic}\"", flush=True)
                # Seeded by topic and position, so a restarted run draws the same lengths and seeds.
                rng = random.Random(f"{args.seed}/{topic}/{len(convs)}/{failures}")
                conv = simulate(args, topic, rng, [c[0][1] for c in convs[-6:]], show)
                key = " ".join(x.lower() for _, x in conv) if conv else None
                if conv is None or key in seen[ti]:
                    dropped += 1
                    failures += 1
                    print("    (not kept: " + ("empty" if conv is None else "same as an earlier one") + ")", flush=True)
                    if failures >= 8:
                        print(f"  leaving out \"{topic}\" after 8 unusable conversations in a row; it has {len(convs)} of {wanted[ti]}")
                        given_up.add(ti)
                        break
                    continue
                convs.append(conv)
                seen[ti].add(key)
                with files[ti].open("a", encoding="utf-8") as f:
                    f.write(f"\nEntry {len(convs)}\n" + "".join(f"{who}: {said}\n" for who, said in conv))
                made += 1
                passed = time.time() - started
                done = before + made
                left = (todo - made) * passed / made
                status = (f"[{done}/{args.total}  {100 * done / args.total:.1f}%]  passed {clock(passed)}  about {clock(left)} left  "
                          f"{made / passed * 60:.1f} conversations/min  {passed / max(messages, 1):.1f} s per message")
                if args.quiet:
                    status += f"  round {rnd + 1}/{rounds}, topic {ti + 1}/{len(topics)} \"{topic[:30]}\""
                print(status + ("" if args.quiet else "\n"), flush=True)

    written = sum(len(read_file(f)) for f in files)
    print(f"finished in {clock(time.time() - started)}: {made} conversations and {messages} messages this run"
          + (f", {dropped} not kept" if dropped else "") + f"; {written} of {args.total} now in {out}")


if __name__ == "__main__":
    try:
        main()
    except Stop as e:
        sys.exit(f"\n{e}")
    except KeyboardInterrupt:
        print("\nstopped; run the same command again to carry on")
