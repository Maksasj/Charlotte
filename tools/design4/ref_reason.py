"""Independent proof checker for the Design 4.0 spike (section 7.4, the checker of record).

Written from the design's base rules (crates/reason4/core/base.dl is the rule text of section
4.3-4.4), not from reason4's code. For every proof record dumped by `sla4 spike --proofs FILE`:
  - each rule node `core/Bk/n` is an instance of that rule: its head unifies with the node atom
    and its positive body literals unify, in order, with the node's premises;
  - every negated body literal, instantiated, is listed in the node's checked_absent;
  - built-in comparisons (X != Y) hold under the binding;
  - each leaf is a delivered taught fact (from the facts file), a query `given` premise, or a
    core marker (`inheritable(...)`).
Learned-rule nodes (`learn/...`, `lbridge/...`) are checked for leaves only (their rule text is
data of the run, recorded in the run file).

Usage: python3 -I tools/design4/ref_reason.py --proofs P.jsonl --facts F.txt
Exit 1 if any proof fails.
"""
import json
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
ATOM = re.compile(r"\s*(not\s+)?([a-z_][a-z0-9_]*)\(([^()]*)\)\s*")


def split_top(s, sep=","):
    out, depth, cur = [], 0, ""
    for ch in s:
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
        if ch == sep and depth == 0:
            out.append(cur.strip())
            cur = ""
        else:
            cur += ch
    if cur.strip():
        out.append(cur.strip())
    return out


def parse_atom(s):
    m = ATOM.fullmatch(s)
    if not m:
        return None
    args = [a.strip() for a in m.group(3).split(",")] if m.group(3).strip() else []
    return (bool(m.group(1)), m.group(2), args)


def load_rules():
    rules, sec, n = {}, None, 0
    for line in open(os.path.join(ROOT, "crates/reason4/core/base.dl")):
        line = line.rstrip("\n")
        if line.startswith("%% "):
            sec, n = line[3:].split()[0], 0
            continue
        line = line.split("%")[0].strip()
        if not line or not sec:
            continue
        for stmt in [x for x in re.split(r"\.\s*(?=[a-z]|$)", line) if x.strip()]:
            head, body = stmt.split(":-")
            n += 1
            lits = []
            for b in split_top(body.strip().rstrip(".")):
                if "!=" in b:
                    l, r = [x.strip() for x in b.split("!=")]
                    lits.append(("neq", l, r))
                else:
                    lits.append(("lit",) + parse_atom(b))
            rules[f"core/{sec}/{n}"] = (parse_atom(head.strip()), lits)
    return rules


def is_var(t):
    return t[:1].isupper() or t == "_"


def unify(pattern_args, ground_args, env):
    if len(pattern_args) != len(ground_args):
        return None
    env = dict(env)
    for p, g in zip(pattern_args, ground_args):
        if is_var(p):
            if p == "_":
                continue
            if p in env and env[p] != g:
                return None
            env[p] = g
        elif p != g:
            return None
    return env


def ground(pred, args, env):
    return f"{pred}({', '.join(env.get(a, a) for a in args)})"


def check_node(node, rules, facts, given, errors, path):
    atom = parse_atom(node["atom"])
    if "given" in node:
        s = node["atom"].replace(" ", "")
        if not (s in facts or s in given or s.startswith("inheritable(")):
            errors.append(f"{path}: leaf {node['atom']} is not a delivered fact or given premise")
        return
    rid = node.get("rule", "")
    for k, p in enumerate(node.get("premises", [])):
        check_node(p, rules, facts, given, errors, f"{path}/{k}")
    if not rid.startswith("core/"):
        return
    if rid not in rules:
        errors.append(f"{path}: unknown rule {rid}")
        return
    (hneg, hpred, hargs), body = rules[rid]
    env = unify(hargs, atom[2], {}) if hpred == atom[1] else None
    if env is None:
        errors.append(f"{path}: {node['atom']} does not match the head of {rid}")
        return
    prem = [parse_atom(p["atom"]) for p in node.get("premises", [])]
    pos = [l for l in body if l[0] == "lit" and not l[1]]
    if len(pos) != len(prem):
        errors.append(f"{path}: {rid} has {len(pos)} positive literals, proof has {len(prem)} premises")
        return
    for lit, pr in zip(pos, prem):
        if lit[2] != pr[1]:
            errors.append(f"{path}: premise {pr[1]} where {rid} needs {lit[2]}")
            return
        env2 = unify(lit[3], pr[2], env)
        if env2 is None:
            errors.append(f"{path}: premise {pr} does not unify with {lit[2]}{lit[3]} under {env}")
            return
        env = env2
    absent = [a.replace(" ", "") for a in node.get("checked_absent", [])]
    for lit in body:
        if lit[0] == "neq":
            l, r = env.get(lit[1], lit[1]), env.get(lit[2], lit[2])
            if l == r:
                errors.append(f"{path}: {lit[1]} != {lit[2]} fails")
        elif lit[1]:
            g = ground(lit[2], lit[3], env).replace(" ", "")
            if "_" in lit[3]:
                continue
            if g not in absent:
                errors.append(f"{path}: negated {g} not checked absent")


def main():
    a = sys.argv
    proofs = a[a.index("--proofs") + 1]
    facts = set(l.strip().replace(" ", "") for l in open(a[a.index("--facts") + 1]) if l.strip())
    rules = load_rules()
    # --out FILE: one verdict per proof record, for the bench4 scorer
    per = open(a[a.index("--out") + 1], "w") if "--out" in a else None
    n = bad = 0
    examples = []
    for line in open(proofs):
        rec = json.loads(line)
        n += 1
        given = set(x.replace(" ", "") for x in rec.get("given", []))
        errors = []
        tree = rec["proof"].get("tree")
        if not tree:
            errors.append("no proof")
        else:
            check_node(tree, rules, facts, given, errors, rec["q"])
        if errors:
            bad += 1
            if len(examples) < 8:
                examples.append(errors[0])
        if per is not None:
            per.write(json.dumps({"q": rec["q"], "claim": rec["claim"], "ok": not errors, "error": errors[0] if errors else ""}) + "\n")
    print(json.dumps({"proofs": n, "failed": bad, "examples": examples}))
    sys.exit(1 if bad else 0)


if __name__ == "__main__":
    main()
