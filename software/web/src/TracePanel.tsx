import { fmtAction, fmtCond, shortSource } from './format';
import { FailedIcon, UnusedIcon, UsedIcon } from './Icons';
import type { AttemptTrace, CompetingMatch, Fact, Pattern, StepTrace, TurnTrace } from './types';

/** Relations shown collapsed by default (long, mostly for "repeat"). */
const COLLAPSED_RELATIONS = new Set(['said']);

function fmtArgs(args: Record<string, string>) {
  return Object.entries(args)
    .map(([k, v]) => `${k}="${v}"`)
    .join(', ');
}

function ChatMemory({ facts, turn }: { facts: Fact[]; turn: number }) {
  if (facts.length === 0) {
    return (
      <div className="card">
        <div className="section-label">Chat memory after this turn</div>
        <div className="muted" style={{ marginTop: 6 }}>
          Empty. Facts with chat scope (<span className="mono">+chat …</span> or a relation declared
          with <span className="mono">chat_memory</span>) appear here and are cleared by New chat.
        </div>
      </div>
    );
  }
  // Group by relation, newest first inside each group.
  const groups = new Map<string, Fact[]>();
  for (const f of facts) {
    groups.set(f.rel, [...(groups.get(f.rel) ?? []), f]);
  }
  const fresh = facts.filter((f) => f.turn === turn).length;

  return (
    <div className="card">
      <div className="card-head">
        <span className="section-label">Chat memory after this turn</span>
        <span className="muted">
          {facts.length} facts{fresh > 0 && ` · ${fresh} new this turn`}
        </span>
      </div>
      <div className="memory-groups">
        {[...groups.entries()].map(([rel, list]) => {
          const items = [...list].reverse();
          const body = (
            <div className="memory-list">
              {items.map((f) => (
                <div key={f.id} className={`memory-fact ${f.turn === turn ? 'fresh' : ''}`}>
                  <span className="mono">{fmtArgs(f.args)}</span>
                  <span className="muted mono">
                    {f.turn === turn ? 'this turn' : `turn ${f.turn}`}
                  </span>
                </div>
              ))}
            </div>
          );
          return COLLAPSED_RELATIONS.has(rel) ? (
            <details key={rel} className="trace-details">
              <summary>
                <span className="mono">{rel}</span> ({list.length})
              </summary>
              {body}
            </details>
          ) : (
            <div key={rel}>
              <div className="memory-rel">
                <span className="mono">{rel}</span> <span className="muted">({list.length})</span>
              </div>
              {body}
            </div>
          );
        })}
      </div>
    </div>
  );
}

const ATTEMPT_LABEL: Record<AttemptTrace['kind'], string> = {
  whole: 'Whole sentence',
  clause: 'Clause',
  ellipsis: 'Clause + earlier words',
  exact: 'Exact wording',
};

const SCOPE_OF_EFFECT: [string, string, string][] = [
  // prefix, css class, label
  ['++', 'fx-long', 'long-term'],
  ['+chat ', 'fx-chat', 'chat'],
  ['+', 'fx-working', 'working'],
  ['-', 'fx-retract', 'retract'],
  ['@', 'fx-tool', 'tool'],
  ['=>', 'fx-reply', 'reply'],
];

function effectKind(effect: string): [string, string, string] {
  for (const [prefix, cls, label] of SCOPE_OF_EFFECT) {
    if (effect.startsWith(prefix)) {
      return [cls, label, effect.slice(prefix.length).trim()];
    }
  }
  return ['fx-working', '', effect];
}

function round(n: number) {
  return Math.round(n * 100) / 100;
}

function ScoreParts({ match }: { match: CompetingMatch }) {
  if (!match.parts || match.parts.length === 0) {
    return <span className="mono">{round(match.score)}</span>;
  }
  return (
    <span className="score-parts mono">
      {match.parts.map((p, i) => (
        <span key={p.label}>
          {i > 0 && <span className="score-op">+</span>}
          <span className="score-part">
            <span className="score-label">{p.label}</span> {round(p.value)}
          </span>
        </span>
      ))}
      <span className="score-op">=</span>
      <strong>{round(match.score)}</strong>
    </span>
  );
}

function StepCard({ step, pattern }: { step: StepTrace; pattern?: Pattern }) {
  const winner = step.competing.find((c) => c.patternId === step.winnerId) ?? step.competing[0];
  const runnerUp = step.competing.find((c) => c.patternId !== step.winnerId);
  const margin = winner && runnerUp ? round(winner.score - runnerUp.score) : null;
  const close = margin !== null && margin < 1;

  return (
    <div className="card step-card">
      <div className="card-head">
        <span className="card-title">
          <span className="step-no">{step.step}</span>
          {step.winnerName}
        </span>
        <span className="row" style={{ gap: 6 }}>
          {pattern && <span className="badge">{pattern.kind}</span>}
          {pattern?.status === 'candidate' && <span className="badge warn">candidate</span>}
          <span className="muted mono">{step.winnerId}</span>
        </span>
      </div>

      {pattern && (
        <div className="trace-source muted" title={pattern.provenance}>
          from <span className="mono">{shortSource(pattern.provenance)}</span> · trust{' '}
          {pattern.trust.toFixed(2)} · support {pattern.support}
        </div>
      )}

      {winner && (
        <div className="trace-why">
          <span className="trace-label">Why it won</span>
          <ScoreParts match={winner} />
          {runnerUp && (
            <span className={`muted ${close ? 'close-call' : ''}`}>
              {margin! >= 0 ? `+${margin}` : margin} over {runnerUp.name}
              {close && ' - close call'}
            </span>
          )}
          {!runnerUp && <span className="muted">only matching rule</span>}
        </div>
      )}

      {Object.keys(step.bindings).length > 0 && (
        <div className="bindings">
          {Object.entries(step.bindings).map(([k, v]) => (
            <div key={k} className="binding">
              <span className="mono binding-var">{k}</span>
              <span className="mono binding-val">{v}</span>
            </div>
          ))}
        </div>
      )}

      <div className="effects">
        {step.effects.map((ef, i) => {
          const [cls, label, body] = effectKind(ef);
          return (
            <div key={i} className={`effect ${cls}`}>
              {label && <span className="effect-tag">{label}</span>}
              <span className="mono">{body}</span>
            </div>
          );
        })}
      </div>

      {pattern && (
        <details className="trace-details">
          <summary>Rule</summary>
          <div className="rule-block mono">
            <div>
              <span className="rule-kw">when</span>
            </div>
            {pattern.when.map((c, i) => (
              <div key={i} className="rule-line">
                {fmtCond(c)}
              </div>
            ))}
            <div>
              <span className="rule-kw">then</span>
            </div>
            {pattern.then.map((a, i) => (
              <div key={i} className="rule-line">
                {fmtAction(a)}
              </div>
            ))}
          </div>
        </details>
      )}

      {step.competing.length > 1 && (
        <details className="trace-details" open={close}>
          <summary>Competing rules ({step.competing.length})</summary>
          <table className="compete-table">
            <tbody>
              {step.competing.map((c, i) => (
                <tr key={c.patternId} className={c.patternId === step.winnerId ? 'winner' : ''}>
                  <td className="muted">{i + 1}</td>
                  <td>
                    {c.name} <span className="muted mono">{c.patternId}</span>
                  </td>
                  <td>
                    <ScoreParts match={c} />
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </details>
      )}
    </div>
  );
}

export function TracePanel({ trace, patterns }: { trace: TurnTrace; patterns: Pattern[] }) {
  const byId = new Map(patterns.map((p) => [p.id, p]));
  const understood = trace.understood ?? [];
  const missed = trace.missed ?? [];
  const attempts = trace.attempts ?? [];
  const status = trace.impasse
    ? { label: 'Impasse', cls: 'warn' }
    : missed.length > 0
      ? { label: 'Partly understood', cls: 'warn' }
      : { label: 'Understood', cls: 'ok' };

  // Steps grouped by the clause they ran on (in order of first appearance).
  const groups: { text: string; steps: StepTrace[] }[] = [];
  for (const s of trace.steps) {
    const text = s.text ?? '';
    const last = groups[groups.length - 1];
    if (last && last.text === text) last.steps.push(s);
    else groups.push({ text, steps: [s] });
  }

  return (
    <>
      <div className="card">
        <div className="card-head">
          <span className="card-title">Turn {trace.turn}</span>
          <span className="row" style={{ gap: 6 }}>
            <span className="muted">
              {trace.steps.length} {trace.steps.length === 1 ? 'step' : 'steps'} ·{' '}
              {new Set(trace.firedPatternIds).size} rules
            </span>
            <span className={`badge ${status.cls}`}>{status.label}</span>
          </span>
        </div>
        <div className="trace-grid">
          <span className="trace-label">Heard</span>
          <span className="mono">{trace.input}</span>
          {understood.length > 0 && (
            <>
              <span className="trace-label">Understood as</span>
              <span className="chip-row">
                {understood.map((u, i) => (
                  <span key={i} className="clause-chip ok mono">
                    {u}
                  </span>
                ))}
              </span>
            </>
          )}
          {missed.length > 0 && (
            <>
              <span className="trace-label">Missed</span>
              <span className="chip-row">
                {missed.map((m, i) => (
                  <span key={i} className="clause-chip warn mono">
                    {m}
                  </span>
                ))}
              </span>
            </>
          )}
          <span className="trace-label">Reply</span>
          <span>{trace.reply}</span>
          {trace.chunkedPatternId && (
            <>
              <span className="trace-label">Learned</span>
              <span className="mono">chunked into {trace.chunkedPatternId}</span>
            </>
          )}
        </div>
      </div>

      {attempts.length > 0 && (
        <div className="card">
          <div className="section-label">Readings tried</div>
          <ol className="attempts">
            {attempts.map((a, i) => {
              const used = a.answered && understood.includes(a.text);
              const state = used ? 'answered' : a.answered ? 'unused' : 'failed';
              return (
                <li key={i} className={state}>
                  <span className="attempt-mark" title={used ? 'used' : a.answered ? 'matched, not used' : 'no match'}>
                    {used ? <UsedIcon /> : a.answered ? <UnusedIcon /> : <FailedIcon />}
                  </span>
                  <span className="badge">{ATTEMPT_LABEL[a.kind] ?? a.kind}</span>
                  <span className="mono">{a.text}</span>
                  {state === 'unused' && <span className="muted">matched, but not used</span>}
                </li>
              );
            })}
          </ol>
        </div>
      )}

      {groups.map((g, gi) => (
        <div key={gi} className="clause-group">
          {(groups.length > 1 || (g.text && g.text !== trace.input)) && (
            <div className="clause-head">
              <span className="trace-label">Clause</span>
              <span className="mono">{g.text}</span>
            </div>
          )}
          {g.steps.map((s) => (
            <StepCard key={s.step} step={s} pattern={byId.get(s.winnerId)} />
          ))}
        </div>
      ))}

      {trace.steps.length === 0 && (
        <div className="muted">
          No rule matched any reading. Teach a reply below the chat, or add a lexicon entry so
          this wording maps to one Karin knows.
        </div>
      )}

      {trace.chatMemory !== undefined && (
        <ChatMemory facts={trace.chatMemory} turn={trace.turn} />
      )}
    </>
  );
}
