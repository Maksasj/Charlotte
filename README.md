# Charlotte — case-based line (`cbr`)

Second line of research for a conversational agent that learns from examples.
Each reply is treated as a problem to solve from memory: the message and the
conversation so far are the problem, the reply is the solution, and the
outcome of the conversation decides what memory is trusted next time.

This branch starts empty on purpose. The first line of research (a symbolic
agent on a Datalog vault, designs 1 to 4) lives on `main` and `design-2` to
`design-4` and is not carried over here.

The repository has four parts:

| Folder | What |
|---|---|
| [`software/`](software) | The engine and the web interface |
| [`vault/`](vault) | The knowledge the agent answers from; open it in Obsidian to browse it |
| [`data/`](data) | Conversations to learn from |
| [`research/`](research) | How it got here and what was found |

## Where things stand

A Rust engine with a web interface. Knowledge lives in a Markdown vault, split
into

- **patterns** — generic solution shapes as ordered rule lists, learned from
  conversations, with no domain content;
- **atoms** — formal facts (`refund` if `days <= 30` and `packaging ==
  original`), with no dialogue logic;
- a **lexicon** — the phrases that mean each intent and each fact.

The vault was pretrained on 240 simulated conversations and answers
step-by-step procedures, record lookups and rule checks (refunds, warranty and
the like). It has also been taught about 36,000 plain chats, which it stores
and replays when a new message resembles a remembered one.

The limiters are understanding (the engine only reads wordings its lexicon
holds) and the lack of any generalisation from chats. The current state,
including how the formal system works, is in
[`research/report-2026-11-23.md`](research/report-2026-11-23.md); results and
defects in detail are in [`research/findings.md`](research/findings.md).

## Software

| Path | What |
|---|---|
| `software/crates/charlotte-core` | Data types and the five layer traits |
| `software/crates/charlotte-vault` | Reading and writing vault notes |
| `software/crates/charlotte-engine` | Default layers, the turn pipeline, the learners, the simulated teacher |
| `software/crates/charlotte` | The binary: terminal commands and the HTTP server |
| `software/web/` | Web interface (TypeScript, React, Vite) over the HTTP API |
| [`software/docs/design.md`](software/docs/design.md) | Engine design, and what was built against it |
| [`software/docs/api.md`](software/docs/api.md) | The HTTP API |

## Research

| Path | What |
|---|---|
| [`research/report-2026-11-23.md`](research/report-2026-11-23.md) | Report: what was discovered, built and fixed so far, and what is still wrong |
| [`research/findings.md`](research/findings.md) | What held up, what did not, open problems, candidate next steps |
| [`research/journey.md`](research/journey.md) | How the idea developed |
| [`research/v5-architecture.md`](research/v5-architecture.md) | How the v5 prototype works |
| [`research/experiments/pattern-learning/`](research/experiments/pattern-learning/README.md) | Experiment: can solution shapes be learned from labelled conversations? |
| [`research/prototypes/caseforge-v5/`](research/prototypes/caseforge-v5/index.html) | The v5 prototype page and a headless probe of its engine |
| [`research/archive/`](research/archive/2026-11-22-design-conversation.txt) | The raw conversation the prototype came out of |

## Running

Needs a Rust toolchain; the web interface also needs Node. Build inside
`software/`, then run from the repository root.

```sh
(cd software && cargo build --release)
alias charlotte=software/target/release/charlotte

charlotte chat                       # talk to it in the terminal; :trace, :close resolved, :quit
charlotte ask --trace "I forgot my password" "done"

(cd software/web && npm install && npm run build)
charlotte serve                      # API and web interface on http://127.0.0.1:7878

(cd software && cargo test)          # the build-order checks
```

Learning:

```sh
charlotte learn data/processed/pretrain-80-per-shape.jsonl       # prints a report; --review keeps everything as candidates
charlotte promote check-a-rule-candidate               # trust a candidate by hand
charlotte check                                        # lint the vault

# more simulated conversations, for example 400 of each shape
charlotte simulate --n 400 --per-shape --seed 2 --paraphrases data/paraphrases.yaml --out data/sim-400.jsonl
```

The corpus format is one conversation per line of JSON, with optional labels;
see "Learning" in [`software/docs/design.md`](software/docs/design.md).

The research scripts run from the repository root:

```sh
node research/prototypes/caseforge-v5/probe.mjs
node research/experiments/pattern-learning/run.mjs
```

## Relation to the first line

Only one link is recorded here, taken from the `main` README rather than from a
full review of that work: the symbolic agent checked every input against a
closed predicate registry. Here the vocabulary is data in the vault and can
grow from a corpus, but only as far as that corpus's own words reach.
