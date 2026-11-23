import { useState } from "react";
import { getItem, listKnowledge, promoteItem, rejectItem } from "../api/client";
import type { FrontMatter, ItemSummary } from "../api/types";
import { ErrorNotice, KnowledgeLink, Loading } from "../components/common";
import { ItemPanel } from "../components/ItemPanel";
import { percent, scalar } from "../format";
import { useLoad } from "../hooks";
import { href, navigate } from "../router";

interface CandidateRow {
  summary: ItemSummary;
  /** null when the item itself could not be loaded */
  front: FrontMatter | null;
}

// support and validated live in the note's front matter, which the list
// endpoint does not return, so each candidate is fetched as well.
async function loadCandidates(): Promise<CandidateRow[]> {
  const summaries = await listKnowledge({ status: "candidate" });
  const fronts = await Promise.all(
    summaries.map((summary) =>
      getItem(summary.id).then(
        (item) => item.front ?? null,
        () => null,
      ),
    ),
  );
  return summaries.map((summary, index) => ({ summary, front: fronts[index] ?? null }));
}

type Outcome = { action: "promoted" | "rejected"; id: string; promotedId?: string };

export function CandidatesView({ id }: { id: string | null }) {
  const candidates = useLoad(loadCandidates, "candidates");
  const [busy, setBusy] = useState<"promote" | "reject" | null>(null);
  const [actionError, setActionError] = useState<{ what: string; error: unknown } | null>(null);
  const [outcome, setOutcome] = useState<Outcome | null>(null);
  const rows = candidates.data;

  const review = (action: "promote" | "reject", target: string) => {
    if (busy !== null) return;
    setBusy(action);
    setActionError(null);
    setOutcome(null);
    const call: Promise<Outcome> =
      action === "promote"
        ? promoteItem(target).then((item) => ({ action: "promoted", id: target, promotedId: item.id }))
        : rejectItem(target).then(() => ({ action: "rejected", id: target }));
    call
      .then((done) => {
        setOutcome(done);
        navigate("candidates", null, true);
        candidates.reload();
      })
      .catch((error: unknown) =>
        setActionError({ what: action === "promote" ? "Not promoted" : "Not rejected", error }),
      )
      .finally(() => setBusy(null));
  };

  return (
    <div className="split">
      <section className="pane">
        <div className="pane-head">
          <h2>Candidates</h2>
          <button type="button" onClick={candidates.reload} disabled={candidates.loading}>
            {candidates.loading ? "Loading…" : "Refresh"}
          </button>
        </div>
        <p className="muted">Learned items waiting for review. Promote makes an item trusted; Reject discards it.</p>

        {outcome ? (
          <div className="notice ok" role="status">
            <span>
              {outcome.action === "promoted" ? "Promoted " : "Rejected "}
              {outcome.action === "promoted" ? (
                <KnowledgeLink id={outcome.promotedId ?? outcome.id} />
              ) : (
                <span className="mono">{outcome.id}</span>
              )}
              .
            </span>
          </div>
        ) : null}
        {actionError ? <ErrorNotice error={actionError.error} what={actionError.what} /> : null}

        {candidates.loading && !rows ? <Loading what="candidates" /> : null}
        <ErrorNotice error={candidates.error} onRetry={candidates.reload} what="Could not list candidates" />
        {rows && rows.length === 0 ? (
          <p className="empty">No candidates are waiting for review. Run a learning job to produce some.</p>
        ) : null}
        {rows && rows.length > 0 ? (
          <div className="table-wrap">
            <table className="table selectable">
              <thead>
                <tr>
                  <th>Id</th>
                  <th>Kind</th>
                  <th>Title</th>
                  <th>Support</th>
                  <th>Validated</th>
                </tr>
              </thead>
              <tbody>
                {rows.map(({ summary, front }) => {
                  const support = front?.["support"];
                  const validated = front?.["validated"];
                  return (
                    <tr
                      key={summary.id}
                      className={summary.id === id ? "selected" : undefined}
                      onClick={() => navigate("candidates", summary.id)}
                    >
                      <td>
                        <a
                          className="mono"
                          href={href("candidates", summary.id)}
                          onClick={(e) => e.stopPropagation()}
                        >
                          {summary.id}
                        </a>
                      </td>
                      <td>{summary.kind}</td>
                      <td>{summary.title}</td>
                      <td className="num">{support === undefined || support === null ? "—" : scalar(support)}</td>
                      <td className="num">{percent(validated)}</td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        ) : null}
      </section>

      <section className="pane">
        {id === null ? (
          <p className="muted">Select a candidate to read it before promoting or rejecting it.</p>
        ) : (
          <ItemPanel
            key={id}
            id={id}
            onChanged={candidates.reload}
            actions={
              <>
                <button type="button" className="primary" disabled={busy !== null} onClick={() => review("promote", id)}>
                  {busy === "promote" ? "Promoting…" : "Promote"}
                </button>
                <button type="button" className="danger" disabled={busy !== null} onClick={() => review("reject", id)}>
                  {busy === "reject" ? "Rejecting…" : "Reject"}
                </button>
              </>
            }
          />
        )}
      </section>
    </div>
  );
}
