import type { Action, Condition } from './types';

const args = (a: Record<string, string>) =>
  Object.entries(a)
    .map(([k, v]) => `${k}=${v}`)
    .join(', ');

export function fmtCond(c: Condition): string {
  switch (c.kind) {
    case 'text':
      return `text("${c.template}")`;
    case 'contains':
      return `contains("${c.phrase}")`;
    case 'fact':
      return `${c.rel}(${args(c.args)})`;
    case 'not_fact':
      return `¬${c.rel}(${args(c.args)})`;
    case 'not_eq':
      return `${c.left} ≠ ${c.right}`;
    case 'word_count':
      return `words(${c.var}) ${c.op} ${c.n}`;
  }
}

export function fmtAction(a: Action): string {
  switch (a.kind) {
    case 'assert':
      return `assert[${a.scope === 'long_term' ? 'lt' : a.scope === 'chat' ? 'chat' : 'w'}] ${a.rel}(${args(a.args)})`;
    case 'retract':
      return `retract ${a.rel}`;
    case 'tool':
      return `tool:${a.name}(${args(a.args)})`;
    case 'reply':
      return `reply "${a.template}"`;
  }
}

/** "vault:generated-claude-opus-5.5-nlu-2026-11-26/people" → "generated-…/people". */
export function shortSource(provenance: string): string {
  const p = provenance.replace(/^vault:/, '');
  const parts = p.split('/');
  if (parts.length > 1 && parts[0].length > 24) {
    return `${parts[0].slice(0, 22)}…/${parts.slice(1).join('/')}`;
  }
  return p;
}
