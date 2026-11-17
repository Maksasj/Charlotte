//! Milestone 17 acceptance: learned input patterns.
//!
//! The teaching turns (`data/synthetic/teaching/`, dev then held-out) are
//! translated by the translator with its model (cached). The first `teach`
//! turns teach the patterns: every validated model translation of a sentence
//! is generalized (`sla_translate::learned`). The patterns are then frozen and
//! the remaining turns are checked: a turn is handled by learned patterns when
//! every sentence the fixed patterns do not take gets a learned translation
//! (no model call needed); the learned translation agrees when it equals the
//! model's.

use serde::Serialize;
use sla_parser::Intent;
use sla_translate::learned::LearnedPatterns;
use sla_translate::{ModelTier, TContext, Tier, Translator};
use std::path::Path;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize)]
pub struct SentenceResult {
    pub text: String,
    pub tier: String,
    pub ok: bool,
    pub acts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct PatternReport {
    pub teach_turns: usize,
    pub test_turns: usize,
    pub templates: usize,
    pub active: usize,
    /// Test turns that needed the model without learned patterns.
    pub model_turns: usize,
    /// Test turns handled by learned patterns with no model call.
    pub learned_turns: usize,
    /// Learned sentence translations, and how many equal the model's.
    pub learned_sentences: usize,
    pub agree: usize,
    pub disagreements: Vec<(String, Vec<String>, Vec<String>)>,
    /// Model translations in the teaching turns that taught nothing.
    pub not_generalized: Vec<(String, Vec<String>)>,
    pub template_keys: Vec<(String, u32, Vec<String>)>,
    pub unhandled: Vec<(String, Vec<String>)>,
}

/// User turns: the teaching scripts, the parser sets and the translator gold
/// set, in a fixed shuffled order (so teaching and test turns are alike).
pub fn turns(root: &Path) -> Vec<String> {
    let mut out = Vec::new();
    let d = root.join("data");
    for set in ["dev", "held_out"] {
        let dir = d.join("synthetic").join("teaching").join(set);
        let mut files: Vec<_> = std::fs::read_dir(&dir).map(|d| d.filter_map(|e| e.ok()).map(|e| e.path()).collect()).unwrap_or_default();
        files.sort();
        for f in files {
            let mut section = String::new();
            for l in std::fs::read_to_string(&f).unwrap_or_default().lines() {
                if let Some(h) = l.strip_prefix("## ") {
                    section = h.trim().to_string();
                } else if let (Some(t), "script") = (l.strip_prefix("- "), section.as_str()) {
                    out.push(t.split(" => ").next().unwrap_or("").trim().to_string());
                }
            }
        }
        for l in std::fs::read_to_string(d.join("synthetic").join("parser").join(format!("{set}.md"))).unwrap_or_default().lines() {
            if let Some(t) = l.strip_prefix("- ") {
                out.push(t.split(" => ").next().unwrap_or("").trim().to_string());
            }
        }
    }
    for l in std::fs::read_to_string(d.join("gold").join("translator_gold.jsonl")).unwrap_or_default().lines() {
        if let Some(t) = serde_json::from_str::<serde_json::Value>(l).ok().and_then(|v| v["text"].as_str().map(String::from)) {
            out.push(t);
        }
    }
    out.retain(|t| !t.is_empty() && !t.contains(":-") && !t.contains('('));
    let h = |t: &str| t.bytes().fold(0xcbf29ce484222325u64, |h, b| (h ^ b as u64).wrapping_mul(0x100000001b3));
    out.sort_by_key(|t| h(t));
    out
}

fn ctx() -> TContext {
    TContext { speaker: "maksim".into(), next_step: 1, ..Default::default() }
}

pub fn run(root: &Path, model: &str, teach: usize, parallel: usize) -> PatternReport {
    let turns = turns(root);
    let cache = root.join("eval").join("translate_cache.jsonl");
    let registry = sla_lexicon::Registry::builtin();
    // 1. The model's translation of every turn (fixed patterns + model, no learned tier).
    let next = std::sync::atomic::AtomicUsize::new(0);
    let out: Mutex<Vec<(usize, Vec<SentenceResult>)>> = Mutex::new(Vec::new());
    std::thread::scope(|s| {
        for _ in 0..parallel.max(1) {
            s.spawn(|| {
                let mut t = Translator::new(registry.clone()).with_cache(cache.clone());
                t.model = Some(ModelTier::new(model));
                t.learn = false;
                loop {
                    let k = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    let Some(text) = turns.get(k) else { break };
                    let tr = t.translate(text, &ctx());
                    let parts = tr
                        .parts
                        .iter()
                        .map(|p| SentenceResult { text: p.text.clone(), tier: p.tier.name().into(), ok: p.ok, acts: p.acts.iter().map(Intent::canonical).collect() })
                        .collect();
                    out.lock().unwrap_or_else(|e| e.into_inner()).push((k, parts));
                }
            });
        }
    });
    let mut all = out.into_inner().unwrap_or_default();
    all.sort_by_key(|x| x.0);
    let teach = teach.min(all.len());
    // 2. Learn from the teaching turns.
    let mut p = LearnedPatterns { classes: registry.adjectives.clone(), ..Default::default() };
    let mut not_gen = Vec::new();
    for (_, parts) in &all[..teach] {
        for s in parts.iter().filter(|s| s.tier == Tier::Model.name() && s.ok && !s.text.contains('(') && !s.acts.iter().any(|a| a.starts_with("unknown"))) {
            if sla_translate::learned::generalize(&s.text, &s.acts, "maksim").is_none() {
                not_gen.push((s.text.clone(), s.acts.clone()));
            }
            p.observe(&s.text, &s.acts, "maksim");
        }
    }
    let mut rep = PatternReport { teach_turns: teach, test_turns: all.len() - teach, templates: p.templates.len(), active: p.active(), not_generalized: not_gen, ..Default::default() };
    rep.template_keys = p.templates.iter().map(|(k, t)| (k.clone(), t.seen, t.acts.clone())).collect();
    // 3. Frozen patterns on the rest.
    for (_, parts) in &all[teach..] {
        let open: Vec<&SentenceResult> = parts.iter().filter(|s| s.tier != Tier::Pattern.name() || !s.ok).collect();
        if open.is_empty() {
            continue;
        }
        rep.model_turns += 1;
        let mut handled = true;
        for s in open {
            match p.apply(&s.text, "maksim") {
                Some(lines) => {
                    rep.learned_sentences += 1;
                    if s.ok && lines == s.acts {
                        rep.agree += 1;
                    } else {
                        rep.disagreements.push((s.text.clone(), lines, s.acts.clone()));
                    }
                }
                None => {
                    handled = false;
                    rep.unhandled.push((s.text.clone(), s.acts.clone()));
                }
            }
        }
        if handled {
            rep.learned_turns += 1;
        }
    }
    rep
}

pub fn markdown(r: &PatternReport) -> String {
    let pct = |a: usize, b: usize| if b == 0 { 0.0 } else { a as f64 * 100.0 / b as f64 };
    let mut s = format!(
        "# Learned input patterns\n\nTaught on {} turns: {} templates, {} in use.\n\nOn the next {} turns: learned patterns handle **{} turns ({:.1}%)** with no model call; \
         {} of them needed the model before ({:.1}% of those).\nLearned translations agree with the model in **{} of {} sentences ({:.1}%)**.\n",
        r.teach_turns,
        r.templates,
        r.active,
        r.test_turns,
        r.learned_turns,
        pct(r.learned_turns, r.test_turns),
        r.model_turns,
        pct(r.learned_turns, r.model_turns),
        r.agree,
        r.learned_sentences,
        pct(r.agree, r.learned_sentences)
    );
    s.push_str("
## Templates

");
    for (k, n, a) in &r.template_keys {
        s.push_str(&format!("- `{k}` x{n} -> {}
", a.join(" | ")));
    }
    s.push_str("
## Model translations that taught nothing

");
    for (t, a) in &r.not_generalized {
        s.push_str(&format!("- {t} -> {}
", a.join(" | ")));
    }
    s.push_str("
## Test sentences not handled

");
    for (t, a) in &r.unhandled {
        s.push_str(&format!("- {t} -> {}
", a.join(" | ")));
    }
    if !r.disagreements.is_empty() {
        s.push_str("\n## Disagreements (learned / model)\n\n");
        for (t, a, b) in &r.disagreements {
            s.push_str(&format!("- {t}\n  - learned: {}\n  - model: {}\n", a.join(" | "), b.join(" | ")));
        }
    }
    s
}
