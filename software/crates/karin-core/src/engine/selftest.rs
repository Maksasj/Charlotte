//! Self-tests (run by `cargo test`): short scripted conversations with a fresh
//! Karin built from the vault on disk. Each one records every step (what was
//! said, Karin's reply, what was expected), so a failure names the exact turn.

use super::Engine;
use crate::types::TurnTrace;

pub struct SelfTestResult {
    pub name: String,
    /// What ability this checks, in plain words.
    #[allow(dead_code)]
    pub about: String,
    pub passed: bool,
    /// The first step that failed (empty when passed).
    pub detail: String,
    #[allow(dead_code)]
    pub steps: Vec<SelfTestStep>,
}

/// One step: something said to Karin (`say`), done to her engine directly
/// (`action`), or an extra condition (`check`), with what was expected.
pub struct SelfTestStep {
    pub kind: String,
    pub input: String,
    pub output: String,
    pub expect: String,
    pub ok: bool,
}

impl Engine {
    pub fn run_selftests() -> Vec<SelfTestResult> {
        vec![
            test_persona_and_multiturn(),
            test_impasse_and_teaching(),
            test_anti_unification(),
            test_chain_chunking(),
            test_knowledge_graph_inheritance(),
            test_shared_slot_filling_and_intents(),
            test_data_driven_unit_conversions(),
            test_typed_relations_and_inverses(),
            test_generic_workflows(),
            test_collect_and_additive_memory(),
            test_correction_rule_induction_and_active_learning(),
        ]
    }
}

/// A scripted conversation that keeps a record of each step.
struct Script {
    eng: Engine,
    steps: Vec<SelfTestStep>,
}

impl Script {
    fn new() -> Self {
        Self { eng: Engine::new(), steps: Vec::new() }
    }

    fn record(&mut self, kind: &str, input: &str, output: &str, expect: String, ok: bool) {
        self.steps.push(SelfTestStep {
            kind: kind.into(),
            input: input.into(),
            output: output.into(),
            expect,
            ok,
        });
    }

    /// Says something without checking the reply (setting the scene).
    fn say(&mut self, input: &str) -> TurnTrace {
        let t = self.eng.turn(input);
        self.record("say", input, &t.reply, String::new(), true);
        t
    }

    /// Says something; the reply must contain every one of `parts`.
    fn expect(&mut self, input: &str, parts: &[&str]) -> TurnTrace {
        let t = self.eng.turn(input);
        let ok = parts.iter().all(|p| t.reply.contains(p));
        let expect = parts
            .iter()
            .map(|p| format!("\u{201c}{p}\u{201d}"))
            .collect::<Vec<_>>()
            .join(" and ");
        self.record("say", input, &t.reply, format!("mentions {expect}"), ok);
        t
    }

    /// Says something; `check` decides, `expect` says what it looks for.
    fn check(&mut self, input: &str, expect: &str, check: impl Fn(&TurnTrace) -> bool) -> TurnTrace {
        let t = self.eng.turn(input);
        let ok = check(&t);
        self.record("say", input, &t.reply, expect.into(), ok);
        t
    }

    /// Something done to the engine directly rather than said.
    fn action(&mut self, input: &str, output: &str, expect: &str, ok: bool) {
        self.record("action", input, output, expect.into(), ok);
    }

    /// An extra condition about what happened so far.
    fn assert(&mut self, what: &str, observed: String, ok: bool) {
        self.record("check", what, &observed, what.into(), ok);
    }

    fn finish(self, name: &str, about: &str) -> SelfTestResult {
        let failed = self.steps.iter().find(|s| !s.ok);
        let detail = match failed {
            Some(s) => format!("{} \u{2192} '{}' (expected: {})", s.input, s.output, s.expect),
            None => String::new(),
        };
        SelfTestResult {
            name: name.into(),
            about: about.into(),
            passed: failed.is_none(),
            detail,
            steps: self.steps,
        }
    }
}

fn test_persona_and_multiturn() -> SelfTestResult {
    let mut s = Script::new();
    s.expect("who are you", &["Karin"]);
    s.expect("my name is Alex", &["Alex"]);
    s.expect("hello", &["Alex"]);
    s.expect("weather", &["Which city"]);
    s.expect("Paris", &["partly cloudy"]);
    s.finish(
        "Persona & multi-turn slot context",
        "Karin knows who she is, remembers your name, and finishes a question over two turns.",
    )
}

fn test_impasse_and_teaching() -> SelfTestResult {
    let mut s = Script::new();
    s.check("launch rockets", "does not understand yet", |t| t.impasse);
    let (_, pat) = s.eng.teach_reply("launch rockets", "Ignition sequence started!", true);
    s.action(
        "teach: \u{201c}launch rockets\u{201d} \u{2192} \u{201c}Ignition sequence started!\u{201d}",
        &format!("new rule {}", pat.id),
        "",
        true,
    );
    s.check("launch rockets", "replies with the taught answer", |t| {
        !t.impasse && t.reply == "Ignition sequence started!"
    });
    s.finish(
        "Impasse detection & live teaching",
        "When Karin doesn't understand she says so, and a reply you teach works right away.",
    )
}

fn test_anti_unification() -> SelfTestResult {
    let mut s = Script::new();
    s.check("greet Clara", "does not understand yet", |t| t.impasse);
    let (created, _) = s.eng.generalize_episodes(true);
    s.action(
        "generalize from earlier examples",
        &format!("{} new rule(s)", created.len()),
        "at least one rule",
        !created.is_empty(),
    );
    s.check("greet Clara", "now greets Clara", |t| !t.impasse && t.reply.contains("Clara"));
    s.finish(
        "Anti-unification generalization",
        "From similar examples Karin builds a general rule that handles a sentence she never saw.",
    )
}

fn test_chain_chunking() -> SelfTestResult {
    let mut s = Script::new();
    let before = s.expect("calculate 6 * 7", &["42"]);
    s.assert(
        "takes 3 steps",
        format!("{} steps", before.steps.len()),
        before.steps.len() == 3,
    );
    let (chunked, _) = s.eng.chunk_last_chain(true);
    s.action(
        "merge the last chain of steps into one rule",
        &chunked.as_ref().map(|p| format!("new rule {}", p.id)).unwrap_or("nothing merged".into()),
        "a merged rule",
        chunked.is_some(),
    );
    let after = s.expect("calculate 9 * 9", &["81"]);
    s.assert(
        "takes fewer steps than before",
        format!("{} steps (before: {})", after.steps.len(), before.steps.len()),
        after.steps.len() < before.steps.len(),
    );
    s.finish(
        "Chain chunking & composition",
        "A chain of rules that often fires together becomes one rule, so the same job takes fewer steps.",
    )
}

fn test_knowledge_graph_inheritance() -> SelfTestResult {
    let mut s = Script::new();
    s.expect("is Rust a tool", &["Yes, Rust is a tool"]);
    s.expect("what is the purpose of Rust", &["writing software"]);
    s.expect("Go is a programming language", &["Learned: Go is a programming language"]);
    s.expect("what is the purpose of Go", &["writing software"]);
    s.finish(
        "Knowledge graph & transitive inheritance",
        "Facts pass down \u{201c}is a\u{201d} links: something new inherits what its category knows.",
    )
}

fn test_shared_slot_filling_and_intents() -> SelfTestResult {
    let mut s = Script::new();
    s.expect("forecast for Tokyo", &["Tokyo"]);
    s.expect("compute 5 * 8", &["40"]);
    s.expect("define", &["define"]);
    s.expect("symbolic AI", &["explicit facts"]);
    s.expect("weather", &["Which city"]);
    s.expect("cancel", &["cancelled"]);
    s.finish(
        "Shared slot-filling & canonical intents",
        "Different wordings reach the same skill, missing details are asked for, and \u{201c}cancel\u{201d} drops a question.",
    )
}

fn test_data_driven_unit_conversions() -> SelfTestResult {
    let mut s = Script::new();
    s.expect("convert 2 km to m", &["2000 m"]);
    s.expect("3 h in min", &["180 min"]);
    s.expect("convert 5 km to kg", &["incompatible dimensions"]);
    s.finish(
        "Data-driven unit conversions",
        "Unit conversions come from facts in the vault, and impossible ones are refused.",
    )
}

fn test_typed_relations_and_inverses() -> SelfTestResult {
    let mut s = Script::new();
    s.expect("what does France contain", &["Paris", "inverse"]);
    s.expect("Go was created by Google", &["Learned: Go was created by Google"]);
    s.expect("what did Google create", &["Go", "inverse"]);
    s.finish(
        "Typed relations & automatic inverses",
        "A relation can be asked from either side: \u{201c}created by\u{201d} also answers \u{201c}what did X create\u{201d}.",
    )
}

fn test_generic_workflows() -> SelfTestResult {
    let mut s = Script::new();
    s.expect("add a reminder", &["What should I remind you to do"]);
    s.expect("buy milk", &["When should I remind you"]);
    s.expect("tomorrow", &["Reminder set", "buy milk", "tomorrow"]);
    s.expect("remind me to call mom", &["When should I remind you"]);
    s.expect("at 5", &["Reminder set", "call mom"]);
    s.finish(
        "Generic multi-step workflows",
        "A workflow asks only for what's missing, step by step, then finishes.",
    )
}

fn test_collect_and_additive_memory() -> SelfTestResult {
    let mut s = Script::new();
    s.say("my favorite drink is black tea");
    s.expect("what drink I like", &["black tea"]);
    s.expect("I also like caramel macchiato", &["caramel macchiato"]);
    s.expect("what drink do I like", &["black tea", "caramel macchiato"]);
    s.finish(
        "Generic @collect & additive topic memory",
        "New likes add to old ones, and a question lists them all.",
    )
}

fn test_correction_rule_induction_and_active_learning() -> SelfTestResult {
    let mut s = Script::new();
    s.say("my favorite drink is black tea");
    s.say("I like caramel macchiato");
    s.say("caramel macchiato is a drink");
    s.check(
        "what is my favorite drink",
        "lists black tea, not yet caramel macchiato",
        |t| t.reply.contains("black tea") && !t.reply.contains("caramel macchiato"),
    );
    s.expect("what about caramel macchiato", &["proposed candidate rule"]);
    s.expect("approve", &["Approved rule"]);
    s.expect("what drink do I like", &["caramel macchiato"]);
    s.say("I like matcha");
    s.say("matcha is a drink");
    s.expect("what drink do I like", &["matcha"]);
    s.expect("what about oolong", &["haven't heard of oolong"]);
    s.expect("yes", &["oolong"]);
    s.expect("what drink do I like", &["oolong"]);
    s.finish(
        "Correction-driven rule induction & active learning",
        "Correcting Karin makes her propose a general rule; once approved it covers new cases, and she asks about things she doesn't know.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_selftests_pass() {
        let results = Engine::run_selftests();
        assert_eq!(results.len(), 11);
        for result in &results {
            assert!(
                result.passed,
                "selftest failed: {} - {}",
                result.name, result.detail
            );
        }
    }
}
