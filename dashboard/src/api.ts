// Types and polling for the sla_trainer API (crates/eval/src/trainer.rs,
// crates/eval/src/curriculum.rs::dashboard_status).
import { useEffect, useState } from 'react';

export type Who = 'user' | 'agent' | 'teacher' | 'judge' | 'phase' | 'stored' | 'vault';

export interface LiveEvent {
  t: number;
  enc: number;
  kind: string;
  scenario: string;
  who: Who;
  text: string;
  /// Which parallel teacher (1..N); 0 or missing with one teacher.
  teacher?: number;
}

export interface KindCount {
  kind: string;
  done: number;
  passed: number;
}

export interface GridRow {
  scenario: string;
  cells: Record<string, boolean>;
}

export interface Question {
  t: number;
  question: string;
  answer: string;
  reply: string;
}

export interface TeachLine {
  text: string;
  reply: string;
  stored: string;
}

export interface TeachBlock {
  scenario: string;
  t: number;
  title: string;
  lines: TeachLine[];
}

export interface TeacherQuality {
  teach_lines: number;
  meta_requests: number;
  lines_storing_nothing: number;
  facts_per_line: number;
}

export interface Run {
  name: string;
  messages: number;
  last_t: number;
  kinds: KindCount[];
  grid: GridRow[];
  questions: Question[];
  teach: TeachBlock[];
  teacher_quality: TeacherQuality;
  alerts: string[];
  recent: LiveEvent[];
}

export interface Health {
  vault: string;
  procedures: number;
  procedures_on_trial: number;
  words_pending: number;
  merges_queued: number;
  failed_turns: number;
}

export interface Curriculum {
  run?: Run;
  health?: Health;
  feed?: string[];
  running: boolean;
  mock: boolean;
}

export interface Limit {
  name: string;
  percent: number;
  resets: string;
}

export interface Usage {
  calls: number;
  errors: number;
  seconds: number;
  input_tokens: number;
  output_tokens: number;
  cache_write_tokens: number;
  cache_read_tokens: number;
  cost_usd: number;
}

export interface TrainState {
  limits: Limit[];
  limits_checked: number | null;
  limits_error: string | null;
  usage: Record<string, Usage>;
  usage_total: Usage;
  vault_path: string;
}

/// The trainer prints an address with ?token=...; every call carries it.
export const TOKEN: string = (() => {
  const q = new URLSearchParams(location.search).get('token');
  try {
    if (q) sessionStorage.setItem('trainer-token', q);
    return q || sessionStorage.getItem('trainer-token') || '';
  } catch {
    return q || '';
  }
})();

export async function api<T>(path: string, init: RequestInit = {}): Promise<T> {
  const r = await fetch(path, { cache: 'no-store', ...init, headers: { ...(init.headers || {}), 'X-Trainer-Token': TOKEN } });
  const body = await r.json().catch(() => ({}));
  if (!r.ok) throw new Error((body as { error?: string }).error || `${r.status}`);
  return body as T;
}

/// Poll a GET endpoint every `ms`; returns the latest value and the last error.
export function usePoll<T>(path: string, ms: number): [T | null, string | null] {
  const [data, setData] = useState<T | null>(null);
  const [err, setErr] = useState<string | null>(null);
  useEffect(() => {
    let alive = true;
    let timer: ReturnType<typeof setTimeout>;
    const tick = async () => {
      try {
        const d = await api<T>(path);
        if (alive) {
          setData(d);
          setErr(null);
        }
      } catch (e) {
        if (alive) setErr(String((e as Error).message || e));
      }
      if (alive) timer = setTimeout(tick, ms);
    };
    tick();
    return () => {
      alive = false;
      clearTimeout(timer);
    };
  }, [path, ms]);
  return [data, err];
}

export const pct = (a: number, b: number) => (b ? Math.round((a / b) * 100) : 0);
export const fmtInt = (n: number | undefined | null) => (n ?? 0).toLocaleString();
