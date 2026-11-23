// Can solution shapes (skeletons) be learned from labelled conversations?
//
// A hidden teacher policy holds conversations in randomly generated domains. The learner sees,
// for every agent turn, a fixed generic description of the conversation state and the act the
// teacher chose. It induces an ordered rule list (the same form as a v5 skeleton) and is scored
// on conversations in domains it never saw.
//
// Fixed before the first run, the same for every rung:
//   - the state description (features) and the argument selectors;
//   - the learner (ordered rule list, beam search, Laplace accuracy, at most 4 conditions per rule);
//   - pass mark: median over 10 seeds of unseen conversations reproduced exactly >= 95%
//     when trained on 20 conversations.
//
// Run: node experiments/pattern-learning/run.mjs [--rules] [--errors] [--more]
//   --rules   print the rule list learned from 20 conversations (seed 0)
//   --errors  print the commonest mistakes at 20 conversations
//   --more    train on 20 to 320 conversations instead of 5 to 40
//   --domains N  spread the training conversations over N domains instead of 6

const OPS = ["ask", "grant", "offer", "deny", "confirm", "lookup", "report", "instruct", "repeat", "finish", "escalate"];
const UACTS = ["request", "provide", "correct", "done", "failed"];
const MORE = process.argv.includes("--more"), ERRORS = process.argv.includes("--errors");
const DOMS = process.argv.includes("--domains") ? +process.argv[process.argv.indexOf("--domains") + 1] : 6;
const SIZES = MORE ? [20, 80, 160, 320] : [5, 10, 20, 40], SEEDS = 10, TEST_CONVS = 200, PASS = 95;

function rng(seed) { let a = seed >>> 0; return () => { a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const pick = (r, n) => Math.floor(r() * n);

// ---------- domains ----------
// gated: a primary rule (conditions over yes/no facts) and sometimes an alternative rule.
function gatedDomain(name, r, minP, maxP, record) {
  const np = minP + pick(r, maxP - minP + 1), prim = [];
  for (let i = 0; i < np; i++) prim.push({ slot: name + ".p" + i, val: r() < 0.5, src: "user" });
  if (record) { prim.forEach((c) => { if (r() < 0.5) c.src = "record"; }); if (!prim.some((c) => c.src === "record")) prim[pick(r, np)].src = "record"; }
  let alt = null;
  if (r() < 0.65) {
    alt = [];
    const shareable = prim.filter((c) => c.src === "user");
    if (shareable.length && r() < 0.5) { const c = shareable[pick(r, shareable.length)]; alt.push({ slot: c.slot, val: !c.val, src: "user" }); }
    if (!alt.length || r() < 0.4) alt.push({ slot: name + ".a0", val: r() < 0.5, src: "user" });
  }
  return { name, kind: "gated", prim, alt };
}
const procDomain = (name, r, minL, maxL) => ({ name, kind: "proc", steps: minL + pick(r, maxL - minL + 1) });
const statusDomain = (name) => ({ name, kind: "status" });

// ---------- the hidden teacher ----------
function evalAtom(conds, bound) { let viol = false; const unb = []; for (const c of conds) { if (!(c.slot in bound)) unb.push(c); else if (bound[c.slot] !== c.val) viol = true; } return { viol, unb }; }
function decide(s) {
  const d = s.dom;
  if (d.kind === "proc") {
    if (s.user.at(-1) === "failed") return s.agent.at(-1) === "repeat" ? { op: "escalate" } : { op: "repeat", arg: s.done };
    return s.done < d.steps ? { op: "instruct", arg: s.done } : { op: "finish" };
  }
  if (d.kind === "status") { if (!s.fetched) return s.subject ? { op: "lookup" } : { op: "ask", arg: "subject" }; return s.found ? { op: "report" } : { op: "escalate" }; }
  const P = evalAtom(d.prim, s.bound);
  if (P.viol) {
    if (!d.alt) return { op: "deny" };
    const A = evalAtom(d.alt, s.bound);
    if (A.viol) return { op: "deny" };
    return A.unb.length ? { op: "ask", arg: A.unb[0].slot } : { op: "offer" };
  }
  if (P.unb.some((c) => c.src === "record")) { if (!s.fetched) return s.subject ? { op: "lookup" } : { op: "ask", arg: "subject" }; return { op: "escalate" }; }
  return P.unb.length ? { op: "ask", arg: P.unb[0].slot } : { op: "grant" };
}
// After a correction, a decision that still stands is confirmed rather than repeated.
// s.standing is the teacher's own memory; the learner never sees it.
function teacher(s) { const d = decide(s); return s.user.at(-1) === "correct" && s.standing && d.op === s.standing ? { op: "confirm" } : d; }

// ---------- what the learner sees ----------
function features(s, trainNames) {
  const d = s.dom, f = {}, g = d.kind === "gated";
  const P = g ? evalAtom(d.prim, s.bound) : null, A = g && d.alt ? evalAtom(d.alt, s.bound) : null;
  f.req_gated = g; f.req_proc = d.kind === "proc"; f.req_status = d.kind === "status";
  f.p_unbound = !!P && P.unb.length > 0; f.p_viol = !!P && P.viol; f.p_sat = !!P && !P.viol && !P.unb.length;
  f.a_exists = !!A; f.a_unbound = !!A && A.unb.length > 0; f.a_viol = !!A && A.viol; f.a_sat = !!A && !A.viol && !A.unb.length;
  f.p_need_record = !!P && P.unb.some((c) => c.src === "record"); f.p_need_user = !!P && P.unb.some((c) => c.src === "user");
  f.subject_known = s.subject; f.fetched = s.fetched; f.found = s.found;
  f.steps_left = d.kind === "proc" && s.done < d.steps; f.none_done = d.kind === "proc" && s.done === 0;
  for (const o of OPS) { f["last_agent=" + o] = s.agent.at(-1) === o; f["prev_agent=" + o] = s.agent.at(-2) === o; f["ever_agent=" + o] = s.agent.includes(o); }
  for (const u of UACTS) { f["last_user=" + u] = s.user.at(-1) === u; f["prev_user=" + u] = s.user.at(-2) === u; f["ever_user=" + u] = s.user.includes(u); }
  f.turn_ge3 = s.turn >= 3; f.turn_ge5 = s.turn >= 5; f.bound_ge2 = Object.keys(s.bound).length >= 2;   // distractors
  for (const n of trainNames) f["dom=" + n] = d.name === n;                                             // distractors
  return f;
}
// Ways to name an act's argument without naming domain content.
function selectors(s) {
  const d = s.dom, o = { subject: "subject" };
  if (d.kind === "gated") {
    const P = evalAtom(d.prim, s.bound);
    o.p_first_unbound = P.unb[0]?.slot; o.p_last_unbound = P.unb.at(-1)?.slot; o.p_first_unbound_user = P.unb.find((c) => c.src === "user")?.slot;
    if (d.alt) { const A = evalAtom(d.alt, s.bound); o.a_first_unbound = A.unb[0]?.slot; o.a_last_unbound = A.unb.at(-1)?.slot; }
  }
  if (d.kind === "proc") { o.step_first_undone = s.done < d.steps ? s.done : undefined; o.step_last_done = s.done > 0 ? s.done - 1 : undefined; o.step_first = 0; o.step_last = d.steps - 1; }
  return o;
}

// ---------- one conversation ----------
function simulate(dom, r, trainNames, corrections) {
  const conds = dom.kind === "gated" ? [...dom.prim, ...(dom.alt || [])] : [], truth = {};
  for (const c of conds) if (!(c.slot in truth)) truth[c.slot] = r() < 0.5;
  if (dom.kind === "gated" && r() < 0.5) for (const c of dom.prim) truth[c.slot] = c.val;
  const s = { dom, bound: {}, subject: false, fetched: false, found: false, done: 0, agent: [], user: ["request"], turn: 0, standing: null };
  const usesRecord = dom.kind === "status" || (dom.kind === "gated" && dom.prim.some((c) => c.src === "record"));
  if (usesRecord && r() < 0.5) s.subject = true;
  const recordExists = r() < 0.85;
  for (const c of conds) if (c.src === "user" && !(c.slot in s.bound) && r() < 0.4) s.bound[c.slot] = truth[c.slot];
  const correct = (skip) => { const ks = Object.keys(s.bound).filter((k) => k !== skip); if (!ks.length) return false; const k = ks[pick(r, ks.length)]; truth[k] = !truth[k]; s.bound[k] = truth[k]; return true; };
  const ex = []; let nCorr = 0;
  for (let t = 0; t < 40; t++) {
    const act = teacher(s);
    ex.push({ f: features(s, trainNames), sel: selectors(s), op: act.op, arg: act.arg });
    s.agent.push(act.op); s.turn++;
    switch (act.op) {
      case "ask":
        if (act.arg === "subject") { s.subject = true; s.user.push("provide"); }
        else { s.bound[act.arg] = truth[act.arg]; s.user.push(corrections && r() < 0.15 && correct(act.arg) ? "correct" : "provide"); }
        break;
      case "lookup":
        s.fetched = true; s.found = recordExists;
        if (s.found && dom.kind === "gated") for (const c of dom.prim) if (c.src === "record") s.bound[c.slot] = truth[c.slot];
        break;
      case "grant": case "offer": case "deny": case "confirm":
        if (act.op !== "confirm") s.standing = act.op;
        if (corrections && nCorr < 3 && r() < (nCorr ? 0.5 : 0.7) && correct()) { nCorr++; s.user.push("correct"); } else return ex;
        break;
      case "instruct": case "repeat":
        if (r() < (act.op === "instruct" ? 0.8 : 0.6)) { s.done++; s.user.push("done"); } else s.user.push("failed");
        break;
      default: return ex;
    }
  }
  return ex;
}

// ---------- the learner ----------
// Name each concrete argument by a selector: per act, take the selector that explains the most
// examples, then the next one for whatever is left.
function label(exs) {
  const byOp = {};
  for (const e of exs) { if (e.arg === undefined) e.y = e.op; else (byOp[e.op] ??= []).push(e); }
  for (const op in byOp) {
    let rem = byOp[op];
    while (rem.length) {
      const cnt = {};
      for (const e of rem) for (const k in e.sel) if (e.sel[k] === e.arg) cnt[k] = (cnt[k] || 0) + 1;
      const best = Object.keys(cnt).sort((a, b) => cnt[b] - cnt[a] || a.localeCompare(b))[0];
      if (!best) { rem.forEach((e) => { e.y = op + "(?)"; }); break; }
      rem = rem.filter((e) => { if (e.sel[best] === e.arg) { e.y = op + "(" + best + ")"; return false; } return true; });
    }
  }
}
function learn(exs, maxRules = Infinity) {
  const feats = [...new Set(exs.flatMap((e) => Object.keys(e.f)))], K = new Set(exs.map((e) => e.y)).size;
  const lits = feats.flatMap((f) => [{ f, v: true }, { f, v: false }]);
  const score = (sub) => { const c = {}; for (const e of sub) c[e.y] = (c[e.y] || 0) + 1; let cls = null, m = 0; for (const k of Object.keys(c).sort()) if (c[k] > m) { m = c[k]; cls = k; } return { cls, n: sub.length, q: (m + 1) / (sub.length + K) }; };
  let rem = exs.slice(); const rules = [];
  while (rem.length && rules.length < maxRules) {
    let beam = [{ lits: [], sub: rem, ...score(rem) }], best = null;
    for (let depth = 0; depth < 4; depth++) {
      const cand = new Map();
      for (const b of beam) for (const l of lits) {
        if (b.lits.some((x) => x.f === l.f)) continue;
        const sub = b.sub.filter((e) => !!e.f[l.f] === l.v);
        if (sub.length < 2 || sub.length === b.sub.length) continue;
        const sc = score(sub); if (sc.q <= b.q) continue;
        const ls = [...b.lits, l], key = ls.map((x) => x.f + x.v).sort().join("&");
        if (!cand.has(key)) cand.set(key, { lits: ls, sub, ...sc });
      }
      if (!cand.size) break;
      beam = [...cand.values()].sort((a, b) => b.q - a.q || b.n - a.n).slice(0, 5);
      if (!best || beam[0].q > best.q) best = beam[0];
    }
    if (!best) break;
    rules.push({ lits: best.lits, cls: best.cls, n: best.n });
    const covered = new Set(best.sub); rem = rem.filter((e) => !covered.has(e));
  }
  return { rules, def: score(rem.length ? rem : exs).cls };
}
function predict(model, e) {
  const cls = (model.rules.find((r) => r.lits.every((l) => !!e.f[l.f] === l.v)) || { cls: model.def }).cls;
  const m = cls.match(/^(\w+)(?:\((.+)\))?$/);
  return { op: m[1], arg: m[2] ? e.sel[m[2]] : undefined };
}
function evaluate(model, convs, errs) {
  let okC = 0, okT = 0, nT = 0;
  for (const c of convs) { let all = true; for (const e of c) { const p = predict(model, e); const ok = p.op === e.op && p.arg === e.arg; nT++; if (ok) okT++; else { all = false; if (errs) { const k = `teacher ${e.op}, learner ${p.op}${p.op === e.op ? " with the wrong argument" : ""}`; errs[k] = (errs[k] || 0) + 1; } } } if (all) okC++; }
  return { conv: 100 * okC / convs.length, turn: 100 * okT / nT };
}
const showRules = (m) => m.rules.map((r, i) => `    ${i + 1}. when ${r.lits.map((l) => (l.v ? "" : "not ") + l.f).join(" and ")}  ->  ${r.cls}   [${r.n}]`).join("\n") + `\n    otherwise  ->  ${m.def}`;

// ---------- rungs ----------
const RUNGS = [
  { id: "1 flat", what: "check a rule: ask, grant, offer an alternative, deny",
    train: (r, i) => gatedDomain("T" + i, r, 1, 3, false), test: (r, i) => gatedDomain("H" + i, r, 2, 4, false) },
  { id: "2 position", what: "multi-step procedure with one retry per step; unseen lengths (train 2-4 steps, test 5-7)",
    train: (r, i) => procDomain("T" + i, r, 2, 4), test: (r, i) => procDomain("H" + i, r, 5, 7) },
  { id: "3 revision", what: "rung 1 plus user corrections; a decision that still stands is confirmed, not repeated",
    train: (r, i) => gatedDomain("T" + i, r, 1, 3, false), test: (r, i) => gatedDomain("H" + i, r, 2, 4, false), corrections: true },
  { id: "4a composition, shown", what: "some facts come from a record: ask the id, look it up, then check the rule",
    train: (r, i) => gatedDomain("T" + i, r, 1, 3, true), test: (r, i) => gatedDomain("H" + i, r, 2, 4, true) },
  { id: "4b composition, never shown", what: "trained on rule checks and on plain lookups separately, tested on the combination",
    train: (r, i) => (i % 2 ? statusDomain("T" + i) : gatedDomain("T" + i, r, 1, 3, false)), test: (r, i) => gatedDomain("H" + i, r, 2, 4, true) },
];
const median = (a) => { const s = a.slice().sort((x, y) => x - y); return (s[4] + s[5]) / 2; };
const fmt = (a) => `${median(a).toFixed(0).padStart(3)}% (min ${Math.min(...a).toFixed(0)})`.padEnd(15);
const wantRules = process.argv.includes("--rules");

console.log(`Unseen conversations reproduced exactly: median over ${SEEDS} seeds (worst seed). Pass mark: ${PASS}% at 20 conversations.\n`);
console.log("rung".padEnd(30) + "no rules".padEnd(15) + SIZES.map((n) => (n + " convs").padEnd(15)).join("") + "20, 10% wrong".padEnd(15) + "verdict");
for (const [ri, rung] of RUNGS.entries()) {
  const res = { base: [], noise: [] }, errs = {}; SIZES.forEach((n) => { res[n] = []; }); let shown = null;
  for (let seed = 0; seed < SEEDS; seed++) {
    const r = rng(1000 * (ri + 1) + seed);
    const trainDoms = Array.from({ length: DOMS }, (_, i) => rung.train(r, i)), testDoms = Array.from({ length: 6 }, (_, i) => rung.test(r, i));
    const names = trainDoms.map((d) => d.name);
    const train = Array.from({ length: SIZES.at(-1) }, (_, i) => simulate(trainDoms[i % DOMS], r, names, rung.corrections));
    const test = Array.from({ length: TEST_CONVS }, (_, i) => simulate(testDoms[i % 6], r, names, rung.corrections));
    for (const n of SIZES) {
      const exs = train.slice(0, n).flat(); label(exs);
      const model = learn(exs); res[n].push(evaluate(model, test, n === 20 ? errs : null).conv);
      if (n === 20) { res.base.push(evaluate(learn(exs, 0), test).conv); if (seed === 0) shown = model; }
    }
    const noisy = train.slice(0, 20).flat().map((e) => (r() < 0.1 ? { ...e, op: OPS.filter((o) => o !== e.op)[pick(r, OPS.length - 1)], arg: undefined } : { ...e }));
    label(noisy); res.noise.push(evaluate(learn(noisy), test).conv);
  }
  console.log(rung.id.padEnd(30) + fmt(res.base) + SIZES.map((n) => fmt(res[n])).join("") + fmt(res.noise) + (median(res[20]) >= PASS ? "PASS" : "FAIL"));
  if (ERRORS) console.log("  commonest mistakes at 20 conversations, all seeds: " + (Object.entries(errs).sort((a, b) => b[1] - a[1]).slice(0, 4).map(([k, v]) => `${k} (${v})`).join("; ") || "none"));
  if (wantRules) console.log(`  ${rung.what}\n  learned from 20 conversations, seed 0:\n${showRules(shown)}\n`);
}
