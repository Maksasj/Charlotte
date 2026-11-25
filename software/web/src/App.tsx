import { useEffect, useRef, useState } from 'react';
import { api } from './api';
import type {
  Action,
  AgentState,
  Condition,
  Scope,
  SelfTestResult,
  ToolInfo,
  TurnTrace,
} from './types';

type Tab = 'trace' | 'facts' | 'patterns' | 'tools' | 'tests';

const SUGGESTIONS = [
  'who are you',
  'my name is Alex',
  'weather',
  'calculate (12 + 8) * 3',
  'define anti-unification',
  'remember buy coffee',
  'generalize',
  'chunk last',
];

function fmtCond(c: Condition): string {
  switch (c.kind) {
    case 'text':
      return `text("${c.template}")`;
    case 'contains':
      return `contains("${c.phrase}")`;
    case 'fact':
      return `${c.rel}(${Object.entries(c.args)
        .map(([k, v]) => `${k}=${v}`)
        .join(', ')})`;
    case 'not_fact':
      return `¬${c.rel}(${Object.entries(c.args)
        .map(([k, v]) => `${k}=${v}`)
        .join(', ')})`;
  }
}

function fmtAction(a: Action): string {
  switch (a.kind) {
    case 'assert':
      return `assert[${a.scope === 'long_term' ? 'lt' : 'w'}] ${a.rel}(${Object.entries(a.args)
        .map(([k, v]) => `${k}=${v}`)
        .join(', ')})`;
    case 'retract':
      return `retract ${a.rel}`;
    case 'tool':
      return `tool:${a.name}(${Object.entries(a.args)
        .map(([k, v]) => `${k}=${v}`)
        .join(', ')})`;
    case 'reply':
      return `reply "${a.template}"`;
  }
}

export default function App() {
  const [theme, setTheme] = useState<'light' | 'dark'>('light');
  const [state, setState] = useState<AgentState | null>(null);
  const [tools, setTools] = useState<ToolInfo[]>([]);
  const [tab, setTab] = useState<Tab>('trace');
  const [input, setInput] = useState('');
  const [selectedTrace, setSelectedTrace] = useState<TurnTrace | null>(null);
  const [statusMsg, setStatusMsg] = useState<string | null>(null);
  const [errorMsg, setErrorMsg] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  const [patternFilter, setPatternFilter] = useState('');
  const [factFilter, setFactFilter] = useState('');

  const [teachReply, setTeachReply] = useState('');
  const [teachPromote, setTeachPromote] = useState(true);

  const [factRel, setFactRel] = useState('weather_city');
  const [factArgsText, setFactArgsText] = useState('city=Berlin, condition=sunny, temp=21°C');
  const [factScope, setFactScope] = useState<Scope>('long_term');

  const [tests, setTests] = useState<SelfTestResult[]>([]);

  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    document.documentElement.setAttribute('data-theme', theme);
  }, [theme]);

  async function loadInitial() {
    try {
      const data = await api.getState();
      setState(data.state);
      setTools(data.tools);
      const lastTrace = [...data.state.messages].reverse().find((m) => m.trace)?.trace;
      if (lastTrace) setSelectedTrace(lastTrace);
      setErrorMsg(null);
    } catch (e) {
      setErrorMsg(`Cannot reach Rust backend on /api: ${(e as Error).message}`);
    }
  }

  useEffect(() => {
    loadInitial();
  }, []);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [state?.messages.length]);

  async function sendMessage(text: string) {
    const trimmed = text.trim();
    if (!trimmed || busy) return;
    setBusy(true);
    setErrorMsg(null);
    setStatusMsg(null);
    try {
      const res = await api.chat(trimmed);
      setState(res.state);
      setSelectedTrace(res.trace);
      setInput('');
      setTeachReply('');
    } catch (e) {
      setErrorMsg((e as Error).message);
    } finally {
      setBusy(false);
    }
  }

  async function handleTeach(impasseInput: string) {
    if (!teachReply.trim()) return;
    setBusy(true);
    try {
      const res = await api.teach(impasseInput, teachReply.trim(), teachPromote);
      setState(res.state);
      setStatusMsg(
        `Taught "${impasseInput}" (${teachPromote ? 'active' : 'candidate'}). Send it again to test.`
      );
      setTeachReply('');
    } catch (e) {
      setErrorMsg((e as Error).message);
    } finally {
      setBusy(false);
    }
  }

  async function handleAddFact(e: React.FormEvent) {
    e.preventDefault();
    if (!factRel.trim()) return;
    const args: Record<string, string> = {};
    for (const part of factArgsText.split(',')) {
      const idx = part.indexOf('=');
      if (idx > 0) {
        args[part.slice(0, idx).trim()] = part.slice(idx + 1).trim();
      }
    }
    const res = await api.addFact(factRel.trim(), args, factScope);
    setState(res.state);
    setStatusMsg(`Added ${factScope} fact: ${factRel.trim()}`);
  }

  async function runGeneralize(autoPromote: boolean) {
    const res = await api.generalize(autoPromote);
    setState(res.state);
    setStatusMsg(res.summary);
    setTab('patterns');
  }

  async function runChunk(autoPromote: boolean) {
    const res = await api.chunk(autoPromote);
    setState(res.state);
    setStatusMsg(res.summary);
    setTab('patterns');
  }

  async function runSelfTests() {
    const res = await api.selftest();
    setTests(res);
    setTab('tests');
  }

  async function handleReset() {
    const res = await api.reset();
    setState(res.state);
    setSelectedTrace(null);
    setStatusMsg('Reset to pretrained seed state.');
  }

  const lastAgentMsg = state
    ? [...state.messages].reverse().find((m) => m.role === 'agent')
    : undefined;
  const activeImpasse = lastAgentMsg?.trace?.impasse ? lastAgentMsg.trace : null;
  const candidateCount = state?.patterns.filter((p) => p.status === 'candidate').length ?? 0;

  const visibleFacts = (state ? [...state.facts].reverse() : []).filter((f) => {
    if (!factFilter.trim()) return true;
    const q = factFilter.toLowerCase();
    return (
      f.rel.toLowerCase().includes(q) ||
      JSON.stringify(f.args).toLowerCase().includes(q) ||
      f.scope.includes(q)
    );
  });

  const visiblePatterns = (state ? [...state.patterns] : [])
    .sort((a, b) => (a.status === 'candidate' ? -1 : b.status === 'candidate' ? 1 : 0))
    .filter((p) => {
      if (!patternFilter.trim()) return true;
      const q = patternFilter.toLowerCase();
      return (
        p.name.toLowerCase().includes(q) ||
        p.kind.includes(q) ||
        p.status.includes(q) ||
        p.when.some((c) => fmtCond(c).toLowerCase().includes(q)) ||
        p.then.some((a) => fmtAction(a).toLowerCase().includes(q))
      );
    });

  return (
    <div className="app">
      <header className="topbar">
        <div className="brand">
          <span className="brand-dot" />
          <h1>Karin</h1>
          {state && (
            <span className="brand-stats">
              {state.facts.length} facts · {state.patterns.length} patterns · turn {state.turn}
            </span>
          )}
        </div>
        <div className="topbar-actions">
          <button
            className="btn btn-ghost btn-sm"
            onClick={() => setTheme((t) => (t === 'light' ? 'dark' : 'light'))}
          >
            {theme === 'light' ? 'Dark' : 'Light'}
          </button>
          <button className="btn btn-ghost btn-sm" onClick={runSelfTests}>
            Self-Tests
          </button>
          <button className="btn btn-ghost btn-sm" onClick={handleReset}>
            Reset
          </button>
        </div>
      </header>

      {errorMsg && (
        <div className="toast err">
          <span>{errorMsg}</span>
          <button className="meta-link" onClick={() => setErrorMsg(null)}>
            dismiss
          </button>
        </div>
      )}
      {statusMsg && (
        <div className="toast">
          <span>{statusMsg}</span>
          <button className="meta-link" onClick={() => setStatusMsg(null)}>
            dismiss
          </button>
        </div>
      )}

      <div className="main-grid">
        <section className="chat-pane">
          <div className="messages">
            {!state || state.messages.length === 0 ? (
              <div className="empty-state">
                <h2>Karin</h2>
                <p>
                  A purely symbolic assistant built on facts, composable patterns, and inspectable
                  reasoning.
                </p>
                <div className="suggestion-grid">
                  {SUGGESTIONS.map((s) => (
                    <button key={s} className="chip" onClick={() => sendMessage(s)}>
                      {s}
                    </button>
                  ))}
                </div>
              </div>
            ) : (
              state.messages.map((m) => (
                <div
                  key={m.id}
                  className={`msg-row ${m.role} ${m.trace?.impasse ? 'impasse' : ''}`}
                >
                  <div className="msg-bubble">{m.text}</div>
                  {m.role === 'agent' && m.trace && (
                    <div className="msg-meta">
                      <span>
                        {m.trace.impasse
                          ? 'Impasse'
                          : `${m.trace.steps.length} ${m.trace.steps.length === 1 ? 'step' : 'steps'}`}
                      </span>
                      <button
                        className="meta-link"
                        onClick={() => {
                          setSelectedTrace(m.trace!);
                          setTab('trace');
                        }}
                      >
                        Trace
                      </button>
                      <span className="vote-group">
                        <button
                          className={`vote-btn ${m.feedback === 'up' ? 'active' : ''}`}
                          onClick={async () => {
                            const r = await api.feedback(m.turn, true);
                            setState(r.state);
                          }}
                        >
                          +1
                        </button>
                        <button
                          className={`vote-btn ${m.feedback === 'down' ? 'active' : ''}`}
                          onClick={async () => {
                            const r = await api.feedback(m.turn, false);
                            setState(r.state);
                          }}
                        >
                          -1
                        </button>
                      </span>
                    </div>
                  )}
                </div>
              ))
            )}
            <div ref={messagesEndRef} />
          </div>

          {activeImpasse && (
            <div className="teach-card">
              <div className="teach-title">
                No pattern matched &ldquo;{activeImpasse.input}&rdquo; — teach Karin
              </div>
              <div className="row">
                <input
                  type="text"
                  className="field"
                  style={{ flex: 1 }}
                  placeholder="Type the reply Karin should give..."
                  value={teachReply}
                  onChange={(e) => setTeachReply(e.target.value)}
                />
                <label className="muted row" style={{ gap: 5 }}>
                  <input
                    type="checkbox"
                    checked={teachPromote}
                    onChange={(e) => setTeachPromote(e.target.checked)}
                  />
                  promote
                </label>
                <button
                  className="btn btn-primary btn-sm"
                  onClick={() => handleTeach(activeImpasse.input)}
                  disabled={busy}
                >
                  Teach
                </button>
              </div>
            </div>
          )}

          <div className="composer-wrap">
            <form
              className="composer"
              onSubmit={(e) => {
                e.preventDefault();
                sendMessage(input);
              }}
            >
              <input
                type="text"
                placeholder="Message Karin..."
                value={input}
                onChange={(e) => setInput(e.target.value)}
              />
              <button type="submit" className="btn btn-primary" disabled={busy || !input.trim()}>
                Send
              </button>
            </form>
          </div>
        </section>

        <section className="inspector-pane">
          <div className="tabs">
            <button
              className={`tab ${tab === 'trace' ? 'active' : ''}`}
              onClick={() => setTab('trace')}
            >
              Trace
            </button>
            <button
              className={`tab ${tab === 'facts' ? 'active' : ''}`}
              onClick={() => setTab('facts')}
            >
              Facts <span className="tab-count">{state?.facts.length ?? 0}</span>
            </button>
            <button
              className={`tab ${tab === 'patterns' ? 'active' : ''}`}
              onClick={() => setTab('patterns')}
            >
              Patterns <span className="tab-count">{state?.patterns.length ?? 0}</span>
              {candidateCount > 0 && <span className="badge warn">{candidateCount}</span>}
            </button>
            <button
              className={`tab ${tab === 'tools' ? 'active' : ''}`}
              onClick={() => setTab('tools')}
            >
              Tools
            </button>
            <button
              className={`tab ${tab === 'tests' ? 'active' : ''}`}
              onClick={() => setTab('tests')}
            >
              Tests
            </button>
          </div>

          <div className="inspector-body">
            {tab === 'trace' && (
              <>
                {!selectedTrace ? (
                  <div className="muted">
                    Send a message to inspect the Perceive → Match → Select → Fire trace.
                  </div>
                ) : (
                  <>
                    <div className="card">
                      <div className="card-head">
                        <span className="card-title">
                          Turn {selectedTrace.turn} · &ldquo;{selectedTrace.input}&rdquo;
                        </span>
                        <span className={`badge ${selectedTrace.impasse ? 'warn' : 'ok'}`}>
                          {selectedTrace.impasse
                            ? 'Impasse'
                            : `${selectedTrace.steps.length} steps`}
                        </span>
                      </div>
                      <div className="muted">{selectedTrace.reply}</div>
                    </div>

                    {selectedTrace.steps.map((s) => (
                      <div key={s.step} className="card">
                        <div className="card-head">
                          <span className="card-title">
                            {s.step}. {s.winnerName}
                          </span>
                          <span className="muted mono">{s.winnerId}</span>
                        </div>
                        {Object.keys(s.bindings).length > 0 && (
                          <div className="muted mono" style={{ marginBottom: 6 }}>
                            {Object.entries(s.bindings)
                              .map(([k, v]) => `${k} = "${v}"`)
                              .join(' · ')}
                          </div>
                        )}
                        <div className="rule-block mono">
                          {s.effects.map((ef, i) => (
                            <div key={i}>→ {ef}</div>
                          ))}
                        </div>
                        {s.competing.length > 1 && (
                          <div className="muted">
                            Competing:{' '}
                            {s.competing
                              .map((c) => `${c.name} (${c.score.toFixed(1)})`)
                              .join(' · ')}
                          </div>
                        )}
                      </div>
                    ))}
                  </>
                )}
              </>
            )}

            {tab === 'facts' && state && (
              <>
                <form className="card" onSubmit={handleAddFact}>
                  <div className="section-label">Add Fact</div>
                  <div className="row" style={{ marginTop: 6 }}>
                    <input
                      type="text"
                      className="field"
                      value={factRel}
                      onChange={(e) => setFactRel(e.target.value)}
                      placeholder="relation"
                      style={{ width: 130 }}
                    />
                    <input
                      type="text"
                      className="field"
                      value={factArgsText}
                      onChange={(e) => setFactArgsText(e.target.value)}
                      placeholder="k=v, k2=v2"
                      style={{ flex: 1 }}
                    />
                    <select
                      className="field"
                      value={factScope}
                      onChange={(e) => setFactScope(e.target.value as Scope)}
                    >
                      <option value="long_term">long_term</option>
                      <option value="working">working</option>
                    </select>
                    <button type="submit" className="btn btn-primary btn-sm">
                      Add
                    </button>
                  </div>
                </form>

                <input
                  type="text"
                  className="field"
                  placeholder="Filter facts..."
                  value={factFilter}
                  onChange={(e) => setFactFilter(e.target.value)}
                />

                {visibleFacts.map((f) => (
                  <div key={f.id} className="card">
                    <div className="card-head">
                      <span className="mono">
                        <strong>{f.rel}</strong>(
                        {Object.entries(f.args)
                          .map(([k, v]) => `${k}="${v}"`)
                          .join(', ')}
                        )
                      </span>
                      <div className="row">
                        <span className={`badge ${f.scope === 'working' ? 'warn' : ''}`}>
                          {f.scope === 'long_term' ? 'long-term' : 'working'}
                        </span>
                        <button
                          className="btn-danger"
                          title="Delete fact"
                          onClick={async () => {
                            const r = await api.deleteFact(f.id);
                            setState(r.state);
                          }}
                        >
                          ×
                        </button>
                      </div>
                    </div>
                    <div className="muted">
                      {f.id} · {f.provenance} · turn {f.turn}
                    </div>
                  </div>
                ))}
              </>
            )}

            {tab === 'patterns' && state && (
              <>
                <input
                  type="text"
                  className="field"
                  placeholder="Filter patterns..."
                  value={patternFilter}
                  onChange={(e) => setPatternFilter(e.target.value)}
                />

                {visiblePatterns.map((p) => (
                  <div key={p.id} className="card">
                    <div className="card-head">
                      <span className="card-title">
                        {p.name} <span className="muted mono">{p.id}</span>
                      </span>
                      <div className="row">
                        <span className="badge">{p.kind}</span>
                        <span className={`badge ${p.status === 'candidate' ? 'warn' : 'ok'}`}>
                          {p.status}
                        </span>
                        {p.status === 'candidate' && (
                          <button
                            className="btn btn-primary btn-sm"
                            onClick={async () => {
                              const r = await api.promotePattern(p.id);
                              setState(r.state);
                            }}
                          >
                            Promote
                          </button>
                        )}
                        <button
                          className="btn-danger"
                          title="Delete pattern"
                          onClick={async () => {
                            const r = await api.deletePattern(p.id);
                            setState(r.state);
                          }}
                        >
                          ×
                        </button>
                      </div>
                    </div>
                    <div className="rule-block mono">
                      <div>
                        <span className="rule-kw">when</span>
                        {p.when.map(fmtCond).join(' ∧ ')}
                      </div>
                      <div>
                        <span className="rule-kw">then</span>
                        {p.then.map(fmtAction).join(' ; ')}
                      </div>
                    </div>
                    <div className="muted">
                      trust {p.trust.toFixed(2)} · support {p.support} · {p.provenance}
                    </div>
                  </div>
                ))}
              </>
            )}

            {tab === 'tools' && state && (
              <>
                <div className="card">
                  <div className="section-label">Learning Meta-Tools</div>
                  <div className="row" style={{ marginTop: 8 }}>
                    <button className="btn btn-primary btn-sm" onClick={() => runGeneralize(false)}>
                      Anti-Unify Episodes
                    </button>
                    <button className="btn btn-ghost btn-sm" onClick={() => runGeneralize(true)}>
                      Anti-Unify &amp; Promote
                    </button>
                    <button className="btn btn-primary btn-sm" onClick={() => runChunk(false)}>
                      Chunk Last Chain
                    </button>
                    <button className="btn btn-ghost btn-sm" onClick={() => runChunk(true)}>
                      Chunk &amp; Promote
                    </button>
                  </div>
                </div>

                <div className="section-label">Registered Tools</div>
                {tools.map((t) => (
                  <div key={t.name} className="card">
                    <div className="card-head">
                      <span className="mono card-title">{t.name}</span>
                      <span className={`badge ${t.meta ? 'brand' : ''}`}>
                        {t.meta ? 'meta-tool' : 'world tool'}
                      </span>
                    </div>
                    <div className="muted">{t.description}</div>
                  </div>
                ))}

                <div className="section-label">Episodes ({state.episodes.length})</div>
                <div className="card mono">
                  {state.episodes.map((ep) => (
                    <div key={ep.id} style={{ marginBottom: 4 }}>
                      <span className={`badge ${ep.generalized ? 'ok' : ''}`}>
                        {ep.generalized ? 'done' : 'raw'}
                      </span>{' '}
                      &ldquo;{ep.input}&rdquo; → &ldquo;{ep.reply}&rdquo;
                    </div>
                  ))}
                </div>

                <div className="section-label">Multi-Step Chains ({state.chains.length})</div>
                <div className="card mono">
                  {state.chains.length === 0 ? (
                    <span className="muted">
                      Run a multi-step prompt like &ldquo;calculate 6 * 7&rdquo; to record a chain.
                    </span>
                  ) : (
                    state.chains.map((c) => (
                      <div key={c.id} style={{ marginBottom: 4 }}>
                        <span className={`badge ${c.chunked ? 'ok' : 'warn'}`}>
                          {c.chunked ? 'chunked' : 'ready'}
                        </span>{' '}
                        turn {c.turn}: &ldquo;{c.input}&rdquo; ({c.patternIds.join(' → ')})
                      </div>
                    ))
                  )}
                </div>
              </>
            )}

            {tab === 'tests' && (
              <>
                <div className="row">
                  <button className="btn btn-primary btn-sm" onClick={runSelfTests}>
                    Run Rust Self-Tests
                  </button>
                </div>
                {tests.length === 0 ? (
                  <div className="muted">Click &ldquo;Run Rust Self-Tests&rdquo; to verify.</div>
                ) : (
                  tests.map((t) => (
                    <div key={t.name} className="card">
                      <div className="card-head">
                        <span className="card-title">{t.name}</span>
                        <span className={`badge ${t.passed ? 'ok' : 'err'}`}>
                          {t.passed ? 'PASS' : 'FAIL'}
                        </span>
                      </div>
                      <div className="mono muted">{t.detail}</div>
                    </div>
                  ))
                )}
              </>
            )}
          </div>
        </section>
      </div>
    </div>
  );
}
