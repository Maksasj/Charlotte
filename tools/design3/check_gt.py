"""Validate v3 ground-truth scenario files (research/DESIGN_3_schema.md section 8).

Usage: python3 -I tools/design3/check_gt.py [files or dirs ...]   (default: data/curriculum3)
Exit code 1 if any error. Calls no model.
"""
import os
import re
import sys
from collections import defaultdict

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

ARITY = {
    "task": 1, "step": 2, "goal": 2, "domain": 2,
    "needs": 2, "optional_need": 2, "serves": 2, "takes_time": 4,
    "action": 2, "object_of": 2, "uses": 2, "role": 3, "into": 2, "onto": 2, "from": 2,
    "with_part": 2, "setting": 4, "amount": 4, "until": 2, "repeat": 2,
    "before": 2, "overlaps": 2, "optional": 1, "alternative": 2,
    "purpose": 2, "achieves": 2, "requires_state": 2, "condition": 2, "warning": 2,
    "avoid": 3, "check": 2,
    "alias": 2, "task_alias": 2,
}
SITUATION = {"lacks": 2, "has": 2, "wants": 2, "excludes": 2, "time_limit": 3, "count": 3}
DIFF = {"swap": 3, "drop": 1, "add_step": 2, "change": 4, "extra_need": 1, "no_need": 1, "cannot": 1}
DOMAINS = {"cooking", "cleaning", "repair", "planning", "errands", "personal_care", "gardening",
           "travel", "crafts"}
SYM = re.compile(r"^[a-z0-9][a-z0-9_]*$")
NUM = re.compile(r"^-?[0-9]+(\.[0-9]+)?$")
ATOM = re.compile(r"^([a-z_]+)\((.*)\)\.$")


def split_args(s):
    out, cur, q = [], "", False
    for ch in s:
        if ch == '"':
            q = not q
            cur += ch
        elif ch == "," and not q:
            out.append(cur.strip())
            cur = ""
        else:
            cur += ch
    if cur.strip():
        out.append(cur.strip())
    return out


def parse(path):
    """Return dict: sections (name -> list of lines), blocks (list of (section, label, atoms))."""
    sec = None
    sections = defaultdict(list)
    blocks = []
    block = None
    head = {}
    lines = open(path, encoding="utf-8").read().splitlines()
    if lines and lines[0] == "---":
        i = 1
        while i < len(lines) and lines[i] != "---":
            if ":" in lines[i]:
                k, v = lines[i].split(":", 1)
                head[k.strip()] = v.strip()
            i += 1
    for n, line in enumerate(lines, 1):
        if block is not None:
            if line.startswith("```"):
                blocks.append(block)
                block = None
                continue
            t = line.strip()
            if not t or t.startswith("%"):
                continue
            m = ATOM.match(t)
            if not m:
                block[2].append(("__bad__", (t,), n))
            else:
                block[2].append((m.group(1), tuple(split_args(m.group(2))), n))
            continue
        if line.startswith("```"):
            label = line[3:].strip().split()
            block = (sec, label[1] if len(label) > 1 else "", [])
            continue
        if line.startswith("## "):
            sec = line[3:].strip()
            sections[sec]  # create
        elif sec:
            sections[sec].append(line)
    return head, sections, blocks


def check(path):
    errs = []
    head, sections, blocks = parse(path)
    E = lambda n, msg: errs.append(f"{os.path.relpath(path, ROOT)}:{n}: {msg}")
    task = head.get("task") or head.get("id")
    gt = [b for b in blocks if b[0] == "ground truth"]
    if len(gt) != 1:
        E(0, f"expected one ground truth block, found {len(gt)}")
        return errs
    atoms = gt[0][2]
    for sname in ("requests", "teach", "aliases"):
        if sname not in sections and not (sname == "aliases" and any(b[0] == "aliases" for b in blocks)):
            if head.get("source") != "seed" or sname == "aliases":
                E(0, f"missing section ## {sname}")
    alias_atoms = [a for b in blocks if b[0] == "aliases" for a in b[2]]
    by = defaultdict(list)
    for p, args, n in atoms + alias_atoms:
        if p == "__bad__":
            E(n, f"not an atom: {args[0]}")
            continue
        if p not in ARITY:
            E(n, f"unknown predicate {p}")
            continue
        if len(args) != ARITY[p]:
            E(n, f"{p} needs {ARITY[p]} args, got {len(args)}")
            continue
        for k, a in enumerate(args):
            if p in ("alias", "task_alias") and k == 1:
                if not (a.startswith('"') and a.endswith('"')):
                    E(n, f"{p} text must be quoted")
                continue
            if not (SYM.match(a) or NUM.match(a)):
                E(n, f"bad symbol {a!r} in {p}")
        by[p].append((args, n))
    tasks = [a[0] for a, _ in by["task"]]
    if tasks != [task]:
        E(0, f"task facts {tasks} != file task {task}")
    doms = [a[1] for a, _ in by["domain"]]
    if len(doms) != 1 or doms[0] not in DOMAINS:
        E(0, f"need exactly one domain from {sorted(DOMAINS)}, got {doms}")
    steps = [a[1] for a, _ in by["step"]]
    sset = set(steps)
    for s in steps:
        if not re.match(rf"^{re.escape(task)}_s[0-9]+$", s):
            E(0, f"step id {s} must be {task}_sN")
    acts = defaultdict(int)
    for (s, _), n in by["action"]:
        acts[s] += 1
    objs = defaultdict(int)
    for (s, _), n in by["object_of"]:
        objs[s] += 1
    for s in steps:
        if acts[s] != 1:
            E(0, f"step {s} has {acts[s]} action facts (need exactly 1)")
        if objs[s] > 1:
            E(0, f"step {s} has {objs[s]} object_of facts")
    step_preds = {"action", "object_of", "uses", "role", "into", "onto", "from", "with_part",
                  "setting", "amount", "until", "repeat", "optional", "purpose", "achieves",
                  "requires_state", "condition", "warning", "avoid", "check"}
    for p in step_preds:
        for args, n in by[p]:
            if args[0] not in sset:
                E(n, f"{p} refers to unknown step {args[0]}")
    for p in ("before", "overlaps", "alternative"):
        for args, n in by[p]:
            for s in args:
                if s not in sset:
                    E(n, f"{p} refers to unknown step {s}")
    for args, n in by["takes_time"]:
        if args[0] not in sset and args[0] != task:
            E(n, f"takes_time on unknown {args[0]}")
        if NUM.match(args[1]) and NUM.match(args[2]) and float(args[1]) > float(args[2]):
            E(n, "takes_time min > max")
    uses = {(a[0], a[1]) for a, _ in by["uses"]}
    roles = {(a[0], a[1]) for a, _ in by["role"]}
    for su in sorted(uses - roles):
        E(0, f"uses{su} has no role fact")
    for su in sorted(roles - uses):
        E(0, f"role on {su} without uses")
    needs = {a[1] for a, _ in by["needs"]} | {a[1] for a, _ in by["optional_need"]}
    for (s, i) in sorted(uses):
        if i not in needs:
            E(0, f"item {i} used in {s} is not in needs/optional_need")
    # order: acyclic
    g = defaultdict(set)
    for (a, b), _ in by["before"]:
        g[a].add(b)
    state = {}

    def dfs(u):
        state[u] = 1
        for v in g[u]:
            if state.get(v) == 1:
                return True
            if v not in state and dfs(v):
                return True
        state[u] = 2
        return False
    for u in list(g):
        if u not in state and dfs(u):
            E(0, "before facts form a cycle")
            break
    # every step reachable in order (except overlapping/alternative/first)
    if len(steps) > 1:
        linked = set()
        for p in ("before", "overlaps", "alternative"):
            for args, _ in by[p]:
                linked.update(args)
        for s in steps:
            if s not in linked:
                E(0, f"step {s} has no order fact")
    # teach coverage: each taught step number must exist
    teach = [l[2:] for l in sections.get("teach", []) if l.startswith("- ")]
    nums = set()
    for l in teach:
        for m in re.finditer(r"\bstep (\d+)\b", l.lower()):
            nums.add(int(m.group(1)))
    # variants
    vsit = [b for b in blocks if b[0] == "variant" and b[1] == "situation"]
    vdif = [b for b in blocks if b[0] == "variant" and b[1] == "difference"]
    nvar = sum(1 for l in sections.get("variant", []) if l.startswith("- change:"))
    if nvar and (len(vsit) != nvar or len(vdif) != nvar):
        E(0, f"{nvar} variant(s) but {len(vsit)} situation and {len(vdif)} difference blocks")
    for b in vsit:
        for p, args, n in b[2]:
            if p not in SITUATION or len(args) != SITUATION[p] or args[0] != "user":
                E(n, f"bad situation fact {p}{args}")
    for b in vdif:
        newsteps = set()
        for p, args, n in b[2]:
            if p == "add_step":
                newsteps.add(args[0])
        for p, args, n in b[2]:
            if p in DIFF:
                if len(args) != DIFF[p]:
                    E(n, f"{p} needs {DIFF[p]} args")
                    continue
                if p in ("swap", "drop", "change") and args[0] not in sset:
                    E(n, f"{p} refers to unknown step {args[0]}")
                if p == "add_step":
                    if args[1] != "start" and args[1] not in sset | newsteps:
                        E(n, f"add_step after unknown step {args[1]}")
                    m = re.match(rf"^{re.escape(task)}_s([0-9]+)$", args[0])
                    if not m or int(m.group(1)) < 9:
                        E(n, f"new step {args[0]} must be {task}_s9 or higher")
            elif p in ARITY and p in step_preds | {"before", "overlaps", "step", "takes_time"}:
                ok = args[0] in newsteps or (p == "step" and args[1] in newsteps)
                if p in ("before", "overlaps"):
                    ok = any(a in newsteps for a in args)
                if not ok:
                    E(n, f"{p} in a difference block must describe a new step")
            else:
                E(n, f"unknown difference fact {p}")
    return errs


def main(argv):
    paths = argv or [os.path.join(ROOT, "data", "curriculum3")]
    files = []
    for p in paths:
        if os.path.isdir(p):
            for d, _, fs in os.walk(p):
                files += [os.path.join(d, f) for f in fs if f.endswith(".md")]
        else:
            files.append(p)
    errs = []
    for f in sorted(files):
        errs += check(f)
    for e in errs:
        print(e)
    print(f"{len(files)} files, {len(errs)} errors")
    return 1 if errs else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
