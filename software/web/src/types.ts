export type Scope = 'working' | 'long_term';

export interface Fact {
  id: string;
  rel: string;
  args: Record<string, string>;
  scope: Scope;
  provenance: string;
  turn: number;
}

export type Condition =
  | { kind: 'text'; template: string }
  | { kind: 'contains'; phrase: string }
  | { kind: 'fact'; rel: string; args: Record<string, string> }
  | { kind: 'not_fact'; rel: string; args: Record<string, string> };

export type Action =
  | { kind: 'assert'; rel: string; args: Record<string, string>; scope: Scope }
  | { kind: 'retract'; rel: string; args?: Record<string, string> }
  | { kind: 'tool'; name: string; args: Record<string, string> }
  | { kind: 'reply'; template: string };

export interface Pattern {
  id: string;
  name: string;
  status: 'active' | 'candidate';
  when: Condition[];
  then: Action[];
  trust: number;
  support: number;
  provenance: string;
  kind: 'language' | 'reasoning' | 'reply' | 'composite' | 'induced';
}

export interface Episode {
  id: string;
  input: string;
  reply: string;
  source: string;
  generalized: boolean;
}

export interface CompetingMatch {
  patternId: string;
  name: string;
  score: number;
  bindings: Record<string, string>;
}

export interface StepTrace {
  step: number;
  competing: CompetingMatch[];
  winnerId: string;
  winnerName: string;
  bindings: Record<string, string>;
  effects: string[];
}

export interface TurnTrace {
  turn: number;
  input: string;
  reply: string;
  impasse: boolean;
  steps: StepTrace[];
  firedPatternIds: string[];
  chunkedPatternId?: string | null;
}

export interface ChatMessage {
  id: string;
  role: 'user' | 'agent' | 'system';
  text: string;
  turn: number;
  trace?: TurnTrace | null;
  feedback?: 'up' | 'down' | null;
}

export interface ChainRecord {
  id: string;
  turn: number;
  input: string;
  patternIds: string[];
  bindings: Record<string, string>;
  chunked: boolean;
}

export interface AgentState {
  turn: number;
  facts: Fact[];
  patterns: Pattern[];
  episodes: Episode[];
  chains: ChainRecord[];
  messages: ChatMessage[];
  nextId: number;
}

export interface ToolInfo {
  name: string;
  description: string;
  meta: boolean;
}

export interface SelfTestResult {
  name: string;
  passed: boolean;
  detail: string;
}
