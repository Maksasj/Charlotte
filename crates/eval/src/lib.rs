//! Evaluation: fixed knowledge tests (parser sets, teaching dialogues),
//! live Claude teachers answering curiosity questions, and Haiku judges that
//! chat with the agent and score conversation quality. Scores are appended
//! to `eval/history.csv` so trends are visible over time.


pub mod archive;
pub mod ops;
pub mod outcome;
pub mod regression;
pub mod trainer;

use sla_agent::curiosity::Teacher;
use sla_agent::Agent;
use sla_core::{parse_ground, parse_rule, GroundAtom, Rule, Timestamp};
use sla_parser::{parse, Context};
use std::fmt::Write as _;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;

// ------------------------------------------------------------------ dialogues

#[derive(Debug, Clone, PartialEq)]
pub enum Check {
    Yes,
    No,
    Unknown,
    Contains(Vec<String>),
    Not(Vec<String>),
}

impl Check {
    pub fn parse(s: &str) -> Option<Check> {
        let s = s.trim();
        let list = |x: &str| x.split(';').map(|p| p.trim().to_lowercase()).filter(|p| !p.is_empty()).collect();
        match s {
            "yes" => Some(Check::Yes),
            "no" => Some(Check::No),
            "unknown" => Some(Check::Unknown),
            _ => {
                if let Some(r) = s.strip_prefix("contains:") {
                    Some(Check::Contains(list(r)))
                } else {
                    s.strip_prefix("not:").map(|r| Check::Not(list(r)))
                }
            }
        }
    }

    pub fn passes(&self, reply: &str) -> bool {
        let low = reply.to_lowercase();
        match self {
            Check::Yes => reply.starts_with("Yes") || reply.starts_with("That's right"),
            Check::No => reply.starts_with("No"),
            Check::Unknown => ["don't know", "not sure", "no idea"].iter().any(|k| low.contains(k)),
            Check::Contains(v) => v.iter().all(|k| low.contains(k)),
            Check::Not(v) => !v.iter().any(|k| low.contains(k)),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Dialogue {
    pub name: String,
    pub domain: String,
    pub script: Vec<(String, Option<Check>)>,
    pub facts: Vec<GroundAtom>,
    pub rules: Vec<Rule>,
    pub truth: Vec<GroundAtom>,
    /// Lines that could not be read (bad atoms, unknown checks).
    pub errors: Vec<String>,
}

impl Dialogue {
    pub fn parse(src: &str, fallback_name: &str) -> Dialogue {
        let mut d = Dialogue { name: fallback_name.into(), ..Default::default() };
        let mut section = String::new();
        for line in src.lines() {
            let t = line.trim();
            if let Some(n) = t.strip_prefix("# Dialogue:") {
                d.name = n.trim().into();
            } else if let Some(dom) = t.strip_prefix("domain:") {
                d.domain = dom.trim().into();
            } else if let Some(sec) = t.strip_prefix("## ") {
                section = sec.trim().to_lowercase();
            } else if let Some(item) = t.strip_prefix("- ") {
                let item = item.trim();
                match section.as_str() {
                    "script" => match item.rsplit_once(" => ") {
                        Some((msg, check)) => match Check::parse(check) {
                            Some(c) => d.script.push((msg.trim().into(), Some(c))),
                            None => d.errors.push(format!("bad check: {item}")),
                        },
                        None => d.script.push((item.into(), None)),
                    },
                    "expect facts" => match parse_ground(item) {
                        Ok(g) => d.facts.push(g),
                        Err(e) => d.errors.push(format!("bad fact {item}: {e}")),
                    },
                    "expect rules" => match parse_rule(item) {
                        Ok(r) => d.rules.push(r),
                        Err(e) => d.errors.push(format!("bad rule {item}: {e}")),
                    },
                    "ground truth" => match parse_ground(item) {
                        Ok(g) => d.truth.push(g),
                        Err(e) => d.errors.push(format!("bad truth {item}: {e}")),
                    },
                    _ => {}
                }
            }
        }
        d
    }

    pub fn load_dir(dir: &Path) -> Vec<Dialogue> {
        let mut paths: Vec<PathBuf> = std::fs::read_dir(dir)
            .map(|rd| rd.flatten().map(|e| e.path()).filter(|p| p.extension().map_or(false, |x| x == "md")).collect())
            .unwrap_or_default();
        paths.sort();
        paths
            .iter()
            .filter_map(|p| {
                let src = std::fs::read_to_string(p).ok()?;
                Some(Dialogue::parse(&src, &p.file_stem()?.to_string_lossy()))
            })
            .collect()
    }
}

#[derive(Debug, Clone, Default)]
pub struct DialogueResult {
    pub name: String,
    pub checks: (usize, usize),
    pub facts: (usize, usize),
    pub rules: (usize, usize),
    pub failures: Vec<String>,
    pub transcript: Vec<(String, String)>,
    pub teacher_questions: usize,
    pub teacher_consistent: usize,
}

impl DialogueResult {
    pub fn passed(&self) -> usize {
        self.checks.0 + self.facts.0 + self.rules.0
    }
    pub fn total(&self) -> usize {
        self.checks.1 + self.facts.1 + self.rules.1
    }
}

fn temp_dir(tag: &str) -> PathBuf {
    let n = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_nanos());
    let d = std::env::temp_dir().join(format!("sla_eval_{tag}_{}_{n}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    d
}

/// A stored rule satisfies an expected rule if the heads match and every
/// expected body literal is present (corrections may add conditions).
fn rule_satisfied(agent: &Agent, want: &Rule) -> bool {
    agent.brain().kb.rules(true).iter().any(|r| r.rule.head == want.head && want.body.iter().all(|l| r.rule.body.contains(l)))
}

/// Run one dialogue on a fresh agent. With a teacher, the agent then asks up
/// to `teacher_questions` curiosity questions and learns from the answers.
pub fn run_dialogue(d: &Dialogue, teacher: Option<&mut dyn Teacher>, teacher_questions: usize) -> DialogueResult {
    let root = temp_dir("dlg");
    let mut res = DialogueResult { name: d.name.clone(), ..Default::default() };
    let mut agent = match Agent::open(&root, "maksim") {
        Ok(a) => a,
        Err(e) => {
            res.failures.push(format!("cannot open agent: {e}"));
            return res;
        }
    };
    // Scripted lines don't answer questions, so the agent keeps its curiosity
    // questions for the teacher phase instead of asking them in chat.
    agent.ask_in_chat = false;
    let mut t = Timestamp::from_ymd_hms(2026, 1, 5, 9, 0, 0);
    for (msg, check) in &d.script {
        t = t.plus_secs(60);
        agent.clock = Some(t);
        let reply = agent.respond(msg).text;
        res.transcript.push((msg.clone(), reply.clone()));
        if let Some(c) = check {
            res.checks.1 += 1;
            if c.passes(&reply) {
                res.checks.0 += 1;
            } else {
                res.failures.push(format!("check {c:?} failed: {msg:?} -> {reply:?}"));
            }
        }
    }
    let model = agent.model(false);
    for f in &d.facts {
        res.facts.1 += 1;
        if model.contains(f) {
            res.facts.0 += 1;
        } else {
            res.failures.push(format!("missing fact {f}"));
        }
    }
    for r in &d.rules {
        res.rules.1 += 1;
        if rule_satisfied(&agent, r) {
            res.rules.0 += 1;
        } else {
            res.failures.push(format!("missing rule {r}"));
        }
    }
    for e in &d.errors {
        res.failures.push(format!("data error: {e}"));
    }
    // Live teacher phase: does the agent learn only true things from its questions?
    if let Some(teacher) = teacher {
        for _ in 0..teacher_questions {
            let before = agent.brain().kb.all_facts();
            t = t.plus_secs(60);
            agent.clock = Some(t);
            let Some((q, a, reply)) = agent.ask_teacher(teacher) else { break };
            res.transcript.push((format!("[agent asks] {q}"), format!("[teacher] {a} / [agent] {}", reply.text)));
            res.teacher_questions += 1;
            let new: Vec<GroundAtom> = agent.brain().kb.all_facts().into_iter().filter(|f| !before.contains(f)).map(|f| f.atom).collect();
            let contradicts = new.iter().any(|g| d.truth.contains(&g.complement()));
            if !contradicts {
                res.teacher_consistent += 1;
            } else {
                res.failures.push(format!("teacher answer led to a false fact: {q} -> {a}"));
            }
        }
    }
    let _ = std::fs::remove_dir_all(&root);
    res
}

// ------------------------------------------------------------------ parser sets

pub fn load_parser_set(path: &Path) -> Vec<(String, String)> {
    std::fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .filter_map(|l| l.strip_prefix("- "))
        .filter_map(|l| l.rsplit_once(" => "))
        .map(|(s, e)| (s.trim().to_string(), e.trim().to_string()))
        .collect()
}

/// Exact-match accuracy of the parser on a set.
pub fn parser_accuracy(set: &[(String, String)]) -> f64 {
    if set.is_empty() {
        return 0.0;
    }
    let ctx = Context::default();
    set.iter().filter(|(s, want)| &parse(s, &ctx).canonical() == want).count() as f64 / set.len() as f64
}

// ------------------------------------------------------------------ Claude

/// One model call (lean, with usage accounting under the role "eval").
pub fn claude(prompt: &str, model: &str) -> Result<String, String> {
    sla_parser::claude::call("eval", model, prompt)
}

/// A model call recorded under a specific role ("teacher", "reviewer", "judge").
pub fn claude_as(role: &str, prompt: &str, model: &str) -> Result<String, String> {
    sla_parser::claude::call(role, model, prompt)
}

const AGENT_ABILITIES: &str = "The agent only understands short, simple English sentences such as: \
'Anna likes tea.', 'Rex is a dog.', 'Dogs are animals.', 'The car is red.', 'Anna lives in Berlin.', 'Anna's job is teacher.', \
'I don't like coffee anymore.', 'If someone likes tea then they want tea.', 'Does Anna like tea?', 'What does Anna like?', \
'Who likes tea?', 'Where does Anna live?', 'Why?', 'What do you know about Anna?', 'No, Rex is a cat.', 'yes', 'no'. \
The user is called Maksim; 'I' means Maksim.";

/// A live teacher: answers the agent's curiosity questions from a ground truth only.
pub struct ClaudeTeacher {
    pub truth: Vec<GroundAtom>,
    pub model: String,
    pub log: Vec<String>,
}

impl Teacher for ClaudeTeacher {
    fn answer(&mut self, question: &str) -> String {
        let truth: Vec<String> = self.truth.iter().map(|g| g.to_string()).collect();
        let prompt = format!(
            "You are a teacher helping a small symbolic learning agent. {AGENT_ABILITIES}\n\n\
             The world you know is exactly these Datalog facts (a leading '-' means 'not'):\n{}\n\n\
             The agent asks you: \"{question}\"\n\n\
             Answer in ONE short simple sentence of the kinds above, using only the facts listed. \
             For a yes/no question about a rule or pattern, answer 'yes' only if the facts clearly support it for every case, else 'no'. \
             If the facts do not answer it, reply exactly: I don't know. Output only the answer sentence.",
            truth.join("\n")
        );
        let ans = claude(&prompt, &self.model).unwrap_or_else(|_| "I don't know.".into());
        let ans = ans.lines().next().unwrap_or("I don't know.").trim().trim_matches('"').to_string();
        self.log.push(format!("Q: {question} A: {ans}"));
        ans
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct JudgeResult {
    pub topic: String,
    pub score: Option<f64>,
    pub reason: String,
    pub transcript: Vec<(String, String)>,
}

/// One judge: chats with a fresh agent for `turns` messages about `topic`, then scores it.
pub fn run_judge(topic: &str, turns: usize, model: &str) -> JudgeResult {
    run_judge_with(topic, turns, model, false, None)
}

/// Copy a directory tree (a vault) without external crates.
pub fn copy_dir(from: &Path, to: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(to)?;
    for e in std::fs::read_dir(from)? {
        let e = e?;
        let target = to.join(e.file_name());
        if e.file_type()?.is_dir() {
            copy_dir(&e.path(), &target)?;
        } else {
            std::fs::copy(e.path(), target)?;
        }
    }
    Ok(())
}

/// A judge run; with `llm` the agent uses the LLM translator for input; with
/// `seed` the agent starts from a copy of that vault instead of an empty one.
pub fn run_judge_with(topic: &str, turns: usize, model: &str, llm: bool, seed: Option<&Path>) -> JudgeResult {
    let root = temp_dir("judge");
    if let Some(seed) = seed {
        let _ = copy_dir(seed, &root);
    }
    let mut res = JudgeResult { topic: topic.into(), score: None, reason: String::new(), transcript: Vec::new() };
    let mut agent = match Agent::open(&root, "maksim") {
        Ok(a) => a,
        Err(e) => {
            res.reason = format!("cannot open agent: {e}");
            return res;
        }
    };
    if llm {
        agent.enable_llm(model);
    }
    let mut t = Timestamp::from_ymd_hms(2026, 1, 5, 9, 0, 0);
    let render = |tr: &[(String, String)]| tr.iter().map(|(u, a)| format!("User: {u}\nAgent: {a}")).collect::<Vec<_>>().join("\n");
    for _ in 0..turns {
        let prompt = format!(
            "You are role-playing a user (Maksim) chatting with a simple learning chatbot, to test how natural and coherent it is. \
             {AGENT_ABILITIES}\n\nTopic: {topic}. Teach it a few facts about the topic, ask questions about what you taught, \
             ask 'Why?' once, and answer its questions if it asks any.\n\nConversation so far:\n{}\n\n\
             Write ONLY your next single message to the agent (one short sentence), nothing else.",
            if res.transcript.is_empty() { "(none yet)".to_string() } else { render(&res.transcript) }
        );
        let msg = match claude(&prompt, model) {
            Ok(m) => m.lines().next().unwrap_or("").trim().trim_matches('"').to_string(),
            Err(e) => {
                res.reason = e;
                let _ = std::fs::remove_dir_all(&root);
                return res;
            }
        };
        if msg.is_empty() {
            break;
        }
        t = t.plus_secs(60);
        agent.clock = Some(t);
        let reply = agent.respond(&msg).text;
        res.transcript.push((msg, reply));
    }
    let prompt = format!(
        "Below is a conversation between a user and a simple symbolic learning chatbot. {AGENT_ABILITIES}\n\n{}\n\n\
         Rate the chatbot's side from 1 to 10 for how natural and coherent it is as a conversation partner \
         (grammar, relevance, consistency, sensible answers to what it was taught). \
         Reply in exactly two lines:\nSCORE: <integer 1-10>\nREASON: <one short sentence>",
        render(&res.transcript)
    );
    match claude(&prompt, model) {
        Ok(out) => {
            for line in out.lines() {
                if let Some(s) = line.trim().strip_prefix("SCORE:") {
                    res.score = s.trim().split(|c: char| !c.is_ascii_digit()).next().and_then(|n| n.parse().ok());
                }
                if let Some(r) = line.trim().strip_prefix("REASON:") {
                    res.reason = r.trim().to_string();
                }
            }
            if res.score.is_none() {
                res.reason = format!("unparseable judge output: {out}");
            }
        }
        Err(e) => res.reason = e,
    }
    let _ = std::fs::remove_dir_all(&root);
    res
}

pub const JUDGE_TOPICS: &[&str] = &[
    "your family and what they like to eat",
    "pets and animals you know",
    "your city, your work and your colleagues",
    "hobbies and sports of your friends",
    "tools in a workshop and what they are for",
    "animals and nature",
    "food and cooking",
    "weather and the seasons",
];

// ------------------------------------------------------------------ the whole run

#[derive(Debug, Clone)]
pub struct EvalConfig {
    pub root: PathBuf,
    pub history: PathBuf,
    pub judges: usize,
    pub turns: usize,
    pub teacher_questions: usize,
    pub use_claude: bool,
    pub model: String,
    pub notes: String,
    /// Judges talk to the agent with the LLM translator enabled.
    pub llm: bool,
    /// Judges' agents start from a copy of this vault (e.g. the pretrained one).
    pub seed_vault: Option<PathBuf>,
}

impl EvalConfig {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        let root = root.into();
        EvalConfig {
            history: root.join("eval").join("history.csv"),
            root,
            judges: 3,
            turns: 6,
            teacher_questions: 2,
            use_claude: true,
            model: "haiku".into(),
            notes: String::new(),
            llm: false,
            seed_vault: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct EvalReport {
    pub timestamp: String,
    pub commit: String,
    pub parser_dev: f64,
    pub parser_held_out: f64,
    pub dev: Vec<DialogueResult>,
    pub held_out: Vec<DialogueResult>,
    pub judges: Vec<JudgeResult>,
    pub teacher_log: Vec<String>,
}

fn ratio(p: usize, t: usize) -> f64 {
    if t == 0 {
        0.0
    } else {
        p as f64 / t as f64
    }
}

impl EvalReport {
    pub fn knowledge(results: &[DialogueResult]) -> f64 {
        ratio(results.iter().map(|r| r.passed()).sum(), results.iter().map(|r| r.total()).sum())
    }

    pub fn checks(results: &[DialogueResult]) -> (usize, usize) {
        (results.iter().map(|r| r.checks.0).sum(), results.iter().map(|r| r.checks.1).sum())
    }

    pub fn conversation(&self) -> Option<f64> {
        let s: Vec<f64> = self.judges.iter().filter_map(|j| j.score).collect();
        if s.is_empty() {
            None
        } else {
            Some(s.iter().sum::<f64>() / s.len() as f64)
        }
    }

    pub fn teacher(&self) -> (usize, usize) {
        let all = self.dev.iter().chain(&self.held_out);
        (all.clone().map(|r| r.teacher_questions).sum(), all.map(|r| r.teacher_consistent).sum())
    }

    pub const HEADER: &'static str = "timestamp,commit,parser_dev,parser_held_out,dialogue_checks_dev,dialogue_checks_held_out,knowledge_dev,knowledge_held_out,teacher_questions,teacher_consistent,conversation_score,judges,notes";

    pub fn csv_row(&self, notes: &str) -> String {
        let (cd, td) = Self::checks(&self.dev);
        let (ch, th) = Self::checks(&self.held_out);
        let (tq, tc) = self.teacher();
        let conv = self.conversation().map(|c| format!("{c:.2}")).unwrap_or_default();
        let judges = self.judges.iter().filter(|j| j.score.is_some()).count();
        let notes = notes.replace([',', '\n'], ";");
        format!(
            "{},{},{:.3},{:.3},{cd}/{td},{ch}/{th},{:.3},{:.3},{tq},{tc},{conv},{judges},{notes}",
            self.timestamp,
            self.commit,
            self.parser_dev,
            self.parser_held_out,
            Self::knowledge(&self.dev),
            Self::knowledge(&self.held_out),
        )
    }

    /// Detailed Markdown report.
    pub fn markdown(&self) -> String {
        let mut s = String::new();
        let _ = writeln!(s, "# Evaluation report {}\n\ncommit `{}`\n", self.timestamp, self.commit);
        let _ = writeln!(s, "## Knowledge and reasoning\n");
        let _ = writeln!(s, "- parser exact accuracy: dev {:.1}%, held-out {:.1}%", self.parser_dev * 100.0, self.parser_held_out * 100.0);
        for (label, rs) in [("dev", &self.dev), ("held-out", &self.held_out)] {
            let (c, t) = Self::checks(rs);
            let _ = writeln!(s, "- teaching dialogues ({label}): checks {c}/{t}, knowledge score {:.1}%", Self::knowledge(rs) * 100.0);
        }
        let (tq, tc) = self.teacher();
        let _ = writeln!(s, "- live teacher: {tq} questions asked, {tc} answers learned without contradicting the ground truth\n");
        for (label, rs) in [("dev", &self.dev), ("held-out", &self.held_out)] {
            let _ = writeln!(s, "### Dialogues: {label}\n");
            for r in rs {
                let _ = writeln!(s, "- **{}**: checks {}/{}, facts {}/{}, rules {}/{}", r.name, r.checks.0, r.checks.1, r.facts.0, r.facts.1, r.rules.0, r.rules.1);
                // Held-out failures are listed only by count, so they don't steer tuning.
                if label == "dev" {
                    for f in &r.failures {
                        let _ = writeln!(s, "  - {f}");
                    }
                } else if !r.failures.is_empty() {
                    let _ = writeln!(s, "  - {} failure(s) (details hidden for the held-out set)", r.failures.len());
                }
            }
            let _ = writeln!(s);
        }
        let _ = writeln!(s, "## Conversation quality\n");
        match self.conversation() {
            Some(c) => {
                let _ = writeln!(s, "Average judge score: **{c:.2} / 10** from {} judge(s).\n", self.judges.iter().filter(|j| j.score.is_some()).count());
            }
            None => {
                let _ = writeln!(s, "No judge scores (judges disabled or unavailable).\n");
            }
        }
        for j in &self.judges {
            let score = j.score.map(|x| x.to_string()).unwrap_or("-".into());
            let _ = writeln!(s, "### Judge: {} (score {score})\n\n{}\n", j.topic, j.reason);
            for (u, a) in &j.transcript {
                let _ = writeln!(s, "> **User:** {u}  \n> **Agent:** {a}\n");
            }
        }
        s
    }
}

/// Run the full evaluation and append one row to the history file.
pub fn run(cfg: &EvalConfig) -> std::io::Result<EvalReport> {
    let data = cfg.root.join("data").join("synthetic");
    let mut rep = EvalReport {
        timestamp: Timestamp::now().to_string(),
        commit: Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .current_dir(&cfg.root)
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|| "unknown".into()),
        ..Default::default()
    };
    rep.parser_dev = parser_accuracy(&load_parser_set(&data.join("parser").join("dev.md")));
    rep.parser_held_out = parser_accuracy(&load_parser_set(&data.join("parser").join("held_out.md")));

    for (dir, out) in [("dev", &mut rep.dev), ("held_out", &mut rep.held_out)] {
        for d in Dialogue::load_dir(&data.join("teaching").join(dir)) {
            let r = if cfg.use_claude && cfg.teacher_questions > 0 && !d.truth.is_empty() {
                let mut t = ClaudeTeacher { truth: d.truth.clone(), model: cfg.model.clone(), log: Vec::new() };
                let r = run_dialogue(&d, Some(&mut t), cfg.teacher_questions);
                rep.teacher_log.extend(t.log);
                r
            } else {
                run_dialogue(&d, None, 0)
            };
            out.push(r);
        }
    }
    if cfg.use_claude {
        for i in 0..cfg.judges {
            let j = run_judge_with(JUDGE_TOPICS[i % JUDGE_TOPICS.len()], cfg.turns, &cfg.model, cfg.llm, cfg.seed_vault.as_deref());
            // Every judge chat is training data: keep it.
            let _ = archive::append(&archive::dir(&cfg.root), "judges", &serde_json::json!({ "model": cfg.model, "llm_translator": cfg.llm, "judge": j }));
            rep.judges.push(j);
        }
    }

    // Append the history row (header on first use) and save the report.
    if let Some(dir) = cfg.history.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let new = !cfg.history.exists();
    let mut f = std::fs::OpenOptions::new().create(true).append(true).open(&cfg.history)?;
    if new {
        writeln!(f, "{}", EvalReport::HEADER)?;
    }
    writeln!(f, "{}", rep.csv_row(&cfg.notes))?;
    if let Some(dir) = cfg.history.parent() {
        let reports = dir.join("reports");
        std::fs::create_dir_all(&reports)?;
        let name = rep.timestamp.replace(':', "-");
        std::fs::write(reports.join(format!("{name}.md")), rep.markdown())?;
        std::fs::write(dir.join("last_report.md"), rep.markdown())?;
    }
    Ok(rep)
}
