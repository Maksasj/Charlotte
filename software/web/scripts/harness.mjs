// Shared by smoke.mjs and smoke-live.mjs: starts a headless Chrome, drives one
// page over the DevTools protocol and records anything that counts as a
// failure (uncaught exceptions, unhandled promise rejections, console.error).
// Needs Node 22+ (built-in WebSocket) and a local Chrome or Edge
// (CHROME=/path/to/browser to override).

import { spawn } from "node:child_process";
import { existsSync, rmSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

export const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const profile = join(root, ".smoke-profile");

export const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

const children = [];
export const stats = { failures: 0, checks: 0 };

export function fail(message) {
  stats.failures++;
  console.log(`  FAIL  ${message}`);
}

function findBrowser() {
  const candidates = [
    process.env.CHROME,
    "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
    "/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge",
    "/Applications/Chromium.app/Contents/MacOS/Chromium",
    "/usr/bin/google-chrome",
    "/usr/bin/chromium",
    "/usr/bin/chromium-browser",
  ].filter(Boolean);
  const found = candidates.find((p) => existsSync(p));
  if (!found) throw new Error("No Chrome/Edge found; set CHROME=/path/to/browser");
  return found;
}

export async function startVite(port) {
  const base = `http://127.0.0.1:${port}`;
  const child = spawn(
    process.execPath,
    [join(root, "node_modules/vite/bin/vite.js"), "--port", String(port), "--strictPort", "--host", "127.0.0.1"],
    { cwd: root, stdio: ["ignore", "pipe", "pipe"] },
  );
  children.push(child);
  let log = "";
  child.stdout.on("data", (d) => (log += d));
  child.stderr.on("data", (d) => (log += d));
  for (let i = 0; i < 100; i++) {
    if (child.exitCode !== null) throw new Error(`vite exited early:\n${log}`);
    try {
      const res = await fetch(`${base}/`);
      if (res.ok) return base;
    } catch {
      // not up yet
    }
    await sleep(100);
  }
  throw new Error(`vite did not start:\n${log}`);
}

async function startBrowser(browserPath) {
  rmSync(profile, { recursive: true, force: true });
  const child = spawn(
    browserPath,
    [
      "--headless=new",
      "--remote-debugging-port=0",
      `--user-data-dir=${profile}`,
      "--no-first-run",
      "--no-default-browser-check",
      "--disable-extensions",
      "--window-size=1500,1000",
      "about:blank",
    ],
    { stdio: ["ignore", "ignore", "pipe"] },
  );
  children.push(child);
  return await new Promise((resolve, reject) => {
    let err = "";
    const timer = setTimeout(() => reject(new Error(`browser did not start:\n${err}`)), 20000);
    child.stderr.on("data", (d) => {
      err += d;
      const match = /DevTools listening on (ws:\/\/\S+)/.exec(err);
      if (match) {
        clearTimeout(timer);
        resolve(match[1]);
      }
    });
    child.on("exit", () => reject(new Error(`browser exited:\n${err}`)));
  });
}

class Cdp {
  constructor(ws) {
    this.ws = ws;
    this.next = 1;
    this.pending = new Map();
    this.handlers = [];
    ws.addEventListener("message", (event) => {
      const msg = JSON.parse(event.data);
      if (msg.id && this.pending.has(msg.id)) {
        const { resolve, reject } = this.pending.get(msg.id);
        this.pending.delete(msg.id);
        if (msg.error) reject(new Error(msg.error.message));
        else resolve(msg.result);
      } else if (msg.method) {
        for (const handler of this.handlers) handler(msg);
      }
    });
  }
  static async connect(url) {
    const ws = new WebSocket(url);
    await new Promise((resolve, reject) => {
      ws.addEventListener("open", resolve);
      ws.addEventListener("error", () => reject(new Error("cannot connect to the browser")));
    });
    return new Cdp(ws);
  }
  send(method, params = {}, sessionId) {
    const id = this.next++;
    this.ws.send(JSON.stringify({ id, method, params, sessionId }));
    return new Promise((resolve, reject) => this.pending.set(id, { resolve, reject }));
  }
}

// Available in the page as window.__t.
const HELPERS = `
window.__t = {
  all: (sel) => [...document.querySelectorAll(sel)],
  byText: (sel, text) => [...document.querySelectorAll(sel)].find((e) => e.textContent.includes(text)),
  click: (sel, text) => {
    const el = text === undefined ? document.querySelector(sel) : window.__t.byText(sel, text);
    if (!el) throw new Error('no element ' + sel + (text === undefined ? '' : ' with text ' + text));
    if (el.disabled) throw new Error('element is disabled: ' + sel + ' ' + (text ?? ''));
    el.click();
    return true;
  },
  set: (sel, value) => {
    const el = document.querySelector(sel);
    if (!el) throw new Error('no element ' + sel);
    if (el.disabled) throw new Error('element is disabled: ' + sel);
    const proto = el instanceof HTMLTextAreaElement ? HTMLTextAreaElement.prototype
      : el instanceof HTMLSelectElement ? HTMLSelectElement.prototype : HTMLInputElement.prototype;
    Object.getOwnPropertyDescriptor(proto, 'value').set.call(el, value);
    el.dispatchEvent(new Event(el instanceof HTMLSelectElement ? 'change' : 'input', { bubbles: true }));
    return true;
  },
  text: (sel) => (document.querySelector(sel) || { textContent: '' }).textContent,
  texts: (sel) => [...document.querySelectorAll(sel)].map((e) => e.textContent),
};
`;

/** Opens one page and returns the tools to drive it. */
export async function openPage() {
  const browserPath = findBrowser();
  const cdp = await Cdp.connect(await startBrowser(browserPath));
  const { targetId } = await cdp.send("Target.createTarget", { url: "about:blank" });
  const { sessionId } = await cdp.send("Target.attachToTarget", { targetId, flatten: true });
  const send = (method, params) => cdp.send(method, params, sessionId);

  /** Uncaught exceptions, unhandled rejections and console.error calls. */
  const problems = [];
  /** "Failed to load resource" entries the browser logs for non-2xx responses. */
  const networkErrors = [];
  cdp.handlers.push((msg) => {
    if (msg.sessionId !== sessionId) return;
    if (msg.method === "Runtime.exceptionThrown") {
      const d = msg.params.exceptionDetails;
      problems.push(`uncaught: ${d.exception?.description ?? d.text}`);
    } else if (msg.method === "Runtime.consoleAPICalled" && msg.params.type === "error") {
      problems.push(`console.error: ${msg.params.args.map((a) => a.value ?? a.description).join(" ")}`);
    } else if (msg.method === "Log.entryAdded" && msg.params.entry.level === "error") {
      const entry = msg.params.entry;
      if (entry.source === "network") networkErrors.push(`${entry.text} ${entry.url ?? ""}`);
      else problems.push(`log: ${entry.text}`);
    }
  });
  await send("Runtime.enable");
  await send("Log.enable");
  await send("Page.enable");
  await send("DOM.enable");
  await send("Page.addScriptToEvaluateOnNewDocument", { source: HELPERS });

  const evaluate = async (expression) => {
    const result = await send("Runtime.evaluate", { expression, returnByValue: true, awaitPromise: true });
    if (result.exceptionDetails) {
      throw new Error(result.exceptionDetails.exception?.description ?? result.exceptionDetails.text);
    }
    return result.result.value;
  };
  const waitFor = async (expression, timeout = 5000) => {
    const start = Date.now();
    for (;;) {
      let value;
      try {
        value = await evaluate(expression);
      } catch {
        value = false;
      }
      if (value) return value;
      if (Date.now() - start > timeout) throw new Error(`timed out waiting for: ${expression}`);
      await sleep(50);
    }
  };
  const has = (sel, text) => `!!__t.byText(${JSON.stringify(sel)}, ${JSON.stringify(text)})`;
  const goto = async (url) => {
    await send("Page.navigate", { url: "about:blank" });
    await send("Page.navigate", { url });
    await waitFor("document.readyState === 'complete' && !!document.querySelector('.topbar')");
  };
  const hash = async (h) => {
    await evaluate(`(location.hash = ${JSON.stringify(h)}, true)`);
  };
  /** Puts files into an <input type="file">, as the file picker would. */
  const setFiles = async (selector, files) => {
    const { root: doc } = await send("DOM.getDocument", { depth: 0 });
    const { nodeId } = await send("DOM.querySelector", { nodeId: doc.nodeId, selector });
    if (!nodeId) throw new Error(`no element ${selector}`);
    await send("DOM.setFileInputFiles", { nodeId, files });
  };
  const check = async (name, fn) => {
    stats.checks++;
    const before = problems.length;
    try {
      await fn();
      await sleep(100);
      if (problems.length > before) throw new Error(problems.slice(before).join("\n      "));
      console.log(`  ok    ${name}`);
    } catch (error) {
      stats.failures++;
      console.log(`  FAIL  ${name}\n      ${String(error.message).split("\n").join("\n      ")}`);
    }
  };

  console.log(`browser: ${browserPath}`);
  return {
    evaluate,
    waitFor,
    has,
    goto,
    hash,
    setFiles,
    check,
    problems,
    networkErrors,
    close: () => cdp.ws.close(),
  };
}

/** Runs `main`, always stops the child processes, prints the verdict and exits. */
export async function run(main) {
  try {
    await main();
  } catch (error) {
    stats.failures++;
    console.error(`smoke test aborted: ${error.stack ?? error}`);
  } finally {
    for (const child of children) {
      try {
        child.kill("SIGTERM");
      } catch {
        // already gone
      }
    }
    await sleep(300);
    rmSync(profile, { recursive: true, force: true });
  }
  console.log(
    stats.failures === 0
      ? `\nAll ${stats.checks} checks passed.`
      : `\n${stats.failures} failure(s) in ${stats.checks} checks.`,
  );
  process.exit(stats.failures === 0 ? 0 : 1);
}
