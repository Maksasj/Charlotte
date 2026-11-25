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

const QUICK_PROMPTS = [
  'who are you',
  'my name is Alex',
  'hello',
  'weather in Paris',
  'weather',
  'calculate (12 + 8) * 3',
  'define anti-unification',
  'remember buy coffee',
  'my notes',
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
      return `fact ${c.rel}(${Object.entries(c.args)
        .map(([k, v]) => `${k}=${v}`)
        .join(', ')})`;
    case 'not_fact':
      return `¬fact ${c.rel}(${Object.entries(c.args)
        .map(([k, v]) => `${k}=${v}`)
        .join(', ')})`;
  }
}

function fmtAction(a: Action): string {
  switch (a.kind) {
    case 'assert':
      return `assert[${a.scope}] ${a.rel}(${Object.entries(a.args)
        .map(([k, v]) => `${k}=${v}`)
        .join(', ')})`;
    case 'retract':
      return `retract ${a.rel}`;
    case 'tool':
      return `tool ${a.name}(${Object.entries(a.args)
        .map(([k, v]) => `${k}=${v}`)
        .join(', ')})`;
    case 'reply':
      return `reply "${a.template}"`;
  }
}

export default function App() {
  const [state, setState] = useState<AgentState | null>(null);
  const [tools, setTools] = useState<ToolInfo[]>([]);
  const [tab, setTab] = useState<Tab>('trace');
  const [input, setInput] = useState('');
  const [selectedTrace, setSelectedTrace] = useState<TurnTrace | null>(null);
  const [statusMsg, setStatusMsg] = useState<string | null>(null);
  const [errorMsg, setErrorMsg] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  // Impasse teach form
  const [teachReply, setTeachReply] = useState('');
  const [teachPromote, setTeachPromote] = useState(true);

  // Add fact form
  const [factRel, setFactRel] = useState('weather_city');
  const [factArgsText, setFactArgsText] = useState('city=Berlin, condition=sunny, temp=21°C');
  const [factScope, setFactScope] = useState<Scope>('long_term');

  // Self-test results
  const [tests, setTests] = useState<SelfTestResult[]>([]);

  const messagesEndRef = useRef<HTMLDivElement>(null);

  async function loadInitial() {
    try {
      const data = await api.getState();
      setState(data.state);
      setTools(data.tools);
      const lastTrace = [...data.state.messages]
        .reverse()
        .find((m) => m.trace)?.trace;
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
        `Taught "${impasseInput}" (${teachPromote ? 'active' : 'candidate'}). Try sending it again!`
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
    setStatusMsg('Reset to clean pretrained seed state.');
  }

  const lastAgentMsg = state
    ? [...state.messages].reverse().find((m) => m.role === 'agent')
    : undefined;
  const activeImpasse = lastAgentMsg?.trace?.impasse ? lastAgentMsg.trace : null;

  const candidateCount =
    state?.patterns.filter((p) => p.status === 'candidate').length ?? 0;

  return (
    <div className="app">
      <header className="topbar">
        <div className="brand">
          <h1>Karin</h1>
          <span className="pill ok">Rust Engine</span>
          {state && (
            <span className="muted">
              {state.facts.length} facts · {state.patterns.length} patterns · turn {state.turn}
            </span>
          )}
        </div>
        <div className="topbar-actions">
          <button className="secondary" onClick={runSelfTests}>
            Run Self-Tests
          </button>
          <button className="secondary" onClick={handleReset}>
            Reset Seed
          </button>
        </div>
      </header>

      {errorMsg && <div className="banner err">{errorMsg}</div>}
      {statusMsg && <div className="banner">{statusMsg}</div>}

      <div className="main-grid">
        {/* Left: Chat */}
        <section className="chat-pane">
          <div className="quick-bar">
            {QUICK_PROMPTS.map((q) => (
              <button key={q} className="quick-btn" onClick={() => sendMessage(q)}>
                {q}
              </button>
            ))}
          </div>

          <div className="messages">
            {!state || state.messages.length === 0 ? (
              <div className="empty-hint">
                <strong>Uniform Facts + Patterns Core (Rust + React)</strong>
                <br />
                Click any prompt above or type a message. Try <code>weather</code> (multi-turn
                slot filling), <code>calculate 6 * 7</code> then <code>chunk last</code>, or{' '}
                <code>generalize</code> to anti-unify the seed episodes.
              </div>
            ) : (
              state.messages.map((m) => (
                <div
                  key={m.id}
                  className={`msg ${m.role} ${m.trace?.impasse ? 'impasse' : ''}`}
                >
                  <div>{m.text}</div>
                  {m.role === 'agent' && m.trace && (
                    <div className="msg-meta">
                      <span>
                        {m.trace.impasse
                          ? 'impasse'
                          : `${m.trace.steps.length} step${m.trace.steps.length === 1 ? '' : 's'}`}
                      </span>
                      <button
                        className="link-btn"
                        onClick={() => {
                          setSelectedTrace(m.trace!);
                          setTab('trace');
                        }}
                      >
                        inspect trace
                      </button>
                      <button
                        className={`fb-btn ${m.feedback === 'up' ? 'active' : ''}`}
                        onClick={async () => {
                          const r = await api.feedback(m.turn, true);
                          setState(r.state);
                        }}
                      >
                        +1
                      </button>
                      <button
                        className={`fb-btn ${m.feedback === 'down' ? 'active' : ''}`}
                        onClick={async () => {
                          const r = await api.feedback(m.turn, false);
                          setState(r.state);
                        }}
                      >
                        -1
                      </button>
                    </div>
                  )}
                </div>
              ))
            )}
            <div ref={messagesEndRef} />
          </div>

          {activeImpasse && (
            <div className="teach-box">
              <h4>Impasse on &ldquo;{activeImpasse.input}&rdquo; — teach a reply:</h4>
              <div className="row">
                <input
                  type="text"
                  style={{ flex: 1 }}
                  placeholder="Type the reply Karin should give..."
                  value={teachReply}
                  onChange={(e) => setTeachReply(e.target.value)}
                />
                <label className="muted row" style={{ gap: 4 }}>
                  <input
                    type="checkbox"
                    checked={teachPromote}
                    onChange={(e) => setTeachPromote(e.target.checked)}
                  />
                  auto-promote
                </label>
                <button
                  className="primary"
                  onClick={() => handleTeach(activeImpasse.input)}
                  disabled={busy}
                >
                  Teach
                </button>
              </div>
            </div>
          )}

          <form
            className="chat-form"
            onSubmit={(e) => {
              e.preventDefault();
              sendMessage(input);
            }}
          >
            <input
              type="text"
              placeholder="Say something to Karin..."
              value={input}
              onChange={(e) => setInput(e.target.value)}
            />
            <button type="submit" className="primary" disabled={busy}>
              Send
            </button>
          </form>
        </section>

        {/* Right: Inspector */}
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
              Facts ({state?.facts.length ?? 0})
            </button>
            <button
              className={`tab ${tab === 'patterns' ? 'active' : ''}`}
              onClick={() => setTab('patterns')}
            >
              Patterns ({state?.patterns.length ?? 0}
              {candidateCount > 0 ? ` · ${candidateCount} pending` : ''})
            </button>
            <button
              className={`tab ${tab === 'tools' ? 'active' : ''}`}
              onClick={() => setTab('tools')}
            >
              Tools &amp; Meta
            </button>
            <button
              className={`tab ${tab === 'tests' ? 'active' : ''}`}
              onClick={() => setTab('tests')}
            >
              Self-Tests
            </button>
          </div>

          <div className="inspector-body">
            {tab === 'trace' && (
              <>
                {!selectedTrace ? (
                  <div className="muted">Send a message to inspect its symbolic execution trace.</div>
                ) : (
                  <>
                    <div className="card">
                      <div className="card-header">
                        <span className="card-title">
                          Turn {selectedTrace.turn}: &ldquo;{selectedTrace.input}&rdquo;
                        </span>
                        <span className={`pill ${selectedTrace.impasse ? 'warn' : 'ok'}`}>
                          {selectedTrace.impasse
                            ? 'IMPASSE'
                            : `${selectedTrace.steps.length} steps`}
                        </span>
                      </div>
                      <div className="muted">Reply: {selectedTrace.reply}</div>
                    </div>

                    {selectedTrace.steps.map((s) => (
                      <div key={s.step} className="card">
                        <div className="card-header">
                          <span className="card-title">
                            Step {s.step}: {s.winnerName}{' '}
                            <span className="muted mono">({s.winnerId})</span>
                          </span>
                        </div>
                        {Object.keys(s.bindings).length > 0 && (
                          <div className="mono muted" style={{ marginBottom: 6 }}>
                            bindings: {JSON.stringify(s.bindings)}
                          </div>
                        )}
                        <div className="mono" style={{ marginBottom: 6 }}>
                          {s.effects.map((ef, i) => (
                            <div key={i}>→ {ef}</div>
                          ))}
                        </div>
                        {s.competing.length > 1 && (
                          <div className="muted">
                            Competing:{' '}
                            {s.competing
                              .map((c) => `${c.name} (${c.score.toFixed(2)})`)
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
                  <div className="card-title" style={{ marginBottom: 8 }}>
                    Add Fact
                  </div>
                  <div className="row">
                    <input
                      type="text"
                      value={factRel}
                      onChange={(e) => setFactRel(e.target.value)}
                      placeholder="relation"
                      style={{ width: 140 }}
                    />
                    <input
                      type="text"
                      value={factArgsText}
                      onChange={(e) => setFactArgsText(e.target.value)}
                      placeholder="k=v, k2=v2"
                      style={{ flex: 1 }}
                    />
                    <select
                      value={factScope}
                      onChange={(e) => setFactScope(e.target.value as Scope)}
                    >
                      <option value="long_term">long_term</option>
                      <option value="working">working</option>
                    </select>
                    <button type="submit" className="primary">
                      Add
                    </button>
                  </div>
                </form>

                {[...state.facts].reverse().map((f) => (
                  <div key={f.id} className="card">
                    <div className="card-header">
                      <span className="mono">
                        <strong>{f.rel}</strong>(
                        {Object.entries(f.args)
                          .map(([k, v]) => `${k}="${v}"`)
                          .join(', ')}
                        )
                      </span>
                      <div className="row">
                        <span className={`pill ${f.scope === 'working' ? 'warn' : ''}`}>
                          {f.scope}
                        </span>
                        <button
                          className="danger"
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
                      {f.id} · provenance: {f.provenance} · turn {f.turn}
                    </div>
                  </div>
                ))}
              </>
            )}

            {tab === 'patterns' && state && (
              <>
                {[...state.patterns]
                  .sort((a, b) => (a.status === 'candidate' ? -1 : b.status === 'candidate' ? 1 : 0))
                  .map((p) => (
                    <div key={p.id} className="card">
                      <div className="card-header">
                        <span className="card-title">
                          {p.name} <span className="muted mono">({p.id})</span>
                        </span>
                        <div className="row">
                          <span className="pill">{p.kind}</span>
                          <span className={`pill ${p.status === 'candidate' ? 'warn' : 'ok'}`}>
                            {p.status}
                          </span>
                          {p.status === 'candidate' && (
                            <button
                              className="secondary"
                              onClick={async () => {
                                const r = await api.promotePattern(p.id);
                                setState(r.state);
                              }}
                            >
                              Promote
                            </button>
                          )}
                          <button
                            className="danger"
                            onClick={async () => {
                              const r = await api.deletePattern(p.id);
                              setState(r.state);
                            }}
                          >
                            ×
                          </button>
                        </div>
                      </div>
                      <div className="mono" style={{ marginBottom: 4 }}>
                        <div>
                          <span className="muted">when: </span>
                          {p.when.map(fmtCond).join(' ∧ ')}
                        </div>
                        <div>
                          <span className="muted">then: </span>
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
                  <div className="card-title" style={{ marginBottom: 8 }}>
                    Meta-Tool Actions
                  </div>
                  <div className="row">
                    <button className="primary" onClick={() => runGeneralize(false)}>
                      Anti-Unify Episodes (Candidate)
                    </button>
                    <button className="secondary" onClick={() => runGeneralize(true)}>
                      Anti-Unify &amp; Promote
                    </button>
                    <button className="primary" onClick={() => runChunk(false)}>
                      Chunk Last Chain (Candidate)
                    </button>
                    <button className="secondary" onClick={() => runChunk(true)}>
                      Chunk &amp; Promote
                    </button>
                  </div>
                </div>

                <div className="card">
                  <div className="card-title" style={{ marginBottom: 6 }}>
                    Registered Tools ({tools.length})
                  </div>
                  {tools.map((t) => (
                    <div key={t.name} style={{ marginBottom: 6 }}>
                      <span className="mono">
                        <strong>{t.name}</strong>
                      </span>{' '}
                      <span className={`pill ${t.meta ? 'warn' : ''}`}>
                        {t.meta ? 'meta' : 'world'}
                      </span>
                      <div className="muted">{t.description}</div>
                    </div>
                  ))}
                </div>

                <div className="card">
                  <div className="card-title" style={{ marginBottom: 6 }}>
                    Episodes ({state.episodes.length})
                  </div>
                  {state.episodes.map((ep) => (
                    <div key={ep.id} className="mono" style={{ marginBottom: 4 }}>
                      [{ep.generalized ? 'done' : 'raw'}] &ldquo;{ep.input}&rdquo; → &ldquo;
                      {ep.reply}&rdquo;
                    </div>
                  ))}
                </div>

                <div className="card">
                  <div className="card-title" style={{ marginBottom: 6 }}>
                    Recorded Multi-Step Chains ({state.chains.length})
                  </div>
                  {state.chains.length === 0 ? (
                    <div className="muted">
                      Run a multi-step query like &ldquo;calculate 6 * 7&rdquo; or &ldquo;weather
                      in Paris&rdquo; to record a chain.
                    </div>
                  ) : (
                    state.chains.map((c) => (
                      <div key={c.id} className="mono" style={{ marginBottom: 4 }}>
                        [{c.chunked ? 'chunked' : 'ready'}] turn {c.turn}: &ldquo;{c.input}&rdquo;{' '}
                        ({c.patternIds.join(' → ')})
                      </div>
                    ))
                  )}
                </div>
              </>
            )}

            {tab === 'tests' && (
              <>
                <div className="row">
                  <button className="primary" onClick={runSelfTests}>
                    Re-run Rust Self-Tests
                  </button>
                </div>
                {tests.length === 0 ? (
                  <div className="muted">Click &ldquo;Run Self-Tests&rdquo; to verify the Rust engine.</div>
                ) : (
                  tests.map((t) => (
                    <div key={t.name} className="card">
                      <div className="card-header">
                        <span className="card-title">{t.name}</span>
                        <span className={`pill ${t.passed ? 'ok' : 'err'}`}>
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
