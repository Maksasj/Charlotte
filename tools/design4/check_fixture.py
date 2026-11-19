"""Offline check of the Design 4.0 spike fixture (data/spike4/), Appendix A and decision D4.3.

Run:  python3 -I tools/design4/check_fixture.py [--root data/spike4] [--quiet]

No dependencies. Written from the design document only (sections 4.1-4.5, 6.1, 7.4, Appendix A),
never from the agent's code. It checks:
  1. every line of teach/*.msgs parses (`topic d/u.`, `new(sym, sort).`, `R(a, b).`, `not R(a, b).`,
     `%` comments), symbols are declared before first use, every statement is in schema v4 except
     exactly two planted sort errors;
  2. the counts per relation and per domain, and the Appendix A question composition (100 questions,
     type x hop strata, IND 4/3/3, GAP 16 with 6 near-miss), 8 procedures (2 per domain);
  3. leakage: no gold atom and no bench/deny.txt atom is taught anywhere (teach/ or as an oracle
     answer in script/answers.jsonl), after `same_as` canonicalisation; the negation of a deny atom
     is counted ("contradicts gold"), not blocked (D4.8);
  4. derivability: each `answer` gold atom follows from the fixture by base rules B1-B9 at exactly
     the stated hops (section 7.4 hop count), and also from the cited proof lines alone; `excluded`
     atoms do not follow; each `gap` gold is underivable (or is a B9 `gap(S, I)` atom), and becomes
     derivable when its `missing` pieces are added; each `hyp` gold is underivable by base rules but
     predicted by its induced rule, whose support meets D4.7 (s >= 3 from >= 2 kinds, no unhandled
     counterexample, u >= 1); IND questions with gold `gap` name a rule that fails D4.7.

The small reasoner here is a reference for the fixture only. It is not `ref_reason.py`.
"""
import json
import os
import re
import sys
from collections import defaultdict, deque

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
DEFAULT_FIX = os.path.join(ROOT, "data", "spike4")
TEACH_ORDER = ["baking.msgs", "cooking.msgs", "cleaning.msgs", "repair.msgs", "procedures_fixture.msgs"]
DOMAINS = ["baking", "cooking", "cleaning", "repair"]

SORTS = {"item", "place", "material", "property", "function", "state", "action", "step", "task"}
SHARE_OK = {frozenset({"function", "action"})}  # section 4.2: function and action may share a name

# schema v4: predicate -> argument sorts ("item" accepts item or place; "*" any)
SCHEMA = {
    "is_a": ("ISA", "ISA"), "same_as": ("SAME", "SAME"), "has_part": ("item", "item"),
    "made_of": ("item", "material"), "has_property": ("item", "property"),
    "can_do": ("item", "function"), "can_do_on": ("item", "function", "item"),
    "used_for": ("item", "function"), "can_go_in": ("item", "place"),
    "effect": ("action", "item", "state"), "precondition": ("action", "item", "state"),
    # v3 procedure facts and the 4.0 bridges
    "task": ("task",), "step": ("task", "step"), "action": ("step", "action"),
    "object_of": ("step", "item"), "uses": ("step", "item"), "role": ("step", "item", "function"),
    "before": ("step", "step"), "requires_state": ("step", "state"), "achieves": ("step", "state"),
    "needs": ("task", "item"), "demands": ("step", "function", "REL", "VAL"),
    "into": ("step", "item"), "onto": ("step", "item"), "from": ("step", "item"),
    # situations and differences (taught cases, section 3.6 of the v3 schema)
    "lacks": ("agent", "item"), "has": ("agent", "item"), "swap": ("step", "item", "item"),
}
ITEM_RELATIONS = ["is_a", "same_as", "has_part", "made_of", "has_property", "can_do", "can_do_on",
                  "used_for", "can_go_in", "effect", "precondition"]
PROCEDURE_PREDS = ["task", "step", "action", "object_of", "uses", "role", "before", "requires_state",
                   "achieves", "needs", "demands", "lacks", "has", "swap"]
# inheritable relations and the argument that inherits down is_a (section 4.1)
SUBJ = {"has_part": 0, "made_of": 0, "has_property": 0, "can_do": 0, "can_do_on": 0, "used_for": 0,
        "can_go_in": 0, "effect": 1, "precondition": 1}
DEMANDABLE = {"made_of", "has_property", "can_do", "used_for", "can_go_in"}

ATOM_RE = re.compile(r"^(not\s+)?([a-z][a-z0-9_]*)\(([^()]*)\)$")
SYM_RE = re.compile(r"^[a-z][a-z0-9_]*$")
VAR_RE = re.compile(r"^[A-Z][A-Za-z0-9_]*$")


# ----------------------------------------------------------------------------------- parsing
def parse_atom(text, allow_vars=False):
    """'not p(a, b)' -> (neg, pred, args) or None."""
    m = ATOM_RE.match(text.strip())
    if not m:
        return None
    args = tuple(a.strip() for a in m.group(3).split(",")) if m.group(3).strip() else ()
    for a in args:
        if not (SYM_RE.match(a) or (allow_vars and VAR_RE.match(a))):
            return None
    return (bool(m.group(1)), m.group(2), args)


def astr(atom):
    neg, p, args = atom
    return ("not " if neg else "") + "%s(%s)" % (p, ", ".join(args))


def split_statements(text):
    """'a(b). not c(d).' -> ['a(b)', 'not c(d)'] (split on '.' outside parentheses)."""
    out, depth, cur = [], 0, ""
    for ch in text:
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
        if ch == "." and depth == 0:
            if cur.strip():
                out.append(cur.strip())
            cur = ""
        else:
            cur += ch
    if cur.strip():
        out.append(cur.strip())
    return out


class Fixture:
    def __init__(self, root):
        self.root = root
        self.errors = []
        self.warnings = []
        self.decl = {}          # sym -> set of sorts
        self.first_seen = {}    # sym -> delivery index (for canonical representative)
        self.statements = []    # dicts: file, line, topic, atom, raw, status, case
        self.rejected = []
        self._order = 0

    def err(self, msg):
        self.errors.append(msg)

    # ---- sorts
    def sort_ok(self, sym, want):
        have = self.decl.get(sym)
        if have is None:
            return None
        if want == "item":
            return bool(have & {"item", "place"})
        return want in have

    def check_schema(self, atom):
        """Returns None if fine, else a reason string (a sort error)."""
        neg, p, args = atom
        if p not in SCHEMA:
            return "unknown predicate %s" % p
        sig = SCHEMA[p]
        if len(args) != len(sig):
            return "arity of %s is %d" % (p, len(sig))
        for a in args:
            if a != "user" and a not in self.decl and not (p == "demands" and a == args[2]):
                return "undeclared symbol %s" % a
        if p == "is_a":
            for s in ("item", "action"):
                if self.sort_ok(args[0], s) and self.sort_ok(args[1], s):
                    return None
            return "is_a needs item x item or action x action"
        if p == "same_as":
            if self.decl[args[0]] & self.decl[args[1]]:
                return None
            return "same_as across sorts"
        if p == "demands":
            if not self.sort_ok(args[0], "step") or not self.sort_ok(args[1], "function"):
                return "demands(step, function, rel, value)"
            rel = args[2]
            if rel not in DEMANDABLE:
                return "demands on non-demandable relation %s" % rel
            want = SCHEMA[rel][1]
            if not self.sort_ok(args[3], want):
                return "demands value %s is not %s" % (args[3], want)
            return None
        for a, want in zip(args, sig):
            if want == "agent":
                if a != "user":
                    return "agent slot holds %s" % a
                continue
            if not self.sort_ok(a, want):
                return "%s is %s, not %s" % (a, "/".join(sorted(self.decl.get(a, {"?"}))), want)
        return None

    def note_syms(self, atom):
        for a in atom[2]:
            if a not in self.first_seen:
                self.first_seen[a] = self._order
                self._order += 1

    # ---- teach files
    def load_teach(self):
        for fname in TEACH_ORDER:
            path = os.path.join(self.root, "teach", fname)
            if not os.path.exists(path):
                self.err("missing teach/%s" % fname)
                continue
            topic = None
            first_content = True
            with open(path, encoding="utf-8") as fh:
                for n, raw in enumerate(fh, 1):
                    line = raw.split("%", 1)[0].strip()
                    if not line:
                        continue
                    if not line.endswith("."):
                        self.err("teach/%s:%d: statement must end with '.'" % (fname, n))
                        continue
                    if len(split_statements(line)) != 1:
                        self.err("teach/%s:%d: one statement per line" % (fname, n))
                        continue
                    body = line[:-1].strip()
                    if body.startswith("topic "):
                        t = body[6:].strip()
                        if not re.match(r"^[a-z][a-z0-9_]*/[a-z][a-z0-9_]*$", t):
                            self.err("teach/%s:%d: bad topic %r" % (fname, n, t))
                        topic = t
                        first_content = False
                        continue
                    if first_content:
                        self.err("teach/%s:%d: file must start with a topic line" % (fname, n))
                        first_content = False
                    if topic is None:
                        self.err("teach/%s:%d: statement before any topic" % (fname, n))
                    atom = parse_atom(body)
                    if atom is None:
                        self.err("teach/%s:%d: unparsable %r" % (fname, n, body))
                        continue
                    if atom[1] == "new" and not atom[0]:
                        sym, sort = atom[2] if len(atom[2]) == 2 else (None, None)
                        if sort not in SORTS:
                            self.err("teach/%s:%d: bad sort in %r" % (fname, n, body))
                            continue
                        have = self.decl.setdefault(sym, set())
                        if have and sort not in have and frozenset(have | {sort}) not in SHARE_OK:
                            self.err("teach/%s:%d: %s redeclared as %s" % (fname, n, sym, sort))
                        if sort in have:
                            self.warnings.append("teach/%s:%d: %s declared twice" % (fname, n, sym))
                        have.add(sort)
                        continue
                    st = {"file": "teach/" + fname, "line": n, "topic": topic, "atom": atom,
                          "raw": body, "src": "teach/%s:%d" % (fname, n)}
                    unit = (topic or "/").split("/", 1)[1]
                    st["case"] = unit.startswith("case_") and atom[1] in ("lacks", "has")
                    reason = self.check_schema(atom)
                    if reason:
                        st["status"] = "rejected"
                        st["reason"] = reason
                        self.rejected.append(st)
                    else:
                        st["status"] = "accepted"
                        self.note_syms(atom)
                    self.statements.append(st)

    # ---- oracle answers
    def load_answers(self):
        self.answers = []
        path = os.path.join(self.root, "script", "answers.jsonl")
        if not os.path.exists(path):
            self.err("missing script/answers.jsonl")
            return
        with open(path, encoding="utf-8") as fh:
            for n, raw in enumerate(fh, 1):
                if not raw.strip():
                    continue
                try:
                    rec = json.loads(raw)
                except ValueError:
                    self.err("answers.jsonl:%d: not JSON" % n)
                    continue
                q, a = rec.get("q", ""), rec.get("a", "").strip()
                self.answers.append((n, q, a))
                taught = []
                if a in ("yes", "no"):
                    m = re.match(r"^ask:\s*(.+)\?$", q.strip())
                    if m and parse_atom(m.group(1)):
                        at = parse_atom(m.group(1))
                        taught.append((a == "no", at[1], at[2]))
                elif a not in ("exception", "unknown"):
                    for s in split_statements(a):
                        at = parse_atom(s)
                        if at is None:
                            self.err("answers.jsonl:%d: unparsable answer %r" % (n, s))
                        else:
                            taught.append(at)
                for at in taught:
                    reason = self.check_schema(at)
                    st = {"file": "script/answers.jsonl", "line": n, "topic": "answer", "atom": at,
                          "raw": astr(at), "src": "script/answers.jsonl:%d" % n, "case": False}
                    if reason:
                        self.err("answers.jsonl:%d: answer %s fails schema: %s" % (n, astr(at), reason))
                        st["status"] = "rejected"
                    else:
                        st["status"] = "accepted"
                        self.note_syms(at)
                    self.statements.append(st)

    # ---- canonicalisation (B4, section 4.5): union-find, earliest-taught name represents
    def build_canon(self):
        parent = {}

        def find(x):
            while parent.get(x, x) != x:
                x = parent[x]
            return x
        for st in self.statements:
            neg, p, args = st["atom"]
            if st["status"] == "accepted" and p == "same_as" and not neg:
                a, b = find(args[0]), find(args[1])
                if a != b:
                    ra, rb = self.first_seen.get(a, 1e9), self.first_seen.get(b, 1e9)
                    if ra <= rb:
                        parent[b] = a
                    else:
                        parent[a] = b
        self._parent = parent
        self.canon = lambda s: find(s)
        self.same_as_src = defaultdict(list)
        for st in self.statements:
            neg, p, args = st["atom"]
            if st["status"] == "accepted" and p == "same_as" and not neg:
                for s in args:
                    self.same_as_src[s].append(st["src"])

    def canon_atom(self, atom):
        neg, p, args = atom
        return (neg, p, tuple(self.canon(a) for a in args))

    def taught_facts(self):
        """Global KB: accepted statements, canonical, case-local situations excluded."""
        out = []
        for st in self.statements:
            if st["status"] != "accepted" or st["case"]:
                continue
            at = self.canon_atom(st["atom"])
            if at[1] == "same_as":
                continue
            out.append((at, st["src"]))
        return out


# ----------------------------------------------------------------------------------- reasoner
class Proof:
    __slots__ = ("atom", "rule", "cost", "prem", "src", "extra")

    def __init__(self, atom, rule, cost, prem=(), src=None, extra=()):
        self.atom, self.rule, self.cost, self.prem, self.src, self.extra = atom, rule, cost, list(prem), src, list(extra)

    def leaves(self):
        if self.rule in ("taught", "given"):
            return [self]
        out = []
        for p in self.prem:
            out.extend(p.leaves())
        return out

    def inner(self):
        """Non-leaf atoms of the proof (the deny-worthy intermediate nodes) and path extras."""
        if self.rule in ("taught", "given"):
            return []
        out = [self.atom] + list(self.extra)
        for p in self.prem:
            out.extend(p.inner())
        return out


def leaf(atom, src, given=False):
    return Proof(atom, "given" if given else "taught", 0, src=src)


class Reasoner:
    """B1-B9 over canonical facts. facts: list of (atom, src); given: list of atoms."""

    def __init__(self, facts, given=()):
        self.src = {}
        for at, src in facts:
            self.src.setdefault(at, src)
        for at in given:
            self.src.setdefault(at, "given")
        self.facts = set(self.src)
        self.by_pred = defaultdict(list)
        for at in self.facts:
            self.by_pred[(at[0], at[1])].append(at[2])
        self._build_isa()
        self._build_b2()
        self._build_b6()
        self._build_b5()
        self._build_b7_b9()

    def leaf(self, at):
        s = self.src[at]
        return leaf(at, s, given=(s == "given"))

    # B1
    def _build_isa(self):
        self.parents = defaultdict(list)
        for a, k in self.by_pred[(False, "is_a")]:
            self.parents[a].append(k)
        self.anc = {}
        syms = set(self.parents)
        for ps in list(self.parents.values()):
            syms.update(ps)
        for x in syms:
            dist, pred = {x: 0}, {}
            dq = deque([x])
            while dq:
                u = dq.popleft()
                for k in sorted(self.parents.get(u, ())):
                    if k not in dist:
                        dist[k] = dist[u] + 1
                        pred[k] = u
                        dq.append(k)
            self.anc[x] = (dist, pred)

    def isa(self, x, k):
        return x != k and k in self.anc.get(x, ({x: 0}, {}))[0]

    def path(self, x, k):
        dist, pred = self.anc[x]
        nodes = [k]
        while nodes[-1] != x:
            nodes.append(pred[nodes[-1]])
        nodes.reverse()
        return nodes  # x ... k

    # B2: default inheritance with specificity, skeptical on diamonds
    def _build_b2(self):
        pos, neg = defaultdict(set), defaultdict(set)  # (rel, rest) -> subjects
        for (n, p), lst in list(self.by_pred.items()):
            if p not in SUBJ:
                continue
            i = SUBJ[p]
            for args in lst:
                rest = args[:i] + ("_",) + args[i + 1:]
                (neg if n else pos)[(p, rest)].add(args[i])
        self.pos, self.neg = pos, neg
        self.holds, self.holdsn, self.conflicts = {}, {}, set()
        keys_by_subj = defaultdict(set)
        for table in (pos, neg):
            for key, subs in table.items():
                for sub in subs:
                    keys_by_subj[sub].add(key)
        subjects = set(self.anc) | set(keys_by_subj)
        for x in subjects:
            dist = self.anc.get(x, ({x: 0}, {}))[0]
            xkeys = set()
            for k in dist:
                xkeys |= keys_by_subj.get(k, set())
            for key in sorted(xkeys):
                P, Q = pos[key], neg[key]
                inh = [k for k in dist if k != x and k in P]
                inhn = [k for k in dist if k != x and k in Q]
                own_p, own_n = x in P, x in Q
                if not (inh or inhn or own_p or own_n):
                    continue
                live = [k for k in inh if not own_n and not any(self.isa(k2, k) for k2 in inhn)]
                liven = [k for k in inhn if not own_p and not any(self.isa(k2, k) for k2 in inh)]
                conflict = bool(live) and bool(liven)
                rel, rest = key
                atom = (False, rel, tuple(x if a == "_" else a for a in rest))
                natom = (True,) + atom[1:]
                if conflict:
                    self.conflicts.add(atom)
                if own_p:
                    self.holds[atom] = self.leaf(atom)
                elif live and not conflict and not own_n:
                    self.holds[atom] = self._inh_proof(x, min(live, key=lambda k: (dist[k], k)), atom, key, False)
                if own_n:
                    self.holdsn[atom] = self.leaf(natom)
                elif liven and not conflict and not own_p:
                    self.holdsn[atom] = self._inh_proof(x, min(liven, key=lambda k: (dist[k], k)), natom, key, True)

    def _inh_proof(self, x, k, atom, key, negative):
        nodes = self.path(x, k)
        prem = [self.leaf((False, "is_a", (a, b))) for a, b in zip(nodes, nodes[1:])]
        rel, rest = key
        katom = (negative, rel, tuple(k if a == "_" else a for a in rest))
        prem.append(self.leaf(katom))
        extra = []
        for j in range(1, len(nodes) - 1):  # intermediate kinds: their inherited atom, and transitive is_a
            extra.append((negative, rel, tuple(nodes[j] if a == "_" else a for a in rest)))
        for j in range(2, len(nodes)):
            extra.append((False, "is_a", (x, nodes[j])))
        return Proof(atom, "B2", len(nodes) - 1, prem, extra=extra)

    # B6 bridges
    def _build_b6(self):
        self.cando = {}
        taught_neg = {(a[0], a[1]) for a in self.by_pred[(True, "can_do")]}

        def offer(x, f, pr):
            at = (False, "can_do", (x, f))
            if (x, f) in taught_neg:
                return
            cur = self.cando.get(at)
            if cur is None or pr.cost < cur.cost:
                self.cando[at] = pr
        for at, pr in self.holds.items():
            if at[1] == "can_do":
                offer(at[2][0], at[2][1], pr)
            elif at[1] == "used_for":
                offer(at[2][0], at[2][1], Proof((False, "can_do", at[2]), "B6", pr.cost, [pr]))
            elif at[1] == "can_do_on":
                x, f, _o = at[2]
                offer(x, f, Proof((False, "can_do", (x, f)), "B6", pr.cost, [pr]))
        for s, i, f in self.by_pred[(False, "role")]:
            at = (False, "can_do", (i, f))
            if at in self.conflicts:
                continue
            offer(i, f, Proof(at, "B6", 1, [self.leaf((False, "role", (s, i, f)))]))

    def holds_any(self, rel, x, v):
        """(polarity, proof) for a binary demand relation; can_do through B6."""
        at = (False, rel, (x, v))
        if rel == "can_do":
            if at in self.cando:
                return True, self.cando[at]
            if at in self.holdsn:
                return False, self.holdsn[at]
            return None, None
        if at in self.holds:
            return True, self.holds[at]
        if at in self.holdsn:
            return False, self.holdsn[at]
        return None, None

    # B5 has_part transitive (on demand; here for all subjects)
    def _build_b5(self):
        self.parts = {}
        edges = defaultdict(list)
        for at, pr in self.holds.items():
            if at[1] == "has_part":
                edges[at[2][0]].append((at[2][1], pr))
        for x in list(edges):
            best = {}
            dq = [(pr.cost, p, pr) for p, pr in edges[x]]
            while dq:
                dq.sort(key=lambda t: (t[0], t[1]))
                c, p, pr = dq.pop(0)
                if p in best:
                    continue
                best[p] = pr
                for q, epr in edges.get(p, ()):
                    if q not in best and q != x:
                        at = (False, "has_part", (x, q))
                        npr = Proof(at, "B5", pr.cost + epr.cost + 1, [pr, epr])
                        dq.append((npr.cost, q, npr))
            for p, pr in best.items():
                if pr.rule == "B5":
                    self.parts[(False, "has_part", (x, p))] = pr

    # B7 stand-in, B8 replacement, B9 gap
    def _build_b7_b9(self):
        lacks = {a[1] for a in self.by_pred[(False, "lacks")] if a[0] == "user"}
        has = {a[1] for a in self.by_pred[(False, "has")] if a[0] == "user"}
        self.lacks, self.has = lacks, has
        demands = defaultdict(list)
        for s, f, rel, v in self.by_pred[(False, "demands")]:
            demands[(s, f)].append((rel, v))
        self.stand_in, self.unfit, self.ask_fit = {}, {}, {}
        for s, i, f in self.by_pred[(False, "role")]:
            if i not in lacks:
                continue
            for j in sorted(has):
                if j == i:
                    continue
                cd = self.cando.get((False, "can_do", (j, f)))
                if cd is None:
                    continue
                prem = [self.leaf((False, "role", (s, i, f))), self.leaf((False, "lacks", ("user", i))),
                        self.leaf((False, "has", ("user", j))), cd]
                bad, open_, cost = [], [], 1 + cd.cost
                for rel, v in demands.get((s, f), ()):
                    dleaf = self.leaf((False, "demands", (s, f, rel, v)))
                    pol, pr = self.holds_any(rel, j, v)
                    if pol is False:
                        bad.append((dleaf, pr))
                    elif pol is None:
                        open_.append((False, rel, (j, v)))
                    else:
                        prem += [dleaf, pr]
                        cost += pr.cost
                key = (False, "stand_in", (s, i, j))
                if bad:
                    self.unfit[key] = bad
                elif open_:
                    self.ask_fit[key] = open_
                else:
                    cur = self.stand_in.get(key)
                    if cur is None or cost < cur.cost:
                        self.stand_in[key] = Proof(key, "B7", cost, prem, extra=[(False, "swap", (s, i, j))])
        some = {(a[2][0], a[2][1]) for a in self.stand_in}
        # achieves: taught, or from action + object_of + effect (bridge 3)
        achieves = {}
        for s, z in self.by_pred[(False, "achieves")]:
            achieves[(s, z)] = self.leaf((False, "achieves", (s, z)))
        acts = defaultdict(list)
        for s, a in self.by_pred[(False, "action")]:
            acts[s].append(a)
        objs = defaultdict(list)
        for s, o in self.by_pred[(False, "object_of")]:
            objs[s].append(o)
        effects = defaultdict(list)  # object -> [(action, state, proof)]
        for at, pr in self.holds.items():
            if at[1] == "effect":
                a, o, z = at[2]
                effects[o].append((a, z, pr))
        for s in acts:
            for a in acts[s]:
                for o in objs.get(s, ()):
                    for a2, z, pr in effects.get(o, ()):
                        if a2 == a and (s, z) not in achieves:
                            achieves[(s, z)] = Proof((False, "achieves", (s, z)), "bridge", pr.cost,
                                                     [self.leaf((False, "action", (s, a))),
                                                      self.leaf((False, "object_of", (s, o))), pr])
        uses = defaultdict(list)
        for s, i in self.by_pred[(False, "uses")]:
            uses[s].append(i)
        blocked = {}
        for s, items in uses.items():
            for i in items:
                if i in lacks and (s, i) not in some:
                    blocked.setdefault(s, i)
        self.replace = {}
        for s1, s2 in self.by_pred[(False, "before")]:
            if s1 not in blocked:
                continue
            for s2b, z in self.by_pred[(False, "requires_state")]:
                if s2b != s2 or (s1, z) not in achieves:
                    continue
                for o in objs.get(s1, ()):
                    for a2, z2, pr in effects.get(o, ()):
                        if z2 != z:
                            continue
                        key = (False, "replace_step", (s1, a2))
                        prem = [self.leaf((False, "uses", (s1, blocked[s1]))),
                                self.leaf((False, "lacks", ("user", blocked[s1]))),
                                self.leaf((False, "before", (s1, s2))),
                                self.leaf((False, "requires_state", (s2, z))), achieves[(s1, z)],
                                self.leaf((False, "object_of", (s1, o))), pr]
                        cost = 1 + pr.cost + achieves[(s1, z)].cost
                        cur = self.replace.get(key)
                        if cur is None or cost < cur.cost:
                            self.replace[key] = Proof(key, "B8", cost, prem,
                                                      extra=[(False, "needed_state", (s1, z)), (False, "blocked", (s1,))])
        replaced = {a[2][0] for a in self.replace}
        self.gaps = {}
        for s, items in uses.items():
            for i in items:
                if i in lacks and (s, i) not in some and s not in replaced:
                    key = (False, "gap", (s, i))
                    self.gaps[key] = Proof(key, "B9", 0, [self.leaf((False, "uses", (s, i))),
                                                          self.leaf((False, "lacks", ("user", i)))])

    # lookup
    def prove(self, atom):
        """Best proof of a ground atom (taught, given or derived), or None."""
        neg, p, args = atom
        pos_atom = (False, p, args)
        if atom in self.src:
            return self.leaf(atom)
        if p == "is_a" and not neg:
            x, k = args
            if self.isa(x, k):
                nodes = self.path(x, k)
                prem = [self.leaf((False, "is_a", (a, b))) for a, b in zip(nodes, nodes[1:])]
                extra = [(False, "is_a", (x, nodes[j])) for j in range(2, len(nodes) - 1)]
                return Proof(atom, "B1", len(nodes) - 2, prem, extra=extra)
            return None
        if p == "can_do" and not neg:
            return self.cando.get(atom) or self.holds.get(atom)
        if p == "has_part" and not neg:
            return self.holds.get(atom) or self.parts.get(atom)
        if p in SUBJ:
            return (self.holdsn if neg else self.holds).get(pos_atom)
        if neg:
            return None
        for table in (self.stand_in, self.replace, self.gaps):
            if atom in table:
                return table[atom]
        return None

    def contrary_known(self, atom):
        neg, p, args = atom
        return self.prove((not neg, p, args)) is not None


# ----------------------------------------------------------------------------------- induction
def parse_rule(text):
    """'not can_go_in(X, oven) :- made_of(X, plastic)' -> (head, body)."""
    head, body = [s.strip() for s in text.split(":-")]
    return parse_atom(head, allow_vars=True), parse_atom(body, allow_vars=True)


def rule_stats(rs, rule_text, fixture):
    head, body = parse_rule(rule_text)
    subjects = set(rs.anc) | {a[2][0] for a in list(rs.holds) + list(rs.holdsn)}
    sup, ctr, unk = [], [], []
    for x in sorted(subjects):
        b = (body[0], body[1], tuple(x if a == "X" else fixture.canon(a) for a in body[2]))
        if rs.prove(b) is None:
            continue
        h = (head[0], head[1], tuple(x if a == "X" else fixture.canon(a) for a in head[2]))
        if rs.prove(h) is not None:
            sup.append(x)
        elif rs.contrary_known(h):
            ctr.append(x)
        else:
            unk.append(x)
    kinds = {frozenset(rs.parents.get(x, ())) for x in sup}
    return {"support": sup, "counter": ctr, "unknown": unk, "kinds": len(kinds),
            "accepted": len(sup) >= 3 and len(kinds) >= 2 and not ctr and len(unk) >= 1}


# ----------------------------------------------------------------------------------- questions
def parse_query(q):
    """-> (goal atom or ('request', task), given atoms)."""
    q = q.strip()
    if q.startswith("request("):
        parts = split_statements(q)
        task = parse_atom(parts[0])[2][0]
        return ("request", task), [parse_atom(s) for s in parts[1:]]
    m = re.match(r"^query:\s*(.*?)\.\s*(given:\s*(.*))?$", q)
    if not m:
        return None, None
    goal = parse_atom(m.group(1), allow_vars=True)
    given = [parse_atom(s) for s in split_statements(m.group(3) or "")]
    return goal, given


COMPOSITION = {"K": (24, {1: 8, 2: 10, 3: 6}), "SUB": (20, {1: 4, 2: 10, 3: 6}),
               "X": (10, {1: 0, 2: 6, 3: 4}), "EXC": (10, {1: 4, 2: 4, 3: 2}),
               "STEP": (4, {1: 2, 2: 2, 3: 0}), "NAME": (6, {1: 2, 2: 4, 3: 0}),
               "IND": (10, None), "GAP": (16, None)}


def check(root, quiet=False):
    fx = Fixture(root)
    fx.load_teach()
    fx.load_answers()
    fx.build_canon()
    facts = fx.taught_facts()
    taught_canon = {at for at, _ in facts}
    # also every raw taught atom, for exact (L0) matching
    taught_raw = {st["atom"] for st in fx.statements if st["status"] == "accepted"}
    base = Reasoner(facts)
    out = []

    # ---- 1. sort errors
    if len(fx.rejected) != 2:
        fx.err("expected exactly 2 planted sort errors, found %d: %s" %
               (len(fx.rejected), [s["src"] for s in fx.rejected]))

    # ---- 2. counts
    counts = defaultdict(lambda: defaultdict(int))
    per_file = defaultdict(int)
    for st in fx.statements:
        if st["status"] != "accepted" or st["file"] == "script/answers.jsonl":
            continue
        dom = (st["topic"] or "?/").split("/")[0]
        neg, p, _ = st["atom"]
        key = ("not " if neg else "") + p if p in ITEM_RELATIONS else p
        counts[dom][key] += 1
        per_file[st["file"]] += 1
    for d in DOMAINS:
        n = per_file.get("teach/%s.msgs" % d, 0)
        if not 80 <= n <= 125:
            fx.err("teach/%s.msgs has %d accepted statements (want about 100)" % (d, n))
    tasks_by_dom = defaultdict(set)
    for st in fx.statements:
        if st["status"] == "accepted" and st["atom"][1] == "task":
            tasks_by_dom[st["topic"].split("/")[0]].add(st["atom"][2][0])
    for d in DOMAINS:
        if len(tasks_by_dom[d]) != 2:
            fx.err("domain %s has %d procedures, want 2" % (d, len(tasks_by_dom[d])))
        if sum(v for k, v in counts[d].items() if k == "demands") < 3:
            fx.err("domain %s has fewer than 3 demands" % d)
    deepest = max((max(d.values()) for d, _ in base.anc.values()), default=0)
    if deepest < 3:
        fx.err("no is_a chain of depth >= 3")

    # ---- 3/4. questions
    qpath = os.path.join(root, "bench", "questions.jsonl")
    deny_path = os.path.join(root, "bench", "deny.txt")
    questions = []
    if os.path.exists(qpath):
        with open(qpath, encoding="utf-8") as fh:
            for n, raw in enumerate(fh, 1):
                if raw.strip():
                    try:
                        questions.append(json.loads(raw))
                    except ValueError:
                        fx.err("questions.jsonl:%d: not JSON" % n)
    else:
        fx.err("missing bench/questions.jsonl")
    deny = []
    if os.path.exists(deny_path):
        with open(deny_path, encoding="utf-8") as fh:
            for n, raw in enumerate(fh, 1):
                s = raw.split("%", 1)[0].strip()
                if s:
                    at = parse_atom(s)
                    if at is None:
                        fx.err("deny.txt:%d: unparsable %r" % (n, s))
                    else:
                        deny.append(at)
    else:
        fx.err("missing bench/deny.txt")
    deny_set = {fx.canon_atom(a) for a in deny}

    if len(questions) != 100:
        fx.err("%d questions, want 100" % len(questions))
    ids = [q.get("id") for q in questions]
    if len(set(ids)) != len(ids):
        fx.err("duplicate question ids")
    strata = defaultdict(lambda: defaultdict(int))
    ind_kinds = defaultdict(int)
    near = 0
    for q in questions:
        strata[q["type"]][q.get("hops")] += 1
        if q["type"] == "IND":
            ind_kinds[(q["gold"]["kind"], q.get("ind_case", ""))] += 1
        if q["type"] == "GAP" and q.get("near_miss"):
            near += 1
    for t, (total, hops) in COMPOSITION.items():
        got = sum(strata[t].values())
        if got != total:
            fx.err("type %s has %d questions, want %d" % (t, got, total))
        if hops:
            for h, want in hops.items():
                if strata[t].get(h, 0) != want:
                    fx.err("type %s %d-hop: %d, want %d" % (t, h, strata[t].get(h, 0), want))
        else:
            if any(h is not None for h in strata[t]):
                fx.err("type %s must have hops null" % t)
    if near != 6:
        fx.err("GAP near-miss count %d, want 6" % near)
    if ind_kinds[("hyp", "support")] != 4 or ind_kinds[("gap", "counterexample")] != 3 or \
            ind_kinds[("gap", "below_threshold")] != 3:
        fx.err("IND split %s, want hyp/support 4, gap/counterexample 3, gap/below_threshold 3" % dict(ind_kinds))

    gold_all = []
    srcs = {st["src"]: st for st in fx.statements}
    for q in questions:
        qid = q["id"]
        gold = q["gold"]
        goal, given = parse_query(q["query"])
        if goal is None or any(g is None for g in given):
            fx.err("%s: unparsable query" % qid)
            continue
        given_c = [fx.canon_atom(g) for g in given]
        rs = Reasoner(facts, given_c)
        atoms = [parse_atom(a) for a in gold["atoms"]]
        if any(a is None for a in atoms) or not atoms:
            fx.err("%s: bad gold atoms" % qid)
            continue
        for line in gold.get("proof_lines", []):
            if line not in srcs:
                fx.err("%s: proof line %s is not a taught statement" % (qid, line))
            elif srcs[line]["status"] != "accepted":
                fx.err("%s: proof line %s is a rejected statement" % (qid, line))
        catoms = [fx.canon_atom(a) for a in atoms]
        gold_all.extend(catoms)
        for a in gold.get("missing", []):
            gold_all.append(fx.canon_atom(parse_atom(a)))
        kind = gold["kind"]
        if kind == "answer":
            hop_max = 0
            for a, ca in zip(atoms, catoms):
                pr = rs.prove(ca)
                if pr is None or pr.rule in ("taught", "given"):
                    fx.err("%s: gold %s not derived" % (qid, astr(a)))
                    continue
                hop_max = max(hop_max, pr.cost)
            if q.get("hops") != hop_max:
                fx.err("%s: hops label %s but minimal proof has %d" % (qid, q.get("hops"), hop_max))
            for e in gold.get("excluded", []):
                if rs.prove(fx.canon_atom(parse_atom(e))) is not None:
                    fx.err("%s: excluded %s is derivable" % (qid, e))
            # sufficiency: cited lines alone (plus given) derive every gold atom
            cited = [(fx.canon_atom(srcs[l]["atom"]), l) for l in gold.get("proof_lines", [])
                     if l in srcs and srcs[l]["atom"][1] != "same_as"]
            sub = Reasoner(cited, given_c)
            for a, ca in zip(atoms, catoms):
                if sub.prove(ca) is None:
                    fx.err("%s: gold %s not derivable from its proof lines alone" % (qid, astr(a)))
        elif kind == "gap":
            for a, ca in zip(atoms, catoms):
                if ca[1] == "gap":
                    if rs.prove(ca) is None:
                        fx.err("%s: B9 gap %s not derived" % (qid, astr(a)))
                elif rs.prove(ca) is not None or rs.contrary_known(ca):
                    fx.err("%s: gap gold %s is decided by base rules" % (qid, astr(a)))
            miss = [fx.canon_atom(parse_atom(m)) for m in gold.get("missing", [])]
            if miss and not gold.get("rule"):
                rs2 = Reasoner(facts + [(m, "withheld") for m in miss], given_c)
                target = gold.get("completes", [])
                for t in target:
                    if rs2.prove(fx.canon_atom(parse_atom(t))) is None:
                        fx.err("%s: adding missing pieces does not derive %s" % (qid, t))
            if gold.get("rule"):
                stt = rule_stats(rs, gold["rule"], fx)
                if stt["accepted"]:
                    fx.err("%s: rule %s meets D4.7 (support %s) but gold is gap" % (qid, gold["rule"], stt["support"]))
        elif kind == "hyp":
            stt = rule_stats(rs, gold["rule"], fx)
            if not stt["accepted"]:
                fx.err("%s: rule %s fails D4.7: %s" % (qid, gold["rule"], stt))
            for a, ca in zip(atoms, catoms):
                if rs.prove(ca) is not None or rs.contrary_known(ca):
                    fx.err("%s: hyp gold %s is decided by base rules" % (qid, astr(a)))
                x = ca[2][0]
                if x not in stt["unknown"]:
                    fx.err("%s: %s is not a new prediction of %s" % (qid, x, gold["rule"]))
        else:
            fx.err("%s: unknown gold kind %s" % (qid, kind))

    # ---- leakage
    for g in gold_all:
        if g not in deny_set:
            fx.err("gold atom %s missing from deny.txt" % astr(g))
    leaks = sorted(astr(d) for d in deny_set if d in taught_canon or d in taught_raw)
    for l in leaks:
        fx.err("LEAK: deny atom %s is taught" % l)
    contra = sorted(astr(d) for d in deny_set if (not d[0], d[1], d[2]) in taught_canon)

    # ---- summary
    out.append("fixture: %s" % os.path.relpath(root, ROOT))
    out.append("statements: %d accepted, %d rejected (sort errors: %s)" % (
        sum(1 for s in fx.statements if s["status"] == "accepted"), len(fx.rejected),
        ", ".join("%s %s" % (s["src"], s["raw"]) for s in fx.rejected)))
    for f in ["teach/" + t for t in TEACH_ORDER] + ["script/answers.jsonl"]:
        out.append("  %-34s %4d" % (f, sum(1 for s in fx.statements if s["file"] == f and s["status"] == "accepted")))
    rels = ITEM_RELATIONS + ["not " + r for r in ITEM_RELATIONS] + PROCEDURE_PREDS
    doms = DOMAINS
    out.append("counts per relation per domain (teach/, accepted):")
    out.append("  %-22s %s" % ("relation", " ".join("%8s" % d for d in doms)))
    for r in rels:
        row = [counts[d].get(r, 0) for d in doms]
        if any(row):
            out.append("  %-22s %s" % (r, " ".join("%8d" % v for v in row)))
    out.append("questions: %d  strata: %s" % (len(questions), {t: dict(v) for t, v in strata.items()}))
    out.append("deny atoms: %d  delivered leaks: %d  deny negations taught (contradicts gold, counted): %d" % (
        len(deny_set), len(leaks), len(contra)))
    out.append("is_a depth max: %d" % deepest)
    out.append("unresolved conflicts (skeptical blocking, after oracle answers): %s" %
               (", ".join(sorted(astr(c) for c in base.conflicts)) or "none"))
    no_ans = Reasoner([f for f in facts if not f[1].startswith("script/")])
    out.append("conflicts before oracle answers: %s" % ", ".join(sorted(astr(c) for c in no_ans.conflicts)))
    if fx.warnings and not quiet:
        out.extend("warning: " + w for w in fx.warnings)
    return fx, out


def main(argv):
    root = DEFAULT_FIX
    quiet = "--quiet" in argv
    if "--root" in argv:
        root = os.path.abspath(argv[argv.index("--root") + 1])
    fx, out = check(root, quiet)
    print("\n".join(out))
    if fx.errors:
        print("\n%d ERROR(S):" % len(fx.errors))
        for e in fx.errors:
            print("  " + e)
        return 1
    print("\nOK: fixture is clean.")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
