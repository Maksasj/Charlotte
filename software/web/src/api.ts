import type { AgentState, Scope, SelfTestResult, ToolInfo, TurnTrace } from './types';

async function req<T>(path: string, init?: RequestInit): Promise<T> {
  const res = await fetch(path, {
    headers: { 'Content-Type': 'application/json' },
    ...init,
  });
  if (!res.ok) {
    throw new Error(`${res.status} ${res.statusText}`);
  }
  return res.json() as Promise<T>;
}

export const api = {
  getState: () => req<{ state: AgentState; tools: ToolInfo[] }>('/api/state'),
  chat: (input: string) =>
    req<{ trace: TurnTrace; state: AgentState }>('/api/chat', {
      method: 'POST',
      body: JSON.stringify({ input }),
    }),
  newChat: () => req<{ state: AgentState }>('/api/chat/new', { method: 'POST' }),
  teach: (input: string, reply: string, promote = true) =>
    req<{ state: AgentState }>('/api/teach', {
      method: 'POST',
      body: JSON.stringify({ input, reply, promote }),
    }),
  feedback: (turn: number, positive: boolean) =>
    req<{ state: AgentState }>('/api/feedback', {
      method: 'POST',
      body: JSON.stringify({ turn, positive }),
    }),
  addFact: (rel: string, args: Record<string, string>, scope: Scope = 'long_term') =>
    req<{ state: AgentState }>('/api/facts', {
      method: 'POST',
      body: JSON.stringify({ rel, args, scope }),
    }),
  deleteFact: (id: string) =>
    req<{ state: AgentState }>(`/api/facts/${encodeURIComponent(id)}`, { method: 'DELETE' }),
  promotePattern: (id: string) =>
    req<{ state: AgentState }>(`/api/patterns/${encodeURIComponent(id)}/promote`, {
      method: 'POST',
    }),
  deletePattern: (id: string) =>
    req<{ state: AgentState }>(`/api/patterns/${encodeURIComponent(id)}`, { method: 'DELETE' }),
  generalize: (autoPromote = false) =>
    req<{ summary: string; state: AgentState }>('/api/meta/generalize', {
      method: 'POST',
      body: JSON.stringify({ autoPromote }),
    }),
  chunk: (autoPromote = false) =>
    req<{ summary: string; state: AgentState }>('/api/meta/chunk', {
      method: 'POST',
      body: JSON.stringify({ autoPromote }),
    }),
  reset: () => req<{ state: AgentState }>('/api/reset', { method: 'POST' }),
  selftest: () => req<SelfTestResult[]>('/api/selftest'),
};
