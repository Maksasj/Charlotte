import os, sys

OUT = r"C:\Programming\Test\data\curriculum\training"
ALL = []


def S(action, obj, text, t=None, uses=(), until=None, ov=None):
    return dict(a=action, o=obj, text=text, t=t, u=list(uses), until=until, ov=ov)


def T(id, stage, title, words, ing, requests, situation, needs, total, steps, variant=None, checks=None):
    ALL.append(dict(id=id, stage=stage, title=title, words=words, ing=ing, requests=requests,
                    situation=situation, needs=needs, total=total, steps=steps, variant=variant, checks=checks))


def join_list(items):
    if len(items) == 1:
        return items[0]
    return ", ".join(items[:-1]) + " and " + items[-1]


def mins(t):
    lo, hi = t
    return f"{lo} minute" + ("" if lo == 1 else "s") if lo == hi else f"{lo} to {hi} minutes"


def render(sc):
    tid = sc["id"]
    steps = sc["steps"]
    n = len(steps)
    sid = lambda i: f"{tid}_s{i}"
    facts = [f"task({tid})."]
    for sym, _ in sc["needs"]:
        facts.append(f"needs({tid}, {sym}).")
    lo, hi = sc["total"]
    facts.append(f"takes_time({tid}, {lo}, {hi}, minute).")
    for i, st in enumerate(steps, 1):
        s = sid(i)
        facts.append(f"step({tid}, {s}).")
        facts.append(f"action({s}, {st['a']}).")
        facts.append(f"object_of({s}, {st['o']}).")
        for u in st["u"]:
            facts.append(f"uses({s}, {u}).")
        if st["until"]:
            facts.append(f"until({s}, {st['until']}).")
        if st["t"]:
            facts.append(f"takes_time({s}, {st['t'][0]}, {st['t'][1]}, minute).")
        if st["ov"]:
            facts.append(f"overlaps({s}, {sid(st['ov'])}).")
    # ordering
    main = [i for i in range(1, n + 1) if not steps[i - 1]["ov"]]
    over = {}
    for i in range(1, n + 1):
        j = steps[i - 1]["ov"]
        if j:
            over.setdefault(j, []).append(i)
    edges = []
    for k, m in enumerate(main):
        nxt = main[k + 1] if k + 1 < len(main) else None
        prv = main[k - 1] if k > 0 else None
        ovs = over.get(m, [])
        if ovs:
            if prv:
                edges.append((prv, ovs[0]))
            for a, b in zip(ovs, ovs[1:]):
                edges.append((a, b))
        if nxt:
            edges.append((m, nxt))
            if ovs:
                edges.append((ovs[-1], nxt))
    edges.sort(key=lambda e: (e[1], e[0]))
    for a, b in edges:
        facts.append(f"before({sid(a)}, {sid(b)}).")

    teach = [f"To {sc['words']} you need {join_list([p for _, p in sc['needs']])}.",
             f"{sc['ing'][0].upper() + sc['ing'][1:]} takes {lo} to {hi} minutes."]
    labels = []
    for i, st in enumerate(steps, 1):
        txt = st["text"]
        if st["t"]:
            sep = ", for " if (st["until"] or "," in txt) else " for "
            txt += sep + mins(st["t"])
        if st["ov"]:
            teach.append(f"While step {st['ov']} runs, step {i} of {sc['ing']}: {txt}.")
        else:
            teach.append(f"Step {i} of {sc['ing']}: {txt}.")
        labels.append(f"{st['a']} {st['o'].replace('_', ' ')}")
    lab2 = []
    for i, st in enumerate(steps, 1):
        l = labels[i - 1]
        if st["ov"]:
            l += f" (while {labels[st['ov'] - 1]})"
        lab2.append(l)
    checks = sc["checks"] or [
        "names what is needed: " + join_list([p for _, p in sc["needs"]]),
        "gives the steps in order: " + ", ".join(lab2),
        f"gives a total time of about {lo} to {hi} minutes",
    ]
    checks = checks + ["closes politely"]

    out = ["---", f"id: {tid}", f"stage: {sc['stage']}", f"task: {tid}", "---", f"# {sc['title']}", "",
           "## requests"] + [f"- {r}" for r in sc["requests"]] + ["", "## situation"] + \
          [f"- {s}" for s in sc["situation"]] + ["", "## ground truth", "```datalog"] + facts + ["```", "",
           "## teach"] + [f"- {t}" for t in teach] + ["", "## checklist"] + [f"- {c}" for c in checks]
    if sc["variant"]:
        c, r, s, e = sc["variant"]
        out += ["", "## variant", f"- change: {c}", f"- request: {r}", f"- situation: {s}", f"- expect: {e}"]
    return "\n".join(out) + "\n"


exec(open(os.path.join(os.path.dirname(__file__), "data.py"), encoding="utf-8").read())

for sc in ALL:
    with open(os.path.join(OUT, sc["id"] + ".md"), "w", encoding="ascii", newline="\n") as f:
        f.write(render(sc))
print(len(ALL), "written")
