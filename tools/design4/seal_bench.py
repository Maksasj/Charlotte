"""Split and seal the derivation benchmark (design 10.1, milestone 35).

Reads data/bench4/pool/*.jsonl (questions kept after review), splits them at random, with a
committed seed, stratified by type x hops x kb_scope, into 300 dev and 600 sealed. Writes:
  data/bench4/dev.jsonl            readable
  data/bench4/sealed.jsonl         sealed (hashed in MANIFEST.sha256; read at most 3 times)
  data/bench4/deny.txt             dev deny atoms, plain (the curriculum's filter)
  data/bench4/deny_sealed.sha256   sealed deny atoms, as sha256 of the canonical atom
  data/bench4/MANIFEST.sha256      hashes of everything
Usage: python3 -I tools/design4/seal_bench.py [--seed 4035] [--keep data/bench4/review/kept.txt]
"""
import hashlib
import json
import os
import random
import sys
from collections import defaultdict

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
B = os.path.join(ROOT, "data", "bench4")


def key(s):
    t = "".join(s.strip().rstrip(".").split())
    return t[3:] if t.startswith("not") else t


def main():
    seed = int(sys.argv[sys.argv.index("--seed") + 1]) if "--seed" in sys.argv else 4035
    keep = None
    if "--keep" in sys.argv:
        keep = set(open(sys.argv[sys.argv.index("--keep") + 1]).read().split())
    qs = []
    for f in sorted(os.listdir(os.path.join(B, "pool"))):
        if f.endswith(".jsonl"):
            for l in open(os.path.join(B, "pool", f)):
                if l.strip():
                    q = json.loads(l)
                    if keep is None or q["id"] in keep:
                        qs.append(q)
    strata = defaultdict(list)
    for q in qs:
        strata[(q["type"], q.get("hops"), q.get("kb_scope"))].append(q)
    rnd = random.Random(seed)
    keys = sorted(strata, key=str)
    for k in keys:
        rnd.shuffle(strata[k])
    # largest remainder: each stratum gets floor(n/3) dev places; the rest go to the largest
    # remainders (ties broken by the seed) until dev is exactly a third of the pool
    target = len(qs) // 3
    n_dev = {k: len(strata[k]) // 3 for k in keys}
    tie = {k: rnd.random() for k in keys}
    for k in sorted(keys, key=lambda k: (-(len(strata[k]) % 3), tie[k]))[:target - sum(n_dev.values())]:
        n_dev[k] += 1
    dev, sealed = [], []
    for k in keys:
        dev.extend(strata[k][:n_dev[k]])
        sealed.extend(strata[k][n_dev[k]:])
    def write(name, rows):
        with open(os.path.join(B, name), "w") as f:
            for r in sorted(rows, key=lambda q: q["id"]):
                f.write(json.dumps(r) + "\n")
    write("dev.jsonl", dev)
    write("sealed.jsonl", sealed)
    dev_deny = sorted({key(d) for q in dev for d in q.get("deny", [])} | {key(d) for q in dev for d in q.get("gold", {}).get("missing", [])})
    open(os.path.join(B, "deny.txt"), "w").write("\n".join(dev_deny) + "\n")
    sealed_deny = sorted({key(d) for q in sealed for d in q.get("deny", [])} | {key(d) for q in sealed for d in q.get("gold", {}).get("missing", [])})
    open(os.path.join(B, "deny_sealed.sha256"), "w").write("\n".join("sha256:" + hashlib.sha256(d.encode()).hexdigest() for d in sealed_deny) + "\n")
    rows = []
    for f in sorted(os.listdir(B)):
        p = os.path.join(B, f)
        if os.path.isfile(p) and f != "MANIFEST.sha256":
            rows.append(f"{hashlib.sha256(open(p, 'rb').read()).hexdigest()}  {f}")
    open(os.path.join(B, "MANIFEST.sha256"), "w").write("\n".join(rows) + "\n")
    print(json.dumps({"pool": len(qs), "dev": len(dev), "sealed": len(sealed), "dev_deny": len(dev_deny), "sealed_deny_hashed": len(sealed_deny), "seed": seed}))


if __name__ == "__main__":
    main()
