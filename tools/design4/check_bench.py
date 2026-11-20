"""Check derivation-benchmark files (tools/design4/BENCH_SPEC.md, design 10.1).

Uses the independent B1-B9 reasoner of tools/design4/check_fixture.py (written from the
design, not from the agent's code). For every question:
  - fields and types are present; pieces, given and gold atoms parse;
  - `answer` golds derive from pieces + given at exactly the stated hops;
  - `gap` golds: the completed atoms are underivable, and derivable once `missing` is added;
  - `hyp` golds have no base-rule proof and min_support >= 3;
  - every gold atom is in the question's deny list.
Across the files: no denied atom of one question is a piece of another; no piece appears in
more than 5% of the questions; no two queries are identical; composition per type and hop.

Usage: python3 -I tools/design4/check_bench.py FILE.jsonl [FILE.jsonl ...]
Exit 1 on any error. Zero model calls.
"""
import json
import os
import sys
from collections import Counter, defaultdict

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from check_fixture import Reasoner, astr, parse_atom, parse_query, split_statements  # noqa: E402

TYPES = {"K", "SUB", "X", "EXC", "STEP", "NAME", "IND", "GAP"}


def canon_map(atoms):
    parent = {}

    def find(x):
        while parent.get(x, x) != x:
            x = parent[x]
        return x
    for neg, p, args in atoms:
        if p == "same_as" and not neg and len(args) == 2:
            a, b = find(args[0]), find(args[1])
            if a != b:
                parent[a] = b
    return find


def canon(atom, find):
    neg, p, args = atom
    return (neg, p, tuple(find(a) for a in args))


# relations a teacher could state; internal proof nodes (holds, stand_in, ...) are not deny-able text
SURFACE = {"is_a", "has_part", "made_of", "has_property", "can_do", "can_do_on", "used_for", "can_go_in",
           "effect", "precondition", "achieves", "requires_state", "demands", "role", "uses"}


def canon_rev(atoms):
    """canonical name -> one surface name, from same_as pieces"""
    find = canon_map(atoms)
    out = {}
    for neg, p, args in atoms:
        for a in args:
            out.setdefault(a, find(a))
    return out


def variants(node, inv):
    """surface spellings of a canonical node: each argument replaced by any of its names"""
    neg, p, args = node
    names = [[a] + [k for k, v in inv.items() if v == a and k != a] for a in args]
    out = [""]
    for opts in names:
        out = [o + ("," if o else "") + x for o in out for x in opts]
    return [f"{p}({o})" for o in out]


def pieces_of(q):
    out = []
    for s in q.get("pieces", []):
        for st in split_statements(s):
            a = parse_atom(st)
            if a is None:
                raise ValueError(f"piece does not parse: {st}")
            out.append(a)
    return out


def check_question(q):
    errs = []
    for f in ("id", "type", "query", "gold", "pieces", "deny", "kb_scope"):
        if f not in q:
            errs.append(f"missing field {f}")
    if errs:
        return errs
    if q["type"] not in TYPES:
        errs.append(f"unknown type {q['type']}")
    try:
        pieces = pieces_of(q)
    except ValueError as e:
        return [str(e)]
    goal, given = parse_query(q["query"])
    if goal is None or any(g is None for g in given):
        return ["query does not parse"]
    find = canon_map(pieces + list(given))
    facts = [(canon(a, find), f"p{i}") for i, a in enumerate(pieces) if a[1] != "same_as"]
    gv = [canon(a, find) for a in given if a[1] != "same_as"]
    rs = Reasoner(facts, gv)
    gold = q["gold"]
    atoms = []
    for s in gold.get("atoms", []):
        a = parse_atom(s.strip().rstrip("."))
        if a is None:
            errs.append(f"gold atom does not parse: {s}")
        else:
            atoms.append(canon(a, find))
    deny = {s.replace(" ", "").rstrip(".") for s in q.get("deny", [])}
    deny_pos = {d[3:] if d.startswith("not") else d for d in deny}
    for s in gold.get("atoms", []):
        if gold["kind"] != "gap" and s.replace(" ", "").rstrip(".") not in deny:
            errs.append(f"gold atom {s} not in deny")
    kind = gold.get("kind")
    if kind == "answer":
        for a in atoms:
            pr = rs.prove(a)
            if pr is None:
                errs.append(f"gold {astr(a)} does not derive from its pieces")
            elif q.get("hops") is not None and pr.cost != q["hops"]:
                errs.append(f"gold {astr(a)} derives at {pr.cost} hops, labelled {q['hops']}")
            else:
                # design 10.1: deny holds the answer and every intermediate node of the proof
                inv = {v: k for k, v in canon_rev(pieces + list(given)).items()}
                for node in pr.inner():
                    if not isinstance(node, tuple) or len(node) != 3 or node[1] not in SURFACE:
                        continue
                    k = astr((False, node[1], node[2])).replace(" ", "")
                    if k not in deny_pos and not any(k2 in deny_pos for k2 in variants(node, inv)):
                        errs.append(f"intermediate {k} of the gold proof is not in deny")
    elif kind == "gap":
        missing = []
        for s in gold.get("missing", []):
            m = parse_atom(s.strip().rstrip("."))
            if m is None:
                errs.append(f"missing piece does not parse: {s}")
            else:
                missing.append(canon(m, find))
        completes = []
        for s in gold.get("completes", []):
            c = parse_atom(s.strip().rstrip("."))
            if c is not None:
                completes.append(canon(c, find))
        for c in completes:
            if rs.prove(c) is not None:
                errs.append(f"gap: {astr(c)} already derives without the missing pieces")
        if missing and completes:
            rs2 = Reasoner(facts + [(m, "missing") for m in missing], gv)
            for c in completes:
                if rs2.prove(c) is None:
                    errs.append(f"gap: {astr(c)} does not derive even with the missing pieces")
        if not missing:
            errs.append("gap without missing pieces")
    elif kind == "hyp":
        for a in atoms:
            if rs.prove(a) is not None:
                errs.append(f"hyp gold {astr(a)} has a base-rule proof")
        if gold.get("min_support", 0) < 3:
            errs.append("hyp gold needs min_support >= 3")
    else:
        errs.append(f"unknown gold kind {kind}")
    return errs


def main(paths):
    qs = []
    for p in paths:
        for n, l in enumerate(open(p), 1):
            if l.strip():
                try:
                    q = json.loads(l)
                except ValueError as e:
                    print(f"{p}:{n}: not JSON: {e}")
                    continue
                q["_src"] = f"{os.path.basename(p)}:{n}"
                qs.append(q)
    errors = 0
    for q in qs:
        for e in check_question(q):
            errors += 1
            print(f"{q['_src']} {q.get('id')}: {e}")
    # cross-question checks
    piece_count = Counter()
    deny_by = {}
    for q in qs:
        ps = {s.replace(" ", "").rstrip(".") for s in q.get("pieces", [])}
        for s in ps:
            piece_count[s] += 1
        for d in q.get("deny", []):
            deny_by[d.replace(" ", "").rstrip(".")] = q.get("id")
    for q in qs:
        for s in q.get("pieces", []):
            k = s.replace(" ", "").rstrip(".")
            if k in deny_by and deny_by[k] != q.get("id"):
                errors += 1
                print(f"{q['_src']} {q.get('id')}: piece {s} is a denied atom of {deny_by[k]}")
    limit = max(5, int(0.05 * len(qs)))
    for s, c in piece_count.items():
        if c > limit:
            errors += 1
            print(f"piece {s} appears in {c} questions (limit {limit})")
    # one sort per symbol across all questions: registry, `new` fields and argument positions
    POS = {("made_of", 1): "material", ("has_property", 1): "property", ("can_go_in", 1): "place",
           ("can_do", 1): "function", ("used_for", 1): "function", ("can_do_on", 1): "function",
           ("effect", 0): "action", ("effect", 2): "state", ("precondition", 0): "action", ("precondition", 2): "state"}
    reg = {}
    rp = os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))), "data", "bench4", "registry.jsonl")
    if os.path.exists(rp):
        for l in open(rp):
            r = json.loads(l)
            reg[r["sym"]] = r["sort"]
    seen = defaultdict(lambda: defaultdict(set))  # sym -> sort -> question ids
    compatible = {frozenset(("item", "place")), frozenset(("action", "function"))}
    for q in qs:
        for d in q.get("new", []):
            if ":" in d:
                sym, srt = d.split(":", 1)
                seen[sym.strip()][srt.strip()].add(q.get("id"))
        try:
            atoms = pieces_of(q)
        except ValueError:
            atoms = []
        for neg, pr, args in atoms:
            for i, a in enumerate(args):
                if (pr, i) in POS:
                    seen[a][POS[(pr, i)]].add(q.get("id"))
    for sym, by in sorted(seen.items()):
        sorts = set(by) | ({reg[sym]} if sym in reg else set())
        clash = [frozenset((x, y)) for x in sorts for y in sorts if x < y and frozenset((x, y)) not in compatible]
        if clash:
            errors += 1
            where = "; ".join(f"{k}: {sorted(v)[:4]}" for k, v in sorted(by.items()))
            print(f"symbol {sym} has sorts {sorted(sorts)}{' (registry ' + reg[sym] + ')' if sym in reg else ''}: {where}")
    dup = Counter(q["query"].replace(" ", "") for q in qs)
    for k, c in dup.items():
        if c > 1:
            errors += 1
            print(f"duplicate query ({c}x): {k[:80]}")
    comp = defaultdict(Counter)
    for q in qs:
        comp[q.get("type")][q.get("hops") if q.get("hops") is not None else "-"] += 1
    print(json.dumps({"questions": len(qs), "errors": errors, "composition": {t: dict(c) for t, c in sorted(comp.items())},
                      "kb_scope": dict(Counter(q.get("kb_scope") for q in qs))}))
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
