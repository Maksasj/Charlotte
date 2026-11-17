//! Design 2.0, milestone 12 ("honest measurement and safe ground"):
//! the v1 correctness fixes and the shared, single-writer brain.

use sla_agent::{brain, Agent, Brain};
use sla_core::{parse_ground, parse_rule, Fact, RuleFile, RuleMeta, RuleStatus, Timestamp};
use std::fs;
use std::sync::Arc;

fn temp_vault(name: &str) -> std::path::PathBuf {
    let d = std::env::temp_dir().join(format!("sla_m12_{name}_{}", std::process::id()));
    let _ = fs::remove_dir_all(&d);
    d
}

fn talk(agent: &mut Agent, msg: &str) -> String {
    let r = agent.respond(msg).text;
    println!("> {msg}\n{r}");
    r
}

/// v1: one bad rule made the agent evaluate with no rules at all, so core
/// inheritance vanished. 2.0: the bad rule is set aside and reported; core
/// and good learned rules keep working.
#[test]
fn bad_rule_is_quarantined_not_everything() {
    let root = temp_vault("quarantine");
    let mut a = Agent::open(&root, "maksim").unwrap();
    a.ask_in_chat = false;
    a.clock = Some(Timestamp::from_ymd_hms(2026, 11, 17, 9, 0, 0));
    talk(&mut a, "Rex is a dog.");
    talk(&mut a, "Dogs are animals.");
    talk(&mut a, "If someone likes tea then they want tea.");
    talk(&mut a, "Anna likes tea.");
    // A hand edit adds a rule with negation through recursion.
    let bad = "---\nid: broken\nstatus: confirmed\nconf: 1\nevidence: 0\ncounter: 0\nmodule: general\nsource: hand\n---\n# broken\n\n```datalog\nodd(X) :- is_a(X, dog), not odd(X).\n```\n";
    fs::write(root.join("rules").join("broken.md"), bad).unwrap();
    let r = talk(&mut a, "Is Rex an animal?");
    assert!(r.starts_with("Yes") || r.contains("Rex is an animal"), "core inheritance still works: {r}");
    let r = talk(&mut a, "Does Anna want tea?");
    assert!(r.contains("Anna wants tea"), "good learned rules still work: {r}");
    let b = a.brain();
    assert!(b.quarantined.keys().any(|k| k.starts_with("broken#")), "{:?}", b.quarantined);
    let log = b.kb.vault.read_log().unwrap();
    assert_eq!(log.iter().filter(|(_, act, s, _)| act == "quarantine" && s.starts_with("broken#")).count(), 1, "reported once");
    drop(b);
    let _ = fs::remove_dir_all(&root);
}

/// v1 ran 3 to 5 full evaluations per message. 2.0 shares one model per
/// state of the vault: a question costs at most one evaluation, and none when
/// nothing changed since the last one.
#[test]
fn one_model_per_message() {
    let root = temp_vault("onemodel");
    let mut a = Agent::open(&root, "maksim").unwrap();
    a.clock = Some(Timestamp::from_ymd_hms(2026, 11, 17, 9, 0, 0));
    for m in ["Rex is a dog.", "Dogs are animals.", "Anna likes tea.", "Bob likes tea.", "Anna lives in Berlin."] {
        talk(&mut a, m);
        let t = a.last_trace.as_ref().unwrap();
        assert!(t.evaluations <= 2, "a statement: learner + one shared model, got {}", t.evaluations);
    }
    talk(&mut a, "Is Rex an animal?");
    assert!(a.last_trace.as_ref().unwrap().evaluations <= 1);
    for q in ["Does Anna like tea?", "Who likes tea?", "What do you know about Anna?", "Why?"] {
        talk(&mut a, q);
        let t = a.last_trace.as_ref().unwrap();
        assert_eq!(t.evaluations, 0, "{q}: nothing changed, the model is reused");
    }
    let _ = fs::remove_dir_all(&root);
}

/// v1 evaluated "used to" questions over every fact ever stored at once, so
/// a rule could join premises that were never true together. 2.0 evaluates
/// the past moment by moment.
#[test]
fn history_never_joins_different_times() {
    let root = temp_vault("history");
    let t1 = Timestamp::from_ymd(2026, 1, 1);
    let t2 = Timestamp::from_ymd(2026, 2, 1);
    let t3 = Timestamp::from_ymd(2026, 3, 1);
    let now = Timestamp::from_ymd(2026, 4, 1);
    let mut a = Agent::open(&root, "maksim").unwrap();
    a.ask_in_chat = false;
    a.clock = Some(now);
    {
        let mut b = a.brain();
        let f = |s: &str| Fact::new(parse_ground(s).unwrap(), "test");
        // Anna liked tea in January only; she lives in Paris from March on.
        b.kb.vault.add_fact(f("likes(anna, tea)").with_from(t1).with_until(t2));
        b.kb.vault.add_fact(f("located_in(anna, paris)").with_from(t3));
        // Bob liked tea while he lived in Paris (both in January).
        b.kb.vault.add_fact(f("likes(bob, tea)").with_from(t1).with_until(t2));
        b.kb.vault.add_fact(f("located_in(bob, paris)").with_from(t1).with_until(t2));
        let meta = RuleMeta { id: "paris_tea".into(), status: RuleStatus::Confirmed, conf: 1.0, evidence: 0, counter: 0, module: "general".into(), source: "test".into() };
        b.kb.vault.put_rule_file(RuleFile::new(&meta, "Tea lovers in Paris want tea.", &[parse_rule("wants(X, tea) :- likes(X, tea), located_in(X, paris).").unwrap()]));
        assert!(!b.was_true(&parse_ground("wants(anna, tea)").unwrap(), now), "never true at one moment");
        assert!(b.was_true(&parse_ground("wants(bob, tea)").unwrap(), now), "true in January");
    }
    let r = talk(&mut a, "Did Anna want tea?");
    assert!(!r.contains("used to"), "{r}");
    let r = talk(&mut a, "Did Bob want tea?");
    assert!(r.contains("used to"), "{r}");
    let _ = fs::remove_dir_all(&root);
}

/// "Used to" now goes through contradiction handling: the current fact is closed.
#[test]
fn used_to_ends_the_current_fact() {
    let root = temp_vault("usedto");
    let t = Timestamp::from_ymd_hms(2026, 11, 1, 9, 0, 0);
    let mut a = Agent::open(&root, "maksim").unwrap();
    a.ask_in_chat = false;
    a.clock = Some(t);
    talk(&mut a, "Anna likes tea.");
    a.clock = Some(t.plus_days(3));
    talk(&mut a, "Anna used to like tea.");
    a.clock = Some(t.plus_days(4));
    let r = talk(&mut a, "Does Anna like tea?");
    // Content is right ("used to ... but not anymore"); the leading "Yes" of
    // this template is a wording problem left to milestone 16.
    assert!(r.to_lowercase().contains("not anymore") && !r.starts_with("Yes"), "{r}");
    let r = talk(&mut a, "Did Anna like tea?");
    assert!(r.contains("used to"), "{r}");
    let b = a.brain();
    let open: Vec<_> = b.kb.all_facts().into_iter().filter(|f| f.atom.to_string() == "likes(anna, tea)" && f.is_open()).collect();
    assert!(open.is_empty(), "no current copy left: {open:?}");
    drop(b);
    let _ = fs::remove_dir_all(&root);
}

/// Parallel conversations share one brain: every fact any of them taught is
/// known to all and saved once, and every log line stays whole.
#[test]
fn parallel_conversations_share_one_vault() {
    let root = temp_vault("parallel");
    let shared = Brain::shared(&root).unwrap();
    let threads: Vec<_> = (0..8)
        .map(|i| {
            let b = Arc::clone(&shared);
            std::thread::spawn(move || {
                let mut a = Agent::with_brain(b, "maksim");
                a.ask_in_chat = false;
                for j in 0..15 {
                    a.respond(&format!("Person{i}x{j} likes tea."));
                    a.respond(&format!("Does person{i}x{j} like tea?"));
                }
            })
        })
        .collect();
    for t in threads {
        t.join().unwrap();
    }
    // Everyone sees everything, in memory and after a restart.
    let mut a = Agent::with_brain(Arc::clone(&shared), "maksim");
    let r = a.respond("Does person7x14 like tea?").text;
    assert!(r.contains("likes tea"), "{r}");
    drop(a);
    drop(shared);
    let b = Brain::open(&root).unwrap();
    let likes = b.kb.all_facts().into_iter().filter(|f| f.atom.pred == "likes").count();
    assert_eq!(likes, 8 * 15);
    for (file, line, err) in b.kb.vault.errors() {
        panic!("unreadable vault line in {file}: {line} ({err})");
    }
    let mut lines = 0;
    for e in fs::read_dir(root.join("log")).unwrap().flatten() {
        for l in fs::read_to_string(e.path()).unwrap().lines() {
            if l.starts_with("- ") {
                lines += 1;
                assert_eq!(l.splitn(4, " | ").count(), 4, "corrupted log line: {l}");
            } else {
                assert!(l.is_empty() || l.starts_with("# Change log"), "corrupted log line: {l}");
            }
        }
    }
    assert!(lines >= 8 * 15 * 2, "every turn is logged ({lines})");
    let _ = brain::lock;
    let _ = fs::remove_dir_all(&root);
}
