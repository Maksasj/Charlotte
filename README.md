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

## Composition and conversation memory

- **Call/return.** A request that needs a record can `call(record_source)`: a subframe is pushed, runs the plain lookup it already knows (ask for the subject, look up, report), and pops silently; the parent resumes with `returned_from` and `subgoal_failed` set and the record's facts landed. Features: `subgoal_active`, `returned`, `returned_from=…`, `subgoal_failed`. `charlotte simulate --calls` produces such conversations; the learner replays them with the same stack. On 23 composed conversations, a pattern learned without any composed demonstrations reproduced 0%; with 3 it reproduced 100% (small sample; flattened lookup reached 3–7%).
- **Conversation memory.** The session keeps every subject mentioned. "That order" resolves to the one in focus, "the other one" to the one before it (`same`/`other` signals in the lexicon). With nothing to point at, the agent asks.

## Tier 4 additions

- **Conflict detection.** A message that gives one slot two different values ("5 days ago, no wait, 40 days ago") no longer silently keeps the first: the slot is left unbound and the agent asks which value is right. Atoms that contradict themselves, or two atoms at the same priority for the same intent and object with different results that can both hold, are listed by `charlotte check`.
- **Teaching a reply.** `charlotte teach "message" "reply"`, or `:teach <reply>` in `chat` after a message the agent could not answer, writes a candidate chat. It is used only once promoted, and then also for rewordings the recall accepts.
- **Learned split connectors.** `charlotte connectors FILE` proposes the words found between two requests in chats (support, purity, and a held-out check) as a lexicon candidate; promoting adds them to the lexicon's `connectors`. On the chat data in `data/processed` it finds nothing new, since those chats have almost no multi-request messages.

## Task intents from labelled dialogues

`charlotte tasks data/processed/sgd.jsonl [--promote] [--eval N]` learns task intents from the SGD labels (candidates only; `--promote` is for scratch measurement):

- **wordings** from the user's `INFORM_INTENT` turns, kept by n-gram precision and a held-out check;
- **required slots and their order** from the agent's `REQUEST` acts, with the agent's own question as the ask text;
- **slot types** (bool, number, closed set, free text) from the values users gave;
- **an atom** with "slot given" conditions, so the existing rule asks what is missing and then hands over.

On a clean vault, 1,463 held-out dialogues: intent read from the first request 77%, hand-over reached 72%, 78% of the slots asked were ones the real agent asked (77% the other way). Known gap: values volunteered in the first request are not extracted (free text has no wording patterns), so those slots are asked again.
