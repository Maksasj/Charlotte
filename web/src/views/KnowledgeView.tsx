import { useEffect, useState } from "react";
import { listKnowledge } from "../api/client";
import { ITEM_KINDS, ITEM_STATUSES, type ItemKind, type ItemStatus, type KnowledgeQuery } from "../api/types";
import { Badge, ErrorNotice, Loading, statusTone } from "../components/common";
import { ItemPanel } from "../components/ItemPanel";
import { percent } from "../format";
import { useLoad } from "../hooks";
import { href, navigate } from "../router";

export function KnowledgeView({ id }: { id: string | null }) {
  const [kind, setKind] = useState<ItemKind | "">("");
  const [status, setStatus] = useState<ItemStatus | "">("");
  const [search, setSearch] = useState("");
  const [q, setQ] = useState("");

  // Query the engine once typing pauses, not on every keystroke.
  useEffect(() => {
    const timer = window.setTimeout(() => setQ(search.trim()), 300);
    return () => window.clearTimeout(timer);
  }, [search]);

  const query: KnowledgeQuery = {};
  if (kind) query.kind = kind;
  if (status) query.status = status;
  if (q) query.q = q;
  const results = useLoad(() => listKnowledge(query), JSON.stringify(query));
  const rows = results.data;

  return (
    <div className="split">
      <section className="pane">
        <div className="pane-head">
          <h2>Knowledge</h2>
          {results.loading && rows ? <span className="muted small loading">Refreshing…</span> : null}
        </div>
        <form
          className="filters"
          onSubmit={(event) => {
            event.preventDefault();
            setQ(search.trim());
          }}
        >
          <label>
            Kind
            <select value={kind} onChange={(event) => setKind(event.target.value as ItemKind | "")}>
              <option value="">all</option>
              {ITEM_KINDS.map((k) => (
                <option key={k} value={k}>
                  {k}
                </option>
              ))}
            </select>
          </label>
          <label>
            Status
            <select value={status} onChange={(event) => setStatus(event.target.value as ItemStatus | "")}>
              <option value="">all</option>
              {ITEM_STATUSES.map((s) => (
                <option key={s} value={s}>
                  {s}
                </option>
              ))}
            </select>
          </label>
          <label className="grow">
            Search
            <input
              type="search"
              placeholder="text to search for"
              value={search}
              onChange={(event) => setSearch(event.target.value)}
            />
          </label>
        </form>

        {results.loading && !rows ? <Loading what="knowledge" /> : null}
        <ErrorNotice error={results.error} onRetry={results.reload} what="Could not list knowledge" />
        {rows && rows.length === 0 ? <p className="muted">No items match these filters.</p> : null}
        {rows && rows.length > 0 ? (
          <div className="table-wrap">
            <table className="table selectable">
              <thead>
                <tr>
                  <th>Id</th>
                  <th>Kind</th>
                  <th>Status</th>
                  <th>Trust</th>
                  <th>Title</th>
                  <th>Path</th>
                </tr>
              </thead>
              <tbody>
                {rows.map((row) => (
                  <tr
                    key={row.id}
                    className={row.id === id ? "selected" : undefined}
                    onClick={() => navigate("knowledge", row.id)}
                  >
                    <td>
                      <a className="mono" href={href("knowledge", row.id)} onClick={(e) => e.stopPropagation()}>
                        {row.id}
                      </a>
                    </td>
                    <td>{row.kind}</td>
                    <td>
                      <Badge tone={statusTone(row.status)}>{row.status}</Badge>
                    </td>
                    <td className="num">{percent(row.trust)}</td>
                    <td>{row.title}</td>
                    <td className="mono small">{row.path}</td>
                  </tr>
                ))}
              </tbody>
            </table>
            <p className="muted small">{rows.length} item(s)</p>
          </div>
        ) : null}
      </section>

      <section className="pane">
        {id === null ? (
          <p className="muted">Select an item to see its front matter, body and links.</p>
        ) : (
          <ItemPanel key={id} id={id} onChanged={results.reload} />
        )}
      </section>
    </div>
  );
}
