import { useState } from "react";
import { getHealth, mockMode, UNREACHABLE_MESSAGE } from "./api/client";
import { ErrorBoundary } from "./components/common";
import { useConnection } from "./hooks";
import { href, useRoute, VIEWS, type Route, type ViewName } from "./router";
import { CandidatesView } from "./views/CandidatesView";
import { ChatView } from "./views/ChatView";
import { KnowledgeView } from "./views/KnowledgeView";
import { LearnView } from "./views/LearnView";
import { StatusView } from "./views/StatusView";

const LABELS: Record<ViewName, string> = {
  chat: "Chat",
  knowledge: "Knowledge",
  candidates: "Candidates",
  learn: "Learn",
  status: "Status",
};

function ConnectionBanner() {
  const { reachable } = useConnection();
  const [checking, setChecking] = useState(false);
  if (reachable) return null;

  // A successful call marks the engine reachable again, which reloads every view's data.
  const retry = () => {
    setChecking(true);
    getHealth()
      .catch(() => undefined)
      .finally(() => setChecking(false));
  };

  return (
    <div className="banner" role="alert">
      <span>{UNREACHABLE_MESSAGE}</span>
      <button type="button" onClick={retry} disabled={checking}>
        {checking ? "Checking…" : "Retry"}
      </button>
    </div>
  );
}

function View({ route }: { route: Route }) {
  switch (route.view) {
    case "chat":
      return <ChatView id={route.id} />;
    case "knowledge":
      return <KnowledgeView id={route.id} />;
    case "candidates":
      return <CandidatesView id={route.id} />;
    case "learn":
      return <LearnView id={route.id} />;
    case "status":
      return <StatusView />;
  }
}

export function App() {
  const route = useRoute();
  return (
    <div className="app">
      <header className="topbar">
        <span className="brand">Charlotte</span>
        <nav aria-label="Views">
          {VIEWS.map((view) => (
            <a key={view} href={href(view)} className={view === route.view ? "active" : undefined}>
              {LABELS[view]}
            </a>
          ))}
        </nav>
        {mockMode ? <span className="badge warn">mock data — not the engine</span> : null}
      </header>
      <ConnectionBanner />
      <main>
        <ErrorBoundary key={route.view}>
          <View route={route} />
        </ErrorBoundary>
      </main>
    </div>
  );
}
