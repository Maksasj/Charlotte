// Display helpers only; no engine logic.

/** A 0..1 fraction from the API as a percentage, e.g. 0.975 → "97.5%". */
export function percent(value: unknown): string {
  if (typeof value !== "number" || !Number.isFinite(value)) return "—";
  return `${Math.round(value * 1000) / 10}%`;
}

export function scalar(value: unknown): string {
  if (typeof value === "string") return value;
  if (value === undefined) return "—";
  return JSON.stringify(value);
}

export function list<T>(value: readonly T[] | null | undefined): readonly T[] {
  return Array.isArray(value) ? value : [];
}
