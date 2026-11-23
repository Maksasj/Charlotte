// Browser smoke test against the MOCK engine. Starts the Vite dev server,
// drives a headless Chrome through every view in mock mode (?mock=1), then
// loads the app without the mock and without an engine to check the
// "cannot reach the engine" banner. Fails on any uncaught exception,
// unhandled promise rejection or console.error.
//
// Usage: npm run smoke        (see harness.mjs for the requirements)
// For the same against a real `charlotte serve`, see smoke-live.mjs.

import { fail, openPage, run, startVite } from "./harness.mjs";

await run(async () => {
  const BASE = await startVite(5179);
  const { evaluate, waitFor, has, goto, hash, check, networkErrors, close } = await openPage();

  console.log("mock mode:");
  await goto(`${BASE}/?mock=1`);

  await check("chat: opens the newest conversation and shows the latest trace", async () => {
    await waitFor("location.hash === '#/chat/c2'");
    await waitFor(has(".topbar", "mock data"));
    await waitFor("__t.all('.message').length === 6");
    await waitFor(has(".trace-pane", "Request 1 of 1"));
    for (const text of ["not understood", "no rule fired", "teach_me", "Engine notes", "no lexicon phrase matched"]) {
      await waitFor(has(".trace-pane", text), 1000);
    }
    await waitFor(has(".trace-pane", "no frame — the request was not understood"), 1000);
    await waitFor("__t.all('.trace-pane .layer').length === 4", 1000);
    await waitFor("__t.all('.conversation-list li').length === 2", 1000);
  });

  await check("chat: clicking an earlier reply shows its trace with fired rules and knowledge links", async () => {
    await evaluate("__t.all('button.message.agent')[0].click(), true");
    await waitFor(has(".trace-pane", "Request 1 of 2"));
    for (const text of [
      "Understand", "Track", "Decide", "Realize",
      "send back", "subject = 7741", "days = 5", "rule #1 fired", "ask(p_first_unbound)", "ask(packaging)",
      "rule #0 fired", "lookup(orders)", "rule #2 fired", "State features that were true (7)",
      "Is it still in the original packaging, unopened?", "Last asked",
    ]) {
      await waitFor(has(".trace-pane", text), 1000);
    }
    const links = await evaluate("__t.all('.trace-pane .request a').map((a) => a.getAttribute('href'))");
    for (const want of ["#/knowledge/check-a-rule", "#/knowledge/refund-order-within-30-days", "#/knowledge/refund-order-opened", "#/knowledge/status-lookup"]) {
      if (!links.includes(want)) throw new Error(`missing link ${want} in ${JSON.stringify(links)}`);
    }
    const open = await evaluate("__t.all('.trace-pane details.features').some((d) => d.open)");
    if (open) throw new Error("feature lists should start collapsed");
  });

  await check("chat: a clause that continues a request has no intent but is not marked 'not understood'", async () => {
    await evaluate("__t.all('button.message.agent')[1].click(), true");
    await waitFor(has(".trace-pane", "never opened"));
    await waitFor(has(".trace-pane .layer-1", "none in this clause — it continues frame #1 (refund)"), 1000);
    if (await evaluate(has(".trace-pane", "not understood"))) throw new Error("marked as not understood");
    await waitFor(has(".trace-pane", "rule #2 fired"), 1000);
    await waitFor(has(".trace-pane code.act", "grant"), 1000);
    if (await evaluate("__t.text('.trace-pane code.act') !== 'grant'")) throw new Error("act without arg should read 'grant'");
  });

  await check("chat: sending a message appends the reply and selects its trace", async () => {
    await evaluate("__t.set('.composer input', 'hello there')");
    await evaluate("document.querySelector('.composer').requestSubmit(), true");
    await waitFor("document.querySelector('.composer input').disabled");
    await waitFor("__t.all('.message').length === 8");
    await waitFor(has(".trace-pane", "mock engine: every new message"));
    await waitFor("!document.querySelector('.composer input').disabled && document.querySelector('.composer input').value === ''");
    await waitFor(has(".conversation-list a.selected", "8 turns"));
  });

  await check("chat: closing shows the verdict and trust changes and disables the input", async () => {
    await evaluate("__t.click('.transcript .toolbar button', 'Resolved')");
    await waitFor(has(".closed-summary", "Trust changes"));
    for (const text of ["resolved", "check-a-rule", "84%", "89%", "88%", "93%"]) {
      await waitFor(has(".closed-summary", text), 1000);
    }
    await waitFor("document.querySelector('.composer input').disabled");
    await waitFor("!__t.byText('.transcript .toolbar button', 'Failed')");
    await waitFor(has(".conversation-list a.selected", "resolved"));
  });

  await check("chat: new conversation", async () => {
    await evaluate("__t.click('.conversations button', 'New conversation')");
    await waitFor("location.hash === '#/chat/c3'");
    await waitFor(has(".transcript", "No messages yet"));
    await waitFor("__t.all('.conversation-list li').length === 3");
    await waitFor("!document.querySelector('.composer input').disabled");
  });

  await check("chat: unknown conversation id shows the server's error", async () => {
    await hash("#/chat/nope");
    await waitFor(has(".transcript .notice.error", "no conversation nope"));
  });

  await check("knowledge: list, filters and search", async () => {
    await hash("#/knowledge");
    await waitFor("__t.all('table.table tbody tr').length === 11");
    await evaluate("__t.set('.filters select', 'atom')");
    await waitFor("__t.all('table.table tbody tr').length === 2");
    await evaluate("__t.set('.filters input[type=search]', 'store credit')");
    await waitFor("__t.all('table.table tbody tr').length === 1");
    await waitFor(has("table.table tbody", "refund-order-opened"), 1000);
    await waitFor(has("table.table tbody", "80%"), 1000);
    await evaluate("__t.set('.filters input[type=search]', 'zzz')");
    await waitFor(has(".pane", "No items match these filters."));
    await evaluate("__t.set('.filters input[type=search]', '')");
    await evaluate("__t.set('.filters select', '')");
    await waitFor("__t.all('table.table tbody tr').length === 11");
  });

  await check("knowledge: open an item with a dotted id and follow a link", async () => {
    await evaluate("__t.byText('table.table tbody tr', 'refund.send-back').click(), true");
    await waitFor("location.hash === '#/knowledge/refund.send-back'");
    await waitFor(has(".item pre.code", '"support": 14'));
    await waitFor(has(".item .item-meta", "candidate"), 1000);
    await evaluate("__t.click('.item ul.inline a', 'refund')");
    await waitFor("location.hash === '#/knowledge/refund'");
    await waitFor(has(".item pre.code", "money back"));
    await waitFor(has(".item", "check-a-rule"), 1000);
    await waitFor(has(".item .item-meta", "trust 95%"), 1000);
  });

  await check("knowledge: edit — invalid JSON is caught locally, a 400 shows the server message, a valid save works", async () => {
    await evaluate("__t.click('.item .toolbar button', 'Edit')");
    await waitFor("!!document.querySelector('#edit-front')");
    await evaluate("__t.set('#edit-front', '{ not json')");
    await evaluate("__t.click('.edit button', 'Save')");
    await waitFor(has(".edit .notice.error", "Front matter is not valid JSON"));
    await evaluate("__t.set('#edit-front', '{\"id\": \"refund\"}')");
    await evaluate("__t.click('.edit button', 'Save')");
    await waitFor(has(".edit .notice.error", "note is not valid: front matter needs a known `kind`"));
    await evaluate("__t.set('#edit-front', '{\"id\": \"refund\", \"kind\": \"intent\", \"trust\": 0.5}')");
    await evaluate("__t.set('#edit-body', 'edited body text')");
    await evaluate("__t.click('.edit button', 'Save')");
    await waitFor("!document.querySelector('#edit-front')");
    await waitFor(has(".item pre.body", "edited body text"));
    await waitFor(has(".item .item-meta", "trust 50%"), 1000);
    await waitFor(has("table.table tbody tr.selected", "50%"));
  });

  await check("knowledge: unknown id shows the server's error", async () => {
    await hash("#/knowledge/does.not-exist");
    await waitFor(has(".notice.error", "no knowledge item does.not-exist"));
  });

  await check("candidates: list with support and validated, promote, reject, empty state", async () => {
    await hash("#/candidates");
    await waitFor("__t.all('table.table tbody tr').length === 2");
    const rows = await evaluate("__t.all('table.table tbody tr').map((r) => [...r.cells].map((c) => c.textContent))");
    const expected = [
      ["warranty-claim-v2", "pattern", "warranty-claim-v2", "37", "92%"],
      ["refund.send-back", "phrases", "refund.send-back", "14", "—"],
    ];
    if (JSON.stringify(rows) !== JSON.stringify(expected)) throw new Error(`rows were ${JSON.stringify(rows)}`);
    await evaluate("__t.byText('table.table tbody tr', 'warranty-claim-v2').click(), true");
    await waitFor(has(".item pre.code", '"validated": 0.92'));
    await evaluate("__t.click('.item .toolbar button', 'Promote')");
    await waitFor(has(".notice.ok", "Promoted warranty-claim-v2"));
    await waitFor("__t.all('table.table tbody tr').length === 1");
    await evaluate("__t.byText('table.table tbody tr', 'refund.send-back').click(), true");
    await waitFor("location.hash === '#/candidates/refund.send-back'");
    await waitFor(has(".item pre.code", '"support": 14'));
    await evaluate("__t.click('.item .toolbar button', 'Reject')");
    await waitFor(has(".notice.ok", "Rejected refund.send-back"));
    await waitFor(has(".empty", "No candidates are waiting for review"));
  });

  await check("learn: finished job renders the whole report", async () => {
    await hash("#/learn");
    await waitFor(has(".job", "j1"));
    const text = await evaluate("__t.text('.job')");
    for (const want of [
      "done", "corpus/pretrain-80.jsonl", "Read80", "Skipped1", "Train64", "Held out16",
      "act100%", "intent97.5%", "facts90%", "0% → 94%", "71% → 90%",
      "check-a-rule", "51", "95%", "yes", "5 rules for gated requests", "warranty-claim-v2", "no",
      "teacher deny, engine offer", "2", "Took 0.4 seconds",
    ]) {
      if (!text.includes(want)) throw new Error(`report lacks ${JSON.stringify(want)}`);
    }
  });

  await check("learn: start a job from a path and poll it to completion", async () => {
    await evaluate("__t.set('.learn-form input[type=text]', 'corpus/batch-01.jsonl')");
    await evaluate("__t.click('.learn-form button', 'Start learning')");
    await waitFor("location.hash === '#/learn/j2'");
    await waitFor(has(".job", "Running — checking every second"));
    await waitFor("__t.text('.stages li.current').startsWith('validate')", 8000);
    await waitFor(has(".job", "Took 0.4 seconds"), 10000);
    await waitFor(has("table.selectable tbody tr.selected", "done"), 1000);
  });

  await check("learn: uploaded JSON lines, and a job that fails", async () => {
    await evaluate("__t.all('.learn-form input[type=radio]')[1].click(), true");
    await waitFor("!document.querySelector('.learn-form textarea').disabled");
    await evaluate(`__t.set('.learn-form textarea', '{"turns": [{"role": "user", "text": "hi"}]}')`);
    await evaluate("__t.click('.learn-form label.checkbox input')");
    await evaluate("__t.click('.learn-form button', 'Start learning')");
    await waitFor("location.hash === '#/learn/j3'");
    await waitFor(has("table.selectable tbody tr.selected", "uploaded"));
    await evaluate("__t.all('.learn-form input[type=radio]')[0].click(), true");
    await evaluate("__t.set('.learn-form input[type=text]', 'corpus/missing.jsonl')");
    await evaluate("__t.click('.learn-form button', 'Start learning')");
    await waitFor("location.hash === '#/learn/j4'");
    await waitFor(has(".job .notice.error", "cannot open corpus/missing.jsonl"), 8000);
    await waitFor(has("table.selectable tbody tr.selected", "failed"), 1000);
  });

  await check("status: health", async () => {
    await hash("#/status");
    await waitFor(has(".pane", "0.1.0-mock"));
    const text = await evaluate("__t.text('.pane')");
    for (const want of ["/abs/path/vault", "Candidates0", "Conversations3", "atom2", "pattern3", "Refresh"]) {
      if (!text.includes(want)) throw new Error(`status lacks ${JSON.stringify(want)}`);
    }
    await evaluate("__t.click('.pane-head button', 'Refresh')");
    await waitFor(has(".pane-head button", "Refresh"));
  });

  if (networkErrors.length > 0) {
    fail(`mock mode made network requests that failed:\n      ${networkErrors.join("\n      ")}`);
  }

  console.log("no mock, no engine (requests go to the Vite proxy and fail):");
  networkErrors.length = 0;
  await goto(`${BASE}/`);
  for (const view of ["chat", "knowledge", "candidates", "learn", "status"]) {
    await check(`${view}: banner and a retryable error instead of a crash`, async () => {
      await hash(`#/${view}`);
      await waitFor(has(".banner", "Cannot reach the engine at /v1 — is `charlotte serve` running?"));
      await waitFor(has("main .notice.error", "Cannot reach the engine"));
      await evaluate("__t.click('.banner button', 'Retry')");
      await waitFor(has(".banner button", "Retry"));
      await evaluate("__t.click('main .notice.error button', 'Retry')");
      await waitFor(has("main .notice.error", "Cannot reach the engine"));
      if (!(await evaluate("!!document.querySelector('.topbar nav a.active')"))) throw new Error("navigation is gone");
    });
  }
  console.log(`  (${networkErrors.length} failed network requests were logged by the browser, as expected)`);


  close();
});
