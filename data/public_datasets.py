#!/usr/bin/env python3
"""Download two public dialogue datasets and convert them for training.

    DailyDialog              13,118 everyday dialogues          CC BY-NC-SA 4.0 (non-commercial)
    Schema-Guided Dialogue   22,825 task dialogues with labels  CC BY-SA 4.0

    python3 data/public_datasets.py                 # both: download if missing, then convert
    python3 data/public_datasets.py dailydialog     # one of them
    python3 data/public_datasets.py sgd

The originals go to data/raw/dailydialog/ and data/raw/sgd/, the converted files to
data/processed/dailydialog.jsonl and data/processed/sgd.jsonl. None of these are kept in git
(SGD alone is about 590 MB); run this script on each machine that needs them.

Converted form, one conversation per line, the same as our own sets:

    {"id": "...", "domain": "...", "split": "train",
     "turns": [{"role": "user", "text": "...", "labels": {...}}, {"role": "agent", ...}]}

`labels` carries each dataset's own annotations. The engine ignores that field for now: it reads
only role and text, so both sets are learned as plain chats until the labels are mapped onto the
vault's intents, slots and operations.
"""

import io
import json
import re
import sys
import urllib.request
import zipfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
RAW, OUT = HERE / "raw", HERE / "processed"

DD_URL = "https://huggingface.co/datasets/roskoN/dailydialog/resolve/main/{split}.zip"  # mirror of the original release
SGD_URL = "https://github.com/google-research-datasets/dstc8-schema-guided-dialogue/archive/refs/heads/master.zip"
DD_ACTS = {1: "inform", 2: "question", 3: "directive", 4: "commissive"}
DD_EMOTIONS = {0: "none", 1: "anger", 2: "disgust", 3: "fear", 4: "happiness", 5: "sadness", 6: "surprise"}


def fetch(url):
    print(f"  downloading {url}", flush=True)
    req = urllib.request.Request(url, headers={"User-Agent": "charlotte-data"})
    with urllib.request.urlopen(req, timeout=600) as r:
        return r.read()


def shown(path):
    try:
        return path.relative_to(Path.cwd())
    except ValueError:
        return path


# ---------- DailyDialog ----------

def tidy(text):
    """The release is tokenised ("Say , Jim , how about ... ?"); put the punctuation back."""
    text = " ".join(text.split())
    text = re.sub(r"\s+([,.?!;:])", r"\1", text)
    text = re.sub(r"(\w)\s+['’]\s+(\w)", r"\1'\2", text)
    return re.sub(r"([.?!,])(?=[A-Za-z]{2})", r"\1 ", text)


def dailydialog():
    raw = RAW / "dailydialog"
    if not (raw / "train" / "dialogues_train.txt").exists():
        for split in ("train", "validation", "test"):
            with zipfile.ZipFile(io.BytesIO(fetch(DD_URL.format(split=split)))) as z:
                for name in z.namelist():
                    if name.endswith(".txt"):
                        dest = raw / split / Path(name).name
                        dest.parent.mkdir(parents=True, exist_ok=True)
                        dest.write_bytes(z.read(name))
        (raw / "LICENSE.txt").write_text("DailyDialog (Li et al., 2017, https://aclanthology.org/I17-1099/) is licensed under CC BY-NC-SA 4.0.\nDownloaded from https://huggingface.co/datasets/roskoN/dailydialog, a mirror of the original release.\n")
    OUT.mkdir(exist_ok=True)
    out, n, turns = OUT / "dailydialog.jsonl", 0, 0
    with out.open("w", encoding="utf-8") as f:
        for split in ("train", "validation", "test"):
            rows = [(raw / split / f"dialogues_{kind}{split}.txt").read_text(encoding="utf-8").splitlines() for kind in ("", "act_", "emotion_")]
            for i, (text, acts, emotions) in enumerate(zip(*rows), 1):
                said = [tidy(u) for u in text.split("__eou__") if u.strip()]
                acts, emotions = acts.split(), emotions.split()
                if len(said) < 2:
                    continue
                conv = []
                for k, u in enumerate(said):
                    turn = {"role": "user" if k % 2 == 0 else "agent", "text": u}
                    if len(acts) == len(said) and len(emotions) == len(said):
                        turn["labels"] = {"act": DD_ACTS.get(int(acts[k])), "emotion": DD_EMOTIONS.get(int(emotions[k]))}
                    conv.append(turn)
                f.write(json.dumps({"id": f"dailydialog-{split}-{i:05d}", "domain": "daily conversation", "split": split, "turns": conv}, ensure_ascii=False) + "\n")
                n += 1
                turns += len(conv)
    print(f"dailydialog: {n} conversations ({turns} messages)\n  written to {shown(out)}")


# ---------- Schema-Guided Dialogue ----------

def sgd():
    raw = RAW / "sgd"
    if not (raw / "train" / "schema.json").exists():
        with zipfile.ZipFile(io.BytesIO(fetch(SGD_URL))) as z:
            for name in z.namelist():
                parts = Path(name).parts[1:]  # drop the archive's top folder
                keep = len(parts) == 2 and parts[0] in ("train", "dev", "test") and parts[1].endswith(".json")
                if keep or parts == ("LICENSE.txt",) or parts == ("README.md",):
                    dest = raw.joinpath(*parts)
                    dest.parent.mkdir(parents=True, exist_ok=True)
                    dest.write_bytes(z.read(name))
    OUT.mkdir(exist_ok=True)
    out, n, turns = OUT / "sgd.jsonl", 0, 0
    with out.open("w", encoding="utf-8") as f:
        for split in ("train", "dev", "test"):
            for path in sorted((raw / split).glob("dialogues_*.json")):
                for d in json.loads(path.read_text(encoding="utf-8")):
                    conv = []
                    for t in d["turns"]:
                        labels = []
                        for fr in t["frames"]:
                            lab = {"service": fr["service"], "acts": [{k: a[k] for k in ("act", "slot", "values") if a.get(k)} for a in fr.get("actions", [])]}
                            state = fr.get("state")
                            if state:
                                lab["intent"] = state.get("active_intent")
                                lab["slot_values"] = state.get("slot_values", {})
                            labels.append(lab)
                        conv.append({"role": "user" if t["speaker"] == "USER" else "agent", "text": " ".join(t["utterance"].split()), "labels": labels})
                    domain = ", ".join(sorted({s.rsplit("_", 1)[0].lower() for s in d["services"]}))
                    f.write(json.dumps({"id": f"sgd-{split}-{d['dialogue_id']}", "domain": domain, "split": split, "turns": conv}, ensure_ascii=False) + "\n")
                    n += 1
                    turns += len(conv)
    print(f"sgd: {n} conversations ({turns} messages)\n  written to {shown(out)}")


def main():
    known = {"dailydialog": dailydialog, "sgd": sgd}
    names = sys.argv[1:] or list(known)
    for name in names:
        if name not in known:
            sys.exit(f"unknown dataset '{name}'; choose from: {', '.join(known)}")
        known[name]()


if __name__ == "__main__":
    main()
