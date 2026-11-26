import { useEffect, useRef, useState } from 'react';
import { api } from './api';
import { isSpeechRecognitionSupported, listenOnce, type ListenHandle } from './speech';
import { karinVoice, type VoiceStatus } from './voice';
import { fmtAction, fmtCond } from './format';
import { FactRow, PatternRow } from './Rows';
import { VaultExplorer, vaultPathOf } from './SourceTree';
import { factDsl, patternDsl } from './VaultSource';
import { GraphView } from './GraphView';
import { TracePanel } from './TracePanel';
import type {
  AgentState,
  Fact,
  Pattern,
  Scope,
  SelfTestResult,
  ToolInfo,
  TurnTrace,
} from './types';

type Tab = 'trace' | 'knowledge' | 'tools' | 'tests';
type KnowKind = 'all' | 'facts' | 'patterns';
/** One entry of the Knowledge view: a fact or a pattern (rule). */
type KItem = { kind: 'fact'; fact: Fact } | { kind: 'pattern'; pattern: Pattern };

const SUGGESTIONS = [
  'who are you',
  'my name is Alex',
  'weather',
  'calculate (12 + 8) * 3',
  'define anti-unification',
  'remember buy coffee',
  'generalize',
  'chunk last',
];

/** "understood as …" note for a reply, when the language layer changed the wording. */
function understoodNote(t: TurnTrace): string | null {
  const norm = (x: string) => x.toLowerCase().replace(/[^a-z0-9 ]+/g, '').replace(/\s+/g, ' ').trim();
  const understood = t.understood ?? [];
  const missed = t.missed ?? [];
  const parts: string[] = [];
  if (understood.length > 0 && norm(understood.join(' ')) !== norm(t.input)) {
    parts.push(`understood as ${understood.map((u) => `“${u}”`).join(' + ')}`);
  }
  if (missed.length > 0 && !t.impasse) {
    parts.push(`missed ${missed.map((m) => `“${m}”`).join(', ')}`);
  }
  return parts.length > 0 ? parts.join(' · ') : null;
}

export default function App() {
  // Remembered across reloads; first visit follows the system setting.
  const [theme, setTheme] = useState<'light' | 'dark'>(() => {
    const saved = localStorage.getItem('karin-theme');
    if (saved === 'light' || saved === 'dark') return saved;
    return window.matchMedia?.('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
  });
  const [state, setState] = useState<AgentState | null>(null);
  const [tools, setTools] = useState<ToolInfo[]>([]);
  const [tab, setTab] = useState<Tab>(() => {
    const fromHash = window.location.hash.replace('#', '');
    if (fromHash === 'facts' || fromHash === 'patterns' || fromHash === 'graph') return 'knowledge';
    return (['trace', 'knowledge', 'tools', 'tests'] as Tab[]).includes(fromHash as Tab)
      ? (fromHash as Tab)
      : 'trace';
  });
  const [input, setInput] = useState('');
  const [selectedTrace, setSelectedTrace] = useState<TurnTrace | null>(null);
  const [wideInspector, setWideInspector] = useState(false);
  const [showAddFact, setShowAddFact] = useState(false);
  const [statusMsg, setStatusMsg] = useState<string | null>(null);
  const [errorMsg, setErrorMsg] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  const [knowFilter, setKnowFilter] = useState('');
  const [knowView, setKnowView] = useState<'tree' | 'graph'>(() =>
    window.location.hash === '#graph' ? 'graph' : 'tree',
  );
  const [knowKind, setKnowKind] = useState<KnowKind>(() => {
    const h = window.location.hash.replace('#', '');
    return h === 'facts' || h === 'patterns' ? h : 'all';
  });

  const [teachReply, setTeachReply] = useState('');
  const [teachPromote, setTeachPromote] = useState(true);

  const [factRel, setFactRel] = useState('weather_city');
  const [factArgsText, setFactArgsText] = useState('city=Berlin, condition=sunny, temp=21°C');
  const [factScope, setFactScope] = useState<Scope>('long_term');

  const [tests, setTests] = useState<SelfTestResult[]>([]);
  const [voiceEnabled, setVoiceEnabled] = useState(false);
  const [voiceStatus, setVoiceStatus] = useState<VoiceStatus>('idle');
  const [dialogMode, setDialogMode] = useState<'text' | 'voice'>('text');
  const [voiceDialogPhase, setVoiceDialogPhase] = useState<
    'idle' | 'loading' | 'listening' | 'thinking' | 'speaking'
  >('idle');
  const [handsFree, setHandsFree] = useState(true);
  const [liveTranscript, setLiveTranscript] = useState('');

  const messagesEndRef = useRef<HTMLDivElement>(null);
  const voiceLoopRunId = useRef(0);
  const listenHandleRef = useRef<ListenHandle | null>(null);
  const handsFreeRef = useRef(handsFree);
  handsFreeRef.current = handsFree;

  const speechSupported = isSpeechRecognitionSupported();

  function looksLikeEcho(heard: string, karinReply: string): boolean {
    const norm = (s: string) =>
      s
        .toLowerCase()
        .replace(/[^a-z0-9\s]/g, ' ')
        .replace(/\s+/g, ' ')
        .trim();
    const h = norm(heard);
    if (!h) return true;
    const r = norm(karinReply);
    if (!r) return false;
    return r.includes(h);
  }

  function stopVoiceDialog() {
    voiceLoopRunId.current += 1;
    listenHandleRef.current?.stop();
    listenHandleRef.current = null;
    karinVoice.stop();
    setLiveTranscript('');
    setVoiceDialogPhase('idle');
  }

  async function startVoiceDialog() {
    if (!speechSupported) {
      setErrorMsg('Speech recognition is not supported in this browser (try Chrome or Safari).');
      return;
    }
    stopVoiceDialog();
    const runId = ++voiceLoopRunId.current;
    setErrorMsg(null);
    setStatusMsg(null);

    try {
      setVoiceDialogPhase('loading');
      await karinVoice.ensureLoaded();
    } catch (e) {
      if (runId === voiceLoopRunId.current) {
        setErrorMsg(`Voice load failed: ${(e as Error).message}`);
        setVoiceDialogPhase('idle');
      }
      return;
    }

    let pendingHeard: string | null = null;

    while (runId === voiceLoopRunId.current) {
      let heard = pendingHeard;
      pendingHeard = null;

      if (!heard) {
        setVoiceDialogPhase('listening');
        setLiveTranscript('');

        const handle = listenOnce({
          onInterim: (t) => {
            if (runId === voiceLoopRunId.current) setLiveTranscript(t);
          },
        });
        listenHandleRef.current = handle;

        try {
          heard = await handle.promise;
        } catch (e) {
          if (runId === voiceLoopRunId.current) {
            setErrorMsg((e as Error).message);
            setVoiceDialogPhase('idle');
          }
          return;
        } finally {
          if (listenHandleRef.current === handle) listenHandleRef.current = null;
        }

        if (runId !== voiceLoopRunId.current) return;

        if (!heard) {
          if (!handsFreeRef.current) {
            setVoiceDialogPhase('idle');
            return;
          }
          await new Promise((r) => setTimeout(r, 200));
          continue;
        }
      }

      setLiveTranscript(heard);
      setVoiceDialogPhase('thinking');
      setBusy(true);
      let reply = '';
      try {
        const res = await api.chat(heard);
        if (runId !== voiceLoopRunId.current) return;
        setState(res.state);
        setSelectedTrace(res.trace);
        setTeachReply('');
        reply = res.trace.reply;
      } catch (e) {
        if (runId === voiceLoopRunId.current) {
          setErrorMsg((e as Error).message);
          setVoiceDialogPhase('idle');
        }
        return;
      } finally {
        setBusy(false);
      }

      if (runId !== voiceLoopRunId.current) return;
      setLiveTranscript('');
      setVoiceDialogPhase('speaking');

      let karinSpeaking = true;
      let bargedIn = false;

      const speakPromise = karinVoice
        .speak(reply)
        .catch((e) => {
          if (runId === voiceLoopRunId.current) {
            setErrorMsg((e as Error).message);
          }
        })
        .finally(() => {
          karinSpeaking = false;
          if (runId === voiceLoopRunId.current && !bargedIn && handsFreeRef.current) {
            setVoiceDialogPhase('listening');
          }
        });

      if (!handsFreeRef.current) {
        await speakPromise;
        if (runId === voiceLoopRunId.current) {
          setVoiceDialogPhase('idle');
        }
        return;
      }

      // Keep barge-in listener active throughout Karin's speech so user speech cuts her off immediately
      while (runId === voiceLoopRunId.current && !pendingHeard) {
        const bargeHandle = listenOnce({
          onInterim: (t) => {
            if (runId !== voiceLoopRunId.current) return;
            if (karinSpeaking) {
              if (looksLikeEcho(t, reply)) return;
              bargedIn = true;
              karinVoice.stop();
              setVoiceDialogPhase('listening');
            }
            setLiveTranscript(t);
          },
        });
        listenHandleRef.current = bargeHandle;

        let nextHeard: string | null = null;
        try {
          nextHeard = await bargeHandle.promise;
        } catch (e) {
          karinVoice.stop();
          if (runId === voiceLoopRunId.current) {
            setErrorMsg((e as Error).message);
            setVoiceDialogPhase('idle');
          }
          return;
        } finally {
          if (listenHandleRef.current === bargeHandle) listenHandleRef.current = null;
        }

        if (runId !== voiceLoopRunId.current) return;

        if (nextHeard && (bargedIn || !karinSpeaking || !looksLikeEcho(nextHeard, reply))) {
          karinVoice.stop();
          pendingHeard = nextHeard;
          break;
        }

        if (!karinSpeaking) break;
      }

      if (!pendingHeard) {
        await speakPromise;
      }
    }
  }

  useEffect(() => {
    return () => {
      voiceLoopRunId.current += 1;
      listenHandleRef.current?.stop();
      karinVoice.stop();
    };
  }, []);

  useEffect(() => {
    return karinVoice.subscribe((status, detail) => {
      setVoiceStatus(status);
      if (status === 'error' && detail) {
        setErrorMsg(`Voice error: ${detail}`);
      }
    });
  }, []);

  useEffect(() => {
    document.documentElement.setAttribute('data-theme', theme);
    localStorage.setItem('karin-theme', theme);
  }, [theme]);

  async function loadInitial() {
    try {
      const data = await api.getState();
      setState(data.state);
      setTools(data.tools);
      const lastTrace = [...data.state.messages].reverse().find((m) => m.trace)?.trace;
      if (lastTrace) setSelectedTrace(lastTrace);
      setErrorMsg(null);
    } catch (e) {
      setErrorMsg(`Cannot reach Rust backend on /api: ${(e as Error).message}`);
    }
  }

  useEffect(() => {
    loadInitial();
  }, []);

  // Hot reload: the backend reloads the vault when its files change; follow it.
  const [vaultVersion, setVaultVersion] = useState(0);
  const seenVaultVersion = useRef<number | null>(null);
  useEffect(() => {
    let alive = true;
    const check = async () => {
      try {
        const status = await api.vaultStatus();
        if (!alive) return;
        const seen = seenVaultVersion.current;
        seenVaultVersion.current = status.version;
        if (seen === null || status.version === seen) return;
        setVaultVersion(status.version);
        if (status.ok) {
          const data = await api.getState();
          if (alive) setState(data.state);
          const added = status.addedRules.length;
          const removed = status.removedRules.length;
          const changes = [
            added && `+${added} ${added === 1 ? 'rule' : 'rules'}`,
            removed && `-${removed} ${removed === 1 ? 'rule' : 'rules'}`,
          ].filter(Boolean);
          setStatusMsg(
            `Vault reloaded${changes.length ? `: ${changes.join(', ')}` : ''} · ${status.rules} rules, ${status.facts} vault facts`,
          );
        } else {
          setStatusMsg(status.message);
        }
      } catch {
        /* backend not reachable or too old: ignore */
      }
    };
    check();
    const id = window.setInterval(check, 2000);
    return () => {
      alive = false;
      window.clearInterval(id);
    };
  }, []);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [state?.messages.length]);

  async function sendMessage(text: string) {
    const trimmed = text.trim();
    if (!trimmed || busy) return;
    setBusy(true);
    setErrorMsg(null);
    setStatusMsg(null);
    try {
      const res = await api.chat(trimmed);
      setState(res.state);
      setSelectedTrace(res.trace);
      setInput('');
      setTeachReply('');
      if (voiceEnabled) {
        void karinVoice.speak(res.trace.reply);
      }
    } catch (e) {
      setErrorMsg((e as Error).message);
    } finally {
      setBusy(false);
    }
  }

  async function handleTeach(impasseInput: string) {
    if (!teachReply.trim()) return;
    setBusy(true);
    try {
      const res = await api.teach(impasseInput, teachReply.trim(), teachPromote);
      setState(res.state);
      setStatusMsg(
        `Taught "${impasseInput}" (${teachPromote ? 'active' : 'candidate'}). Send it again to test.`
      );
      setTeachReply('');
    } catch (e) {
      setErrorMsg((e as Error).message);
    } finally {
      setBusy(false);
    }
  }

  async function handleAddFact(e: React.FormEvent) {
    e.preventDefault();
    if (!factRel.trim()) return;
    const args: Record<string, string> = {};
    for (const part of factArgsText.split(',')) {
      const idx = part.indexOf('=');
      if (idx > 0) {
        args[part.slice(0, idx).trim()] = part.slice(idx + 1).trim();
      }
    }
    const res = await api.addFact(factRel.trim(), args, factScope);
    setState(res.state);
    setStatusMsg(`Added ${factScope} fact: ${factRel.trim()}`);
  }

  async function runGeneralize(autoPromote: boolean) {
    const res = await api.generalize(autoPromote);
    setState(res.state);
    setStatusMsg(res.summary);
    setTab('knowledge');
    setKnowKind('patterns');
  }

  async function runChunk(autoPromote: boolean) {
    const res = await api.chunk(autoPromote);
    setState(res.state);
    setStatusMsg(res.summary);
    setTab('knowledge');
    setKnowKind('patterns');
  }

  async function runSelfTests() {
    const res = await api.selftest();
    setTests(res);
    setTab('tests');
  }

  async function handleNewChat() {
    stopVoiceDialog();
    const res = await api.newChat();
    setState(res.state);
    setSelectedTrace(null);
    setInput('');
    setTeachReply('');
    setStatusMsg('Started a new chat (long-term memory kept).');
  }

  async function handleReset() {
    stopVoiceDialog();
    const res = await api.reset();
    setState(res.state);
    setSelectedTrace(null);
    setStatusMsg('Reset to pretrained seed state.');
  }

  const lastAgentMsg = state
    ? [...state.messages].reverse().find((m) => m.role === 'agent')
    : undefined;
  const activeImpasse = lastAgentMsg?.trace?.impasse ? lastAgentMsg.trace : null;
  const candidateCount = state?.patterns.filter((p) => p.status === 'candidate').length ?? 0;

  const patternNames = new Map((state?.patterns ?? []).map((p) => [p.id, p.name]));
  const ruleName = (id: string) => patternNames.get(id);

  const visibleFacts = (state ? [...state.facts] : []).filter((f) => {
    if (!knowFilter.trim()) return true;
    const q = knowFilter.toLowerCase();
    return (
      f.rel.toLowerCase().includes(q) ||
      JSON.stringify(f.args).toLowerCase().includes(q) ||
      f.scope.includes(q) ||
      f.provenance.toLowerCase().includes(q)
    );
  });

  const visiblePatterns = (state ? [...state.patterns] : [])
    .sort((a, b) => (a.status === 'candidate' ? -1 : b.status === 'candidate' ? 1 : 0))
    .filter((p) => {
      if (!knowFilter.trim()) return true;
      const q = knowFilter.toLowerCase();
      return (
        p.name.toLowerCase().includes(q) ||
        p.kind.includes(q) ||
        p.status.includes(q) ||
        p.provenance.toLowerCase().includes(q) ||
        p.when.some((c) => fmtCond(c).toLowerCase().includes(q)) ||
        p.then.some((a) => fmtAction(a).toLowerCase().includes(q))
      );
    });

  const knowledgeItems: KItem[] = [
    ...(knowKind === 'facts' ? [] : visiblePatterns.map((pattern) => ({ kind: 'pattern' as const, pattern }))),
    ...(knowKind === 'patterns' ? [] : visibleFacts.map((fact) => ({ kind: 'fact' as const, fact }))),
  ];
  const countKnowledge = (list: KItem[]) => {
    const rules = list.filter((k) => k.kind === 'pattern').length;
    const facts = list.length - rules;
    const parts = [];
    if (rules > 0 || facts === 0) parts.push(`${rules} ${rules === 1 ? 'rule' : 'rules'}`);
    if (facts > 0) parts.push(`${facts} ${facts === 1 ? 'fact' : 'facts'}`);
    return parts.join(' · ');
  };

  return (
    <div className="app">
      <header className="topbar">
        <div className="brand">
          <span className="brand-dot" />
          <h1>Karin</h1>
          {state && (
            <span className="brand-stats">
              {state.facts.length} facts · {state.patterns.length} patterns · turn {state.turn}
            </span>
          )}
        </div>
        <div className="topbar-actions">
          <button
            className={`btn btn-sm ${voiceEnabled ? 'btn-primary' : 'btn-ghost'}`}
            onClick={async () => {
              if (voiceEnabled) {
                karinVoice.stop();
                setVoiceEnabled(false);
              } else {
                setVoiceEnabled(true);
                try {
                  await karinVoice.ensureLoaded();
                } catch {
                  setVoiceEnabled(false);
                }
              }
            }}
            title="Speak Karin's replies using tiny-voca (F2 female voice)"
          >
            {voiceStatus === 'loading'
              ? 'Loading Voice…'
              : voiceStatus === 'speaking'
                ? 'Speaking… (Stop)'
                : voiceEnabled
                  ? 'Voice: On (F2)'
                  : 'Voice: Off'}
          </button>
          <button className="btn btn-primary btn-sm" onClick={handleNewChat}>
            New Chat
          </button>
          <button
            className="btn btn-ghost btn-sm"
            onClick={() => setTheme((t) => (t === 'light' ? 'dark' : 'light'))}
          >
            {theme === 'light' ? 'Dark' : 'Light'}
          </button>
          <button className="btn btn-ghost btn-sm" onClick={runSelfTests}>
            Self-Tests
          </button>
          <button className="btn btn-ghost btn-sm" onClick={handleReset}>
            Reset
          </button>
        </div>
      </header>

      {errorMsg && (
        <div className="toast err">
          <span>{errorMsg}</span>
          <button className="meta-link" onClick={() => setErrorMsg(null)}>
            dismiss
          </button>
        </div>
      )}
      {statusMsg && (
        <div className="toast">
          <span>{statusMsg}</span>
          <button className="meta-link" onClick={() => setStatusMsg(null)}>
            dismiss
          </button>
        </div>
      )}

      <div className={`main-grid ${wideInspector ? 'inspector-wide' : ''}`}>
        <section className="chat-pane">
          <div className="dialog-mode-bar">
            <div className="dialog-mode-tabs">
              <button
                className={`mode-tab ${dialogMode === 'text' ? 'active' : ''}`}
                onClick={() => {
                  stopVoiceDialog();
                  setDialogMode('text');
                }}
              >
                Text Chat
              </button>
              <button
                className={`mode-tab ${dialogMode === 'voice' ? 'active' : ''}`}
                onClick={() => setDialogMode('voice')}
              >
                Voice Dialog
              </button>
            </div>
            {dialogMode === 'voice' && (
              <label className="muted row" style={{ gap: 6, fontSize: 12 }}>
                <input
                  type="checkbox"
                  checked={handsFree}
                  onChange={(e) => setHandsFree(e.target.checked)}
                />
                Hands-free loop
              </label>
            )}
          </div>
          <div className="messages">
            {!state || state.messages.length === 0 ? (
              <div className="empty-state">
                <h2>Karin</h2>
                <p>
                  A purely symbolic assistant built on facts, composable patterns, and inspectable
                  reasoning.
                </p>
                <div className="suggestion-grid">
                  {SUGGESTIONS.map((s) => (
                    <button key={s} className="chip" onClick={() => sendMessage(s)}>
                      {s}
                    </button>
                  ))}
                </div>
              </div>
            ) : (
              state.messages.map((m) => (
                <div
                  key={m.id}
                  className={`msg-row ${m.role} ${m.trace?.impasse ? 'impasse' : ''} ${
                    m.trace && selectedTrace?.turn === m.trace.turn && selectedTrace.input === m.trace.input ? "selected" : ""
                  }`}
                >
                  <div className="msg-bubble">{m.text}</div>
                  {m.role === 'agent' && m.trace && understoodNote(m.trace) && (
                    <div className="msg-understood">{understoodNote(m.trace)}</div>
                  )}
                  {m.role === 'agent' && m.trace && (
                    <div className="msg-meta">
                      <span>
                        {m.trace.impasse
                          ? 'Impasse'
                          : `${m.trace.steps.length} ${m.trace.steps.length === 1 ? 'step' : 'steps'}`}
                      </span>
                      <button
                        className="meta-link"
                        onClick={() => {
                          setSelectedTrace(m.trace!);
                          setTab('trace');
                        }}
                      >
                        Trace
                      </button>
                      <button
                        className="meta-link"
                        onClick={() => void karinVoice.speak(m.text)}
                      >
                        Speak
                      </button>
                      <span className="vote-group">
                        <button
                          className={`vote-btn ${m.feedback === 'up' ? 'active' : ''}`}
                          onClick={async () => {
                            const r = await api.feedback(m.turn, true);
                            setState(r.state);
                          }}
                        >
                          +1
                        </button>
                        <button
                          className={`vote-btn ${m.feedback === 'down' ? 'active' : ''}`}
                          onClick={async () => {
                            const r = await api.feedback(m.turn, false);
                            setState(r.state);
                          }}
                        >
                          -1
                        </button>
                      </span>
                    </div>
                  )}
                </div>
              ))
            )}
            <div ref={messagesEndRef} />
          </div>

          {activeImpasse && (
            <div className="teach-card">
              <div className="teach-title">
                No pattern matched &ldquo;{activeImpasse.input}&rdquo; - teach Karin
              </div>
              <div className="row">
                <input
                  type="text"
                  className="field"
                  style={{ flex: 1 }}
                  placeholder="Type the reply Karin should give..."
                  value={teachReply}
                  onChange={(e) => setTeachReply(e.target.value)}
                />
                <label className="muted row" style={{ gap: 5 }}>
                  <input
                    type="checkbox"
                    checked={teachPromote}
                    onChange={(e) => setTeachPromote(e.target.checked)}
                  />
                  promote
                </label>
                <button
                  className="btn btn-primary btn-sm"
                  onClick={() => handleTeach(activeImpasse.input)}
                  disabled={busy}
                >
                  Teach
                </button>
              </div>
            </div>
          )}

          <div className="composer-wrap">
            {dialogMode === 'text' ? (
              <form
                className="composer"
                onSubmit={(e) => {
                  e.preventDefault();
                  sendMessage(input);
                }}
              >
                <input
                  type="text"
                  placeholder="Message Karin..."
                  value={input}
                  onChange={(e) => setInput(e.target.value)}
                />
                <button type="submit" className="btn btn-primary" disabled={busy || !input.trim()}>
                  Send
                </button>
              </form>
            ) : (
              <div className={`voice-composer phase-${voiceDialogPhase}`}>
                <div className="voice-status-row">
                  <span className={`voice-orb ${voiceDialogPhase}`} />
                  <div className="voice-text">
                    <div className="voice-phase-label">
                      {!speechSupported
                        ? 'Speech recognition unavailable in this browser'
                        : voiceDialogPhase === 'idle'
                          ? 'Ready - click Start Talking and speak to Karin'
                          : voiceDialogPhase === 'loading'
                            ? 'Loading Karin voice (tiny-voca F2)…'
                            : voiceDialogPhase === 'listening'
                              ? 'Listening… speak now'
                              : voiceDialogPhase === 'thinking'
                                ? 'Karin is thinking…'
                                : 'Karin is speaking…'}
                    </div>
                    {liveTranscript && (
                      <div className="voice-transcript">&ldquo;{liveTranscript}&rdquo;</div>
                    )}
                  </div>
                </div>
                <div className="voice-actions">
                  {voiceDialogPhase === 'idle' ? (
                    <button
                      className="btn btn-primary"
                      type="button"
                      disabled={!speechSupported}
                      onClick={() => void startVoiceDialog()}
                    >
                      Start Talking
                    </button>
                  ) : (
                    <>
                      {voiceDialogPhase === 'speaking' && (
                        <button
                          className="btn btn-ghost"
                          type="button"
                          onClick={() => karinVoice.stop()}
                        >
                          Interrupt
                        </button>
                      )}
                      <button
                        className="btn btn-primary"
                        type="button"
                        onClick={stopVoiceDialog}
                      >
                        Stop
                      </button>
                    </>
                  )}
                </div>
              </div>
            )}
          </div>
        </section>

        <section className="inspector-pane">
          <div className="tabs">
            <button
              className={`tab ${tab === 'trace' ? 'active' : ''}`}
              onClick={() => setTab('trace')}
            >
              Trace
            </button>
            <button
              className={`tab ${tab === 'knowledge' ? 'active' : ''}`}
              onClick={() => setTab('knowledge')}
            >
              Knowledge{' '}
              <span className="tab-count">
                {(state?.facts.length ?? 0) + (state?.patterns.length ?? 0)}
              </span>
              {candidateCount > 0 && <span className="badge warn">{candidateCount}</span>}
            </button>
            <button
              className={`tab ${tab === 'tools' ? 'active' : ''}`}
              onClick={() => setTab('tools')}
            >
              Tools
            </button>
            <button
              className={`tab ${tab === 'tests' ? 'active' : ''}`}
              onClick={() => setTab('tests')}
            >
              Tests
            </button>
            <button
              className="tab tab-right"
              title={wideInspector ? 'Give the chat more room' : 'Give the inspector more room'}
              onClick={() => setWideInspector((w) => !w)}
            >
              {wideInspector ? 'Collapse' : 'Expand'}
            </button>
          </div>

          <div className={`inspector-body ${tab === 'knowledge' ? 'flush' : ''}`}>
            {tab === 'trace' && (
              <>
                {!selectedTrace ? (
                  <div className="muted">
                    Send a message to inspect the Perceive → Match → Select → Fire trace.
                  </div>
                ) : (
                  <TracePanel trace={selectedTrace} patterns={state?.patterns ?? []} />
                )}
              </>
            )}

            {tab === 'knowledge' && state && (
              <>
                <div className="ex-toolbar">
                  <div className="vx-switch" role="tablist" aria-label="View">
                    <button className={knowView === 'tree' ? 'on' : ''} onClick={() => setKnowView('tree')}>
                      Tree
                    </button>
                    <button className={knowView === 'graph' ? 'on' : ''} onClick={() => setKnowView('graph')}>
                      Graph
                    </button>
                  </div>
                  {knowView === 'tree' && (
                  <div className="vx-switch" role="tablist" aria-label="Show">
                    {(
                      [
                        ['all', 'All'],
                        ['patterns', `Rules ${visiblePatterns.length}`],
                        ['facts', `Facts ${visibleFacts.length}`],
                      ] as [KnowKind, string][]
                    ).map(([k, label]) => (
                      <button key={k} className={knowKind === k ? 'on' : ''} onClick={() => setKnowKind(k)}>
                        {label}
                      </button>
                    ))}
                  </div>
                  )}
                  <input
                    type="text"
                    className="field"
                    placeholder="Filter by name, value, condition, action or source…"
                    value={knowFilter}
                    onChange={(e) => setKnowFilter(e.target.value)}
                  />
                  <button
                    className={`btn btn-sm ${showAddFact ? 'btn-ghost' : 'btn-primary'}`}
                    onClick={() => setShowAddFact((v) => !v)}
                  >
                    {showAddFact ? 'Cancel' : 'Add fact'}
                  </button>
                </div>
                {showAddFact && (
                  <form className="ex-addform" onSubmit={handleAddFact}>
                    <input
                      type="text"
                      className="field"
                      value={factRel}
                      onChange={(e) => setFactRel(e.target.value)}
                      placeholder="relation"
                      style={{ width: 130 }}
                    />
                    <input
                      type="text"
                      className="field"
                      value={factArgsText}
                      onChange={(e) => setFactArgsText(e.target.value)}
                      placeholder="k=v, k2=v2"
                      style={{ flex: 1 }}
                    />
                    <select
                      className="field"
                      value={factScope}
                      onChange={(e) => setFactScope(e.target.value as Scope)}
                    >
                      <option value="long_term">long-term</option>
                      <option value="chat">chat</option>
                      <option value="working">working</option>
                    </select>
                    <button type="submit" className="btn btn-primary btn-sm">
                      Add
                    </button>
                  </form>
                )}
                {knowView === 'graph' ? (
                  <GraphView
                    patterns={state.patterns}
                    facts={state.facts}
                    query={knowFilter}
                    theme={theme}
                  />
                ) : (
                <VaultExplorer<KItem>
                  refreshKey={vaultVersion}
                  items={knowledgeItems}
                  pathOf={(k) =>
                    vaultPathOf(k.kind === 'fact' ? k.fact.provenance : k.pattern.provenance, ruleName)
                  }
                  itemKey={(k) => (k.kind === 'fact' ? k.fact.id : k.pattern.id)}
                  itemLabel={(k) =>
                    k.kind === 'fact' ? (
                      <span
                        className="mono vx-fact-label"
                        title={`${k.fact.rel}(${Object.entries(k.fact.args)
                          .map(([a, v]) => `${a}="${v}"`)
                          .join(', ')})`}
                      >
                        {Object.values(k.fact.args).join(' · ') || k.fact.rel}
                      </span>
                    ) : (
                      <span className="vx-rule-label">
                        {k.pattern.name}
                        {k.pattern.status === 'candidate' && <span className="vx-flag">candidate</span>}
                      </span>
                    )
                  }
                  groupOf={(k) => (k.kind === 'fact' ? k.fact.rel : undefined)}
                  locatorOf={(k) =>
                    k.kind === 'fact'
                      ? { kind: 'fact', rel: k.fact.rel, values: Object.values(k.fact.args) }
                      : { kind: 'rule', name: k.pattern.name }
                  }
                  dslOf={(k) => (k.kind === 'fact' ? factDsl(k.fact) : patternDsl(k.pattern))}
                  filtering={knowFilter.trim() !== ''}
                  noun="items"
                  countLabel={countKnowledge}
                  renderItem={(k) =>
                    k.kind === 'fact' ? (
                      <FactRow
                        fact={k.fact}
                        onDelete={async () => {
                          const r = await api.deleteFact(k.fact.id);
                          setState(r.state);
                        }}
                      />
                    ) : (
                      <PatternRow
                        pattern={k.pattern}
                        onPromote={async () => {
                          const r = await api.promotePattern(k.pattern.id);
                          setState(r.state);
                        }}
                        onDelete={async () => {
                          const r = await api.deletePattern(k.pattern.id);
                          setState(r.state);
                        }}
                      />
                    )
                  }
                />
                )}
              </>
            )}

            {tab === 'tools' && state && (
              <>
                <div className="card">
                  <div className="section-label">Learning Meta-Tools</div>
                  <div className="row" style={{ marginTop: 8 }}>
                    <button className="btn btn-primary btn-sm" onClick={() => runGeneralize(false)}>
                      Anti-Unify Episodes
                    </button>
                    <button className="btn btn-ghost btn-sm" onClick={() => runGeneralize(true)}>
                      Anti-Unify &amp; Promote
                    </button>
                    <button className="btn btn-primary btn-sm" onClick={() => runChunk(false)}>
                      Chunk Last Chain
                    </button>
                    <button className="btn btn-ghost btn-sm" onClick={() => runChunk(true)}>
                      Chunk &amp; Promote
                    </button>
                  </div>
                </div>

                <div className="section-label">Registered Tools</div>
                {tools.map((t) => (
                  <div key={t.name} className="card">
                    <div className="card-head">
                      <span className="mono card-title">{t.name}</span>
                      <span className={`badge ${t.meta ? 'brand' : ''}`}>
                        {t.meta ? 'meta-tool' : 'world tool'}
                      </span>
                    </div>
                    <div className="muted">{t.description}</div>
                  </div>
                ))}

                <div className="section-label">Episodes ({state.episodes.length})</div>
                <div className="card mono">
                  {state.episodes.map((ep) => (
                    <div key={ep.id} style={{ marginBottom: 4 }}>
                      <span className={`badge ${ep.generalized ? 'ok' : ''}`}>
                        {ep.generalized ? 'done' : 'raw'}
                      </span>{' '}
                      &ldquo;{ep.input}&rdquo; → &ldquo;{ep.reply}&rdquo;
                    </div>
                  ))}
                </div>

                <div className="section-label">Multi-Step Chains ({state.chains.length})</div>
                <div className="card mono">
                  {state.chains.length === 0 ? (
                    <span className="muted">
                      Run a multi-step prompt like &ldquo;calculate 6 * 7&rdquo; to record a chain.
                    </span>
                  ) : (
                    state.chains.map((c) => (
                      <div key={c.id} style={{ marginBottom: 4 }}>
                        <span className={`badge ${c.chunked ? 'ok' : 'warn'}`}>
                          {c.chunked ? 'chunked' : 'ready'}
                        </span>{' '}
                        turn {c.turn}: &ldquo;{c.input}&rdquo; ({c.patternIds.join(' → ')})
                      </div>
                    ))
                  )}
                </div>
              </>
            )}

            {tab === 'tests' && (
              <>
                <div className="row">
                  <button className="btn btn-primary btn-sm" onClick={runSelfTests}>
                    Run Rust Self-Tests
                  </button>
                </div>
                {tests.length === 0 ? (
                  <div className="muted">Click &ldquo;Run Rust Self-Tests&rdquo; to verify.</div>
                ) : (
                  tests.map((t) => (
                    <div key={t.name} className="card">
                      <div className="card-head">
                        <span className="card-title">{t.name}</span>
                        <span className={`badge ${t.passed ? 'ok' : 'err'}`}>
                          {t.passed ? 'PASS' : 'FAIL'}
                        </span>
                      </div>
                      <div className="mono muted">{t.detail}</div>
                    </div>
                  ))
                )}
              </>
            )}
          </div>
        </section>
      </div>
    </div>
  );
}
