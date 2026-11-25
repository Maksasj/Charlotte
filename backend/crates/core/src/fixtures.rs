use crate::types::{
    Action, AgentState, Condition, Episode, Fact, Pattern, PatternKind, PatternStatus, Scope,
};
use std::collections::BTreeMap;

fn map(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
    pairs
        .iter()
        .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
        .collect()
}

pub fn initial_state() -> AgentState {
    let facts = vec![
        // Persona (long-term facts)
        Fact {
            id: "f1".into(),
            rel: "persona_name".into(),
            args: map(&[("value", "Karin")]),
            scope: Scope::LongTerm,
            provenance: "seed:persona".into(),
            turn: 0,
        },
        Fact {
            id: "f2".into(),
            rel: "persona_role".into(),
            args: map(&[("value", "a purely symbolic, self-improving assistant")]),
            scope: Scope::LongTerm,
            provenance: "seed:persona".into(),
            turn: 0,
        },
        Fact {
            id: "f3".into(),
            rel: "persona_tone".into(),
            args: map(&[("value", "direct, curious, and transparent")]),
            scope: Scope::LongTerm,
            provenance: "seed:persona".into(),
            turn: 0,
        },
        // Domain weather facts
        Fact {
            id: "f4".into(),
            rel: "weather_city".into(),
            args: map(&[("city", "Paris"), ("condition", "partly cloudy"), ("temp", "18°C")]),
            scope: Scope::LongTerm,
            provenance: "seed:world".into(),
            turn: 0,
        },
        Fact {
            id: "f5".into(),
            rel: "weather_city".into(),
            args: map(&[("city", "Tokyo"), ("condition", "clear skies"), ("temp", "22°C")]),
            scope: Scope::LongTerm,
            provenance: "seed:world".into(),
            turn: 0,
        },
        Fact {
            id: "f6".into(),
            rel: "weather_city".into(),
            args: map(&[("city", "London"), ("condition", "light rain"), ("temp", "13°C")]),
            scope: Scope::LongTerm,
            provenance: "seed:world".into(),
            turn: 0,
        },
        Fact {
            id: "f7".into(),
            rel: "weather_city".into(),
            args: map(&[("city", "Vilnius"), ("condition", "crisp and sunny"), ("temp", "9°C")]),
            scope: Scope::LongTerm,
            provenance: "seed:world".into(),
            turn: 0,
        },
        // Definitions
        Fact {
            id: "f8".into(),
            rel: "definition".into(),
            args: map(&[
                ("topic", "symbolic ai"),
                (
                    "text",
                    "AI built from explicit facts, rules, and reasoning steps rather than neural weights.",
                ),
            ]),
            scope: Scope::LongTerm,
            provenance: "seed:knowledge".into(),
            turn: 0,
        },
        Fact {
            id: "f9".into(),
            rel: "definition".into(),
            args: map(&[
                ("topic", "anti-unification"),
                (
                    "text",
                    "Finding the most specific general template that covers two or more concrete examples.",
                ),
            ]),
            scope: Scope::LongTerm,
            provenance: "seed:knowledge".into(),
            turn: 0,
        },
        Fact {
            id: "f10".into(),
            rel: "definition".into(),
            args: map(&[
                ("topic", "chunking"),
                (
                    "text",
                    "Compiling a successful multi-step chain of patterns into one reusable composite pattern.",
                ),
            ]),
            scope: Scope::LongTerm,
            provenance: "seed:knowledge".into(),
            turn: 0,
        },
    ];

    let patterns = vec![
        // --- Social & Persona ---
        Pattern {
            id: "p1".into(),
            name: "greet-known-user".into(),
            status: PatternStatus::Active,
            when: vec![
                Condition::Text { template: "hello".into() },
                Condition::Fact { rel: "user_name".into(), args: map(&[("value", "?u")]) },
                Condition::Fact { rel: "persona_name".into(), args: map(&[("value", "?me")]) },
            ],
            then: vec![Action::Reply {
                template: "Hello ?u! I'm ?me. What shall we work on?".into(),
            }],
            trust: 0.95,
            support: 5,
            provenance: "seed".into(),
            kind: PatternKind::Reply,
        },
        Pattern {
            id: "p2".into(),
            name: "greet-default".into(),
            status: PatternStatus::Active,
            when: vec![
                Condition::Text { template: "hello".into() },
                Condition::NotFact { rel: "user_name".into(), args: map(&[]) },
                Condition::Fact { rel: "persona_name".into(), args: map(&[("value", "?me")]) },
            ],
            then: vec![Action::Reply {
                template: "Hello! I'm ?me. Ask me for weather, math, definitions, notes, or teach me something new.".into(),
            }],
            trust: 0.9,
            support: 5,
            provenance: "seed".into(),
            kind: PatternKind::Reply,
        },
        Pattern {
            id: "p3".into(),
            name: "greet-hi".into(),
            status: PatternStatus::Active,
            when: vec![
                Condition::Text { template: "hi".into() },
                Condition::Fact { rel: "persona_name".into(), args: map(&[("value", "?me")]) },
            ],
            then: vec![Action::Reply {
                template: "Hi! I'm ?me — ready when you are.".into(),
            }],
            trust: 0.9,
            support: 4,
            provenance: "seed".into(),
            kind: PatternKind::Reply,
        },
        Pattern {
            id: "p4".into(),
            name: "thanks".into(),
            status: PatternStatus::Active,
            when: vec![Condition::Contains { phrase: "thank".into() }],
            then: vec![Action::Reply {
                template: "You're welcome! Let me know if there's anything else.".into(),
            }],
            trust: 0.85,
            support: 4,
            provenance: "seed".into(),
            kind: PatternKind::Reply,
        },
        Pattern {
            id: "p5".into(),
            name: "goodbye".into(),
            status: PatternStatus::Active,
            when: vec![Condition::Text { template: "bye".into() }],
            then: vec![Action::Reply {
                template: "Goodbye! All long-term facts and patterns stay saved.".into(),
            }],
            trust: 0.9,
            support: 3,
            provenance: "seed".into(),
            kind: PatternKind::Reply,
        },
        Pattern {
            id: "p6".into(),
            name: "who-are-you".into(),
            status: PatternStatus::Active,
            when: vec![
                Condition::Text { template: "who are you".into() },
                Condition::Fact { rel: "persona_name".into(), args: map(&[("value", "?me")]) },
                Condition::Fact { rel: "persona_role".into(), args: map(&[("value", "?role")]) },
                Condition::Fact { rel: "persona_tone".into(), args: map(&[("value", "?tone")]) },
            ],
            then: vec![Action::Reply {
                template: "I'm ?me, ?role. My style is ?tone.".into(),
            }],
            trust: 0.95,
            support: 5,
            provenance: "seed".into(),
            kind: PatternKind::Reply,
        },
        // --- User memory ---
        Pattern {
            id: "p7".into(),
            name: "learn-user-name".into(),
            status: PatternStatus::Active,
            when: vec![Condition::Text { template: "my name is ?name".into() }],
            then: vec![
                Action::Retract { rel: "user_name".into(), args: None },
                Action::Assert {
                    rel: "user_name".into(),
                    args: map(&[("value", "?name")]),
                    scope: Scope::LongTerm,
                },
                Action::Reply {
                    template: "Nice to meet you, ?name! I've stored your name in long-term memory.".into(),
                },
            ],
            trust: 0.95,
            support: 5,
            provenance: "seed".into(),
            kind: PatternKind::Language,
        },
        Pattern {
            id: "p8".into(),
            name: "recall-user-name".into(),
            status: PatternStatus::Active,
            when: vec![
                Condition::Text { template: "what is my name".into() },
                Condition::Fact { rel: "user_name".into(), args: map(&[("value", "?name")]) },
            ],
            then: vec![Action::Reply {
                template: "Your name is ?name.".into(),
            }],
            trust: 0.95,
            support: 5,
            provenance: "seed".into(),
            kind: PatternKind::Reply,
        },
        Pattern {
            id: "p9".into(),
            name: "recall-user-name-unknown".into(),
            status: PatternStatus::Active,
            when: vec![
                Condition::Text { template: "what is my name".into() },
                Condition::NotFact { rel: "user_name".into(), args: map(&[]) },
            ],
            then: vec![Action::Reply {
                template: "You haven't told me your name yet. Say \"my name is ...\" and I'll remember it.".into(),
            }],
            trust: 0.9,
            support: 3,
            provenance: "seed".into(),
            kind: PatternKind::Reply,
        },
        // --- Weather 3-step composable chain + multi-turn slot asking ---
        Pattern {
            id: "p10".into(),
            name: "parse-weather-city".into(),
            status: PatternStatus::Active,
            when: vec![
                Condition::Text { template: "weather in ?city".into() },
                Condition::NotFact { rel: "want_weather".into(), args: map(&[]) },
            ],
            then: vec![Action::Assert {
                rel: "want_weather".into(),
                args: map(&[("city", "?city")]),
                scope: Scope::Working,
            }],
            trust: 0.9,
            support: 5,
            provenance: "seed".into(),
            kind: PatternKind::Language,
        },
        Pattern {
            id: "p11".into(),
            name: "parse-weather-no-city".into(),
            status: PatternStatus::Active,
            when: vec![
                Condition::Text { template: "weather".into() },
                Condition::NotFact { rel: "want_weather".into(), args: map(&[]) },
            ],
            then: vec![
                Action::Assert {
                    rel: "asked_slot".into(),
                    args: map(&[("slot", "city"), ("goal", "weather")]),
                    scope: Scope::Working,
                },
                Action::Reply {
                    template: "Which city would you like the weather for? (I know Paris, Tokyo, London, and Vilnius.)".into(),
                },
            ],
            trust: 0.9,
            support: 4,
            provenance: "seed".into(),
            kind: PatternKind::Language,
        },
        Pattern {
            id: "p12".into(),
            name: "bind-asked-city-for-weather".into(),
            status: PatternStatus::Active,
            when: vec![
                Condition::Fact {
                    rel: "asked_slot".into(),
                    args: map(&[("slot", "city"), ("goal", "weather")]),
                },
                Condition::Text { template: "?city".into() },
                Condition::NotFact { rel: "want_weather".into(), args: map(&[]) },
            ],
            then: vec![
                Action::Retract { rel: "asked_slot".into(), args: None },
                Action::Assert {
                    rel: "want_weather".into(),
                    args: map(&[("city", "?city")]),
                    scope: Scope::Working,
                },
            ],
            trust: 0.85,
            support: 4,
            provenance: "seed".into(),
            kind: PatternKind::Language,
        },
        Pattern {
            id: "p13".into(),
            name: "run-weather-tool".into(),
            status: PatternStatus::Active,
            when: vec![
                Condition::Fact { rel: "want_weather".into(), args: map(&[("city", "?city")]) },
                Condition::NotFact { rel: "weather_report".into(), args: map(&[]) },
                Condition::NotFact { rel: "weather_unknown".into(), args: map(&[]) },
            ],
            then: vec![Action::Tool {
                name: "weather".into(),
                args: map(&[("city", "?city")]),
            }],
            trust: 0.92,
            support: 5,
            provenance: "seed".into(),
            kind: PatternKind::Reasoning,
        },
        Pattern {
            id: "p14".into(),
            name: "reply-weather-report".into(),
            status: PatternStatus::Active,
            when: vec![Condition::Fact {
                rel: "weather_report".into(),
                args: map(&[("city", "?city"), ("condition", "?cond"), ("temp", "?temp")]),
            }],
            then: vec![Action::Reply {
                template: "Weather in ?city: ?cond, ?temp.".into(),
            }],
            trust: 0.92,
            support: 5,
            provenance: "seed".into(),
            kind: PatternKind::Reply,
        },
        Pattern {
            id: "p15".into(),
            name: "reply-weather-unknown".into(),
            status: PatternStatus::Active,
            when: vec![Condition::Fact {
                rel: "weather_unknown".into(),
                args: map(&[("city", "?city")]),
            }],
            then: vec![Action::Reply {
                template: "I don't have a weather fact for ?city yet. You can add a weather_city fact in the Facts tab.".into(),
            }],
            trust: 0.9,
            support: 3,
            provenance: "seed".into(),
            kind: PatternKind::Reply,
        },
        // --- Calculator 3-step composable chain ---
        Pattern {
            id: "p16".into(),
            name: "parse-calculate".into(),
            status: PatternStatus::Active,
            when: vec![
                Condition::Text { template: "calculate ?expr".into() },
                Condition::NotFact { rel: "want_calc".into(), args: map(&[]) },
            ],
            then: vec![Action::Assert {
                rel: "want_calc".into(),
                args: map(&[("expr", "?expr")]),
                scope: Scope::Working,
            }],
            trust: 0.92,
            support: 5,
            provenance: "seed".into(),
            kind: PatternKind::Language,
        },
        Pattern {
            id: "p17".into(),
            name: "run-calc-tool".into(),
            status: PatternStatus::Active,
            when: vec![
                Condition::Fact { rel: "want_calc".into(), args: map(&[("expr", "?expr")]) },
                Condition::NotFact { rel: "calc_result".into(), args: map(&[]) },
                Condition::NotFact { rel: "calc_error".into(), args: map(&[]) },
            ],
            then: vec![Action::Tool {
                name: "calc".into(),
                args: map(&[("expr", "?expr")]),
            }],
            trust: 0.92,
            support: 5,
            provenance: "seed".into(),
            kind: PatternKind::Reasoning,
        },
        Pattern {
            id: "p18".into(),
            name: "reply-calc-result".into(),
            status: PatternStatus::Active,
            when: vec![Condition::Fact {
                rel: "calc_result".into(),
                args: map(&[("expr", "?expr"), ("value", "?val")]),
            }],
            then: vec![Action::Reply {
                template: "?expr = ?val".into(),
            }],
            trust: 0.92,
            support: 5,
            provenance: "seed".into(),
            kind: PatternKind::Reply,
        },
        Pattern {
            id: "p19".into(),
            name: "reply-calc-error".into(),
            status: PatternStatus::Active,
            when: vec![Condition::Fact {
                rel: "calc_error".into(),
                args: map(&[("expr", "?expr"), ("error", "?err")]),
            }],
            then: vec![Action::Reply {
                template: "Couldn't evaluate \"?expr\": ?err.".into(),
            }],
            trust: 0.9,
            support: 3,
            provenance: "seed".into(),
            kind: PatternKind::Reply,
        },
        // --- Definitions ---
        Pattern {
            id: "p20".into(),
            name: "parse-define".into(),
            status: PatternStatus::Active,
            when: vec![
                Condition::Text { template: "define ?topic".into() },
                Condition::NotFact { rel: "definition_result".into(), args: map(&[]) },
                Condition::NotFact { rel: "definition_unknown".into(), args: map(&[]) },
            ],
            then: vec![Action::Tool {
                name: "define".into(),
                args: map(&[("topic", "?topic")]),
            }],
            trust: 0.9,
            support: 4,
            provenance: "seed".into(),
            kind: PatternKind::Language,
        },
        Pattern {
            id: "p21".into(),
            name: "reply-definition".into(),
            status: PatternStatus::Active,
            when: vec![Condition::Fact {
                rel: "definition_result".into(),
                args: map(&[("topic", "?topic"), ("text", "?text")]),
            }],
            then: vec![Action::Reply {
                template: "?topic: ?text".into(),
            }],
            trust: 0.9,
            support: 4,
            provenance: "seed".into(),
            kind: PatternKind::Reply,
        },
        Pattern {
            id: "p22".into(),
            name: "reply-definition-unknown".into(),
            status: PatternStatus::Active,
            when: vec![Condition::Fact {
                rel: "definition_unknown".into(),
                args: map(&[("topic", "?topic")]),
            }],
            then: vec![Action::Reply {
                template: "I don't have a definition for \"?topic\" yet. Add a `definition` fact or teach me!".into(),
            }],
            trust: 0.85,
            support: 2,
            provenance: "seed".into(),
            kind: PatternKind::Reply,
        },
        // --- Notes ---
        Pattern {
            id: "p23".into(),
            name: "save-note".into(),
            status: PatternStatus::Active,
            when: vec![
                Condition::Text { template: "remember ?text".into() },
                Condition::NotFact { rel: "note_saved".into(), args: map(&[]) },
            ],
            then: vec![Action::Tool {
                name: "note_save".into(),
                args: map(&[("text", "?text")]),
            }],
            trust: 0.9,
            support: 4,
            provenance: "seed".into(),
            kind: PatternKind::Language,
        },
        Pattern {
            id: "p24".into(),
            name: "reply-note-saved".into(),
            status: PatternStatus::Active,
            when: vec![Condition::Fact {
                rel: "note_saved".into(),
                args: map(&[("text", "?text")]),
            }],
            then: vec![Action::Reply {
                template: "Saved to long-term memory: \"?text\".".into(),
            }],
            trust: 0.9,
            support: 4,
            provenance: "seed".into(),
            kind: PatternKind::Reply,
        },
        Pattern {
            id: "p25".into(),
            name: "list-notes".into(),
            status: PatternStatus::Active,
            when: vec![
                Condition::Text { template: "my notes".into() },
                Condition::NotFact { rel: "notes_listing".into(), args: map(&[]) },
            ],
            then: vec![Action::Tool {
                name: "note_list".into(),
                args: map(&[]),
            }],
            trust: 0.9,
            support: 4,
            provenance: "seed".into(),
            kind: PatternKind::Language,
        },
        Pattern {
            id: "p26".into(),
            name: "reply-notes-listing".into(),
            status: PatternStatus::Active,
            when: vec![Condition::Fact {
                rel: "notes_listing".into(),
                args: map(&[("list", "?list")]),
            }],
            then: vec![Action::Reply {
                template: "Your notes: ?list".into(),
            }],
            trust: 0.9,
            support: 4,
            provenance: "seed".into(),
            kind: PatternKind::Reply,
        },
        // --- Meta-tools callable through normal conversation ---
        Pattern {
            id: "p27".into(),
            name: "meta-generalize-command".into(),
            status: PatternStatus::Active,
            when: vec![
                Condition::Text { template: "generalize".into() },
                Condition::NotFact { rel: "meta_result".into(), args: map(&[]) },
            ],
            then: vec![Action::Tool {
                name: "generalize_episodes".into(),
                args: map(&[]),
            }],
            trust: 0.95,
            support: 3,
            provenance: "seed:meta".into(),
            kind: PatternKind::Reasoning,
        },
        Pattern {
            id: "p28".into(),
            name: "meta-chunk-command".into(),
            status: PatternStatus::Active,
            when: vec![
                Condition::Text { template: "chunk last".into() },
                Condition::NotFact { rel: "meta_result".into(), args: map(&[]) },
            ],
            then: vec![Action::Tool {
                name: "chunk_last_chain".into(),
                args: map(&[]),
            }],
            trust: 0.95,
            support: 3,
            provenance: "seed:meta".into(),
            kind: PatternKind::Reasoning,
        },
        Pattern {
            id: "p29".into(),
            name: "reply-meta-result".into(),
            status: PatternStatus::Active,
            when: vec![Condition::Fact {
                rel: "meta_result".into(),
                args: map(&[("summary", "?summary")]),
            }],
            then: vec![Action::Reply {
                template: "?summary".into(),
            }],
            trust: 0.95,
            support: 3,
            provenance: "seed:meta".into(),
            kind: PatternKind::Reply,
        },
    ];

    // Seed episodes ready for anti-unification demonstration
    let episodes = vec![
        Episode {
            id: "e1".into(),
            input: "greet Alice".into(),
            reply: "Warm greetings, Alice! Glad you're here.".into(),
            source: "seed".into(),
            generalized: false,
        },
        Episode {
            id: "e2".into(),
            input: "greet Bob".into(),
            reply: "Warm greetings, Bob! Glad you're here.".into(),
            source: "seed".into(),
            generalized: false,
        },
        Episode {
            id: "e3".into(),
            input: "praise Rust".into(),
            reply: "Rust is genuinely impressive — clean, fast, and reliable.".into(),
            source: "seed".into(),
            generalized: false,
        },
        Episode {
            id: "e4".into(),
            input: "praise Prolog".into(),
            reply: "Prolog is genuinely impressive — clean, fast, and reliable.".into(),
            source: "seed".into(),
            generalized: false,
        },
    ];

    AgentState {
        turn: 0,
        facts,
        patterns,
        episodes,
        chains: Vec::new(),
        messages: Vec::new(),
        next_id: 100,
    }
}
