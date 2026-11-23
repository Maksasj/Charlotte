# HTTP API

Served by `charlotte serve` on `http://127.0.0.1:7878`. JSON in and out.
Errors are `{"error": "message"}` with status 400 (the request is not valid),
404 (no such conversation, item or job) or 409 (the conversation is closed, or
the item is not a candidate).

## Conversations

| Method and path | Body | Returns |
|---|---|---|
| `POST /v1/conversations` | — | `201 {"id": "c1"}` |
| `GET /v1/conversations` | — | `[ConversationSummary]`, newest first |
| `GET /v1/conversations/{id}` | — | `Conversation` |
| `POST /v1/conversations/{id}/messages` | `{"text": "..."}` | `Reply` |
| `POST /v1/conversations/{id}/close` | `{"verdict": "resolved" \| "partial" \| "failed"}` | `{"updated": [TrustChange]}` |

```jsonc
// ConversationSummary
{ "id": "c1", "turns": 4, "closed": false, "verdict": null, "last": "last message text" }

// Conversation
{ "id": "c1", "closed": false, "verdict": null,
  "messages": [ { "role": "user", "text": "..." },
                { "role": "agent", "text": "...", "trace": Trace } ],
  "frames": [Frame] }

// Reply
{ "text": "Is it still in the original packaging, unopened?",
  "acts": [ { "op": "ask", "arg": "packaging" } ],
  "trace": Trace }

// Trace: one entry per request found in the message
{ "requests": [RequestTrace], "notes": ["free-text remarks from the engine"] }

// RequestTrace: what each layer did for one request
{ "clause": "can I send back order 7741",          // the part of the message
  "intent": "refund",                               // null when the clause names no request: an answer such as
                                                    // "5 days ago" continues an open frame; if `frame` is null too,
                                                    // the clause was not understood
  "matched": [ { "phrase": "send back", "kind": "intent", "target": "refund" },
               { "phrase": "order 7741", "kind": "fact", "target": "subject" } ],
  "facts": { "subject": "7741", "days": 5 },        // facts read from this clause
  "user_act": "request",                            // request | provide | correct | done | failed | null
  "kind": "gated",                                  // gated | procedure | lookup | move | unknown
  "pattern": "check-a-rule",                        // knowledge ids, null when not used
  "atom": "refund-order-within-30-days",
  "alternative": "refund-order-opened",
  "steps": [ { "features": ["req_gated", "p_unbound", "last_user=request"],   // state facts that were true
               "rule": { "index": 1, "when": ["p_unbound", "not p_viol"], "do": "ask(p_first_unbound)" },  // null if no rule fired
               "act": { "op": "ask", "arg": "packaging" } } ],
  "text": "Is it still in the original packaging, unopened?",
  "frame": Frame }

// Frame: the engine's state for one open request
{ "id": 1, "intent": "refund", "facts": { "subject": "7741", "days": 5 },
  "agent_acts": ["ask"], "user_acts": ["request"],
  "last_asked": "packaging", "standing": null,      // standing: grant | offer | deny | null
  "fetched": false, "found": false, "steps_done": 0 }

// TrustChange
{ "id": "check-a-rule", "before": 0.8, "after": 0.85 }
```

Operations (`op`): `ask grant offer deny confirm lookup report instruct repeat
finish escalate say teach_me`.

## Knowledge

| Method and path | Body | Returns |
|---|---|---|
| `GET /v1/knowledge?kind=&status=&q=` | — | `[ItemSummary]` (all filters optional) |
| `GET /v1/knowledge/{id}` | — | `Item` |
| `PUT /v1/knowledge/{id}` | `{"front": {...}, "body": "..."}`, both required | `Item`, or `400` if the note is not valid |
| `POST /v1/knowledge/{id}/promote` | — | `Item` (the promoted item) |
| `POST /v1/knowledge/{id}/reject` | — | `{"ok": true}` |

```jsonc
// ItemSummary
{ "id": "refund-order-within-30-days", "kind": "atom", "status": "trusted",
  "trust": 0.88, "path": "atoms/retail/refund-order-within-30-days.md",
  "title": "refund · order" }

// Item
{ "id": "...", "kind": "atom", "status": "trusted", "trust": 0.88, "path": "...",
  "front": { /* the note's front matter as JSON */ },
  "body": "Refund approved for {subject} ...",
  "links": ["refund", "days"], "backlinks": ["check-a-rule"] }
```

Kinds: `pattern atom procedure intent slot records move lexicon phrases`.
Statuses: `candidate trusted retired`.

## Learning

| Method and path | Body | Returns |
|---|---|---|
| `POST /v1/learn` | `{"path": "corpus/file.jsonl"}` or `{"jsonl": "..."}` (the path wins if both are given), optional `"auto_promote"`, true by default | `202 {"job": "j1"}` |
| `GET /v1/learn` | — | `[Job]`, newest first |
| `GET /v1/learn/{job}` | — | `Job` |

```jsonc
// Job
{ "id": "j1", "state": "running",                  // running | done | failed
  "stage": "validate",                              // ingest | induce | validate | write | done
  "source": "corpus/pretrain-80.jsonl",
  "report": null,                                   // Report when done
  "error": null }

// Report
{ "conversations": { "read": 80, "skipped": 0, "train": 64, "held_out": 16 },
  "labels": { "act": 1.0, "intent": 1.0, "facts": 1.0 },   // share of turns carrying each label
  "held_out": { "before": 0.0, "after": 0.94 },            // share of held-out conversations reproduced exactly
  "understanding": { "before": 0.71, "after": 0.9 },       // share of held-out user turns whose intent is read correctly
  "candidates": [ { "id": "check-a-rule", "kind": "pattern", "support": 51,
                    "validated": 0.95, "promoted": true,
                    "summary": "5 rules for gated requests" } ],
  "mistakes": [ { "what": "teacher deny, engine offer", "count": 2 } ],
  "seconds": 0.4 }
```

`candidates` lists everything the job learned. An entry that is neither
promoted nor in the review queue was identical to what is already trusted; its
summary says so.

## Status

`GET /v1/health` →

```jsonc
{ "version": "0.1.0", "vault": "/abs/path/vault",
  "items": { "atom": 19, "pattern": 3, "intent": 14 },    // trusted items per kind
  "candidates": 0, "conversations": 2 }
```
