"""Replies of the milestone 12 build to the held-out second-encounter requests.

    python tools/m12_baseline.py SLA_BINARY

SLA_BINARY is the `sla` chat binary built from the milestone 12 commit
(245609f). It gets the same teaching as a curriculum run (every held-out
scenario's teach lines, as the teacher), then, in a fresh session on the same
vault, each scenario's second-encounter request. The replies are written to
eval/curriculum/m12_baseline.json for `sla_eval pairwise`.
"""
import glob
import json
import os
import subprocess
import sys
import tempfile

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def scenarios():
    out = []
    for f in sorted(glob.glob(os.path.join(ROOT, "data", "curriculum", "scenarios", "*.md"))):
        sid = os.path.basename(f)[:-3]
        reqs, teach, sec = [], [], ""
        for line in open(f, encoding="utf-8"):
            line = line.strip()
            if line.startswith("## "):
                sec = line[3:].strip().lower()
            elif line.startswith("- ") and sec == "requests":
                reqs.append(line[2:])
            elif line.startswith("- ") and sec == "teach":
                teach.append(line[2:])
        out.append((sid, reqs, teach))
    return out


def run(binary, vault, lines, source=None):
    with tempfile.NamedTemporaryFile("w", suffix=".txt", delete=False, encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")
        path = f.name
    args = [binary, "--vault", vault, "--script", path, "--llm"]
    if source:
        args += ["--source", source, "--speaker", "teacher"]
    out = subprocess.run(args, capture_output=True, text=True, encoding="utf-8", errors="replace").stdout
    os.unlink(path)
    # "> message" then the reply, possibly over several lines.
    replies, cur = [], None
    for line in out.splitlines():
        if line.startswith("> "):
            if cur is not None:
                replies.append(cur.strip())
            cur = ""
        elif cur is not None:
            cur += line + "\n"
    if cur is not None:
        replies.append(cur.strip())
    return replies


def main():
    binary = sys.argv[1]
    vault = tempfile.mkdtemp(prefix="sla_m12_baseline_")
    sc = scenarios()
    run(binary, vault, [l for _, _, t in sc for l in t], source="teacher:curriculum")
    asks = [r[1] if len(r) > 1 else r[0] for _, r, _ in sc]
    replies = run(binary, vault, asks)
    result = {sid: {"request": asks[i], "reply": replies[i] if i < len(replies) else ""} for i, (sid, _, _) in enumerate(sc)}
    out = os.path.join(ROOT, "eval", "curriculum", "m12_baseline.json")
    json.dump(result, open(out, "w", encoding="utf-8"), indent=1)
    print(f"wrote {out} ({sum(1 for v in result.values() if v['reply'])} replies); vault {vault}")


if __name__ == "__main__":
    main()
