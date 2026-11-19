import os, re, sys
sys.path.insert(0, "/home/maksasj/Programming/Charlotte/tools/design4")
import check_fixture as cf
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = "/home/maksasj/Programming/Charlotte"
OUT = os.path.join(REPO, "data/spike4/teach")
os.makedirs(OUT, exist_ok=True)

def proc_body():
    spec = [l.split() for l in open(os.path.join(HERE, "body/procedures.spec")) if l.strip() and not l.startswith("#")]
    adds, cur = {}, None
    for l in open(os.path.join(HERE, "body/additions.dl")):
        l = l.rstrip("\n")
        if l.startswith("@"):
            cur = l[1:]; adds[cur] = []
        elif l.strip():
            adds[cur].append(l)
    out = ["% Design 4.0 spike fixture: PROCEDURES (marked fixture). Test-only (D4.3); channel `fixture`.",
           "% 8 procedures, 2 per domain, from data/curriculum3 train/dev. Each block copies the v3 ground-truth",
           "% facts task, step, action, object_of, uses, role, before, requires_state, achieves, needs verbatim",
           "% (settings, times, purposes, checks, aliases and into/overlaps are left out), then adds `demands` and,",
           "% where a STEP question needs them, achieves/requires_state/before lines marked `fixture addition`.",
           "% No archive3 reading was replayed: the fixture uses the ground truth instead (D4.3, Appendix A)."]
    for task, split, dom in spec:
        out.append("topic %s/%s." % (dom, task))
        out.append("%% from data/curriculum3/%s/%s.md, ## ground truth (core facts)" % (split, task))
        out += [l.strip() for l in open(os.path.join(HERE, "body/gt_%s.dl" % task)) if l.strip()]
        out.append("% fixture additions (not in the v3 ground truth)")
        out += adds.get(task, [])
    out += [l.rstrip("\n") for l in open(os.path.join(HERE, "body/cases.dl"))]
    return out

files = {}
for d in cf.DOMAINS:
    files[d + ".msgs"] = [l.rstrip("\n") for l in open(os.path.join(HERE, "body/%s.body" % d))]
files["procedures_fixture.msgs"] = proc_body()
PLANTED = "% planted sort error"

# infer sorts from argument positions
sorts = {}
def add(sym, s):
    if sym == "user": return
    sorts.setdefault(sym, set()).add(s)
pending = []
for fname in cf.TEACH_ORDER:
    for l in files[fname]:
        if PLANTED in l: continue
        b = l.split("%", 1)[0].strip()
        if not b or b.startswith("topic "): continue
        at = cf.parse_atom(b[:-1])
        assert at, (fname, l)
        neg, p, args = at
        sig = cf.SCHEMA[p]
        if p in ("is_a", "same_as"):
            pending.append((p, args)); continue
        for a, s in zip(args, sig):
            if s in ("agent", "REL"): continue
            if s == "VAL":
                s = cf.SCHEMA[args[2]][1]
            add(a, s)
for _ in range(3):
    for p, args in pending:
        known = [sorts.get(a, set()) - {"place"} for a in args]
        if p == "is_a":
            s = "action" if any("action" in k and "item" not in k for k in known) else "item"
        else:
            u = (known[0] | known[1]) or {"item"}
            s = sorted(u)[0]
        for a in args:
            if not (s == "item" and "place" in sorts.get(a, set())):
                add(a, s)
# place subsumes item
for k, v in sorts.items():
    if "place" in v: v.discard("item")

declared = {}
order_syms = {}
for fname in cf.TEACH_ORDER:
    new = []
    for l in files[fname]:
        b = l.split("%", 1)[0].strip()
        if not b or b.startswith("topic "): continue
        at = cf.parse_atom(b[:-1])
        for a in at[2]:
            if a == "user" or (at[1] == "demands" and a == at[2][2]): continue
            for s in sorted(sorts.get(a, set())):
                if s not in declared.setdefault(a, set()):
                    declared[a].add(s); new.append((a, s))
    lines = files[fname]
    head = [lines[i] for i in range(len(lines)) if lines[i].startswith("%") and all(x.startswith("%") for x in lines[:i+1])]
    rest = lines[len(head):]
    dom = "fixture" if fname.startswith("procedures") else fname.split(".")[0]
    voc = ["topic %s/vocabulary." % dom] + ["new(%s, %s)." % (a, s) for a, s in new]
    with open(os.path.join(OUT, fname), "w") as fh:
        fh.write("\n".join(head + voc + rest) + "\n")
    print(fname, len(new), "declarations")
