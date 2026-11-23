# Engine design

Status: proposal, 23 November 2026. Nothing here is built yet.

A Rust engine that replaces the two scripts (`prototypes/caseforge-v5` and
`experiments/pattern-learning`). It holds conversations, keeps all knowledge
in a Markdown vault, and learns from conversations in bulk.

## Requirements

1. An API to hold conversations with, as a library, over HTTP and from a
   terminal.
2. Layers with fixed boundaries, so one layer can be replaced or improved
   without touching the others.
3. All knowledge in an Obsidian-style Markdown vault that a person can read and
   edit.
4. Learning through the same API, sized for a batch of 10,000 generated
   conversations.

Carried over from the findings:

- Every reply has a trace that shows which layer decided what.
- Learned knowledge is a candidate until it reproduces conversations it was
  not learned from.
- Vocabulary (intents, slots, phrases) is data in the vault, not tables in the
  code.

## Layers

One turn passes through five layers. Each is a trait in the core crate with
one default implementation; the engine is assembled from whichever
implementations are configured.

```
text ─► 1 UNDERSTAND ─► 2 TRACK ─► 3 DECIDE ─► 4 REALIZE ─► text
             │              │          │            │
             └──────────────┴────┬─────┴────────────┘
                              5 MEMORY (vault)
                                 ▲
                              LEARN (offline, writes candidates)
```

| # | Layer | Input → output | First implementation | Later replacements |
|---|---|---|---|---|
| 1 | Understand | Message text, state → `Reading`: the requests in the message, each with an intent, the facts stated and the user's act | Phrase lexicon read from the vault | Learned classifier; a language model |
| 2 | Track | `Reading`, previous state → `State`: one frame per open request with its own facts, the acts so far, the decision that currently stands | Deterministic tracker | Learned state features |
| 3 | Decide | `State` → `Act` per request (operation and arguments, no wording) | Pick a pattern, bind an atom, run the pattern's rule list | Other policies |
| 4 | Realize | Acts → reply text | Templates from the atom and the pattern | Anything that writes better |
| 5 | Memory | Queries from the other layers → knowledge items | Markdown vault with an in-memory index | Other stores behind the same trait |

Boundaries are plain data types (`Reading`, `State`, `Act`, `Trace`) that
serialise to JSON. That is what makes a layer replaceable: a new Understand
layer only has to produce a `Reading`.

How the layers answer the v5 defects:

| Defect | Where it is fixed |
|---|---|
| D1 facts shared by all requests | Track: one frame per request |
| D2 negation ignored | Understand: a lexicon entry carries polarity |
| D3 atoms of another object compete | Decide: atoms are bound by predicate and object type |
| D4 alternatives decided outside the skeleton | Decide: the rule list chooses offer and deny; atom ranking only orders candidates |
| D5 promotion crash | Learn: candidates are complete items, validated before promotion |
| No memory of the standing decision | Track: recorded in the frame |

## Knowledge vault

A folder of Markdown notes. Each note is one item: YAML front matter for the
formal part, body text for people, `[[links]]` between items so the graph view
shows what depends on what. The vault is the source of truth; the index the
engine builds from it is disposable.

```
vault/
  patterns/        gated-request.md, status-lookup.md, ...
  atoms/<domain>/  refund-order-within-30-days.md, ...
  lexicon/
    intents/       refund.md          phrases that mean this intent
    slots/         packaging.md       type, question to ask, phrases per value
    connectors.md                     words that divide a message
  records/         orders.md          lookup tables
  moves/           cached replies for things with no pattern
  candidates/      learned items waiting for validation or review
```

An atom:

```markdown
---
id: atom.refund.order.30d
kind: atom
predicate: "[[refund]]"
object: order
conditions:
  - { slot: "[[days]]", op: "<=", value: 30 }
  - { slot: "[[packaging]]", op: "==", value: original }
result: grant
status: trusted          # candidate | trusted | retired
trust: 0.88
source: policy-handbook.md
---
Refund approved for {subject} — funds return in 3–5 business days.
```

A pattern is the learned rule list in the same style:

```markdown
---
id: pattern.gated-request
kind: pattern
intents: ["[[refund]]", "[[warranty]]", "[[cancel]]"]
rules:
  - { when: [gate_unbound, not gate_violated], do: "ask(first_unbound)" }
  - { when: [not gate_violated], do: grant }
  - { when: [alt_violated], do: deny }
  - { when: [alt_unbound], do: "ask(alt_first_unbound)" }
  - { otherwise: offer }
status: trusted
trust: 0.84
support: 80              # conversations it was learned from
validated: 0.99          # share of held-out conversations reproduced
---
```

An intent in the lexicon, which is where understanding grows:

```markdown
---
id: refund
kind: intent
predicate: refund
phrases:
  - { text: "refund", weight: 1.0 }
  - { text: "money back", weight: 1.0 }
  - { text: "send back", weight: 0.7, status: candidate, support: 14 }
---
```

Raw conversations are **not** vault notes. Ten thousand of them would bury the
knowledge. They live in a corpus folder as JSON lines; a learned item records
how many conversations support it and can list a few by id.

## API

### Library

```rust
let engine = Engine::open("vault/")?;

let mut conv = engine.conversation();
let reply = conv.say("I'd like to return order 7741, got it 5 days ago")?;
println!("{}", reply.text);
println!("{:#?}", reply.trace);          // reading, state, pattern, atom, rule fired
conv.close(Verdict::Resolved)?;          // feedback: adjusts trust

let report = engine.learn(Corpus::open("corpus/batch-01.jsonl")?, LearnOptions::default())?;
println!("{report}");                    // what was proposed, validated, rejected
```

### HTTP

| Method and path | Purpose |
|---|---|
| `POST /v1/conversations` | Start a conversation |
| `POST /v1/conversations/{id}/messages` | Send a message; returns the reply and its trace |
| `GET /v1/conversations/{id}` | Transcript and state |
| `POST /v1/conversations/{id}/close` | Verdict, optionally per request |
| `POST /v1/learn` | Start a learning job over a corpus file or an uploaded batch |
| `GET /v1/learn/{job}` | Progress and the report |
| `GET /v1/knowledge?kind=&status=` | List items |
| `POST /v1/knowledge/{id}/promote` · `/reject` | Review a candidate |
| `GET /v1/health` | Vault path, item counts, index age |

### Terminal

```
charlotte chat  --vault vault/                 talk to it, :trace shows the last trace
charlotte serve --vault vault/ --port 7878
charlotte learn corpus/batch-01.jsonl          prints the report
charlotte check                                lint the vault: broken links, unknown slots
```

## Learning

### Corpus format

One conversation per line. Only `turns[].role` and `turns[].text` are
required; every label is optional and decides what can be learned from it.

```json
{"id": "c000123", "domain": "retail", "outcome": "resolved", "turns": [
  {"role": "user",  "text": "can I send back order 7741? got it 5 days ago",
   "intent": "refund", "facts": {"subject": "7741", "days": 5}},
  {"role": "agent", "text": "Is it still in the original packaging?",
   "act": "ask", "arg": "packaging"},
  {"role": "user",  "text": "yes, never opened", "facts": {"packaging": "original"}},
  {"role": "agent", "text": "Refund approved for 7741.", "act": "grant"}
]}
```

| Labels present | What can be learned |
|---|---|
| Text only | Cached replies (moves) |
| + `outcome` | Trust of whatever was used |
| + agent `act` and `arg` | Patterns: the rule lists |
| + user `intent` and `facts` | Lexicon: which phrases mean which intent and which fact |

### Pipeline

```
corpus ─► ingest ─► split 90/10 ─► induce ─► validate on the 10% ─► write candidates ─► report
```

- **Ingest** parses and checks each conversation and skips broken ones with a
  count, so one bad line does not stop a batch of 10,000.
- **Induce** runs one learner per kind of knowledge. The pattern learner is the
  one from the experiment (ordered rule list). The lexicon learner collects
  the phrases that go with each labelled intent and fact.
- **Validate** replays the held-out conversations through the full engine with
  the candidates switched on. A candidate is kept only if the conversations it
  takes part in are reproduced.
- **Write** puts candidates in `vault/candidates/`. Promotion to trusted is
  automatic above a threshold or manual through the API; both are recorded in
  the note.
- **Report** says what was proposed, kept and dropped, the held-out score
  before and after, and the commonest mistakes.

Learning is offline and repeatable: the same corpus and vault give the same
candidates. A running server picks up vault changes by reloading the index.

### What the experiment says about the 10,000 conversations

These shape the generator more than the engine:

- **Spread over many domains.** Variety of domains mattered more than volume;
  6 domains stalled at about 95% however many conversations were added.
- **Include the combinations.** Shapes learned separately did not combine, so
  lookups feeding rule checks, corrections and multi-request messages have to
  appear in the corpus.
- **Consistent labels.** One wrong act label in ten cost 4 to 19 points. The
  ingest step should measure label consistency and report it before anything
  is learned.
- **Labels are what make it useful.** Without act labels no pattern can be
  learned; without intent and fact labels the lexicon cannot grow.

## Code layout

One workspace, four crates. The first research line ended with nineteen; this
one stays small until something forces a split.

| Crate | Holds |
|---|---|
| `charlotte-core` | Data types (`Reading`, `State`, `Act`, `Trace`, knowledge items) and the five layer traits |
| `charlotte-vault` | Reading and writing notes, the index, vault lint |
| `charlotte-engine` | Default layer implementations, the turn pipeline, the learners |
| `charlotte` | The binary: `chat`, `serve`, `learn`, `check` |

## Build order

Each step ends with one check that passes or fails.

| Step | Builds | Check |
|---|---|---|
| 1 | Core types, vault reader and writer, a seed vault converted from v5 | The seed vault loads, and writing it back changes nothing |
| 2 | The five layers with default implementations | The engine gives the answers `probe.mjs` gives for the six domains, and the right answers where D1 to D4 gave wrong ones |
| 3 | Library API, `chat`, HTTP server | A scripted conversation over HTTP matches the same one through the library |
| 4 | Pattern learner and the learn pipeline | With the gated pattern removed from the vault, learning from 80 simulated conversations restores it and step 2's check passes again |
| 5 | Lexicon learner | The five rewordings that v5 could not read are answered after learning from other labelled conversations that use the same words |
| 6 | The 10,000-conversation batch | The batch runs to a report without manual steps |

## Open decisions

1. **Labels in the generated conversations.** Can the local model emit the act,
   intent and facts with each turn? The whole learning design leans on it.
2. **A model inside the engine.** The Understand layer is replaceable either
   way. The question is whether the first implementation may call a model or
   must stay lexicon-only.
3. **Automatic promotion.** Should validated candidates become trusted on
   their own, or always wait for review?
