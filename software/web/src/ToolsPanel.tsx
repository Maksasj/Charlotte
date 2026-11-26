import { useState, type ReactNode } from 'react';
import { KarinCode, patternDsl } from './VaultSource';
import type { AgentState, Pattern, ToolInfo } from './types';

/** How a rule calls a tool: `@name(arg="...", ...)`. */
function callSnippet(tool: ToolInfo): string {
  const args = (tool.args ?? []).filter((a) => !a.name.startsWith('<'));
  const parts = args
    .sort((a, b) => Number(b.required) - Number(a.required))
    .map((a) => `${a.name}=${a.required ? `$${a.name}` : `"..."`}`);
  return `@${tool.name}(${parts.join(', ')})`;
}

function callers(tool: ToolInfo, patterns: Pattern[]): Pattern[] {
  return patterns.filter((p) => p.then.some((a) => a.kind === 'tool' && a.name === tool.name));
}

/** Rules that read (positively) one of the facts the tool produces. */
function consumers(tool: ToolInfo, patterns: Pattern[]): Map<string, Pattern[]> {
  const out = new Map<string, Pattern[]>();
  for (const o of tool.produces ?? []) {
    const readers = patterns.filter((p) => p.when.some((c) => c.kind === 'fact' && c.rel === o.rel));
    out.set(o.rel, readers);
  }
  return out;
}

function source(p: Pattern) {
  return p.provenance.replace(/^vault:/, '').replace(/^generated-/, '');
}

/** Rules as small pills, grouped by the file they come from; big lists fold. */
function RuleGroups({
  rules,
  onOpen,
  openFiles = 4,
}: {
  rules: Pattern[];
  onOpen: (p: Pattern) => void;
  openFiles?: number;
}) {
  if (rules.length === 0) return <div className="muted tv-none">None yet.</div>;
  const groups = new Map<string, Pattern[]>();
  for (const r of rules) groups.set(source(r), [...(groups.get(source(r)) ?? []), r]);
  const entries = [...groups.entries()].sort((x, y) => y[1].length - x[1].length || x[0].localeCompare(y[0]));
  const render = ([file, list]: [string, Pattern[]]) => (
    <div key={file} className="tv-group-row">
      <span className="tv-file mono" title={file}>
        {file}
      </span>
      <span className="tv-pills">
        {list.map((r) => (
          <button key={r.id} className="tv-pill" onClick={() => onOpen(r)} title={`${r.provenance} — open in Knowledge`}>
            {r.name}
          </button>
        ))}
      </span>
    </div>
  );
  const shown = entries.slice(0, openFiles);
  const rest = entries.slice(openFiles);
  return (
    <div className="tv-groups">
      {shown.map(render)}
      {rest.length > 0 && (
        <details className="tv-more">
          <summary>
            Show {rest.length} more {rest.length === 1 ? 'file' : 'files'} ({rest.reduce((n, [, l]) => n + l.length, 0)} rules)
          </summary>
          {rest.map(render)}
        </details>
      )}
    </div>
  );
}

export function ToolsPanel({
  tools,
  state,
  onOpenRule,
  learning,
}: {
  tools: ToolInfo[];
  state: AgentState;
  onOpenRule: (p: Pattern) => void;
  /** Extra content for the learning (meta) tools: buttons, episodes, chains. */
  learning: Record<string, ReactNode>;
}) {
  const [selected, setSelected] = useState<string>(() => tools.find((t) => !t.meta)?.name ?? tools[0]?.name);
  const tool = tools.find((t) => t.name === selected) ?? tools[0];
  const world = tools.filter((t) => !t.meta);
  const meta = tools.filter((t) => t.meta);
  const usage = (t: ToolInfo) => callers(t, state.patterns).length;

  const listItem = (t: ToolInfo) => (
    <button key={t.name} className={`vx-row tv-item ${t.name === tool?.name ? 'active' : ''}`} onClick={() => setSelected(t.name)}>
      <span className="tv-at">@</span>
      <span className="vx-label mono">{t.name}</span>
      <span className="vx-count" title="rules that call it">
        {usage(t)}
      </span>
    </button>
  );

  if (!tool) return <div className="muted">No tools registered.</div>;

  const calledBy = callers(tool, state.patterns);
  const readers = consumers(tool, state.patterns);
  const allReaders = [...new Set([...readers.values()].flat())];
  const exampleCaller = calledBy.find((p) => !p.provenance.startsWith('vault:generated')) ?? calledBy[0];
  const exampleReader = allReaders.find((p) => p.then.some((a) => a.kind === 'reply')) ?? allReaders[0];

  return (
    <div className="vault-explorer tools-view">
      <nav className="vx-tree">
        <div className="tv-group">World tools</div>
        {world.map(listItem)}
        <div className="tv-group">Learning tools</div>
        {meta.map(listItem)}
        <div className="tv-legend muted">The number is how many rules call the tool.</div>
      </nav>

      <article className="vx-note">
        <div className="vx-note-head">
          <div className="vx-note-head-row">
            <div className="vx-crumbs">
              <span>{tool.meta ? 'learning tool' : 'world tool'}</span>
              {tool.package && (
                <>
                  <span className="vx-crumb-sep">/</span>
                  <span className="mono">{tool.package}</span>
                </>
              )}
            </div>
          </div>
          <h3 className="vx-title">
            <span className="tv-at big">@</span>
            {tool.name}
          </h3>
          <div className="muted tv-desc">{tool.description}</div>
        </div>

        <div className="vx-note-body tv-body">
          <section>
            <div className="trace-label">Call it from a rule</div>
            <KarinCode code={`then: ${callSnippet(tool)}`} />
          </section>

          <section>
            <div className="trace-label">How it connects</div>
            <div className="tv-diagram">
              <span className="tv-node">
                <strong>{calledBy.length}</strong> {calledBy.length === 1 ? 'rule' : 'rules'}
              </span>
              <span className="tv-edge">calls</span>
              <span className="tv-node tv-node-tool">
                <span className="tv-at">@</span>
                {tool.name}
              </span>
              <span className="tv-edge">produces</span>
              <span className="tv-outputs">
                {(tool.produces ?? []).map((o) => {
                  const n = (readers.get(o.rel) ?? []).length;
                  return (
                    <span key={o.rel} className="tv-output">
                      <span className="mono">
                        <span className="k-rel">{o.rel}</span>
                        <span className="muted">({o.args.join(', ')})</span>
                      </span>
                      <span className="tv-edge small">read by</span>
                      <span className="tv-node">
                        <strong>{n}</strong> {n === 1 ? 'rule' : 'rules'}
                      </span>
                      <span className="muted tv-when">
                        {o.when}
                        {o.scope !== 'working' && ` · ${o.scope === 'long_term' ? 'long-term' : o.scope}`}
                      </span>
                    </span>
                  );
                })}
                {(tool.produces ?? []).length === 0 && <span className="muted">nothing</span>}
              </span>
            </div>
          </section>

          <section>
            <div className="tv-section-head">
              <span className="trace-label">Called by</span>
              <span className="muted">{calledBy.length} rules · click one to open it</span>
            </div>
            <RuleGroups rules={calledBy} onOpen={onOpenRule} />
          </section>

          {(tool.produces ?? []).map((o) => (
            <section key={o.rel}>
              <div className="tv-section-head">
                <span className="trace-label">
                  Reacts to <span className="mono k-rel tv-rel">{o.rel}</span>
                </span>
                <span className="muted">{(readers.get(o.rel) ?? []).length} rules</span>
              </div>
              <RuleGroups rules={readers.get(o.rel) ?? []} onOpen={onOpenRule} openFiles={3} />
            </section>
          ))}

          {(tool.args ?? []).length > 0 && (
            <section>
              <div className="trace-label">Arguments</div>
              <table className="tv-table">
                <tbody>
                  {(tool.args ?? []).map((a) => (
                    <tr key={a.name}>
                      <td className="mono">{a.name}</td>
                      <td>{a.required ? <span className="badge warn">required</span> : <span className="badge">optional</span>}</td>
                      <td className="muted">{a.description}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </section>
          )}

          {(exampleCaller || exampleReader) && (
            <section>
              <div className="trace-label">Example from the vault</div>
              <div className="muted tv-note">
                One rule that calls the tool, and one that turns its result into a reply.
              </div>
              {exampleCaller && <KarinCode code={patternDsl(exampleCaller)} />}
              {exampleReader && exampleReader !== exampleCaller && <KarinCode code={patternDsl(exampleReader)} />}
            </section>
          )}

          {learning[tool.name] && (
            <section>
              <div className="trace-label">Run it</div>
              {learning[tool.name]}
            </section>
          )}
        </div>
      </article>
    </div>
  );
}
