import { useState } from 'react';
import { api, fmtInt, pct, type Curriculum, type GridRow, type KindCount, type LiveEvent, type Run, type TrainState } from './api';

/* ---------- Run control ---------- */

export function Controls({ c }: { c: Curriculum | null }) {
  const [msg, setMsg] = useState('');
  const [teachers, setTeachers] = useState(16);
  const post = async (path: string, confirmText: string, body?: unknown) => {
    if (!window.confirm(confirmText)) return;
    try {
      await api(path, { method: 'POST', body: body ? JSON.stringify(body) : undefined, headers: { 'Content-Type': 'application/json' } });
      setMsg('');
    } catch (e) {
      setMsg(String((e as Error).message));
    }
  };
  return (
    <div className="controls">
      {c?.running ? (
        <>
          <button className="stop" onClick={() => post('/api/curriculum/stop', 'Stop the curriculum run?')}>Stop curriculum run</button>
          <span className="live-dot">● running</span>
        </>
      ) : (
        <>
          <label className="muted">
            teachers{' '}
            <input className="filter" type="number" min={1} max={24} value={teachers} onChange={(e) => setTeachers(Math.max(1, Math.min(24, Number(e.target.value) || 1)))} style={{ width: '4.5em' }} />
          </label>
          <button
            className="start"
            disabled={!c || c.mock}
            onClick={() =>
              post(
                '/api/curriculum/start',
                `Start the curriculum training run with ${teachers} parallel teacher(s)? It trains on the 55 training scenarios, then measures on the 20 held-out ones: about 2,000-2,500 Haiku calls.`,
                { teachers },
              )
            }
          >
            Start curriculum training
          </button>
        </>
      )}
      {c?.mock && <span className="muted">(mock models: starting is off)</span>}
      {msg && <span className="error">{msg}</span>}
    </div>
  );
}

/* ---------- Alerts ---------- */

export function Alerts({ run }: { run?: Run }) {
  if (!run?.alerts?.length) return null;
  return (
    <div role="alert" className="alerts">
      <strong>Alerts</strong>
      <ul>
        {run.alerts.map((a, i) => (
          <li key={i}>{a}</li>
        ))}
      </ul>
    </div>
  );
}

/* ---------- Progress tiles ---------- */

const TOTALS: Record<string, number> = { teach: 20, repeat: 20, transfer: 15, retention: 20 };
const ORDER = ['teach', 'teach_retry', 'repeat', 'transfer', 'transfer_retry', 'spaced_repeat', 'retention'];

export function Progress({ kinds, training }: { kinds: KindCount[]; training: boolean }) {
  const sorted = [...kinds].sort((a, b) => ORDER.indexOf(a.kind) - ORDER.indexOf(b.kind));
  return (
    <div className="tiles">
      {sorted.map((k) => {
        const total = training ? undefined : TOTALS[k.kind];
        return (
          <div className="tile" key={k.kind}>
            <div className="tile-label">{k.kind.replace('_', ' ')}</div>
            <div className="tile-value">
              {k.passed}/{k.done}
            </div>
            <div className="tile-sub">{pct(k.passed, k.done)}% passed{total ? ` · ${k.done} of ${total}` : ''}</div>
            <progress max={total || k.done || 1} value={k.done} />
          </div>
        );
      })}
    </div>
  );
}

/* ---------- Conversations: one lane each ---------- */

interface Lane {
  key: string;
  teacher: number;
  kind: string;
  scenario: string;
  enc: number;
  title: string;
  msgs: { who: string; text: string; stored?: string }[];
  verdict: string | null;
  t: number;
}

function lanesOf(recent: LiveEvent[]): Lane[] {
  // Parallel teachers interleave their messages: group by conversation, not by order.
  const byKey = new Map<string, Lane>();
  const lanes: Lane[] = [];
  for (const e of recent) {
    const key = e.enc ? `enc-${e.enc}` : `${e.kind}-${e.scenario}-${e.teacher ?? 0}`;
    let cur = byKey.get(key);
    if (!cur || (e.who === 'phase' && cur.msgs.length)) {
      cur = { key, teacher: e.teacher ?? 0, kind: e.kind, scenario: e.scenario, enc: e.enc, title: '', msgs: [], verdict: null, t: e.t };
      byKey.set(key, cur);
      lanes.push(cur);
    }
    if (e.who === 'phase') cur.title = e.text;
    else if (e.who === 'judge') cur.verdict = e.text;
    else if (e.who === 'stored') {
      const last = cur.msgs[cur.msgs.length - 1];
      if (last) last.stored = e.text;
    } else if (e.who !== 'vault') cur.msgs.push({ who: e.who, text: e.text });
    cur.t = e.t;
  }
  return lanes.filter((l) => l.msgs.length).sort((a, b) => b.t - a.t);
}

export function Lanes({ recent, max = 16 }: { recent: LiveEvent[]; max?: number }) {
  const lanes = lanesOf(recent).slice(0, max);
  const now = Date.now() / 1000;
  if (!lanes.length) return <p className="muted">Conversations appear here as they happen.</p>;
  return (
    <div className="lanes">
      {lanes.map((l, i) => {
        const live = now - l.t < 20;
        const who = l.teacher ? `T${l.teacher} · ` : '';
        const label = who + (l.enc ? `#${l.enc} · ${l.kind} · ${l.scenario}` : l.kind === 'teach_block' ? `teach block · ${l.scenario}` : l.kind);
        const pass = l.verdict?.startsWith('PASS');
        return (
          <div className="lane" key={`${l.key}-${i}`}>
            <div className="lane-head">
              <strong className="mono ellipsis">{label}</strong>
              <span>
                {live && <span className="live-dot">● live</span>}
                {l.verdict && <span className={pass ? 'pass' : 'fail'}> {pass ? 'PASS' : 'FAIL'}</span>}
              </span>
            </div>
            <div className="bubbles">
              {l.msgs.map((m, j) => (
                <div key={j} className={`bubble ${m.who}`}>
                  <div className="bubble-who">{m.who}</div>
                  {m.text}
                  {m.stored !== undefined && <div className="stored mono">stored: {m.stored || '(nothing)'}</div>}
                </div>
              ))}
            </div>
            {l.verdict && (
              <details className="small">
                <summary>judge</summary>
                {l.verdict}
              </details>
            )}
          </div>
        );
      })}
    </div>
  );
}

/* ---------- Scenario grid ---------- */

const GRID_COLS: [string, string][] = [
  ['teach', 'teach'],
  ['teach_retry', 'after teaching'],
  ['repeat', '2. repeat'],
  ['transfer', '3. transfer'],
  ['transfer_retry', 'transfer, taught'],
  ['spaced_repeat', 'spaced'],
  ['retention', '4. retention'],
];

export function ScenarioGrid({ grid }: { grid: GridRow[] }) {
  const [filter, setFilter] = useState('');
  if (!grid.length) return <p className="muted">The grid fills as encounters are judged.</p>;
  const cols = GRID_COLS.filter(([k]) => grid.some((r) => r.cells[k] !== undefined));
  const rows = grid.filter((r) => r.scenario.includes(filter));
  return (
    <div>
      <input className="filter" placeholder="filter scenarios" value={filter} onChange={(e) => setFilter(e.target.value)} />
      <div className="scroll-x">
        <table className="grid">
          <thead>
            <tr>
              <th className="left">Scenario</th>
              {cols.map(([k, label]) => (
                <th key={k}>{label}</th>
              ))}
            </tr>
          </thead>
          <tbody>
            {rows.map((r) => (
              <tr key={r.scenario}>
                <td className="mono left">{r.scenario}</td>
                {cols.map(([k]) => {
                  const v = r.cells[k];
                  return (
                    <td key={k} className={v === undefined ? 'muted' : v ? 'pass' : 'fail'} title={v === undefined ? 'not yet' : v ? 'solved' : 'failed'}>
                      {v === undefined ? '·' : v ? '✓' : '✗'}
                    </td>
                  );
                })}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}

/* ---------- Question stream, teach log, learning feed ---------- */

export function Questions({ run }: { run: Run }) {
  if (!run.questions.length) return <p className="muted">No questions yet.</p>;
  return (
    <div className="list">
      {[...run.questions].reverse().map((q, i) => (
        <details key={i}>
          <summary>{q.question}</summary>
          <div className="muted">teacher: {q.answer}</div>
          <div className="muted">agent: {q.reply}</div>
        </details>
      ))}
    </div>
  );
}

export function TeachLog({ run }: { run: Run }) {
  if (!run.teach.length) return <p className="muted">No teach blocks yet.</p>;
  return (
    <div className="list">
      {run.teach.map((b, i) => (
        <details key={i}>
          <summary>{b.title}</summary>
          <ul>
            {b.lines.map((l, j) => (
              <li key={j}>
                {l.text}
                <div className={`mono small ${l.stored ? 'muted' : 'fail'}`}>{l.stored || '(nothing stored)'}</div>
              </li>
            ))}
          </ul>
        </details>
      ))}
    </div>
  );
}

export function Feed({ feed }: { feed?: string[] }) {
  if (!feed?.length) return <p className="muted">Nothing learned yet.</p>;
  return (
    <div className="list mono small">
      {[...feed].reverse().map((l, i) => (
        <div key={i}>{l}</div>
      ))}
    </div>
  );
}

/* ---------- Health, teacher quality, plan limits ---------- */

export function HealthPanel({ c }: { c: Curriculum }) {
  const h = c.health;
  const q = c.run?.teacher_quality;
  return (
    <div className="kv">
      {h && (
        <>
          <div>Vault</div>
          <div className="mono">{h.vault}</div>
          <div>Procedures</div>
          <div>
            {fmtInt(h.procedures)} ({fmtInt(h.procedures_on_trial)} on trial)
          </div>
          <div>Words waiting for review</div>
          <div>{fmtInt(h.words_pending)}</div>
          <div>Merges queued</div>
          <div>{fmtInt(h.merges_queued)}</div>
          <div>Failed turns (for questions)</div>
          <div>{fmtInt(h.failed_turns)}</div>
        </>
      )}
      {q && (
        <>
          <div>Teach lines</div>
          <div>{fmtInt(q.teach_lines)}</div>
          <div>Facts stored per line</div>
          <div>{q.facts_per_line.toFixed(1)}</div>
          <div>Lines storing nothing</div>
          <div>{fmtInt(q.lines_storing_nothing)}</div>
          <div>Teacher meta requests</div>
          <div>{fmtInt(q.meta_requests)}</div>
        </>
      )}
    </div>
  );
}

export function Limits({ s }: { s: TrainState | null }) {
  if (!s) return null;
  return (
    <div className="kv">
      {s.limits.map((l) => (
        <div key={l.name} style={{ display: 'contents' }}>
          <div>{l.name}</div>
          <div>
            <progress max={100} value={l.percent} className={l.percent >= 80 ? 'hot' : ''} /> {l.percent.toFixed(0)}% <span className="muted">resets {l.resets}</span>
          </div>
        </div>
      ))}
      {s.limits_error && (
        <>
          <div>Limits</div>
          <div className="error">{s.limits_error}</div>
        </>
      )}
      <div>Model calls (this server)</div>
      <div>
        {fmtInt(s.usage_total.calls)} · {fmtInt(s.usage_total.input_tokens + s.usage_total.output_tokens)} tokens
      </div>
    </div>
  );
}
