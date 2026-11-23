import { useCallback, useEffect, useRef, useState, useSyncExternalStore } from "react";
import { getConnection, subscribeConnection, type ConnectionState } from "./api/client";

export function useConnection(): ConnectionState {
  return useSyncExternalStore(subscribeConnection, getConnection);
}

interface LoadState<T> {
  key: string | null;
  data: T | null;
  error: unknown;
  loading: boolean;
}

export interface Load<T> {
  data: T | null;
  /** null when there is no error */
  error: unknown;
  loading: boolean;
  /** Fetch again, keeping the current data on screen meanwhile. */
  reload: () => void;
  /** Replace the data locally (for example with the body of a PUT response). */
  setData: (update: (previous: T | null) => T | null) => void;
}

/**
 * Runs `load` whenever `key` changes, on `reload()`, and when the engine
 * becomes reachable again. A null key means "nothing to load".
 * Results that arrive after the key has changed are dropped.
 */
export function useLoad<T>(load: () => Promise<T>, key: string | null): Load<T> {
  const [state, setState] = useState<LoadState<T>>({ key, data: null, error: null, loading: key !== null });
  const [nonce, setNonce] = useState(0);
  const { epoch } = useConnection();
  const loadRef = useRef(load);
  loadRef.current = load;

  useEffect(() => {
    if (key === null) {
      setState({ key, data: null, error: null, loading: false });
      return;
    }
    let cancelled = false;
    setState((prev) =>
      prev.key === key ? { ...prev, loading: true, error: null } : { key, data: null, error: null, loading: true },
    );
    loadRef.current().then(
      (data) => {
        if (!cancelled) setState({ key, data, error: null, loading: false });
      },
      (error: unknown) => {
        if (!cancelled) setState((prev) => ({ ...prev, error: error ?? new Error("request failed"), loading: false }));
      },
    );
    return () => {
      cancelled = true;
    };
  }, [key, nonce, epoch]);

  const reload = useCallback(() => setNonce((n) => n + 1), []);
  const setData = useCallback((update: (previous: T | null) => T | null) => {
    setState((prev) => ({ ...prev, data: update(prev.data) }));
  }, []);

  // Until the effect for a new key has run, the stored state belongs to the old key.
  if (state.key !== key) {
    return { data: null, error: null, loading: key !== null, reload, setData };
  }
  return { data: state.data, error: state.error, loading: state.loading, reload, setData };
}
