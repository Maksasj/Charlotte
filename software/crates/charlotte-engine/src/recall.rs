//! Remembered replies: finds, for a new message, the most similar message from the taught chats and
//! hands back the reply that followed it. Similarity is word overlap weighted by how rare each word is,
//! with a smaller share for what was said just before.

use charlotte_core::*;
use std::collections::{HashMap, HashSet};

/// A message has to be at least this similar to a remembered one for its reply to be reused.
pub const THRESHOLD: f64 = 0.45;

pub struct ReplyMemory {
    cases: Vec<ReplyCase>,
    /// Per case: the weighted words of its message, and of what preceded it.
    message: Vec<HashMap<u32, f32>>,
    previous: Vec<HashMap<u32, f32>>,
    /// Word to the cases whose message has it.
    index: HashMap<u32, Vec<u32>>,
    words: HashMap<String, u32>,
    idf: Vec<f32>,
    stop: HashSet<String>,
}

fn tokens<'a>(text: &'a str, stop: &'a HashSet<String>) -> impl Iterator<Item = String> + 'a {
    text.split(|c: char| !(c.is_alphanumeric() || c == '\'')).map(|w| w.trim_matches('\'').to_lowercase()).filter(move |w| w.len() >= 2 && !stop.contains(w))
}

impl ReplyMemory {
    pub fn new(mem: &dyn Memory) -> ReplyMemory {
        let stop: HashSet<String> = mem.lexicon().stopwords.iter().cloned().collect();
        let cases = mem.replies().to_vec();
        let mut words: HashMap<String, u32> = HashMap::new();
        let mut df: Vec<u32> = Vec::new();
        let mut raw: Vec<Vec<u32>> = Vec::new();
        for c in &cases {
            let mut ids: Vec<u32> = Vec::new();
            for w in tokens(&c.message, &stop) {
                let n = words.len() as u32;
                ids.push(*words.entry(w).or_insert(n));
            }
            if df.len() < words.len() {
                df.resize(words.len(), 0);
            }
            for id in ids.iter().collect::<HashSet<_>>() {
                df[*id as usize] += 1;
            }
            raw.push(ids);
        }
        let n = cases.len().max(1) as f32;
        let idf: Vec<f32> = df.iter().map(|d| (1.0 + n / (*d as f32).max(0.5)).ln()).collect();
        let mut m = ReplyMemory { cases, message: Vec::new(), previous: Vec::new(), index: HashMap::new(), words, idf, stop };
        for (i, ids) in raw.iter().enumerate() {
            let v = m.weigh(ids.iter().copied());
            for w in v.keys() {
                m.index.entry(*w).or_default().push(i as u32);
            }
            m.message.push(v);
        }
        m.previous = m.cases.iter().map(|c| c.previous.as_deref().map(|p| m.vector(p)).unwrap_or_default()).collect();
        m
    }
    /// Words to weights, scaled to length one.
    fn weigh(&self, ids: impl Iterator<Item = u32>) -> HashMap<u32, f32> {
        let mut v: HashMap<u32, f32> = HashMap::new();
        for id in ids {
            *v.entry(id).or_insert(0.0) += self.idf[id as usize];
        }
        let norm = v.values().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            v.values_mut().for_each(|x| *x /= norm);
        }
        v
    }
    /// A text's vector over the words the memory knows, and the share of its words that it knows.
    fn vector(&self, text: &str) -> HashMap<u32, f32> {
        self.weigh(tokens(text, &self.stop).filter_map(|w| self.words.get(&w).copied()))
    }
    fn known_share(&self, text: &str) -> f32 {
        let all: Vec<String> = tokens(text, &self.stop).collect();
        if all.is_empty() {
            return 0.0;
        }
        all.iter().filter(|w| self.words.contains_key(*w)).count() as f32 / all.len() as f32
    }
}
fn dot(a: &HashMap<u32, f32>, b: &HashMap<u32, f32>) -> f32 {
    let (small, big) = if a.len() <= b.len() { (a, b) } else { (b, a) };
    small.iter().filter_map(|(k, x)| big.get(k).map(|y| x * y)).sum()
}

impl Recall for ReplyMemory {
    fn recall(&self, text: &str, previous: Option<&str>) -> Option<Recalled> {
        let q = self.vector(text);
        if q.is_empty() {
            return None;
        }
        // Words the memory has never seen count against the match.
        let known = self.known_share(text).sqrt();
        let candidates: HashSet<u32> = q.keys().filter_map(|w| self.index.get(w)).flatten().copied().collect();
        let before = previous.map(|p| self.vector(p)).filter(|v| !v.is_empty());
        let mut best: Option<(f32, u32)> = None;
        for c in candidates {
            let i = c as usize;
            let msg = dot(&q, &self.message[i]) * known;
            // What came before counts for a fifth when both sides have something before them.
            let score = match (&before, self.previous[i].is_empty()) {
                (Some(b), false) => 0.8 * msg + 0.2 * dot(b, &self.previous[i]),
                (None, true) => msg,
                _ => 0.9 * msg,
            };
            if best.map_or(true, |(s, j)| score > s || (score == s && c < j)) {
                best = Some((score, c));
            }
        }
        let (score, i) = best?;
        if (score as f64) < THRESHOLD {
            return None;
        }
        let c = &self.cases[i as usize];
        Some(Recalled { source: c.source.clone(), entry: c.entry, message: c.message.clone(), reply: c.reply.clone(), score: score as f64 })
    }
}
