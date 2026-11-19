import json, os, sys, re, subprocess
sys.path.insert(0, "/home/maksasj/Programming/Charlotte/tools/design4")
import check_fixture as cf
ROOT = "/home/maksasj/Programming/Charlotte/data/spike4"
fx = cf.Fixture(ROOT); fx.load_teach(); fx.load_answers(); fx.build_canon()
Q = [json.loads(l) for l in open(ROOT + "/bench/questions.jsonl")]
def L(s):
    """file:line of a statement, matched raw first, then canonical."""
    at = cf.parse_atom(s)
    for st in fx.statements:
        if st["atom"] == at: return st["src"]
    for st in fx.statements:
        if st["status"] == "accepted" and fx.canon_atom(st["atom"]) == fx.canon_atom(at): return st["src"]
    raise SystemExit("not found: " + s)
def F(*atoms): return "; ".join("`%s` %s" % (a, L(a)) for a in atoms)
def ANS(q):
    for i, l in enumerate(open(ROOT + "/script/answers.jsonl"), 1):
        if json.loads(l)["q"] == q: return "script/answers.jsonl:%d" % i
    raise SystemExit(q)
def QID(atom):
    ids = [q["id"] for q in Q if atom in q["gold"]["atoms"]]
    return ", ".join(ids) if ids else "-"
out = []
w = out.append
w("# Design 4.0 spike fixture: planted-case index")
w("")
w("Test-only fixture (decision D4.3, Appendix A of `research/Symbolic Learning Agent – Design 4.0.md`). Written by a sub-agent from the design documents only, without sight of `crates/`; **to be reviewed by the user before any run.** `MANIFEST.sha256` is not written yet: the main session seals the fixture after review.")
w("")
w("Line references are `file:line` relative to `data/spike4/`. Regenerate the counts and re-check everything with `python3 -I tools/design4/check_fixture.py`.")
w("")
w("## Files")
w("")
w("| File | Content |\n|---|---|")
w("| `teach/baking.msgs`, `cooking.msgs`, `cleaning.msgs`, `repair.msgs` | Item relations, in curriculum order (kinds, materials and properties, functions, places, parts, actions and states, names). Each starts with a `topic <domain>/vocabulary.` batch of `new(sym, sort).` declarations for the symbols first used in that file |")
w("| `teach/procedures_fixture.msgs` | 8 procedures (2 per domain), **marked fixture**: v3 ground-truth core facts copied from `data/curriculum3`, plus `demands` and a few `fixture addition` lines; then 4 taught swap cases |")
w("| `script/answers.jsonl` | Oracle teacher answers keyed by canonical question (naming, conflicts, diamonds, confirming questions). Any other question gets `unknown.` |")
w("| `bench/questions.jsonl` | The 100 spike questions |")
w("| `bench/deny.txt` | Every gold atom, intermediate proof node, withheld piece and `swap` form (291 atoms); none is taught |")
w("")
w("Delivery order: baking, cooking, cleaning, repair, procedures_fixture; oracle answers are delivered when asked.")
w("")
w("## Conventions the fixture assumes (please check)")
w("")
w("1. **Role symbols come from the v3 ground truth.** The loaf tin's role in `bake_banana_bread_s4` is `contain` (as in `data/curriculum3`), not `hold` as in the design's E4. Item relations therefore use `contain` (`can_do_on(baking_tin, contain, batter)`), and the demand is `demands(bake_banana_bread_s4, contain, can_go_in, oven)`.")
w("2. **Procedures come from the v3 ground truth, not from archive3 readings.** No cached Haiku reading was opened (no calls, and the sub-agent did not inspect `data/training/`). Only the predicates task, step, action, object_of, uses, role, before, requires_state, achieves and needs are copied; settings, times, purposes, checks, `into` and `overlaps` are left out. `bake_banana_bread` and `make_apple_crumble` have `domain(..., cooking)` in v3 and are used as baking here (as eval §5.1 suggests).")
w("3. **Fixture additions** to the ground truth (each marked in the file): `achieves`/`requires_state` pairs for the four STEP questions, one `before(make_omelette_s2, make_omelette_s4)` (B8 reads only direct `before`), and 14 `demands`.")
w("4. **Taught swap cases are topic-scoped.** The grammar has no case wrapper, so each case is a `topic <domain>/case_<name>.` batch holding `lacks(user, I). has(user, J). swap(S, I, J).`. The `lacks`/`has` lines are the case's situation only and must not enter the global situation; the checker treats them so. If agent4 cannot scope them, these 4 batches need another form.")
w("5. **Hops** follow section 7.4: an inheritance down n `is_a` links counts n; B5, the B6 role bridge, B7, B8 count 1; the `used_for`/`can_do_on` → `can_do` bridges, reification and leaves count 0; a B7 proof sums its `can_do` proof and every demand proof, plus 1. A transitive `is_a(X, K)` over n links counts n − 1; a `has_part` chain of n links counts n − 1 plus any inheritance. **`same_as` (B4) counts 0** because it is canonicalisation at entry; NAME questions therefore have at least one real hop besides the renaming, otherwise the gold would equal a taught fact under the deny list's `same_as` closure (D4.8 level L1).")
w("6. **Query form.** `query: <goal>.` with an optional `given: <facts>.`; variables are upper case. A help request is `request(<task>). <situation facts>.`. Golds name the user's symbol (e.g. `egg_whisk`); compare modulo `same_as`.")
w("7. **Extra gold fields** beyond `kind/atoms/proof_lines`: `excluded` (SUB look-alikes that must not be claimed), `missing` and `completes` (GAP: the withheld pieces, and what they would make derivable), `rule` and `min_support` (IND), and top-level `ind_case` (`support`, `counterexample`, `below_threshold`) and `note`. For a request, a `gap(S, I)` gold atom is the B9 conclusion the answer must state.")
w("8. **Induction support counts inherited subjects** (section 7.1: taught or inherited). The planted clusters have 3-4 *taught* examples; with subkinds the support sets are larger (listed below).")
w("")
w("## Sort errors (exactly 2, must be rejected)")
w("")
for st in fx.rejected:
    w("- `%s` %s: %s" % (st["raw"], st["src"], st["reason"]))
w("")
w("## Specificity exceptions (9)")
w("")
w("| # | General | Exception | Question |\n|---|---|---|---|")
rows = [
 ("can_go_in(dish, oven)", "not can_go_in(plastic_dish, oven)", "not can_go_in(takeaway_box, oven)", "with the **redundant link** " + F("is_a(takeaway_box, dish)")),
 ("not can_go_in(plastic_dish, oven)", "can_go_in(ovenproof_plastic_tray, oven)", "can_go_in(lasagne_tray, oven)", "**exception to an exception**"),
 ("can_go_in(pan, dishwasher)", "not can_go_in(cast_iron_pan, dishwasher)", "not can_go_in(camp_skillet, dishwasher)", ""),
 ("can_go_in(cutlery, dishwasher)", "not can_go_in(kitchen_knife, dishwasher)", "not can_go_in(santoku_knife, dishwasher)", ""),
 ("can_go_in(cleaning_cloth, washing_machine)", "not can_go_in(chamois, washing_machine)", "not can_go_in(car_chamois, washing_machine)", ""),
 ("has_property(scourer, abrasive)", "not has_property(non_scratch_scourer, abrasive)", "not has_property(glass_hob_scourer, abrasive)", ""),
 ("made_of(hammer, metal)", "not made_of(rubber_mallet, metal)", "not made_of(tile_mallet, metal)", ""),
 ("has_property(flour, contains_gluten)", "not has_property(rice_flour, contains_gluten)", "not has_property(brown_rice_flour, contains_gluten)", ""),
 ("has_property(screw_fixing, magnetic)", "not has_property(brass_screw, magnetic)", "not has_property(antique_hinge_screw, magnetic)", ""),
]
for i, (g, e, qa, extra) in enumerate(rows, 1):
    w("| %d | %s | %s | %s %s%s |" % (i, F(g), F(e), QID(qa), "`%s`" % qa, ("; " + extra) if extra else ""))
w("")
w("Also `not can_go_in(kids_bowl, oven)` (%s) is 3 links below `plastic_dish`. Each exception's taught line raises an E3-style conflict question while teaching; the oracle answers `exception` (script/answers.jsonl, `conflict:` keys)." % QID("not can_go_in(kids_bowl, oven)"))
w("")
w("## Diamonds (3)")
w("")
w("- `enamel_dish`: %s vs %s, through %s. Oracle: %s → `no`. No question." % (F("can_go_in(ceramic_dish, microwave)"), F("not can_go_in(metal_container, microwave)"), F("is_a(enamel_dish, ceramic_dish)", "is_a(enamel_dish, metal_container)"), ANS("ask: can_go_in(enamel_dish, microwave)?")))
w("- `insulated_pliers`: %s (2 links) vs %s (1 link): incomparable kinds, so a conflict although the distances differ. Oracle %s → `no`; then %s `has_property(electricians_pliers, conductive)` is answered `no` in 1 hop." % (F("has_property(metal_tool, conductive)"), F("not has_property(insulated_tool, conductive)"), ANS("ask: has_property(insulated_pliers, conductive)?"), QID("not has_property(electricians_pliers, conductive)")))
w("- `wool_duster`: %s vs %s. Oracle %s → `unknown` (left open on purpose); %s expects a gap." % (F("can_go_in(duster, washing_machine)"), F("not can_go_in(wool_item, washing_machine)"), ANS("ask: can_go_in(wool_duster, washing_machine)?"), QID("can_go_in(wool_duster, washing_machine)")))
w("")
# induction
w("## Induction clusters (4, support from at least 2 domains; gold `hyp`)")
w("")
rs = cf.Reasoner(fx.taught_facts())
def clus(rule, taught, pred, conf):
    st = cf.rule_stats(rs, rule, fx)
    w("- `%s`. Taught examples: %s. Support with inheritance: %d (%s), %d kinds, counter none. Question %s `%s`. Confirming question %s." %
      (rule, F(*taught), len(st["support"]), ", ".join(st["support"]), st["kinds"], QID(pred), pred, conf))
clus("not can_go_in(X, oven) :- made_of(X, plastic)", ["made_of(plastic_jug, plastic)", "not can_go_in(plastic_jug, oven)", "made_of(plastic_tub, plastic)", "not can_go_in(plastic_tub, oven)", "made_of(plastic_bucket, plastic)", "not can_go_in(plastic_bucket, oven)"], "not can_go_in(food_bag, oven)", ANS("ask: can_go_in(spray_bottle, oven)?") + " (`no`; spray_bottle " + L("made_of(spray_bottle, plastic)") + ")")
clus("can_do(X, dissolve_limescale) :- has_property(X, acidic)", ["has_property(acid, acidic)", "can_do(acid, dissolve_limescale)", "is_a(vinegar, acid)", "is_a(citric_acid, acid)", "has_property(cola, acidic)", "can_do(cola, dissolve_limescale)"], "can_do(lemon, dissolve_limescale)", ANS("ask: can_do(lime, dissolve_limescale)?") + " (`yes`)")
clus("can_do(X, cut) :- has_property(X, sharp)", ["has_property(knife, sharp)", "can_do(knife, cut)", "has_property(scissors, sharp)", "can_do(scissors, cut)", "has_property(saw, sharp)", "can_do(saw, cut)"], "can_do(pizza_wheel, cut)", ANS("ask: can_do(wire_cutter, cut)?") + " (`yes`)")
clus("can_do(X, soak_up) :- has_property(X, absorbent)", ["has_property(sponge, absorbent)", "can_do(sponge, soak_up)", "has_property(paper_towel, absorbent)", "can_do(paper_towel, soak_up)", "has_property(tea_towel, absorbent)", "can_do(tea_towel, soak_up)"], "can_do(sawdust, soak_up)", ANS("ask: can_do(cotton_rag, soak_up)?") + " (`yes`)")
w("")
w("The material-based cluster is plastic → not oven; the property-based one is acidic → can dissolve limescale (`dissolve_limescale` is the function symbol). The deny list holds each question's prediction, so its confirming question is about another prediction.")
w("")
w("## Traps with support 2 (must not induce at k = 3; gold `gap`)")
w("")
for rule, taught, preds in [("can_go_in(X, oven) :- made_of(X, glass)", ["made_of(pyrex_jug, glass)", "can_go_in(pyrex_jug, oven)", "made_of(glass_dish, glass)"], ["can_go_in(glass_jar, oven)"]),
                            ("not can_go_in(X, dishwasher) :- made_of(X, wood)", ["made_of(wooden_spoon, wood)", "not can_go_in(wooden_spoon, dishwasher)", "made_of(chopping_board, wood)", "not can_go_in(chopping_board, dishwasher)"], ["can_go_in(rolling_pin, dishwasher)", "can_go_in(bread_board, dishwasher)"])]:
    st = cf.rule_stats(rs, rule, fx)
    w("- `%s`: %s; support %d (%s). Questions: %s." % (rule, F(*taught), len(st["support"]), ", ".join(st["support"]), ", ".join("%s `%s`" % (QID(p), p) for p in preds)))
w("  (`glass_dish` supports through %s.)" % F("can_go_in(dish, oven)"))
w("")
w("## Clusters with counterexamples (must not induce, or must specialise; gold `gap`)")
w("")
st = cf.rule_stats(rs, "can_go_in(X, dishwasher) :- made_of(X, metal)", fx)
w("- **Taught counterexample:** `can_go_in(X, dishwasher) :- made_of(X, metal)`. Support %d (%s) from %s, %s. Counterexamples taught: %s; with subkinds: %s. Questions %s, %s." % (
    len(st["support"]), ", ".join(st["support"]), F("can_go_in(cutlery, dishwasher)", "made_of(fork, metal)"), F("can_go_in(metal_colander, dishwasher)"),
    F("not can_go_in(kitchen_knife, dishwasher)", "made_of(cast_iron_pan, metal)", "not can_go_in(cast_iron_pan, dishwasher)"), ", ".join(st["counter"]),
    QID("can_go_in(whisk, dishwasher)") + " `can_go_in(whisk, dishwasher)`", QID("can_go_in(grater, dishwasher)") + " `can_go_in(grater, dishwasher)`"))
st = cf.rule_stats(rs, "not can_go_in(X, oven) :- made_of(X, rubber)", fx)
w("- **Inherited counterexample (the silicone cases):** `not can_go_in(X, oven) :- made_of(X, rubber)`. Support %d (%s) from %s. Counterexamples by inheritance: %s. Question %s `can_go_in(rubber_plug, oven)`." % (
    len(st["support"]), ", ".join(st["support"]), F("not can_go_in(rubber_glove, oven)", "not can_go_in(rubber_mallet, oven)", "not can_go_in(rubber_seal, oven)"), ", ".join(st["counter"]), QID("can_go_in(rubber_plug, oven)")))
w("")
w("A specialisation (for example `not isa(X, baking_tin)`, or an instance exception) could turn either rule into a hypothesis that predicts the question atom; the golds stay `gap` as Appendix A requires. Please confirm this reading.")
w("")
w("## Silicone-tray cases (2): kind inheritance against a material pattern")
w("")
w("- %s; %s, %s. %s expects `yes` by inheritance (taught and inherited beat learned)." % (F("is_a(silicone_tray, baking_tin)"), F("made_of(silicone_tray, rubber)"), F("can_go_in(baking_tin, oven)"), QID("can_go_in(silicone_tray, oven)")))
w("- %s; %s. %s expects `yes` (2 links)." % (F("is_a(silicone_mould, cake_tin)"), F("made_of(silicone_mould, rubber)"), QID("can_go_in(silicone_mould, oven)")))
w("")
w("## Decoy kinds (2)")
w("")
w("- `plastic_tub` (%s, ancestor `container` shared with `baking_tin` %s) fails %s by %s. %s excludes it." % (L("is_a(plastic_tub, food_container)"), L("is_a(baking_tin, container)"), F("demands(bake_banana_bread_s4, contain, can_go_in, oven)"), F("not can_go_in(plastic_tub, oven)"), QID("stand_in(bake_banana_bread_s4, loaf_tin, muffin_tray)")))
w("- `flathead_screwdriver` (%s, sibling of `phillips_screwdriver`) fails %s by %s. %s excludes it." % (L("is_a(flathead_screwdriver, screwdriver)"), F("demands(replace_door_handle_s1, loosen, has_property, cross_tip)"), F("not has_property(flathead_screwdriver, cross_tip)"), QID("stand_in(replace_door_handle_s1, screwdriver, phillips_screwdriver)")))
w("")
w("Excluded look-alike, not a decoy kind: `dish_soap` in %s (can clean, %s)." % (QID("stand_in(clean_microwave_s1, lemon, white_vinegar)"), F("not has_property(dish_soap, acidic)")))
w("")
w("## Swap cases for part B (4 cases, 4 tasks, 3 domains)")
w("")
for s in ["swap(make_omelette_s2, fork, chopsticks)", "swap(boil_egg_s2, spoon, ladle)", "swap(clean_window_s2, sponge, dish_cloth)", "swap(fix_dripping_tap_s2, screwdriver, butter_knife)"]:
    w("- %s" % F(s))
w("")
w("The lgg of these four is the B7 body without demands (logic §4.3, example B). None of the cases is on a question's (step, item, stand-in) triple.")
w("")
w("## Cross-domain links (12): kind taught in one domain, used in another")
w("")
w("| # | Taught (domain) | Used in | Question |\n|---|---|---|---|")
links = [
 ("is_a(tea_towel, cloth)", "cleaning (clean_window_s4)", "stand_in(clean_window_s4, cloth, tea_towel)"),
 ("is_a(white_vinegar, vinegar)", "cleaning (clean_microwave_s1, with " + L("used_for(white_vinegar, clean)") + ")", "stand_in(clean_microwave_s1, lemon, white_vinegar)"),
 ("is_a(vinegar, acid)", "cleaning (" + L("can_do(acid, dissolve_limescale)") + ")", "can_do(white_vinegar, dissolve_limescale)"),
 ("is_a(lemon, citrus_fruit)", "cleaning (acidic rule)", "can_do(lemon, dissolve_limescale)"),
 ("is_a(lime, citrus_fruit)", "cleaning (clean_microwave_s1 demand acidic)", "gap(clean_microwave_s1, lemon)"),
 ("is_a(kitchen_scissors, scissors)", "gardening", "stand_in(prune_rose_s2, secateurs, kitchen_scissors)"),
 ("is_a(kitchen_knife, knife)", "gardening", "stand_in(repot_plant_s3, secateurs, kitchen_knife)"),
 ("is_a(butter_knife, knife)", "repair (" + L("can_do(butter_knife, loosen)") + ")", "gap(replace_door_handle_s1, screwdriver)"),
 ("is_a(casserole_dish, dish)", "baking (make_apple_crumble_s5)", "stand_in(make_apple_crumble_s5, baking_dish, casserole_dish)"),
 ("is_a(bowl, container)", "gardening", "stand_in(water_garden_s1, watering_can, bowl)"),
 ("is_a(bucket, container)", "gardening", "stand_in(rake_leaves_s3, garden_sack, bucket)"),
 ("is_a(rubber_glove, glove)", "gardening", "stand_in(mow_lawn_s1, gardening_gloves, rubber_glove)"),
 ("is_a(claw_hammer, hammer)", "gardening", "stand_in(stake_tomato_s1, mallet, claw_hammer)"),
 ("is_a(hacksaw, saw)", "gardening", "stand_in(prune_rose_s3, pruning_saw, hacksaw)"),
 ("is_a(pyrex_jug, measuring_jug)", "gardening", "stand_in(water_plant_s2, watering_can, pyrex_jug)"),
]
for i, (a, use, qa) in enumerate(links, 1):
    dom = L(a).split("/")[1].split(".")[0]
    w("| %d | `%s` %s (%s) | %s | %s |" % (i, a, L(a), dom, use, QID(qa)))
w("")
w("Gardening is never taught; its procedure fragments appear only in `given:`.")
w("")
w("## Naming (3 drift pairs, plus taught `same_as`)")
w("")
for drift, qq in [("can_do(egg_whisk, beat)", "same_as(egg_whisk, whisk)? or is_a(egg_whisk, whisk)?"),
                  ("used_for(cutting_board, chop)", "same_as(cutting_board, chopping_board)? or is_a(cutting_board, chopping_board)?"),
                  ("can_do(adjustable_spanner, grip)", "same_as(adjustable_spanner, adjustable_wrench)? or is_a(adjustable_spanner, adjustable_wrench)?")]:
    ans = json.loads(open(ROOT + "/script/answers.jsonl").readlines()[int(ANS(qq).split(":")[1]) - 1])["a"]
    sym = cf.parse_atom(drift)[2][0]
    w("- %s → `ask: %s` → `%s` (%s). Questions: %s." % (F(drift), qq, ans, ANS(qq), ", ".join(q["id"] for q in Q if sym in q["query"])))
w("- `whisk_attachment` (%s) is the E2 negative: the oracle answers `not same_as(whisk_attachment, whisk)` (%s)." % (L("can_do(whisk_attachment, beat)"), ANS("same_as(whisk_attachment, whisk)? or is_a(whisk_attachment, whisk)?")))
w("- Taught `same_as`: %s." % F("same_as(bicarbonate_of_soda, baking_soda)", "same_as(hob, stove_top)", "same_as(washing_up_liquid, dish_soap)", "same_as(kitchen_roll, paper_towel)", "same_as(allen_key, hex_key)"))
w("- `script/answers.jsonl` also answers likely false alarms of the head-token detector with `not same_as` (bread_board, balloon_whisk, white_vinegar, plastic_bucket, kitchen_mixer_tap, adjustable_wrench/spanner).")
w("")
w("## Procedures (8, marked fixture)")
w("")
w("| Domain | Task | Source | Demands |\n|---|---|---|---|")
for line in open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "body/procedures.spec")):
    if line.startswith("#") or not line.strip(): continue
    t, split, d = line.split()
    dm = [st for st in fx.statements if st["atom"][1] == "demands" and st["atom"][2][0].startswith(t + "_")]
    w("| %s | `%s` (%s) | `data/curriculum3/%s/%s.md` | %s |" % (d, t, L("task(%s)" % t), split, t, "; ".join("`%s` %s" % (s["raw"], s["src"]) for s in dm)))
w("")
w("## Questions")
w("")
w("| Type | n | 1-hop | 2-hop | 3-hop | ids |\n|---|---|---|---|---|---|")
for t in ["K", "SUB", "X", "EXC", "STEP", "NAME", "IND", "GAP"]:
    qs = [q for q in Q if q["type"] == t]
    hc = [sum(1 for q in qs if q["hops"] == h) for h in (1, 2, 3)]
    w("| %s | %d | %s | %s-%s |" % (t, len(qs), " | ".join(str(c) for c in hc), qs[0]["id"], qs[-1]["id"]))
w("")
w("IND: 4 `hyp` (q075-q078), 3 with a counterexample (q079-q081), 3 below the threshold (q082-q084). GAP: 16 (q085-q100), near-miss q085-q090.")
w("")
w("## Counts per relation per domain")
w("")
w("Accepted statements in `teach/` (procedure facts counted in their task's domain; `new` and `topic` lines not counted). From `check_fixture.py`:")
w("")
res = subprocess.run([sys.executable, "-I", "/home/maksasj/Programming/Charlotte/tools/design4/check_fixture.py"], capture_output=True, text=True).stdout
blk = res.split("counts per relation per domain (teach/, accepted):\n")[1].split("questions:")[0]
head = res.split("counts per relation")[0]
w("```")
w(head.strip())
w(blk.rstrip())
w("```")
w("")
w("Against Appendix A's per-domain targets: `is_a` is above target (34-60 per domain against about 20), because the 3-hop chains, exception chains and gardening stand-ins each need their own kinds; cooking's `can_go_in` (17 with negatives) is above about 6 because most exceptions are about the oven and the dishwasher; procedure facts are about 75 per domain against about 30, because whole ground-truth procedures are copied. The relation files alone are 83-120 statements each.")
open(ROOT + "/index.md", "w").write("\n".join(out) + "\n")
print("written", len(out))
