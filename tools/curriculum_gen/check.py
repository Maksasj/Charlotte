import os, re, glob, collections

D = r"C:\Programming\Test\data\curriculum\training"
FORBID = set("""make_tea boil_egg make_omelette cook_rice make_pancake bake_potato make_salad make_smoothie water_plant clean_window wash_dishes do_laundry iron_shirt fix_flat_tire hang_picture replace_lightbulb plan_picnic plan_birthday_party pack_for_trip write_thank_you_note make_toast make_coffee scrambled_eggs make_sandwich cook_pasta make_porridge bake_cookie make_lemonade grilled_cheese make_bed vacuum_room clean_bathroom take_out_trash wash_car defrost_freezer feed_cat water_garden book_train_ticket check_in_flight plan_day_trip unclog_sink change_bike_tire reset_router tighten_loose_screw invite_friends_to_dinner plan_movie_night wrap_gift write_birthday_card set_table sew_button garlic_tomato_pasta tomato_soup""".split())
AR = dict(task=1, needs=2, takes_time=4, step=2, action=2, object_of=2, uses=2, until=2, overlaps=2, before=2)
SYM = re.compile(r"^[a-z][a-z0-9_]*$")
stages = collections.Counter()
variants = 0
errs = []
ids = set()
for path in sorted(glob.glob(os.path.join(D, "*.md"))):
    name = os.path.basename(path)[:-3]
    raw = open(path, "rb").read()
    E = lambda m: errs.append(f"{name}: {m}")
    try:
        txt = raw.decode("ascii")
    except UnicodeDecodeError:
        E("non-ascii"); continue
    if name in FORBID: E("forbidden id")
    if name in ids: E("dup")
    ids.add(name)
    m = re.match(r"---\nid: (\S+)\nstage: (\S+)\ntask: (\S+)\n---\n# .+\n", txt)
    if not m: E("front matter"); continue
    if m.group(1) != name or m.group(3) != name: E("id mismatch")
    st = m.group(2)
    if st not in ("procedures", "household", "tools", "social"): E("stage")
    stages[st] += 1
    secs = re.findall(r"^## (.+)$", txt, re.M)
    want = ["requests", "situation", "ground truth", "teach", "checklist"]
    if secs not in (want, want + ["variant"]): E(f"sections {secs}")
    sec = {}
    for s, body in re.findall(r"^## (.+?)\n(.*?)(?=^## |\Z)", txt, re.M | re.S):
        sec[s] = body
    reqs = [l for l in sec["requests"].splitlines() if l.startswith("- ")]
    if len(reqs) != 3 or len(set(reqs)) != 3: E("requests")
    dl = re.search(r"```datalog\n(.*?)```", sec["ground truth"], re.S).group(1)
    facts = collections.defaultdict(list)
    for line in dl.strip().splitlines():
        fm = re.match(r"^([a-z_]+)\(([^()]*)\)\.$", line)
        if not fm: E(f"bad fact {line}"); continue
        p, args = fm.group(1), [a.strip() for a in fm.group(2).split(",")]
        if p not in AR: E(f"pred {p}"); continue
        if len(args) != AR[p]: E(f"arity {line}")
        for a in args:
            if not (SYM.match(a) or a.isdigit()): E(f"symbol {a}")
        facts[p].append(args)
    if facts["task"] != [[name]]: E("task fact")
    steps = [s for t, s in facts["step"] if t == name]
    if not 3 <= len(steps) <= 6: E(f"{len(steps)} steps")
    if [f"{name}_s{i}" for i in range(1, len(steps) + 1)] != steps: E("step names")
    tt = {a[0]: (int(a[1]), int(a[2])) for a in facts["takes_time"]}
    for a in facts["takes_time"]:
        if a[3] != "minute": E("unit")
        if int(a[1]) > int(a[2]): E(f"lo>hi {a}")
    if name not in tt: E("no total"); continue
    tlo, thi = tt[name]
    for p in ("action", "object_of"):
        got = {a[0] for a in facts[p]}
        if got != set(steps): E(f"{p} coverage")
    ov = {a[0]: a[1] for a in facts["overlaps"]}
    main_min = sum(tt.get(s, (0, 0))[0] for s in steps if s not in ov)
    main_max = sum(tt.get(s, (0, 0))[1] for s in steps if s not in ov)
    if main_min > thi: E(f"step mins {main_min} > total {thi}")
    if main_max < tlo: E(f"step maxes {main_max} < total lo {tlo}")
    for s, r in ov.items():
        if s in tt and r in tt and tt[s][1] > tt[r][1]: E(f"overlap longer {s}")
    for s in steps:
        if s in tt and tt[s][1] > thi: E(f"step {s} > total")
    # order: every step connected, acyclic
    bef = [(a, b) for a, b in facts["before"]]
    touched = {x for e in bef for x in e}
    if set(steps) - touched: E(f"unordered {set(steps) - touched}")
    for a, b in bef:
        if int(a.rsplit("s", 1)[1]) >= int(b.rsplit("s", 1)[1]): E(f"backward {a},{b}")
    # teach
    teach = [l[2:] for l in sec["teach"].splitlines() if l.startswith("- ")]
    if not re.match(r"^To .+ you need .+\.$", teach[0]): E("teach needs")
    if not re.match(r"^[A-Z][a-z-]+ing .+ takes %d to %d minutes\.$" % (tlo, thi), teach[1]): E("teach total")
    if len(teach) != 2 + len(steps): E("teach count")
    for i, l in enumerate(teach[2:], 1):
        s = f"{name}_s{i}"
        if s in ov:
            j = ov[s].rsplit("s", 1)[1]
            pat = rf"^While step {j} runs, step {i} of [a-z].*?: [a-z].+\.$"
        else:
            pat = rf"^Step {i} of [a-z].*?: [a-z].+\.$"
        if not re.match(pat, l): E(f"teach frame {l}")
        if s in tt:
            lo, hi = tt[s]
            want = f"{lo} minute" if lo == hi else f"{lo} to {hi} minutes"
            if want not in l: E(f"teach time {l}")
        low = l.lower()
        act = [a[1] for a in facts["action"] if a[0] == s][0]
        objs = [a[1] for a in facts["object_of"] if a[0] == s] + [a[1] for a in facts["uses"] if a[0] == s]
        for o in objs:
            w = o.split("_")[-1][:3]
            if w not in low: E(f"teach misses {o} in step {i}")
        if any(a[0] == s for a in facts["until"]) and "until" not in low: E(f"until missing step {i}")
    nl = teach[0].lower()
    for t, n in facts["needs"]:
        if n.split("_")[-1][:4] not in nl: E(f"needs {n} not taught")
    chk = [l for l in sec["checklist"].splitlines() if l.startswith("- ")]
    if not 3 <= len(chk) <= 4 or chk[-1] != "- closes politely": E("checklist")
    if "variant" in sec:
        variants += 1
        keys = [l.split(":")[0] for l in sec["variant"].splitlines() if l.startswith("- ")]
        if keys != ["- change", "- request", "- situation", "- expect"]: E("variant keys")
print(dict(stages), "variants", variants, "of", sum(stages.values()))
print("\n".join(errs) or "OK")
