import { TOKEN, usePoll, type Curriculum, type TrainState } from './api';
import { Alerts, Controls, Feed, HealthPanel, Lanes, Limits, Progress, Questions, ScenarioGrid, TeachLog, Teachers } from './panels';

function Panel({ title, children, wide }: { title: string; children: React.ReactNode; wide?: boolean }) {
  return (
    <section className={`panel${wide ? ' wide' : ''}`} aria-label={title}>
      <h2>{title}</h2>
      {children}
    </section>
  );
}

export default function App() {
  const [c, cErr] = usePoll<Curriculum>('/api/curriculum', 2000);
  const [s] = usePoll<TrainState>('/api/state', 5000);
  const run = c?.run;
  const ago = run ? Math.max(0, Math.round(Date.now() / 1000 - run.last_t)) : 0;
  // A training stage has many more scenarios than the 20 held-out ones.
  const training = (run?.grid.length ?? 0) > 20;
  return (
    <main>
      <header>
        <h1>Training dashboard</h1>
        <Controls c={c} />
      </header>
      {!TOKEN && <p className="error">No token: open the address sla_trainer printed, with ?token=…, or add it to this page's address.</p>}
      {cErr && <p className="error">API: {cErr}</p>}
      {run && (
        <p className="muted">
          Run <span className="mono">{run.name}</span> · {run.messages.toLocaleString()} messages · last message {ago} s ago{training ? ' · training stage' : ' · held-out stage'}
        </p>
      )}
      <Alerts run={run} />
      {run && <Progress kinds={run.kinds} training={training} />}
      <div className="panels">
        {run && run.recent.some((e) => (e.teacher ?? 0) > 0) && (
          <Panel title="Teachers (each one's current conversation)" wide>
            <Teachers recent={run.recent} />
          </Panel>
        )}
        <Panel title="Conversations (newest first)" wide>
          {run ? <Lanes recent={run.recent} /> : <p className="muted">No run yet.</p>}
        </Panel>
        <Panel title="Scenario grid" wide>
          {run ? <ScenarioGrid grid={run.grid} /> : null}
        </Panel>
        <Panel title={`Question stream${run ? ` (${run.questions.length})` : ''}`}>{run ? <Questions run={run} /> : null}</Panel>
        <Panel title={`Teach log${run ? ` (${run.teach.length})` : ''}`}>{run ? <TeachLog run={run} /> : null}</Panel>
        <Panel title="Learning feed">
          <Feed feed={c?.feed} />
        </Panel>
        <Panel title="Knowledge health and teacher quality">{c ? <HealthPanel c={c} /> : null}</Panel>
        <Panel title="Plan limits and cost">
          <Limits s={s} />
        </Panel>
      </div>
    </main>
  );
}
