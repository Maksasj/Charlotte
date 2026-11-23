import { useEffect, useState, type ChangeEvent } from "react";
import { getJob, listJobs, startLearn } from "../api/client";
import type { BeforeAfter, Job, JobStage, LearnRequest, Report } from "../api/types";
import { Badge, ErrorNotice, KnowledgeLink, Loading } from "../components/common";
import { list, percent } from "../format";
import { useLoad } from "../hooks";
import { href, navigate } from "../router";

const STAGES: readonly JobStage[] = ["ingest", "induce", "validate", "write", "done"];

function stateTone(state: string): "ok" | "warn" | "bad" | "plain" {
  if (state === "done") return "ok";
  if (state === "running") return "warn";
  if (state === "failed") return "bad";
  return "plain";
}

function Change({ value }: { value: BeforeAfter | null | undefined }) {
  if (!value) return <span className="muted">—</span>;
  return (
    <span className="change">
      {percent(value.before)} → <strong>{percent(value.after)}</strong>
    </span>
  );
}

export function ReportView({ report }: { report: Report }) {
  const conversations = report.conversations;
  const labels = Object.entries(report.labels ?? {});
  const candidates = list(report.candidates);
  const mistakes = list(report.mistakes);
  return (
    <div className="report">
      <h3>Conversations</h3>
      <dl className="stats">
        <div>
          <dt>Read</dt>
          <dd>{conversations?.read ?? "—"}</dd>
        </div>
        <div>
          <dt>Skipped</dt>
          <dd>{conversations?.skipped ?? "—"}</dd>
        </div>
        <div>
          <dt>Train</dt>
          <dd>{conversations?.train ?? "—"}</dd>
        </div>
        <div>
          <dt>Held out</dt>
          <dd>{conversations?.held_out ?? "—"}</dd>
        </div>
      </dl>

      <h3>Label coverage</h3>
      <p className="muted small">Share of turns carrying each label.</p>
      <dl className="stats">
        {labels.map(([label, share]) => (
          <div key={label}>
            <dt>{label}</dt>
            <dd>{percent(share)}</dd>
          </div>
        ))}
      </dl>

      <h3>Results on held-out conversations</h3>
      <dl className="kv wide">
        <dt>Held-out score</dt>
        <dd>
          <Change value={report.held_out} />{" "}
          <span className="muted small">conversations reproduced exactly, before → after</span>
        </dd>
        <dt>Understanding</dt>
        <dd>
          <Change value={report.understanding} />{" "}
          <span className="muted small">user turns whose intent is read correctly, before → after</span>
        </dd>
      </dl>

      <h3>Candidates ({candidates.length})</h3>
      {candidates.length === 0 ? (
        <p className="muted">The job proposed no candidates.</p>
      ) : (
        <div className="table-wrap">
          <table className="table">
            <thead>
              <tr>
                <th>Id</th>
                <th>Kind</th>
                <th>Support</th>
                <th>Validated</th>
                <th>Promoted</th>
                <th>Summary</th>
              </tr>
            </thead>
            <tbody>
              {candidates.map((candidate) => (
                <tr key={candidate.id}>
                  <td>
                    <KnowledgeLink id={candidate.id} />
                  </td>
                  <td>{candidate.kind}</td>
                  <td className="num">{candidate.support}</td>
                  <td className="num">{percent(candidate.validated)}</td>
                  <td>{candidate.promoted ? <Badge tone="ok">yes</Badge> : <Badge>no</Badge>}</td>
                  <td>{candidate.summary}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      <h3>Commonest mistakes</h3>
      {mistakes.length === 0 ? (
        <p className="muted">None reported.</p>
      ) : (
        <table className="table compact">
          <thead>
            <tr>
              <th>What</th>
              <th>Count</th>
            </tr>
          </thead>
          <tbody>
            {mistakes.map((mistake, index) => (
              <tr key={index}>
                <td>{mistake.what}</td>
                <td className="num">{mistake.count}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}

      <p className="muted">Took {typeof report.seconds === "number" ? report.seconds : "—"} seconds.</p>
    </div>
  );
}

function JobDetail({ job }: { job: Job }) {
  return (
    <div className="job">
      <div className="item-head">
        <h2>
          Job <span className="mono">{job.id}</span>
        </h2>
        <div className="item-meta">
          <Badge tone={stateTone(job.state)}>{job.state}</Badge>
          <span>
            source <code>{job.source ? job.source : "—"}</code>
          </span>
        </div>
      </div>
      <ol className="stages" aria-label="Stage">
        {STAGES.map((stage) => (
          <li key={stage} className={stage === job.stage ? "current" : undefined}>
            {stage}
            {stage === job.stage ? <span className="muted small"> (current stage)</span> : null}
          </li>
        ))}
      </ol>
      {job.state === "running" ? <p className="muted loading">Running — checking every second…</p> : null}
      {job.state === "failed" ? (
        <div className="notice error" role="alert">
          <span>
            <strong>Job failed at stage {job.stage}: </strong>
            {job.error ?? "the engine gave no reason"}
          </span>
        </div>
      ) : null}
      {job.state !== "failed" && job.error ? <ErrorNotice error={new Error(job.error)} what="Engine error" /> : null}
      {job.report ? (
        <ReportView report={job.report} />
      ) : job.state === "done" ? (
        <p className="muted">The job is done but carries no report.</p>
      ) : null}
    </div>
  );
}

type Source = "path" | "jsonl";

export function LearnView({ id }: { id: string | null }) {
  const jobs = useLoad(listJobs, "jobs");
  const [source, setSource] = useState<Source>("path");
  const [path, setPath] = useState("");
  const [jsonl, setJsonl] = useState("");
  const [autoPromote, setAutoPromote] = useState(true);
  const [submitting, setSubmitting] = useState(false);
  const [submitError, setSubmitError] = useState<unknown>(null);
  const [pollError, setPollError] = useState<unknown>(null);

  const rows = jobs.data;
  const setJobs = jobs.setData;

  const runningKey = list(rows)
    .filter((job) => job.state === "running")
    .map((job) => job.id)
    .sort()
    .join("\n");

  useEffect(() => {
    if (runningKey === "") {
      setPollError(null);
      return;
    }
    const ids = runningKey.split("\n");
    let cancelled = false;
    let timer = 0;
    const tick = () => {
      void Promise.allSettled(ids.map((job) => getJob(job))).then((results) => {
        if (cancelled) return;
        const fresh: Job[] = [];
        let failure: unknown = null;
        for (const result of results) {
          if (result.status === "fulfilled") fresh.push(result.value);
          else failure = result.reason ?? new Error("request failed");
        }
        setPollError(failure);
        if (fresh.length > 0) {
          setJobs((previous) =>
            previous ? previous.map((job) => fresh.find((f) => f.id === job.id) ?? job) : previous,
          );
        }
        timer = window.setTimeout(tick, 1000);
      });
    };
    timer = window.setTimeout(tick, 1000);
    return () => {
      cancelled = true;
      window.clearTimeout(timer);
    };
  }, [runningKey, setJobs]);

  const selectedId = id ?? rows?.[0]?.id ?? null;
  const selected = list(rows).find((job) => job.id === selectedId) ?? null;

  const canSubmit = !submitting && (source === "path" ? path.trim() !== "" : jsonl.trim() !== "");

  const submit = () => {
    if (!canSubmit) return;
    const body: LearnRequest =
      source === "path" ? { path: path.trim(), auto_promote: autoPromote } : { jsonl, auto_promote: autoPromote };
    setSubmitting(true);
    setSubmitError(null);
    startLearn(body)
      .then((started) => {
        jobs.reload();
        navigate("learn", started.job);
      })
      .catch((error: unknown) => setSubmitError(error))
      .finally(() => setSubmitting(false));
  };

  const pickFile = (event: ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;
    setSubmitError(null);
    file.text().then(
      (text) => setJsonl(text),
      (error: unknown) => setSubmitError(error ?? new Error("could not read the file")),
    );
  };

  return (
    <div className="split">
      <section className="pane">
        <div className="pane-head">
          <h2>Learn</h2>
        </div>
        <form
          className="learn-form"
          onSubmit={(event) => {
            event.preventDefault();
            submit();
          }}
        >
          <fieldset disabled={submitting}>
            <legend>Start a learning job</legend>
            <label className="radio">
              <input type="radio" name="source" checked={source === "path"} onChange={() => setSource("path")} />
              Corpus file on the server
            </label>
            <input
              type="text"
              className="mono"
              aria-label="Corpus path on the server"
              placeholder="corpus/pretrain-80.jsonl"
              value={path}
              disabled={source !== "path"}
              onChange={(event) => setPath(event.target.value)}
            />
            <label className="radio">
              <input type="radio" name="source" checked={source === "jsonl"} onChange={() => setSource("jsonl")} />
              Upload JSON lines (one conversation per line)
            </label>
            <input
              type="file"
              aria-label="JSON-lines file"
              accept=".jsonl,.ndjson,.json,.txt"
              disabled={source !== "jsonl"}
              onChange={pickFile}
            />
            <textarea
              className="mono"
              aria-label="JSON-lines content"
              rows={6}
              spellCheck={false}
              placeholder='{"id": "c000123", "turns": [{"role": "user", "text": "..."}, {"role": "agent", "text": "..."}]}'
              value={jsonl}
              disabled={source !== "jsonl"}
              onChange={(event) => setJsonl(event.target.value)}
            />
            <label className="checkbox">
              <input type="checkbox" checked={autoPromote} onChange={(event) => setAutoPromote(event.target.checked)} />
              Auto promote validated candidates
            </label>
            <ErrorNotice error={submitError} what="Job not started" />
            <div className="toolbar">
              <button type="submit" className="primary" disabled={!canSubmit}>
                {submitting ? "Starting…" : "Start learning"}
              </button>
            </div>
          </fieldset>
        </form>

        <div className="pane-head">
          <h3>Jobs</h3>
          <button type="button" onClick={jobs.reload} disabled={jobs.loading}>
            {jobs.loading ? "Loading…" : "Refresh"}
          </button>
        </div>
        {jobs.loading && !rows ? <Loading what="jobs" /> : null}
        <ErrorNotice error={jobs.error} onRetry={jobs.reload} what="Could not list jobs" />
        <ErrorNotice error={pollError} what="Could not check a running job (still trying)" />
        {rows && rows.length === 0 ? <p className="muted">No learning jobs yet.</p> : null}
        {rows && rows.length > 0 ? (
          <div className="table-wrap">
            <table className="table selectable">
              <thead>
                <tr>
                  <th>Job</th>
                  <th>State</th>
                  <th>Stage</th>
                  <th>Source</th>
                </tr>
              </thead>
              <tbody>
                {rows.map((job) => (
                  <tr
                    key={job.id}
                    className={job.id === selectedId ? "selected" : undefined}
                    onClick={() => navigate("learn", job.id)}
                  >
                    <td>
                      <a className="mono" href={href("learn", job.id)} onClick={(e) => e.stopPropagation()}>
                        {job.id}
                      </a>
                    </td>
                    <td>
                      <Badge tone={stateTone(job.state)}>{job.state}</Badge>
                    </td>
                    <td>{job.stage}</td>
                    <td className="mono small">{job.source ? job.source : "—"}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        ) : null}
      </section>

      <section className="pane">
        {selected ? (
          <JobDetail job={selected} />
        ) : (
          <p className="muted">
            {id !== null && rows ? `Job ${id} is not in the list.` : "Start a job or select one to read its report."}
          </p>
        )}
      </section>
    </div>
  );
}
