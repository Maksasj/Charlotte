# Charlotte web interface

A browser interface to the Charlotte engine: Chat (with the trace of every
reply), Knowledge, Candidates, Learn and Status. It holds no logic of its own;
it only calls the HTTP API described in [`../docs/api.md`](../docs/api.md).

TypeScript, React and Vite. No other runtime dependencies.

## Run in development

Start the engine, then the app:

```sh
charlotte serve --vault vault/ --port 7878    # in software/

cd web
npm install
npm run dev                                   # http://localhost:5173
```

Vite proxies `/v1` to `http://127.0.0.1:7878` (see `vite.config.ts`). If the
engine is not running the app still loads and shows "Cannot reach the engine
at /v1 — is `charlotte serve` running?" with a Retry button.

## Build

```sh
npm run build        # type-checks (tsc -b), then writes static files to dist/
npm run preview      # optional: serve dist/ locally, with the same /v1 proxy
```

`dist/` can be served by `charlotte serve` at `/`. All API calls are relative
(`/v1/...`) and navigation uses the URL hash (`#/chat/c1`,
`#/knowledge/<id>`), so the server needs no route fallback: it only has to
serve `index.html` at `/` and the files under `/assets/`.

## Mock mode

To work on the interface without an engine, add `?mock=1` to the URL of the
dev server:

```
http://localhost:5173/?mock=1
```

`src/api/mock.ts` then stands in for `fetch` underneath the real client and
answers with fixture data shaped per `docs/api.md`: two conversations with
full traces, a few knowledge items including two candidates, and one finished
learning job. It keeps state in memory until the page is reloaded. New
messages always get a canned "not understood" reply; a learning job whose
path contains `missing` fails, any other finishes after a few polls. A badge
in the top bar shows that mock data is in use.

The mock is left out of `npm run build`, where `?mock=1` does nothing. To get
a static build that includes it, use `npm run build:mock` (writes
`dist-mock/`).

## Smoke tests

Both drive a headless Chrome (or Edge) over the DevTools protocol and fail on
any uncaught exception, unhandled promise rejection or `console.error`. They
need Node 22+ and a local Chrome; set `CHROME=/path/to/browser` if it is not
found.

```sh
npm run smoke
```

Against the mock. Starts the dev server on port 5179, goes through every view
with `?mock=1`, then loads the app without the mock and without an engine to
check the unreachable-engine banner.

```sh
npm run build
npm run smoke:live -- http://127.0.0.1:7878     # default: http://127.0.0.1:7899
```

Against a real `charlotte serve` that also serves `web/dist` at `/`. It holds
and closes conversations, filters and opens knowledge of every kind, edits
items, runs learning jobs and reviews the candidates, and compares what is on
screen with what the API returns. **It changes the server's state** (it
restores the items it edits, but conversations, jobs and trust changes stay),
so point it at a scratch copy of the vault. It expects the seed vault and
`corpus/pretrain-80-per-shape.jsonl` in the server's working directory.

## Layout

| Path | Holds |
|---|---|
| `src/api/types.ts` | The API's request and response types, mirroring `docs/api.md` |
| `src/api/client.ts` | Every HTTP call, one function per endpoint, and the error handling |
| `src/api/mock.ts` | The mock engine for `?mock=1` |
| `src/views/` | One file per view |
| `src/components/TraceView.tsx` | The trace of a reply, layer by layer |
| `src/components/ItemPanel.tsx` | One knowledge item, with its edit mode |
| `src/router.ts`, `src/hooks.ts`, `src/format.ts` | Hash navigation, data loading, display helpers |
| `scripts/smoke.mjs`, `scripts/smoke-live.mjs`, `scripts/harness.mjs` | The browser smoke tests and what they share |
