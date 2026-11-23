// The only module that talks to the engine. One function per endpoint in
// docs/api.md; every path is relative (/v1/...) so the same build works behind
// the Vite proxy and when served by `charlotte serve`.

import type {
  CloseResult,
  Conversation,
  ConversationCreated,
  ConversationSummary,
  Health,
  Item,
  ItemSummary,
  ItemUpdate,
  Job,
  KnowledgeQuery,
  LearnRequest,
  LearnStarted,
  RejectResult,
  Reply,
  Verdict,
} from "./types";

export class ApiError extends Error {
  /** HTTP status, or null when no response was received at all. */
  readonly status: number | null;
  /** True when the engine could not be reached (as opposed to refusing the request). */
  readonly unreachable: boolean;

  constructor(message: string, status: number | null, unreachable: boolean) {
    super(message);
    this.name = "ApiError";
    this.status = status;
    this.unreachable = unreachable;
  }
}

export const UNREACHABLE_MESSAGE = "Cannot reach the engine at /v1 — is `charlotte serve` running?";

export function errorMessage(error: unknown): string {
  if (error instanceof Error) return error.message;
  return String(error);
}

// ---- Mock mode (development aid, see mock.ts) ----

const MOCK_ALLOWED = import.meta.env.DEV || import.meta.env.VITE_ENABLE_MOCK === "1";

export const mockMode: boolean =
  MOCK_ALLOWED &&
  typeof window !== "undefined" &&
  new URLSearchParams(window.location.search).get("mock") === "1";

type FetchLike = (url: string, init: RequestInit) => Promise<Response>;

const transport: Promise<FetchLike> =
  MOCK_ALLOWED && mockMode
    ? import("./mock").then((m) => m.mockFetch)
    : Promise.resolve((url: string, init: RequestInit) => fetch(url, init));

// ---- Connection state, so the app can show one banner and reload on recovery ----

export interface ConnectionState {
  reachable: boolean;
  /** Incremented each time the engine becomes reachable again. */
  epoch: number;
}

let connection: ConnectionState = { reachable: true, epoch: 0 };
const listeners = new Set<() => void>();

export function getConnection(): ConnectionState {
  return connection;
}

export function subscribeConnection(listener: () => void): () => void {
  listeners.add(listener);
  return () => {
    listeners.delete(listener);
  };
}

function setReachable(reachable: boolean): void {
  if (connection.reachable === reachable) return;
  connection = { reachable, epoch: reachable ? connection.epoch + 1 : connection.epoch };
  listeners.forEach((l) => l());
}

// ---- Request core ----

function unreachable(status: number | null): ApiError {
  setReachable(false);
  return new ApiError(UNREACHABLE_MESSAGE, status, true);
}

function readErrorField(text: string): string | null {
  try {
    const parsed: unknown = JSON.parse(text);
    if (typeof parsed === "object" && parsed !== null && "error" in parsed) {
      const message = (parsed as { error: unknown }).error;
      return typeof message === "string" ? message : JSON.stringify(message);
    }
  } catch {
    // not JSON
  }
  return null;
}

async function request<T>(method: "GET" | "POST" | "PUT", path: string, body?: unknown): Promise<T> {
  const init: RequestInit = { method, headers: { Accept: "application/json" } };
  if (body !== undefined) {
    init.headers = { Accept: "application/json", "Content-Type": "application/json" };
    init.body = JSON.stringify(body);
  }

  let response: Response;
  let text: string;
  try {
    const send = await transport;
    response = await send(path, init);
    text = await response.text();
  } catch {
    throw unreachable(null);
  }

  if (!response.ok) {
    const message = readErrorField(text);
    if (message !== null) {
      setReachable(true);
      throw new ApiError(message, response.status, false);
    }
    // A gateway error, or a 5xx without the engine's error body, comes from
    // the proxy in front of a server that is not running.
    if (response.status >= 500) throw unreachable(response.status);
    setReachable(true);
    // The engine's framework rejects a malformed body with a plain-text 400/422
    // instead of {"error": ...}; that text is still the best message to show.
    const plain = text.trim();
    const isHtml = (response.headers.get("content-type") ?? "").includes("html") || plain.startsWith("<");
    const detail = plain !== "" && !isHtml ? `: ${plain.slice(0, 300)}` : "";
    throw new ApiError(`${method} ${path} failed with status ${response.status}${detail}`, response.status, false);
  }

  setReachable(true);
  try {
    return JSON.parse(text) as T;
  } catch {
    throw new ApiError(`${method} ${path} did not return JSON`, response.status, false);
  }
}

const enc = encodeURIComponent;

// ---- Conversations ----

export function createConversation(): Promise<ConversationCreated> {
  return request("POST", "/v1/conversations");
}

export function listConversations(): Promise<ConversationSummary[]> {
  return request("GET", "/v1/conversations");
}

export function getConversation(id: string): Promise<Conversation> {
  return request("GET", `/v1/conversations/${enc(id)}`);
}

export function sendMessage(id: string, text: string): Promise<Reply> {
  return request("POST", `/v1/conversations/${enc(id)}/messages`, { text });
}

export function closeConversation(id: string, verdict: Verdict): Promise<CloseResult> {
  return request("POST", `/v1/conversations/${enc(id)}/close`, { verdict });
}

// ---- Knowledge ----

export function listKnowledge(query: KnowledgeQuery = {}): Promise<ItemSummary[]> {
  const params = new URLSearchParams();
  if (query.kind) params.set("kind", query.kind);
  if (query.status) params.set("status", query.status);
  if (query.q) params.set("q", query.q);
  const qs = params.toString();
  return request("GET", qs ? `/v1/knowledge?${qs}` : "/v1/knowledge");
}

export function getItem(id: string): Promise<Item> {
  return request("GET", `/v1/knowledge/${enc(id)}`);
}

export function updateItem(id: string, update: ItemUpdate): Promise<Item> {
  return request("PUT", `/v1/knowledge/${enc(id)}`, update);
}

export function promoteItem(id: string): Promise<Item> {
  return request("POST", `/v1/knowledge/${enc(id)}/promote`);
}

export function rejectItem(id: string): Promise<RejectResult> {
  return request("POST", `/v1/knowledge/${enc(id)}/reject`);
}

// ---- Learning ----

export function startLearn(body: LearnRequest): Promise<LearnStarted> {
  return request("POST", "/v1/learn", body);
}

export function listJobs(): Promise<Job[]> {
  return request("GET", "/v1/learn");
}

export function getJob(job: string): Promise<Job> {
  return request("GET", `/v1/learn/${enc(job)}`);
}

// ---- Status ----

export function getHealth(): Promise<Health> {
  return request("GET", "/v1/health");
}
