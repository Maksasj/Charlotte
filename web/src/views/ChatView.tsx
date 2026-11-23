import { useEffect, useRef, useState } from "react";
import {
  closeConversation,
  createConversation,
  getConversation,
  listConversations,
  sendMessage,
} from "../api/client";
import type { Conversation, TrustChange, Verdict } from "../api/types";
import { Badge, ErrorNotice, KnowledgeLink, Loading } from "../components/common";
import { FrameView, TraceView } from "../components/TraceView";
import { list, percent } from "../format";
import { useLoad } from "../hooks";
import { href, navigate } from "../router";

const VERDICTS: { verdict: Verdict; label: string }[] = [
  { verdict: "resolved", label: "Resolved" },
  { verdict: "partial", label: "Partial" },
  { verdict: "failed", label: "Failed" },
];

function verdictTone(verdict: Verdict | null): "ok" | "warn" | "bad" | "plain" {
  if (verdict === "resolved") return "ok";
  if (verdict === "partial") return "warn";
  if (verdict === "failed") return "bad";
  return "plain";
}

type Busy = "send" | "close" | "create" | null;

export function ChatView({ id }: { id: string | null }) {
  const conversations = useLoad(listConversations, "conversations");
  const conversation = useLoad(() => getConversation(id ?? ""), id);
  const [draft, setDraft] = useState("");
  const [busy, setBusy] = useState<Busy>(null);
  const [actionError, setActionError] = useState<{ what: string; error: unknown } | null>(null);
  const [picked, setPicked] = useState<{ conversation: string; index: number } | null>(null);
  const [trustChanges, setTrustChanges] = useState<Record<string, TrustChange[]>>({});
  const inputRef = useRef<HTMLInputElement>(null);
  const messagesRef = useRef<HTMLDivElement>(null);

  const summaries = conversations.data;
  const current: Conversation | null = conversation.data;
  const messages = list(current?.messages);

  // With no conversation in the URL, open the newest one.
  const newest = summaries?.[0]?.id;
  useEffect(() => {
    if (id === null && newest !== undefined) navigate("chat", newest, true);
  }, [id, newest]);

  useEffect(() => {
    setActionError(null);
  }, [id]);

  const messageCount = messages.length;
  useEffect(() => {
    const box = messagesRef.current;
    if (box) box.scrollTop = box.scrollHeight;
  }, [messageCount, id]);

  useEffect(() => {
    if (busy === null) inputRef.current?.focus();
  }, [busy, id]);

  let latestAgent = -1;
  messages.forEach((message, index) => {
    if (message.role === "agent") latestAgent = index;
  });
  const pickedIndex = picked !== null && picked.conversation === id ? picked.index : latestAgent;
  const pickedMessage = messages[pickedIndex];
  const tracedMessage = pickedMessage && pickedMessage.role === "agent" ? pickedMessage : null;

  const closed = current?.closed === true;
  const canSend = id !== null && current !== null && !closed && busy === null;

  const create = () => {
    if (busy !== null) return;
    setBusy("create");
    setActionError(null);
    createConversation()
      .then((created) => {
        conversations.reload();
        navigate("chat", created.id);
      })
      .catch((error: unknown) => setActionError({ what: "Could not start a conversation", error }))
      .finally(() => setBusy(null));
  };

  const send = () => {
    const text = draft.trim();
    if (!canSend || id === null || text === "") return;
    setBusy("send");
    setActionError(null);
    sendMessage(id, text)
      .then((reply) => {
        conversation.setData((previous) =>
          previous && previous.id === id
            ? {
                ...previous,
                messages: [
                  ...list(previous.messages),
                  { role: "user", text },
                  { role: "agent", text: reply.text, trace: reply.trace },
                ],
              }
            : previous,
        );
        setDraft("");
        setPicked(null);
        // The reply is already on screen; this brings the frames and the list up to date.
        conversation.reload();
        conversations.reload();
      })
      .catch((error: unknown) => setActionError({ what: "Message not sent", error }))
      .finally(() => setBusy(null));
  };

  const close = (verdict: Verdict) => {
    if (id === null || busy !== null) return;
    setBusy("close");
    setActionError(null);
    closeConversation(id, verdict)
      .then((result) => {
        setTrustChanges((previous) => ({ ...previous, [id]: list(result.updated).slice() }));
        conversation.setData((previous) =>
          previous && previous.id === id ? { ...previous, closed: true, verdict } : previous,
        );
        conversation.reload();
        conversations.reload();
      })
      .catch((error: unknown) => setActionError({ what: "Conversation not closed", error }))
      .finally(() => setBusy(null));
  };

  const changes = id !== null ? trustChanges[id] : undefined;

  return (
    <div className="chat">
      <aside className="pane conversations">
        <div className="pane-head">
          <h2>Conversations</h2>
          <button type="button" className="primary" onClick={create} disabled={busy !== null}>
            {busy === "create" ? "Starting…" : "New conversation"}
          </button>
        </div>
        {conversations.loading && !summaries ? <Loading what="conversations" /> : null}
        <ErrorNotice error={conversations.error} onRetry={conversations.reload} what="Could not list conversations" />
        {summaries && summaries.length === 0 ? <p className="muted">No conversations yet.</p> : null}
        <ul className="conversation-list">
          {list(summaries).map((summary) => (
            <li key={summary.id}>
              <a href={href("chat", summary.id)} className={summary.id === id ? "selected" : undefined}>
                <span className="row">
                  <span className="mono">{summary.id}</span>
                  <span className="muted small">{summary.turns} turns</span>
                  {summary.closed ? (
                    <Badge tone={verdictTone(summary.verdict)}>{summary.verdict ?? "closed"}</Badge>
                  ) : (
                    <Badge>open</Badge>
                  )}
                </span>
                <span className="last">{summary.last ? summary.last : "(no messages)"}</span>
              </a>
            </li>
          ))}
        </ul>
      </aside>

      <section className="pane transcript">
        {id === null ? (
          <p className="muted">
            {summaries && summaries.length === 0
              ? "Start a new conversation to talk to the agent."
              : "Select a conversation."}
          </p>
        ) : (
          <>
            <div className="pane-head">
              <h2>
                Conversation <span className="mono">{id}</span>{" "}
                {current ? (
                  closed ? (
                    <Badge tone={verdictTone(current.verdict)}>closed: {current.verdict ?? "no verdict"}</Badge>
                  ) : (
                    <Badge>open</Badge>
                  )
                ) : null}
              </h2>
              {current && !closed ? (
                <div className="toolbar">
                  <span className="muted small">Close as</span>
                  {VERDICTS.map(({ verdict, label }) => (
                    <button key={verdict} type="button" disabled={busy !== null} onClick={() => close(verdict)}>
                      {label}
                    </button>
                  ))}
                </div>
              ) : null}
            </div>

            {conversation.loading && !current ? <Loading what="conversation" /> : null}
            <ErrorNotice error={conversation.error} onRetry={conversation.reload} what="Could not load the conversation" />

            {current ? (
              <div className="messages" ref={messagesRef}>
                {messages.length === 0 ? <p className="muted">No messages yet. Say something below.</p> : null}
                {messages.map((message, index) =>
                  message.role === "agent" ? (
                    <button
                      key={index}
                      type="button"
                      className={`message agent${index === pickedIndex ? " selected" : ""}`}
                      aria-pressed={index === pickedIndex}
                      title="Show the trace of this reply"
                      onClick={() => setPicked({ conversation: id, index })}
                    >
                      <span className="who">agent</span>
                      <span className="text">{message.text}</span>
                    </button>
                  ) : (
                    <div key={index} className="message user">
                      <span className="who">{message.role}</span>
                      <span className="text">{message.text}</span>
                    </div>
                  ),
                )}
                {busy === "send" ? <p className="muted loading">Waiting for the reply…</p> : null}
              </div>
            ) : null}

            {current && closed ? (
              <div className="closed-summary">
                <p>
                  This conversation is closed with verdict{" "}
                  <Badge tone={verdictTone(current.verdict)}>{current.verdict ?? "none"}</Badge>.
                </p>
                {changes ? (
                  changes.length === 0 ? (
                    <p className="muted">The engine reported no trust changes.</p>
                  ) : (
                    <table className="table compact">
                      <caption>Trust changes</caption>
                      <thead>
                        <tr>
                          <th>Item</th>
                          <th>Before</th>
                          <th></th>
                          <th>After</th>
                        </tr>
                      </thead>
                      <tbody>
                        {changes.map((change) => (
                          <tr key={change.id}>
                            <td>
                              <KnowledgeLink id={change.id} />
                            </td>
                            <td className="num">{percent(change.before)}</td>
                            <td>→</td>
                            <td className="num">{percent(change.after)}</td>
                          </tr>
                        ))}
                      </tbody>
                    </table>
                  )
                ) : null}
              </div>
            ) : null}

            {actionError ? <ErrorNotice error={actionError.error} what={actionError.what} /> : null}

            <form
              className="composer"
              onSubmit={(event) => {
                event.preventDefault();
                send();
              }}
            >
              <input
                ref={inputRef}
                type="text"
                aria-label="Message"
                placeholder={closed ? "This conversation is closed" : "Type a message and press Enter"}
                value={draft}
                disabled={!canSend}
                onChange={(event) => setDraft(event.target.value)}
              />
              <button type="submit" className="primary" disabled={!canSend || draft.trim() === ""}>
                {busy === "send" ? "Sending…" : "Send"}
              </button>
            </form>
          </>
        )}
      </section>

      <aside className="pane trace-pane">
        <div className="pane-head">
          <h2>Trace</h2>
          {tracedMessage ? (
            <span className="muted small">
              {pickedIndex === latestAgent ? "latest reply" : "selected reply"} · message {pickedIndex + 1} of{" "}
              {messages.length}
            </span>
          ) : null}
        </div>
        {tracedMessage ? (
          <>
            <blockquote className="traced-reply">{tracedMessage.text}</blockquote>
            {tracedMessage.trace ? (
              <TraceView trace={tracedMessage.trace} />
            ) : (
              <p className="muted">This reply carries no trace.</p>
            )}
          </>
        ) : (
          <p className="muted">
            {current ? "No agent reply yet. The trace of each reply appears here." : "No conversation selected."}
          </p>
        )}
        {current && list(current.frames).length > 0 ? (
          <details className="frames">
            <summary>Engine state now: {list(current.frames).length} frame(s)</summary>
            {list(current.frames).map((frame, index) => (
              <FrameView key={index} frame={frame} />
            ))}
          </details>
        ) : null}
      </aside>
    </div>
  );
}
