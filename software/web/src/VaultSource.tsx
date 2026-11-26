import { useEffect, useRef, useState, type ReactNode } from 'react';
import type { Action, Condition, Fact, Pattern } from './types';

/** What to highlight in a file's source. */
export type Locator = { kind: 'fact'; rel: string; values: string[] } | { kind: 'rule'; name: string };

// ---------------------------------------------------------------------------
// Fetching (dev server serves /vault-src/<path inside vault>)
// ---------------------------------------------------------------------------

const cache = new Map<string, Promise<string | null>>();

export function fetchVaultSource(vaultPath: string, reload = false): Promise<string | null> {
  if (reload) cache.delete(vaultPath);
  if (!cache.has(vaultPath)) {
    cache.set(
      vaultPath,
      fetch(`/vault-src/${vaultPath}`)
        .then((r) => (r.ok ? r.text() : null))
        .catch(() => null),
    );
  }
  return cache.get(vaultPath)!;
}

// ---------------------------------------------------------------------------
// DSL reconstruction (for runtime items, or when the file can't be fetched)
// ---------------------------------------------------------------------------

const isVar = (v: string) => /^\$[A-Za-z0-9_]+$/.test(v);
const q = (v: string) => `"${v.replace(/\\/g, '\\\\').replace(/"/g, '\\"')}"`;
const val = (v: string) => (isVar(v) ? v : q(v));
const call = (name: string, args: Record<string, string>) =>
  `${name}(${Object.entries(args)
    .map(([k, v]) => `${k}=${val(v)}`)
    .join(', ')})`;

function condDsl(c: Condition): string {
  switch (c.kind) {
    case 'text':
      return q(c.template);
    case 'contains':
      return `~${q(c.phrase)}`;
    case 'fact':
      return call(c.rel, c.args);
    case 'not_fact':
      return `!${call(c.rel, c.args)}`;
    case 'not_eq':
      return `${val(c.left)} != ${val(c.right)}`;
  }
}

function actionDsl(a: Action): string {
  switch (a.kind) {
    case 'assert':
      return `${a.scope === 'long_term' ? '++' : a.scope === 'chat' ? '+chat ' : '+'}${call(a.rel, a.args)}`;
    case 'retract':
      return a.args && Object.keys(a.args).length > 0 ? `-${call(a.rel, a.args)}` : `-${a.rel}`;
    case 'tool':
      return `@${call(a.name, a.args)}`;
    case 'reply':
      return `=> ${q(a.template)}`;
  }
}

export function factDsl(f: Fact): string {
  const scope = f.scope === 'long_term' ? '' : `[${f.scope}] `;
  return `${scope}${call(f.rel, f.args)}`;
}

export function patternDsl(p: Pattern): string {
  const lines = [`rule ${p.name}`, `when: ${p.when.map(condDsl).join(', ')}`];
  if (Math.abs(p.trust - 0.9) > 1e-9) lines.push(`trust: ${p.trust}`);
  if (p.status === 'candidate') lines.push('status: candidate');
  if (p.then.length === 1) lines.push(`then: ${actionDsl(p.then[0])}`);
  else lines.push('then:', ...p.then.map((a) => `  ${actionDsl(a)}`));
  return lines.join('\n');
}

// ---------------------------------------------------------------------------
// Karin DSL highlighting
// ---------------------------------------------------------------------------

const TOKEN =
  /("(?:[^"\\]|\\.)*")|(\$[A-Za-z0-9_]+)|(\+\+|\+chat\b|=>|!=|[+\-@!~])|([A-Za-z_][A-Za-z0-9_]*)(?=\()|(\b\d+(?:\.\d+)?\b)/g;

function highlightExpr(text: string, keyBase: string): ReactNode[] {
  const out: ReactNode[] = [];
  let last = 0;
  let i = 0;
  for (const m of text.matchAll(TOKEN)) {
    if (m.index! > last) out.push(text.slice(last, m.index));
    const cls = m[1] ? 'k-str' : m[2] ? 'k-var' : m[3] ? 'k-op' : m[4] ? 'k-rel' : 'k-num';
    out.push(
      <span key={`${keyBase}-${i++}`} className={cls}>
        {m[0]}
      </span>,
    );
    last = m.index! + m[0].length;
  }
  if (last < text.length) out.push(text.slice(last));
  return out;
}

function highlightKarinLine(line: string, key: string): ReactNode {
  if (/^\s*\/\//.test(line)) return <span className="k-comment">{line}</span>;
  const rule = line.match(/^(\s*)(rule|##)(\s+)(.*)$/);
  if (rule) {
    return (
      <>
        {rule[1]}
        <span className="k-kw">{rule[2]}</span>
        {rule[3]}
        <span className="k-name">{rule[4]}</span>
      </>
    );
  }
  const field = line.match(/^(\s*)(when|then|trust|status|kind):(.*)$/);
  if (field) {
    return (
      <>
        {field[1]}
        <span className="k-kw">{field[2]}:</span>
        {highlightExpr(field[3], key)}
      </>
    );
  }
  const scope = line.match(/^(\s*)(\[(?:working|chat|long_term)\])(.*)$/);
  if (scope) {
    return (
      <>
        {scope[1]}
        <span className="k-kw">{scope[2]}</span>
        {highlightExpr(scope[3], key)}
      </>
    );
  }
  return <>{highlightExpr(line, key)}</>;
}

// ---------------------------------------------------------------------------
// Locating an item's lines in the source
// ---------------------------------------------------------------------------

function locate(lines: string[], loc: Locator | undefined): Set<number> {
  const hit = new Set<number>();
  if (!loc) return hit;
  if (loc.kind === 'fact') {
    const unescaped = lines.map((l) => l.replace(/\\"/g, '"'));
    const i = unescaped.findIndex(
      (l) => l.includes(`${loc.rel}(`) && loc.values.every((v) => l.includes(`"${v}"`)),
    );
    if (i >= 0) hit.add(i);
    return hit;
  }
  const start = lines.findIndex((l) => {
    const m = l.trim().match(/^(?:rule|##)\s+(.+)$/);
    return m !== null && m[1].trim() === loc.name;
  });
  if (start < 0) return hit;
  hit.add(start);
  for (let i = start + 1; i < lines.length; i++) {
    const t = lines[i].trim();
    if (t === '' || t.startsWith('```') || /^(rule|##)\s/.test(t)) break;
    hit.add(i);
  }
  return hit;
}

// ---------------------------------------------------------------------------
// Markdown (Obsidian-like reading view, code blocks shown as code)
// ---------------------------------------------------------------------------

function inline(text: string, key: string): ReactNode[] {
  const out: ReactNode[] = [];
  const re = /(`[^`]+`)|(\*\*[^*]+\*\*)|(\*[^*]+\*)/g;
  let last = 0;
  let i = 0;
  for (const m of text.matchAll(re)) {
    if (m.index! > last) out.push(text.slice(last, m.index));
    const t = m[0];
    if (m[1]) out.push(<code key={`${key}-${i++}`}>{t.slice(1, -1)}</code>);
    else if (m[2]) out.push(<strong key={`${key}-${i++}`}>{t.slice(2, -2)}</strong>);
    else out.push(<em key={`${key}-${i++}`}>{t.slice(1, -1)}</em>);
    last = m.index! + t.length;
  }
  if (last < text.length) out.push(text.slice(last));
  return out;
}

/** `lineBase`: number shown for line index 0 (1 for real files; 0 when a fence was added around rebuilt DSL). */
function renderMarkdown(lines: string[], hits: Set<number>, lineBase = 1): ReactNode[] {
  const out: ReactNode[] = [];
  let i = 0;
  while (i < lines.length) {
    const line = lines[i];
    const fence = line.match(/^```(\w*)\s*$/);
    if (fence) {
      const lang = fence[1];
      const start = i + 1;
      let end = start;
      while (end < lines.length && !/^```\s*$/.test(lines[end])) end++;
      const body = lines.slice(start, end);
      out.push(
        <div key={`code-${i}`} className="src-code">
          {lang && <div className="src-lang">{lang}</div>}
          <pre>
            {body.map((l, j) => {
              const n = start + j;
              return (
                <div key={n} id={`src-line-${n}`} className={`src-line ${hits.has(n) ? 'hit' : ''}`}>
                  <span className="src-ln">{n + lineBase}</span>
                  <span className="src-text">
                    {lang === 'karin' ? highlightKarinLine(l, `l${n}`) : l}
                    {l === '' ? ' ' : null}
                  </span>
                </div>
              );
            })}
          </pre>
        </div>,
      );
      i = end + 1;
      continue;
    }
    const h = line.match(/^(#{1,6})\s+(.*)$/);
    if (h) {
      const level = Math.min(h[1].length + 1, 6);
      const Tag = `h${level}` as 'h2';
      out.push(
        <Tag key={i} className="src-h">
          {inline(h[2], `h${i}`)}
        </Tag>,
      );
      i++;
      continue;
    }
    if (line.trim().startsWith('|')) {
      const rows: string[][] = [];
      while (i < lines.length && lines[i].trim().startsWith('|')) {
        const cells = lines[i].trim().replace(/^\||\|$/g, '').split('|').map((c) => c.trim());
        if (!cells.every((c) => /^:?-+:?$/.test(c))) rows.push(cells);
        i++;
      }
      out.push(
        <table key={`t${i}`} className="src-table">
          <tbody>
            {rows.map((r, ri) => (
              <tr key={ri}>
                {r.map((c, ci) =>
                  ri === 0 ? <th key={ci}>{inline(c, `t${i}-${ri}-${ci}`)}</th> : <td key={ci}>{inline(c, `t${i}-${ri}-${ci}`)}</td>,
                )}
              </tr>
            ))}
          </tbody>
        </table>,
      );
      continue;
    }
    if (/^\s*[-*]\s+/.test(line)) {
      const items: string[] = [];
      while (i < lines.length && /^\s*[-*]\s+/.test(lines[i])) {
        items.push(lines[i].replace(/^\s*[-*]\s+/, ''));
        i++;
      }
      out.push(
        <ul key={`ul${i}`} className="src-ul">
          {items.map((it, k) => (
            <li key={k}>{inline(it, `li${i}-${k}`)}</li>
          ))}
        </ul>,
      );
      continue;
    }
    if (line.trim() === '') {
      i++;
      continue;
    }
    const para: string[] = [];
    while (
      i < lines.length &&
      lines[i].trim() !== '' &&
      !/^```/.test(lines[i]) &&
      !/^#{1,6}\s/.test(lines[i]) &&
      !lines[i].trim().startsWith('|') &&
      !/^\s*[-*]\s+/.test(lines[i])
    ) {
      para.push(lines[i]);
      i++;
    }
    out.push(
      <p key={`p${i}`} className="src-p">
        {inline(para.join(' '), `p${i}`)}
      </p>,
    );
  }
  return out;
}

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

/** Shows a vault file as written (or generated DSL when there is no file). */
export function VaultSource({
  vaultPath,
  fallback,
  locator,
}: {
  /** Path inside vault/, e.g. "core/persona/identity.md"; null for runtime items. */
  vaultPath: string | null;
  /** DSL to show when the file isn't available. */
  fallback: string;
  locator?: Locator;
}) {
  const [text, setText] = useState<string | null | undefined>(undefined);
  const [nonce, setNonce] = useState(0);
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    let alive = true;
    setText(undefined);
    if (!vaultPath) {
      setText(null);
      return;
    }
    fetchVaultSource(vaultPath, nonce > 0).then((t) => alive && setText(t));
    return () => {
      alive = false;
    };
  }, [vaultPath, nonce]);

  const fromFile = typeof text === 'string';
  const source = fromFile ? text! : `\`\`\`karin\n${fallback}\n\`\`\``;
  const lines = source.replace(/\r\n/g, '\n').split('\n');
  const hits = locate(lines, locator);
  const firstHit = hits.size > 0 ? Math.min(...hits) : -1;

  // Scroll the note pane to the highlighted lines.
  useEffect(() => {
    if (firstHit < 0) return;
    const el = document.getElementById(`src-line-${firstHit}`);
    const pane = ref.current?.closest('.vx-note') as HTMLElement | null;
    if (el && pane) {
      const top = el.getBoundingClientRect().top - pane.getBoundingClientRect().top + pane.scrollTop;
      pane.scrollTo({ top: top - pane.clientHeight / 3 });
    }
  }, [firstHit, text, locator]);

  if (text === undefined) return <div className="muted src-loading">Loading source…</div>;

  return (
    <div ref={ref} className="vault-source">
      <div className="src-bar">
        <span className="muted">
          {fromFile ? (
            <>
              <span className="mono">vault/{vaultPath}</span> · as written
            </>
          ) : vaultPath ? (
            'File not available from this server - showing the DSL rebuilt from memory'
          ) : (
            'Not stored in the vault - DSL rebuilt from memory'
          )}
        </span>
        {fromFile && (
          <button className="vx-tool" onClick={() => setNonce((n) => n + 1)}>
            Reload
          </button>
        )}
      </div>
      {renderMarkdown(lines, hits, fromFile ? 1 : 0)}
    </div>
  );
}
