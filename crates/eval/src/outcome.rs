//! Outcome labels (Design 2.0, "Honest measurement"): every agent turn gets
//! exactly one label saying what it did for the user.
//!
//! - Fixed-template replies (the "not understood" family, pure small talk,
//!   bare acknowledgements of a request) are labelled by exact match, with no
//!   model.
//! - Content replies are labelled by a Haiku judge with a rubric. Judge
//!   labels are cached on disk by (model, rubric version, turn), so the same
//!   commit run twice gets the same labels.
//! - Without a judge, content replies stay `unjudged` and are reported as such.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Label {
    /// Gives the user what they asked for or needed (a fitting social reply to a social message counts).
    Helpful,
    /// Some useful content, but a main part of the request is missing.
    Partial,
    /// A fitting question: a missing detail, or an unclear part of the message.
    Clarify,
    /// Honestly says it does not know this yet and asks to be taught.
    TeachMe,
    /// Says it did not understand, or nothing of the message was understood.
    NotUnderstood,
    /// The kind of reply does not fit: small talk to a request, an answer to another question, an off-topic question.
    WrongMove,
    /// States something false, garbled or unsupported as an answer.
    WrongContent,
    /// Only repeats what the user said.
    Echo,
    /// A content reply with no judge available.
    Unjudged,
}

impl Label {
    pub const ALL: [Label; 9] = [
        Label::Helpful,
        Label::Partial,
        Label::Clarify,
        Label::TeachMe,
        Label::NotUnderstood,
        Label::WrongMove,
        Label::WrongContent,
        Label::Echo,
        Label::Unjudged,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Label::Helpful => "helpful",
            Label::Partial => "partial",
            Label::Clarify => "clarify",
            Label::TeachMe => "teach_me",
            Label::NotUnderstood => "not_understood",
            Label::WrongMove => "wrong_move",
            Label::WrongContent => "wrong_content",
            Label::Echo => "echo",
            Label::Unjudged => "unjudged",
        }
    }

    pub fn parse(s: &str) -> Option<Label> {
        let s = s.trim().trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '_' && c != ' ' && c != '-').to_ascii_lowercase().replace([' ', '-'], "_");
        Label::ALL.iter().copied().find(|l| l.name() == s)
    }

    /// Turns that move the user forward: a helpful answer or a fitting question.
    pub fn is_good(self) -> bool {
        matches!(self, Label::Helpful | Label::Clarify)
    }

    /// A move that fits a request, even if it is not the answer yet.
    pub fn fits_request(self) -> bool {
        matches!(self, Label::Helpful | Label::Partial | Label::Clarify | Label::TeachMe)
    }
}

/// What the user's message asks for, by simple surface cues.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct UserCues {
    pub request: bool,
    pub question: bool,
    pub social_only: bool,
}

const REQUEST_CUES: &[&str] = &[
    "can you", "could you", "would you", "will you", "help me", "how do i", "how can i", "how should i", "how to ", "how long",
    "what should i", "what can i", "what could i", "what do i need", "what do i do", "what else should", "any tips", "any ideas", "any advice",
    "any suggestions", "i need", "i want to", "i'd like", "i would like", "recommend", "suggest", "give me", "tell me how", "what's the best",
    "what is the best", "which ", "what's next", "what next", "should i",
];

const SOCIAL_WORDS: &[&str] = &[
    "hi", "hello", "hey", "thanks", "thank", "you", "so", "much", "great", "cool", "nice", "ok", "okay", "bye", "goodbye", "see", "awesome",
    "perfect", "sounds", "good", "wonderful", "lovely", "cheers", "that's", "helpful", "really", "appreciate", "it", "a", "lot", "have", "day",
    "night", "morning", "evening", "there", "wow", "yes", "sure", "the", "for", "help", "your",
];

pub fn user_cues(text: &str) -> UserCues {
    let low = text.to_lowercase();
    let request = REQUEST_CUES.iter().any(|c| low.contains(c));
    let question = text.contains('?');
    let words: Vec<String> = low.split(|c: char| !(c.is_alphanumeric() || c == '\'')).filter(|w| !w.is_empty()).map(String::from).collect();
    let social_only = !words.is_empty() && words.len() <= 10 && words.iter().all(|w| SOCIAL_WORDS.contains(&w.as_str())) && !request;
    UserCues { request, question, social_only }
}

/// Every wording of the "not understood" family.
const NOT_UNDERSTOOD: &[&str] = &["Sorry, I didn't understand that.", "I'm not sure what you mean.", "Could you say that more simply?"];

/// Fixed social lines (greetings, thanks, reactions, small talk about itself).
const SOCIAL_LINES: &[&str] = &[
    "Hello!",
    "Hi there!",
    "Hello, nice to talk to you.",
    "Goodbye!",
    "Bye, talk to you later.",
    "See you!",
    "Thanks!",
    "You're welcome!",
    "Glad I could help.",
    "Any time!",
    "Yes! Tell me something new, or ask me a question.",
    "I think so too. What else would you like to talk about?",
    "Right. What else can you tell me?",
    "That's okay.",
    "Okay, never mind.",
    "Good.",
    "Okay.",
    "Good, thanks! And you?",
    "I'm well. What's on your mind?",
    "Fine, thanks. Tell me something new!",
    "I'm good, thanks!",
    "Doing well, thanks for asking.",
    "All good here. I'm enjoying learning.",
    "I don't really do things myself, I only learn and think. Tell me about it!",
    "I'm a learning program, so I don't have days like you do. What about you?",
    "Nothing much happens to me; I just learn from talks like this one. What's new with you?",
];

const ACK_LEADS: &[&str] = &["Okay, I'll remember that.", "Got it.", "Noted."];

/// Content markers: a reply with one of these says more than an acknowledgement.
const CONTENT_MARKS: &[&str] = &["Here is what I know", "About ", "This is what I know", "because", "here is what I know", "Yes,", "No,", "That's right", "I don't know", "I'm not sure whether", "I have no idea"];

fn strip_all(mut s: String, lines: &[&str]) -> String {
    for l in lines {
        s = s.replace(l, " ");
    }
    s
}

fn is_blank(s: &str) -> bool {
    !s.chars().any(|c| c.is_alphanumeric())
}

/// The label of a fixed-template reply, or `None` for a content reply.
pub fn template_label(user: &str, reply: &str) -> Option<Label> {
    let reply = reply.trim();
    let cues = user_cues(user);
    if reply.is_empty() {
        return Some(Label::NotUnderstood);
    }
    if NOT_UNDERSTOOD.iter().any(|t| reply.starts_with(t)) {
        return Some(Label::NotUnderstood);
    }
    // Only small talk.
    if is_blank(&strip_all(reply.to_string(), SOCIAL_LINES)) {
        return Some(if cues.request || (cues.question && !cues.social_only) { Label::WrongMove } else { Label::Helpful });
    }
    // An acknowledgement where an answer was needed.
    if (cues.request || cues.question) && ACK_LEADS.iter().any(|l| reply.starts_with(l)) && !CONTENT_MARKS.iter().any(|m| reply.contains(m)) && !reply.contains('?') {
        return Some(Label::Echo);
    }
    None
}

/// One agent turn to label, with what came before it.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TurnToLabel {
    pub id: String,
    pub scenario: String,
    /// Earlier (user, agent) pairs of the same conversation.
    pub history: Vec<(String, String)>,
    pub user: String,
    pub reply: String,
}

/// Rubric version: bump it when the rubric changes, so cached labels are not reused.
pub const RUBRIC_VERSION: u32 = 1;

pub fn judge_prompt(t: &TurnToLabel) -> String {
    let hist = if t.history.is_empty() {
        "(this is the first turn)".to_string()
    } else {
        t.history.iter().rev().take(3).rev().map(|(u, a)| format!("User: {u}\nAgent: {a}")).collect::<Vec<_>>().join("\n")
    };
    format!(
        "You label one reply of a chat agent. The agent is a small symbolic program; judge only what its reply does for the user.\n\
         Situation of the user: {}\n\nEarlier turns (most recent last):\n{hist}\n\nUser: {}\nAgent: {}\n\n\
         Choose exactly one label for the agent's last reply:\n\
         - helpful: gives what the user asked for or needed, correctly (a fitting reply to pure small talk also counts)\n\
         - partial: some useful, correct content, but a main part of what was asked is missing\n\
         - clarify: asks a fitting question that is needed to help (missing detail, or an unclear part)\n\
         - teach_me: honestly says it does not know this yet and asks to be taught\n\
         - not_understood: says it did not understand, or reacts as if nothing was understood\n\
         - wrong_move: the kind of reply does not fit (small talk or an acknowledgement where an answer was needed, an answer to a different question, an off-topic question)\n\
         - wrong_content: states something false, garbled or unrelated as if it were an answer\n\
         - echo: only repeats back what the user said and adds nothing\n\n\
         Reply with exactly one line: LABEL: <label>",
        t.scenario, t.user, t.reply
    )
}

pub fn parse_judge(out: &str) -> Option<Label> {
    out.lines().find_map(|l| l.trim().strip_prefix("LABEL:").and_then(Label::parse)).or_else(|| Label::parse(out.trim()))
}

/// How a label was obtained.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabelSource {
    Template,
    Judge,
    Cache,
    Gold,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CacheLine {
    key: String,
    label: Label,
}

fn cache_key(model: &str, t: &TurnToLabel) -> String {
    // FNV-1a over the inputs the judge sees: stable across runs and platforms.
    let mut h: u64 = 0xcbf29ce484222325;
    let last = t.history.iter().rev().take(3).map(|(u, a)| format!("{u}\u{1}{a}")).collect::<Vec<_>>().join("\u{2}");
    for part in [model, &RUBRIC_VERSION.to_string(), &t.scenario, &last, &t.user, &t.reply] {
        for b in part.as_bytes().iter().chain(std::iter::once(&0u8)) {
            h ^= *b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
    }
    format!("{h:016x}")
}

pub struct Labeller {
    /// Judge model; `None` labels templates only.
    pub judge: Option<String>,
    cache_path: Option<PathBuf>,
    cache: HashMap<String, Label>,
    /// Gold labels by turn id (hand labels) take precedence over everything.
    pub gold: HashMap<String, Label>,
    pub judge_calls: usize,
}

impl Labeller {
    pub fn new(judge: Option<String>, cache_path: Option<PathBuf>) -> Self {
        let mut cache = HashMap::new();
        if let Some(p) = &cache_path {
            if let Ok(src) = std::fs::read_to_string(p) {
                for l in src.lines() {
                    if let Ok(c) = serde_json::from_str::<CacheLine>(l) {
                        cache.insert(c.key, c.label);
                    }
                }
            }
        }
        Labeller { judge, cache_path, cache, gold: HashMap::new(), judge_calls: 0 }
    }

    pub fn label(&mut self, t: &TurnToLabel) -> (Label, LabelSource) {
        if let Some(l) = self.gold.get(&t.id) {
            return (*l, LabelSource::Gold);
        }
        if let Some(l) = template_label(&t.user, &t.reply) {
            return (l, LabelSource::Template);
        }
        let Some(model) = self.judge.clone() else { return (Label::Unjudged, LabelSource::None) };
        let key = cache_key(&model, t);
        if let Some(l) = self.cache.get(&key) {
            return (*l, LabelSource::Cache);
        }
        self.judge_calls += 1;
        let Ok(out) = crate::claude_as("labeller", &judge_prompt(t), &model) else { return (Label::Unjudged, LabelSource::None) };
        let Some(l) = parse_judge(&out) else { return (Label::Unjudged, LabelSource::None) };
        if let Some(p) = &self.cache_path {
            if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(p) {
                if let Ok(mut line) = serde_json::to_string(&CacheLine { key: key.clone(), label: l }) {
                    line.push('\n');
                    let _ = f.write_all(line.as_bytes());
                }
            }
        }
        self.cache.insert(key, l);
        (l, LabelSource::Judge)
    }
}

/// Wilson score interval (95%) for `k` successes in `n`.
pub fn wilson(k: usize, n: usize) -> (f64, f64, f64) {
    if n == 0 {
        return (0.0, 0.0, 0.0);
    }
    let z = 1.96f64;
    let (k, n) = (k as f64, n as f64);
    let p = k / n;
    let d = 1.0 + z * z / n;
    let c = (p + z * z / (2.0 * n)) / d;
    let h = z * ((p * (1.0 - p) / n) + z * z / (4.0 * n * n)).sqrt() / d;
    (p, (c - h).max(0.0), (c + h).min(1.0))
}

/// The outcome mix of a set of labelled turns.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Mix {
    pub n: usize,
    pub counts: BTreeMap<Label, usize>,
    /// Turns whose user message contained a request, and how many got a fitting move.
    pub requests: usize,
    pub requests_fitting: usize,
}

impl Mix {
    pub fn add(&mut self, user: &str, l: Label) {
        self.n += 1;
        *self.counts.entry(l).or_default() += 1;
        if user_cues(user).request {
            self.requests += 1;
            if l.fits_request() {
                self.requests_fitting += 1;
            }
        }
    }

    pub fn count(&self, l: Label) -> usize {
        self.counts.get(&l).copied().unwrap_or(0)
    }

    pub fn rate(&self, l: Label) -> (f64, f64, f64) {
        wilson(self.count(l), self.n)
    }

    /// Helpful answers plus fitting clarifications.
    pub fn good(&self) -> (f64, f64, f64) {
        wilson(self.count(Label::Helpful) + self.count(Label::Clarify), self.n)
    }

    pub fn request_fit(&self) -> (f64, f64, f64) {
        wilson(self.requests_fitting, self.requests)
    }

    /// A readable table with 95% intervals.
    pub fn table(&self) -> String {
        let pct = |(p, lo, hi): (f64, f64, f64)| format!("{:5.1}%  [{:4.1}, {:4.1}]", p * 100.0, lo * 100.0, hi * 100.0);
        let mut s = format!("{:<34} {:>6}  {}\n", "label", "turns", "rate   [95% interval]");
        for l in Label::ALL {
            if self.count(l) > 0 || l != Label::Unjudged {
                s.push_str(&format!("{:<34} {:>6}  {}\n", l.name(), self.count(l), pct(self.rate(l))));
            }
        }
        s.push_str(&format!("{:<34} {:>6}  {}\n", "helpful + clarify", self.count(Label::Helpful) + self.count(Label::Clarify), pct(self.good())));
        s.push_str(&format!("{:<34} {:>6}  {}\n", format!("requests with a fitting move (of {})", self.requests), self.requests_fitting, pct(self.request_fit())));
        s.push_str(&format!("{:<34} {:>6}\n", "turns", self.n));
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn templates() {
        let help = "Hey, I need some help with dinner tonight. What can I make?";
        assert_eq!(template_label(help, "Sorry, I didn't understand that. Short sentences like \"Anna likes tea.\" work best."), Some(Label::NotUnderstood));
        assert_eq!(template_label(help, "Hello!"), Some(Label::WrongMove));
        assert_eq!(template_label("Thanks so much!", "You're welcome!"), Some(Label::Helpful));
        assert_eq!(template_label(help, "Okay, I'll remember that. You have pasta, garlic, olive oil and cherry tomato."), Some(Label::Echo));
        assert_eq!(template_label("Anna likes tea.", "Got it. Anna likes tea."), None, "an acknowledged lesson is judged");
        assert_eq!(template_label(help, "Here is what I know about pasta: Pasta is a food."), None);
    }

    #[test]
    fn labels_parse() {
        for l in Label::ALL {
            assert_eq!(Label::parse(l.name()), Some(l));
        }
        assert_eq!(parse_judge("LABEL: teach_me"), Some(Label::TeachMe));
        assert_eq!(parse_judge("label is\nLABEL: Wrong move"), Some(Label::WrongMove));
        assert_eq!(parse_judge("I think it is good"), None);
    }

    #[test]
    fn wilson_interval() {
        let (p, lo, hi) = wilson(1, 68);
        assert!((p - 0.0147).abs() < 0.001 && lo < p && hi > p && hi < 0.08, "{lo} {hi}");
        assert_eq!(wilson(0, 0), (0.0, 0.0, 0.0));
    }

    #[test]
    fn judge_with_mock_and_cache() {
        use std::sync::Arc;
        let dir = std::env::temp_dir().join(format!("sla_label_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("cache.jsonl");
        let _ = std::fs::remove_file(&path);
        sla_parser::claude::set_thread_mock(Arc::new(|role, prompt| {
            assert_eq!(role, "labeller");
            assert!(prompt.contains("Agent: Boil the pasta"));
            Ok("LABEL: helpful".into())
        }));
        let t = TurnToLabel { id: "x".into(), scenario: "dinner".into(), history: vec![], user: "How do I make it?".into(), reply: "Boil the pasta, then add the garlic.".into() };
        let mut lab = Labeller::new(Some("haiku".into()), Some(path.clone()));
        assert_eq!(lab.label(&t), (Label::Helpful, LabelSource::Judge));
        assert_eq!(lab.label(&t), (Label::Helpful, LabelSource::Cache));
        // A new labeller reads the cache from disk: no model call.
        sla_parser::claude::set_thread_mock(Arc::new(|_, _| panic!("must not call the model")));
        let mut lab2 = Labeller::new(Some("haiku".into()), Some(path.clone()));
        assert_eq!(lab2.label(&t), (Label::Helpful, LabelSource::Cache));
        assert_eq!(lab2.judge_calls, 0);
        // Without a judge, content replies stay unjudged.
        let mut none = Labeller::new(None, None);
        assert_eq!(none.label(&t), (Label::Unjudged, LabelSource::None));
        sla_parser::claude::clear_mocks();
        let _ = std::fs::remove_dir_all(&dir);
    }
}
