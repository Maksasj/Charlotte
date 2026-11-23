import { getHealth } from "../api/client";
import { ErrorNotice, Loading } from "../components/common";
import { useLoad } from "../hooks";

export function StatusView() {
  const health = useLoad(getHealth, "health");
  const data = health.data;
  const kinds = Object.entries(data?.items ?? {});

  return (
    <section className="pane narrow">
      <div className="pane-head">
        <h2>Status</h2>
        <button type="button" onClick={health.reload} disabled={health.loading}>
          {health.loading ? "Loading…" : "Refresh"}
        </button>
      </div>
      {health.loading && !data ? <Loading what="status" /> : null}
      <ErrorNotice error={health.error} onRetry={health.reload} what="Could not load the status" />
      {data ? (
        <>
          <dl className="kv wide">
            <dt>Engine version</dt>
            <dd>
              <code>{data.version}</code>
            </dd>
            <dt>Vault</dt>
            <dd>
              <code>{data.vault}</code>
            </dd>
            <dt>Candidates</dt>
            <dd>{data.candidates}</dd>
            <dt>Conversations</dt>
            <dd>{data.conversations}</dd>
          </dl>
          <h3>Trusted items per kind</h3>
          {kinds.length === 0 ? (
            <p className="muted">The vault holds no trusted items.</p>
          ) : (
            <table className="table compact">
              <thead>
                <tr>
                  <th>Kind</th>
                  <th>Trusted items</th>
                </tr>
              </thead>
              <tbody>
                {kinds.map(([kind, count]) => (
                  <tr key={kind}>
                    <td>{kind}</td>
                    <td className="num">{count}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </>
      ) : null}
    </section>
  );
}
