import { Component, type ErrorInfo, type ReactNode } from "react";
import { errorMessage } from "../api/client";
import { href } from "../router";

export function ErrorNotice({ error, onRetry, what }: { error: unknown; onRetry?: () => void; what?: string }) {
  if (error === null || error === undefined) return null;
  return (
    <div className="notice error" role="alert">
      <span>
        {what ? <strong>{what}: </strong> : null}
        {errorMessage(error)}
      </span>
      {onRetry ? (
        <button type="button" onClick={onRetry}>
          Retry
        </button>
      ) : null}
    </div>
  );
}

export function Loading({ what }: { what: string }) {
  return <p className="muted loading">Loading {what}…</p>;
}

export function Badge({ children, tone }: { children: ReactNode; tone?: "ok" | "warn" | "bad" | "plain" }) {
  return <span className={`badge ${tone ?? "plain"}`}>{children}</span>;
}

export function statusTone(status: string): "ok" | "warn" | "plain" {
  if (status === "trusted") return "ok";
  if (status === "candidate") return "warn";
  return "plain";
}

/** A knowledge id that opens the item in the Knowledge view. */
export function KnowledgeLink({ id }: { id: string | null | undefined }) {
  if (id === null || id === undefined || id === "") return <span className="muted">—</span>;
  return (
    <a className="mono" href={href("knowledge", id)}>
      {id}
    </a>
  );
}

interface BoundaryState {
  error: unknown;
}

/** Keeps an unexpected response shape from blanking the whole app. */
export class ErrorBoundary extends Component<{ children: ReactNode }, BoundaryState> {
  state: BoundaryState = { error: null };

  static getDerivedStateFromError(error: unknown): BoundaryState {
    return { error: error ?? new Error("render failed") };
  }

  componentDidCatch(error: unknown, info: ErrorInfo): void {
    console.error("view failed to render", error, info.componentStack);
  }

  render(): ReactNode {
    if (this.state.error !== null) {
      return (
        <div className="notice error" role="alert">
          <span>
            <strong>This view could not be displayed: </strong>
            {errorMessage(this.state.error)}
          </span>
          <button type="button" onClick={() => this.setState({ error: null })}>
            Try again
          </button>
        </div>
      );
    }
    return this.props.children;
  }
}
