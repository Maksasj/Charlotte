//! Cold start ("pretraining"): a Claude teacher seeds the vault with simple,
//! true general knowledge, and a Claude reviewer checks what the agent
//! actually stored, because mistakes would be kept as truth.
//!
//! Per topic: teacher writes sentences in the parser's patterns -> the agent
//! reads them (source `teacher:pretrain`) -> the reviewer marks each stored
//! fact and rule OK or WRONG -> wrong facts are closed and wrong rules
//! rejected, with the reason in the audit log.

use crate::claude;
use sla_agent::Agent;
use sla_core::{GroundAtom, RuleStatus};
use std::fmt::Write as _;
use std::path::PathBuf;

pub const DEFAULT_TOPICS: &[&str] = &[
    "animals and their kinds, body parts and abilities",
    "food, drinks and their colours or tastes",
    "countries, cities and where they are",
    "household tools and objects and their parts",
    "jobs and what people in them do",
    "vehicles and their parts",
    "plants, trees and nature",
    "the human body",
    "rooms and furniture in a home",
    "weather and seasons",
];

const PATTERNS: &str = "Use ONLY these sentence patterns (one sentence per line, no numbering, no names of real private people):\n\
- category: 'A dog is an animal.' or 'Dogs are animals.'\n\
- parts/having: 'Birds have wings.' 'A car has wheels.'\n\
- colour/size/quality: 'Lemons are yellow.' 'Elephants are big.' 'Ice is cold.'\n\
- place: 'Paris is in France.' 'Berlin is in Germany.'\n\
- part of: 'A wheel is part of a car.'\n\
- habits with a plural subject: 'Cats like fish.' 'Bees make honey.' 'Birds fly.'\n\
- rules: 'If something is a bird then it has feathers.' 'If someone is a doctor then they help people.'\n\
- property: 'The colour of the sky is blue.'\n\
Each sentence must be short, plainly true in general, and contain one fact (no 'and', no 'but', no commas, no 'usually').";

#[derive(Debug, Clone, Default)]
pub struct TopicResult {
    pub topic: String,
    pub sentences: usize,
    pub understood: usize,
    pub facts_added: usize,
    pub rules_added: usize,
    pub rejected_facts: Vec<String>,
    pub rejected_rules: Vec<String>,
    pub not_understood: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PretrainConfig {
    pub vault: PathBuf,
    pub topics: Vec<String>,
    pub per_topic: usize,
    pub model: String,
    pub review: bool,
}

fn stored_facts(agent: &Agent) -> Vec<GroundAtom> {
    let now = agent.now();
    agent.kb.all_facts().into_iter().filter(|f| f.valid_at(now) && f.atom.pred != "proper_name").map(|f| f.atom).collect()
}

/// Run pretraining on the vault. Returns one result per topic.
pub fn pretrain(cfg: &PretrainConfig, mut progress: impl FnMut(&str)) -> std::io::Result<Vec<TopicResult>> {
    let mut agent = Agent::open(&cfg.vault, "maksim")?;
    agent.source = "teacher:pretrain".into();
    agent.ask_in_chat = false;
    let mut results = Vec::new();
    for topic in &cfg.topics {
        let mut res = TopicResult { topic: topic.clone(), ..Default::default() };
        progress(&format!("teaching: {topic}"));
        let prompt = format!(
            "You are a teacher giving a small symbolic learning agent its first general knowledge. \
             Write {} different true statements about: {topic}.\n{PATTERNS}\nOutput only the sentences.",
            cfg.per_topic
        );
        let text = match claude(&prompt, &cfg.model) {
            Ok(t) => t,
            Err(e) => {
                progress(&format!("  teacher failed: {e}"));
                results.push(res);
                continue;
            }
        };
        let facts_before = stored_facts(&agent);
        let rules_before: Vec<String> = agent.kb.vault.rules.keys().cloned().collect();
        for line in text.lines().map(|l| l.trim().trim_start_matches(['-', '*', ' ']).trim()).filter(|l| l.len() > 3) {
            res.sentences += 1;
            let reply = agent.respond(line);
            if reply.intent == "social" {
                res.not_understood.push(line.to_string());
            } else {
                res.understood += 1;
            }
        }
        let new_facts: Vec<GroundAtom> = stored_facts(&agent).into_iter().filter(|f| !facts_before.contains(f)).collect();
        let new_rules: Vec<String> = agent.kb.vault.rules.keys().filter(|k| !rules_before.contains(k)).cloned().collect();
        res.facts_added = new_facts.len();
        res.rules_added = new_rules.len();

        // Review: the reviewer sees what was stored, in plain English, and flags mistakes.
        if cfg.review && (!new_facts.is_empty() || !new_rules.is_empty()) {
            progress(&format!("  reviewing {} facts and {} rules", new_facts.len(), new_rules.len()));
            let mut items: Vec<String> = new_facts.iter().map(|f| agent.realizer.clause(f)).collect();
            let rule_texts: Vec<String> = new_rules
                .iter()
                .map(|id| agent.kb.vault.rules[id].rules().first().map(|r| agent.realizer.rule_text(r)).unwrap_or_default())
                .collect();
            items.extend(rule_texts.iter().cloned());
            let listing: String = items.iter().enumerate().map(|(i, t)| format!("{}. {t}\n", i + 1)).collect();
            let prompt = format!(
                "A simple learning agent stored these statements as general knowledge (the wording is mechanical; \
                 'you' means the user). Check each one for being true in general.\n\n{listing}\n\
                 Statements about a kind ('A is a cat', 'cats like fish') are general defaults: they are OK if they hold for \
                 typical members, even with exceptions (the agent learns exceptions later). \
                 For every number, output one line 'N: OK' or 'N: WRONG <short reason>'. Mark WRONG only if it is false \
                 for typical cases or garbled into a different meaning; awkward but true wording is OK.",
            );
            match claude(&prompt, &cfg.model) {
                Ok(out) => {
                    let now = agent.now();
                    for line in out.lines() {
                        let Some((n, verdict)) = line.split_once(':') else { continue };
                        let Ok(n) = n.trim().trim_start_matches('#').parse::<usize>() else { continue };
                        let verdict = verdict.trim();
                        if !verdict.to_uppercase().starts_with("WRONG") || n == 0 || n > items.len() {
                            continue;
                        }
                        let reason = format!("reviewer: {}", verdict.get(5..).unwrap_or("").trim());
                        if n <= new_facts.len() {
                            let f = &new_facts[n - 1];
                            agent.kb.close_matching(&f.to_atom(), now, &reason);
                            res.rejected_facts.push(format!("{} ({reason})", items[n - 1]));
                        } else {
                            let id = &new_rules[n - 1 - new_facts.len()];
                            if let Some(rf) = agent.kb.vault.rules.get_mut(id) {
                                let mut m = rf.meta();
                                m.status = RuleStatus::Rejected;
                                rf.set_meta(&m);
                                agent.kb.vault.mark_rule_dirty(id);
                                agent.kb.log_change(now, "reject_rule", id, &reason);
                            }
                            res.rejected_rules.push(format!("{} ({reason})", items[n - 1]));
                        }
                    }
                    agent.save()?;
                }
                Err(e) => progress(&format!("  reviewer failed: {e}")),
            }
        }
        progress(&format!(
            "  {} sentences, {} understood, +{} facts, +{} rules, rejected {} facts and {} rules",
            res.sentences,
            res.understood,
            res.facts_added,
            res.rules_added,
            res.rejected_facts.len(),
            res.rejected_rules.len()
        ));
        results.push(res);
    }
    agent.save()?;
    let now = agent.now();
    agent.kb.log_change(now, "pretrain", &format!("{} topics", results.len()), "cold start with a Claude teacher and reviewer");
    Ok(results)
}

pub fn report(results: &[TopicResult]) -> String {
    let mut s = String::from("# Pretraining report\n\n");
    let tot = |f: fn(&TopicResult) -> usize| results.iter().map(f).sum::<usize>();
    let _ = writeln!(
        s,
        "{} topics: {} sentences, {} understood, +{} facts, +{} rules; reviewer rejected {} facts and {} rules.\n",
        results.len(),
        tot(|r| r.sentences),
        tot(|r| r.understood),
        tot(|r| r.facts_added),
        tot(|r| r.rules_added),
        tot(|r| r.rejected_facts.len()),
        tot(|r| r.rejected_rules.len())
    );
    for r in results {
        let _ = writeln!(s, "## {}\n\n{} sentences, {} understood, +{} facts, +{} rules\n", r.topic, r.sentences, r.understood, r.facts_added, r.rules_added);
        for x in &r.rejected_facts {
            let _ = writeln!(s, "- rejected fact: {x}");
        }
        for x in &r.rejected_rules {
            let _ = writeln!(s, "- rejected rule: {x}");
        }
        for x in &r.not_understood {
            let _ = writeln!(s, "- not understood: {x}");
        }
        let _ = writeln!(s);
    }
    s
}

// ---------------------------------------------------------------- conversations

pub const CONVERSATION_TOPICS: &[&str] = &[
    "pets and farm animals", "wild animals", "birds", "fish and sea animals", "insects", "fruit", "vegetables", "drinks",
    "cooking and kitchens", "breakfast foods", "countries in Europe", "countries in Asia", "countries in the Americas and Africa",
    "big cities and where they are", "oceans, rivers and mountains", "the solar system", "weather", "seasons and months",
    "colours of common things", "shapes and sizes of common things", "tools and what they are for", "household objects and their parts",
    "furniture and rooms", "clothes", "cars and their parts", "bicycles, trains, ships and planes", "jobs in a hospital",
    "jobs in a school", "jobs in shops and restaurants", "sports", "music and instruments", "school subjects", "computers and phones",
    "materials like wood, metal, glass and plastic", "trees and flowers", "the human body", "health and illness", "family relations in general",
    "feelings in general", "time: days, weeks and years",
];

#[derive(Debug, Clone, Default)]
pub struct ConversationResult {
    pub topic: String,
    pub turns: usize,
    pub understood: usize,
    pub facts_added: usize,
    pub rules_added: usize,
    pub rejected: Vec<String>,
    pub transcript: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct ConversationConfig {
    pub vault: PathBuf,
    pub topics: Vec<String>,
    pub turns: usize,
    pub parallel: usize,
    pub model: String,
    pub review: bool,
    /// Natural conversation: everyday phrasing; the agent uses the LLM translator.
    pub natural: bool,
    /// Friends chatting instead of a teacher giving lessons (implies `natural`).
    pub chat: bool,
}

/// Everyday things two friends talk about. Facts come up in passing.
pub const CHAT_TOPICS: &[&str] = &[
    "cooking dinner and favourite foods", "what pets are like to live with", "a trip to the seaside", "the weather lately",
    "gardening and growing vegetables", "going hiking in the mountains", "how a city is different from the countryside",
    "football and other sports", "music and instruments", "what animals live in a forest", "breakfast around the world",
    "how cars and bikes work", "a visit to a zoo", "rainy days and what to do", "the night sky and planets", "baking bread and cakes",
    "fixing things around the house", "what jobs people do in a hospital", "coffee, tea and other drinks", "life on a farm",
    "fish and the ocean", "winter and snow", "summer and the heat", "trees and flowers in the park", "insects in the garden",
    "travelling by train and plane", "famous cities in Europe", "countries in Asia and their food", "rivers and lakes",
    "how the human body stays healthy", "colours and what they remind people of", "kitchen tools", "school subjects",
    "computers and phones", "birds you see in town", "clothes for different weather", "holidays and seasons",
    "what wood, metal and glass are used for", "deserts and hot places", "the moon and the sun",
];

fn chat_prompt(topic: &str, transcript: &[(String, String)]) -> String {
    let so_far = if transcript.is_empty() {
        "(you start the conversation)".to_string()
    } else {
        transcript.iter().map(|(u, a)| format!("You: {u}\nFriend: {a}")).collect::<Vec<_>>().join("\n")
    };
    format!(
        "You are having a relaxed, friendly chat with a friend. The topic drifts naturally around: {topic}.\n\
         Talk like a real person in a casual conversation: share things you know as they come up ('oh, I love \
         pears, they grow on trees, you know'), react to what your friend says, follow up on it, answer their \
         questions, and sometimes ask what they know or think. Do NOT quiz them or act like a teacher; it is a \
         conversation between equals. Your friend is still learning about the world and talks a bit stiffly; that's \
         fine, just keep chatting naturally and say things again more simply if they did not get it. Keep each \
         message to one to three short sentences. What you say about the world must be true and general (no \
         made-up stories about specific or private people; you may talk about yourself only in general terms).\n\n\
         Conversation so far:\n{so_far}\n\nWrite only your next message."
    )
}

fn natural_teacher_prompt(topic: &str, transcript: &[(String, String)]) -> String {
    let so_far = if transcript.is_empty() {
        "(nothing yet)".to_string()
    } else {
        transcript.iter().map(|(u, a)| format!("You: {u}\nAgent: {a}")).collect::<Vec<_>>().join("\n")
    };
    format!(
        "You are chatting with a curious young learning agent, like a friendly teacher or an older friend, about: {topic}.\n\
         Talk naturally, the way people really talk: everyday phrasing, small reactions ('oh nice', 'right'), follow-up \
         questions ('what else do you know about it?', 'what is a ...?'), sometimes two facts in one sentence. Keep each \
         message to one or two sentences. It is a beginner: teach it facts first, and only ask about facts you already \
         taught it; never ask for its opinion, feelings or guesses ('do you think...', 'who is better...'). Share only true, general facts (never about specific or private people). Now and \
         then check what it learned by asking it. If it gets something wrong, correct it naturally ('no, actually ...'). If it \
         asks you something, answer it. If it did not understand, say it again more simply.\n\n\
         Conversation so far:\n{so_far}\n\nWrite only your next message."
    )
}

fn teacher_prompt(topic: &str, transcript: &[(String, String)]) -> String {
    let so_far = if transcript.is_empty() {
        "(nothing yet)".to_string()
    } else {
        transcript.iter().map(|(u, a)| format!("Teacher: {u}\nAgent: {a}")).collect::<Vec<_>>().join("\n")
    };
    format!(
        "You are a patient teacher giving a simple learning agent its first general knowledge about: {topic}.\n\
         The agent only understands short simple sentences such as 'Dogs are animals.', 'A dog is an animal.', \
         'Birds have wings.', 'Lemons are yellow.', 'Paris is in France.', 'A wheel is part of a car.', 'Cats like fish.', \
         'Bees make honey.', 'If something is a bird then it has feathers.', 'What is a dog?', 'Does a bird have wings?', \
         'What do you know about birds?', 'Why?', 'No, a whale is a mammal.', 'yes', 'no', 'I don't know'.\n\
         Your rules: each message is one or two sentences, each at most 7 words, exactly like the examples: no \
         commas, no 'and', no 'but', no greetings, no explanations, no 'most' or 'usually'. Start teaching right away. \
         Teach true, general facts about kinds of things (never about specific or private people). Every few turns ask \
         a question to check what it learned. If its answer is wrong, correct it with 'No, ...'. If it asks you \
         something, answer with a simple true sentence, 'yes', 'no' or 'I don't know'. If it did not understand, say \
         the fact more simply. Don't repeat facts it already confirmed.\n\n\
         Conversation so far:\n{so_far}\n\nWrite only your next message."
    )
}

fn review_and_fix(
    agent: &mut Agent,
    lock: &std::sync::Mutex<()>,
    new_facts: &[GroundAtom],
    new_rules: &[String],
    model: &str,
    out: &mut Vec<String>,
) {
    if new_facts.is_empty() && new_rules.is_empty() {
        return;
    }
    let mut items: Vec<String> = new_facts.iter().map(|f| agent.realizer.clause(f)).collect();
    items.extend(new_rules.iter().map(|id| {
        agent.kb.vault.rules.get(id).and_then(|rf| rf.rules().first().map(|r| agent.realizer.rule_text(r))).unwrap_or_default()
    }));
    let listing: String = items.iter().enumerate().map(|(i, t)| format!("{}. {t}\n", i + 1)).collect();
    let prompt = format!(
        "A simple learning agent stored these statements as general knowledge (the wording is mechanical; 'you' means \
         the user). Statements about a kind are general defaults: OK if they hold for typical members, even with \
         exceptions.\n\n{listing}\nFor every number, output one line 'N: OK' or 'N: WRONG <short reason>'. Mark WRONG \
         only if false for typical cases or garbled into a different meaning."
    );
    let Ok(verdicts) = claude(&prompt, model) else { return };
    let _g = lock.lock().unwrap();
    let _ = agent.reload_if_edited();
    let now = agent.now();
    for line in verdicts.lines() {
        let Some((n, verdict)) = line.split_once(':') else { continue };
        let Ok(n) = n.trim().trim_start_matches('#').parse::<usize>() else { continue };
        let verdict = verdict.trim();
        if !verdict.to_uppercase().starts_with("WRONG") || n == 0 || n > items.len() {
            continue;
        }
        let reason = format!("reviewer: {}", verdict.get(5..).unwrap_or("").trim());
        if n <= new_facts.len() {
            agent.kb.close_matching(&new_facts[n - 1].to_atom(), now, &reason);
        } else {
            let id = &new_rules[n - 1 - new_facts.len()];
            if let Some(rf) = agent.kb.vault.rules.get_mut(id) {
                let mut m = rf.meta();
                m.status = RuleStatus::Rejected;
                rf.set_meta(&m);
                agent.kb.vault.mark_rule_dirty(id);
                agent.kb.log_change(now, "reject_rule", id, &reason);
            }
        }
        out.push(format!("{} ({reason})", items[n - 1]));
    }
    let _ = agent.save();
}

/// One teacher conversation on a shared vault. Messages to the agent are
/// serialized by `lock`; each agent session reloads the vault when another
/// session changed it, so all teachers build one body of knowledge.
fn one_conversation(cfg: &ConversationConfig, topic: &str, lock: &std::sync::Mutex<()>) -> ConversationResult {
    let mut res = ConversationResult { topic: topic.into(), ..Default::default() };
    // In chat mode the friend is a speaker of its own: what they say about
    // themselves ("I like soup") is about `friend`, never about the user.
    let speaker = if cfg.chat { "friend" } else { "maksim" };
    let mut agent = {
        let _g = lock.lock().unwrap();
        match Agent::open(&cfg.vault, speaker) {
            Ok(a) => a,
            Err(_) => return res,
        }
    };
    agent.source = "teacher:pretrain".into();
    if cfg.natural || cfg.chat {
        agent.enable_llm(&cfg.model);
    }
    if cfg.chat {
        agent.source = "teacher:chat".into();
    }
    let mut new_facts: Vec<GroundAtom> = Vec::new();
    let mut new_rules: Vec<String> = Vec::new();
    for _ in 0..cfg.turns {
        let prompt = if cfg.chat {
            chat_prompt(topic, &res.transcript)
        } else if cfg.natural {
            natural_teacher_prompt(topic, &res.transcript)
        } else {
            teacher_prompt(topic, &res.transcript)
        };
        let msg = match claude(&prompt, &cfg.model) {
            Ok(m) => m.lines().map(str::trim).find(|l| !l.is_empty()).unwrap_or("").trim_start_matches("Teacher:").trim_start_matches("You:").trim().trim_matches('"').to_string(),
            Err(_) => break,
        };
        if msg.is_empty() {
            break;
        }
        let reply = {
            let _g = lock.lock().unwrap();
            let _ = agent.reload_if_edited();
            let facts_before = stored_facts(&agent);
            let rules_before: Vec<String> = agent.kb.vault.rules.keys().cloned().collect();
            let reply = agent.respond(&msg);
            new_facts.extend(stored_facts(&agent).into_iter().filter(|f| !facts_before.contains(f)));
            new_rules.extend(agent.kb.vault.rules.keys().filter(|k| !rules_before.contains(k)).cloned());
            reply
        };
        res.turns += 1;
        if reply.intent != "social" {
            res.understood += 1;
        }
        res.transcript.push((msg, reply.text));
    }
    if cfg.chat {
        // Personal remarks of the chat friend are conversation, not general knowledge: close them.
        let personal = |g: &GroundAtom| g.args.iter().any(|v| v.as_sym().map_or(false, |x| x == "friend" || x.starts_with("my_")));
        let _g = lock.lock().unwrap();
        let _ = agent.reload_if_edited();
        let now = agent.now();
        for f in new_facts.iter().filter(|f| personal(f)) {
            agent.kb.close_matching(&f.to_atom(), now, "personal remark in a chat, not general knowledge");
        }
        let _ = agent.save();
        new_facts.retain(|f| !personal(f));
    }
    res.facts_added = new_facts.len();
    res.rules_added = new_rules.len();
    if cfg.review {
        review_and_fix(&mut agent, lock, &new_facts, &new_rules, &cfg.model, &mut res.rejected);
    }
    res
}

/// Run many teacher conversations, `parallel` at a time, on one vault.
pub fn converse(cfg: &ConversationConfig, progress: impl Fn(&str) + Sync) -> Vec<ConversationResult> {
    let queue = std::sync::Mutex::new(cfg.topics.iter().cloned().rev().collect::<Vec<_>>());
    let lock = std::sync::Mutex::new(());
    let results = std::sync::Mutex::new(Vec::new());
    std::thread::scope(|s| {
        for _ in 0..cfg.parallel.max(1) {
            s.spawn(|| loop {
                let Some(topic) = queue.lock().unwrap().pop() else { break };
                progress(&format!("start: {topic}"));
                let r = one_conversation(cfg, &topic, &lock);
                progress(&format!(
                    "done:  {topic}: {} turns, {} understood, +{} facts, +{} rules, {} rejected",
                    r.turns,
                    r.understood,
                    r.facts_added,
                    r.rules_added,
                    r.rejected.len()
                ));
                results.lock().unwrap().push(r);
            });
        }
    });
    let mut v = results.into_inner().unwrap();
    v.sort_by(|a, b| a.topic.cmp(&b.topic));
    v
}

pub fn conversation_report(results: &[ConversationResult]) -> String {
    let mut s = String::from("# Pretraining conversations\n\n");
    let sum = |f: fn(&ConversationResult) -> usize| results.iter().map(f).sum::<usize>();
    let _ = writeln!(
        s,
        "{} conversations: {} turns, {} understood, +{} facts, +{} rules; reviewer rejected {}.\n",
        results.len(),
        sum(|r| r.turns),
        sum(|r| r.understood),
        sum(|r| r.facts_added),
        sum(|r| r.rules_added),
        sum(|r| r.rejected.len())
    );
    for r in results {
        let _ = writeln!(s, "## {}\n\n{} turns, {} understood, +{} facts, +{} rules\n", r.topic, r.turns, r.understood, r.facts_added, r.rules_added);
        for x in &r.rejected {
            let _ = writeln!(s, "- rejected: {x}");
        }
        for (u, a) in &r.transcript {
            let _ = writeln!(s, "> **Teacher:** {u}  \n> **Agent:** {a}\n");
        }
    }
    s
}
