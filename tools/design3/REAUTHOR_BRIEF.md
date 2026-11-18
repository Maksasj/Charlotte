# Brief: re-author ground truth to schema v3 (Design 3.0, decision D7)

You convert 2.0 scenario files to the v3 schema. Read first:
1. `research/DESIGN_3_schema.md` (the schema, all of it).
2. `data/curriculum3/dev/bake_potato.md` (the worked template; copy its layout exactly).
3. The source file you convert, from `data/curriculum/scenarios/`, `data/curriculum/training/` or `data/curriculum/seed/`.

Write each converted file to the target folder you were given, under the same file name. Never modify anything under `data/curriculum/` (the old files stay as they are). Gardening tasks (`water_plant`, `mow_lawn`, `rake_leaves`, `repot_plant`, `water_garden`) go to `data/curriculum3/heldout_domain/` whatever folder you were given, with `domain(T, gardening)`.

## Rules

1. **Keep the old content.** Every old fact survives unless it is wrong or a compound step is split. Keep the English `requests`, `situation`, `checklist` and `variant` lines word for word. Add `schema: v3` to the front matter.
2. **One action per step.** If a teach line does two actions ("put soil in the pot, set the plant in and fill around it"), make one step per action, numbered in order, and add `before` facts between them. Renumber steps `_s1.._sN` in taught order. Rewrite every `before`/`overlaps` to the new ids. Rewrite the teach line itself only by splitting it into separate "Step N of ...:" lines, so the teaching states each step once. Keep the "While step k runs," prefix where it applies.
3. **Add what the teach lines say but the old facts lost:**
   - temperatures and settings (`setting`);
   - quantities (`amount`);
   - destinations (`into`, `onto`, `from`);
   - `role` for every `uses`;
   - `until`, `check` and `purpose` when the line gives a reason ("so", "to", "for a crispy skin");
   - `warning` and `avoid` when the line says to be careful or not to do something;
   - `condition` for "if ..." clauses;
   - `optional` for "if you like" steps.
4. **Do not invent.** Every ground-truth fact must be stated or plainly implied by a teach line. You MAY add a short reason clause to a teach line (", so steam can escape") when the reason is common knowledge and it makes a `purpose` fact learnable, but at most one per three steps, and never change the actions, items, times or order. Seed files have no teach lines; for them, add only facts that the existing facts and the file's description imply.
5. **Symbols.** Lower-case, underscores, singular nouns (`taco_shell`, not `taco_shells`). Keep the old symbols for items and actions where they exist. Roles, states, purposes and hazards are short symbols (`pierce`, `hold`, `set`, `steam_escapes`, `oil_spits`); reuse the same role symbols across files: `cut`, `pierce`, `beat`, `stir`, `mix`, `hold`, `contain`, `cover`, `coat`, `season`, `heat`, `cool`, `clean`, `wipe`, `dry`, `measure`, `spread`, `fasten`, `loosen`, `lift`, `pour`, `scrub`, `protect`, `flavour`, `sweeten`, `bind`, `blend`, `press`, `light`, `inform`, `pay`, `record`. Add a new one only when none fits.
6. **Every task gets** `domain` (one of the nine in the schema) and `goal` (what it produces or achieves; use a `solves` fact's second argument if the old file has one, and drop `solves`).
7. **Aliases.** An `## aliases` block with:
   - `task_alias` for each way the requests, teach lines or title name the task (gerund forms, "jacket potato");
   - `alias` for every surface form in the requests, situation, teach and variant lines that differs from the symbol (plural, adjectives, "frying pan" for `pan`, "baking sheet").
   
   Aliases come from the file's own text plus common synonyms; never more than about 25 lines.
8. **Variants.** Each `- change:` variant gets a `datalog situation` block and a `datalog difference` block (schema 3.6), as in the template:
   - The situation lists every item the variant's situation line says the user has (`has`), lacks (`lacks`), wants (`wants`), excludes (`excludes`), and any time limit or count.
   - The difference is the smallest set of `swap`/`drop`/`add_step`/`change`/`extra_need`/`no_need`/`cannot` facts that turns the base procedure into what `expect:` describes. New steps are numbered `_s9` upward and carry their own `step`/`action`/`object_of`/`uses`/`role`/`takes_time`/`before` facts.
9. **Validate.** Run `python3 -I tools/design3/check_gt.py <your target folder>` and fix every error before you finish. Also run it on `data/curriculum3/heldout_domain` if you wrote there.

When done, reply with:
- the list of files written;
- how many steps were split;
- how many facts were added by type;
- any teach-line reason clauses you added;
- anything you were unsure about.

Do not commit. Make no calls to Haiku or the Claude CLI.
