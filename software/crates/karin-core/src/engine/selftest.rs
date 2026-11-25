use super::Engine;
use crate::types::SelfTestResult;

impl Engine {
    pub fn run_selftests() -> Vec<SelfTestResult> {
        vec![
            test_persona_and_multiturn(),
            test_impasse_and_teaching(),
            test_anti_unification(),
            test_chain_chunking(),
        ]
    }
}

fn test_persona_and_multiturn() -> SelfTestResult {
    let mut eng = Engine::new();
    let t1 = eng.turn("who are you");
    let t2 = eng.turn("my name is Alex");
    let t3 = eng.turn("hello");
    let t4 = eng.turn("weather");
    let t5 = eng.turn("Paris");

    let passed = t1.reply.contains("Karin")
        && t2.reply.contains("Alex")
        && t3.reply.contains("Alex")
        && t4.reply.contains("Which city")
        && t5.reply.contains("partly cloudy");

    SelfTestResult {
        name: "Persona & multi-turn slot context".into(),
        passed,
        detail: format!("persona='{}' | multi-turn weather='{}'", t1.reply, t5.reply),
    }
}

fn test_impasse_and_teaching() -> SelfTestResult {
    let mut eng = Engine::new();
    let before = eng.turn("launch rockets");
    let (_, pat) = eng.teach_reply("launch rockets", "Ignition sequence started!", true);
    let after = eng.turn("launch rockets");

    let passed = before.impasse && !after.impasse && after.reply == "Ignition sequence started!";

    SelfTestResult {
        name: "Impasse detection & live teaching".into(),
        passed,
        detail: format!(
            "impasse={} -> taught {} -> '{}'",
            before.impasse, pat.id, after.reply
        ),
    }
}

fn test_anti_unification() -> SelfTestResult {
    let mut eng = Engine::new();
    let before = eng.turn("greet Clara");
    let (created, _) = eng.generalize_episodes(true);
    let after = eng.turn("greet Clara");

    let passed =
        before.impasse && !created.is_empty() && !after.impasse && after.reply.contains("Clara");

    SelfTestResult {
        name: "Anti-unification generalization".into(),
        passed,
        detail: format!(
            "induced {} pattern(s) -> unseen 'greet Clara' => '{}'",
            created.len(),
            after.reply
        ),
    }
}

fn test_chain_chunking() -> SelfTestResult {
    let mut eng = Engine::new();
    let before = eng.turn("calculate 6 * 7");
    let (chunked, _) = eng.chunk_last_chain(true);
    let after = eng.turn("calculate 9 * 9");

    let passed = before.reply.contains("42")
        && before.steps.len() == 3
        && chunked.is_some()
        && after.reply.contains("81")
        && after.steps.len() < before.steps.len();

    SelfTestResult {
        name: "Chain chunking & composition".into(),
        passed,
        detail: format!(
            "before={} steps ('{}') -> after={} steps ('{}')",
            before.steps.len(),
            before.reply,
            after.steps.len(),
            after.reply
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_selftests_pass() {
        let results = Engine::run_selftests();
        assert_eq!(results.len(), 4);
        for result in &results {
            assert!(
                result.passed,
                "selftest failed: {} — {}",
                result.name, result.detail
            );
        }
    }
}
