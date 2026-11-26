import { useEffect, useState, type ReactNode } from 'react';
import { VaultSource, type Locator } from './VaultSource';
import { RuntimeIcon, VaultIcon } from './Icons';

/**
 * Where a fact or pattern lives, as a vault-like path: folders…, then the file.
 *   vault:core/entities/me                        → vault / core / entities / me.md
 *   vault:skills/weather/weather                  → vault / skills / weather / weather.md
 *   taught:…, anti_unify(…), induced…, chunk(…)   → runtime / learned / <how>
 *   pattern:p266                                  → runtime / conversation / <rule name>
 *   perceive, meta:*                              → runtime / conversation / …
 */
export function vaultPathOf(provenance: string, ruleName: (id: string) => string | undefined): string[] {
  if (provenance.startsWith('vault:')) {
    const parts = provenance.slice('vault:'.length).split('/');
    const file = `${parts[parts.length - 1]}.md`;
    const folders = parts.slice(0, -1);
    return ['vault', ...folders, file];
  }
  if (provenance.startsWith('taught')) return ['runtime', 'learned', 'taught replies'];
  if (provenance.startsWith('anti_unify')) return ['runtime', 'learned', 'induced from episodes'];
  if (provenance.startsWith('induced')) return ['runtime', 'learned', 'induced from corrections'];
  if (provenance.startsWith('chunk')) return ['runtime', 'learned', 'chunked chains'];
  if (provenance.startsWith('pattern:')) {
    const id = provenance.slice('pattern:'.length);
    return ['runtime', 'conversation', `by ${ruleName(id) ?? id}`];
  }
  if (provenance === 'perceive') return ['runtime', 'conversation', 'heard & said'];
  if (provenance.startsWith('meta:')) return ['runtime', 'conversation', 'meta tools'];
  return ['runtime', 'other', provenance || 'unknown'];
}

type NodeType = 'folder' | 'file';

interface Node<T> {
  name: string;
  key: string;
  type: NodeType;
  children: Map<string, Node<T>>;
  items: T[];
  count: number;
}

const newNode = <T,>(name: string, key: string, type: NodeType): Node<T> => ({
  name,
  key,
  type,
  children: new Map(),
  items: [],
  count: 0,
});

/** Folders first, then files; "runtime" always after "vault". */
function sortedChildren<T>(n: Node<T>): Node<T>[] {
  return [...n.children.values()].sort((a, b) => {
    if (a.type !== b.type) return a.type === 'folder' ? -1 : 1;
    return a.name.localeCompare(b.name);
  });
}

function Chevron({ open }: { open: boolean }) {
  return (
    <svg className={`chev ${open ? 'open' : ''}`} width="10" height="10" viewBox="0 0 10 10" aria-hidden>
      <path
        d="M3.5 2 L7 5 L3.5 8"
        fill="none"
        stroke="currentColor"
        strokeWidth="1.4"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  );
}

interface Props<T> {
  items: T[];
  pathOf: (item: T) => string[];
  itemKey: (item: T) => string;
  /** One-line label of an item in the tree. */
  itemLabel: (item: T) => ReactNode;
  /** Full rendering of an item in the note pane. */
  renderItem: (item: T) => ReactNode;
  /** Optional outline heading inside a file (facts: the relation); undefined = no heading. */
  groupOf?: (item: T) => string | undefined;
  /** Icon shown before an item in the tree. */
  itemIcon?: (item: T) => ReactNode;
  /** "12 rules · 3 facts" for a list of items. */
  countLabel?: (items: T[]) => string;
  filtering: boolean;
  noun: string;
  /** Where an item sits in its file's source (for highlighting). */
  locatorOf: (item: T) => Locator;
  /** The item as DSL, for items with no vault file. */
  dslOf: (item: T) => string;
  /** Changes when the vault was reloaded, so source views re-read their files. */
  refreshKey?: number;
  /** Open this node (and item) from outside, e.g. a rule clicked in the Tools view. */
  focus?: { node: string; item?: string; nonce: number };
}

type Selection = { node: string; item?: string } | null;

export function VaultExplorer<T>({
  items,
  pathOf,
  itemKey,
  itemLabel,
  renderItem,
  groupOf,
  filtering,
  noun,
  locatorOf,
  dslOf,
  refreshKey = 0,
  focus,
  itemIcon,
  countLabel,
}: Props<T>) {
  const describe = (list: T[]) => (countLabel ? countLabel(list) : `${list.length} ${noun}`);
  const [expanded, setExpanded] = useState<Set<string>>(() => new Set(['vault', 'runtime']));
  const [selection, setSelection] = useState<Selection>(null);
  const [view, setView] = useState<'source' | 'items'>('source');

  useEffect(() => {
    if (!focus) return;
    setSelection({ node: focus.node, item: focus.item });
    const parts = focus.node.split('/');
    setExpanded((prev) => {
      const next = new Set(prev);
      for (let i = 1; i <= parts.length; i++) next.add(parts.slice(0, i).join('/'));
      return next;
    });
  }, [focus?.nonce]);

  // ---- build the tree ----
  const top = new Map<string, Node<T>>();
  const nodes = new Map<string, Node<T>>();
  for (const item of items) {
    const path = pathOf(item);
    let parent: Map<string, Node<T>> = top;
    let key = '';
    let node: Node<T> | undefined;
    path.forEach((part, i) => {
      key = key ? `${key}/${part}` : part;
      const type: NodeType = i === path.length - 1 ? 'file' : 'folder';
      node = parent.get(part);
      if (!node) {
        node = newNode<T>(part, key, type);
        parent.set(part, node);
        nodes.set(key, node);
      }
      node.count++;
      parent = node.children;
    });
    node!.items.push(item);
  }
  const roots = [...top.values()].sort((a, b) =>
    a.name === 'vault' ? -1 : b.name === 'vault' ? 1 : a.name.localeCompare(b.name),
  );

  const isOpen = (key: string) => filtering || expanded.has(key);
  const toggle = (key: string) =>
    setExpanded((prev) => {
      const next = new Set(prev);
      if (next.has(key)) next.delete(key);
      else next.add(key);
      return next;
    });

  /** Items without a heading, then headings with their items (null = show flat). */
  const groupsOf = (list: T[]): { loose: T[]; groups: [string, T[]][] } | null => {
    if (!groupOf) return null;
    const loose: T[] = [];
    const g = new Map<string, T[]>();
    for (const it of list) {
      const key = groupOf(it);
      if (key === undefined) loose.push(it);
      else g.set(key, [...(g.get(key) ?? []), it]);
    }
    if (g.size === 0 || (g.size === 1 && loose.length === 0)) return null;
    return { loose, groups: [...g.entries()] };
  };

  const current = selection && nodes.has(selection.node) ? selection : null;
  const zoneOf = (key: string) => (key.split('/')[0] === 'runtime' ? 'runtime' : 'vault');

  /** Only the two roots get an icon: the vault on disk, and runtime memory. */
  const nodeIcon = (n: Node<T>) => {
    if (n.key === 'vault') return <VaultIcon className="ic-vault" />;
    if (n.key === 'runtime') return <RuntimeIcon className="ic-runtime" />;
    return null;
  };

  // Scroll the selected item into view in the note pane.
  useEffect(() => {
    if (!current?.item || view !== 'items') return;
    // Scroll only the note pane (scrollIntoView would also move the outer panes).
    const el = document.getElementById(`vx-${current.item}`);
    const pane = el?.closest('.vx-note') as HTMLElement | null;
    if (el && pane) {
      const top = el.getBoundingClientRect().top - pane.getBoundingClientRect().top + pane.scrollTop;
      pane.scrollTo({ top: top - pane.clientHeight / 3 });
    }
  }, [current?.node, current?.item, view]);

  // ---- tree ----
  const itemRow = (it: T, fileKey: string) => {
    const k = itemKey(it);
    return (
      <button
        key={k}
        className={`vx-row vx-item ${current?.item === k ? 'active' : ''}`}
        onClick={() => setSelection({ node: fileKey, item: k })}
      >
        {itemIcon ? <span className="vx-icon">{itemIcon(it)}</span> : <span className="vx-bullet" />}
        <span className="vx-label">{itemLabel(it)}</span>
      </button>
    );
  };

  const fileChildren = (n: Node<T>) => {
    const grouped = groupsOf(n.items);
    if (!grouped) return n.items.map((it) => itemRow(it, n.key));
    return [
      ...grouped.loose.map((it) => itemRow(it, n.key)),
      ...grouped.groups.map(([g, list]) => {
      const gk = `${n.key}#${g}`;
      return (
        <div key={gk}>
          <button className="vx-row vx-heading" onClick={() => toggle(gk)}>
            <span className="vx-chev">
              <Chevron open={isOpen(gk)} />
            </span>
            <span className="vx-label">{g}</span>
            <span className="vx-count">{list.length}</span>
          </button>
          {isOpen(gk) && <div className="vx-children">{list.map((it) => itemRow(it, n.key))}</div>}
        </div>
      );
      }),
    ];
  };

  const treeNode = (n: Node<T>): ReactNode => {
    const open = isOpen(n.key);
    const selected = current?.node === n.key && !current.item;
    return (
      <div key={n.key} className={n.key.includes('/') ? undefined : `vx-zone zone-${zoneOf(n.key)}`}>
        <button
          className={`vx-row ${n.type === 'folder' ? 'vx-folder' : 'vx-file'} ${
            n.key.includes('/') ? '' : 'vx-root'
          } ${selected ? 'active' : ''}`}
          onClick={() => {
            setSelection({ node: n.key });
            if (n.type === 'folder' || !open) toggle(n.key);
          }}
        >
          <span
            className="vx-chev"
            onClick={(e) => {
              e.stopPropagation();
              toggle(n.key);
            }}
          >
            <Chevron open={open} />
          </span>
          {nodeIcon(n) && <span className="vx-icon">{nodeIcon(n)}</span>}
          <span className="vx-label">{n.type === 'file' ? n.name.replace(/\.md$/, '') : n.name}</span>
          <span className="vx-count">{n.count}</span>
        </button>
        {open && (
          <div className="vx-children">
            {n.type === 'folder' ? sortedChildren(n).map(treeNode) : fileChildren(n)}
          </div>
        )}
      </div>
    );
  };

  // ---- note pane ----
  const filesUnder = (n: Node<T>): Node<T>[] =>
    n.type === 'file' ? [n] : sortedChildren(n).flatMap(filesUnder);

  const crumbs = (key: string) => {
    const parts = key.split('/');
    return (
      <div className="vx-crumbs">
        {parts.slice(0, -1).map((p, i) => (
          <span key={i}>
            <button
              className="vx-crumb"
              onClick={() => setSelection({ node: parts.slice(0, i + 1).join('/') })}
            >
              {p}
            </button>
            <span className="vx-crumb-sep">/</span>
          </span>
        ))}
      </div>
    );
  };

  const renderList = (list: T[]) => {
    const wrap = (it: T) => {
      const k = itemKey(it);
      return (
        <div key={k} id={`vx-${k}`} className={current?.item === k ? 'vx-target' : ''}>
          {renderItem(it)}
        </div>
      );
    };
    const grouped = groupsOf(list);
    if (!grouped) return list.map(wrap);
    return [
      ...grouped.loose.map(wrap),
      ...grouped.groups.map(([g, gl]) => (
      <section key={g} className="vx-section">
        <h4 className="vx-h">
          <span className="vx-hash">#</span>
          {g}
          <span className="vx-count">{gl.length}</span>
        </h4>
        {gl.map(wrap)}
      </section>
      )),
    ];
  };

  let note: ReactNode;
  if (items.length === 0) {
    note = <div className="vx-empty">No {noun} match the filter.</div>;
  } else if (!current) {
    note = (
      <div className="vx-empty">
        <div className="vx-empty-title">{describe(items)}</div>
        <div>Pick a folder, a file, or a single item in the tree.</div>
      </div>
    );
  } else {
    const n = nodes.get(current.node)!;
    if (n.type === 'file') {
      note = (
        <>
          <div className="vx-note-head">
            <div className="vx-note-head-row">
              {crumbs(n.key)}
              <div className="vx-switch" role="tablist">
                <button className={view === 'source' ? 'on' : ''} onClick={() => setView('source')}>
                  Source
                </button>
                <button className={view === 'items' ? 'on' : ''} onClick={() => setView('items')}>
                  Items
                </button>
              </div>
            </div>
            <h3 className={`vx-title zone-${zoneOf(n.key)}`}>
              {nodeIcon(n) && <span className="vx-title-icon">{nodeIcon(n)}</span>}
              {n.name.replace(/\.md$/, '')}
              <span className="vx-count">{describe(n.items)}</span>
            </h3>
          </div>
          <div className="vx-note-body">
            {view === 'source' ? (
              <VaultSource
                vaultPath={n.key.startsWith('vault/') ? n.key.slice('vault/'.length) : null}
                refreshKey={refreshKey}
                fallback={(() => {
                  const blocks = n.items.map(dslOf);
                  // Multi-line items (rules) get a blank line between them, facts don't.
                  return blocks.join(blocks.some((b) => b.includes('\n')) ? '\n\n' : '\n');
                })()}
                locator={(() => {
                  const it = current.item ? n.items.find((x) => itemKey(x) === current.item) : undefined;
                  return it ? locatorOf(it) : undefined;
                })()}
              />
            ) : (
              renderList(n.items)
            )}
          </div>
        </>
      );
    } else {
      const files = filesUnder(n);
      note = (
        <>
          <div className="vx-note-head">
            {crumbs(n.key)}
            <h3 className={`vx-title zone-${zoneOf(n.key)}`}>
              {nodeIcon(n) && <span className="vx-title-icon">{nodeIcon(n)}</span>}
              {n.name}
              <span className="vx-count">
                {describe(files.flatMap((f) => f.items))} in {files.length}{' '}
                {files.length === 1 ? 'file' : 'files'}
              </span>
            </h3>
          </div>
          <div className="vx-note-body">
            {files.map((f) => (
              <button key={f.key} className="vx-file-link" onClick={() => setSelection({ node: f.key })}>
                <span className="vx-file-path">
                  {f.key.slice(n.key.length + 1).replace(/\.md$/, '')}
                </span>
                <span className="vx-count">{f.count}</span>
              </button>
            ))}
          </div>
        </>
      );
    }
  }

  return (
    <div className="vault-explorer">
      <nav className="vx-tree">
        <div className="vx-tree-actions">
          <button className="vx-tool" onClick={() => setExpanded(new Set(['vault', 'runtime']))}>
            Collapse all
          </button>
        </div>
        {roots.map(treeNode)}
      </nav>
      <article className="vx-note">{note}</article>
    </div>
  );
}
