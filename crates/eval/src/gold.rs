//! Milestone 14 acceptance: the translator against the gold sets written
//! without sight of the code (`data/gold/`).
//!
//! - **Atom-level F1**: every act and every atom of a message is one item
//!   (`tell|likes(cat, fish)`, `request|plan_dinner`, `greet`); predicted
//!   items are compared with the gold items of the same message.
//! - **Act accuracy**: a message is right when its multiset of act names
//!   (without `unknown`) equals the gold one.
//! - **Long messages**: a gold sentence is recovered when at least half of
//!   its items appear in the translation of the message.
//! - **Grounding**: constants of stored acts that are in neither the message
//!   nor the context, re-checked independently of the validator.
//! - **Speed**: translation time per message.

use serde::{Deserialize, Serialize};
use sla_parser::llm::{from_canonical, numbered};
use sla_parser::Intent;
use sla_translate::{ModelTier, TContext, Translator};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::sync::Mutex;

#[derive(Debug, Clone, Deserialize)]
pub struct GoldContext {
    pub speaker: Option<String>,
    pub active_task: Option<String>,
    pub next_step: Option<usize>,
    pub last_reply: Option<String>,
    #[serde(default)]
    pub recent: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GoldItem {
    pub id: String,
    pub text: String,
    pub context: Option<GoldContext>,
    pub lines: Vec<String>,
}

pub fn load(path: &Path) -> std::io::Result<Vec<GoldItem>> {
    let src = std::fs::read_to_string(path)?;
    Ok(src.lines().filter(|l| !l.trim().is_empty()).filter_map(|l| serde_json::from_str(l).ok()).collect())
}

/// The items of one act: its name, plus `name|atom` per atom or payload.
pub fn items(i: &Intent) -> Vec<String> {
    let name = i.name();
    let c = i.canonical();
    let payload = c.split_once(':').map(|(_, p)| p.trim()).unwrap_or("");
    let mut out = vec![name.to_string()];
    match i {
        Intent::Tell { facts, rules, .. } | Intent::Teach { facts, rules } => {
            out.extend(facts.iter().map(|a| format!("{name}|{a}")));
            out.extend(rules.iter().map(|r| format!("{name}|{r}")));
            if let Intent::Tell { past: true, .. } = i {
                out.push(format!("{name}|@past"));
            }
            if let Intent::Tell { hedge: Some(h), .. } = i {
                out.push(format!("{name}|@{h}"));
            }
        }
        Intent::Inform(v) | Intent::Correct(v) => out.extend(v.iter().map(|a| format!("{name}|{a}"))),
        Intent::Lack(v) => out.extend(v.iter().map(|a| format!("{name}|{a}"))),
        _ if !payload.is_empty() => out.push(format!("{name}|{payload}")),
        _ => {}
    }
    out
}

fn gold_parse(item: &GoldItem) -> BTreeMap<usize, Vec<Intent>> {
    let mut by: BTreeMap<usize, Vec<Intent>> = BTreeMap::new();
    for l in &item.lines {
        if let Some((n, rest)) = numbered(l) {
            if let Some(i) = from_canonical(rest) {
                by.entry(n).or_default().push(i);
            }
        }
    }
    by
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ItemResult {
    pub id: String,
    pub text: String,
    pub gold: Vec<String>,
    pub got: Vec<String>,
    pub tp: usize,
    pub act_ok: bool,
    pub sentences: usize,
    pub recovered: usize,
    pub ms: f64,
    pub model_calls: u64,
    pub ungrounded: Vec<String>,
    pub problems: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GoldReport {
    pub set: String,
    pub items: Vec<ItemResult>,
}

fn count(v: &[String]) -> BTreeMap<String, usize> {
    let mut m = BTreeMap::new();
    for x in v {
        *m.entry(x.clone()).or_default() += 1;
    }
    m
}

impl GoldReport {
    pub fn f1(&self) -> (f64, f64, f64) {
        let tp: usize = self.items.iter().map(|i| i.tp).sum();
        let got: usize = self.items.iter().map(|i| i.got.len()).sum();
        let gold: usize = self.items.iter().map(|i| i.gold.len()).sum();
        let p = if got == 0 { 0.0 } else { tp as f64 / got as f64 };
        let r = if gold == 0 { 0.0 } else { tp as f64 / gold as f64 };
        let f = if p + r == 0.0 { 0.0 } else { 2.0 * p * r / (p + r) };
        (p, r, f)
    }

    pub fn act_accuracy(&self) -> f64 {
        self.items.iter().filter(|i| i.act_ok).count() as f64 / self.items.len().max(1) as f64
    }

    pub fn recovered(&self) -> (usize, usize) {
        (self.items.iter().map(|i| i.recovered).sum(), self.items.iter().map(|i| i.sentences).sum())
    }

    pub fn times(&self) -> Vec<f64> {
        self.items.iter().map(|i| i.ms).collect()
    }

    pub fn ungrounded(&self) -> usize {
        self.items.iter().map(|i| i.ungrounded.len()).sum()
    }
}

/// Constants in stored acts that are neither in the message nor in the
/// context (an independent re-check of the validator's grounding).
fn ungrounded(text: &str, ctx: &TContext, acts: &[Intent], registry: &sla_lexicon::Registry) -> Vec<String> {
    let words = sla_translate::validate::Words::of(text);
    let mut out = Vec::new();
    for i in acts {
        let facts = match i {
            Intent::Tell { facts, .. } | Intent::Inform(facts) | Intent::Correct(facts) | Intent::Teach { facts, .. } => facts,
            _ => continue,
        };
        for a in facts {
            for (k, t) in a.args.iter().enumerate() {
                let sla_core::Term::Val(v) = t else { continue };
                match v {
                    sla_core::Value::Sym(s) => {
                        if s == &ctx.speaker || registry.is_unit(s) || ctx.recent.contains(s) || ctx.active_task.as_deref() == Some(s.as_str()) {
                            continue;
                        }
                        if a.pred.trim_start_matches('-') == "property" && k == 1 && registry.property_names.contains(s) {
                            continue;
                        }
                        let task_like = s.contains("_s") || matches!(a.pred.as_str(), "step" | "task" | "needs" | "takes_time" | "solves" | "subtask");
                        let ok = s.split('_').filter(|p| !p.is_empty() && *p != "my").enumerate().all(|(n, p)| {
                            words.has(p) || ctx.recent.iter().chain(ctx.active_task.iter()).any(|r| r.split('_').any(|q| q == p)) || (task_like && n == 0) || p.starts_with('s') && p[1..].chars().all(|c| c.is_ascii_digit())
                        });
                        if !ok {
                            out.push(format!("{a}: {s}"));
                        }
                    }
                    _ => {
                        if !words.numbers.contains(&v.to_string()) {
                            out.push(format!("{a}: {v}"));
                        }
                    }
                }
            }
        }
    }
    out
}

/// Translate every gold item with a model translator, `parallel` at a time.
pub fn run(root: &Path, set: &str, model: &str, parallel: usize, use_cache: bool) -> std::io::Result<GoldReport> {
    let path = root.join("data").join("gold").join(format!("{set}.jsonl"));
    let items = load(&path)?;
    let cache = root.join("eval").join("translate_cache.jsonl");
    let registry = sla_lexicon::Registry::builtin();
    let next = std::sync::atomic::AtomicUsize::new(0);
    let out: Mutex<Vec<ItemResult>> = Mutex::new(Vec::new());
    std::thread::scope(|s| {
        for _ in 0..parallel.max(1) {
            s.spawn(|| {
                let mut t = if use_cache { Translator::new(registry.clone()).with_cache(cache.clone()) } else { Translator::new(registry.clone()) };
                t.model = Some(ModelTier::new(model));
                loop {
                    let k = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    let Some(item) = items.get(k) else { break };
                    let c = item.context.clone();
                    let ctx = TContext {
                        speaker: c.as_ref().and_then(|c| c.speaker.clone()).unwrap_or_else(|| "maksim".into()),
                        active_task: c.as_ref().and_then(|c| c.active_task.clone()),
                        next_step: c.as_ref().and_then(|c| c.next_step).unwrap_or(1),
                        last_reply: c.as_ref().and_then(|c| c.last_reply.clone()),
                        recent: c.as_ref().map(|c| c.recent.clone()).unwrap_or_default(),
                        known_tasks: Vec::new(),
                        task_hints: Vec::new(),
                    };
                    let started = std::time::Instant::now();
                    let tr = t.translate(&item.text, &ctx);
                    let ms = started.elapsed().as_secs_f64() * 1000.0;
                    let gold = gold_parse(item);
                    let gold_items: Vec<String> = gold.values().flatten().filter(|i| **i != Intent::Unknown).flat_map(items_of).collect();
                    let got_acts: Vec<Intent> = tr.parts.iter().filter(|p| p.ok).flat_map(|p| p.acts.clone()).filter(|i| *i != Intent::Unknown).collect();
                    let got_items: Vec<String> = got_acts.iter().flat_map(items_of).collect();
                    let (gc, pc) = (count(&gold_items), count(&got_items));
                    let tp: usize = gc.iter().map(|(k, n)| (*n).min(*pc.get(k).unwrap_or(&0))).sum();
                    let names = |v: &[Intent]| -> Vec<String> {
                        let mut x: Vec<String> = v.iter().map(|i| i.name().to_string()).collect();
                        x.sort();
                        x
                    };
                    let gold_acts: Vec<Intent> = gold.values().flatten().filter(|i| **i != Intent::Unknown).cloned().collect();
                    let act_ok = names(&gold_acts) == names(&got_acts);
                    let got_set: BTreeSet<&String> = got_items.iter().collect();
                    let mut sentences = 0;
                    let mut recovered = 0;
                    for acts in gold.values() {
                        let its: Vec<String> = acts.iter().filter(|i| **i != Intent::Unknown).flat_map(items_of).collect();
                        if its.is_empty() {
                            continue;
                        }
                        sentences += 1;
                        if its.iter().filter(|x| got_set.contains(x)).count() * 2 >= its.len() {
                            recovered += 1;
                        }
                    }
                    let problems = tr.parts.iter().filter(|p| !p.ok).map(|p| format!("{}: {}", p.text, p.problems.join("; "))).collect();
                    let r = ItemResult {
                        id: item.id.clone(),
                        text: item.text.clone(),
                        gold: gold_items,
                        got: got_items,
                        tp,
                        act_ok,
                        sentences,
                        recovered,
                        ms,
                        model_calls: tr.model_calls,
                        ungrounded: ungrounded(&item.text, &ctx, &got_acts, &registry),
                        problems,
                    };
                    out.lock().unwrap_or_else(|e| e.into_inner()).push(r);
                }
            });
        }
    });
    let mut items = out.into_inner().unwrap_or_default();
    items.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(GoldReport { set: set.into(), items })
}

fn items_of(i: &Intent) -> Vec<String> {
    items(i)
}

/// A Markdown report with every translation, and the misses first.
pub fn markdown(r: &GoldReport) -> String {
    let (p, rc, f) = r.f1();
    let (rec, tot) = r.recovered();
    let t = r.times();
    let pct = |v: &[f64], q: f64| crate::regression::percentile(v, q);
    let mut s = format!(
        "# Translator on `{}`\n\nAtom-level precision {:.3}, recall {:.3}, F1 **{:.3}**; act accuracy **{:.1}%**; sentences recovered {rec}/{tot} ({:.1}%); ungrounded constants {}; translation time median {:.0} ms, 95th percentile {:.0} ms; model calls {}.\n\n",
        r.set,
        p,
        rc,
        f,
        r.act_accuracy() * 100.0,
        rec as f64 * 100.0 / tot.max(1) as f64,
        r.ungrounded(),
        pct(&t, 50.0),
        pct(&t, 95.0),
        r.items.iter().map(|i| i.model_calls).sum::<u64>()
    );
    s.push_str("## Items that differ from the gold\n\n");
    for i in r.items.iter().filter(|i| i.tp != i.gold.len() || i.got.len() != i.gold.len()) {
        let g: BTreeSet<&String> = i.gold.iter().collect();
        let o: BTreeSet<&String> = i.got.iter().collect();
        s.push_str(&format!("### {} {}\n\n> {}\n\n", i.id, if i.act_ok { "" } else { "(acts differ)" }, i.text));
        for m in g.difference(&o) {
            s.push_str(&format!("- missing `{m}`\n"));
        }
        for m in o.difference(&g) {
            s.push_str(&format!("- extra `{m}`\n"));
        }
        for p in &i.problems {
            s.push_str(&format!("- not understood: {p}\n"));
        }
        for u in &i.ungrounded {
            s.push_str(&format!("- ungrounded: {u}\n"));
        }
        s.push('\n');
    }
    s
}
