use super::Engine;
use crate::types::SelfTestResult;

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

fn test_knowledge_graph_inheritance() -> SelfTestResult {
    let mut eng = Engine::new();
    let t1 = eng.turn("is Rust a tool");
    let t2 = eng.turn("what is the purpose of Rust");
    let t3 = eng.turn("Go is a programming language");
    let t4 = eng.turn("what is the purpose of Go");

    let passed = t1.reply.contains("Yes, Rust is a tool")
        && t2.reply.contains("writing software")
        && t3.reply.contains("Learned: Go is a programming language")
        && t4.reply.contains("writing software");

    SelfTestResult {
        name: "Knowledge graph & transitive inheritance".into(),
        passed,
        detail: format!(
            "hop='{}' | inherited='{}' | taught+inherited='{}'",
            t1.reply, t2.reply, t4.reply
        ),
    }
}

fn test_shared_slot_filling_and_intents() -> SelfTestResult {
    let mut eng = Engine::new();
    let t1 = eng.turn("forecast for Tokyo");
    let t2 = eng.turn("compute 5 * 8");
    let t3 = eng.turn("define");
    let t4 = eng.turn("symbolic AI");
    let t5 = eng.turn("weather");
    let t6 = eng.turn("cancel");

    let passed = t1.reply.contains("Tokyo")
        && t2.reply.contains("40")
        && t3.reply.contains("define")
        && t4.reply.contains("explicit facts")
        && t5.reply.contains("Which city")
        && t6.reply.contains("cancelled");

    SelfTestResult {
        name: "Shared slot-filling & canonical intents".into(),
        passed,
        detail: format!(
            "paraphrase='{}' | slot='{}' | cancel='{}'",
            t1.reply, t4.reply, t6.reply
        ),
    }
}

fn test_data_driven_unit_conversions() -> SelfTestResult {
    let mut eng = Engine::new();
    let t1 = eng.turn("convert 2 km to m");
    let t2 = eng.turn("3 h in min");
    let t3 = eng.turn("convert 5 km to kg");

    let passed = t1.reply.contains("2000 m")
        && t2.reply.contains("180 min")
        && t3.reply.contains("incompatible dimensions");

    SelfTestResult {
        name: "Data-driven unit conversions".into(),
        passed,
        detail: format!(
            "km->m='{}' | h->min='{}' | mismatch='{}'",
            t1.reply, t2.reply, t3.reply
        ),
    }
}

fn test_typed_relations_and_inverses() -> SelfTestResult {
    let mut eng = Engine::new();
    let t1 = eng.turn("what does France contain");
    let t2 = eng.turn("Go was created by Google");
    let t3 = eng.turn("what did Google create");

    let passed = t1.reply.contains("Paris")
        && t1.reply.contains("inverse")
        && t2.reply.contains("Learned: Go was created by Google")
        && t3.reply.contains("Go")
        && t3.reply.contains("inverse");

    SelfTestResult {
        name: "Typed relations & automatic inverses".into(),
        passed,
        detail: format!("seed_inv='{}' | taught_inv='{}'", t1.reply, t3.reply),
    }
}

fn test_generic_workflows() -> SelfTestResult {
    let mut eng = Engine::new();
    let t1 = eng.turn("plan a trip");
    let t2 = eng.turn("Tokyo");
    let t3 = eng.turn("5");
    let t4 = eng.turn("plan a trip to Paris");
    let t5 = eng.turn("3");

    let passed = t1.reply.contains("Which city")
        && t2.reply.contains("How many days")
        && t3.reply.contains("5 days in Tokyo")
        && t3.reply.contains("clear skies")
        && t4.reply.contains("How many days")
        && t5.reply.contains("3 days in Paris");

    SelfTestResult {
        name: "Generic multi-step workflows".into(),
        passed,
        detail: format!("full='{}' | prefilled='{}'", t3.reply, t5.reply),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_selftests_pass() {
        let results = Engine::run_selftests();
        assert_eq!(results.len(), 9);
        for result in &results {
            assert!(
                result.passed,
                "selftest failed: {} — {}",
                result.name, result.detail
            );
        }
    }
}
