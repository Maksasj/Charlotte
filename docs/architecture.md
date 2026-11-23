# CaseForge v5 architecture

Describes [`prototypes/caseforge-v5/index.html`](../prototypes/caseforge-v5/index.html)
as it is written, read from the code. Function names in `code font` are the
ones in that file.

## Memory

Everything learnable is a case of one of four kinds, kept in one list.

| Kind | Holds | Count at start | Comes from |
|---|---|---|---|
| **Pattern** | A generic solution: the intents it serves, a skeleton of guarded steps, an order for asking questions and for merging | 3 | Authored; induced from traces; taught |
| **Atom** | One formal fact: predicate, object, conditions over typed slots, a reply payload, a result label, priority, provenance | 16 | Authored; mined from articles; taught |
| **Split** | Connector words that mark where a message divides, for a set of intents | 3 | Authored; learned from resolved conversations |
| **Move** | A cached reply for a message, with the slots it needs | 8 | Authored; mined from chats; taught live; retained on close |

Every case carries `trust` (0.05 to 1), `uses`, `wins`, `source` and
`verified`. Beside the cases there is a small table of **records** (orders,
tickets) that lookups read from, and a log of **traces** used for induction.

A pattern:

```json
{ "name": "gated-request", "shape": "gated",
  "intents": ["refund", "warranty", "reimburse", "leave", "enable", "upgrade", "cancel"],
  "skeleton": [
    { "op": "ask",   "guard": "missing_gate_slots" },
    { "op": "grant", "guard": "gate_satisfied" },
    { "op": "offer", "guard": "violated_with_fallback" },
    { "op": "deny",  "guard": "always" } ],
  "learned": { "askOrder": ["packaging", "days", "..."], "mergeOrder": ["refund", "..."],
               "batch": false, "proactive": true },
  "trust": 0.84, "support": 34 }
```

An atom:

```json
{ "predicate": "refund", "object": "order", "domain": "retail",
  "conditions": [ { "slot": "days", "op": "<=", "value": 30 },
                  { "slot": "packaging", "op": "==", "value": "original" } ],
  "payload": "Refund approved for {subject} — funds return in 3–5 business days.",
  "result": "money_back", "priority": 3,
  "provenance": { "source": "policy-handbook.md", "span": [0, 0] }, "trust": 0.88 }
```

The three patterns and their skeletons:

| Pattern | Shape | Steps (operation when guard holds) |
|---|---|---|
| gated-request | `gated` | ask when gate slots are missing; grant when the gate is satisfied; offer when violated and a fallback exists; else deny |
| status-lookup | `lookup` | ask when the identifier is missing; report when the record is found; else escalate |
| troubleshoot-branch | `branch` | ask when gate slots are missing; fix when satisfied; offer when violated with a fallback; else escalate |

Operations are a closed set of seven (`ask grant offer deny report fix
escalate`) and guards a closed set of eight.

## One turn

```
message
  │  SEGMENT    segment(): find all intents; if two or more, cut the message
  │             with a split case's connectors, else the default connectors,
  │             else at the positions of the intent keywords (anchoring)
  ▼
for each subproblem                                     solveSub()
  │  RECOGNIZE  recognize(): best enabled pattern serving this intent, only if
  │             an atom with conditions exists for the predicate
  │  BIND       bindRoles(): pick the atom, evaluate its gate against the
  │             slots, find a sibling atom as fallback
  │  INTERPRET  interpret(): run the skeleton top to bottom; the first step
  │             whose guard holds produces the reply
  │  else       rankMoves(): best cached move above the threshold
  │  else       cold start: ask the user to teach a reply
  ▼
MERGE           mergeSubs(): order the partial replies, ask each missing slot
                once, flag cancel together with pause as a conflict
  ▼
reply ──► later: verdict per subproblem ──► trust updates, retained moves
```

Slots are read by `extractSlots()` from all user messages of the conversation
joined together, using one fixed regex per slot (19 slots). A clarifying
question leaves the subproblem in an `awaiting` list; the next message that
supplies the slot re-runs it.

## Scoring

| Decision | Rule |
|---|---|
| Intent | Of all regex families that match, the one highest in a fixed priority table, then with the most hits (`detect`). In a message longer than six words a greeting is set aside and the rest is classified (`eff`) |
| Pattern | `fit × (w × specificity + (1 − w) × trust)`, where fit is 1 for an intent match and 0.8 for a predicate match, specificity is `1.2 − 0.06 × number of intents` clamped to 0.3 to 1, and `w` is 0.7. Must reach 0.25 |
| Atom | `(200 if no condition is violated) + 20 × trust + 5 × priority − 0.5 × unbound slots`, among atoms with the same predicate |
| Move | `(0.5 × TF-IDF cosine + 0.5 × intent match) × (0.7 + 0.3 × trust)`. Must reach 0.42 |
| Split case | `0.55 if a connector is present + 0.25 × intent overlap + 0.2 × trust`. Must reach 0.3 |

## What changes with experience

| Signal | Effect | Where |
|---|---|---|
| A case is used in a reply | `trust += 0.25 × (0.65 − trust)` for the pattern, atom and move | `pushAgent` |
| Conversation closed as resolved, partial or failed, adjustable per subproblem | Same update with target 1, 0.5 or 0, on all three layers | `closeConv`, `outcome` |
| Resolved or partial subproblem answered from a move | The clause and reply are stored as a new move | `closeConv` |
| Resolved conversation whose split was found by anchoring | The discovered connector is added to a split case, or a new split case is made | `learnMarkers` |
| User teaches a reply at a cold start | New move at trust 0.8 | `teachCold` |
| Three or more traces with the same step signature across two or more intents, not covered by an existing pattern | A candidate appears; promoting it creates a pattern with the authored skeleton for that shape | `induce`, `promote` |

## Mining

- **Article to atoms** (`mineArticle`): split on `##` headings; map the
  heading to a predicate through six fixed keyword rules (`headPred`); for
  each sentence, extract conditions through seventeen fixed phrases
  (`parseConds`); sentences with no condition are skipped. The word "instead"
  marks an alternative with lower priority. Trust starts at 0.62 to 0.68.
- **Chat to moves** (`mineChats`): blocks headed `[chat id · topic]` with
  `User:` and `Agent:` lines; each user and agent pair becomes a move at trust
  0.5 to 0.71.
- Both flag a candidate as a near-duplicate above 0.9 cosine and wait for
  approval.

## Fixed machinery

None of this is learned or editable from the lab:

- the 22 intent regex families and their priority table (`IP`, `PRIO`);
- the intent to predicate map (`PRED`);
- the 19 slot extractors, their types and prompts (`extractSlots`, `SLOTS`);
- the default connectors and the cancel and pause conflict pair;
- the three skeletons, seven operations and eight guards;
- the article miner's heading and condition rules.
