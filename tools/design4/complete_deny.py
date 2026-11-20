"""Complete each question's deny list with the intermediate nodes of its gold proof (design 10.1).

Writers list the gold atoms; the intermediate nodes are computed here, mechanically, by the
independent reasoner of check_fixture.py, so the curriculum never teaches a step of a proof.
Added atoms are recorded in `deny_added`. Conflicts, where an added atom is another question's
piece, are reported (they would let teaching one question's piece shorten another's proof).

Usage: python3 -I tools/design4/complete_deny.py [--write] FILE.jsonl ...
Zero model calls.
"""
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from check_bench import SURFACE, canon, canon_map, canon_rev, pieces_of, variants  # noqa: E402
from check_fixture import Reasoner, parse_atom, parse_query  # noqa: E402


def inner_atoms(q):
    if q["gold"].get("kind") != "answer":
        return []
    pieces = pieces_of(q)
    goal, given = parse_query(q["query"])
    find = canon_map(pieces + list(given))
    facts = [(canon(a, find), f"p{i}") for i, a in enumerate(pieces) if a[1] != "same_as"]
    rs = Reasoner(facts, [canon(a, find) for a in given if a[1] != "same_as"])
    inv = {v: k for k, v in canon_rev(pieces + list(given)).items()}
    out = []
    for s in q["gold"]["atoms"]:
        a = parse_atom(s.strip().rstrip("."))
        pr = rs.prove(canon(a, find)) if a else None
        if pr is None:
            continue
        for node in pr.inner():
            if isinstance(node, tuple) and len(node) == 3 and node[1] in SURFACE:
                out.extend(variants(node, inv))
    return out


def norm(s):
    t = s.replace(" ", "").rstrip(".")
    return t[3:] if t.startswith("not") else t


def main(argv):
    write = "--write" in argv
    paths = [p for p in argv if not p.startswith("--")]
    data = {p: [json.loads(l) for l in open(p) if l.strip()] for p in paths}
    pieces_by = {}
    for p, qs in data.items():
        for q in qs:
            for s in q.get("pieces", []):
                pieces_by.setdefault(norm(s), set()).add(q["id"])
    added = conflicts = 0
    conf = []
    for p, qs in data.items():
        for q in qs:
            have = {norm(d) for d in q.get("deny", [])}
            new = []
            for k in inner_atoms(q):
                if k not in have and k not in new:
                    new.append(k)
            q["deny_added"] = new
            q["deny"] = list(q.get("deny", [])) + new
            added += len(new)
            for k in new:
                others = pieces_by.get(k, set()) - {q["id"]}
                if others:
                    conflicts += 1
                    conf.append((q["id"], k, sorted(others)))
        if write:
            with open(p, "w") as f:
                for q in qs:
                    f.write(json.dumps(q) + "\n")
    for c in conf[:30]:
        print("conflict", c)
    print(json.dumps({"questions": sum(len(v) for v in data.values()), "deny_added": added, "conflicts": conflicts,
                      "questions_in_conflict": len({c[0] for c in conf} | {o for c in conf for o in c[2]})}))


if __name__ == "__main__":
    main(sys.argv[1:])
