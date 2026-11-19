# Design 4.0: evaluation and data

**Status.** Proposal for the design session, written on 2026-11-19. It covers the derivation benchmark, the curriculum, the Haiku call budget, the three-layer knowledge base, the spike's benchmark and knowledge base, baselines, and the stopping rules. Each section gives options and one recommendation.

**Binding inputs.** Decisions P1-P16 in `research/DESIGN_4_preplanning.md`:
- generalisation first;
- teaching only through structured formal conversation with Haiku, plus the agent's own questions;
- no injection;
- a three-layer knowledge base;
- a derivation benchmark whose answers are never taught;
- household domains only;
- 5,000 Haiku calls as a hard limit;
- thresholds set after the spike.

**Lessons used.** From 3.0: `research/DESIGN_3_experiments.md`, `research/PREREG_3.md`, `tools/design3/HELDOUT_SPEC.md` and `data/heldout3/READINGS.md`.

**Inventory.** Every count below comes from read-only scripts run over `data/`. No model was called and no data file was changed. H3 (`data/heldout3/`) was not opened. Only its file counts and task names were compared, and the names only by script.

Rule names R1-R9 used throughout are defined in section 5.2.

---

## 0. Summary of recommendations

| Area | Recommendation |
|---|---|
| Benchmark | About 900 questions written by sub-agents from a spec, before any relational teaching and without sight of agent code, then split at random into **dev 300** (readable) and **sealed 600** (hashed, at most 3 readings). A separate **spike set of 100** is hand-checked and becomes dev-only afterwards |
| Gold | Two golds per question. The **world gold** is fixed, sealed and authored. The **KB-relative gold** is computed at each reading by an independent reference checker over what was actually taught. Together they separate "not taught" from "not derived" |
| Scoring | Every answer falls in exactly one class: correct, wrong (valid but defeated or wrong-premise proof), invented (claim without a valid proof), gap (correct or missed), or hypothesis (correct or wrong). Proofs are checked by a second implementation of the base rules, never by the agent's own code |
| Leakage | A deny list holds the gold answer, every intermediate node of the gold proof, and the withheld pieces of the gap questions, at three normalisation levels. `tools/design4/leak_check.py` runs at five points: prompts, teacher replies, agent questions, archive imports and the reading audit. Hits are quarantined. One delivered hit invalidates the run |
| Power | 2-hop n = 200 in the sealed set gives ±6.9 points. Paired contrasts on 600 questions detect about 2.6-point differences, and on one 200-question stratum about 7.8 points. 0 inventions in 600 claims gives a 95% upper bound of about 0.6% |
| Curriculum | Stages 0-7 (vocabulary, kinds, materials and properties, functions, parts, actions and states, procedures, situations and exceptions). There is one fixed system prompt per stage. A call batches 20-30 statements plus answers to up to 10 agent questions. A symbol registry is in every prompt |
| Budget | 2,510 planned calls, 1,490 conditional on scaling paying (P10), and a 1,000 reserve, for 5,000 in total. The spike uses 0. 3.0's cache is reused as **archived teacher messages** (1,385 formal procedure readings and 295 situation readings) at 0 calls |
| Knowledge base | Raw: JSONL per session, append-only. Facts: Datalog text per build. Provenance: JSONL per build. Every record carries a schema version. `sla4 replay` rebuilds facts from raw deterministically, and curve points are replays of raw prefixes at 0 calls |
| Baselines | B0 retrieval only (also a leakage alarm), B1 base rules only, B1-k one rule off at a time, B3 Haiku open-book, B4 Haiku closed-book, B5 3.0's X4 fact-space cases (19.2% change recall), B6 a naive sibling stand-in |
| Gates | G0 benchmark ready; G1 after the spike; G2 after KB size S (sealed reading 1, then pre-registration); G3 scaling S to M to L; G4 final. Every gate has a fixed form, and the numbers are fixed by formula from the spike |

---

## 1. The derivation benchmark

### 1.1 What it must measure

The benchmark measures one claim: **the agent creates new, correct knowledge from what it was taught, shows why, and claims nothing it cannot support.** 3.0 gives three warnings.

1. **A single held-out set read once tells little.** H3's tiers had 60 cases, so each tier's interval was about ±12 points. No paired test reached significance (PROGRESS: "no significant gain from adaptation, rules arm, C2, distillation"). Section 1.6 sizes 4.0's benchmark so that its main contrasts can be decided.
2. **Measures can move more than the agent does.** 3.0's biggest second-encounter gain, from 16 to 29 of 71, came from changing the name matcher, not from the agent (experiments, phase B finding 4). In 4.0 the scorer, the normalisation and the proof checker are frozen with the benchmark and versioned with it.
3. **Hand splits are easier than random ones.** In X8 the hand dev split scored 56%, against 38% for random splits. In 4.0, dev and sealed are a random stratified split of one authored pool, so dev is not easier by construction.

### 1.2 Question types

The questions vary along two axes: a **type** (what kind of new knowledge is needed) and a **hop count**. The hop count is the minimum number of rule-application nodes in a valid proof under the base rules alone. Taught leaves and the question's own premises count 0, and learned rules may shorten a proof but never change the label.

| Code | Type | What the agent must produce | Gold kind |
|---|---|---|---|
| K | **Kind reasoning:** inheritance and transitivity over `is_a`, `part_of` and `same_as` | A derived relation fact | `yes` + proof, or bindings + proof |
| SUB | **Substitution:** a step's item is missing and the user has others (rule R5, with its requirement checks) | `stand_in(Step, Missing, Item)` or none | bindings + proof, or a gap |
| X | **Cross-domain:** the question's procedure fragment is in a domain never taught (gardening), or in a taught domain other than the one where the pieces were taught | As SUB or K | as SUB or K |
| EXC | **Exception:** a general fact is overridden by a more specific contrary one (Q2's specificity) | Usually `no` + proof showing the defeat | `no` + proof |
| STEP | **Step replacement:** another action `achieves` the state a later step `requires` (rule R6) | `replace_step(Step, Action)` | bindings + proof |
| NAME | **Naming:** the answer needs `same_as` (rule R4), because the question uses one name and the teaching another | As K or SUB | as K or SUB |
| IND | **Induced:** no base-rule proof exists, but a pattern with enough support does | `hypothesis(...)` + support set, or a gap | `hyp` (preferred) or `gap` (acceptable) |
| GAP | **Honest gap:** a needed piece is deliberately withheld | `gap(missing: [...])` naming the missing piece | `gap` + the missing pieces |

**Hops** are labelled 1, 2 or 3 for K, SUB, X, EXC, STEP and NAME. IND questions are labelled by their hops including the learned rule, and reported separately. GAP questions are labelled by the hops they would have if the piece were present.

**"Near-miss" gaps.** A quarter of the GAP questions are one piece away from a valid proof, and they share entities with answerable questions. The tempting wrong answer is a plausible stand-in, such as a newspaper for a squeegee. These are where 3.0's case transfer made 20 wrong changes in 48 cases (X4).

#### Options for the type mix

- **Option 1: hops only.** 1/2/3-hop questions of kind reasoning only. Simple, but it does not test the household use (substitution) or exceptions.
- **Option 2: the type × hop grid above.** Every cell is reported, and the headline is pooled.
- **Option 3: natural distribution.** Sample questions from simulated user requests. Realistic, but 1-hop questions dominate, and the claim is about multi-hop.

**Recommendation: Option 2.** The target cell sizes are in 1.6, and 2-hop is the headline stratum because the G1 gate is stated on it.

### 1.3 Question, gold answer and gold proof format

One JSONL record per question, in `data/bench4/{spike,dev,sealed}/questions.jsonl`. The schema is `bench4/1`.

```json
{"schema": "bench4/1", "id": "bk-0031", "split": "dev", "author": "w2", "type": "SUB", "hops": 3,
 "domains": ["baking"], "context_domain": "baking", "kb_scope": "S",
 "query": "stand_in(bake_banana_bread_s4, loaf_tin, X)",
 "given": ["lacks(user, loaf_tin)", "has(user, muffin_tray)", "has(user, plastic_tub)"],
 "gold": {"kind": "bindings", "answers": [{"X": "muffin_tray"}], "excluded": [{"X": "plastic_tub"}]},
 "gold_proof": [
   {"n": 1, "atom": "role(bake_banana_bread_s4, loaf_tin, contain)", "by": "taught"},
   {"n": 2, "atom": "is_a(muffin_tray, baking_tin)",                 "by": "taught"},
   {"n": 3, "atom": "can_do(baking_tin, contain)",                    "by": "taught"},
   {"n": 4, "atom": "can_do(muffin_tray, contain)",                   "by": "R2", "from": [2, 3]},
   {"n": 5, "atom": "has_property(baking_tin, oven_safe)",            "by": "taught"},
   {"n": 6, "atom": "has_property(muffin_tray, oven_safe)",           "by": "R2", "from": [2, 5]},
   {"n": 7, "atom": "requires(bake, oven_safe)",                      "by": "taught"},
   {"n": 8, "atom": "action(bake_banana_bread_s5, bake)",             "by": "taught"},
   {"n": 9, "atom": "stand_in(bake_banana_bread_s4, loaf_tin, muffin_tray)", "by": "R5", "from": [1, 4, 6, 7, 8]}],
 "pieces": [1, 2, 3, 5, 7, 8],
 "deny": ["stand_in(bake_banana_bread_s4, loaf_tin, muffin_tray)", "can_do(muffin_tray, contain)",
          "has_property(muffin_tray, oven_safe)"],
 "withheld": [],
 "rules_needed": ["R2", "R5"],
 "review": {"world_true": true, "reviewer": "r1", "notes": ""}}
```

**Fields.**

- **`query`.** A formal goal in the 4.0 language.
- **`given`.** Per-question premises: the user's situation, or a procedure fragment for cross-domain questions. They are never stored. The agent answers each question in a read-only mode, and the harness checks that the facts layer's hash is unchanged after each question.
- **`gold`.** One of:
  - `yes` or `no`;
  - `bindings`, with an optional `excluded` list: candidates that look right but fail, which tests overgeneralisation;
  - `gap`, with `missing`;
  - `hyp`, with `rule` and `min_support`.
- **`gold_proof`.** At least one valid chain. Any other valid proof is also accepted, because the scorer checks proofs, it does not compare them.
- **`pieces`.** The leaves of the gold proof. They are what the curriculum must teach, and they feed the KB-relative gold and coverage.
- **`deny`.** Every non-leaf node of the gold proof, including the answer. Teaching an intermediate node would turn a 3-hop question into a 1-hop one and make the hop label false.
- **`withheld`.** For GAP questions only: the pieces that must not be taught. They are deny-listed too.
- **`kb_scope`.** The smallest knowledge-base size (S, M or L) whose curriculum covers the question's entities.

**The agent's answer** is formal text, rendered from the reasoner's `Derivation` tree (`crates/reasoner` already exposes `leaves()`, `rules_used()` and `depth()`):

```
answer: stand_in(bake_banana_bread_s4, loaf_tin, muffin_tray).
proof:
  [1] role(bake_banana_bread_s4, loaf_tin, contain).    taught s0041-t003
  [2] is_a(muffin_tray, baking_tin).                    taught s0007-t012
  [4] can_do(muffin_tray, contain).                     R2 [2,3]
  ...
excluded: plastic_tub   because no proof of has_property(plastic_tub, oven_safe)
```

The other answer forms are:
- `gap: missing can_do(newspaper, F) with role(clean_window_s3, squeegee, F)`;
- `hypothesis: can_do(lemon, remove_limescale) by L0007 support [vinegar, citric_acid, cola] conf 0.75`.

The JSON form of the answer, which the scorer reads, has the same fields. Each leaf cites the raw message id it came from.

### 1.4 Two golds: world and KB-relative

The benchmark is written before teaching (1.9), and Haiku teaches from its own knowledge without being given the pieces (P2). So some gold pieces will not be taught, or will be taught in another form: `used_for(tray, baking)` instead of `can_do(tray, contain)`.

**Options.**

- **(a) World gold only.** Questions whose pieces were not taught count as failures. The score then mixes curriculum coverage with reasoning, and the agent cannot be blamed for a fact nobody taught.
- **(b) Give Haiku the pieces to teach.** This makes the benchmark's pieces a hand-written knowledge base passed through Haiku, which breaks P2, and the answers sit next to the pieces in the prompts.
- **(c) Both golds (recommended).**
  - The world gold is fixed and sealed.
  - At each reading, a **reference checker** (`tools/design4/ref_reason.py`) computes the KB-relative gold. It is a second, independent Python implementation of the frozen base rules, run over the teacher statements actually delivered (the raw layer, not the agent's facts). It never sees the agent's code or output.
  - For each question the reference checker reports whether the world-gold answer is derivable from what was taught, and what it derives instead.

From (c), three numbers are reported at every reading:
- **Coverage:** the share of questions whose world gold is KB-derivable. This measures the curriculum.
- **Conditional accuracy:** correct given derivable. This measures the reasoning, and is the cleanest measure of generalisation.
- **Headline correct:** correct against the world gold, with a valid proof. This is the measure of record, as P15 requires.

The world gold stays the measure of record. The KB-relative gold is diagnostic, so a gap on an uncovered question gets no credit in the headline. It is reported as a "justified gap" in the breakdown.

### 1.5 Metrics

Each answer falls in exactly one class.

| Class | Definition |
|---|---|
| **Correct** | Matches the world gold. Every proof checks: each leaf is a delivered teacher statement or a `given` premise, each step is an instance of a base rule or of a learned rule whose own support checks, and no step uses an inheritance defeated by a more specific contrary fact. For `bindings`, all gold answers are found and no `excluded` candidate is claimed |
| **Wrong** | A claim whose proof is structurally valid but whose conclusion is not the world gold. Two kinds: **defeated** (the proof uses an inheritance that a more specific taught fact overrides, the missed-exception case) and **wrong premise** (a taught statement is false in the world, which is a teacher error and is traced to its message) |
| **Invented** | A claim with no proof, a leaf that was never delivered, or a step that is no rule instance. **This includes correct answers with an invalid proof.** Target 0, as in the pre-planning |
| **Gap: correct** | No claim, where the world gold is `gap` (designed gaps) or where the KB-relative check says the world gold is not derivable (justified gaps). Gap precision is also reported: whether the named missing piece is one that would complete a proof |
| **Gap: missed** | No claim, where the world gold was derivable from what was taught |
| **Hypothesis: correct or wrong** | Marked as a hypothesis, with a support set that checks (at least `min_support` distinct examples, from at least 2 tasks or domains, and no delivered counterexample). Correct or wrong against the world gold. A hypothesis stated as a plain fact without its marker counts as invented |

**Derived rates** (all reported with Wilson 95% intervals):
- correct rate;
- invention rate, as a share of all claims;
- wrong rate;
- correct-gap rate on designed gaps;
- false-gap rate;
- **proof validity:** claims with a valid proof over all claims (the complement of invention, reported for comparison with Haiku);
- coverage and conditional accuracy;
- hypothesis precision.

Each rate is broken down by hop count, type, domain, `kb_scope`, curve point and knowledge-base size.

**Cost** is reported per question: milliseconds, facts touched and maximum proof depth. The pre-planning counts inference explosion as a risk.

### 1.6 Sizes and power

**Assumptions.** α = 0.05 two-sided, power 80% (z = 1.96 and 0.84). Accuracies are near 0.5, which is the worst case for interval width.

**One proportion.** The Wilson or normal half-width is 1.96 × √(p(1−p)/n). At p = 0.5:

| n | 40 | 60 | 80 | 100 | 150 | 200 | 300 | 600 |
|---|---|---|---|---|---|---|---|---|
| ± points | 15.5 | 12.7 | 11.0 | 9.8 | 8.0 | 6.9 | 5.7 | 4.0 |

For example, at n = 200: 1.96 × √(0.25/200) = 1.96 × 0.0354 = 0.069.

**Zero inventions.** If 0 of n claims are invented, the one-sided 95% upper bound is about 3/n (rule of three). The two-sided Wilson upper bound is a little larger: 3.7% at n = 100 and 1.3% at n = 300.

| Claims n | 100 | 300 | 600 |
|---|---|---|---|
| Upper bound on the invention rate | 3.0% | 1.0% | 0.5% |

**Paired contrasts** are learned rules on against off, curve point 50% against 100%, size S against L, and agent against baseline, all on the same questions. They use the exact McNemar test. With discordant share p_d = p10 + p01 and effect δ = p10 − p01:

n = (1.96·√p_d + 0.84·√(p_d − δ²))² / δ²

- δ = 0.10, p_d = 0.20 (15% gained, 5% lost): (1.96·0.447 + 0.84·0.436)² / 0.01 = (0.877 + 0.366)² / 0.01 = 1.544 / 0.01 = **155 questions**.
- δ = 0.05, p_d = 0.10: (0.620 + 0.262)² / 0.0025 = 0.778 / 0.0025 = **312 questions**.
- With p_d = 2δ (gains twice the net), the minimum detectable effect is 19.1 points at n = 80, 15.3 at n = 100, 7.8 at n = 200, 5.2 at n = 300 and **2.6 at n = 600**.

**Unpaired** comparisons, such as the agent against another system on different questions, need 388 per arm for 50% against 60%. That is why every comparison is designed to be paired.

**Clustering.** Questions that share pieces are not independent: one wrong `is_a` fact can sink ten questions. Three measures handle this:
- the author spec caps each taught piece at **5% of the gold proofs in a split**;
- intervals are also reported by **cluster bootstrap**, resampling entities (2,000 resamples);
- if the bootstrap interval is more than 1.3 times the Wilson interval, the bootstrap one is reported as primary.

**Sizes and strata.**

| Part | Size | Readable? | Purpose |
|---|---|---|---|
| Spike set | 100 | Yes, hand-checked | G1. 2-hop stratum n ≈ 36, so ±16 points: enough for a go/no-go, not a precise number |
| Dev | 300 | Yes | Development, ablations and debugging. Paired minimum detectable effect about 5 points |
| Sealed | 600 | No: counts only until the final reading | Measure of record. 2-hop n = 200 (±6.9); paired minimum detectable effect 2.6 points overall and about 7.8 per stratum of 200 |

**Sealed composition** (dev is the same at half size, from the same random split):

| Type | Questions | 1-hop | 2-hop | 3-hop |
|---|---|---|---|---|
| K kind reasoning | 130 | 40 | 50 | 40 |
| SUB substitution | 130 | 25 | 55 | 50 |
| X cross-domain | 90 | 10 | 45 | 35 |
| EXC exception | 60 | 20 | 25 | 15 |
| STEP step replacement | 40 | 10 | 15 | 15 |
| NAME same_as | 20 | 5 | 10 | 5 |
| **Answerable deductive** | **470** | **110** | **200** | **160** |
| IND induced | 50 | (labelled with the learned rule) | | |
| GAP honest gap (20 near-miss) | 80 | | | |
| **Total** | **600** | | | |

`kb_scope` targets: about 40% S, 30% M and 30% L. The fixed S-scope subset (about 240 sealed questions) is what the scaling gate G3 compares across sizes.

**Option considered: 200 sealed questions.** That is a third of the authoring work, but it gives ±11 points per hop stratum of about 70, too coarse to tell 2-hop from 3-hop or to see a learning curve of +5 points. **Recommendation: 600 sealed.** Authoring costs sub-agent time, not Haiku calls.

### 1.7 Keeping answers out of teaching: the deny list and its script

**What is denied.**
1. Every `deny` atom: the answer and every intermediate node of the gold proof.
2. Every `withheld` atom.
3. Their **trivial variants**:
   - the arguments of `same_as` swapped;
   - the negation of the atom (flagged as "contradicts gold", not blocked);
   - each argument replaced by any symbol the current knowledge base declares `same_as`, a closure that is recomputed as the knowledge base grows.

**Three normalisation levels.**

| Level | Normalisation | Action on a hit |
|---|---|---|
| L0 | Exact atom after whitespace and case normalisation | Block |
| L1 | Symbol form: lower case, singular (a tool-side rule list, allowed because the no-word-list rule covers agent crates only), separators unified, `same_as` closure | Block |
| L2 | Head-word form: last token of each argument | **Warn only.** `can_do(baking_tray, contain)` is a legitimate piece and must not be blocked by a denied `can_do(muffin_tray, contain)`. Warnings are counted and a sample is hand-checked |

**Sealed atoms are hashed.** At sealing, the main session runs `leak_check.py --build-deny sealed` without printing anything. The tool writes `data/bench4/sealed/deny.sha256`: SHA-256 hashes of the L0 and L1 forms, salted with a committed salt. The check then needs no clear text. L2 for the sealed part also works on hashes (head-word forms are hashed too), and it reports counts only. The `same_as` closure is applied on the teaching side: every alias form of a candidate statement is hashed and looked up.

**Where it runs** (`tools/design4/leak_check.py`, called by the teaching harness):

| Point | Checks | On a hit |
|---|---|---|
| P1: every outgoing teacher prompt | The prompt contains no denied atom (so the teacher is never told an answer) | The call is not made; harness error |
| P2: every teacher reply, before the agent sees it | Each statement in the reply | The statement is **quarantined**: removed from the message delivered to the agent, stored in raw with `delivered: false, quarantine: "deny"`, and counted. The rest of the message is delivered |
| P3: every agent question | An agent `ask` whose canonical form matches a denied atom | The question is **dropped without a reply**: no call, recorded with `speaker: harness, event: ask_dropped`. Replying "unknown" would tell the agent something, so the drop is silent |
| P4: every archive import (2.0/3.0 seeds) | Each imported statement | Quarantined as at P2 |
| P5: reading audit, before scoring | The whole delivered raw layer is re-checked under the current `same_as` closure | **Any delivered hit invalidates the reading.** Output is counts only for the sealed part |

**Cross-benchmark consistency.** The validator rejects a benchmark in which one question's denied atom is another question's piece. That would force the curriculum either to leak or to under-teach. The validator also rejects a GAP question whose withheld atom is a piece of any other question.

**Retrieval as an alarm.** Baseline B0 (section 6) answers only what is literally stored. On answerable deductive questions it must score 0. Any B0 hit means a leak that the script missed, for example through a relation the normaliser does not know.

**Readings never teach.** During a reading, the agent's gap questions are logged and thrown away. In 3.0, cases had been quietly stored with the scenario's situation facts (phase B finding 7); the 4.0 harness asserts the facts-layer hash before and after every reading.

### 1.8 Dev and sealed parts; sealing and read budget

**Options.**
- (a) Dev and sealed written separately, with different writers.
- (b) **One pool, split at random and stratified by type × hop × `kb_scope` (recommended).** It avoids X8's finding that the first-written split was easier.

**Procedure.**
1. The writers produce about 950 candidates.
2. The validator and reviewer (1.9) keep about 900 or more.
3. A script with a committed seed draws 300 for dev and 600 for sealed.
4. The script writes `data/bench4/sealed/MANIFEST.sha256` and `deny.sha256`.
5. A test in `cargo test` checks the manifest, as for H3.

The main session reads dev only.

**Read budget for the sealed part: 3 readings**, logged in `data/bench4/sealed/READINGS.md` with date, commit, build and what was run.

| Reading | When | What runs |
|---|---|---|
| R1 | After knowledge-base size S is taught (gate G2) | All curve points of S (replays), B0, B1, the B1-k ablations and B3 at 100% of S |
| R2 | After size M (gate G3) | M's curve points, and S again on the S-scope subset |
| R3 | Final, one frozen build | Every size, every curve point, every baseline |

**Rules.**
- Counts only until R3.
- No sealed transcript is read before R3.
- Changes between readings may use dev only.
- After R1, `research/PREREG_4.md` freezes the targets for R3, with the same rule as `PREREG_3.md`: targets are set once, after the first reading, and never lowered.

**Option considered: one sealed reading only.** That is the most rigorous, but then no gate can use sealed data, and dev has already been tuned on. Three readings with counts only and a frozen pre-registration after R1 worked procedurally in 3.0 (`data/heldout3/READINGS.md`). **Recommendation: 3.** A fourth only by explicit user decision, logged.

**H3 stays sealed and is not used by 4.0.**
- Its 90 T and G task names go on 4.0's name-level deny list for teaching, as in `gen_dataset.py`.
- 4.0's benchmark does not reuse H3. H3 was read 3 times, and its inputs are English, not formal.

### 1.9 Who writes the benchmark

**Writers.** Claude Code sub-agents, working from `tools/design4/BENCH_SPEC.md`. They do not count against the Haiku ledger, and their authoring is logged in `data/bench4/AUTHORING.md`.

**Inputs the writers may see:**
- the spec;
- the relation set and the base rules, written as logic, not code;
- the **symbol registry** from curriculum stage 0 (names only, no relations);
- the 3.0 procedure ground truth for the seed tasks, which is public data. Writers need step ids and roles to write SUB questions.

**What they may not see:**
- `crates/` and the 4.0 agent tools;
- the teaching prompts;
- any raw-layer teaching beyond the registry;
- agent output;
- H3.

**Who does what:**
- **Four writers**, one per domain group (baking and cooking; cleaning; repair; errands, travel, crafts and personal care). Each also writes its share of cross-domain gardening questions.
- **One reviewer** sub-agent per question batch checks the **world truth** of each gold, and whether an obvious counter-proof exists. A question is kept only when writer and reviewer agree. The agreement rate is reported, and G0 requires at least 90% agreement before dropping.
- **The validator** (`tools/design4/check_bench.py`) checks that:
  - every record has the right schema and uses only registry symbols (or `given` symbols for X questions);
  - the reference checker derives the gold from `pieces` plus `given` at the stated hop count;
  - the proof is **minimal**: removing any piece breaks every proof, so the hop count is real;
  - GAP questions are not derivable without their withheld atoms, and are derivable with them;
  - no denied atom of one question is a piece of another (1.7);
  - no piece is in more than 5% of a split's proofs;
  - no question repeats another up to renaming.

**Order of work:** curriculum stage 0 (vocabulary) runs live first. Then the benchmark is authored, validated, split and sealed. Then the deny lists are built. Only then do stages 1-7 run. The benchmark therefore cannot be shaped by what Haiku taught, and teaching cannot be shaped by the benchmark, except through the deny list, which only removes.

### 1.10 Learning-curve protocol

**Curve points cost 0 calls.** The raw layer is chronological and append-only, and the agent is deterministic. Replaying the first k% of the raw layer under the same agent build therefore rebuilds exactly the state at that moment. `sla4 replay --upto 25%` is the snapshot, and a test checks byte equality with the facts layer saved live at the 25% mark.

**How the curriculum is ordered.**

- **Option A: stage order** (all kinds, then all functions, ... then procedures). At 25%, every substitution question is a gap, so the curve mainly shows stage order.
- **Option B: four quarter-blocks (recommended).** The topic units (stage × domain × entity group) are randomly assigned, stratified, to four blocks. Each block runs stages 0-7 in order for its own units. Every prefix is then a representative sample of the curriculum, and the curve measures quantity of teaching, not stage.

**Points.**
- Sizes S, M and L are nested: M adds four domains to S, and L adds depth (more entities per domain).
- Each size is taught in four blocks, giving **12 curve points**: S25, S50, S75, S100, M25, M50, M75, M100, L25, L50, L75, L100.
- At every point the benchmark reports correct, invented, coverage and conditional accuracy, with learned rules on and off.

**Paired curve tests.**
- **Pre-registered primary contrast:** 50% against 100% within L, and S100 against L100 on the S-scope subset.
- **Learned-rule contribution as the knowledge base grows:** the on-minus-off difference at S100, M100 and L100. P7 expects it to rise, because more examples mean more induced rules that clear the support threshold.

**Robustness checks at 0 calls.**
- **Order-shuffled replay:** blocks are permuted, and the agent's questions are answered from raw only when an identical canonical question exists. Unanswered questions are counted. This tells whether the order of teaching matters.
- **Teacher variance:** one replicate teaching run of size S with a different block seed and prompt sampling order (300 calls, section 3) gives a range for S100.

---

## 2. The curriculum

### 2.1 Stages

| Stage | Relations taught | Topic unit | Driven by |
|---|---|---|---|
| 0 Vocabulary | Names: `item(X)`, `material(X)`, `action(X)`, `state(X)` declarations, `same_as`, and `not same_as` | Domain × group (tins, cloths, fasteners, ...) | Seed item lists from 3.0 (412 items, 285 actions), plus Haiku's additions |
| 1 Kinds | `is_a` | Entity group | The registry |
| 2 Materials and properties | `made_of`, `has_property`, and negatives | Entity group | The registry |
| 3 Functions | `can_do`, `used_for` | Entity group; also each of 3.0's 76 role symbols ("what can `pierce`?") | 3.0 roles: 32 role symbols have 3 or more distinct items in 3.0 |
| 4 Parts | `part_of` | Appliances, fixtures and tools with parts | The registry |
| 5 Actions and states | `achieves`, `requires` | Each action in the seed procedures | 3.0 actions and `until` states |
| 6 Procedures | v3 procedure facts (`task`, `step`, `needs`, `action`, `object_of`, `uses`, `role`, `before`, ...) | One task | 3.0 seeds (2.3) |
| 7 Situations and exceptions | `has`, `lacks` cases with their resolution, and negative facts for exceptions | One task × situation kind | 3.0 variant kinds (missing tool, exclusion, time limit, count, broken item) |
| Agent questions | Any | Asked by the agent | Interleaved, budgeted (2.4) |

Cleaning, cooking and repair appear in every block. **Gardening is never taught** (its 5 gardening tasks in `data/curriculum3/heldout_domain/` and the cached gardening readings are excluded). It exists only as question context for cross-domain questions, which follows 3.0's domain tier.

### 2.2 What Haiku is asked to teach per call

**Options.**
- (a) One fact per call: clean, but it wastes the budget.
- (b) Free-form "teach me about baking": cheap, but the names drift and the coverage is uncontrolled.
- (c) **Batched formal statements about a named topic unit, with the registry in the prompt and the agent's pending questions answered first (recommended).**

**System prompt.** There is one per stage, fixed and versioned (`tools/design4/prompts/stage3_v1.txt`), so cache keys are stable and replays are exact. A sketch:

```
You teach a learner household knowledge in a formal language. Reply with statements only,
one per line, ending in '.', no prose.
Relations: is_a(X, Kind). can_do(X, Function). used_for(X, Purpose). has_property(X, P).
made_of(X, M). part_of(P, X). achieves(Action, State). requires(Action, P). same_as(X, Y).
A negative fact is written: not R(...).
Use only the symbols in REGISTRY. If you need a new symbol, first declare it:
new(symbol, item|material|action|state|property|function) and say what it is: is_a(symbol, ...).
State only what is true for typical households. Prefer general facts about kinds over facts about
single items, and state exceptions as negative facts about the more specific kind.
First answer the learner's QUESTIONS, each under 'answer N:', then teach the TOPIC.
```

**User prompt.** It contains:
- the stage;
- the topic unit;
- the registry slice for the unit (40 or fewer symbols, plus the 30 most frequent general kinds);
- up to 10 agent questions;
- "teach 20-30 statements";
- the last 2 teacher-agent exchanges of the session, for context.

It never contains benchmark material. Check P1 runs on every prompt.

**Yield.** Pilot T0 (section 3) measures the statements accepted per call: schema-valid, registry names, not duplicates, not quarantined. The plan assumes **15 net facts per call** for stages 1-5 and one task per call for stage 6. If the pilot gives less than 10, the plan is re-cut before stage 1.

**Teacher quality audit.** Per stage, 100 random delivered statements are hand-checked for world truth. 3.0 found that its errors were naming, not content (finding 4). The audit tells whether that holds in formal teaching. A threshold is in G2.

### 2.3 Seeding from 2.0 and 3.0: re-taught, never injected

**What exists**, all counted by script:

| Source | Count | Content |
|---|---|---|
| `data/curriculum3/` | 105 tasks (train 52, dev 19, seed 29, gardening 5); 563 teach lines | v3 ground truth. In train, dev and seed: 269 + 91 + 121 needs facts, 236 + 74 + 97 role facts |
| `data/dataset3/` | 98 valid tasks in 8 domains, plus 4 rejected or leak files; 667 teach lines; 196 variants | v3, Haiku-generated (154 cached `dataset_file` calls) |
| Union of v3 tasks | 203 tasks; 8,612 ground-truth facts; 412 items; 285 actions; 76 role symbols; 395 distinct item-role pairs (132 seen in 2 or more tasks); 46 items in 2 or more domains | |
| `data/curriculum/` (2.0) | 105 files (scenarios 20, seed 30, training 55); ten-predicate ground truth with no roles | Superseded by the v3 re-authoring of the same tasks |
| `data/training/calls3/cache.jsonl` | 4,930 lines, 4,331 ok, 0 malformed | Tags in 3.1 |
| `data/training/calls/` (2.0) | 12,623 records: translator 8,288, curriculum judge 2,286, labeller 1,864, variant teacher 185 | English to the 2.0 act language |

**Seeding routes.**

1. **Archived teacher messages, 0 calls (recommended for procedures).**
   - 3.0's `parse_fallback` answers are Haiku's own formal statements in the procedure schema, which 4.0 keeps (Q1).
   - Those for train and dev tasks (1,385 answers, 6,735 fact lines, 71 tasks) are imported into raw as `channel: archive3` teacher messages. Each keeps its cache key, and messages are grouped into one session per task in step order.
   - They go through the agent's full conversation path: schema check, consistency check, provenance, derivation and gap questions. The agent's questions about them are answered live, within the question budget.
   - **Excluded:**
     - 948 answers on 90 tasks unknown to the curriculum, which matches the 60 T + 30 G tasks of H3, so it is treated as sealed H3 content;
     - 34 gardening answers (gardening is untaught);
     - 338 `baseline_b2` answers, which are Haiku's answers to H3 questions.
   - `situation_fallback` messages: only the **295** whose message text matches a non-H3 request or situation line are importable. The other 1,053 are quarantined.
2. **Topic seeds, no content passed.** The task names, item lists and role symbols of all 203 v3 tasks drive the stage 0, 3, 6 and 7 topic units. For dataset3 tasks, which have no cached formal readings, stage 6 asks Haiku live, one task per call, with only the task name and the step outline (action and object per step). This is the same information a user request gives.
3. **Agreement check, never delivered.** The v3 ground truth is used only to score how well the taught procedures agree with it, like M3 in 3.0. It is not fed to the agent.
4. **Not used for teaching:**
   - the 2.0 translator and labeller archives (English and an old formal language), kept for the later controlled-English layer (P4, option B);
   - the x7 paraphrases (English);
   - the 95 `teacher_answer` replies (English, and they contain variant answers that may overlap SUB questions).

**Open question for the user.** Are archived 3.0 teacher messages "teaching through conversation" (P2, P12)? The plan treats them as teacher messages from a past session, replayed through the agent's conversation path, and an ablation drops channel `archive3` at 0 calls. If the user says no, stage 6 for the 71 tasks costs about 85 more live calls, which fits in the reserve.

### 2.4 The agent's questions

**Why the agent asks** is fixed by the agent design. The kinds the harness budgets for, in priority order:
1. **Contradiction:** a fact and its negation, or an exception candidate.
2. **Naming:** a new symbol with a close known symbol by head word or edit distance ("same_as(egg_whisk, whisk)?").
3. **Typing:** a symbol with no `is_a`.
4. **Thin function:** a role with only one item known ("what else can_do beat?").
5. **Blocked derivation:** the agent tried to derive something during teaching and failed by one piece.

**Budget.**
- Questions **ride along** in teaching calls: up to 10 per call, answered under `answer N:` before the topic.
- **Question-only calls are capped at 20%** of a stage's teaching calls (as Q7 recommends).
- Unanswered questions carry over. When the queue is full, the lowest-priority ones are dropped and logged.
- Reported per stage: questions asked, answered, dropped at P3, and **question yield** (accepted facts per answered question, compared with per-topic teaching). If questions yield less than topic teaching after stage 3, their share drops to 10%.

### 2.5 Naming consistency

Naming drift was 3.0's largest measured error source (finding 4: "shelve_and_drawer" against `shelf`, "bananas" against `banana`). Four measures meet it.

1. **One registry,** `data/kb4/facts/<build>/vocab.jsonl`, built by the agent from stage 0 and every `new(...)` statement. Its slice is in every prompt.
2. **The agent rejects unknown symbols:** a statement with an undeclared symbol is not stored, and the agent asks, in the next call, for a declaration or a `same_as`.
3. **Near-symbol questions** (2.4, kind 2). The answer, `same_as` or `not same_as`, is taught knowledge with provenance.
4. **Measured each stage:**
   - the new-symbol rate per call, which should fall as the registry fills;
   - the `same_as` rate;
   - the share of statements rejected for naming.

   A rising new-symbol rate is a stopping signal (7.3).

Benchmark questions use registry symbols only, so the benchmark and the teaching share one vocabulary by construction. NAME questions use registry symbols that the curriculum is expected to join by `same_as`.

---

## 3. Haiku call budget (5,000, hard limit)

### 3.1 Starting point and reuse

**4.0 gets a new ledger.** The 3.0 ledger records **4,533 successful calls** in 5,130 lines (PROGRESS says 4,486, from an earlier count), so 3.0's 5,000 is almost spent.

- **This plan assumes P14's 5,000 is a fresh allowance for 4.0,** in its own ledger `data/training/calls4/{cache,ledger}.jsonl`. `tools/design4/haiku.py` is a copy of `tools/design3/haiku.py` with a new folder and `BUDGET = 5000`.
- **If the user meant the 3.0 remainder (about 467-514),** only pilot T0 and a reduced size S fit. This must be confirmed in the design session.

**The 3.0 cache in numbers** (successful answers, all kept unmodified):

| Tag | Ok answers | Reusable in 4.0 |
|---|---|---|
| `parse_fallback` | 2,367 | 1,385 (train and dev) as archived teacher messages. 948 (H3) and 34 (gardening) excluded |
| `situation_fallback` | 1,348 | 295 matched non-H3; 1,053 quarantined |
| `dataset_file` | 154 (441 lines in all) | Already in `data/dataset3`, as topic seeds |
| `baseline_b2` | 338 | No (H3 answers) |
| `teacher_answer` | 95 | No (English; possible SUB overlap); reference only |
| `x7_paraphrase` | 20 | No (English layer later) |
| `dataset_names` | 8 | Topic seeds |
| `smoke` | 1 | No |

**Cache hits.** The cache key is sha256(model, system, prompt). 4.0's prompts are new, so 3.0 cache hits will be about 0. Reuse comes through import into the raw layer, not through hits. Within 4.0 every repeated call (replays, reruns after a crash) is a hit, and costs nothing.

### 3.2 Plan per phase

| Phase | What | Calls | Basis |
|---|---|---|---|
| Spike | Offline, fixture knowledge base | **0** | P-spike |
| T0 Pilot | 2 prompt versions × 4 stages × 4 topic units, plus 8 question round-trips. Measures yield, naming and format | 40 | Decides batch size and yield |
| T1 Stage 0 vocabulary (S, M and L entity groups) | About 120 entity groups at 3 per call | 40 | Registry before benchmark authoring |
| **Size S** (baking, cooking, cleaning, repair) | Stages 1-5: about 2,000 facts at 15 net per call, plus 25% re-asks | 170 | |
| | Stage 6: 71 tasks from archive3 (0 calls); about 39 dataset3 tasks in these domains, live, plus 20% retries | 50 | |
| | Stage 7: situations and exceptions | 60 | |
| | Question-only calls (20% or less of S teaching) | 55 | |
| | R1 baseline B3 (Haiku open-book, 10 questions per call): dev 300 + sealed 600 | 90 | |
| **Size M** (+ errands, travel, crafts, personal care and planning) | Stages 1-7 and questions | 400 | About 2,500 more facts and 59 tasks |
| | B3 at M100 | 90 | |
| **Size L** (depth: more entities per group) | Stages 1-5 and 7, and questions | 750 | About 5,000 more facts |
| Replicate | Size S taught again with another seed (teacher variance) | 300 | 1.10 |
| Final baselines | B3 at L100 and L50 (180); B4 closed-book (90) | 270 | |
| Agent-version top-up | New questions that a changed agent asks during replay, answered live | 195 | 4.4, `--answers live` |
| **Planned subtotal** | | **2,510** | |
| **Conditional: size XL** | Only if G3 shows that L beats M (P10: scale only when it helps) | 1,490 | Breadth within household domains |
| **Reserve** | Retries, prompt fixes, re-teaching after a failed teacher audit, and the archive3 fallback (2.3) | 1,000 | 20% |
| **Total** | | **5,000** | |

**Rules.**
- `haiku.py` raises at 5,000, and fails fast on the first plan-limit refusal (3.0 finding 2).
- The harness also keeps **phase envelopes**: a phase that exceeds its row by 20% stops and asks for a re-plan.
- Reserve use is logged with its reason.
- Calls that Claude sub-agents make while writing the benchmark are not Haiku calls. They are logged separately.

---

## 4. The three-layer knowledge base

Layout:

```
data/kb4/
  raw/
    sessions/<session_id>.jsonl          append-only, one record per message
    imports/<source>.jsonl               import manifests (archive3, fixture), never edited
    INDEX.jsonl                          one line per session: id, channel, stage, block, first/last unix, sha256
  builds/<build_id>/
    facts.dl                             Datalog text: facts, negatives, exceptions, learned rules
    vocab.jsonl                          the symbol registry
    provenance.jsonl                     one record per stored fact, rule, exception or retraction
    BUILD.json                           agent commit, schema versions, raw prefix, base-rule hash, counts
  MANIFEST.json                          builds and their hashes
```

### 4.1 Raw layer (never changes)

One record per message, for every speaker: teacher, agent, harness and import.

```json
{"rec": "raw", "schema": "raw4/1", "lang": "formal4/1",
 "msg_id": "s0007-t012", "session": "s0007", "turn": 12, "unix": 1795000000,
 "speaker": "teacher", "channel": "live", "stage": 3, "block": 2, "topic": "baking/tins",
 "in_reply_to": "s0007-t011",
 "text": "answer 1: same_as(egg_whisk, whisk).\nis_a(muffin_tray, baking_tin).\ncan_do(baking_tin, contain).\nnot has_property(plastic_tub, oven_safe).",
 "statements": [
   {"i": 0, "atom": "same_as(egg_whisk, whisk)", "answers": "s0007-t011#q1", "delivered": true},
   {"i": 1, "atom": "is_a(muffin_tray, baking_tin)", "delivered": true},
   {"i": 2, "atom": "can_do(baking_tin, contain)", "delivered": true},
   {"i": 3, "atom": "not has_property(plastic_tub, oven_safe)", "delivered": true}],
 "call": {"key": "3f9a…", "ledger": "calls4", "prompt_version": "stage3_v1"},
 "quarantine": []}
```

An agent message in the same session:

```json
{"rec": "raw", "schema": "raw4/1", "lang": "formal4/1", "msg_id": "s0007-t013", "session": "s0007",
 "turn": 13, "speaker": "agent", "agent_build": "a4-9c1e2d0",
 "text": "ok. derived: can_do(muffin_tray, contain) [R2: s0007-t012#1, s0007-t012#2]\nask: is_a(plastic_tub, X)?",
 "asks": [{"q": "q1", "canonical": "is_a(plastic_tub, _)", "priority": "typing"}]}
```

An archive import record:

```json
{"rec": "raw", "schema": "raw4/1", "lang": "v3facts", "msg_id": "a3-bake_potato-03", "session": "a3-bake_potato",
 "turn": 3, "speaker": "teacher", "channel": "archive3", "stage": 6,
 "text": "step(bake_potato, bake_potato_s3)\naction(bake_potato_s3, prick)\nobject_of(bake_potato_s3, potato)\nuses(bake_potato_s3, fork)\nrole(bake_potato_s3, fork, pierce)",
 "source": {"file": "data/training/calls3/cache.jsonl", "key": "<cache key>", "tag": "parse_fallback", "unix": 1794900000}}
```

**Rules.**
- Records are never edited or deleted (memory: keep training data).
- A correction is a new message: `retract(...)` from the teacher, or an annotation record from a human that points to the `msg_id`.
- `delivered: false` marks quarantined statements. They are kept for audit, and replay never feeds them.
- `lang` names the formal-language version, so an old message is always read with its own grammar.

### 4.2 Facts layer (rebuilt, never migrated)

`facts.dl`:

```
% schema: facts4/1  build: b-2026-12-02-S100  agent: a4-9c1e2d0  raw: 5,412 msgs (prefix 100%)
% base_rules: R1-R9 sha256:7be1…
is_a(muffin_tray, baking_tin).                        % f:0001a2
can_do(baking_tin, contain).                          % f:0001a3
not has_property(plastic_tub, oven_safe).             % f:0001a4  exception-of f:00007c
same_as(egg_whisk, whisk).                            % f:0001a5
step(bake_potato, bake_potato_s3).                    % f:0020b1
role(bake_potato_s3, fork, pierce).                   % f:0020b5
% learned rules: id, layer, status, support
rule(l0007, 1, hyp): can_do(X, remove_limescale) :- has_property(X, acidic).   % support 3 conf 0.75
rule(l0012, 2, active): stand_in(S, I, J) :- role(S, I, F), has(user, J), can_do(J, F), ok_for(S, J).  % from R5+l0004
```

Only stored knowledge is written: taught facts, accepted exceptions and learned rules. Derived facts are computed on demand, because the facts layer must not grow by closure. The agent's "derived:" lines are in raw as conversation and can be recomputed.

`provenance.jsonl`:

```json
{"rec": "prov", "schema": "prov4/1", "id": "f:0001a2", "kind": "fact", "atom": "is_a(muffin_tray, baking_tin)",
 "source": {"msg_id": "s0007-t012", "i": 1}, "teacher": "haiku", "channel": "live",
 "conf": 0.95, "status": "active", "build": "b-2026-12-02-S100"}
{"rec": "prov", "schema": "prov4/1", "id": "l0007", "kind": "rule", "layer": 1, "status": "hyp",
 "head": "can_do(X, remove_limescale)", "body": ["has_property(X, acidic)"],
 "support": ["f:0031c0+f:0031c1", "f:0032a0+f:0032a4", "f:0040e2+f:0040e3"], "counterexamples": [],
 "built_from": ["induction"], "conf": 0.75, "first_msg": "s0031-t004"}
{"rec": "prov", "schema": "prov4/1", "id": "x:0003", "kind": "retraction", "target": "l0005",
 "reason": "counterexample", "by": "f:0051d2", "source": {"msg_id": "s0051-t008", "i": 0}}
```

### 4.3 Schema versioning

- **Version tags.** Every record carries `schema` as `<layer><major>/<minor>`, and raw also carries `lang`.
  - A minor version adds optional fields, and readers ignore unknown fields.
  - A major version changes meaning, which needs a new reader. Old readers refuse it.
- **Raw is never migrated.** The replay tool must read every raw version that has ever existed, and a test replays one frozen sample session of each raw version.
- **Facts and provenance are build outputs.** A schema change means a rebuild by replay. A build is therefore a function of (raw prefix, agent build, base-rule hash), recorded in `BUILD.json`.
- **Formal-language changes.** A relation can be renamed or split: if `can_do` were split into `can_do` and `affords`, the mapping lives in the agent's reader for `lang` formal4/1, not in the data.

### 4.4 The replay tool

```
sla4 replay --raw data/kb4/raw --agent <build> [--upto 25%|<msg_id>] [--blocks 1,3,2,4]
            [--channels live,archive3] [--answers raw|none|live] --out data/kb4/builds/<id>
```

**Behaviour.**
1. Read `INDEX.jsonl` and order sessions by block, then by time, or in a given block order.
2. Feed every delivered teacher, user and archive statement to the agent in order, through the same conversation entry point as live teaching. There is no bulk-load path, and a test asserts that the facts layer has exactly one writer.
3. **Agent questions under replay.**
   - Under a new agent version, the agent may ask questions that are not in raw.
   - `--answers raw` (the default) answers a question only when raw holds a teacher answer to the identical canonical question, at or before this point.
   - `none` leaves it unanswered.
   - `live` asks Haiku: the call is counted, appended to raw as a new session that points to the replay, and so the raw layer grows.
   - Every replay reports the number of unanswered questions. When it is above 5% of questions asked, the replay's results are marked "under-answered".
4. Write the facts layer, vocabulary, provenance and `BUILD.json`, plus a diff against a reference build (facts added or dropped by relation, and rules gained or lost).

**Determinism.** The same raw, agent build and flags give byte-identical outputs (test). Curve points and ablations are replays.

### 4.5 How 2.0 and 3.0 data map into the raw layer

| Source | Raw channel | Statements | Included by default |
|---|---|---|---|
| calls3 `parse_fallback`, train and dev (71 tasks) | `archive3` | 1,385 messages; 6,735 lines (after the P4 deny check) | Yes (ablation: off) |
| calls3 `situation_fallback`, matched non-H3 | `archive3` | 295 messages | Yes, as stage 7 situations, paired with their task session |
| calls3 `parse_fallback`, unknown or H3 (90 tasks) | Not imported | 948 | Never |
| calls3 gardening (`parse_fallback`) | Not imported | 34 | Never (untaught domain) |
| calls3 `baseline_b2`, `teacher_answer`, `x7`, `smoke` | Not imported | 454 | No |
| `data/curriculum3` and `data/dataset3` files | Not imported; topic seeds and the agreement check | 203 tasks | Topic lists only |
| 2.0 `data/curriculum`, `data/training/calls` | Not imported | 105 files; 12,623 calls | Reserved for the English layer |
| Spike fixture (section 5) | `fixture` | About 420 statements | **Spike only.** Never in the main raw layer |

---

## 5. The spike's benchmark and knowledge base

### 5.1 Knowledge base: about 420 teaching statements in 4 domains

The spike uses zero calls, so its teaching messages come from two places:

1. **Procedures:** the archive3 formal readings of 8 tasks, 2 per domain, all from 3.0 train and dev. For example `bake_banana_bread` (cooking, used as baking), `beat_egg_whites`-like tasks from the seeds, `clean_window`, and `fix_dripping_tap` if it is in train or dev; the spike writer picks from the list by script.
2. **Relations:** a **fixture written by a spike sub-agent from the spec**, delivered as teacher messages in channel `fixture` through the conversation path.

The fixture is a test fixture for the reasoning, not taught knowledge. It is never reused in 4.0's raw layer, so P2 holds for the real knowledge base. **The user should confirm this exemption.**

| Domain | is_a | made_of / has_property (incl. negatives) | can_do / used_for | part_of | achieves / requires | same_as | Procedure facts (2 tasks) | Total |
|---|---|---|---|---|---|---|---|---|
| Baking | 22 | 18 | 16 | 3 | 6 | 2 | 36 | 103 |
| Cooking | 22 | 16 | 18 | 4 | 8 | 3 | 36 | 107 |
| Cleaning | 20 | 18 | 20 | 3 | 7 | 2 | 34 | 104 |
| Repair | 18 | 14 | 14 | 12 | 6 | 1 | 36 | 101 |
| **Total** | 82 | 66 | 68 | 22 | 27 | 8 | 142 | **415** |

**Fixture rules.**
- Hierarchies are 3-4 levels deep, so that 3-hop chains exist.
- Every exception is a general fact plus a more specific negative.
- There are **3 induction support sets** of 3-4 examples each, plus 1 set with a taught counterexample and 1 set below the support threshold.
- **12 cross-domain links:** items whose kind is taught in one domain and used in another, such as kitchen scissors (cooking) used in gardening context, or vinegar (cooking) in cleaning.
- **2 decoy kinds** share a parent with the right answer but lack the required property: the plastic tub. The decoys and near-miss cases test that the agent does not overgeneralise.

**Hand-checkability.** 415 statements in 4 files of about 100 lines each. A person can read each file in about 15 minutes, and every gold proof cites line numbers.

### 5.2 Base rules used in the gold proofs

The rule ids follow Q3, so the spike's ablations can be scored per rule:

| Id | Rule |
|---|---|
| R1 | `is_a` is transitive |
| R2 | `can_do`, `used_for`, `has_property` and `made_of` inherit down `is_a`, defeated by a more specific contrary fact |
| R3 | `part_of` is transitive |
| R4 | `same_as` is an equivalence and substitutes in every relation |
| R5 | Stand-in: `role(S, I, F)`, `has(user, J)`, `can_do(J, F)`, and every `requires(A, P)` for the step's action holds as `has_property(J, P)`. Then `stand_in(S, I, J)` |
| R6 | Step replacement via `achieves` and `requires` |
| R7 | Contradiction raises a question (teaching only, never in a proof) |
| R8 | *Candidate:* `made_of(X, M)`, `has_property(M, P)` gives `has_property(X, P)`, defeasible |
| R9 | *Candidate:* a step's role implies an ability. `uses(S, I)` and `role(S, I, F)` give `can_do(I, F)` |

R8 and R9 are not in Q3's list. The spike measures whether they pay, and questions that need them are tagged, so they can be dropped from the spike set if the rules are rejected.

### 5.3 Spike questions: 100

| Type | n | 1-hop | 2-hop | 3-hop |
|---|---|---|---|---|
| K | 24 | 8 | 10 | 6 |
| SUB | 20 | 4 | 10 | 6 |
| X (gardening context) | 10 | 0 | 6 | 4 |
| EXC | 10 | 4 | 4 | 2 |
| STEP | 4 | 2 | 2 | 0 |
| NAME | 6 | 2 | 4 | 0 |
| IND | 10 | 4 with support, 3 with a counterexample (gold `gap`), 3 below the threshold (gold `gap`) | | |
| GAP | 16 | 6 of them near-miss | | |
| **Total** | **100** | 20 | **36** | 18 |

The G1 gate's 2-hop stratum has n = 36. 22 of 36 correct (61%) has a Wilson interval of 0.45-0.75. The gate's verdict is therefore read with that interval in view. Since the spike set is small and readable, the hand check covers every question.

### 5.4 Ten example questions with gold proofs

In each proof, `T` marks a taught fixture or archive statement, and `G` a `given` premise.

**E1. K, 1-hop, baking.**

```
?- can_do(muffin_tray, contain).
gold: yes
[1] T is_a(muffin_tray, baking_tin).
[2] T can_do(baking_tin, contain).
[3]   can_do(muffin_tray, contain).          R2 [1,2]
deny: can_do(muffin_tray, contain)
```

**E2. K, 2-hop, cleaning.**

```
?- can_do(microfibre_cloth, wipe).
gold: yes
[1] T is_a(microfibre_cloth, cleaning_cloth).
[2] T is_a(cleaning_cloth, cloth).
[3] T can_do(cloth, wipe).
[4]   can_do(cleaning_cloth, wipe).          R2 [2,3]
[5]   can_do(microfibre_cloth, wipe).        R2 [1,4]
deny: can_do(microfibre_cloth, wipe), can_do(cleaning_cloth, wipe)
```

The alternative proof R1 [1,2] then R2 is also 2 hops, and either is accepted.

**E3. SUB, 3-hop, baking (the worked example).**

```
?- stand_in(bake_banana_bread_s4, loaf_tin, X).
given: lacks(user, loaf_tin). has(user, muffin_tray). has(user, plastic_tub).
gold: X = muffin_tray; excluded: plastic_tub
[1] T role(bake_banana_bread_s4, loaf_tin, contain).
[2] T action(bake_banana_bread_s5, bake).  T requires(bake, oven_safe).
[3] T is_a(muffin_tray, baking_tin).  T can_do(baking_tin, contain).
[4]   can_do(muffin_tray, contain).                          R2
[5] T has_property(baking_tin, oven_safe).
[6]   has_property(muffin_tray, oven_safe).                  R2
[7]   stand_in(bake_banana_bread_s4, loaf_tin, muffin_tray). R5 [1,2,4,6]
plastic_tub: T can_do(plastic_tub, contain); T not has_property(plastic_tub, oven_safe)
             so the requirement fails and plastic_tub is excluded.
```

Claiming plastic_tub is **wrong** (its proof fails the requirement check), not invented.

The requirement in R5 is read across steps: s4 fills the tin and s5 bakes it. The spec states that `requires` checks apply to every later step that acts on the same item (the `before` chain, plus `object_of` or `into`). This is a design detail the spike must settle.

**E4. EXC, 2-hop, cooking.**

```
?- has_property(cast_iron_skillet, dishwasher_safe).
gold: no
[1] T is_a(cast_iron_skillet, cast_iron_pan).
[2] T is_a(cast_iron_pan, pan).
[3] T has_property(pan, dishwasher_safe).
[4] T not has_property(cast_iron_pan, dishwasher_safe).
[5]   not has_property(cast_iron_skillet, dishwasher_safe).  R2 [1,4]; [4] is more specific than [3] (depth 1 against 2)
deny: not has_property(cast_iron_skillet, dishwasher_safe), has_property(cast_iron_skillet, dishwasher_safe)
```

Answering "yes" through [3] is **wrong (defeated)**.

**E5. GAP, near-miss, cleaning.**

```
?- stand_in(clean_window_s3, squeegee, X).
given: lacks(user, squeegee). has(user, newspaper). has(user, sponge).
gold: gap; missing: can_do(newspaper, wipe_dry) (or any J with can_do(J, wipe_dry))
pieces present: T role(clean_window_s3, squeegee, wipe_dry). T is_a(newspaper, paper).
                T can_do(sponge, absorb).  (absorb is not wipe_dry, so it is a decoy)
withheld: can_do(newspaper, wipe_dry), can_do(paper, wipe_dry)
```

Claiming newspaper or sponge is **invented** (no proof), the same failure 3.0's X4 noted for "what a newspaper can do to a window".

**E6. X, 2-hop, gardening context with pieces from cooking.**

```
?- stand_in(prune_rose_s2, secateurs, X).
given: step(prune_rose, prune_rose_s2). action(prune_rose_s2, cut). object_of(prune_rose_s2, stem).
       uses(prune_rose_s2, secateurs). role(prune_rose_s2, secateurs, cut).
       lacks(user, secateurs). has(user, kitchen_scissors).
gold: X = kitchen_scissors
[1] G role(prune_rose_s2, secateurs, cut).
[2] T is_a(kitchen_scissors, scissors).        (taught in cooking)
[3] T can_do(scissors, cut).                   (taught in crafts or cooking)
[4]   can_do(kitchen_scissors, cut).           R2 [2,3]
[5]   stand_in(prune_rose_s2, secateurs, kitchen_scissors).  R5 [1,4]
```

No gardening fact is ever taught. The `given` lines are premises that are not stored.

**E7. NAME, 2-hop, cooking.**

```
?- stand_in(whip_cream_s2, egg_whisk, X).
given: lacks(user, egg_whisk). has(user, fork).
gold: X = fork
[1] T role(whip_cream_s2, whisk, beat).        (the procedure was taught with "whisk")
[2] T same_as(egg_whisk, whisk).               (taught as the answer to a naming question)
[3]   role(whip_cream_s2, egg_whisk, beat).    R4 [1,2]
[4] T can_do(fork, beat).
[5]   stand_in(whip_cream_s2, egg_whisk, fork). R5 [3,4]
```

**E8. STEP, 1-hop, baking.**

```
?- replace_step(cream_butter_and_sugar_s1, A).
given: time_limit(user, 15, minute). has(user, microwave).
gold: A = microwave_on_low
[1] T achieves(cream_butter_and_sugar_s1, softened_butter).   (taught: leave at room temperature)
[2] T requires(cream_butter_and_sugar_s2, softened_butter).
[3] T achieves(microwave_on_low, softened_butter).
[4] T requires(microwave_on_low, microwave).   G has(user, microwave).
[5]   replace_step(cream_butter_and_sugar_s1, microwave_on_low).  R6 [1,2,3,4]
```

The time limit is context. The 1-hop label counts only R6. That the replacement is faster is a separate `takes_time` check, outside the spike.

**E9. K, 2-hop, repair (parts).**

```
?- part_of(o_ring, mixer_tap).
gold: yes
[1] T part_of(o_ring, cartridge).
[2] T part_of(cartridge, valve).
[3] T part_of(valve, mixer_tap).
[4]   part_of(o_ring, valve).        R3 [1,2]
[5]   part_of(o_ring, mixer_tap).    R3 [4,3]
deny: part_of(o_ring, valve), part_of(o_ring, mixer_tap), part_of(cartridge, mixer_tap)
```

**E10. IND, 3 hops with a learned rule, cleaning with pieces from cooking.**

```
?- stand_in(descale_kettle_s2, descaler, X).
given: lacks(user, descaler). has(user, lemon).
gold: hyp X = lemon (preferred); gap (acceptable)
support for the induced rule L: can_do(X, remove_limescale) :- has_property(X, acidic)
  T has_property(vinegar, acidic).      T can_do(vinegar, remove_limescale).       (cleaning)
  T has_property(citric_acid, acidic).  T can_do(citric_acid, remove_limescale).   (cleaning)
  T has_property(cola, acidic).         T can_do(cola, remove_limescale).          (cooking)
  no taught counterexample; 3 examples from 2 domains
[1] T is_a(lemon, citrus_fruit).    T has_property(citrus_fruit, acidic).   (cooking)
[2]   has_property(lemon, acidic).                 R2
[3]   can_do(lemon, remove_limescale).             L (hypothesis, support 3)
[4] T role(descale_kettle_s2, descaler, remove_limescale).
[5]   stand_in(descale_kettle_s2, descaler, lemon). R5 [4,3]   status: hypothesis
deny: can_do(lemon, remove_limescale), stand_in(descale_kettle_s2, descaler, lemon)
```

Stating [5] as a plain fact, without the hypothesis marker, is **invented**. The spike's companion question teaches a counterexample (`has_property(milk, acidic)`, `not can_do(milk, remove_limescale)`), after which the rule must be specialised or retracted, and the gold becomes `gap`.

---

## 6. Baselines

Every baseline runs on the same questions and is scored by the same scorer and proof checker, so every comparison is paired (McNemar).

| Id | Baseline | What it shows | Calls |
|---|---|---|---|
| B0 | **Retrieval only:** answer `yes` or bind only if the query atom, under the `same_as` closure, is a stored fact; otherwise gap | The floor. Must be about 0 on answerable deductive questions, so it is also the **leakage alarm** (1.7) | 0 |
| B1 | **Base rules only:** learned rules off | What P7's rule learning adds (on minus off) | 0 |
| B1-k | **Base rules minus rule k** (k = R1-R6, R8, R9), on dev and spike only | Each rule's contribution (Q3: keep rules that pay) | 0 |
| B2 | **No exceptions:** monotonic inheritance | The cost of P8's mechanism. EXC questions should flip to wrong | 0 |
| B3 | **Haiku open-book:** Haiku receives the delivered teacher statements relevant to the question (all statements mentioning the question's symbols or their `is_a`/`part_of` ancestors up to 3 levels, at most about 1,500 statements) and is asked for an answer with a proof in the same formal format, or `gap`. 10 questions per call | Whether a language model reasoning over the **same taught facts** beats the agent, and with how many inventions. In 3.0, B2 got T 18/60 and S 10/60 with more false claims | 90 per reading on dev + sealed |
| B4 | **Haiku closed-book:** the same questions, no facts | How much of the world gold is common knowledge. Haiku's proofs cannot cite taught statements, so every claim is "invented" by definition; answer correctness is reported apart | 90, once |
| B5 | **3.0's agent where comparable:** X4's fact-space leave-one-task-out on the 48 train and dev variant cases (3.0: change recall 19.2% affordance, 1.4% rules; 9 cases with wrong changes). 4.0 is run on the same 48 cases, with the situation given as formal facts and the 4.0 knowledge base at L100 | Direct comparison with 3.0's generalisation number, at 0 calls. The variant differences of these 48 cases join the deny list, so they cannot be taught | 0 |
| B6 | **Naive sibling stand-in:** any item that shares an `is_a` parent with the missing one is offered, without function or requirement checks | A reference for overgeneralisation. It should have many wrong answers on SUB, EXC and near-miss GAP questions | 0 |

**Note on B3.** Its retrieval step is a choice that favours Haiku: it is shown only relevant facts. The same retrieved set is also given to a variant of the agent (agent-on-subset), so the comparison is with equal input.

**H3 tier S is not used as a baseline.** It would be a fourth reading of a sealed set, and its inputs are English. B5 is the comparable fact-space number.

**Pre-registered primary contrasts at R3** (Holm-corrected across the three):
1. **Learning curve:** L50 against L100, agent with learned rules, sealed, correct rate.
2. **Rule learning:** learned rules on against off (B1) at L100, sealed, correct rate on all questions, with the IND type reported apart.
3. **Grounding:** agent against B3 at L100, sealed, **invention rate** (a one-sided test that the agent's is lower), with the correct rate reported beside it.

---

## 7. Stopping rules and go/no-go gates

Thresholds that P16 says must come from the spike are written below as **formulas over spike numbers**. The form is fixed now; the numbers follow from the spike without a new decision. Only G1 has numbers now, because the pre-planning gave them.

### 7.1 Gates

**G0: benchmark ready** (before any stage 1-7 call):
- **Spike set:** 100 questions; 100% pass the validator (gold derivable, minimal proofs, gaps not derivable); hand check done.
- **Main pool:**
  - at least 900 questions validated;
  - writer-reviewer agreement of at least 90% before dropping;
  - the split and both manifests committed;
  - `deny.sha256` built;
  - B0 is 0 on answerable deductive questions against the stage 0 knowledge base.
- **Pilot T0:** net yield of at least 10 facts per call, and at least 90% of statements in schema.

**G1: after the spike** (0 calls). The pre-planning's thresholds, in measurable form:
- 2-hop correct ≥ 60% (at least 22 of 36) in the spike set;
- **0 inventions** over all claims in the spike set;
- learned rules add measurably: on the 10 IND questions, at least 3 more answered correctly (hypothesis or gap as the gold says), and no answerable deductive question changes from correct to wrong;
- inference cost: p95 under 1 s per question on the spike knowledge base replicated 10 times with renamed symbols (about 4,000 facts), as a synthetic test of size.

**If G1 fails:** rethink the relation set, the rules or the approach (pre-planning section 6). No teaching budget is spent.

**G1 outputs** (recorded in the spike report):
- s2 = spike 2-hop rate;
- s_all = spike correct rate;
- y = pilot yield.

**G2: after size S** (sealed reading R1, then `PREREG_4.md`):
- **delivered leak hits = 0** (otherwise the run is invalid and is re-taught from the hit point);
- teacher audit error of at most 10% per stage (100 statements each);
- **dev**: 2-hop correct at S100 ≥ s2 − 15 points (real teaching is noisier than the fixture, but not much worse);
- **dev**: the upper 95% bound on the invention rate is at most 1%;
- **dev**: curve slope S50 to S100 > 0 (exact McNemar p < 0.10 on dev alone; R1 adds the sealed counts);
- coverage at S100 ≥ 50% on S-scope questions. Below that, the curriculum is fixed before more scale.

**If G2 fails:** fix on dev and re-teach S within the reserve. A second failure means stop and report.

**G3: scaling** (S to M, then M to L, at R2):
- Go to the next size only if all of these hold:
  - **non-inferiority:** correct on the S-scope subset at M100 ≥ S100 − 2 points (paired, one-sided 95% bound);
  - **gain:** correct on the full set at M100 > S100 (paired, p < 0.05);
  - inference p95 ≤ 2 s;
  - invention upper bound ≤ 1%.
- The same test decides L to XL (the conditional 1,490 calls, P10).

**G4: final** (R3): every pre-registered target is reported met or not, with intervals. Nothing is lowered.

### 7.2 Hard stops (at any time)

| Condition | Action |
|---|---|
| Ledger reaches 5,000 | `haiku.py` raises. Only the user can raise the limit (P14) |
| The first plan-limit refusal | Fail fast, resume later (3.0 finding 2) |
| A phase exceeds its envelope by 20% | Stop the phase and re-plan from the reserve |
| One delivered deny hit at the P5 audit | The reading is invalid. Find the path, fix it, re-teach from the hit point |
| Facts-layer hash changes during a reading | The reading is invalid (readings never teach) |
| Sealed reads used up (3) | No more sealed numbers without an explicit user decision, logged |
| Replay unanswered questions above 5% | The results are marked "under-answered"; top up live or report them as such |

### 7.3 Soft stops (review before continuing)

- **The new-symbol rate per call rises** for 2 consecutive stages: naming is drifting. Revise the prompts and the registry slice.
- **The teacher audit error goes above 10%** in a stage: stop that stage and use the reserve to re-teach it.
- **Question yield falls below half the topic yield** after stage 3: cut the question share to 10%.
- **Wrong (defeated) answers make up more than 10% of EXC questions on dev:** fix the exception mechanism before the next size.
- **More than 3 hypotheses retracted per 100 teacher messages:** raise the support threshold.

---

## 8. Open questions for the design session

1. Is P14's 5,000 a fresh 4.0 ledger? 3.0 used about 4,533 (4,486 by PROGRESS's count). The plan assumes a fresh ledger (3.1).
2. Do archived 3.0 Haiku formal messages count as conversational teaching (2.3)? The default is yes, with an ablation at 0 calls.
3. Is the spike's relational fixture allowed as a test fixture outside P2 (5.1)?
4. Should R8 (material property transfer) and R9 (role implies ability) be candidate base rules (5.2)?
5. How do R5's requirement checks follow an item across steps (E3)?
6. Three sealed readings (1.8)?

## 9. Files this plan creates later (none created now)

| File | Purpose |
|---|---|
| `tools/design4/BENCH_SPEC.md` | The spec the benchmark writers work from |
| `tools/design4/check_bench.py` | Benchmark validator |
| `tools/design4/ref_reason.py` | Independent reference reasoner and proof checker |
| `tools/design4/leak_check.py` | Deny list build and check |
| `tools/design4/haiku.py` | Copy of the 3.0 client with its own `calls4` ledger |
| `tools/design4/import_archive3.py` | Imports the 3.0 cache into the raw layer |
| `tools/design4/prompts/` | One versioned system prompt per stage |
| `data/bench4/{spike,dev,sealed}/` | The benchmark parts |
| `data/kb4/` | The knowledge base |
| `research/PREREG_4.md` | Targets frozen after R1 |
| `research/SPIKE_4.md` | The spike report |
