import { useState, type ReactNode } from "react";
import { getItem, updateItem } from "../api/client";
import type { FrontMatter, Item } from "../api/types";
import { percent, list } from "../format";
import { useLoad } from "../hooks";
import { Badge, ErrorNotice, KnowledgeLink, Loading, statusTone } from "./common";

interface Draft {
  front: string;
  body: string;
}

function IdList({ ids }: { ids: readonly string[] }) {
  if (ids.length === 0) return <span className="muted">none</span>;
  return (
    <ul className="inline">
      {ids.map((id) => (
        <li key={id}>
          <KnowledgeLink id={id} />
        </li>
      ))}
    </ul>
  );
}

function parseFront(text: string): FrontMatter {
  let parsed: unknown;
  try {
    parsed = JSON.parse(text);
  } catch (error) {
    throw new Error(`Front matter is not valid JSON: ${error instanceof Error ? error.message : String(error)}`);
  }
  if (typeof parsed !== "object" || parsed === null || Array.isArray(parsed)) {
    throw new Error("Front matter must be a JSON object, for example { \"kind\": \"atom\" }.");
  }
  return parsed as FrontMatter;
}

/**
 * One knowledge item: front matter, body, links and backlinks, with an edit
 * mode. Mount it with key={id} so the edit draft never leaks between items.
 */
export function ItemPanel({
  id,
  onChanged,
  actions,
}: {
  id: string;
  onChanged?: (item: Item) => void;
  actions?: ReactNode;
}) {
  const item = useLoad(() => getItem(id), id);
  const [draft, setDraft] = useState<Draft | null>(null);
  const [saving, setSaving] = useState(false);
  const [saveError, setSaveError] = useState<unknown>(null);

  const data = item.data;

  const save = () => {
    if (!draft || saving) return;
    let front: FrontMatter;
    try {
      front = parseFront(draft.front);
    } catch (error) {
      setSaveError(error);
      return;
    }
    setSaving(true);
    setSaveError(null);
    updateItem(id, { front, body: draft.body })
      .then((saved) => {
        item.setData(() => saved);
        setDraft(null);
        onChanged?.(saved);
      })
      .catch((error: unknown) => setSaveError(error))
      .finally(() => setSaving(false));
  };

  return (
    <div className="item">
      <div className="item-head">
        <h2 className="mono">{id}</h2>
        {data ? (
          <div className="item-meta">
            <Badge>{data.kind}</Badge>
            <Badge tone={statusTone(data.status)}>{data.status}</Badge>
            <span>trust {percent(data.trust)}</span>
            <code>{data.path}</code>
          </div>
        ) : null}
      </div>

      {item.loading && !data ? <Loading what="item" /> : null}
      <ErrorNotice error={item.error} onRetry={item.reload} what="Could not load the item" />

      {data && draft === null ? (
        <>
          <div className="toolbar">
            <button
              type="button"
              onClick={() => {
                setSaveError(null);
                setDraft({ front: JSON.stringify(data.front ?? {}, null, 2), body: data.body ?? "" });
              }}
            >
              Edit
            </button>
            {actions}
          </div>
          <h3>Front matter</h3>
          <pre className="code">{JSON.stringify(data.front ?? {}, null, 2)}</pre>
          <h3>Body</h3>
          {data.body ? <pre className="body">{data.body}</pre> : <p className="muted">empty</p>}
          <h3>Links</h3>
          <IdList ids={list(data.links)} />
          <h3>Backlinks</h3>
          <IdList ids={list(data.backlinks)} />
        </>
      ) : null}

      {data && draft !== null ? (
        <form
          className="edit"
          onSubmit={(event) => {
            event.preventDefault();
            save();
          }}
        >
          <label htmlFor="edit-front">Front matter (JSON)</label>
          <textarea
            id="edit-front"
            className="mono"
            rows={18}
            spellCheck={false}
            value={draft.front}
            disabled={saving}
            onChange={(event) => setDraft({ ...draft, front: event.target.value })}
          />
          <label htmlFor="edit-body">Body</label>
          <textarea
            id="edit-body"
            rows={8}
            value={draft.body}
            disabled={saving}
            onChange={(event) => setDraft({ ...draft, body: event.target.value })}
          />
          <ErrorNotice error={saveError} what="Not saved" />
          <div className="toolbar">
            <button type="submit" className="primary" disabled={saving}>
              {saving ? "Saving…" : "Save"}
            </button>
            <button
              type="button"
              disabled={saving}
              onClick={() => {
                setDraft(null);
                setSaveError(null);
              }}
            >
              Cancel
            </button>
          </div>
        </form>
      ) : null}
    </div>
  );
}
