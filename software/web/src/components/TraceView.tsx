import type { ReactNode } from "react";
import type { Act, Facts, Frame, RequestTrace, Step, Trace } from "../api/types";
import { list, percent, scalar } from "../format";
import { Badge, KnowledgeLink } from "./common";

function Row({ label, children }: { label: string; children: ReactNode }) {
  return (
    <>
      <dt>{label}</dt>
      <dd>{children}</dd>
    </>
  );
}

function None({ children = "none" }: { children?: ReactNode }) {
  return <span className="muted">{children}</span>;
}

function Chips({ values }: { values: readonly string[] }) {
  if (values.length === 0) return <None />;
  return (
    <span className="chips">
      {values.map((value, i) => (
        <code key={i}>{value}</code>
      ))}
    </span>
  );
}

export function FactsView({ facts }: { facts: Facts | null | undefined }) {
  const entries = Object.entries(facts ?? {});
  if (entries.length === 0) return <None />;
  return (
    <span className="chips">
      {entries.map(([name, value]) => (
        <code key={name}>
          {name} = {scalar(value)}
        </code>
      ))}
    </span>
  );
}

export function actText(act: Act | null | undefined): string {
  if (!act) return "—";
  return act.arg === undefined || act.arg === null ? act.op : `${act.op}(${scalar(act.arg)})`;
}

export function FrameView({ frame }: { frame: Frame }) {
  return (
    <dl className="kv">
      <Row label="Frame">
        <code>#{frame.id}</code>
      </Row>
      <Row label="Intent">{frame.intent ? <code>{frame.intent}</code> : <None>none</None>}</Row>
      <Row label="Facts">
        <FactsView facts={frame.facts} />
      </Row>
      <Row label="Agent acts">
        <Chips values={list(frame.agent_acts)} />
      </Row>
      <Row label="User acts">
        <Chips values={list(frame.user_acts)} />
      </Row>
      <Row label="Last asked">{frame.last_asked ? <code>{frame.last_asked}</code> : <None>nothing</None>}</Row>
      <Row label="Standing decision">
        {frame.standing ? <Badge tone="ok">{frame.standing}</Badge> : <None>none</None>}
      </Row>
      <Row label="Fetched / found">
        <code>{String(frame.fetched)}</code> / <code>{String(frame.found)}</code>
      </Row>
      <Row label="Steps done">
        <code>{frame.steps_done}</code>
      </Row>
    </dl>
  );
}

function StepView({ step, number }: { step: Step; number: number }) {
  const features = list(step.features);
  return (
    <li className="step">
      <div className="step-head">Step {number}</div>
      <dl className="kv">
        <Row label="Rule">
          {step.rule ? (
            <span className="rule">
              <Badge tone="ok">rule #{step.rule.index} fired</Badge>
              <span className="rule-part">
                <span className="label">when</span> <Chips values={list(step.rule.when)} />
              </span>
              <span className="rule-part">
                <span className="label">do</span> <code>{step.rule.do}</code>
              </span>
            </span>
          ) : (
            <Badge tone="warn">no rule fired</Badge>
          )}
        </Row>
        <Row label="Act">
          <code className="act">{actText(step.act)}</code>
          {step.act ? (
            <span className="muted small">
              {" "}
              op <code>{step.act.op}</code>, arg{" "}
              {step.act.arg === null || step.act.arg === undefined ? "none" : <code>{scalar(step.act.arg)}</code>}
            </span>
          ) : null}
        </Row>
      </dl>
      <details className="features">
        <summary>State features that were true ({features.length})</summary>
        <Chips values={features} />
      </details>
    </li>
  );
}

function Layer({ number, name, children }: { number: number; name: string; children: ReactNode }) {
  return (
    <section className={`layer layer-${number}`}>
      <h4>
        <span className="layer-number">{number}</span> {name}
      </h4>
      <div className="layer-body">{children}</div>
    </section>
  );
}

const KIND_WORDS: Record<string, string> = {
  gated: "a rule check",
  procedure: "a step-by-step procedure",
  lookup: "a record lookup",
};

/** Why the last act was chosen, in a sentence. */
function reason(request: RequestTrace, act: Act | null | undefined): string {
  switch (act?.op) {
    case "ask":
      return act.arg === "subject"
        ? "A reference is needed to look this up, and none was given, so it asks for one."
        : `The rule depends on “${act.arg ?? "something"}”, which is not known yet, so it asks.`;
    case "grant":
      return "Every condition of the rule holds, so the request is granted.";
    case "offer":
      return "The main rule does not apply, but the alternative does, so that is offered instead.";
    case "deny":
      return "A condition of the rule is not met and no alternative applies, so the request is refused.";
    case "confirm":
      return "A fact was corrected, but the earlier decision still stands, so it is confirmed.";
    case "report":
      return "The record was found, so its state is reported.";
    case "instruct":
      return "This is the next step of the procedure that is not done yet.";
    case "repeat":
      return "The last step did not work, so it is given once more.";
    case "finish":
      return "All steps of the procedure are done.";
    case "escalate":
      return request.frame?.fetched && !request.frame.found
        ? "The record could not be found, so it hands over."
        : "Nothing further can be tried here, so it hands over.";
    case "teach_me":
      return request.intent || request.frame
        ? "The request was understood, but the vault has no trusted pattern or knowledge to answer it."
        : "No phrase the agent knows matched, and no remembered message was similar enough.";
    default:
      return "";
  }
}

/** A one-line label for where a reply came from, shown under the message. */
export function sourceLabel(trace: Trace | null | undefined): string {
  return list(trace?.requests)
    .map((r) => {
      const last = list(r.steps).at(-1);
      if (r.recalled) return `remembered reply · ${percent(r.recalled.score)} match`;
      if (r.pattern && last?.rule) return `${r.pattern} · rule ${last.rule.index} · ${actText(last.act)}`;
      if (last?.act?.op === "say") return "stock reply";
      if (last?.act?.op === "teach_me") return r.intent || r.frame ? "no pattern for this" : "not understood";
      return actText(last?.act);
    })
    .join("  +  ");
}

function WhyView({ request }: { request: RequestTrace }) {
  const steps = list(request.steps);
  const last = steps.at(-1);
  const intentPhrases = list(request.matched).filter((m) => m.kind === "intent");
  const facts = request.frame?.facts ?? request.facts;
  const intent = request.intent ?? request.frame?.intent ?? null;

  if (request.recalled) {
    const r = request.recalled;
    const width = `${Math.max(0, Math.min(100, Math.round(r.score * 100)))}%`;
    const mark = `${Math.round(r.threshold * 100)}%`;
    return (
      <section className="why why-recalled">
        <h4>Why this answer</h4>
        <p className="why-head">It reused a reply it remembered from a taught chat.</p>
        <ol className="why-steps">
          <li>
            {request.intent ? (
              <>
                The message was read as small talk (<code>{request.intent}</code>), which no rule or procedure covers.
              </>
            ) : (
              <>The message was not recognised as a request the vault has a rule or procedure for.</>
            )}
          </li>
          <li>
            The closest message it remembers is <q>{r.message}</q>, from <KnowledgeLink id={r.source} />, entry{" "}
            {r.entry}.
          </li>
          <li>
            <span className="meter" title={`needs ${mark}`}>
              <span className="meter-fill" style={{ width }} />
              <span className="meter-mark" style={{ left: mark }} />
            </span>{" "}
            <strong>{percent(r.score)}</strong> similar by shared words; {mark} is enough to reuse a reply.
          </li>
          <li>The reply that followed that message is given unchanged. Nothing checks that it fits this conversation.</li>
        </ol>
      </section>
    );
  }

  if (last?.act?.op === "say") {
    return (
      <section className="why">
        <h4>Why this answer</h4>
        <p className="why-head">It gave a stock reply.</p>
        <ol className="why-steps">
          <li>
            The message was read as <code>{intent ?? "small talk"}</code>
            {intentPhrases.length > 0 ? (
              <>
                {" "}
                because of <q>{intentPhrases[0]?.phrase}</q>
              </>
            ) : null}
            .
          </li>
          <li>
            The vault has a fixed reply for that: <KnowledgeLink id={last.act.arg} />. No remembered chat message was
            similar enough to use instead.
          </li>
        </ol>
      </section>
    );
  }

  if (!request.pattern) {
    return (
      <section className="why why-none">
        <h4>Why this answer</h4>
        <p className="why-head">It had nothing to answer with.</p>
        <ol className="why-steps">
          {intent ? (
            <li>
              The message was read as <code>{intent}</code>
              {KIND_WORDS[request.kind] ? <>, {KIND_WORDS[request.kind]}</> : null}.
            </li>
          ) : null}
          <li>{reason(request, last?.act)}</li>
        </ol>
      </section>
    );
  }

  return (
    <section className="why">
      <h4>Why this answer</h4>
      <p className="why-head">
        It followed the learned pattern <KnowledgeLink id={request.pattern} /> for {KIND_WORDS[request.kind] ?? request.kind}.
      </p>
      <ol className="why-steps">
        <li>
          {request.intent ? (
            <>
              The message was read as <code>{request.intent}</code>
              {intentPhrases.length > 0 ? (
                <>
                  {" "}
                  because of <q>{intentPhrases[0]?.phrase}</q>
                </>
              ) : null}
              .
            </>
          ) : (
            <>
              The message continues the open request <code>{intent ?? "?"}</code>
              {request.user_act ? (
                <>
                  {" "}
                  (<code>{request.user_act}</code>)
                </>
              ) : null}
              .
            </>
          )}
        </li>
        <li>
          What it knows about this request: <FactsView facts={facts} />
        </li>
        {request.atom ? (
          <li>
            The rule that applies is <KnowledgeLink id={request.atom} />
            {request.alternative ? (
              <>
                , with <KnowledgeLink id={request.alternative} /> as the alternative
              </>
            ) : null}
            .
          </li>
        ) : null}
        {steps.map((step, i) => (
          <li key={i}>
            {step.rule ? (
              <>
                Rule {step.rule.index} of the pattern matched (<Chips values={list(step.rule.when)} />) and says{" "}
                <code>{step.rule.do}</code>, which here means <code className="act">{actText(step.act)}</code>.
              </>
            ) : (
              <>
                No rule of the pattern matched; it chose <code className="act">{actText(step.act)}</code>.
              </>
            )}
            {step.act?.op === "lookup" ? " The record was looked up silently, then the pattern was asked again." : null}
          </li>
        ))}
        <li>{reason(request, last?.act)}</li>
      </ol>
    </section>
  );
}

function RequestView({ request, number, total }: { request: RequestTrace; number: number; total: number }) {
  const matched = list(request.matched);
  const steps = list(request.steps);
  return (
    <article className="request">
      <h3>
        Request {number} of {total}
      </h3>

      <WhyView request={request} />

      <h4 className="detail-head">Detail by layer</h4>

      <Layer number={1} name="Understand">
        <dl className="kv">
          <Row label="Clause">
            <q>{request.clause}</q>
          </Row>
          <Row label="Intent">
            {request.intent ? (
              <code>{request.intent}</code>
            ) : request.frame ? (
              <None>
                none in this clause — it continues frame <code>#{request.frame.id}</code>
                {request.frame.intent ? (
                  <>
                    {" "}
                    (<code>{request.frame.intent}</code>)
                  </>
                ) : null}
              </None>
            ) : request.recalled ? (
              <None>none — not a request; answered from a remembered chat</None>
            ) : (
              <Badge tone="bad">not understood</Badge>
            )}
          </Row>
          <Row label="Matched phrases">
            {matched.length === 0 ? (
              <None />
            ) : (
              <ul className="plain">
                {matched.map((m, i) => (
                  <li key={i}>
                    <q>{m.phrase}</q> → <span className="muted">{m.kind}</span> <code>{m.target}</code>
                  </li>
                ))}
              </ul>
            )}
          </Row>
          <Row label="Facts read">
            <FactsView facts={request.facts} />
          </Row>
          <Row label="User act">{request.user_act ? <code>{request.user_act}</code> : <None>none</None>}</Row>
        </dl>
      </Layer>

      <Layer number={2} name="Track">
        {request.frame ? (
          <FrameView frame={request.frame} />
        ) : request.recalled ? (
          <None>no frame — a remembered reply needs no state</None>
        ) : (
          <None>no frame — the request was not understood</None>
        )}
      </Layer>

      <Layer number={3} name="Decide">
        <dl className="kv">
          <Row label="Request kind">
            {request.kind === "unknown" ? <Badge tone="bad">unknown</Badge> : <code>{request.kind}</code>}
          </Row>
          <Row label="Pattern">
            <KnowledgeLink id={request.pattern} />
          </Row>
          <Row label="Atom">
            <KnowledgeLink id={request.atom} />
          </Row>
          <Row label="Alternative">
            <KnowledgeLink id={request.alternative} />
          </Row>
        </dl>
        {steps.length === 0 ? (
          <p className="muted">No steps.</p>
        ) : (
          <ol className="steps">
            {steps.map((step, i) => (
              <StepView key={i} step={step} number={i + 1} />
            ))}
          </ol>
        )}
      </Layer>

      <Layer number={4} name="Realize">
        {request.text ? <p className="realized">{request.text}</p> : <None>no text produced</None>}
      </Layer>
    </article>
  );
}

export function TraceView({ trace }: { trace: Trace }) {
  const requests = list(trace.requests);
  const notes = list(trace.notes);
  return (
    <div className="trace">
      {requests.length === 0 ? <p className="muted">The trace lists no requests for this message.</p> : null}
      {requests.map((request, i) => (
        <RequestView key={i} request={request} number={i + 1} total={requests.length} />
      ))}
      {notes.length > 0 ? (
        <section className="notes">
          <h4>Engine notes</h4>
          <ul>
            {notes.map((note, i) => (
              <li key={i}>{note}</li>
            ))}
          </ul>
        </section>
      ) : null}
    </div>
  );
}
