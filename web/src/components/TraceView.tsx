import type { ReactNode } from "react";
import type { Act, Facts, Frame, RequestTrace, Step, Trace } from "../api/types";
import { list, scalar } from "../format";
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
  return act.arg === null || act.arg === undefined ? act.op : `${act.op}(${scalar(act.arg)})`;
}

export function FrameView({ frame }: { frame: Frame }) {
  return (
    <dl className="kv">
      <Row label="Frame">
        <code>#{frame.id}</code>
      </Row>
      <Row label="Intent">{frame.intent === null ? <None>none</None> : <code>{frame.intent}</code>}</Row>
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

function RequestView({ request, number, total }: { request: RequestTrace; number: number; total: number }) {
  const matched = list(request.matched);
  const steps = list(request.steps);
  return (
    <article className="request">
      <h3>
        Request {number} of {total}
      </h3>

      <Layer number={1} name="Understand">
        <dl className="kv">
          <Row label="Clause">
            <q>{request.clause}</q>
          </Row>
          <Row label="Intent">
            {request.intent === null || request.intent === undefined ? (
              <Badge tone="bad">not understood</Badge>
            ) : (
              <code>{request.intent}</code>
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
        {request.frame ? <FrameView frame={request.frame} /> : <None>no frame for this request</None>}
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
