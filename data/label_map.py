#!/usr/bin/env python3
"""Turns a dataset's own labels into labels the vault knows, so `charlotte learn` can use them.

    python3 data/label_map.py sgd          # data/processed/sgd.jsonl -> data/processed/sgd-labelled.jsonl

What is mapped is listed in label_map.json. A user turn gets `intent` from its user act
(THANK_YOU -> thanks) and `facts` from its slot values (amount "$1,630" -> amount 1630). A user turn with no mapped act is a negative example (`intent: none`), so a wording is only
learned when it does not also occur in turns without the label. Agent turns carry no labels.
"""
import json, re, sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
MAP = json.loads((HERE / "label_map.json").read_text())


def number(text):
    t = re.sub(r"[$,\s]", "", str(text))
    try:
        n = float(t)
    except ValueError:
        return None
    return int(n) if n == int(n) else n


def sgd():
    m, out, n, kept = MAP["sgd"], HERE / "processed" / "sgd-labelled.jsonl", 0, 0
    with open(HERE / "processed" / "sgd.jsonl") as src, out.open("w") as dst:
        for line in src:
            d, turns, any_label = json.loads(line), [], False
            n += 1
            for t in d["turns"]:
                turn = {"role": t["role"], "text": t["text"]}
                if t["role"] == "user":
                    intents, facts = [], {}
                    for lab in t["labels"]:
                        for a in lab["acts"]:
                            if a["act"] in m["user_acts"]:
                                intents.append(m["user_acts"][a["act"]])
                            slot = m["number_slots"].get(a.get("slot"))
                            if a["act"] == "INFORM" and slot and a.get("values"):
                                v = number(a["values"][0])
                                if v is not None:
                                    facts[slot] = v
                    # One mapped act is a label, none is a negative example ("none"), two are ambiguous: unlabelled.
                    if len(set(intents)) == 1:
                        turn["intent"] = intents[0]
                    elif not intents:
                        turn["intent"] = "none"
                    if facts:
                        turn["facts"] = facts
                    any_label = True
                turns.append(turn)
            if any_label:
                kept += 1
                dst.write(json.dumps({"id": d["id"], "domain": d["domain"], "split": d["split"], "turns": turns}, ensure_ascii=False) + "\n")
    print(f"sgd: {kept} of {n} conversations\n  written to {out}")


if __name__ == "__main__":
    known = {"sgd": sgd}
    if len(sys.argv) != 2 or sys.argv[1] not in known:
        sys.exit("usage: label_map.py sgd")
    known[sys.argv[1]]()
