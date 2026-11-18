# Spec: the Design 3.0 held-out set H3 (decision D9)

H3 measures whether the agent learned, not whether we tuned it. It is written once from this spec, then sealed: a SHA-256 manifest is committed and a test checks it. Nobody who works on the agent reads H3's contents before the measured readings. Only the validator and counts touch it.

## Tiers (authored)

| Tier | Folder | Count | What is new |
|---|---|---|---|
| S, new situation | `data/heldout3/S/` | 60 | A known task (one in `data/curriculum3/train`, `dev` or `seed`) with a situation never used in its variants. One file per case; the base task's facts are copied from the known file |
| T, new task, known domain | `data/heldout3/T/` | 60 | A task that exists nowhere in `data/curriculum3/` or `data/curriculum/`, in the domains cooking, cleaning, repair, planning, errands, travel, crafts (about 9 each) |
| G, new domain | `data/heldout3/G/` | 30 | New gardening tasks. Together with the 5 gardening tasks in `data/curriculum3/heldout_domain/` this is the domain tier |

The algorithmic tier is computed from the training data with repeated random task splits (experiment X8); nobody authors it.

## File format

The v3 scenario format, exactly as in `data/curriculum3/dev/bake_potato.md`, validated by `tools/design3/check_gt.py`, with these rules:

- **T and G files** have:
  - front matter with `tier: T` or `tier: G`;
  - 3 requests;
  - 1-2 situation lines;
  - the ground truth;
  - aliases;
  - teach lines in the curriculum form ("To X you need …", "X takes …", "Step N of X: …", "While step k runs, step N of X: …");
  - a checklist;
  - **exactly 2 variants**, each with situation and difference blocks.

  Steps have one action each. Use 3-7 steps.
- **S files** have:
  - front matter with `tier: S` and `base: <task>`;
  - the base task's ground truth, aliases and teach lines, copied unchanged;
  - 1 request;
  - **one variant** with a situation and difference block (no other variants).

  Situations must differ in kind from the base file's own variants.
- **Wording.** Requests are written the way a person would ask, varied:
  - name the task in a different way from the teach lines in at least half of them;
  - sometimes imply the task without naming it ("my tap won't stop dripping").
- **Situations across tiers.** Spread them: missing tool or ingredient (with a stand-in the user has, or with no stand-in, so a step drops or the answer is `cannot`), an extra wanted, an exclusion (diet, allergy), a time limit, a count (guests), a broken item.
- **Role symbols.** Reuse the role symbols already used in `data/curriculum3/` (list them with grep), so substitution by function is learnable.

  About a third of the swap cases should need a stand-in whose role is shown in some other known task, a third a stand-in whose role is not shown anywhere, and a third no stand-in.
- **Held out from training.** No T or G task may share its task name with any existing task. G tasks must be gardening: lawn, plants, seeds, soil, pruning, compost, weeding, hedges, bulbs, watering systems, pests. Their tools and items may be new, which is the point of the domain tier.

## Sealing

After writing, the writer runs the validator on its folder, then writes nothing else. The main session computes `data/heldout3/MANIFEST.sha256` and commits it.
