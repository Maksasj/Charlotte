# Charlotte — case-based line (`cbr`)

Second line of research for a conversational agent that learns from examples.
Each reply is treated as a problem to solve from memory: the message and the
conversation so far are the problem, the reply is the solution, and the
outcome of the conversation decides what memory is trusted next time.

This branch starts empty on purpose. The first line of research (a symbolic
agent on a Datalog vault, designs 1 to 4) lives on `main` and `design-2` to
`design-4` and is not carried over here.

## Where things stand

One working prototype, **CaseForge v5**: a single HTML page with no model
calls, where memory is split into

- **patterns** — a few generic solution shapes (ask for what is missing, grant
  if the conditions hold, otherwise deny), with no domain content;
- **atoms** — many formal facts mined from articles (`refund` if `days <= 30`
  and `packaging == original`), with no dialogue logic;
- **binding** — the step that joins one pattern with one atom for one message.

What it shows, reproduced by the probe: one unedited pattern answers requests
in six domains, and a seventh domain (cancellation) starts working after two
atoms are mined from a four-sentence article. Its limits are just as concrete:
patterns are selected from three authored skeletons rather than learned, the
vocabulary of intents, predicates and slots is hand-written regex, and three
of its five known defects give wrong answers on ordinary inputs. Details are in
[`docs/findings.md`](docs/findings.md).

## Layout

| Path | What |
|---|---|
| [`docs/journey.md`](docs/journey.md) | How the idea developed, step by step, and what each step added |
| [`docs/architecture.md`](docs/architecture.md) | How v5 works: memory kinds, the turn pipeline, scoring, what learns |
| [`docs/findings.md`](docs/findings.md) | What held up, what did not, defects, open problems, candidate next steps |
| [`prototypes/caseforge-v5/index.html`](prototypes/caseforge-v5/index.html) | The prototype, as produced. Open it in a browser |
| [`prototypes/caseforge-v5/probe.mjs`](prototypes/caseforge-v5/probe.mjs) | Runs the prototype's engine without a browser and prints the behaviours the findings rely on |
| [`archive/2026-11-22-design-conversation.txt`](archive/2026-11-22-design-conversation.txt) | The raw conversation the prototype came out of |

## Running

```sh
open prototypes/caseforge-v5/index.html     # the lab; state is kept in localStorage under cf5.*
node prototypes/caseforge-v5/probe.mjs      # headless check, needs Node 18+
```

The page loads its fonts from a CDN and otherwise works offline.

## Relation to the first line

Only one link is recorded here, taken from the `main` README rather than from a
full review of that work: the symbolic agent checked every input against a
closed predicate registry, and the same problem reappears in v5 as its
hand-written intent, predicate and slot tables. Whichever line continues, a
vocabulary that cannot grow from data is the shared bottleneck.
