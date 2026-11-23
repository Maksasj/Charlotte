// Hand-written mirror of docs/api.md. Field names and null semantics follow
// that file and were checked against the responses of a running `charlotte serve`;
// comments say what the engine was seen to send where the doc is silent.

export type FactValue = string | number | boolean | null;
export type Facts = Record<string, FactValue>;

export type Verdict = "resolved" | "partial" | "failed";

export type Op =
  | "ask"
  | "grant"
  | "offer"
  | "deny"
  | "confirm"
  | "lookup"
  | "report"
  | "instruct"
  | "repeat"
  | "finish"
  | "escalate"
  | "say"
  | "teach_me";

export type UserAct = "request" | "provide" | "correct" | "done" | "failed";
export type RequestKind = "gated" | "procedure" | "lookup" | "move" | "unknown";
export type Standing = "grant" | "offer" | "deny";

// ---- Conversations ----

export interface ConversationCreated {
  id: string;
}

export interface ConversationSummary {
  id: string;
  turns: number;
  closed: boolean;
  verdict: Verdict | null;
  /** Text of the last message; null while the conversation has no messages. */
  last: string | null;
}

/** User messages carry no `trace` key at all. */
export interface UserMessage {
  role: "user";
  text: string;
}

export interface AgentMessage {
  role: "agent";
  text: string;
  trace: Trace;
}

export type Message = UserMessage | AgentMessage;

export interface Conversation {
  id: string;
  closed: boolean;
  verdict: Verdict | null;
  messages: Message[];
  frames: Frame[];
}

export interface SendMessageRequest {
  text: string;
}

export interface Act {
  op: Op;
  /** Omitted (not null) when the operation takes no argument. Always a string, e.g. "0" for a step number. */
  arg?: string;
}

export interface Reply {
  text: string;
  acts: Act[];
  trace: Trace;
}

export interface Trace {
  requests: RequestTrace[];
  notes: string[];
}

export interface Matched {
  phrase: string;
  /** Seen from the engine: intent | fact | answer | signal. */
  kind: string;
  target: string;
}

export interface Rule {
  /** 1-based position in the pattern's rule list; the fall-through rule is the number of rules + 1. */
  index: number;
  /** ["otherwise"] when the pattern's fall-through rule fired. */
  when: string[];
  do: string;
}

export interface Step {
  features: string[];
  /** null when no rule fired (not-understood requests and cached replies). */
  rule: Rule | null;
  act: Act;
}

export interface RequestTrace {
  clause: string;
  /**
   * The intent read from this clause. null when the request was not understood
   * (then `frame` is null too), and also null for a clause that only continues
   * an open request ("5 days ago", "yes", "done"); the request's intent is
   * then in `frame.intent`.
   */
  intent: string | null;
  matched: Matched[];
  facts: Facts;
  user_act: UserAct | null;
  kind: RequestKind;
  pattern: string | null;
  atom: string | null;
  alternative: string | null;
  steps: Step[];
  text: string;
  /** null when the request was not understood. */
  frame: Frame | null;
}

export interface Frame {
  id: number;
  intent: string;
  facts: Facts;
  agent_acts: string[];
  user_acts: string[];
  /** null when nothing has been asked. */
  last_asked: string | null;
  standing: Standing | null;
  fetched: boolean;
  found: boolean;
  steps_done: number;
}

export interface CloseRequest {
  verdict: Verdict;
}

export interface TrustChange {
  id: string;
  before: number;
  after: number;
}

export interface CloseResult {
  updated: TrustChange[];
}

// ---- Knowledge ----

export type ItemKind =
  | "pattern"
  | "atom"
  | "procedure"
  | "intent"
  | "slot"
  | "records"
  | "move"
  | "lexicon"
  | "phrases";

export const ITEM_KINDS: readonly ItemKind[] = [
  "pattern",
  "atom",
  "procedure",
  "intent",
  "slot",
  "records",
  "move",
  "lexicon",
  "phrases",
];

export type ItemStatus = "candidate" | "trusted" | "retired";

export const ITEM_STATUSES: readonly ItemStatus[] = ["trusted", "candidate", "retired"];

export interface KnowledgeQuery {
  kind?: ItemKind;
  status?: ItemStatus;
  q?: string;
}

export interface ItemSummary {
  id: string;
  kind: ItemKind;
  status: ItemStatus;
  trust: number;
  path: string;
  title: string;
}

export type FrontMatter = Record<string, unknown>;

export interface Item {
  id: string;
  kind: ItemKind;
  status: ItemStatus;
  trust: number;
  path: string;
  front: FrontMatter;
  body: string;
  links: string[];
  backlinks: string[];
}

export interface ItemUpdate {
  front: FrontMatter;
  body: string;
}

export interface RejectResult {
  ok: boolean;
}

// ---- Learning ----

export type LearnRequest =
  | { path: string; auto_promote?: boolean }
  | { jsonl: string; auto_promote?: boolean };

export interface LearnStarted {
  job: string;
}

export type JobState = "running" | "done" | "failed";
export type JobStage = "ingest" | "induce" | "validate" | "write" | "done";

export interface Job {
  id: string;
  state: JobState;
  stage: JobStage;
  /** The corpus path, or "uploaded" for a `jsonl` upload. */
  source: string;
  report: Report | null;
  error: string | null;
}

export interface BeforeAfter {
  before: number;
  after: number;
}

export interface ReportCandidate {
  id: string;
  kind: ItemKind;
  support: number;
  validated: number;
  promoted: boolean;
  summary: string;
}

export interface ReportMistake {
  what: string;
  count: number;
}

export interface Report {
  conversations: { read: number; skipped: number; train: number; held_out: number };
  labels: { act: number; intent: number; facts: number };
  held_out: BeforeAfter;
  understanding: BeforeAfter;
  candidates: ReportCandidate[];
  mistakes: ReportMistake[];
  seconds: number;
}

// ---- Status ----

export interface Health {
  version: string;
  vault: string;
  items: Record<string, number>;
  candidates: number;
  conversations: number;
}

// ---- Errors ----

export interface ErrorBody {
  error: string;
}
