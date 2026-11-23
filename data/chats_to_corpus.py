#!/usr/bin/env python3
"""Turn generated chats into a file the engine can be taught from.

Reads the Markdown files written by generate_conversations.py (one per topic, "Entry N" blocks of
"U:" and "A:" lines) and writes one JSON-lines file, one conversation per line:

    {"id": "weekend-plans-1", "domain": "weekend plans",
     "turns": [{"role": "user", "text": "..."}, {"role": "agent", "text": "..."}]}

    python3 data/chats_to_corpus.py                                  # every data/raw/friendly-chats-* folder
    python3 data/chats_to_corpus.py data/raw/friendly-chats-qwen3.5-4b   # one folder

The result is written to data/processed/<folder>.jsonl. Teach it with:

    software/target/release/charlotte learn data/processed/<folder>.jsonl
"""

import json
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ENTRY = re.compile(r"^\s*entry\s+\d+\s*$", re.IGNORECASE)
TURN = re.compile(r"^([UA]):\s?(.*)$")


def read_topic(path):
    """The topic title and the conversations in one file, each a list of (speaker, text)."""
    topic, convs = None, []
    for line in path.read_text(encoding="utf-8").splitlines():
        if topic is None and line.startswith("# "):
            topic = line[2:].strip()
        elif ENTRY.match(line):
            convs.append([])
        else:
            m = TURN.match(line.strip())
            if m and convs:
                convs[-1].append((m.group(1), " ".join(m.group(2).split())))
    return topic or path.stem, convs


def usable(conv):
    """The conversation up to its last agent reply, or None if it is not a proper exchange."""
    conv = [(who, said) for who, said in conv if said]
    while conv and conv[-1][0] != "A":
        conv.pop()
    speakers = [who for who, _ in conv]
    if len(conv) < 2 or speakers[0] != "U" or any(a == b for a, b in zip(speakers, speakers[1:])):
        return None
    return conv


def convert(folder):
    files = sorted(folder.glob("*.md"))
    if not files:
        print(f"{folder}: no Markdown files")
        return
    out = HERE / "processed" / (folder.name + ".jsonl")
    out.parent.mkdir(parents=True, exist_ok=True)
    kept = dropped = topics = messages = 0
    with out.open("w", encoding="utf-8") as f:
        for path in files:
            topic, convs = read_topic(path)
            slug = re.sub(r"[^a-z0-9]+", "-", topic.lower()).strip("-")
            wrote = 0
            for n, conv in enumerate(convs, 1):
                conv = usable(conv)
                if conv is None:
                    dropped += 1
                    continue
                turns = [{"role": "user" if who == "U" else "agent", "text": said} for who, said in conv]
                f.write(json.dumps({"id": f"{slug}-{n}", "domain": topic, "turns": turns}, ensure_ascii=False) + "\n")
                wrote += 1
                messages += len(turns)
            kept += wrote
            topics += 1 if wrote else 0
    try:
        shown = out.relative_to(Path.cwd())
    except ValueError:
        shown = out
    print(f"{folder.name}: {kept} conversations ({messages} messages) on {topics} topics" + (f", {dropped} left out as malformed" if dropped else ""))
    print(f"  written to {shown}")
    print(f"  teach it:  software/target/release/charlotte learn {shown}")


def main():
    folders = [Path(a) for a in sys.argv[1:]] or sorted(p for p in (HERE / "raw").glob("friendly-chats-*") if p.is_dir())
    if not folders:
        sys.exit("no data/raw/friendly-chats-* folder found; run generate_conversations.py first, or give a folder")
    for folder in folders:
        if not folder.is_dir():
            sys.exit(f"not a folder: {folder}")
        convert(folder)


if __name__ == "__main__":
    main()
