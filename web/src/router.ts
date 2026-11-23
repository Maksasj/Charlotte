// Hash-based navigation: #/<view>[/<id>]. Deep links need no server fallback.

import { useMemo, useSyncExternalStore } from "react";

export const VIEWS = ["chat", "knowledge", "candidates", "learn", "status"] as const;
export type ViewName = (typeof VIEWS)[number];

export interface Route {
  view: ViewName;
  id: string | null;
}

function safeDecode(part: string): string {
  try {
    return decodeURIComponent(part);
  } catch {
    return part;
  }
}

export function parseHash(hash: string): Route {
  const parts = hash.replace(/^#\/?/, "").split("/").filter(Boolean);
  const first = parts[0];
  const view = VIEWS.find((v) => v === first) ?? "chat";
  const second = parts[1];
  return { view, id: second ? safeDecode(second) : null };
}

export function href(view: ViewName, id?: string | null): string {
  return id ? `#/${view}/${encodeURIComponent(id)}` : `#/${view}`;
}

export function navigate(view: ViewName, id?: string | null, replace = false): void {
  const target = href(view, id);
  if (window.location.hash === target) return;
  if (replace) {
    window.history.replaceState(null, "", target);
    window.dispatchEvent(new HashChangeEvent("hashchange"));
  } else {
    window.location.hash = target;
  }
}

function subscribe(listener: () => void): () => void {
  window.addEventListener("hashchange", listener);
  return () => window.removeEventListener("hashchange", listener);
}

export function useRoute(): Route {
  const hash = useSyncExternalStore(subscribe, () => window.location.hash);
  return useMemo(() => parseHash(hash), [hash]);
}
