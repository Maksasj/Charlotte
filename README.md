# Symbolic Learning Agent

A conversational agent whose reasoning, memory, learning and replies are all
symbolic. Knowledge lives in a vault of Datalog facts, rules and procedures; a
stratified Datalog reasoner answers with proofs; a learner guesses rules and
procedures from what it is taught and keeps them only when they hold up. A
language model is used only at the edges: to translate user input into
checked dialogue acts, and in training as teacher, reviewer and judge.

The current design is [`research/Symbolic Learning Agent – Design 2.0.md`](research/Symbolic%20Learning%20Agent%20–%20Design%202.0.md)
(milestones 12 to 18). v1 is [`research/DESIGN_1.md`](research/DESIGN_1.md), with its findings in
[`research/REPORT_DESIGN_1.md`](research/REPORT_DESIGN_1.md). Build status, acceptance results and
decisions are kept in [`PROGRESS.md`](PROGRESS.md).

## How it works

```
message ─► segmenter ─► translator (fixed patterns → learned patterns → model) ─► validator
                                                                                     │
reply ◄── realizer (sentence frames) ◄── dialogue manager (moves chosen by rules) ◄──┘
                                              │
                         reasoner (Datalog, proofs, aggregates, time) ◄─► vault
                                              │
                         learner (rules, procedures, corrections, words) ─► review queue
```

- **Understanding:** each part of a message is translated on its own and checked
  against a closed predicate registry; only failed parts are retried or split.
- **Dialogue:** a session memory, task frames with slots, and reply moves chosen
  by Datalog policy rules, each with its proof.
- **Learning:** procedures, guessed rules, new words and input patterns enter as
  hypotheses, are used on trial ("I was taught this once", "I'd guess so ...
  Is that right?"), and are confirmed or dropped. Corrections are gradual: one
  counterexample is an exception, two with a shared feature change the rule.
- **Measurement:** every turn gets an outcome label (helpful, clarify, teach me,
  not understood, wrong move, wrong content, echo) with confidence intervals.

## Layout

| Path | What |
|---|---|
| `crates/core` | Values, facts, rules, the vault (Markdown files with Datalog blocks), journal, transactions |
| `crates/reasoner` | Stratified Datalog with negation, aggregates, arithmetic, time, proofs, quarantine |
| `crates/learner` | Rule guessing, corrections, learning from demonstrations |
| `crates/lexicon` | The predicate registry and type checks |
| `crates/parser` | Fixed sentence patterns, the oracle parser for tests, the Claude CLI wrapper |
| `crates/translate` | Segmenter, three-tier translator, learned patterns, validator |
| `crates/nlg` | The realizer: content plans to sentences through registry frames |
| `crates/agent` | The agent: brain, sessions, dialogue manager, tasks, learning, review queue, hygiene; the `sla` chat binary |
| `crates/eval` | Outcome labels, regression replay, gold sets, wording, patterns, curriculum runs, benchmarks; `sla_eval` and `sla_trainer` |
| `data/` | Scenarios, curriculum (held-out scenarios and seed procedures), gold sets, synthetic test sets, and `training/` (every generated conversation, kept) |
| `eval/` | Evaluation history, reports and caches |
| `docs/` | Dialogue format, language and parser specs |
| `tools/` | `make_registry.py` (generates the registry), `extract_regression.py` |
| `vault/` | The default knowledge vault |

## Running

Rust stable toolchain (developed with `stable-x86_64-pc-windows-gnu`). Model
calls go through the [Claude Code](https://claude.com/claude-code) CLI (`claude -p`), so it must be
installed and logged in for anything that uses a model; everything else runs offline.

```sh
# all tests (no model calls: model paths run on mocks)
cargo test

# chat with the agent
cargo run --release -p sla_agent --bin sla              # pattern parser only
cargo run --release -p sla_agent --bin sla -- --llm     # with the model translator
#   options: --vault DIR, --script FILE; commands :facts :rules :log :reload :quit

# offline knowledge tests (v1 dialogues)
cargo run --release -p sla_eval -- --no-claude
```

Evaluations (results are appended to `eval/` and conversations archived in `data/training/`):

```sh
sla_eval outcomes --parser llm:haiku --judge haiku   # outcome mix with 95% intervals
sla_eval gold                                        # translator gold set
sla_eval wording --ceiling                           # old vs new wording, judged pairwise
sla_eval patterns --teach 300                        # learned input patterns
sla_eval curriculum [--limit N]                      # the four encounters over held-out scenarios
sla_eval hygiene --vault DIR                         # duplicate entities, knowledge health (use a copy)
sla_eval bench --vault DIR                           # speed of the agent's own work
sla_eval stress --teachers 16 --minutes 10           # parallel writers on mock models
```

(`sla_eval` is `cargo run --release -p sla_eval --`.)

The training dashboard has two parts: the API server, and the page.

- API: `cargo run --release -p sla_eval --bin sla_trainer` (add `--mock` to try it
  without model calls). It prints an address with `?token=...`.
- Page (Vite, React, TypeScript; needs Node.js): `cd dashboard && npm install && npm run dev`,
  then open `http://127.0.0.1:5173/?token=<the token sla_trainer printed>`. Edits to
  `dashboard/src/` show up at once (hot reload). `npm run build` makes a static copy in
  `dashboard/dist/`, which sla_trainer then also serves at its own address.

## Status

Milestones 12 to 17 are built, and milestone 18 (curriculum training and
hygiene) is in progress. Per-milestone acceptance numbers, unmet targets and
the decisions behind them are in [`PROGRESS.md`](PROGRESS.md).
