// Development-only stand-in for the engine, enabled with ?mock=1.
// It replaces `fetch` underneath the real client, so URL building, JSON
// handling and error handling in client.ts are exercised unchanged.
// Every response is shaped per docs/api.md and per what the real engine sends
// (omitted `arg`, null `frame` for a request that was not understood, null
// `intent` on a clause that continues an open request). Nothing here is engine logic:
// replies to new messages are canned.

import type {
  Conversation,
  ConversationSummary,
  Frame,
  Health,
  Item,
  ItemKind,
  ItemStatus,
  ItemSummary,
  Job,
  JobStage,
  Message,
  Reply,
  Report,
  RequestTrace,
  Trace,
  TrustChange,
  Verdict,
} from "./types";
import { ITEM_KINDS, ITEM_STATUSES } from "./types";

// ---- Fixtures: conversations ----

const refundFrameAsked: Frame = {
  id: 1,
  intent: "refund",
  facts: { subject: "7741", days: 5 },
  agent_acts: ["ask"],
  user_acts: ["request"],
  last_asked: "packaging",
  standing: null,
  fetched: false,
  found: false,
  steps_done: 0,
};

const statusFrame: Frame = {
  id: 2,
  intent: "order-status",
  facts: { subject: "8802" },
  agent_acts: ["lookup", "report"],
  user_acts: ["request"],
  last_asked: null,
  standing: null,
  fetched: true,
  found: true,
  steps_done: 0,
};

const refundFrameGranted: Frame = {
  id: 1,
  intent: "refund",
  facts: { subject: "7741", days: 5, packaging: "original" },
  agent_acts: ["ask", "grant"],
  user_acts: ["request", "provide"],
  last_asked: "packaging",
  standing: "grant",
  fetched: false,
  found: false,
  steps_done: 0,
};

const traceFirst: Trace = {
  requests: [
    {
      clause: "can I send back order 7741? got it 5 days ago",
      intent: "refund",
      matched: [
        { phrase: "send back", kind: "intent", target: "refund" },
        { phrase: "order 7741", kind: "fact", target: "subject" },
        { phrase: "5 days ago", kind: "fact", target: "days" },
      ],
      facts: { subject: "7741", days: 5 },
      user_act: "request",
      kind: "gated",
      pattern: "check-a-rule",
      atom: "refund-order-within-30-days",
      alternative: "refund-order-opened",
      steps: [
        {
          features: [
            "req_gated",
            "p_unbound",
            "not p_viol",
            "alt_unbound",
            "last_user=request",
            "standing=none",
            "agent_acts=0",
          ],
          rule: { index: 1, when: ["p_unbound", "not p_viol"], do: "ask(p_first_unbound)" },
          act: { op: "ask", arg: "packaging" },
        },
      ],
      text: "Is it still in the original packaging, unopened?",
      frame: refundFrameAsked,
    },
    {
      clause: "where is order 8802",
      intent: "order-status",
      matched: [
        { phrase: "where is", kind: "intent", target: "order-status" },
        { phrase: "order 8802", kind: "fact", target: "subject" },
      ],
      facts: { subject: "8802" },
      user_act: "request",
      kind: "lookup",
      pattern: "status-lookup",
      atom: null,
      alternative: null,
      steps: [
        {
          features: ["req_lookup", "key_bound", "not fetched", "last_user=request"],
          rule: { index: 0, when: ["key_bound", "not fetched"], do: "lookup(orders)" },
          act: { op: "lookup", arg: "orders" },
        },
        {
          features: ["req_lookup", "key_bound", "fetched", "found", "last_user=request"],
          rule: { index: 2, when: ["fetched", "found"], do: "report" },
          act: { op: "report" },
        },
      ],
      text: "Order 8802 shipped on Tuesday and is due on Friday.",
      frame: statusFrame,
    },
  ],
  notes: [],
};

const traceSecond: Trace = {
  requests: [
    {
      clause: "yes, never opened",
      intent: null,
      matched: [{ phrase: "never opened", kind: "fact", target: "packaging" }],
      facts: { packaging: "original" },
      user_act: "provide",
      kind: "gated",
      pattern: "check-a-rule",
      atom: "refund-order-within-30-days",
      alternative: "refund-order-opened",
      steps: [
        {
          features: ["req_gated", "p_bound", "not p_viol", "last_user=provide", "last_agent=ask", "agent_acts=1"],
          rule: { index: 2, when: ["not p_viol"], do: "grant" },
          act: { op: "grant" },
        },
      ],
      text: "Refund approved for 7741 — funds return in 3–5 business days.",
      frame: refundFrameGranted,
    },
  ],
  notes: [],
};

function unknownRequest(clause: string): RequestTrace {
  return {
    clause,
    intent: null,
    matched: [],
    facts: {},
    user_act: null,
    kind: "unknown",
    pattern: null,
    atom: null,
    alternative: null,
    steps: [
      {
        features: [],
        rule: null,
        act: { op: "teach_me" },
      },
    ],
    text: "I don't know how to help with that yet. Can you tell me what you need in other words?",
    frame: null,
  };
}

const traceThird: Trace = {
  requests: [unknownRequest("also my flux capacitor is humming")],
  notes: ["no lexicon phrase matched the clause", "no pattern covers request kind `unknown`; fell back to teach_me"],
};

const conversations: Conversation[] = [
  {
    id: "c2",
    closed: false,
    verdict: null,
    messages: [
      { role: "user", text: "can I send back order 7741? got it 5 days ago, and where is order 8802" },
      {
        role: "agent",
        text: "Is it still in the original packaging, unopened? Order 8802 shipped on Tuesday and is due on Friday.",
        trace: traceFirst,
      },
      { role: "user", text: "yes, never opened" },
      { role: "agent", text: "Refund approved for 7741 — funds return in 3–5 business days.", trace: traceSecond },
      { role: "user", text: "also my flux capacitor is humming" },
      {
        role: "agent",
        text: "I don't know how to help with that yet. Can you tell me what you need in other words?",
        trace: traceThird,
      },
    ],
    frames: [refundFrameGranted, statusFrame],
  },
  {
    id: "c1",
    closed: true,
    verdict: "resolved",
    messages: [
      { role: "user", text: "where is order 8802" },
      {
        role: "agent",
        text: "Order 8802 shipped on Tuesday and is due on Friday.",
        trace: { requests: [traceFirst.requests[1]!], notes: [] },
      },
    ],
    frames: [statusFrame],
  },
];

// ---- Fixtures: knowledge ----

const items: Item[] = [
  {
    id: "check-a-rule",
    kind: "pattern",
    status: "trusted",
    trust: 0.84,
    path: "patterns/check-a-rule.md",
    front: {
      id: "check-a-rule",
      kind: "pattern",
      intents: ["[[refund]]", "[[warranty]]", "[[cancel]]"],
      rules: [
        { when: ["p_unbound", "not p_viol"], do: "ask(p_first_unbound)" },
        { when: ["not p_viol"], do: "grant" },
        { when: ["alt_viol"], do: "deny" },
        { when: ["alt_unbound"], do: "ask(alt_first_unbound)" },
        { otherwise: "offer" },
      ],
      status: "trusted",
      trust: 0.84,
      support: 80,
      validated: 0.99,
    },
    body: "Gated requests: ask for whatever the atom's conditions still need, then grant, offer the alternative or deny.",
    links: ["refund", "warranty", "cancel"],
    backlinks: [],
  },
  {
    id: "status-lookup",
    kind: "pattern",
    status: "trusted",
    trust: 0.9,
    path: "patterns/status-lookup.md",
    front: {
      id: "status-lookup",
      kind: "pattern",
      intents: ["[[order-status]]"],
      rules: [
        { when: ["key_bound", "not fetched"], do: "lookup(orders)" },
        { when: ["not key_bound"], do: "ask(key)" },
        { when: ["fetched", "found"], do: "report" },
        { otherwise: "say(not_found)" },
      ],
      status: "trusted",
      trust: 0.9,
    },
    body: "Look a record up by its key and report what was found.",
    links: ["order-status", "orders"],
    backlinks: [],
  },
  {
    id: "refund-order-within-30-days",
    kind: "atom",
    status: "trusted",
    trust: 0.88,
    path: "atoms/retail/refund-order-within-30-days.md",
    front: {
      id: "refund-order-within-30-days",
      kind: "atom",
      predicate: "[[refund]]",
      object: "order",
      conditions: [
        { slot: "[[days]]", op: "<=", value: 30 },
        { slot: "[[packaging]]", op: "==", value: "original" },
      ],
      result: "grant",
      status: "trusted",
      trust: 0.88,
      source: "policy-handbook.md",
    },
    body: "Refund approved for {subject} — funds return in 3–5 business days.",
    links: ["refund", "days", "packaging"],
    backlinks: ["refund-order-opened"],
  },
  {
    id: "refund-order-opened",
    kind: "atom",
    status: "trusted",
    trust: 0.8,
    path: "atoms/retail/refund-order-opened.md",
    front: {
      id: "refund-order-opened",
      kind: "atom",
      predicate: "[[refund]]",
      object: "order",
      conditions: [{ slot: "[[days]]", op: "<=", value: 30 }],
      result: "offer",
      alternative_to: "[[refund-order-within-30-days]]",
      status: "trusted",
      trust: 0.8,
    },
    body: "{subject} has been opened, so we can offer store credit instead of a refund.",
    links: ["refund", "days", "refund-order-within-30-days"],
    backlinks: [],
  },
  {
    id: "refund",
    kind: "intent",
    status: "trusted",
    trust: 0.95,
    path: "lexicon/intents/refund.md",
    front: {
      id: "refund",
      kind: "intent",
      predicate: "refund",
      phrases: [
        { text: "refund", weight: 1.0 },
        { text: "money back", weight: 1.0 },
        { text: "send back", weight: 0.7, status: "candidate", support: 14 },
      ],
    },
    body: "",
    links: [],
    backlinks: ["check-a-rule", "refund-order-within-30-days", "refund-order-opened", "refund.send-back"],
  },
  {
    id: "packaging",
    kind: "slot",
    status: "trusted",
    trust: 0.9,
    path: "lexicon/slots/packaging.md",
    front: {
      id: "packaging",
      kind: "slot",
      type: "enum",
      question: "Is it still in the original packaging, unopened?",
      values: { original: ["never opened", "unopened", "sealed"], opened: ["opened it", "used it"] },
    },
    body: "",
    links: [],
    backlinks: ["refund-order-within-30-days"],
  },
  {
    id: "days",
    kind: "slot",
    status: "trusted",
    trust: 0.92,
    path: "lexicon/slots/days.md",
    front: { id: "days", kind: "slot", type: "number", question: "How many days ago did it arrive?" },
    body: "",
    links: [],
    backlinks: ["refund-order-within-30-days", "refund-order-opened"],
  },
  {
    id: "orders",
    kind: "records",
    status: "trusted",
    trust: 1,
    path: "records/orders.md",
    front: { id: "orders", kind: "records", key: "subject", fields: ["shipped", "due"] },
    body: "| subject | shipped | due |\n|---|---|---|\n| 7741 | Monday | Thursday |\n| 8802 | Tuesday | Friday |",
    links: [],
    backlinks: ["status-lookup"],
  },
  {
    id: "greeting-v1",
    kind: "move",
    status: "retired",
    trust: 0.31,
    path: "moves/greeting-v1.md",
    front: { id: "greeting-v1", kind: "move", triggers: ["hello", "hi"], status: "retired", trust: 0.31 },
    body: "Hello! What can I do for you?",
    links: [],
    backlinks: [],
  },
  {
    id: "warranty-claim-v2",
    kind: "pattern",
    status: "candidate",
    trust: 0.5,
    path: "candidates/warranty-claim-v2.md",
    front: {
      id: "warranty-claim-v2",
      kind: "pattern",
      intents: ["[[warranty]]"],
      rules: [
        { when: ["p_unbound"], do: "ask(p_first_unbound)" },
        { when: ["p_viol"], do: "deny" },
        { otherwise: "grant" },
      ],
      status: "candidate",
      trust: 0.5,
      support: 37,
      validated: 0.92,
    },
    body: "Learned from job j1.",
    links: ["warranty"],
    backlinks: [],
  },
  {
    id: "refund.send-back",
    kind: "phrases",
    status: "candidate",
    trust: 0.5,
    path: "candidates/refund.send-back.md",
    front: {
      id: "refund.send-back",
      kind: "phrases",
      intent: "[[refund]]",
      phrases: [{ text: "send back", weight: 0.7 }],
      status: "candidate",
      support: 14,
    },
    body: "",
    links: ["refund"],
    backlinks: [],
  },
];

// ---- Fixtures: learning ----

const report: Report = {
  conversations: { read: 80, skipped: 1, train: 64, held_out: 16 },
  labels: { act: 1.0, intent: 0.975, facts: 0.9 },
  held_out: { before: 0.0, after: 0.94 },
  understanding: { before: 0.71, after: 0.9 },
  candidates: [
    {
      id: "check-a-rule",
      kind: "pattern",
      support: 51,
      validated: 0.95,
      promoted: true,
      summary: "5 rules for gated requests",
    },
    {
      id: "warranty-claim-v2",
      kind: "pattern",
      support: 37,
      validated: 0.92,
      promoted: false,
      summary: "3 rules for warranty claims",
    },
    {
      id: "refund.send-back",
      kind: "phrases",
      support: 14,
      validated: 0.88,
      promoted: false,
      summary: "1 phrase for intent refund",
    },
  ],
  mistakes: [
    { what: "teacher deny, engine offer", count: 2 },
    { what: "teacher ask(days), engine ask(packaging)", count: 1 },
  ],
  seconds: 0.4,
};

const jobs: Job[] = [
  { id: "j1", state: "done", stage: "done", source: "corpus/pretrain-80.jsonl", report, error: null },
];

// ---- Handlers ----

class HttpError extends Error {
  constructor(
    readonly status: number,
    message: string,
  ) {
    super(message);
  }
}

function summaryOf(c: Conversation): ConversationSummary {
  const last = c.messages[c.messages.length - 1];
  return { id: c.id, turns: c.messages.length, closed: c.closed, verdict: c.verdict, last: last ? last.text : null };
}

function findConversation(id: string): Conversation {
  const c = conversations.find((x) => x.id === id);
  if (!c) throw new HttpError(404, `no conversation ${id}`);
  return c;
}

function titleOf(item: Item): string {
  const predicate = item.front["predicate"];
  const object = item.front["object"];
  if (typeof predicate === "string" && typeof object === "string") {
    return `${predicate.replace(/[[\]]/g, "")} · ${object}`;
  }
  return item.id;
}

function itemSummary(item: Item): ItemSummary {
  return {
    id: item.id,
    kind: item.kind,
    status: item.status,
    trust: item.trust,
    path: item.path,
    title: titleOf(item),
  };
}

function findItem(id: string): Item {
  const item = items.find((x) => x.id === id);
  if (!item) throw new HttpError(404, `no knowledge item ${id}`);
  return item;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

const STAGES: JobStage[] = ["ingest", "induce", "validate", "write", "done"];

function advance(job: Job): void {
  if (job.state !== "running") return;
  if (job.source.includes("missing")) {
    job.state = "failed";
    job.error = `cannot open ${job.source}: No such file or directory`;
    return;
  }
  const next = STAGES[STAGES.indexOf(job.stage) + 1] ?? "done";
  job.stage = next;
  if (next === "done") {
    job.state = "done";
    job.report = report;
  }
}

function route(method: string, path: string, query: URLSearchParams, body: unknown): { status: number; body: unknown } {
  const parts = path.split("/").filter(Boolean).map(decodeURIComponent);
  if (parts[0] !== "v1") throw new HttpError(404, "not found");
  const [, resource, id, action] = parts;

  if (resource === "conversations") {
    if (!id) {
      if (method === "POST") {
        const created: Conversation = {
          id: `c${conversations.length + 1}`,
          closed: false,
          verdict: null,
          messages: [],
          frames: [],
        };
        conversations.unshift(created);
        return { status: 201, body: { id: created.id } };
      }
      if (method === "GET") return { status: 200, body: conversations.map(summaryOf) };
    } else if (!action && method === "GET") {
      return { status: 200, body: findConversation(id) };
    } else if (action === "messages" && method === "POST") {
      const c = findConversation(id);
      if (c.closed) throw new HttpError(409, `conversation ${id} is closed`);
      if (!isRecord(body) || typeof body["text"] !== "string" || body["text"].trim() === "") {
        throw new HttpError(400, "text must be a non-empty string");
      }
      const text = body["text"];
      const request = unknownRequest(text);
      const reply: Reply = {
        text: request.text,
        acts: request.steps.map((s) => s.act),
        trace: { requests: [request], notes: ["mock engine: every new message is answered as not understood"] },
      };
      const user: Message = { role: "user", text };
      const agent: Message = { role: "agent", text: reply.text, trace: reply.trace };
      c.messages.push(user, agent);
      return { status: 200, body: reply };
    } else if (action === "close" && method === "POST") {
      const c = findConversation(id);
      if (c.closed) throw new HttpError(409, `conversation ${id} is already closed`);
      const verdict = isRecord(body) ? body["verdict"] : undefined;
      if (verdict !== "resolved" && verdict !== "partial" && verdict !== "failed") {
        throw new HttpError(400, "verdict must be resolved, partial or failed");
      }
      c.closed = true;
      c.verdict = verdict satisfies Verdict;
      const delta = verdict === "resolved" ? 0.05 : verdict === "partial" ? 0 : -0.1;
      const updated: TrustChange[] = ["check-a-rule", "refund-order-within-30-days"].map((itemId) => {
        const item = findItem(itemId);
        const before = item.trust;
        item.trust = Math.max(0, Math.min(1, Math.round((before + delta) * 100) / 100));
        return { id: itemId, before, after: item.trust };
      });
      return { status: 200, body: { updated } };
    }
  }

  if (resource === "knowledge") {
    if (!id && method === "GET") {
      const kind = query.get("kind");
      const status = query.get("status");
      const q = (query.get("q") ?? "").toLowerCase();
      if (kind && !(ITEM_KINDS as readonly string[]).includes(kind)) throw new HttpError(400, `unknown kind ${kind}`);
      if (status && !(ITEM_STATUSES as readonly string[]).includes(status)) {
        throw new HttpError(400, `unknown status ${status}`);
      }
      const found = items
        .filter((item) => !kind || item.kind === kind)
        .filter((item) => !status || item.status === status)
        .filter((item) => !q || `${item.id} ${titleOf(item)} ${item.body}`.toLowerCase().includes(q))
        .map(itemSummary);
      return { status: 200, body: found };
    }
    if (id && !action && method === "GET") return { status: 200, body: findItem(id) };
    if (id && !action && method === "PUT") {
      const item = findItem(id);
      if (!isRecord(body) || !isRecord(body["front"]) || typeof body["body"] !== "string") {
        throw new HttpError(400, "note is not valid: expected front (object) and body (string)");
      }
      const front = body["front"];
      const kind = front["kind"];
      if (typeof kind !== "string" || !(ITEM_KINDS as readonly string[]).includes(kind)) {
        throw new HttpError(400, "note is not valid: front matter needs a known `kind`");
      }
      item.front = front;
      item.body = body["body"];
      item.kind = kind as ItemKind;
      const status = front["status"];
      if (typeof status === "string" && (ITEM_STATUSES as readonly string[]).includes(status)) {
        item.status = status as ItemStatus;
      }
      if (typeof front["trust"] === "number") item.trust = front["trust"];
      return { status: 200, body: item };
    }
    if (id && action === "promote" && method === "POST") {
      const item = findItem(id);
      if (item.status !== "candidate") throw new HttpError(400, `${id} is not a candidate`);
      item.status = "trusted";
      item.front = { ...item.front, status: "trusted" };
      item.path = item.path.replace(/^candidates\//, `${item.kind}s/`);
      return { status: 200, body: item };
    }
    if (id && action === "reject" && method === "POST") {
      const item = findItem(id);
      if (item.status !== "candidate") throw new HttpError(400, `${id} is not a candidate`);
      items.splice(items.indexOf(item), 1);
      return { status: 200, body: { ok: true } };
    }
  }

  if (resource === "learn") {
    if (!id && method === "POST") {
      if (!isRecord(body)) throw new HttpError(400, "expected a JSON object");
      const source =
        typeof body["path"] === "string" && body["path"] !== ""
          ? body["path"]
          : typeof body["jsonl"] === "string" && body["jsonl"].trim() !== ""
            ? "uploaded"
            : null;
      if (source === null) throw new HttpError(400, "give either path or jsonl");
      const job: Job = {
        id: `j${jobs.length + 1}`,
        state: "running",
        stage: "ingest",
        source,
        report: null,
        error: null,
      };
      jobs.unshift(job);
      return { status: 202, body: { job: job.id } };
    }
    if (!id && method === "GET") return { status: 200, body: jobs };
    if (id && method === "GET") {
      const job = jobs.find((j) => j.id === id);
      if (!job) throw new HttpError(404, `no job ${id}`);
      const snapshot = { ...job };
      advance(job);
      return { status: 200, body: snapshot };
    }
  }

  if (resource === "health" && method === "GET") {
    const perKind: Record<string, number> = {};
    for (const item of items) {
      if (item.status === "trusted") perKind[item.kind] = (perKind[item.kind] ?? 0) + 1;
    }
    const health: Health = {
      version: "0.1.0-mock",
      vault: "/abs/path/vault",
      items: perKind,
      candidates: items.filter((i) => i.status === "candidate").length,
      conversations: conversations.length,
    };
    return { status: 200, body: health };
  }

  throw new HttpError(404, `no route for ${method} ${path}`);
}

export async function mockFetch(url: string, init: RequestInit): Promise<Response> {
  await new Promise((resolve) => setTimeout(resolve, 60));
  const parsed = new URL(url, "http://mock.invalid");
  const method = init.method ?? "GET";
  let body: unknown;
  let result: { status: number; body: unknown };
  try {
    body = typeof init.body === "string" ? JSON.parse(init.body) : undefined;
    result = route(method, parsed.pathname, parsed.searchParams, body);
  } catch (error) {
    const status = error instanceof HttpError ? error.status : 400;
    const message = error instanceof Error ? error.message : String(error);
    result = { status, body: { error: message } };
  }
  // Round-trip through JSON so the app never shares objects with the fixtures.
  return new Response(JSON.stringify(result.body), {
    status: result.status,
    headers: { "Content-Type": "application/json" },
  });
}
