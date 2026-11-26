import { useState } from 'react';
import { fmtAction, fmtCond } from './format';
import type { Fact, Pattern } from './types';

export function FactRow({ fact, onDelete }: { fact: Fact; onDelete: () => void }) {
  return (
    <div className="ex-row fact-row" title={`${fact.id} · ${fact.provenance}`}>
      <span className="fact-args">
        {Object.entries(fact.args).map(([k, v]) => (
          <span key={k} className="arg">
            <span className="arg-k">{k}</span>
            <span className="arg-v">{v}</span>
          </span>
        ))}
      </span>
      <span className="row-meta">
        {fact.scope !== 'long_term' && (
          <span className={`badge ${fact.scope === 'chat' ? 'ok' : 'warn'}`}>{fact.scope}</span>
        )}
        {fact.turn > 0 && <span className="muted mono">t{fact.turn}</span>}
        <button className="row-del" title="Delete fact" onClick={onDelete}>
          ×
        </button>
      </span>
    </div>
  );
}

export function PatternRow({
  pattern,
  onDelete,
  onPromote,
}: {
  pattern: Pattern;
  onDelete: () => void;
  onPromote: () => void;
}) {
  const [open, setOpen] = useState(false);
  const p = pattern;
  return (
    <div className={`ex-row pattern-row ${open ? 'open' : ''}`}>
      <div className="pattern-top" onClick={() => setOpen((o) => !o)}>
        <span className="pattern-name">{p.name}</span>
        <span className="muted mono">{p.id}</span>
        <span className="row-meta">
          <span className="badge">{p.kind}</span>
          {p.status === 'candidate' && <span className="badge warn">candidate</span>}
          {p.status === 'candidate' && (
            <button
              className="btn btn-primary btn-sm"
              onClick={(e) => {
                e.stopPropagation();
                onPromote();
              }}
            >
              Promote
            </button>
          )}
          <button
            className="row-del"
            title="Delete pattern"
            onClick={(e) => {
              e.stopPropagation();
              onDelete();
            }}
          >
            ×
          </button>
        </span>
      </div>
      {open ? (
        <div className="pattern-full mono">
          <div className="rule-kw">when</div>
          {p.when.map((c, i) => (
            <div key={i} className="rule-line">
              {fmtCond(c)}
            </div>
          ))}
          <div className="rule-kw">then</div>
          {p.then.map((a, i) => (
            <div key={i} className="rule-line">
              {fmtAction(a)}
            </div>
          ))}
          <div className="muted pattern-stats">
            trust {p.trust.toFixed(2)} · support {p.support} · {p.provenance}
          </div>
        </div>
      ) : (
        <div className="pattern-summary mono" onClick={() => setOpen(true)}>
          <span className="rule-kw">when</span>
          <span className="clip">{p.when.map(fmtCond).join(' ∧ ')}</span>
          <span className="rule-kw">then</span>
          <span className="clip">{p.then.map(fmtAction).join(' ; ')}</span>
        </div>
      )}
    </div>
  );
}
