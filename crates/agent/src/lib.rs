//! The agent: message -> parser -> knowledge base update or query -> reply -> log.
//!
//! An [`Agent`] is one conversation (a session) over a [`Brain`], which owns
//! the vault. Several agents can share one brain ([`Agent::with_brain`]), as
//! the trainer's parallel teachers do: each keeps its own conversation state,
//! all of them change knowledge through the brain's single lock.
//!
//! [`Agent::respond`] handles one user message: the message is translated
//! first (outside the lock, since that may call a model), then handled while
//! holding the lock, and every turn leaves a [`TurnTrace`].

use sla_core::kb::matches;
use sla_core::{Atom, AssertOutcome, Fact, GroundAtom, Literal, Rule, RuleFile, RuleMeta, RuleStatus, Term, Timestamp, Value};
use sla_learner::corrections::{self, Change, Demonstration};
use sla_learner::LearnReport;
use sla_nlg::{Act, Realizer, Reply, Truth};
use sla_parser::{Context, Intent, Parser, PatternParser, Social};
use sla_reasoner::{Model, RuleInput};
use std::io;
use std::path::PathBuf;
use std::sync::{Arc, MutexGuard};
use std::time::Instant;

pub mod brain;
pub mod curiosity;
pub mod dialogue;
pub mod events;
pub mod consolidate;
pub mod hygiene;
pub mod learning;
pub mod queue;
pub mod router;
pub mod tasks;
pub mod trace;

pub use brain::{Brain, SharedBrain};
use curiosity::Teacher;
pub use trace::{ChangeTrace, PartTrace, TurnTrace};

/// Something the agent asked and is waiting for an answer to.
#[derive(Debug, Clone, PartialEq)]
pub enum Pending {
    /// "Does Bob like coffee?" -> yes stores it, no stores the negation.
    Fact(GroundAtom),
    /// "What does Bob like?" -> the next statement is the answer.
    Wh(Atom),
    /// "Is it true that ...?" about a hypothesis rule (by rule file id).
    Rule(String),
    /// "Which one is true?" about two contradicting facts.
    Contradiction(GroundAtom, GroundAtom),
    /// An open question for a teacher ("How do I fix a tire? What is the first
    /// step?"): the answer is read as ordinary teaching.
    Open(String),
}

pub struct Agent {
    brain: SharedBrain,
    /// Used only for the oracle parser (gold acts by message); everything
    /// else goes through the translator.
    parser: Box<dyn Parser>,
    /// Understanding (Design 2.0): segmenter, pattern and model tiers, validator.
    pub translator: sla_translate::Translator,
    /// Things recently talked about, most recent first (translator context).
    pub recent: Vec<String>,
    /// The agent's last reply (translator context).
    pub last_reply: Option<String>,
    pub ctx: Context,
    pub realizer: Realizer,
    /// The atom the last answer was about (for "why?" and "that's wrong").
    pub last_answer: Option<GroundAtom>,
    pub pending: Option<Pending>,
    /// Fixed clock for tests and simulations; `None` uses the system clock.
    pub clock: Option<Timestamp>,
    /// Source label for facts from this conversation partner.
    pub source: String,
    /// Whether the last message (sentence) was understood.
    pub last_understood: bool,
    /// Last recall (entity, items shown) for "what else?".
    pub last_recall: Option<(String, usize)>,
    /// The fact count was already mentioned in this session.
    told_count: bool,
    small_talk_turn: usize,
    /// Add curiosity questions to replies while chatting.
    pub ask_in_chat: bool,
    /// What the learner did after the last message.
    pub last_learn: LearnReport,
    /// This conversation's id, used in traces.
    pub session_id: String,
    /// Turns handled so far.
    pub turns: usize,
    /// The trace of the last turn.
    pub last_trace: Option<TurnTrace>,
    /// Write each turn trace as a JSON line to `<dir>/<session>.jsonl`.
    pub trace_dir: Option<PathBuf>,
    /// Publish each turn trace on the brain's event stream.
    pub publish_events: bool,
    /// Trace being built for the current turn.
    cur: Option<TurnTrace>,
    /// Entities this turn added facts about (the learner's focus).
    told_about: std::collections::BTreeSet<Value>,
    /// The speaker's situation in this conversation (`inform`, `done`, `lack`):
    /// reasoned over with the vault, never written to it.
    pub session_facts: Vec<GroundAtom>,
    /// The task this conversation is about, and the step last given.
    pub active_task: Option<String>,
    pub last_step: Option<String>,
    /// The open task frame (a task schema from `vault/tasks/`) and the options already offered.
    pub frame: Option<String>,
    pub chosen: Vec<String>,
    /// The dialogue manager made a move this turn.
    dialogue_ran: bool,
    /// The tier of the part being handled (`pattern`, `model`, ...).
    cur_tier: String,
    /// Replies already given in this conversation (none is repeated word for word).
    said: std::collections::HashSet<String>,
    /// A correction this turn was stored as an exception only.
    exception_noted: bool,
    /// Everything the user said in this conversation, lowercased.
    heard_text: String,
    /// Items the agent asked the user for a replacement of ("what do you have
    /// that could do the job?").
    pub(crate) asked_lack: std::cell::RefCell<Vec<String>>,
}

fn session_name() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static N: AtomicU64 = AtomicU64::new(1);
    let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis()).unwrap_or(0);
    format!("s{t}-{}-{}", std::process::id(), N.fetch_add(1, Ordering::SeqCst))
}

impl Agent {
    /// Open (or create) a vault and start an agent talking to `speaker`.
    pub fn open(root: impl Into<PathBuf>, speaker: &str) -> io::Result<Self> {
        Ok(Agent::with_brain(Brain::shared(root)?, speaker))
    }

    /// A new conversation over a brain that may be shared with others.
    pub fn with_brain(brain: SharedBrain, speaker: &str) -> Self {
        let registry = sla_lexicon::Registry::from_vault(&brain::lock(&brain).kb.vault);
        let mut realizer = Realizer::new(speaker);
        for (name, info) in &registry.preds {
            if let Some(f) = info.frames.first() {
                realizer.frames.insert(name.clone(), f.clone());
            }
            realizer.arg_types.insert(name.clone(), info.types.iter().map(|t| t.name().to_string()).collect());
        }
        Agent {
            brain,
            parser: Box::new(PatternParser),
            translator: sla_translate::Translator::new(registry),
            recent: Vec::new(),
            last_reply: None,
            ctx: Context { speaker: speaker.to_string(), last_entity: None },
            realizer,
            last_answer: None,
            pending: None,
            clock: None,
            source: "user".into(),
            last_understood: true,
            last_recall: None,
            told_count: false,
            small_talk_turn: 0,
            ask_in_chat: true,
            last_learn: LearnReport::default(),
            session_id: session_name(),
            turns: 0,
            last_trace: None,
            trace_dir: None,
            publish_events: true,
            cur: None,
            told_about: Default::default(),
            session_facts: Vec::new(),
            active_task: None,
            last_step: None,
            frame: None,
            chosen: Vec::new(),
            dialogue_ran: false,
            exception_noted: false,
            heard_text: String::new(),
            asked_lack: Default::default(),
            cur_tier: String::new(),
            said: Default::default(),
        }
    }

    /// The shared brain (clone it to start another conversation on it).
    pub fn shared_brain(&self) -> SharedBrain {
        Arc::clone(&self.brain)
    }

    /// Lock the brain. Do not hold the guard across a call to `respond`.
    pub fn brain(&self) -> MutexGuard<'_, Brain> {
        brain::lock(&self.brain)
    }

    /// Swap the parser (for example an LLM translator or the oracle parser).
    pub fn set_parser(&mut self, p: Box<dyn Parser>) {
        self.parser = p;
    }

    /// Use the LLM translator (Claude CLI, headless) for messages the pattern
    /// parser does not understand. Translations are cached in the vault.
    pub fn enable_llm(&mut self, model: &str) {
        let cache = self.brain().kb.vault.root.join("translate_cache.jsonl");
        self.enable_llm_cached(model, cache);
    }

    /// The LLM translator with its cache in a given file (shared by runs).
    pub fn enable_llm_cached(&mut self, model: &str, cache: PathBuf) {
        let registry = std::sync::Arc::clone(&self.translator.registry);
        let patterns = self.brain().kb.vault.root.join("grammar").join("input_patterns.json");
        let mut t = sla_translate::Translator::new((*registry).clone()).with_cache(cache);
        t.model = Some(sla_translate::ModelTier::new(model));
        // Learned input patterns live in the vault and grow with every validated translation.
        t.learned = sla_translate::learned::LearnedPatterns::load(patterns);
        self.translator = t;
    }

    /// What the translator should know about this conversation.
    fn tcontext(&self) -> sla_translate::TContext {
        let b = self.brain();
        let mut known: Vec<String> = b.kb.vault.procedures.keys().chain(b.kb.vault.tasks.keys()).cloned().collect();
        known.sort();
        // What each procedure involves: its needs and the objects of its steps.
        let task_hints: Vec<(String, Vec<String>)> = b
            .kb
            .vault
            .procedures
            .iter()
            .map(|(k, pf)| {
                let mut h: Vec<String> = Vec::new();
                for f in pf.facts() {
                    if matches!(f.atom.pred.as_str(), "needs" | "object_of") {
                        if let Some(w) = f.atom.args.get(1).and_then(|v| v.as_sym()) {
                            if !h.iter().any(|x| x == w) && h.len() < 8 {
                                h.push(w.to_string());
                            }
                        }
                    }
                }
                (k.clone(), h)
            })
            .collect();
        let next_step = self.active_task.as_ref().map_or(1, |t| {
            b.kb.vault.procedures.get(t).map_or(1, |p| p.facts().iter().filter(|f| f.atom.pred == "step").count() + 1)
        });
        let mut recent = self.recent.clone();
        if let Some(e) = &self.ctx.last_entity {
            if !recent.contains(e) {
                recent.insert(0, e.clone());
            }
        }
        sla_translate::TContext { speaker: self.ctx.speaker.clone(), active_task: self.active_task.clone(), next_step, last_reply: self.last_reply.clone(), recent, known_tasks: known, task_hints }
    }

    pub fn parser_name(&self) -> &str {
        if self.parser.name() == "oracle" {
            "oracle"
        } else if self.translator.model.is_some() {
            "pattern+model"
        } else {
            "pattern"
        }
    }

    /// Translate a message ahead of `respond` (fills the translator's cache).
    /// `respond` translates before taking the lock anyway; this only lets a
    /// caller time translation separately.
    pub fn prepare(&mut self, text: &str) {
        let ctx = self.tcontext();
        let _ = self.translator.translate(text, &ctx);
    }

    pub fn now(&self) -> Timestamp {
        self.clock.unwrap_or_else(Timestamp::now)
    }

    pub fn reload_if_edited(&mut self) -> io::Result<bool> {
        self.brain().reload_if_edited()
    }

    pub fn save(&mut self) -> io::Result<()> {
        self.brain().save()
    }

    /// The model at the current time (`history`: every moment of the past merged,
    /// for inspection only; answers use the moment-by-moment models).
    pub fn model(&self, history: bool) -> Arc<Model> {
        let now = self.now();
        let mut b = self.brain();
        if !history {
            return b.model_at(now);
        }
        let past = b.all_past_models(now);
        let mut facts: Vec<sla_reasoner::FactInput> = Vec::new();
        for m in past.iter().chain(std::iter::once(&b.model_at(now))) {
            facts.extend(m.all_facts().into_iter().map(sla_reasoner::FactInput::new));
        }
        Arc::new(sla_reasoner::evaluate(&facts, &[]).unwrap_or_default())
    }

    /// Handle one user message and return the reply. A message with several
    /// sentences is handled sentence by sentence.
    pub fn respond(&mut self, text: &str) -> Reply {
        let started = Instant::now();
        // What the user said in this conversation (for wording: plurals they used).
        self.heard_text.push('\n');
        self.heard_text.push_str(&text.to_lowercase());
        let calls_before = sla_parser::claude::thread_calls();
        // Translation happens before the vault lock: it may call a model.
        // Oracle: gold acts for the whole message. Otherwise the translator
        // (segmenter, tiers, validator), always before the vault lock.
        let oracle = if self.parser.name() == "oracle" { Some(self.parser.parse_many(text, &self.ctx)) } else { None };
        let translation = if oracle.is_none() {
            let ctx = self.tcontext();
            Some(self.translator.translate(text, &ctx))
        } else {
            None
        };
        let translate_ms = started.elapsed().as_secs_f64() * 1000.0;
        let brain = Arc::clone(&self.brain);
        let mut b = brain::lock(&brain);
        let think_started = Instant::now();
        let evals_before = b.evaluations;
        let (eval_ms_before, learn_ms_before) = (b.eval_ms, b.learn_ms);
        let mark = b.kb.journal_len();
        self.turns += 1;
        self.cur = Some(TurnTrace {
            session: self.session_id.clone(),
            turn: self.turns,
            at: self.now().to_string(),
            speaker: self.ctx.speaker.clone(),
            source: self.source.clone(),
            message: text.to_string(),
            parser: self.parser.name().to_string(),
            ..Default::default()
        });
        let _ = b.reload_if_edited();
        self.told_about.clear();
        self.dialogue_ran = false;
        let mut reply = match (oracle, translation) {
            (Some(intents), _) => self.respond_intents(&mut b, text, intents, "oracle"),
            (None, Some(tr)) => self.respond_parts(&mut b, text, tr),
            (None, None) => self.respond_sentences(&mut b, text),
        };
        // No reply is repeated word for word in one conversation (the same
        // failure keeps its one wording, which the design also asks).
        if self.said.contains(&reply.text) && reply.text != "Sorry, I didn't understand that." {
            reply.text = self.unrepeat(&reply);
        }
        self.said.insert(reply.text.clone());
        // Failures feed the curiosity goals (questions for teachers).
        crate::curiosity::record_failure(&b.kb, text, &reply.text, &self.source);
        // Resume: a turn that left the open task aside returns to its question.
        if !self.dialogue_ran && !reply.text.trim_end().ends_with('?') {
            if let Some(q) = self.resume_question(&mut b) {
                reply.text = format!("{} {q}", reply.text);
                self.trace_move("resume");
            }
        }
        // Learning: one cheap pass per message, around what it told about.
        if !self.told_about.is_empty() {
            let now = self.now();
            let t = Instant::now();
            let features = b.learner_features(now);
            let learner = b.learner.clone();
            let focus = std::mem::take(&mut self.told_about);
            self.last_learn = learner.update_turn(&mut b.kb, now, &features, &focus);
            b.learn_ms += t.elapsed().as_secs_f64() * 1000.0;
        }
        // At most one question per turn, about what the turn talked about.
        if self.ask_in_chat && self.pending.is_none() && reply.intent != "ask" {
            let topic: Vec<String> = reply
                .claims
                .iter()
                .flat_map(|c| c.args.iter().filter_map(|v| v.as_sym().map(String::from)))
                .chain(self.ctx.last_entity.clone())
                .filter(|e| e != &self.ctx.speaker)
                .collect();
            let cap = b.curiosity.chat_cap;
            if let Some((q, key)) = (!topic.is_empty()).then(|| self.next_question_about(&mut b, cap, "ask", Some(&topic))).flatten() {
                reply.text = format!("{} {q}", reply.text);
                b.kb.log_change(self.now(), "ask", &key, "curiosity while chatting");
                if let Some(t) = self.cur.as_mut() {
                    t.question = Some(key);
                }
            }
        }
        let mut t = self.cur.take().unwrap_or_default();
        t.reply = reply.text.clone();
        t.claims = reply.claims.iter().map(|c| c.to_string()).collect();
        t.changes = b.kb.journal_since(mark).into_iter().map(|e| ChangeTrace { action: e.action, subject: e.subject, reason: e.reason, fact: e.fact }).filter(|c| c.action != "turn").collect();
        t.understood = t.parts.iter().any(|p| p.understood);
        t.translate_ms = translate_ms;
        t.think_ms = think_started.elapsed().as_secs_f64() * 1000.0;
        t.model_calls = sla_parser::claude::thread_calls() - calls_before;
        t.evaluations = b.evaluations - evals_before;
        t.eval_ms = b.eval_ms - eval_ms_before;
        t.learn_ms = b.learn_ms - learn_ms_before;
        if let Err(e) = b.save() {
            t.errors.push(format!("save failed: {e}"));
        }
        if self.publish_events {
            b.events.publish("turn", &t);
        }
        drop(b);
        if let Some(dir) = &self.trace_dir {
            if let Err(e) = write_trace(dir, &t) {
                eprintln!("warning: cannot write turn trace: {e}");
            }
        }
        self.last_trace = Some(t);
        self.last_reply = Some(reply.text.clone());
        reply
    }

    fn trace_part(&mut self, text: &str, intents: &[Intent], tier: &str) {
        let understood = !matches!(intents, [Intent::Unknown] | []);
        if let Some(t) = self.cur.as_mut() {
            t.parts.push(PartTrace { text: text.to_string(), acts: intents.iter().map(|i| i.canonical()).collect(), tier: tier.into(), understood });
        }
    }

    fn trace_move(&mut self, m: &str) {
        if let Some(t) = self.cur.as_mut() {
            t.moves.push(m.to_string());
        }
    }

    /// Handle a translated message part by part: every understood part gets
    /// its reply; a part that was not understood gets one question, or the
    /// "not understood" reply when nothing was understood (Design 2.0:
    /// "No message is lost as a whole").
    fn respond_parts(&mut self, b: &mut Brain, text: &str, tr: sla_translate::Translation) -> Reply {
        let mut texts: Vec<String> = Vec::new();
        let mut claims = Vec::new();
        let mut intent = "social";
        let mut lost: Vec<String> = Vec::new();
        let n = tr.parts.len();
        let mut dlg: Vec<Intent> = Vec::new();
        let mut dlg_at: Option<usize> = None;
        for p in tr.parts {
            if !p.understood() {
                // An honest `unknown` (a filler sentence) needs no question.
                if !p.ok {
                    lost.push(p.text.clone());
                }
                self.trace_part(&p.text, &p.acts, p.tier.name());
                if let Some(t) = self.cur.as_mut() {
                    if let Some(last) = t.parts.last_mut() {
                        last.understood = false;
                    }
                }
                continue;
            }
            // Dialogue acts of the whole message are handled together, once.
            let (d, rest): (Vec<Intent>, Vec<Intent>) = p.acts.into_iter().partition(dialogue::is_dialogue_act);
            if rest.is_empty() {
                self.learn_names(b, &p.text);
                self.trace_part(&p.text, &d, p.tier.name());
                if dlg_at.is_none() {
                    dlg_at = Some(texts.len());
                }
                dlg.extend(d);
                continue;
            }
            // Greetings and thanks next to content add nothing.
            let r = self.respond_intents(b, &p.text, rest, p.tier.name());
            if !d.is_empty() {
                if dlg_at.is_none() {
                    dlg_at = Some(texts.len() + 1);
                }
                dlg.extend(d);
            }
            if n > 1 && r.intent == "social" && !texts.is_empty() {
                continue;
            }
            if r.intent != "social" || texts.is_empty() {
                intent = r.intent;
            }
            claims.extend(r.claims);
            if !texts.contains(&r.text) {
                texts.push(r.text);
            }
        }
        if !dlg.is_empty() {
            let r = self.dialogue(b, dlg);
            let at = dlg_at.unwrap_or(texts.len()).min(texts.len());
            if r.intent != "social" || texts.is_empty() {
                intent = r.intent;
            } else if intent == "social" {
                intent = r.intent;
            }
            claims.extend(r.claims);
            if !texts.contains(&r.text) {
                texts.insert(at, r.text);
            }
        }
        // With a task answered, side answers that only say "I don't know whether ..."
        // are dropped: they bury the answer and any question at its end.
        if self.dialogue_ran && texts.len() > 1 {
            const SIDE: &[&str] = &["I don't know whether", "I'm not sure whether", "I have no idea whether", "I don't know what", "I'm not sure what", "I have no idea what"];
            texts.retain(|t| !SIDE.iter().any(|p| t.starts_with(p)));
            // Nor a check-back of the request itself ("So you want to make pancakes, right?").
            if texts.len() > 1 {
                texts.retain(|t| !(t.starts_with("So ") && t.ends_with(", right?")));
            }
        }
        // A social reply to the opening greeting is dropped when content follows.
        if texts.len() > 1 && matches!(texts[0].as_str(), "Hello!" | "Hi there!" | "Hello, nice to talk to you.") {
            texts.remove(0);
        }
        self.last_understood = !texts.is_empty();
        if !lost.is_empty() {
            let shown: String = lost[0].chars().take(80).collect();
            if texts.is_empty() {
                let r = self.say(Act::NotUnderstood);
                return r;
            }
            // A stray phrase next to a request already answered ("so I can't make
            // holes") is context, not a question to ask back.
            if !self.dialogue_ran {
                texts.push(format!("I didn't understand \"{}\": what do you mean?", shown.trim_end_matches(['.', '!', '?'])));
                intent = "clarify";
            }
        }
        if texts.is_empty() {
            return self.say(Act::NotUnderstood);
        }
        let _ = text;
        Reply { text: texts.join(" "), intent, claims }
    }

    /// The pattern parser: each sentence is parsed and handled in turn, so
    /// "he" in the second sentence can refer to the first.
    fn respond_sentences(&mut self, b: &mut Brain, text: &str) -> Reply {
        let sentences = split_sentences(text);
        if sentences.len() <= 1 {
            let intents = self.parser.parse_many(text, &self.ctx);
            return self.respond_intents(b, text, intents, "pattern");
        }
        let mut texts: Vec<String> = Vec::new();
        let mut claims = Vec::new();
        let mut intent = "social";
        let mut not_understood = 0;
        for s in &sentences {
            let intents = self.parser.parse_many(s, &self.ctx);
            let r = self.respond_intents(b, s, intents, "pattern");
            if !self.last_understood {
                not_understood += 1;
                continue;
            }
            if r.intent != "social" || texts.is_empty() {
                intent = r.intent;
            }
            claims.extend(r.claims);
            if !texts.contains(&r.text) {
                texts.push(r.text);
            }
        }
        if not_understood > 0 {
            texts.push(if texts.is_empty() {
                "Sorry, I didn't understand that. Short sentences like \"Anna likes tea.\" work best.".to_string()
            } else {
                format!("I didn't understand {} of your sentences.", if not_understood == 1 { "one".to_string() } else { not_understood.to_string() })
            });
        }
        Reply { text: texts.join(" "), intent, claims }
    }

    fn respond_intents(&mut self, b: &mut Brain, text: &str, intents: Vec<Intent>, tier: &str) -> Reply {
        self.cur_tier = tier.to_string();
        let n_dialogue = intents.iter().filter(|i| dialogue::is_dialogue_act(i)).count();
        if n_dialogue > 0 && n_dialogue < intents.len() {
            // Mixed: the other acts first, then one dialogue move.
            let (d, rest): (Vec<Intent>, Vec<Intent>) = intents.into_iter().partition(dialogue::is_dialogue_act);
            let r1 = self.respond_intents(b, text, rest, tier);
            let r2 = self.dialogue(b, d);
            let social_first = r1.intent == "social" && matches!(r1.text.as_str(), "Hello!" | "Hi there!" | "Hello, nice to talk to you.");
            let text_out = if social_first { r2.text.clone() } else { format!("{} {}", r1.text, r2.text) };
            let mut claims = r1.claims;
            claims.extend(r2.claims);
            return Reply { text: text_out, intent: r2.intent, claims };
        }
        if n_dialogue > 1 {
            self.learn_names(b, text);
            self.trace_part(text, &intents, tier);
            self.last_understood = true;
            let r = self.dialogue(b, intents);
            b.kb.log_change(self.now(), "turn", text, &r.text);
            return r;
        }
        self.learn_names(b, text);
        self.trace_part(text, &intents, tier);
        self.last_understood = !matches!(intents.as_slice(), [Intent::Unknown] | []);
        let mut intents = intents;
        if intents.is_empty() {
            intents.push(Intent::Unknown);
        }
        {
            let mut seen: Vec<String> = Vec::new();
            intents.retain(|i| {
                let c = i.canonical();
                if seen.contains(&c) {
                    false
                } else {
                    seen.push(c);
                    true
                }
            });
        }
        // Facts from one message get one acknowledgement: merge the plain tells.
        if intents.len() > 1 {
            let mut merged: Vec<Intent> = Vec::new();
            for i in intents {
                match (merged.last_mut(), i) {
                    (Some(Intent::Tell { facts, rules, past: false, hedge: None }), Intent::Tell { facts: f2, rules: r2, past: false, hedge: None }) => {
                        facts.extend(f2);
                        rules.extend(r2);
                    }
                    (_, i) => merged.push(i),
                }
            }
            intents = merged;
        }
        if intents.len() > 1 {
            // Several intents from one sentence (LLM translator): handle each, join the replies.
            let mut texts: Vec<String> = Vec::new();
            let mut claims = Vec::new();
            let mut intent_name = "social";
            // Reactions and small talk only matter when the message had nothing else.
            let has_content = intents.iter().any(|i| !matches!(i, Intent::Social(_) | Intent::Greet | Intent::Unknown));
            for i in intents {
                // Greetings, reactions and stray yes/no are noise next to content
                // (a yes/no still counts when the agent asked something).
                let stray_yes_no = matches!(i, Intent::Yes | Intent::No) && self.pending.is_none();
                if has_content && (matches!(i, Intent::Social(Social::Reaction) | Intent::Unknown | Intent::Greet) || stray_yes_no) {
                    continue;
                }
                let r = self.respond_intent(b, i);
                if r.intent != "social" {
                    intent_name = r.intent;
                }
                claims.extend(r.claims);
                if !texts.contains(&r.text) {
                    texts.push(r.text);
                }
            }
            let reply = Reply { text: texts.join(" "), intent: intent_name, claims };
            // One log entry per message, with the whole reply.
            b.kb.log_change(self.now(), "turn", text, &reply.text);
            return reply;
        }
        let intent = intents.into_iter().next().unwrap_or(Intent::Unknown);
        let reply = self.respond_intent(b, intent);
        b.kb.log_change(self.now(), "turn", text, &reply.text);
        reply
    }

    /// Capitalized words inside a sentence are proper names ("in Berlin",
    /// "Is Rex a dog?"). They are stored as `proper_name(x)` facts so replies
    /// can capitalize them after a restart too.
    fn learn_names(&mut self, b: &mut Brain, text: &str) {
        let now = self.now();
        for name in proper_names(text) {
            let atom = GroundAtom::new("proper_name", vec![Value::sym(name.as_str())]);
            if !b.has_stored(&atom) {
                b.kb.vault.add_fact(Fact::new(atom, "parser").with_from(now));
            }
        }
    }

    /// Another wording for a reply already given in this conversation.
    fn unrepeat(&self, r: &Reply) -> String {
        if r.intent == "acknowledge" {
            for lead in ["Got it.", "Noted.", "Okay, I'll remember that.", "Thanks, noted.", "Good to know.", "I'll keep that in mind.", "All right."] {
                if !self.said.contains(lead) {
                    return lead.to_string();
                }
            }
            // A long lesson: name what was learned, so no two replies are the same.
            if !r.claims.is_empty() {
                let body: Vec<String> = r.claims.iter().take(3).map(|c| self.realizer.clause(c)).collect();
                let mut text = format!("Got it: {}.", body.join(", and "));
                if let Some(c) = text.get(8..9) {
                    text = format!("Got it: {}{}", c.to_uppercase(), &text[9..]);
                }
                return text;
            }
        }
        // Keep the reply's own words (its tense and hedges are right); a
        // yes/no answer still leads with its answer.
        for lead in ["Yes", "No"] {
            if let Some(rest) = r.text.strip_prefix(lead) {
                return format!("{lead}, as I said{rest}");
            }
        }
        format!("As I said: {}", r.text)
    }

    /// Names for the realizer: the brain's, plus the speaker; steps are
    /// written as what they do ("the step to lift the car").
    fn refresh_steps(&mut self, b: &mut Brain) {
        let m = b.model_at(self.now());
        for st in m.facts_of("step") {
            let Some(s) = st.args.get(1).and_then(|v| v.as_sym()) else { continue };
            if self.realizer.surface.contains_key(s) {
                continue;
            }
            let action = m.facts_of("action").into_iter().find(|a| a.args.first().and_then(|v| v.as_sym()) == Some(s)).and_then(|a| a.args.get(1).and_then(|v| v.as_sym()).map(String::from));
            let object = m.facts_of("object_of").into_iter().find(|a| a.args.first().and_then(|v| v.as_sym()) == Some(s)).and_then(|a| a.args.get(1).and_then(|v| v.as_sym()).map(|o| o.replace('_', " ")));
            if let Some(a) = action {
                let text = match object {
                    Some(o) => format!("the step to {a} the {o}"),
                    None => format!("the step to {a}"),
                };
                self.realizer.surface.insert(s.to_string(), text);
            }
        }
    }

    /// Names for the realizer: the brain's, plus the speaker.
    fn refresh_names(&mut self, b: &mut Brain) {
        self.refresh_steps(b);
        let names = b.names();
        if self.realizer.names.len() != names.len() + 1 || !self.realizer.names.contains(&self.ctx.speaker) {
            let mut n: std::collections::HashSet<String> = (*names).clone();
            n.insert(self.ctx.speaker.clone());
            self.realizer.names = n;
        } else if !names.iter().all(|x| self.realizer.names.contains(x)) {
            let mut n: std::collections::HashSet<String> = (*names).clone();
            n.insert(self.ctx.speaker.clone());
            self.realizer.names = n;
        }
    }

    fn respond_intent(&mut self, b: &mut Brain, intent: Intent) -> Reply {
        self.refresh_names(b);
        // Remember who we talk about, for "he", "she", "it", "they" next time.
        let subject = match &intent {
            // "My mom is Olga": the person named is who "she" will mean.
            Intent::Tell { facts, .. } => facts.first().and_then(|a| {
                let owner_is_me = a.args.first() == Some(&Term::Val(Value::sym(self.ctx.speaker.as_str())));
                if a.pred == "property" && a.args.len() == 3 && owner_is_me {
                    a.args.get(2).cloned()
                } else {
                    a.args.first().cloned()
                }
            }),
            Intent::Ask { query, .. } => query.iter().find_map(|l| match l {
                Literal::Pos(a) => a.args.first().cloned(),
                _ => None,
            }),
            Intent::Recall(e) => Some(Term::Val(Value::sym(e.as_str()))),
            Intent::Correct(v) => v.first().and_then(|a| a.args.first().cloned()),
            _ => None,
        };
        if let Some(Term::Val(Value::Sym(e))) = subject {
            if e != self.ctx.speaker {
                self.recent.retain(|r| r != &e);
                self.recent.insert(0, e.clone());
                self.recent.truncate(8);
                self.ctx.last_entity = Some(e);
            }
        }
        let was_pending = self.pending.is_some();
        let gen_before = b.kb.vault.generation;
        let facts_before = b.kb.journal_len();
        let reply = self.handle(b, intent);
        self.trace_move(reply.intent);
        self.refresh_names(b);
        // Learning progress lowers curiosity; answering its question lowers it most.
        let learned = b.kb.vault.generation != gen_before && b.kb.journal_since(facts_before).iter().any(|e| e.action == "add" || e.action == "add_rule");
        if learned {
            b.curiosity.reward(if was_pending { 0.3 } else { 0.05 });
        }
        reply
    }

    /// Questions logged today under `action`.
    fn asked_today(&self, b: &Brain, action: &str) -> usize {
        let today = self.now().to_string()[..10].to_string();
        b.kb.vault.read_log().map(|l| l.iter().filter(|(t, a, _, _)| a == action && t.starts_with(&today)).count()).unwrap_or(0)
    }

    /// Pick the best curiosity goal (if the drive is high enough and the daily
    /// cap allows), plan it into a question and set it as pending.
    /// Returns (question text, goal key).
    fn next_question(&mut self, b: &mut Brain, cap: usize, action: &str) -> Option<(String, String)> {
        self.next_question_about(b, cap, action, None)
    }

    /// Like `next_question`, but if `topic` is given, only goals mentioning one
    /// of those entities count (chat: stay on what is being talked about).
    fn next_question_about(&mut self, b: &mut Brain, cap: usize, action: &str, topic: Option<&[String]>) -> Option<(String, String)> {
        let m = b.model_at(self.now());
        let mut goals = b.curiosity.goals(&b.kb, &m);
        if let Some(topic) = topic {
            // In a chat the agent asks only about the thing under discussion;
            // task gaps, unknown tasks and old misunderstandings go to teachers.
            goals.retain(|g| !matches!(g.goal, curiosity::Goal::UnknownTask { .. } | curiosity::Goal::TaskGap { .. } | curiosity::Goal::Unclear { .. }));
            goals.retain(|g| {
                let key = g.goal.key();
                let words: Vec<&str> = key.split(|c: char| !(c.is_alphanumeric() || c == '_')).collect();
                topic.iter().any(|e| words.contains(&e.as_str()))
            });
        }
        b.curiosity.update(&goals);
        if !b.curiosity.wants_to_ask(&goals) || self.asked_today(b, action) >= cap {
            return None;
        }
        let g = goals.into_iter().next()?;
        let key = g.goal.key();
        let (act, pending) = curiosity::plan(&g.goal);
        b.curiosity.asked.insert(key.clone());
        self.pending = Some(pending);
        Some((self.say(act).text, key))
    }

    /// Ask the teacher the most useful question and learn from the answer.
    /// Returns (question, answer, the agent's reply), or `None` if nothing to ask.
    /// The brain is not locked while the teacher answers.
    pub fn ask_teacher(&mut self, teacher: &mut dyn Teacher) -> Option<(String, String, Reply)> {
        let (question, key) = {
            let brain = Arc::clone(&self.brain);
            let mut b = brain::lock(&brain);
            let saved = b.curiosity.level;
            b.curiosity.level = 1.0; // asked explicitly: ignore the drive level
            let q = self.next_question(&mut b, usize::MAX, "ask");
            if q.is_none() {
                b.curiosity.level = saved;
            }
            let (question, key) = q?;
            b.kb.log_change(self.now(), "ask", &key, "question for the teacher");
            let _ = b.save();
            (question, key)
        };
        let _ = key;
        let answer = teacher.answer(&question);
        let was_ask = self.ask_in_chat;
        self.ask_in_chat = false;
        let reply = self.respond(&answer);
        self.ask_in_chat = was_ask;
        Some((question, answer, reply))
    }

    /// A message the agent starts by itself, at most `proactive_cap` per day.
    pub fn proactive(&mut self) -> Option<Reply> {
        let brain = Arc::clone(&self.brain);
        let mut b = brain::lock(&brain);
        let cap = b.curiosity.proactive_cap;
        let (q, key) = self.next_question(&mut b, cap, "proactive")?;
        b.kb.log_change(self.now(), "proactive", &key, "curiosity");
        let _ = b.save();
        Some(Reply { text: q, intent: "ask", claims: Vec::new() })
    }

    fn say(&self, act: Act) -> Reply {
        self.realizer.realize(&act)
    }

    fn handle(&mut self, b: &mut Brain, intent: Intent) -> Reply {
        let pending = self.pending.take();
        match intent {
            Intent::Tell { facts, rules, past, hedge } => {
                // The answer to "which one is true?" settles the contradiction:
                // the side the user states is kept, the other is made false.
                if let Some(Pending::Contradiction(a, bb)) = &pending {
                    if facts.iter().any(|f| f.to_ground().map_or(false, |g| &g == a || &g == bb)) {
                        return self.correct(b, facts);
                    }
                }
                // An answer to "What does Bob like?" arrives as a statement.
                self.tell(b, facts, rules, past, hedge.as_deref())
            }
            Intent::Ask { query, history } => self.ask(b, query, history),
            Intent::Recall(e) => self.recall(b, e, 0),
            Intent::RecallMore(target) => {
                let last = self.last_recall.clone();
                match (target, last) {
                    (Some(e), Some((le, shown))) if e == le => self.recall(b, e, shown),
                    (Some(e), _) => self.recall(b, e, 0),
                    (None, Some((le, shown))) => self.recall(b, le, shown),
                    (None, None) => match self.ctx.last_entity.clone() {
                        Some(e) => self.recall(b, e, 0),
                        None => self.say(Act::Note("What would you like to know more about?".into())),
                    },
                }
            }
            Intent::Social(Social::DontKnow) => self.say(Act::Note(if pending.is_some() { "Okay, never mind.".into() } else { "That's okay.".into() })),
            Intent::Social(kind) => self.social(b, kind),
            Intent::Why(target) => {
                let atom = match target.and_then(|a| a.to_ground()) {
                    Some(a) => Some(a),
                    None => self.last_answer.clone(),
                };
                match atom {
                    None => self.say(Act::Note("Why what? Ask me something first.".into())),
                    Some(a) => {
                        let m = if self.session_facts.is_empty() { b.model_at(self.now()) } else { b.model_with(self.now(), &self.session_facts.clone()) };
                        // "Why do I need a jack?": the need may come from a step of the task under way.
                        let a = if m.proof(&a).is_none() && a.base_pred() == "needs" && a.args.len() == 2 {
                            let item = a.args[1].clone();
                            let task = a.args[0].as_sym().map(String::from);
                            m.facts_of("needs_all")
                                .into_iter()
                                .filter(|x| x.args[1] == item)
                                .find(|x| task.as_deref().map_or(true, |t| x.args[0].as_sym() == Some(t)) || self.active_task.as_deref() == x.args[0].as_sym())
                                .unwrap_or(a)
                        } else {
                            a
                        };
                        match m.proof(&a) {
                            Some(p) => {
                                self.record_uses(b, &m, &p.leaves());
                                self.last_answer = Some(a);
                                self.say(Act::Explain(p))
                            }
                            None => self.say(Act::ExplainUnknown(a)),
                        }
                    }
                }
            }
            Intent::Correct(atoms) => self.correct(b, atoms),
            Intent::Yes | Intent::No => self.answer_pending(b, pending, matches!(intent, Intent::Yes)),
            Intent::Greet => self.say(Act::Greet),
            Intent::HowAreYou => {
                // Honest small talk: report its own state instead of pretending feelings.
                // The fact count is said once per session; after that, short replies.
                let n = self.current_fact_count(b);
                self.small_talk_turn += 1;
                let text = if n == 0 {
                    "I'm fine, thanks, and ready to learn. I don't know anything yet: tell me something, like \"Anna likes tea.\"".to_string()
                } else if !self.told_count {
                    self.told_count = true;
                    format!("I'm fine, thanks. I know {n} fact{} so far. What would you like to talk about?", if n == 1 { "" } else { "s" })
                } else {
                    ["Good, thanks! And you?", "I'm well. What's on your mind?", "Fine, thanks. Tell me something new!"][self.small_talk_turn % 3].to_string()
                };
                self.say(Act::Note(text))
            }
            Intent::AboutYou(thing) => self.about_me(b, thing),
            i @ (Intent::Inform(_)
            | Intent::Request(_)
            | Intent::AskHow(_)
            | Intent::AskNeed(_)
            | Intent::AskDuration(_)
            | Intent::AskNext
            | Intent::Done(_)
            | Intent::Lack(_)
            | Intent::Accept
            | Intent::Reject) => self.dialogue(b, vec![i]),
            Intent::Feedback(positive) => self.feedback(positive),
            Intent::Teach { facts, rules } => self.tell(b, facts, rules, false, None),
            Intent::NewPred { atom, definition } => self.new_pred(b, atom, definition),
            Intent::Bye => self.say(Act::Bye),
            // A non-answer to the agent's own question: let it go.
            Intent::Unknown => self.say(Act::NotUnderstood),
        }
    }

    fn current_fact_count(&self, b: &Brain) -> usize {
        let now = self.now();
        b.kb.all_facts().iter().filter(|f| f.atom.pred != "proper_name" && f.valid_at(now)).count()
    }

    /// Everything known about `e`, `RECALL_PAGE` items at a time from `offset`.
    /// Facts about `e` come first (kinds, parts, qualities, then the rest),
    /// then things of kind `e` ("penguins and robins are birds").
    fn recall(&mut self, b: &mut Brain, e: String, offset: usize) -> Reply {
        const RECALL_PAGE: usize = 8;
        let m = b.model_at(self.now());
        let rank = |p: &str| match p {
            "is_a" => 0,
            "has" | "part_of" => 1,
            "property" | "feels" => 2,
            "located_in" => 3,
            _ => 4,
        };
        let mut about: Vec<GroundAtom> = m
            .all_facts()
            .into_iter()
            .filter(|a| a.args.first().and_then(|v| v.as_sym()) == Some(e.as_str()))
            .filter(|a| !matches!(a.base_pred(), "thing" | "same" | "proper_name"))
            .collect();
        about.sort_by_key(|a| (rank(a.base_pred()), a.to_string()));
        let mut instances: Vec<GroundAtom> = m
            .facts_of("is_a")
            .into_iter()
            .filter(|a| a.args.get(1).and_then(|v| v.as_sym()) == Some(e.as_str()) && a.args[0] != a.args[1])
            .collect();
        instances.sort();
        // One list: facts first, then instances; page through it.
        let total = about.len() + instances.len();
        if offset >= total && total > 0 {
            self.last_recall = Some((e.clone(), offset));
            return self.say(Act::RecallDone(e));
        }
        let end = (offset + RECALL_PAGE).min(total);
        let facts: Vec<GroundAtom> = about.iter().skip(offset).take(end.saturating_sub(offset)).cloned().collect();
        let inst_from = offset.saturating_sub(about.len());
        let inst_take = end.saturating_sub(about.len().max(offset));
        let inst: Vec<GroundAtom> = instances.iter().skip(inst_from).take(inst_take).cloned().collect();
        self.record_uses(b, &m, &facts);
        self.last_recall = Some((e.clone(), end));
        self.last_answer = facts.first().or(inst.first()).cloned();
        let mut reply = self.say(Act::Recall { entity: e, facts, instances: inst, more: offset > 0 });
        if end < total {
            reply.text.push_str(" Ask me \"what else?\" for more.");
        }
        reply
    }

    /// A personal question to the agent: it has no body, taste or experiences,
    /// so it says so plainly and offers what it knows about the thing instead.
    fn about_me(&mut self, b: &mut Brain, thing: Option<String>) -> Reply {
        self.small_talk_turn += 1;
        let k = self.small_talk_turn;
        let Some(e) = thing else {
            let text = ["I don't really do things myself, I only learn and think. Tell me about it!", "I'm a learning program, so I don't have days like you do. What about you?", "Nothing much happens to me; I just learn from talks like this one. What's new with you?"][k % 3];
            return self.say(Act::Note(text.into()));
        };
        let m = b.model_at(self.now());
        let mut facts: Vec<GroundAtom> = m
            .all_facts()
            .into_iter()
            .filter(|a| a.args.first().and_then(|v| v.as_sym()) == Some(e.as_str()))
            .filter(|a| !matches!(a.base_pred(), "thing" | "same" | "proper_name"))
            .collect();
        facts.sort_by_key(|a| (a.base_pred() != "is_a", a.to_string()));
        facts.truncate(3);
        let name = e.replace('_', " ");
        let lead = ["I can't taste or try things myself", "I don't have likes of my own", "I've never experienced that myself"][k % 3];
        if facts.is_empty() {
            self.ctx.last_entity = Some(e.clone());
            return self.say(Act::Note(format!("{lead}, and I don't know much about {name} yet. What is it like?")));
        }
        self.last_answer = facts.first().cloned();
        let mut reply = self.say(Act::Recall { entity: e, facts, instances: Vec::new(), more: false });
        // "About soup: Soup is a food." -> "I can't taste..., but I know this about soup: ..."
        let body = reply.text.splitn(2, ':').nth(1).unwrap_or(&reply.text).trim().to_string();
        reply.text = format!("{lead}, but here is what I know about {name}: {body}");
        reply
    }

    /// Friendly small talk, honest about what the agent is.
    fn social(&mut self, b: &mut Brain, kind: Social) -> Reply {
        let facts = self.current_fact_count(b);
        let rules = b.kb.rules(false).iter().filter(|r| r.module != "core").count();
        self.small_talk_turn += 1;
        let k = self.small_talk_turn;
        let text = match kind {
            Social::Thanks => ["You're welcome!", "Glad I could help.", "Any time!"][k % 3].to_string(),
            Social::Reaction => [
                "Yes! Tell me something new, or ask me a question.",
                "I think so too. What else would you like to talk about?",
                "Right. What else can you tell me?",
            ][k % 3]
                .to_string(),
            Social::WhoAreYou => {
                // From the self layer, not a fixed string.
                let m = b.model_at(self.now());
                let mut can: Vec<String> = m.facts_of("can").iter().filter_map(|a| a.args.first().and_then(|v| v.as_sym()).map(|s| s.replace('_', " ").replace(" i ", " I "))).collect();
                can.sort();
                let list = match can.len() {
                    0 => String::new(),
                    1 => can[0].clone(),
                    n => format!("{} and {}", can[..n - 1].join(", "), can[n - 1]),
                };
                format!("I'm a small learning agent. I can {list}. So far I know {facts} facts and {rules} rules.")
            }
            Social::WhatCanYouDo => "You can teach me facts (\"Dogs are animals.\"), ask me questions (\"What is a dog?\", \"What do you know about birds?\"), \
                 ask \"Why?\" after an answer, and correct me (\"No, a whale is a mammal.\")."
                .to_string(),
            Social::DontKnow => "That's okay.".to_string(),
            Social::AndYou => {
                if self.told_count {
                    ["I'm good, thanks!", "Doing well, thanks for asking.", "All good here. I'm enjoying learning."][k % 3].to_string()
                } else {
                    self.told_count = true;
                    format!("I'm doing well, thanks. I've learned {facts} facts so far, and I'm always happy to learn more.")
                }
            }
        };
        self.say(Act::Note(text))
    }

    fn tell(&mut self, b: &mut Brain, facts: Vec<Atom>, rules: Vec<Rule>, past: bool, hedge: Option<&str>) -> Reply {
        // A hedge is kept as a strength: "usually" is believed less than "always".
        let strength = match hedge {
            Some("usually") => 0.8,
            Some("often") => 0.7,
            Some("sometimes") => 0.4,
            _ => 1.0,
        };
        let now = self.now();
        let mut added = Vec::new();
        let mut known = Vec::new();
        let mut closed = Vec::new();
        // Procedure knowledge goes to the task's procedure file.
        let facts = if past {
            facts
        } else {
            let (learned, rest) = self.learn_procedures(b, facts, strength);
            added.extend(learned);
            rest
        };
        for a in facts {
            let Some(g) = a.to_ground() else { continue };
            let fact = Fact::new(g.clone(), &self.source).with_conv(&self.session_id).with_conf(strength);
            if past {
                // "I used to like tea": true before, not now. A current copy is closed.
                let reason = format!("{} said it used to be true", self.source);
                b.kb.assert_past(fact, now, &reason);
                added.push(g);
                continue;
            }
            match b.kb.assert_fact(fact, now, &format!("{} said so", self.source)) {
                AssertOutcome::Added { closed: c } => {
                    added.push(g);
                    closed.extend(c.into_iter().map(|f| f.atom));
                }
                AssertOutcome::Confirmed { .. } => known.push(g),
            }
        }
        let mut new_rules = Vec::new();
        for r in rules {
            let source = self.source.clone();
            match add_rule_to(b, r.clone(), RuleStatus::Confirmed, 0.9, &source, "stated by the user", now) {
                Ok(true) => new_rules.push(r),
                Ok(false) => {}
                Err(e) => return self.say(Act::Note(format!("I can't use that rule: {e}."))),
            }
        }
        if let Some(a) = added.first().or(known.first()) {
            self.last_answer = Some(a.clone());
        }
        // New facts may support, contradict or suggest learned rules: the
        // learner looks at them once, at the end of the message.
        for g in &added {
            if let Some(e) = g.args.first() {
                self.told_about.insert(e.clone());
            }
        }
        // A lasting fact from a weak (model) parse is said back as a check (Design 2.0).
        if self.cur_tier == "model" && added.len() == 1 && closed.is_empty() && new_rules.is_empty() && !past {
            let g = added[0].clone();
            self.pending = Some(Pending::Fact(g.clone()));
            return Reply { text: self.realizer.confirm(&[g.clone()]), intent: "confirm", claims: vec![g] };
        }
        self.say(Act::Acknowledge { added, closed, known, rules: new_rules, past })
    }

    /// A guess from a hypothesis rule: "I'm not sure, but I'd guess so ... Is that right?"
    fn tentative_answer(&mut self, b: &mut Brain, g: &GroundAtom, now: Timestamp) -> Option<Reply> {
        let tent = b.tentative_at(now);
        let proof = tent.proof(g)?;
        let sla_reasoner::ProofStep::Rule { id, rule, .. } = &proof.step else { return None };
        let stem = id.split('#').next().unwrap_or("").to_string();
        let meta = b.kb.vault.rules.get(&stem)?.meta();
        if meta.status != RuleStatus::Hypothesis {
            return None;
        }
        let clause = self.realizer.clause(g);
        let why = self.realizer.rule_text(rule);
        b.kb.log_change(now, "tentative_answer", &g.to_string(), &format!("from guessed rule {stem}"));
        self.pending = Some(Pending::Rule(stem));
        let text = format!("I'm not sure, but I'd guess so: {clause}. I noticed a pattern: {why}. Is that right?");
        Some(Reply { text, intent: "ask", claims: Vec::new() })
    }

    /// Store a rule in its own file. Returns false if it already exists.
    /// An unsafe or unstratifiable rule is refused.
    pub fn add_rule(&mut self, rule: Rule, status: RuleStatus, conf: f64, source: &str, note: &str) -> Result<bool, String> {
        let now = self.now();
        let mut b = self.brain();
        let out = add_rule_to(&mut b, rule, status, conf, source, note, now);
        let _ = b.save();
        out
    }

    fn ask(&mut self, b: &mut Brain, query: Vec<Literal>, history: bool) -> Reply {
        let now = self.now();
        let now_model = b.model_at(now);
        // Single ground atom: yes/no.
        if let [Literal::Pos(a)] = query.as_slice() {
            if let Some(g) = a.to_ground() {
                self.last_answer = Some(g.clone());
                let truth = if now_model.contains(&g) {
                    Truth::Yes
                } else if now_model.contains(&g.complement()) && !history {
                    Truth::No
                } else if b.was_true(&g, now) {
                    if history {
                        Truth::UsedTo
                    } else {
                        Truth::WasTrue
                    }
                } else if now_model.contains(&g.complement()) {
                    Truth::No
                } else {
                    Truth::Unknown
                };
                if truth == Truth::Yes {
                    if let Some(p) = now_model.proof(&g) {
                        self.record_uses(b, &now_model, &p.leaves());
                    }
                }
                // The tentative layer: a guessed rule may answer, said as a
                // guess, and the guess becomes a question about the rule.
                if truth == Truth::Unknown {
                    if let Some(r) = self.tentative_answer(b, &g, now) {
                        return r;
                    }
                }
                return self.say(Act::YesNo { atom: g, truth });
            }
            // One atom with W: wh-question.
            let mut found = now_model.query_atom(a);
            let mut past = false;
            if found.is_empty() || history {
                let hist = b.past_matches(a, now);
                if history || !hist.is_empty() {
                    past = found.is_empty() && !hist.is_empty();
                    if history {
                        // Past answers: those no longer true now.
                        let old: Vec<GroundAtom> = hist.iter().filter(|h| !found.contains(h)).cloned().collect();
                        if !old.is_empty() {
                            found = old;
                            past = true;
                        }
                    } else {
                        found = hist;
                    }
                }
            }
            found.sort();
            found.truncate(12);
            self.last_answer = found.first().cloned();
            self.record_uses(b, &now_model, &found);
            return self.say(Act::Wh { pattern: a.clone(), found, past });
        }
        // Conjunctive query: answer with the matching first atoms.
        match now_model.query(&query) {
            Ok(answers) => {
                let found: Vec<GroundAtom> = answers.iter().filter_map(|a| a.premises.first().cloned()).collect();
                let pattern = match query.first() {
                    Some(Literal::Pos(a)) => a.clone(),
                    _ => Atom::new("unknown", vec![]),
                };
                self.last_answer = found.first().cloned();
                self.say(Act::Wh { pattern, found, past: false })
            }
            Err(_) => self.say(Act::NotUnderstood),
        }
    }

    fn record_uses(&mut self, b: &mut Brain, m: &Model, atoms: &[GroundAtom]) {
        let mut leaves = Vec::new();
        for a in atoms {
            if let Some(p) = m.proof(a) {
                leaves.extend(p.leaves());
            }
        }
        let now = self.now();
        b.kb.record_use(&leaves, now);
    }

    /// "No, X" / "That's wrong".
    fn correct(&mut self, b: &mut Brain, atoms: Vec<Atom>) -> Reply {
        self.exception_noted = false;
        let r = self.correct_inner(b, atoms);
        if std::mem::take(&mut self.exception_noted) {
            let mut r = r;
            r.text.push_str(" I'll keep that as an exception to my rule for now.");
            return r;
        }
        r
    }

    fn correct_inner(&mut self, b: &mut Brain, atoms: Vec<Atom>) -> Reply {
        let now = self.now();
        let mut retracted = Vec::new();
        let mut added = Vec::new();
        let mut rules = Vec::new();
        let last = self.last_answer.clone();
        if atoms.is_empty() {
            // "That's wrong": the last answer is false.
            let Some(a) = last else {
                return self.say(Act::Note("What is wrong? I haven't said anything yet.".into()));
            };
            if let Some(r) = self.make_false(b, &a, now, &mut retracted) {
                rules.push(r);
            }
            return self.say(Act::Corrected { added, retracted, rules });
        }
        let model = b.model_at(now);
        let functional = b.kb.functional();
        for a in atoms {
            let Some(g) = a.to_ground() else { continue };
            let mut wrong = Vec::new();
            // The opposite of the correction is believed: it is wrong.
            if model.contains(&g.complement()) {
                wrong.push(g.complement());
            }
            // The last answer gave another value of a single-valued relation
            // ("No, Anna lives in Rome" after "Anna lives in Paris"). For a
            // relation with many values ("No, Anna likes coffee" after "Anna
            // likes tea") the old value stays: only an explicit negation removes it.
            if let Some(l) = &last {
                if l != &g && model.contains(l) && same_key(l, &g, &functional) {
                    wrong.push(l.clone());
                }
            }
            for w in wrong {
                if let Some(r) = self.make_false(b, &w, now, &mut retracted) {
                    rules.push(r);
                }
            }
            let out = b.kb.assert_fact(Fact::new(g.clone(), &self.source).with_conv(&self.session_id), now, "correction");
            if let AssertOutcome::Added { closed } = out {
                retracted.extend(closed.into_iter().map(|f| f.atom));
            }
            added.push(g);
        }
        retracted.sort();
        retracted.dedup();
        self.last_answer = added.first().cloned();
        self.say(Act::Corrected { added, retracted, rules })
    }

    /// Make `a` false from now on. A stored fact is closed (and its negation
    /// recorded); a derived one has its rule refined by the learner. Returns
    /// the refined rule, if any.
    fn make_false(&mut self, b: &mut Brain, a: &GroundAtom, now: Timestamp, retracted: &mut Vec<GroundAtom>) -> Option<Rule> {
        let stored = b.kb.all_facts().iter().any(|f| f.atom == *a && f.valid_at(now));
        if stored {
            self.retract(b, a, now, retracted);
            return None;
        }
        let out = corrections::correct_derived(&mut b.kb, a, now, &self.source);
        if !matches!(out.change, Change::NotDerived) {
            retracted.push(a.clone());
        }
        // One counterexample is only an exception: the rule's meaning stays.
        if out.change == Change::Exception {
            self.exception_noted = true;
            return None;
        }
        out.new_rule
    }

    /// Learn effect rules from a before/after demonstration.
    pub fn demonstrate(&mut self, demo: &Demonstration) -> Vec<String> {
        let now = self.now();
        let mut b = self.brain();
        for f in &demo.after {
            b.kb.assert_fact(Fact::new(f.clone(), &self.source), now, "demonstration");
        }
        b.kb.assert_fact(Fact::new(demo.action.clone(), &self.source), now, "demonstration");
        let learner = b.learner.clone();
        let out = corrections::learn_from_demo(&mut b.kb, &learner, demo, now);
        let _ = b.save();
        out
    }

    /// Make `a` false from now on: close it if stored, and record the negation.
    fn retract(&mut self, b: &mut Brain, a: &GroundAtom, now: Timestamp, retracted: &mut Vec<GroundAtom>) {
        let pattern = a.to_atom();
        let closed = b.kb.close_matching(&pattern, now, "user said it is wrong");
        retracted.push(a.clone());
        retracted.extend(closed.into_iter().map(|f| f.atom));
        b.kb.assert_fact(Fact::new(a.complement(), &self.source).with_conv(&self.session_id), now, "user said it is wrong");
    }

    fn answer_pending(&mut self, b: &mut Brain, pending: Option<Pending>, yes: bool) -> Reply {
        let now = self.now();
        match pending {
            Some(Pending::Fact(a)) => {
                let g = if yes { a } else { a.complement() };
                b.kb.assert_fact(Fact::new(g.clone(), &self.source).with_conv(&self.session_id), now, "answer to my question");
                self.last_answer = Some(g.clone());
                self.say(Act::Acknowledge { added: vec![g], closed: vec![], known: vec![], rules: vec![], past: false })
            }
            Some(Pending::Rule(id)) => {
                let stem = sla_core::vault::file_stem(&id);
                if let Some(rf) = b.kb.vault.rules.get_mut(&stem) {
                    let mut m = rf.meta();
                    m.status = if yes { RuleStatus::Confirmed } else { RuleStatus::Rejected };
                    if yes {
                        m.conf = m.conf.max(0.9);
                    }
                    rf.set_meta(&m);
                    b.kb.vault.mark_rule_dirty(&stem);
                    let action = if yes { "confirm_rule" } else { "reject_rule" };
                    b.kb.log_change(now, action, &id, "user answered my question");
                }
                self.say(Act::Note(if yes { "Thanks, I'll treat that as a rule now.".into() } else { "Okay, I'll drop that idea.".into() }))
            }
            Some(Pending::Contradiction(a, bb)) => {
                // "yes" keeps the first, "no" the second.
                let keep = if yes { a } else { bb };
                self.correct(b, vec![keep.to_atom()])
            }
            Some(Pending::Wh(_)) | Some(Pending::Open(_)) | None => self.say(Act::Note(if yes { "Good.".into() } else { "Okay.".into() })),
        }
    }

    /// Facts currently believed, rendered as Datalog lines (for the CLI).
    pub fn dump_facts(&self) -> Vec<String> {
        let now = self.now();
        let mut v: Vec<String> = self.brain().kb.all_facts().iter().filter(|f| f.valid_at(now)).map(|f| f.to_line()).collect();
        v.sort();
        v
    }

    pub fn dump_rules(&self) -> Vec<String> {
        self.brain().kb.rules(true).iter().filter(|r| r.module != "core").map(|r| format!("{} [{} {}] {}", r.id, r.status.as_str(), sla_core::term::fmt_conf(r.conf), r.rule)).collect()
    }

    /// True if `pattern` matches a fact valid now.
    pub fn believes(&self, pattern: &str) -> bool {
        let Ok(p) = sla_core::parse_atom(pattern) else { return false };
        self.model(false).all_facts().iter().any(|g| matches(&p, g))
    }
}

/// Do `a` and `b` give two values of one single-valued key (same predicate
/// and the same first K arguments of a `functional(P, K)` predicate)?
fn same_key(a: &GroundAtom, b: &GroundAtom, functional: &std::collections::HashMap<String, usize>) -> bool {
    if a.pred != b.pred || a.args.len() != b.args.len() || a.is_strong_negation() {
        return false;
    }
    // A thing has one colour at a time, but many qualities.
    if a.base_pred() == "property" && a.args.get(1).and_then(|v| v.as_sym()) == Some("quality") {
        return false;
    }
    match functional.get(a.base_pred()) {
        Some(&k) if k < a.args.len() => a.args[..k] == b.args[..k] && a.args[k..] != b.args[k..],
        _ => false,
    }
}

/// Store a rule in its own file in `b`. Returns false if it already exists.
fn add_rule_to(b: &mut Brain, rule: Rule, status: RuleStatus, conf: f64, source: &str, note: &str, now: Timestamp) -> Result<bool, String> {
    if b.kb.find_rule(&rule).is_some() {
        return Ok(false);
    }
    let module = router::module_for(&rule.head.pred);
    let base = format!("{}_{}", rule.head.pred.trim_start_matches('-').replace('-', "not_"), if rule.head.pred.starts_with('-') { "neg" } else { "rule" });
    let mut id = base.clone();
    let mut n = 2;
    while b.kb.vault.rules.contains_key(&sla_core::vault::file_stem(&id)) {
        id = format!("{base}_{n}");
        n += 1;
    }
    let meta = RuleMeta { id: id.clone(), status, conf, evidence: 0, counter: 0, module: module.into(), source: source.into() };
    b.kb.vault.put_rule_file(RuleFile::new(&meta, note, &[rule.clone()]));
    let rules: Vec<RuleInput> = b.kb.rules(true).iter().map(RuleInput::from).collect();
    if let Err(e) = sla_reasoner::check(&rules) {
        b.kb.vault.remove_rule_file(&id);
        return Err(e.to_string());
    }
    b.kb.log_change(now, "add_rule", &format!("{id}: {rule}"), note);
    Ok(true)
}

fn write_trace(dir: &std::path::Path, t: &TurnTrace) -> io::Result<()> {
    use std::io::Write;
    std::fs::create_dir_all(dir)?;
    let mut line = serde_json::to_string(t).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    line.push('\n');
    let mut f = std::fs::OpenOptions::new().create(true).append(true).open(dir.join(format!("{}.jsonl", t.session)))?;
    f.write_all(line.as_bytes())
}

/// Convenience for building a constant term.
pub fn sym(s: &str) -> Term {
    Term::Val(Value::sym(s))
}

/// Words that start sentences without being names.
const FUNCTION_WORDS: &[&str] = &[
    "the", "a", "an", "if", "is", "are", "do", "does", "did", "what", "who", "where", "why", "how", "when", "which", "my", "your", "yes", "no",
    "hello", "hi", "hey", "well", "ok", "okay", "so", "um", "actually", "that", "this", "these", "those", "there", "it", "he", "she", "they",
    "we", "you", "every", "all", "some", "each", "tell", "please", "thanks", "good", "bye", "nobody", "everyone", "someone", "something",
    "nothing", "everything", "people", "today", "now", "then", "also", "but", "and", "or", "maybe",
];

/// Capitalized words that are not the first word of a sentence, joined when
/// adjacent ("New York" -> new_york). "I" and its contractions are skipped.
pub fn proper_names(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current: Vec<String> = Vec::new();
    let mut sentence_start = true;
    let flush = |current: &mut Vec<String>, out: &mut Vec<String>| {
        if !current.is_empty() {
            let n = current.join("_");
            if !out.contains(&n) {
                out.push(n);
            }
            current.clear();
        }
    };
    let words: Vec<&str> = text.split_whitespace().collect();
    for (i, raw) in words.iter().enumerate() {
        let raw = *raw;
        let ends_sentence = raw.ends_with(['.', '!', '?']);
        let breaks = ends_sentence || raw.ends_with([',', ';', ':']);
        let w = raw.trim_matches(|c: char| !c.is_alphanumeric() && c != '\'');
        let w = w.strip_suffix("'s").unwrap_or(w).trim_matches('\'');
        let is_i = w == "I" || w.starts_with("I'");
        let cap = w.chars().next().map_or(false, |c| c.is_uppercase()) && w.chars().skip(1).all(|c| c.is_lowercase() || c == '-');
        // At the start of a sentence a capital proves nothing, unless the word is
        // followed by a singular verb and is not a common function word ("Pingu is", "Tweety flies").
        let initial_name = sentence_start && cap && !breaks && {
            let lw = w.to_lowercase();
            let next = words.get(i + 1).map(|n| n.trim_matches(|c: char| !c.is_alphanumeric() && c != '\'').to_lowercase()).unwrap_or_default();
            !FUNCTION_WORDS.contains(&lw.as_str())
                && !(lw.ends_with('s') && matches!(next.as_str(), "are" | "were" | "have"))
                && (matches!(next.as_str(), "is" | "was" | "has" | "doesn't" | "does" | "isn't" | "and")
                    || (next.ends_with('s') && !next.ends_with("ss") && next.len() > 2))
        };
        if cap && !is_i && (!sentence_start || initial_name) && w.len() > 1 {
            current.push(w.to_lowercase());
        } else {
            flush(&mut current, &mut out);
        }
        if breaks {
            flush(&mut current, &mut out);
        }
        sentence_start = ends_sentence;
    }
    flush(&mut current, &mut out);
    out
}

#[cfg(test)]
mod tests {
    #[test]
    fn names() {
        assert_eq!(super::proper_names("Anna lives in New York. Is Rex a dog? I'm Maksim"), vec!["anna", "new_york", "rex", "maksim"]);
        assert_eq!(super::proper_names("Pingu doesn't fly. Tweety flies. Dogs are animals. The car is red. If it rains"), vec!["pingu", "tweety"]);
        assert!(super::proper_names("Hello there.").is_empty());
    }
}

/// Split a message into sentences at . ! ? followed by a space (not inside
/// numbers or abbreviations like "e.g.").
pub fn split_sentences(text: &str) -> Vec<String> {
    // Clauses joined by ", and" / ", but" / "; " are separate statements.
    let text = &text.replace(", and ", ". ").replace(", but ", ". ").replace("; ", ". ");
    let mut out = Vec::new();
    let mut cur = String::new();
    let chars: Vec<char> = text.chars().collect();
    for (i, &c) in chars.iter().enumerate() {
        cur.push(c);
        let end = matches!(c, '.' | '!' | '?') && chars.get(i + 1).map_or(true, |n| n.is_whitespace());
        if end {
            let t = cur.trim().to_string();
            // Keep a lone greeting word attached to nothing: it is its own sentence anyway.
            if t.chars().any(|c| c.is_alphanumeric()) {
                out.push(t);
            }
            cur.clear();
        }
    }
    let t = cur.trim().to_string();
    if t.chars().any(|c| c.is_alphanumeric()) {
        out.push(t);
    }
    out
}
