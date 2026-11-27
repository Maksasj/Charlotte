import { useEffect, useMemo, useRef, useState } from 'react';
import type { Fact, Pattern } from './types';

// ---------------------------------------------------------------------------
// Graph model: rules, relations (fact families) and tools.
//   rule --reads--> relation      (a condition: rel(...) or !rel(...))
//   rule --writes--> relation     (+rel, +chat rel, ++rel, -rel)
//   rule --calls--> tool          (@tool(...))
// ---------------------------------------------------------------------------

type NodeKind = 'rule' | 'rel' | 'tool' | 'person' | 'karin' | 'thing' | 'category';
type EdgeKind = 'read' | 'write' | 'call' | 'fact';

/** Which graph is drawn: how rules connect, or what Karin knows. */
export type GraphMode = 'rules' | 'facts';

interface GNode {
  id: string;
  kind: NodeKind;
  label: string;
  group: string;
  facts: number;
  degree: number;
  r: number;
  x: number;
  y: number;
  vx: number;
  vy: number;
  pattern?: Pattern;
  /** Facts mode: every name this entity answers to. */
  names?: string[];
}

interface GEdge {
  s: GNode;
  t: GNode;
  kind: EdgeKind;
  /** Facts mode: the relation, e.g. "friend", "likes (drink)", "is a". */
  label?: string;
  fact?: Fact;
}

/** Which vault module a provenance belongs to ("core", "language", "skills", ...), or "runtime". */
function packOf(provenance: string): string {
  if (provenance.startsWith('vault:')) {
    return provenance.slice('vault:'.length).split('/')[0] || 'core';
  }
  return 'runtime';
}

function buildGraph(patterns: Pattern[], facts: Fact[]) {
  const nodes = new Map<string, GNode>();
  const edges: GEdge[] = [];
  const seen = new Set<string>();
  const factCount = new Map<string, number>();
  for (const f of facts) factCount.set(f.rel, (factCount.get(f.rel) ?? 0) + 1);

  const node = (id: string, kind: NodeKind, label: string, group: string): GNode => {
    let n = nodes.get(id);
    if (!n) {
      // Deterministic start (hash of the id), so the same knowledge always lays out the same way.
      let h = 2166136261;
      for (let i = 0; i < id.length; i++) h = Math.imul(h ^ id.charCodeAt(i), 16777619);
      const a = ((h >>> 0) % 3600) / 3600 * Math.PI * 2;
      const d = 80 + (((h >>> 12) % 1000) / 1000) * 260;
      n = { id, kind, label, group, facts: 0, degree: 0, r: 4, x: Math.cos(a) * d, y: Math.sin(a) * d, vx: 0, vy: 0 };
      nodes.set(id, n);
    }
    return n;
  };
  const rel = (name: string) => node(`rel:${name}`, 'rel', name, 'relation');
  const edge = (s: GNode, t: GNode, kind: EdgeKind) => {
    const key = `${s.id}>${t.id}>${kind}`;
    if (seen.has(key)) return;
    seen.add(key);
    edges.push({ s, t, kind });
  };

  for (const p of patterns) {
    const rn = node(`rule:${p.id}`, 'rule', p.name, packOf(p.provenance));
    rn.pattern = p;
    for (const c of p.when) {
      if (c.kind === 'fact' || c.kind === 'not_fact') edge(rn, rel(c.rel), 'read');
    }
    for (const a of p.then) {
      if (a.kind === 'assert' || a.kind === 'retract') edge(rn, rel(a.rel), 'write');
      if (a.kind === 'tool') edge(rn, node(`tool:${a.name}`, 'tool', `@${a.name}`, 'tool'), 'call');
    }
  }
  for (const [name, count] of factCount) rel(name).facts = count;
  for (const e of edges) {
    e.s.degree++;
    e.t.degree++;
  }
  for (const n of nodes.values()) {
    n.r = n.kind === 'rule' ? 4.2 : n.kind === 'tool' ? 5 : 5 + Math.sqrt(n.facts) * 1.3;
  }
  return { nodes: [...nodes.values()], edges };
}

/** A node's first position: hashed from its id, so the same knowledge lays out the same way. */
function seedNode(id: string, kind: NodeKind, label: string, group: string): GNode {
  let h = 2166136261;
  for (let i = 0; i < id.length; i++) h = Math.imul(h ^ id.charCodeAt(i), 16777619);
  const a = (((h >>> 0) % 3600) / 3600) * Math.PI * 2;
  const d = 80 + (((h >>> 12) % 1000) / 1000) * 260;
  return { id, kind, label, group, facts: 0, degree: 0, r: 4, x: Math.cos(a) * d, y: Math.sin(a) * d, vx: 0, vy: 0 };
}

/**
 * What Karin knows: people and things as nodes, facts as labelled edges.
 *   rel(subj, pred, obj [, topic])  subj --pred--> obj
 *   is_a(child, parent)             child --is a--> parent (a category)
 *   prop(entity, attr, value)       entity --attr--> value
 * Entities (`entity`, `display_name`, `named`) give people their names, so
 * "user" shows as "Maxim" and "Max" points at the same node.
 */
function buildFactGraph(facts: Fact[]) {
  const nodes = new Map<string, GNode>();
  const edges: GEdge[] = [];
  const seen = new Set<string>();
  const kindOf = new Map<string, string>();
  const display = new Map<string, string>();
  const names = new Map<string, string[]>();
  const byName = new Map<string, string>();
  for (const f of facts) {
    const a = f.args;
    if (f.rel === 'entity' && a.id) kindOf.set(a.id, a.kind ?? 'thing');
    if (f.rel === 'display_name' && a.entity && a.value) display.set(a.entity, a.value);
    if (f.rel === 'named' && a.entity && a.value) {
      names.set(a.entity, [...(names.get(a.entity) ?? []), a.value]);
      byName.set(a.value.toLowerCase(), a.entity);
    }
  }
  // A value that is some entity's name means that entity.
  const resolve = (v: string) => (kindOf.has(v) ? v : byName.get(v.toLowerCase()) ?? v);
  const node = (raw: string, as: 'thing' | 'category' = 'thing'): GNode => {
    const id = resolve(raw);
    const key = `k:${id.toLowerCase()}`;
    let n = nodes.get(key);
    if (!n) {
      const ek = kindOf.get(id);
      const kind: NodeKind =
        id === 'karin' || ek === 'assistant' ? 'karin' : ek === 'person' ? 'person' : as;
      n = seedNode(key, kind, display.get(id) ?? raw, kind);
      n.names = names.get(id);
      nodes.set(key, n);
    } else if (as === 'category' && n.kind === 'thing') {
      n.kind = 'category';
      n.group = 'category';
    }
    return n;
  };
  const edge = (s: GNode, t: GNode, label: string, fact: Fact) => {
    const key = `${s.id}>${t.id}>${label}`;
    if (seen.has(key) || s === t) return;
    seen.add(key);
    edges.push({ s, t, kind: 'fact', label, fact });
  };
  const words = (x: string) => x.replace(/_/g, ' ');
  for (const f of facts) {
    const a = f.args;
    if (f.rel === 'rel' && a.subj && a.obj && a.pred) {
      edge(node(a.subj), node(a.obj), a.topic ? `${words(a.pred)} (${a.topic})` : words(a.pred), f);
    } else if (f.rel === 'is_a' && a.child && a.parent) {
      edge(node(a.child), node(a.parent, 'category'), 'is a', f);
    } else if (f.rel === 'prop' && a.entity && a.attr && a.value) {
      edge(node(a.entity), node(a.value), words(a.attr), f);
    }
  }
  // People Karin knows but has no facts about yet still appear.
  for (const [id, k] of kindOf) if (k === 'person' || k === 'assistant') node(id);
  for (const e of edges) {
    e.s.degree++;
    e.t.degree++;
  }
  for (const n of nodes.values()) {
    n.facts = n.degree;
    n.r =
      n.kind === 'person' || n.kind === 'karin'
        ? 9 + Math.sqrt(n.degree) * 1.6
        : n.kind === 'category'
          ? 6 + Math.sqrt(n.degree) * 1.5
          : 4.5 + Math.sqrt(n.degree);
  }
  return { nodes: [...nodes.values()], edges };
}

// ---------------------------------------------------------------------------
// Colours
// ---------------------------------------------------------------------------

const PACK_COLORS = ['#7c3aed', '#2563eb', '#0d9488', '#db2777', '#65a30d', '#0891b2', '#c026d3'];
const PACK_COLORS_DARK = ['#a78bfa', '#60a5fa', '#2dd4bf', '#f472b6', '#a3e635', '#22d3ee', '#e879f9'];

function readVars() {
  const cs = getComputedStyle(document.documentElement);
  const v = (name: string) => cs.getPropertyValue(name).trim();
  const dark = document.documentElement.getAttribute('data-theme') === 'dark';
  return {
    dark,
    bg: v('--surface'),
    text: v('--text'),
    textSecondary: v('--text-secondary'),
    muted: v('--muted'),
    border: v('--border'),
    borderStrong: v('--border-strong'),
    runtime: v('--zone-runtime') || '#d97706',
    brand: v('--brand'),
    ok: v('--ok'),
    core: dark ? '#a1a1aa' : '#52525b',
  };
}

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

export function GraphView({
  patterns,
  facts,
  query,
  theme,
  keyHandlerRef,
}: {
  patterns: Pattern[];
  facts: Fact[];
  query: string;
  theme: string;
  keyHandlerRef?: React.MutableRefObject<((e: React.KeyboardEvent) => void) | null>;
}) {
  const [mode, setMode] = useState<GraphMode>(() =>
    localStorage.getItem('karin.graphMode') === 'rules' ? 'rules' : 'facts',
  );
  useEffect(() => localStorage.setItem('karin.graphMode', mode), [mode]);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const wrapRef = useRef<HTMLDivElement>(null);
  // Keep node positions when the knowledge changes (e.g. after each chat turn).
  const positions = useRef(new Map<string, { x: number; y: number }>());
  const graph = useMemo(() => {
    const g = mode === 'facts' ? buildFactGraph(facts) : buildGraph(patterns, facts);
    let kept = 0;
    for (const n of g.nodes) {
      const old = positions.current.get(n.id);
      if (old) {
        n.x = old.x;
        n.y = old.y;
        kept++;
      }
    }
    (g as { kept?: number }).kept = kept;
    return g;
  }, [patterns, facts, mode]);
  const [selected, setSelected] = useState<GNode | null>(null);
  const [, force] = useState(0);

  // Mutable view state (kept out of React to animate at 60fps).
  const autoFitted = useRef(0);
  const [legendOpen, setLegendOpen] = useState(false);
  const view = useRef({ k: 1, tx: 0, ty: 0, w: 0, h: 0, alpha: 1, hover: null as GNode | null, dirty: true });
  const drag = useRef<{ node: GNode | null; panning: boolean; x: number; y: number; moved: boolean } | null>(null);
  const colors = useRef(readVars());
  const selectedRef = useRef<GNode | null>(null);
  selectedRef.current = selected;

  const packs = useMemo(() => {
    const names = [...new Set(graph.nodes.filter((n) => n.kind === 'rule').map((n) => n.group))];
    names.sort((a, b) => (a === 'core' ? -1 : b === 'core' ? 1 : a === 'runtime' ? 1 : b === 'runtime' ? -1 : a.localeCompare(b)));
    return names;
  }, [graph]);

  const colorOf = (n: GNode) => {
    const c = colors.current;
    if (n.kind === 'rel') return c.textSecondary;
    if (n.kind === 'tool') return c.muted;
    if (n.kind === 'person') return c.brand;
    if (n.kind === 'karin') return c.ok;
    if (n.kind === 'category') return c.textSecondary;
    if (n.kind === 'thing') return c.muted;
    if (n.group === 'core') return c.core;
    if (n.group === 'runtime') return c.runtime;
    const i = packs.filter((p) => p !== 'core' && p !== 'runtime').indexOf(n.group);
    const pal = c.dark ? PACK_COLORS_DARK : PACK_COLORS;
    return pal[i % pal.length];
  };

  const neighbours = useMemo(() => {
    const m = new Map<GNode, Set<GNode>>();
    for (const e of graph.edges) {
      if (!m.has(e.s)) m.set(e.s, new Set());
      if (!m.has(e.t)) m.set(e.t, new Set());
      m.get(e.s)!.add(e.t);
      m.get(e.t)!.add(e.s);
    }
    return m;
  }, [graph]);

  // Theme change → re-read colours.
  useEffect(() => {
    // The app sets data-theme in its own effect, which runs after ours: read on the next frame.
    const id = requestAnimationFrame(() => {
      colors.current = readVars();
      view.current.dirty = true;
      force((x) => x + 1);
    });
    return () => cancelAnimationFrame(id);
  }, [theme]);

  // Restart the layout when the graph changes (gently if most nodes kept their place).
  useEffect(() => {
    const kept = (graph as { kept?: number }).kept ?? 0;
    view.current.alpha = kept > graph.nodes.length * 0.8 ? 0.15 : 1;
    view.current.dirty = true;
    return () => {
      for (const n of graph.nodes) positions.current.set(n.id, { x: n.x, y: n.y });
    };
  }, [graph]);

  useEffect(() => {
    view.current.dirty = true;
  }, [query, selected]);

  // Size the canvas to its container.
  useEffect(() => {
    const wrap = wrapRef.current!;
    const ro = new ResizeObserver(() => {
      const v = view.current;
      const first = v.w === 0;
      v.w = wrap.clientWidth;
      v.h = wrap.clientHeight;
      if (first) {
        v.tx = v.w / 2;
        v.ty = v.h / 2;
        v.k = Math.min(v.w, v.h) / 1600;
      }
      const c = canvasRef.current!;
      const dpr = window.devicePixelRatio || 1;
      c.width = v.w * dpr;
      c.height = v.h * dpr;
      c.style.width = `${v.w}px`;
      c.style.height = `${v.h}px`;
      v.dirty = true;
    });
    ro.observe(wrap);
    return () => ro.disconnect();
  }, []);

  // Physics + drawing loop.
  useEffect(() => {
    let raf = 0;
    const { nodes, edges } = graph;
    const q = () => query.trim().toLowerCase();

    const tick = () => {
      const v = view.current;
      const a = v.alpha;
      for (let i = 0; i < nodes.length; i++) {
        const n = nodes[i];
        for (let j = i + 1; j < nodes.length; j++) {
          const m = nodes[j];
          let dx = n.x - m.x;
          let dy = n.y - m.y;
          let d2 = dx * dx + dy * dy;
          if (d2 < 1) {
            dx = Math.random() - 0.5;
            dy = Math.random() - 0.5;
            d2 = 1;
          }
          if (d2 > 250000) continue;
          const f = ((n.r + m.r) * 220 * a) / d2;
          n.vx += dx * f;
          n.vy += dy * f;
          m.vx -= dx * f;
          m.vy -= dy * f;
        }
      }
      for (const e of edges) {
        const dx = e.t.x - e.s.x;
        const dy = e.t.y - e.s.y;
        const d = Math.sqrt(dx * dx + dy * dy) || 1;
        const rest = 55 + e.t.r * 3;
        const f = ((d - rest) / d) * 0.025 * a;
        e.s.vx += dx * f;
        e.s.vy += dy * f;
        e.t.vx -= dx * f;
        e.t.vy -= dy * f;
      }
      const held = drag.current?.node;
      for (const n of nodes) {
        const pull = n.degree === 0 ? 0.012 : 0.0016;
        n.vx -= n.x * pull * a;
        n.vy -= n.y * pull * a;
        n.vx *= 0.6;
        n.vy *= 0.6;
        if (n === held) continue;
        n.x += n.vx;
        n.y += n.vy;
      }
      v.alpha = Math.max(0, a * 0.994);
      if (autoFitted.current === 0 && v.alpha < 0.8) {
        autoFitted.current = 1;
        fit();
      } else if (autoFitted.current === 1 && v.alpha < 0.25) {
        autoFitted.current = 2;
        fit();
      }
    };

    const draw = () => {
      const v = view.current;
      const c = colors.current;
      const ctx = canvasRef.current!.getContext('2d')!;
      const dpr = window.devicePixelRatio || 1;
      ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
      ctx.clearRect(0, 0, v.w, v.h);
      ctx.translate(v.tx, v.ty);
      ctx.scale(v.k, v.k);

      const focus = v.hover ?? selectedRef.current;
      const near = focus ? neighbours.get(focus) ?? new Set<GNode>() : null;
      const query = q();
      const matches = (n: GNode) => query !== '' && n.label.toLowerCase().includes(query);
      const lit = (n: GNode) => (near ? n === focus || near.has(n) : query ? matches(n) : true);

      // edges
      ctx.lineWidth = 1 / v.k;
      for (const e of edges) {
        const on = near ? e.s === focus || e.t === focus : !query;
        ctx.globalAlpha = on
          ? near
            ? 0.9
            : e.kind === 'fact'
              ? 0.32
              : e.kind === 'write'
                ? 0.28
                : 0.12
          : 0.05;
        ctx.strokeStyle = e.kind === 'write' ? colorOf(e.s) : c.borderStrong;
        ctx.beginPath();
        ctx.moveTo(e.s.x, e.s.y);
        ctx.lineTo(e.t.x, e.t.y);
        ctx.stroke();
      }

      // fact labels on edges: around the focused node, or everywhere when zoomed in
      if (edges.length && edges[0].kind === 'fact') {
        ctx.font = `${10 / v.k}px ui-sans-serif, system-ui, sans-serif`;
        ctx.textAlign = 'center';
        ctx.textBaseline = 'middle';
        for (const e of edges) {
          const around = near && (e.s === focus || e.t === focus);
          if (!around && (near || query || v.k < 1.6)) continue;
          const mx = (e.s.x + e.t.x) / 2;
          const my = (e.s.y + e.t.y) / 2;
          const label = e.label ?? '';
          const w = ctx.measureText(label).width + 6 / v.k;
          ctx.globalAlpha = around ? 1 : 0.8;
          ctx.fillStyle = c.bg;
          ctx.fillRect(mx - w / 2, my - 7 / v.k, w, 14 / v.k);
          ctx.fillStyle = around ? c.text : c.muted;
          ctx.fillText(label, mx, my);
        }
      }

      // nodes (never smaller than ~70% of their size on screen)
      const size = (n: GNode) => Math.max(n.r, (n.r * 0.7) / v.k);
      for (const n of nodes) {
        ctx.globalAlpha = lit(n) ? 1 : 0.15;
        const col = colorOf(n);
        const r = size(n);
        if (n.kind === 'rel' || n.kind === 'category') {
          ctx.fillStyle = c.bg;
          ctx.strokeStyle = col;
          ctx.lineWidth = 1.6 / Math.max(v.k, 0.6);
          ctx.beginPath();
          ctx.arc(n.x, n.y, r, 0, Math.PI * 2);
          ctx.fill();
          ctx.stroke();
        } else if (n.kind === 'tool') {
          ctx.fillStyle = col;
          ctx.fillRect(n.x - r * 0.8, n.y - r * 0.8, r * 1.6, r * 1.6);
        } else {
          ctx.fillStyle = col;
          ctx.beginPath();
          ctx.arc(n.x, n.y, r, 0, Math.PI * 2);
          ctx.fill();
        }
        if (n === selectedRef.current) {
          ctx.globalAlpha = 1;
          ctx.strokeStyle = c.text;
          ctx.lineWidth = 1.5 / v.k;
          ctx.beginPath();
          ctx.arc(n.x, n.y, r + 3 / v.k, 0, Math.PI * 2);
          ctx.stroke();
        }
      }

      // labels (screen-sized text)
      ctx.font = `${11 / v.k}px ui-sans-serif, system-ui, sans-serif`;
      ctx.textAlign = 'center';
      ctx.textBaseline = 'top';
      const shown: GNode[] = [];
      for (const n of nodes) {
        const show =
          (near && (n === focus || near.has(n))) ||
          (query && matches(n)) ||
          (!near &&
            !query &&
            (n.kind === 'person' ||
              n.kind === 'karin' ||
              (n.kind === 'category' && (n.degree >= 2 || v.k > 0.9)) ||
              (n.kind === 'thing' && (n.degree >= 3 || v.k > 1.3)) ||
              (n.kind === 'rel' && (n.facts >= 8 || n.degree >= 8 || v.k > 1.1)) ||
              v.k > 2.2 ||
              (n.kind === 'tool' && (n.degree >= 6 || v.k > 1.3))));
        if (show) shown.push(n);
      }
      // Greedy placement: most important labels first, skip any that would overlap.
      shown.sort((x, y) => (x === focus ? -1 : y === focus ? 1 : y.degree - x.degree));
      const placed: { x0: number; y0: number; x1: number; y1: number }[] = [];
      const lh = 13 / v.k;
      ctx.lineJoin = 'round';
      ctx.lineWidth = 3.5 / v.k;
      ctx.strokeStyle = c.bg;
      for (const n of shown) {
        const w = ctx.measureText(n.label).width;
        const y0 = n.y + size(n) + 3 / v.k;
        const box = { x0: n.x - w / 2 - 2 / v.k, y0, x1: n.x + w / 2 + 2 / v.k, y1: y0 + lh };
        if (
          n !== focus &&
          placed.some((b) => box.x0 < b.x1 && box.x1 > b.x0 && box.y0 < b.y1 && box.y1 > b.y0)
        )
          continue;
        placed.push(box);
        ctx.globalAlpha = lit(n) ? 1 : 0.3;
        ctx.fillStyle = n === focus ? c.text : c.textSecondary;
        ctx.strokeText(n.label, n.x, y0);
        ctx.fillText(n.label, n.x, y0);
      }
      ctx.globalAlpha = 1;
    };

    const loop = () => {
      const v = view.current;
      if (v.alpha > 0.01 || drag.current?.node) {
        for (let i = 0; i < 2; i++) tick();
        v.dirty = true;
      }
      if (v.dirty && v.w > 0) {
        draw();
        v.dirty = false;
      }
      raf = requestAnimationFrame(loop);
    };
    raf = requestAnimationFrame(loop);
    return () => cancelAnimationFrame(raf);
    // colorOf depends on packs, which depends on graph
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [graph, neighbours, query]);

  // ---- pointer interaction ----
  const toWorld = (sx: number, sy: number) => {
    const v = view.current;
    return { x: (sx - v.tx) / v.k, y: (sy - v.ty) / v.k };
  };
  const hit = (sx: number, sy: number) => {
    const p = toWorld(sx, sy);
    let best: GNode | null = null;
    let bestD = Infinity;
    for (const n of graph.nodes) {
      const d = Math.hypot(n.x - p.x, n.y - p.y);
      if (d < n.r + 5 / view.current.k && d < bestD) {
        best = n;
        bestD = d;
      }
    }
    return best;
  };
  const local = (e: React.PointerEvent | React.WheelEvent) => {
    const r = canvasRef.current!.getBoundingClientRect();
    return { x: e.clientX - r.left, y: e.clientY - r.top };
  };

  const onPointerDown = (e: React.PointerEvent) => {
    autoFitted.current = 2;
    const p = local(e);
    const n = hit(p.x, p.y);
    drag.current = { node: n, panning: !n, x: p.x, y: p.y, moved: false };
    (e.target as HTMLElement).setPointerCapture(e.pointerId);
    if (n) view.current.alpha = Math.max(view.current.alpha, 0.3);
  };
  const onPointerMove = (e: React.PointerEvent) => {
    const p = local(e);
    const v = view.current;
    const d = drag.current;
    if (d) {
      const dx = p.x - d.x;
      const dy = p.y - d.y;
      if (Math.abs(dx) + Math.abs(dy) > 2) d.moved = true;
      if (d.node) {
        const w = toWorld(p.x, p.y);
        d.node.x = w.x;
        d.node.y = w.y;
        d.node.vx = 0;
        d.node.vy = 0;
      } else {
        v.tx += dx;
        v.ty += dy;
      }
      d.x = p.x;
      d.y = p.y;
      v.dirty = true;
      return;
    }
    const n = hit(p.x, p.y);
    if (n !== v.hover) {
      v.hover = n;
      v.dirty = true;
      canvasRef.current!.style.cursor = n ? 'pointer' : 'grab';
    }
  };
  const onPointerUp = () => {
    const d = drag.current;
    drag.current = null;
    if (d && !d.moved) {
      setSelected(d.node);
      force((x) => x + 1);
    }
  };
  const onWheel = (e: React.WheelEvent) => {
    autoFitted.current = 2;
    const p = local(e);
    const v = view.current;
    const factor = Math.exp(-e.deltaY * 0.0015);
    const k = Math.min(6, Math.max(0.15, v.k * factor));
    v.tx = p.x - ((p.x - v.tx) * k) / v.k;
    v.ty = p.y - ((p.y - v.ty) * k) / v.k;
    v.k = k;
    v.dirty = true;
  };

  const fit = () => {
    const v = view.current;
    const xs = graph.nodes.map((n) => n.x);
    const ys = graph.nodes.map((n) => n.y);
    if (xs.length === 0) return;
    const minX = Math.min(...xs);
    const maxX = Math.max(...xs);
    const minY = Math.min(...ys);
    const maxY = Math.max(...ys);
    const k = Math.min(v.w / (maxX - minX + 240), v.h / (maxY - minY + 340), 4);
    v.k = k;
    v.tx = v.w / 2 - ((minX + maxX) / 2) * k;
    v.ty = v.h / 2 - ((minY + maxY) / 2) * k;
    v.dirty = true;
  };

  const zoomBy = (f: number) => {
    const v = view.current;
    const k = Math.min(6, Math.max(0.15, v.k * f));
    v.tx = v.w / 2 - ((v.w / 2 - v.tx) * k) / v.k;
    v.ty = v.h / 2 - ((v.h / 2 - v.ty) * k) / v.k;
    v.k = k;
    v.dirty = true;
  };

  // ---- selected node card ----
  const card = (() => {
    const n = selected;
    if (!n) return null;
    const outs = graph.edges.filter((e) => e.s === n);
    const ins = graph.edges.filter((e) => e.t === n);
    const list = (title: string, items: GNode[]) =>
      items.length === 0 ? null : (
        <div className="gv-list">
          <div className="gv-list-title">
            {title} <span className="vx-count">{items.length}</span>
          </div>
          {items.map((m) => (
            <button key={m.id} className="gv-link" onClick={() => setSelected(m)}>
              <span className="gv-dot" style={{ background: colorOf(m) }} />
              {m.label}
            </button>
          ))}
        </div>
      );
    if (mode === 'facts') {
      const sentence = (e: GEdge, from: GNode) =>
        e.s === from ? (
          <>
            <span className="gv-pred">{e.label}</span> {e.t.label}
          </>
        ) : (
          <>
            {e.s.label} <span className="gv-pred">{e.label}</span>
          </>
        );
      const row = (e: GEdge) => {
        const other = e.s === n ? e.t : e.s;
        return (
          <button
            key={`${e.s.id}>${e.t.id}>${e.label}`}
            className="gv-link"
            onClick={() => setSelected(other)}
            title={e.fact ? `${e.fact.rel}(${Object.entries(e.fact.args).map(([k, v]) => `${k}="${v}"`).join(', ')}) · ${e.fact.scope.replace('_', '-')}` : ''}
          >
            <span className="gv-dot" style={{ background: colorOf(other) }} />
            <span className="gv-fact">{sentence(e, n)}</span>
          </button>
        );
      };
      const kindLabel =
        n.kind === 'karin' ? 'Karin' : n.kind === 'person' ? 'person' : n.kind === 'category' ? 'category' : 'thing';
      const extra = (n.names ?? []).filter((x) => x !== n.label);
      return (
        <div className="gv-card">
          <div className="gv-card-head">
            <span className="badge">{kindLabel}</span>
            <strong>{n.label}</strong>
            <button className="gv-close" onClick={() => setSelected(null)} title="Close">
              ×
            </button>
          </div>
          {extra.length > 0 && <div className="muted">also called {extra.join(', ')}</div>}
          {outs.length > 0 && (
            <div className="gv-list">
              <div className="gv-list-title">
                {n.label} <span className="vx-count">{outs.length}</span>
              </div>
              {outs.map(row)}
            </div>
          )}
          {ins.length > 0 && (
            <div className="gv-list">
              <div className="gv-list-title">
                Points here <span className="vx-count">{ins.length}</span>
              </div>
              {ins.map(row)}
            </div>
          )}
          {outs.length + ins.length === 0 && <div className="muted gv-note">Karin knows the name, nothing more yet.</div>}
        </div>
      );
    }
    return (
      <div className="gv-card">
        <div className="gv-card-head">
          <span className="badge">{n.kind === 'rel' ? 'relation' : n.kind}</span>
          <strong>{n.label}</strong>
          <button className="gv-close" onClick={() => setSelected(null)} title="Close">
            ×
          </button>
        </div>
        {n.kind === 'rule' && (
          <>
            <div className="muted">
              from <span className="mono">{n.pattern?.provenance.replace(/^vault:/, '')}</span>
            </div>
            {list('Reads', outs.filter((e) => e.kind === 'read').map((e) => e.t))}
            {list('Writes', outs.filter((e) => e.kind === 'write').map((e) => e.t))}
            {list('Calls', outs.filter((e) => e.kind === 'call').map((e) => e.t))}
          </>
        )}
        {n.kind === 'rel' && (
          <>
            <div className="muted">{n.facts} facts stored</div>
            {list('Written by', ins.filter((e) => e.kind === 'write').map((e) => e.s))}
            {list('Read by', ins.filter((e) => e.kind === 'read').map((e) => e.s))}
          </>
        )}
        {n.kind === 'tool' && list('Called by', ins.map((e) => e.s))}
        {outs.length + ins.length === 0 && (
          <div className="muted gv-note">
            No rule reads or writes this. It is only stored, or used directly by the engine (for
            example the language layer reads the lexicon relations).
          </div>
        )}
      </div>
    );
  })();

  const q = query.trim().toLowerCase();
  const results = q ? graph.nodes.filter((n) => n.label.toLowerCase().includes(q)) : [];
  const jumpTo = (n: GNode) => {
    const v = view.current;
    autoFitted.current = 2;
    const k = Math.max(v.k, 1.3);
    v.k = k;
    v.tx = v.w / 2 - n.x * k;
    v.ty = v.h / 2 - n.y * k;
    v.dirty = true;
    setSelected(n);
  };

  const [activeIdx, setActiveIdx] = useState(0);
  useEffect(() => setActiveIdx(0), [query]);
  useEffect(() => {
    document.querySelector('.gv-result.active')?.scrollIntoView({ block: 'nearest' });
  }, [activeIdx]);
  const shownResults = results;
  if (keyHandlerRef) {
    keyHandlerRef.current = (e) => {
      if (shownResults.length === 0) return;
      if (e.key === 'ArrowDown') {
        e.preventDefault();
        setActiveIdx((i) => (i + 1) % shownResults.length);
      } else if (e.key === 'ArrowUp') {
        e.preventDefault();
        setActiveIdx((i) => (i - 1 + shownResults.length) % shownResults.length);
      } else if (e.key === 'Enter') {
        e.preventDefault();
        jumpTo(shownResults[Math.min(activeIdx, shownResults.length - 1)]);
      }
    };
  }

  const ruleCount = graph.nodes.filter((n) => n.kind === 'rule').length;
  const relCount = graph.nodes.filter((n) => n.kind === 'rel').length;

  return (
    <div className="graph-view" ref={wrapRef}>
      <canvas
        ref={canvasRef}
        onPointerDown={onPointerDown}
        onPointerMove={onPointerMove}
        onPointerUp={onPointerUp}
        onPointerLeave={() => {
          view.current.hover = null;
          view.current.dirty = true;
        }}
        onWheel={onWheel}
        style={{ cursor: 'grab', touchAction: 'none' }}
      />
      {q && (
        <div className="gv-results" role="listbox" aria-label="Search results">
          {results.length === 0 ? (
            <div className="gv-result-none">No nodes found</div>
          ) : (
            shownResults.map((n, i) => (
              <button key={n.id} className={`gv-result ${i === activeIdx ? 'active' : ''}`} role="option" aria-selected={n === selected} onClick={() => jumpTo(n)}>
                <span className="gv-dot" style={{ background: colorOf(n) }} />
                <span className="gv-result-label">{n.label}</span>
                <span className="gv-result-meta">{n.kind}</span>
              </button>
            ))
          )}
        </div>
      )}
      <div className={`gv-legend ${legendOpen ? '' : 'collapsed'}`}>
        <div className="gv-mode" role="tablist" aria-label="Graph">
          <button
            role="tab"
            aria-selected={mode === 'facts'}
            className={mode === 'facts' ? 'on' : ''}
            onClick={() => {
              setSelected(null);
              setMode('facts');
            }}
          >
            Facts
          </button>
          <button
            role="tab"
            aria-selected={mode === 'rules'}
            className={mode === 'rules' ? 'on' : ''}
            onClick={() => {
              setSelected(null);
              setMode('rules');
            }}
          >
            Rules
          </button>
        </div>
        {legendOpen && (
        <div className="gv-legend-body">
        {mode === 'facts' ? (
          <>
            <div className="gv-legend-row muted">
              {graph.nodes.length} people and things · {graph.edges.length} facts
            </div>
            <div className="gv-legend-row">
              <span className="gv-dot" style={{ background: colors.current.brand }} />
              person
            </div>
            <div className="gv-legend-row">
              <span className="gv-dot" style={{ background: colors.current.ok }} />
              Karin
            </div>
            <div className="gv-legend-row">
              <span className="gv-dot" style={{ background: colors.current.muted }} />
              thing
            </div>
            <div className="gv-legend-row">
              <span className="gv-ring" />
              category (is a)
            </div>
            <div className="gv-legend-row muted">select a node to inspect it</div>
          </>
        ) : (
          <>
        <div className="gv-legend-row muted">
          {ruleCount} rules · {relCount} relations · {graph.edges.length} links
        </div>
        {packs.map((p) => {
          const sample = graph.nodes.find((n) => n.kind === 'rule' && n.group === p)!;
          return (
            <div key={p} className="gv-legend-row">
              <span className="gv-dot" style={{ background: colorOf(sample) }} />
              {p}
            </div>
          );
        })}
        <div className="gv-legend-row">
          <span className="gv-ring" />
          relation (size = facts)
        </div>
        <div className="gv-legend-row">
          <span className="gv-square" />
          tool
        </div>
        <div className="gv-legend-row muted">grey line = reads · coloured = writes</div>
          </>
        )}
        </div>
        )}
        <button className="vx-tool" onClick={() => setLegendOpen((o) => !o)} aria-expanded={legendOpen}>
          {legendOpen ? 'Hide legend' : 'Legend'}
        </button>
        <button className="vx-tool" onClick={fit}>
          Fit
        </button>
        <span className="gv-zoom">
          <button className="vx-tool" onClick={() => zoomBy(1 / 1.3)} aria-label="Zoom out" title="Zoom out">
            −
          </button>
          <button className="vx-tool" onClick={() => zoomBy(1.3)} aria-label="Zoom in" title="Zoom in">
            +
          </button>
        </span>
      </div>
      {card}
    </div>
  );
}
