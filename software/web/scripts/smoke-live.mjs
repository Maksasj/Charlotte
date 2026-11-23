// Browser smoke test against a REAL `charlotte serve` that also serves the
// built app (web/dist) at /. No mock: every view is driven for real and what
// is on screen is compared with what the HTTP API returns.
//
//   npm run build && node scripts/smoke-live.mjs [base-url]
//   (default base URL: http://127.0.0.1:7899)
//
// IT CHANGES THE SERVER'S STATE: it holds and closes conversations, edits
// knowledge items (and restores them), runs learning jobs over
// data/processed/pretrain-80-per-shape.jsonl and rejects/promotes the candidates they
// produce. Point it at a scratch vault only.
//
// Fails on any uncaught exception, unhandled promise rejection or
// console.error, and on any failed network request it did not provoke itself.

import { readFileSync } from "node:fs";
import { join } from "node:path";
import { fail, openPage, root, run, sleep } from "./harness.mjs";

const BASE = (process.argv[2] ?? "http://127.0.0.1:7899").replace(/\/$/, "");
const CORPUS_ON_SERVER = "data/processed/pretrain-80-per-shape.jsonl";
const CORPUS_FILE = join(root, "..", "..", CORPUS_ON_SERVER);
const KINDS = ["pattern", "atom", "procedure", "intent", "slot", "records", "move", "lexicon", "phrases"];

const enc = encodeURIComponent;
const pct = (v) => (typeof v === "number" && Number.isFinite(v) ? `${Math.round(v * 1000) / 10}%` : "—");
const scalar = (v) => (typeof v === "string" ? v : JSON.stringify(v));
const same = (a, b) => JSON.stringify(a) === JSON.stringify(b);

async function api(method, path, body) {
  const res = await fetch(BASE + path, {
    method,
    headers: body === undefined ? {} : { "Content-Type": "application/json" },
    body: body === undefined ? undefined : JSON.stringify(body),
  });
  const text = await res.text();
  let json = null;
  try {
    json = JSON.parse(text);
  } catch {
    // not JSON
  }
  return { status: res.status, json, text };
}
async function get(path) {
  const res = await api("GET", path);
  if (res.status !== 200) throw new Error(`GET ${path} → ${res.status} ${res.text}`);
  return res.json;
}

/** Retries `fn` (which throws on a mismatch) until it passes or time runs out. */
async function eventually(fn, timeout = 6000) {
  const start = Date.now();
  for (;;) {
    try {
      return await fn();
    } catch (error) {
      if (Date.now() - start > timeout) throw error;
      await sleep(100);
    }
  }
}
function need(text, wanted, where) {
  for (const want of wanted) {
    if (!text.includes(want)) throw new Error(`${where} lacks ${JSON.stringify(want)}; it reads: ${JSON.stringify(text.slice(0, 700))}`);
  }
}
function equal(actual, expected, what) {
  if (!same(actual, expected)) {
    throw new Error(`${what}: on screen ${JSON.stringify(actual)}, from the API ${JSON.stringify(expected)}`);
  }
}

await run(async () => {
  const health = await api("GET", "/v1/health");
  if (health.status !== 200) throw new Error(`no engine at ${BASE}: GET /v1/health → ${health.status}`);
  console.log(`engine ${health.json.version} at ${BASE}, vault ${health.json.vault}`);

  const page = await openPage();
  const { evaluate, waitFor, has, goto, hash, setFiles, check, networkErrors } = page;

  // Failed requests this script provokes on purpose: [status, path].
  const provoked = [];

  await goto(`${BASE}/`);
  // Record every request the app makes, to check paths and bodies.
  await evaluate(`(() => {
    window.__calls = [];
    const original = window.fetch;
    window.fetch = (url, init) => {
      window.__calls.push({ url: String(url), method: (init && init.method) || 'GET', body: (init && init.body) || null });
      return original(url, init);
    };
    return true;
  })()`);
  const lastCall = async (method, pathStart) => {
    const calls = await evaluate("window.__calls");
    return calls.filter((c) => c.method === method && c.url.startsWith(pathStart)).pop() ?? null;
  };

  // ---------------------------------------------------------------- trace

  /** Compares the trace panel with a Trace from the API, field by field. */
  const expectTrace = async (trace, replyText) => {
    const shown = await evaluate(`({
      reply: __t.text('.trace-pane .traced-reply'),
      notes: __t.texts('.trace-pane .notes li'),
      requests: __t.all('.trace-pane .request').map((r) => ({
        text: r.textContent,
        layers: [...r.querySelectorAll('.layer')].map((l) => l.textContent),
        steps: [...r.querySelectorAll('.step')].map((s) => s.textContent),
        openFeatures: [...r.querySelectorAll('details.features')].filter((d) => d.open).length,
        hrefs: [...r.querySelectorAll('.layer-3 dl a')].map((a) => a.getAttribute('href')),
      })),
    })`);
    equal(shown.reply, replyText, "traced reply");
    equal(shown.requests.length, trace.requests.length, "number of requests in the trace");
    equal(shown.notes, trace.notes, "trace notes");
    trace.requests.forEach((req, i) => {
      const dom = shown.requests[i];
      const where = `request ${i + 1}`;
      need(dom.text, [`Request ${i + 1} of ${trace.requests.length}`], where);
      equal(dom.layers.length, 4, `${where}: layers`);
      const [understand, track, decide, realize] = dom.layers;

      need(understand, ["Understand", `Clause${req.clause}`, `User act${req.user_act ?? "none"}`], `${where} Understand`);
      if (req.intent !== null) need(understand, [`Intent${req.intent}`], `${where} Understand`);
      else if (req.frame !== null) {
        need(understand, [`Intentnone in this clause — it continues frame #${req.frame.id} (${req.frame.intent})`], `${where} Understand`);
      } else need(understand, ["Intentnot understood"], `${where} Understand`);
      if ((req.intent !== null || req.frame !== null) && dom.text.includes("not understood")) {
        throw new Error(`${where} is marked "not understood" although it has an intent or a frame`);
      }
      need(understand, req.matched.map((m) => `${m.phrase} → ${m.kind} ${m.target}`), `${where} Understand`);
      if (req.matched.length === 0) need(understand, ["Matched phrasesnone"], `${where} Understand`);
      need(understand, Object.entries(req.facts).map(([k, v]) => `${k} = ${scalar(v)}`), `${where} Understand`);
      if (Object.keys(req.facts).length === 0) need(understand, ["Facts readnone"], `${where} Understand`);

      if (req.frame === null) need(track, ["Track", "no frame — the request was not understood"], `${where} Track`);
      else {
        const f = req.frame;
        need(
          track,
          [
            "Track",
            `Frame#${f.id}`,
            `Intent${f.intent}`,
            `Facts${Object.entries(f.facts).map(([k, v]) => `${k} = ${scalar(v)}`).join("") || "none"}`,
            `Agent acts${f.agent_acts.join("") || "none"}`,
            `User acts${f.user_acts.join("") || "none"}`,
            `Last asked${f.last_asked ?? "nothing"}`,
            `Standing decision${f.standing ?? "none"}`,
            `Fetched / found${f.fetched} / ${f.found}`,
            `Steps done${f.steps_done}`,
          ],
          `${where} Track`,
        );
      }

      need(
        decide,
        [
          "Decide",
          `Request kind${req.kind}`,
          `Pattern${req.pattern ?? "—"}`,
          `Atom${req.atom ?? "—"}`,
          `Alternative${req.alternative ?? "—"}`,
        ],
        `${where} Decide`,
      );
      equal(
        dom.hrefs,
        [req.pattern, req.atom, req.alternative].filter((id) => id !== null).map((id) => `#/knowledge/${enc(id)}`),
        `${where}: knowledge links`,
      );
      equal(dom.steps.length, req.steps.length, `${where}: number of steps`);
      equal(dom.openFeatures, 0, `${where}: feature lists open by default`);
      req.steps.forEach((step, n) => {
        const text = dom.steps[n];
        const at = `${where} step ${n + 1}`;
        const act = step.act.arg === undefined ? step.act.op : `${step.act.op}(${step.act.arg})`;
        need(
          text,
          [
            `Step ${n + 1}`,
            step.rule === null
              ? "Ruleno rule fired"
              : `Rulerule #${step.rule.index} firedwhen ${step.rule.when.join("")}do ${step.rule.do}`,
            `Act${act} op ${step.act.op}, arg ${step.act.arg ?? "none"}`,
            `State features that were true (${step.features.length})${step.features.join("") || "none"}`,
          ],
          at,
        );
        if (text.includes("null") || text.includes("undefined")) throw new Error(`${at} shows a raw null/undefined: ${text}`);
      });
      need(realize, ["Realize", req.text], `${where} Realize`);
    });
  };

  const messageCount = () => evaluate("__t.all('.transcript .message').length");

  const newConversation = async () => {
    const before = await evaluate("location.hash");
    await evaluate("__t.click('.conversations button', 'New conversation')");
    await waitFor(`location.hash.startsWith('#/chat/') && location.hash !== ${JSON.stringify(before)}`);
    await waitFor(has(".transcript", "No messages yet"));
    await waitFor("!document.querySelector('.composer input').disabled");
    const id = decodeURIComponent((await evaluate("location.hash")).slice("#/chat/".length));
    const summary = (await get("/v1/conversations")).find((c) => c.id === id);
    if (!summary) throw new Error(`conversation ${id} is not in GET /v1/conversations`);
    equal(summary.last, null, "last of a new conversation");
    await waitFor(has(".conversation-list a.selected", "(no messages)"));
    return id;
  };

  /** Types a message, sends it with Enter (form submit) and checks the reply and its trace. */
  const say = async (id, text) => {
    const before = await messageCount();
    await evaluate(`__t.set('.composer input', ${JSON.stringify(text)})`);
    await evaluate("document.querySelector('.composer').requestSubmit(), true");
    await waitFor(`__t.all('.transcript .message').length === ${before + 2}`);
    await waitFor("!document.querySelector('.composer input').disabled && document.querySelector('.composer input').value === ''");
    const sent = await lastCall("POST", `/v1/conversations/${enc(id)}/messages`);
    equal(sent && JSON.parse(sent.body), { text }, "body of POST messages");
    const conversation = await get(`/v1/conversations/${enc(id)}`);
    const reply = conversation.messages[conversation.messages.length - 1];
    equal(conversation.messages.length, before + 2, "messages in the conversation");
    equal(await evaluate("__t.texts('.transcript .message .text')"), conversation.messages.map((m) => m.text), "transcript");
    await eventually(() => expectTrace(reply.trace, reply.text));
    await waitFor(has(".trace-pane .pane-head", "latest reply"));
    return reply;
  };

  const closeAs = async (id, label, verdict) => {
    await evaluate(`__t.click('.transcript .toolbar button', ${JSON.stringify(label)})`);
    await waitFor("!!document.querySelector('.closed-summary')");
    const sent = await lastCall("POST", `/v1/conversations/${enc(id)}/close`);
    equal(sent && JSON.parse(sent.body), { verdict }, "body of POST close");
    await waitFor(has(".closed-summary", `closed with verdict ${verdict}`));
    await waitFor(has(".closed-summary", "Trust changes"));
    const rows = await evaluate("__t.all('.closed-summary tbody tr').map((r) => [...r.cells].map((c) => c.textContent))");
    if (rows.length === 0) throw new Error("no trust changes are listed");
    for (const [itemId, before, arrow, after] of rows) {
      if (!/^\d+(\.\d)?%$/.test(before) || arrow !== "→" || !/^\d+(\.\d)?%$/.test(after)) {
        throw new Error(`odd trust change row ${JSON.stringify([itemId, before, arrow, after])}`);
      }
      const item = await get(`/v1/knowledge/${enc(itemId)}`);
      equal(after, pct(item.trust), `trust of ${itemId} after closing`);
    }
    const conversation = await get(`/v1/conversations/${enc(id)}`);
    equal([conversation.closed, conversation.verdict], [true, verdict], "closed state");
    await waitFor("document.querySelector('.composer input').disabled");
    await waitFor("!__t.byText('.transcript .toolbar button', 'Resolved')");
    await waitFor(has(".conversation-list a.selected", verdict));
    return rows;
  };

  // ----------------------------------------------------------------- chat

  console.log("chat:");
  await check("the app loads from the engine, without mock data and without the unreachable banner", async () => {
    await waitFor("location.hash.startsWith('#/chat')");
    await waitFor("__t.all('.conversation-list li').length > 0 || !!__t.byText('.conversations', 'No conversations yet')");
    if (await evaluate(has(".topbar", "mock data"))) throw new Error("mock mode is on");
    if (await evaluate("!!document.querySelector('.banner')")) throw new Error("the unreachable banner is shown");
    const list = await get("/v1/conversations");
    await eventually(async () =>
      equal(
        await evaluate("__t.all('.conversation-list li .mono').map((e) => e.textContent)"),
        list.map((c) => c.id),
        "conversation list",
      ),
    );
  });

  await check("a rule check over three turns; an earlier reply can be selected; close as Resolved", async () => {
    const id = await newConversation();
    const first = await say(id, "I want a refund for order 7741");
    equal([first.trace.requests[0].kind, first.trace.requests[0].steps[0].act.op], ["gated", "ask"], "first reply");
    const second = await say(id, "5 days ago");
    equal(second.trace.requests[0].intent, null, "intent of a continuing clause (as sent by the engine)");
    const third = await say(id, "yes");
    equal(third.trace.requests[0].steps[0].act, { op: "grant" }, "third act");
    await waitFor(has(".trace-pane", "Standing decisiongrant"));

    await evaluate("__t.all('button.message.agent')[0].click(), true");
    await eventually(() => expectTrace(first.trace, first.text));
    await waitFor(has(".trace-pane .pane-head", "selected reply"));
    await evaluate("__t.all('button.message.agent')[2].click(), true");
    await eventually(() => expectTrace(third.trace, third.text));

    const rows = await closeAs(id, "Resolved", "resolved");
    need(rows.map((r) => r[0]).join(" "), ["check-a-rule", "refund-order-within-30-days"], "trust changes");
  });

  await check("two requests in one message, one with a lookup step followed by a report; close as Partial", async () => {
    const id = await newConversation();
    const reply = await say(id, "cancel my subscription 12 bought 5 days ago, also where is my order 5150?");
    equal(reply.trace.requests.length, 2, "requests in the reply");
    equal(reply.trace.requests[1].steps.map((s) => s.act.op), ["lookup", "report"], "steps of the lookup request");
    equal(await evaluate("__t.all('.trace-pane .request').length"), 2, "request cards");
    equal(await evaluate("__t.all('.trace-pane .request')[1].querySelectorAll('.step').length"), 2, "steps shown");
    await closeAs(id, "Partial", "partial");
  });

  await check("a procedure: instruct, done, didn't work; close as Failed", async () => {
    const id = await newConversation();
    const a = await say(id, "I forgot my password");
    const b = await say(id, "done");
    const c = await say(id, "didn't work");
    equal([a, b, c].map((r) => r.trace.requests[0].kind), ["procedure", "procedure", "procedure"], "kinds");
    equal([a, b, c].map((r) => r.trace.requests[0].steps[0].act.op), ["instruct", "instruct", "repeat"], "ops");
    await closeAs(id, "Failed", "failed");
  });

  await check("a request that is not understood, then a cached reply (move)", async () => {
    const id = await newConversation();
    const unknown = await say(id, "do you ship to Norway?");
    const req = unknown.trace.requests[0];
    equal([req.intent, req.frame, req.kind, req.steps.length, req.steps[0].rule, req.steps[0].act], [null, null, "unknown", 1, null, { op: "teach_me" }], "not-understood trace (as sent by the engine)");
    for (const text of ["not understood", "no frame — the request was not understood", "no rule fired", "teach_me"]) {
      await waitFor(has(".trace-pane", text), 1000);
    }
    const move = await say(id, "thanks");
    equal([move.trace.requests[0].kind, move.trace.requests[0].steps[0].act.op], ["move", "say"], "move trace");
    await waitFor(has(".trace-pane", "Request kindmove"), 1000);
    await waitFor(has(".trace-pane", "no rule fired"), 1000);
  });

  await check("an unknown conversation id shows the engine's error", async () => {
    provoked.push([404, "/v1/conversations/no-such-conversation"]);
    await hash("#/chat/no-such-conversation");
    const res = await api("GET", "/v1/conversations/no-such-conversation");
    await waitFor(has(".transcript .notice.error", res.json.error));
  });

  // ------------------------------------------------------------ knowledge

  const tableRows = () => evaluate("__t.all('table.table tbody tr').map((r) => [...r.cells].map((c) => c.textContent))");
  const summaryRow = (s) => [s.id, s.kind, s.status, pct(s.trust), s.title, s.path];
  const expectList = async (query) => {
    const expected = (await get(`/v1/knowledge${query}`)).map(summaryRow);
    await eventually(async () => {
      if (expected.length === 0) {
        if (!(await evaluate(has(".pane", "No items match these filters.")))) throw new Error(`no empty message for ${query}`);
        equal(await tableRows(), [], `rows for ${query}`);
      } else equal(await tableRows(), expected, `rows for ${query || "no filter"}`);
    });
    return expected;
  };
  /** Compares the item panel with GET /v1/knowledge/{id}. */
  const expectItem = async (id) => {
    const item = await get(`/v1/knowledge/${enc(id)}`);
    await eventually(async () => {
      equal(await evaluate("__t.text('.item h2')"), id, "item heading");
      need(await evaluate("__t.text('.item .item-meta')"), [item.kind, item.status, `trust ${pct(item.trust)}`, item.path], "item meta");
      equal(await evaluate("__t.text('.item pre.code')"), JSON.stringify(item.front, null, 2), `front matter of ${id}`);
      equal(await evaluate("document.querySelector('.item pre.body') ? __t.text('.item pre.body') : ''"), item.body, `body of ${id}`);
      equal(await evaluate("__t.texts('.item ul.inline a')"), [...item.links, ...item.backlinks], `links and backlinks of ${id}`);
    });
    return item;
  };
  const kindSelect = ".filters label:nth-of-type(1) select";
  const statusSelect = ".filters label:nth-of-type(2) select";
  const openItem = async (view, id) => {
    await hash(`#/${view}/${enc(id)}`);
    return expectItem(id);
  };
  /** Opens the editor of the item on screen, replaces what is given and presses Save. */
  const edit = async ({ front, body }) => {
    await evaluate("__t.click('.item .toolbar button', 'Edit')");
    await waitFor("!!document.querySelector('#edit-front')");
    if (front !== undefined) await evaluate(`__t.set('#edit-front', ${JSON.stringify(front)})`);
    if (body !== undefined) await evaluate(`__t.set('#edit-body', ${JSON.stringify(body)})`);
    await evaluate("__t.click('.edit button', 'Save')");
  };

  console.log("knowledge:");
  let everything = [];
  await check("the table lists every item as the API returns it", async () => {
    await hash("#/knowledge");
    everything = await expectList("");
    if (everything.length === 0) throw new Error("the vault is empty");
  });

  for (const kind of KINDS) {
    await check(`filter by kind ${kind}, open one such item`, async () => {
      await evaluate(`__t.set(${JSON.stringify(kindSelect)}, ${JSON.stringify(kind)})`);
      const rows = await expectList(`?kind=${kind}`);
      if (rows.length > 0) {
        await evaluate("__t.all('table.table tbody tr')[0].click(), true");
        await waitFor(`location.hash === ${JSON.stringify(`#/knowledge/${enc(rows[0][0])}`)}`);
        const item = await expectItem(rows[0][0]);
        equal(item.kind, kind, "kind of the opened item");
        await waitFor("!!document.querySelector('table.table tbody tr.selected')");
      } else console.log(`        (the vault has no ${kind} items; the empty message was checked)`);
    });
  }

  await check("filter by status, combined filters, search", async () => {
    await evaluate(`__t.set(${JSON.stringify(kindSelect)}, '')`);
    for (const status of ["trusted", "candidate", "retired"]) {
      await evaluate(`__t.set(${JSON.stringify(statusSelect)}, ${JSON.stringify(status)})`);
      await expectList(`?status=${status}`);
    }
    await evaluate(`__t.set(${JSON.stringify(statusSelect)}, 'trusted')`);
    await evaluate(`__t.set(${JSON.stringify(kindSelect)}, 'atom')`);
    await evaluate("__t.set('.filters input[type=search]', 'refund')");
    const found = await expectList("?kind=atom&status=trusted&q=refund");
    if (found.length === 0) throw new Error("searching atoms for 'refund' found nothing");
    const query = await lastCall("GET", "/v1/knowledge?");
    equal(query && query.url, "/v1/knowledge?kind=atom&status=trusted&q=refund", "query string");
    await evaluate(`__t.set(${JSON.stringify(statusSelect)}, '')`);
    await evaluate(`__t.set(${JSON.stringify(kindSelect)}, '')`);
    await expectList("?q=refund");
    await evaluate("__t.set('.filters input[type=search]', 'Refund approved')");
    await expectList(`?q=${enc("Refund approved")}`);
    await evaluate("__t.set('.filters input[type=search]', '')");
    await expectList("");
  });

  await check("follow a link, then a backlink", async () => {
    const withLinks = await openItem("knowledge", "refund-order-within-30-days");
    if (withLinks.links.length === 0) throw new Error("refund-order-within-30-days has no links");
    const target = withLinks.links[0];
    await evaluate(`__t.click('.item ul.inline a', ${JSON.stringify(target)})`);
    await waitFor(`location.hash === ${JSON.stringify(`#/knowledge/${enc(target)}`)}`);
    const linked = await expectItem(target);
    if (linked.backlinks.length === 0) throw new Error(`${target} has no backlinks`);
    const back = linked.backlinks[linked.backlinks.length - 1];
    await evaluate(`(() => { const links = __t.all('.item ul.inline a'); links[links.length - 1].click(); return true; })()`);
    await waitFor(`location.hash === ${JSON.stringify(`#/knowledge/${enc(back)}`)}`);
    await expectItem(back);
  });

  await check("edit a move's body, save, and change it back", async () => {
    const original = await openItem("knowledge", "bye-reply");
    const changed = `${original.body} (edited by smoke-live)`;
    await edit({ body: changed });
    await waitFor("!document.querySelector('#edit-front')");
    const sent = await lastCall("PUT", "/v1/knowledge/bye-reply");
    equal(sent && JSON.parse(sent.body), { front: original.front, body: changed }, "body of PUT");
    equal((await expectItem("bye-reply")).body, changed, "saved body");
    await edit({ body: original.body });
    await waitFor("!document.querySelector('#edit-front')");
    equal(await expectItem("bye-reply"), original, "item after changing it back");
  });

  await check("invalid JSON is refused locally; an atom without `intent` gets the engine's 400", async () => {
    const original = await openItem("knowledge", "cancel-activated-pause");
    const putsBefore = (await evaluate("window.__calls")).filter((c) => c.method === "PUT").length;
    await edit({ front: "{ not json" });
    await waitFor(has(".edit .notice.error", "Front matter is not valid JSON"));
    equal((await evaluate("window.__calls")).filter((c) => c.method === "PUT").length, putsBefore, "PUT requests sent for invalid JSON");

    const front = { ...original.front };
    delete front.intent;
    const expected = await api("PUT", "/v1/knowledge/cancel-activated-pause", { front, body: original.body });
    equal(expected.status, 400, "status of a PUT without intent");
    provoked.push([400, "/v1/knowledge/cancel-activated-pause"]);
    await evaluate(`__t.set('#edit-front', ${JSON.stringify(JSON.stringify(front, null, 2))})`);
    await evaluate("__t.click('.edit button', 'Save')");
    await waitFor(has(".edit .notice.error", expected.json.error));
    need(expected.json.error, ["intent"], "the engine's message");
    if (!(await evaluate("!!document.querySelector('#edit-front')"))) throw new Error("the editor closed after a failed save");
    await evaluate("__t.click('.edit button', 'Cancel')");
    equal(await expectItem("cancel-activated-pause"), original, "the atom after the refused save");
  });

  await check("an unknown item id shows the engine's error", async () => {
    provoked.push([404, "/v1/knowledge/no-such-item"]);
    await hash("#/knowledge/no-such-item");
    const res = await api("GET", "/v1/knowledge/no-such-item");
    await waitFor(has(".notice.error", res.json.error));
  });

  // ---------------------------------------------------- learn + candidates

  const expectReport = async (jobId) => {
    const job = await get(`/v1/learn/${enc(jobId)}`);
    equal(job.state, "done", `state of ${jobId}`);
    const r = job.report;
    await eventually(async () => {
      const text = await evaluate("__t.text('.job')");
      need(
        text,
        [
          `Job ${job.id}`,
          `done`,
          `source ${job.source}`,
          `Read${r.conversations.read}`,
          `Skipped${r.conversations.skipped}`,
          `Train${r.conversations.train}`,
          `Held out${r.conversations.held_out}`,
          ...Object.entries(r.labels).map(([k, v]) => `${k}${pct(v)}`),
          `Held-out score${pct(r.held_out.before)} → ${pct(r.held_out.after)}`,
          `Understanding${pct(r.understanding.before)} → ${pct(r.understanding.after)}`,
          `Candidates (${r.candidates.length})`,
          `Took ${r.seconds} seconds.`,
          ...(r.candidates.length === 0 ? ["The job proposed no candidates."] : []),
          ...(r.mistakes.length === 0 ? ["None reported."] : []),
        ],
        `report of ${jobId}`,
      );
      equal(
        await evaluate("__t.all('.report .table-wrap tbody tr').map((r) => [...r.cells].map((c) => c.textContent))"),
        r.candidates.map((c) => [c.id, c.kind, String(c.support), pct(c.validated), c.promoted ? "yes" : "no", c.summary]),
        "candidates in the report",
      );
      equal(
        await evaluate("__t.all('.report table.compact tbody tr').map((r) => [...r.cells].map((c) => c.textContent))"),
        r.mistakes.map((m) => [m.what, String(m.count)]),
        "mistakes in the report",
      );
      equal(await evaluate("__t.text('.stages li.current')"), `${job.stage} (current stage)`, "stage");
    });
    return job;
  };
  const expectJobs = async () => {
    const jobs = await get("/v1/learn");
    await eventually(async () =>
      equal(
        await evaluate("__t.all('table.selectable tbody tr').map((r) => [...r.cells].map((c) => c.textContent))"),
        jobs.map((j) => [j.id, j.state, j.stage, j.source]),
        "jobs table",
      ),
    );
    return jobs;
  };
  /** Presses Start, waits for the new job to be selected and finished, returns its id. */
  const startJob = async () => {
    const before = await evaluate("location.hash");
    await evaluate("__t.click('.learn-form button', 'Start learning')");
    await waitFor(`location.hash.startsWith('#/learn/') && location.hash !== ${JSON.stringify(before)}`);
    const id = decodeURIComponent((await evaluate("location.hash")).slice("#/learn/".length));
    await waitFor(`__t.text('.job h2') === ${JSON.stringify(`Job ${id}`)} && !!__t.byText('.job .item-meta .badge', 'done')`, 15000);
    return id;
  };

  const candidateIds = async () => (await get("/v1/knowledge?status=candidate")).map((c) => c.id);
  const expectCandidates = async () => {
    const list = await get("/v1/knowledge?status=candidate");
    const expected = [];
    for (const s of list) {
      const item = await get(`/v1/knowledge/${enc(s.id)}`);
      const support = item.front.support;
      expected.push([s.id, s.kind, s.title, support === undefined || support === null ? "—" : scalar(support), pct(item.front.validated)]);
    }
    await eventually(async () => {
      if (expected.length === 0) {
        if (!(await evaluate(has(".empty", "No candidates are waiting for review")))) throw new Error("no empty-state message");
        equal(await tableRows(), [], "candidate rows");
      } else equal(await tableRows(), expected, "candidate rows");
    });
    return expected;
  };

  // A learning job only writes a candidate when what it learns differs from
  // the trusted pattern, so two trusted patterns are cut down first (through
  // the editor) and restored at the end whatever happens.
  const PATTERNS = ["walk-through-steps", "look-up-a-record"];
  const originals = {};
  for (const id of PATTERNS) originals[id] = await get(`/v1/knowledge/${id}`);
  const preexisting = await candidateIds();
  let produced = [];

  try {
    console.log("candidates and learn:");
    await check("the review queue as the API has it before learning", async () => {
      await hash("#/candidates");
      await expectCandidates();
      if (preexisting.length === 0) await waitFor(has(".empty", "No candidates are waiting for review"));
    });

    await check("cut two trusted patterns down through the editor", async () => {
      for (const id of PATTERNS) {
        const item = await openItem("knowledge", id);
        const front = { ...item.front, rules: item.front.rules.slice(0, 1) };
        await edit({ front: JSON.stringify(front, null, 2) });
        await waitFor("!document.querySelector('#edit-front')");
        equal((await expectItem(id)).front.rules.length, 1, `rules left in ${id}`);
      }
    });

    await check("a pattern's fall-through rule shows as `otherwise` in the trace", async () => {
      await hash("#/chat");
      await waitFor("location.hash.startsWith('#/chat/')");
      const id = await newConversation();
      const reply = await say(id, "where is my order 5150?");
      const rule = reply.trace.requests[0].steps[0].rule;
      equal(rule && rule.when, ["otherwise"], "rule.when of the fall-through rule (as sent by the engine)");
      await waitFor(has(".trace-pane .step", `rule #${rule.index} firedwhen otherwisedo ${rule.do}`), 1000);
    });

    await check("learn: the jobs table and the newest job's report match the API", async () => {
      await hash("#/learn");
      const jobs = await expectJobs();
      if (jobs.length > 0) {
        await waitFor(`__t.text('.job h2') === ${JSON.stringify(`Job ${jobs[0].id}`)}`);
        if (jobs[0].state === "done") await expectReport(jobs[0].id);
      }
    });

    await check("learn: start a job from the corpus path with auto promote off", async () => {
      await evaluate(`__t.set('.learn-form input[type=text]', ${JSON.stringify(CORPUS_ON_SERVER)})`);
      await evaluate("__t.click('.learn-form label.checkbox input')");
      equal(await evaluate("document.querySelector('.learn-form label.checkbox input').checked"), false, "auto promote box");
      const id = await startJob();
      const sent = await lastCall("POST", "/v1/learn");
      equal(sent && JSON.parse(sent.body), { path: CORPUS_ON_SERVER, auto_promote: false }, "body of POST /v1/learn");
      const job = await expectReport(id);
      await expectJobs();
      produced = (await candidateIds()).filter((c) => !preexisting.includes(c));
      equal(produced.length, PATTERNS.length, "candidates produced by the job");
      need(job.report.candidates.map((c) => c.id).join(" "), produced, "report candidates");
    });

    await check("learn: the file picker fills the text box; a pasted batch runs and reports", async () => {
      const corpus = readFileSync(CORPUS_FILE, "utf8");
      await evaluate("__t.all('.learn-form input[type=radio]')[1].click(), true");
      await waitFor("!document.querySelector('.learn-form textarea').disabled");
      await setFiles(".learn-form input[type=file]", [CORPUS_FILE]);
      await waitFor(`document.querySelector('.learn-form textarea').value.length === ${corpus.length}`);
      const pasted = corpus.split("\n").slice(0, 5).join("\n");
      await evaluate(`__t.set('.learn-form textarea', ${JSON.stringify(pasted)})`);
      const id = await startJob();
      const sent = await lastCall("POST", "/v1/learn");
      equal(sent && JSON.parse(sent.body), { jsonl: pasted, auto_promote: false }, "body of POST /v1/learn");
      const job = await expectReport(id);
      equal(job.source, "uploaded", "source of an uploaded job");
      equal(job.report.conversations.read, 5, "conversations read");
      await expectJobs();
    });

    await check("learn: a batch the engine refuses shows its message", async () => {
      const refused = await api("POST", "/v1/learn", { jsonl: "this is not JSON lines", auto_promote: false });
      equal(refused.status, 400, "status for an unreadable batch");
      provoked.push([400, "/v1/learn"]);
      await evaluate("__t.set('.learn-form textarea', 'this is not JSON lines')");
      await evaluate("__t.click('.learn-form button', 'Start learning')");
      await waitFor(has(".learn-form .notice.error", refused.json.error));
    });

    await check("candidates: the queue shows support and validated; open one", async () => {
      await hash("#/candidates");
      const rows = await expectCandidates();
      const mine = rows.filter((r) => produced.includes(r[0]));
      equal(mine.length, produced.length, "produced candidates listed");
      for (const row of mine) {
        if (!/^\d+$/.test(row[3]) || !/^\d+(\.\d)?%$/.test(row[4])) throw new Error(`support/validated missing in ${JSON.stringify(row)}`);
      }
      await evaluate(`__t.byText('table.table tbody tr', ${JSON.stringify(produced[0])}).click(), true`);
      await waitFor(`location.hash === ${JSON.stringify(`#/candidates/${enc(produced[0])}`)}`);
      const item = await expectItem(produced[0]);
      equal(item.status, "candidate", "status of the opened candidate");
    });

    await check("candidates: reject one", async () => {
      const id = produced[0];
      await openItem("candidates", id);
      await evaluate("__t.click('.item .toolbar button', 'Reject')");
      await waitFor(has(".notice.ok", `Rejected ${id}`));
      equal((await api("GET", `/v1/knowledge/${enc(id)}`)).status, 404, "the rejected item is gone");
      equal((await candidateIds()).includes(id), false, "still in the queue");
      await expectCandidates();
    });

    await check("candidates: promote one, then the queue is as the API has it (empty if none are left)", async () => {
      const id = produced[1];
      await openItem("candidates", id);
      await evaluate("__t.click('.item .toolbar button', 'Promote')");
      await waitFor(has(".notice.ok", "Promoted "));
      const promotedId = await evaluate("__t.text('.notice.ok a')");
      const promoted = await get(`/v1/knowledge/${enc(promotedId)}`);
      equal(promoted.status, "trusted", `status of ${promotedId} after promotion`);
      equal((await candidateIds()).includes(id), false, "still in the queue");
      const left = await expectCandidates();
      if (left.length === 0) await waitFor(has(".empty", "No candidates are waiting for review"));
      await evaluate("__t.click('.notice.ok a')");
      await waitFor(`location.hash === ${JSON.stringify(`#/knowledge/${enc(promotedId)}`)}`);
      await expectItem(promotedId);
    });
  } finally {
    // Put the two patterns back, and drop candidates this run left behind.
    for (const id of PATTERNS) {
      const now = await api("GET", `/v1/knowledge/${id}`);
      if (now.status !== 200 || !same(now.json.front.rules, originals[id].front.rules)) {
        const put = await api("PUT", `/v1/knowledge/${id}`, { front: originals[id].front, body: originals[id].body });
        if (put.status !== 200) fail(`could not restore ${id}: ${put.status} ${put.text}`);
      }
    }
    for (const id of (await candidateIds()).filter((c) => !preexisting.includes(c))) {
      await api("POST", `/v1/knowledge/${enc(id)}/reject`);
    }
  }

  // --------------------------------------------------------------- status

  console.log("status:");
  await check("status matches GET /v1/health; Refresh reloads it", async () => {
    await hash("#/status");
    const h = await get("/v1/health");
    await eventually(async () => {
      const text = await evaluate("__t.text('main .pane')");
      need(text, [`Engine version${h.version}`, `Vault${h.vault}`, `Candidates${h.candidates}`, `Conversations${h.conversations}`], "status");
      equal(await tableRows(), Object.entries(h.items).map(([k, n]) => [k, String(n)]), "trusted items per kind");
    });
    const before = (await evaluate("window.__calls")).filter((c) => c.url === "/v1/health").length;
    await evaluate("__t.click('.pane-head button', 'Refresh')");
    await waitFor(`window.__calls.filter((c) => c.url === '/v1/health').length === ${before + 1}`);
    await waitFor(has(".pane-head button", "Refresh"));
  });

  // -------------------------------------------------------------- overall

  console.log("overall:");
  await check("every request used a relative /v1/ path", async () => {
    const calls = await evaluate("window.__calls");
    const odd = calls.filter((c) => !c.url.startsWith("/v1/"));
    if (odd.length > 0) throw new Error(`requests outside /v1/: ${JSON.stringify(odd.slice(0, 5))}`);
    console.log(`        (${calls.length} requests made by the app)`);
  });
  await check("no failed network request other than the ones provoked on purpose", async () => {
    const unexpected = networkErrors.filter(
      (entry) => !provoked.some(([status, path]) => entry.includes(`status of ${status} `) && entry.trim().endsWith(BASE + path)),
    );
    if (unexpected.length > 0) throw new Error(unexpected.join("\n"));
    console.log(`        (${networkErrors.length} provoked: ${provoked.map(([s, p]) => `${s} ${p}`).join(", ")})`);
  });
  if (page.problems.length > 0) console.log(`  problems recorded:\n    ${page.problems.join("\n    ")}`);

  page.close();
});
