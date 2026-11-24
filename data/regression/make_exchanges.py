"""Samples real held-out exchanges for the regression set from DailyDialog's test split.

    python3 data/regression/make_exchanges.py          # appends to data/regression/cases.jsonl

Needs data/processed/dailydialog.jsonl (see data/public_datasets.py). Deterministic: the same
file gives the same cases. Rewordings are written by hand in cases.jsonl and kept.
"""
import json, random, pathlib

root = pathlib.Path(__file__).resolve().parents[2]
out = pathlib.Path(__file__).with_name("cases.jsonl")
N = 150
rng = random.Random(7)

kept = [l for l in out.read_text().splitlines() if l.strip() and json.loads(l)["kind"] != "exchange"] if out.exists() else []
convs = [json.loads(l) for l in open(root / "data/processed/dailydialog.jsonl")]
test = [c for c in convs if c["split"] == "test" and len(c["turns"]) >= 2]
rng.shuffle(test)
rows = []
for c in test:
    t = c["turns"]
    # A user message with an agent reply after it, anywhere in the conversation.
    options = [j for j in range(len(t) - 1) if t[j]["role"] == "user" and t[j + 1]["role"] == "agent"]
    if not options:
        continue
    i = rng.choice(options)
    hist = [[x["role"], x["text"]] for x in t[max(0, i - 4):i]]
    rows.append({"id": f"{c['id']}-{i}", "kind": "exchange", "source": "dailydialog-test", "history": hist, "say": t[i]["text"], "reply": t[i + 1]["text"]})
    if len(rows) == N:
        break
out.write_text("".join(l + "\n" for l in kept) + "".join(json.dumps(r) + "\n" for r in rows))
print(len(kept), "kept,", len(rows), "exchanges")
