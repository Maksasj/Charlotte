"""Cached, budgeted Haiku calls for Design 3.0 (decision D11).

Every call goes through the Claude CLI. Every answer is archived in
data/training/calls3/cache.jsonl, keyed by sha256(model, system, prompt), and never deleted.
A repeated call is served from the archive and costs nothing, so every call can be reused
for later learning or testing. A ledger counts live calls against the budget.

    from haiku import ask
    text = ask(system, prompt, tag="x7_paraphrase")

Budget: 5,000 live calls (D11). ask() raises BudgetExceeded past it.
"""
import hashlib
import json
import os
import subprocess
import time

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
DIR = os.path.join(ROOT, "data", "training", "calls3")
CACHE = os.path.join(DIR, "cache.jsonl")
LEDGER = os.path.join(DIR, "ledger.jsonl")
BUDGET = 5000
MODEL = "haiku"


class BudgetExceeded(RuntimeError):
    pass


_cache = None


def _key(model, system, prompt):
    return hashlib.sha256(json.dumps([model, system, prompt]).encode()).hexdigest()


def _load():
    global _cache
    if _cache is None:
        _cache = {}
        if os.path.exists(CACHE):
            for line in open(CACHE, encoding="utf-8"):
                try:
                    d = json.loads(line)
                    if d.get("ok"):
                        _cache[d["key"]] = d["answer"]
                except ValueError:
                    pass
    return _cache


def live_calls():
    if not os.path.exists(LEDGER):
        return 0
    return sum(1 for _ in open(LEDGER, encoding="utf-8"))


def ask(system, prompt, tag, model=MODEL, timeout=180):
    os.makedirs(DIR, exist_ok=True)
    k = _key(model, system, prompt)
    c = _load()
    if k in c:
        return c[k]
    if live_calls() >= BUDGET:
        raise BudgetExceeded(f"{BUDGET} live Haiku calls used; ask the user before going past it")
    t0 = time.time()
    try:
        p = subprocess.run(
            ["claude", "-p", "--model", model, "--output-format", "json", "--system-prompt", system,
             "--tools", "", "--no-session-persistence", "--strict-mcp-config"],
            input=prompt, capture_output=True, text=True, timeout=timeout)
        out = json.loads(p.stdout)
        answer = out.get("result", "")
        ok = not out.get("is_error", False) and bool(answer)
    except (subprocess.TimeoutExpired, ValueError) as e:
        answer, ok = f"error: {e}", False
    rec = {"key": k, "unix": int(t0), "tag": tag, "model": model, "system": system, "prompt": prompt,
           "answer": answer, "ok": ok, "seconds": round(time.time() - t0, 2)}
    with open(CACHE, "a", encoding="utf-8") as f:
        f.write(json.dumps(rec) + "\n")
    with open(LEDGER, "a", encoding="utf-8") as f:
        f.write(json.dumps({"unix": int(t0), "tag": tag, "ok": ok}) + "\n")
    if ok:
        c[k] = answer
        return answer
    raise RuntimeError(answer)


if __name__ == "__main__":
    print(f"live calls used: {live_calls()} of {BUDGET}; cached answers: {len(_load())}")
