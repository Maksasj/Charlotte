//! Design 2.0, milestone 18: zero duplicate entity pairs after a hygiene pass.

use sla_agent::Agent;

#[test]
fn hygiene_pass_links_every_duplicate() {
    let dir = std::env::temp_dir().join(format!("sla_m18_hygiene_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let mut a = Agent::open(&dir, "maksim").unwrap();
    a.ask_in_chat = false;
    for m in ["Tom has a knife.", "Bob has knives.", "Anna likes cherries.", "Sara likes cherry.", "Lena wants sunglasses.", "Rex is a dog."] {
        a.respond(m);
    }
    let mut b = a.brain();
    let now = sla_core::Timestamp::now();
    // Spellings v1 stored side by side.
    for f in ["likes(sara, cherries)", "has(lena, sunglasses)"] {
        b.kb.assert_fact(sla_core::Fact::new(sla_core::parse_ground(f).unwrap(), "v1"), now, "test");
    }
    let r = sla_agent::hygiene::run(&mut b, now);
    println!("{r:?}");
    assert!(r.duplicates_before.iter().any(|(x, y)| (x, y) == (&"knife".to_string(), &"knive".to_string()) || (x, y) == (&"knife".to_string(), &"knives".to_string())), "{r:?}");
    assert!(r.duplicates_before.iter().any(|(x, y)| x == "cherry" && y == "cherries"), "{r:?}");
    assert!(r.duplicates_before.iter().any(|(x, y)| x == "sunglass" && y == "sunglasses"), "{r:?}");
    assert_eq!(r.duplicates_after, 0);
    assert_eq!(sla_agent::hygiene::run(&mut b, now).duplicates_before.len(), 0, "a second pass finds nothing");
    // Linked, not rewritten: the original facts are still there.
    assert!(b.kb.all_facts().iter().any(|f| f.atom.to_string().contains("cherries") || f.atom.to_string().contains("cherry")));
    drop(b);
    let _ = std::fs::remove_dir_all(&dir);
}

/// The consolidation pass finds a step two procedures share.
#[test]
fn consolidation_finds_shared_steps() {
    let dir = std::env::temp_dir().join(format!("sla_m18_consolidate_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let brain = sla_agent::Brain::shared(&dir).unwrap();
    let mut b = sla_agent::brain::lock(&brain);
    for (id, extra) in [("make_tea", "pour"), ("make_coffee", "stir")] {
        let src = format!(
            "---
id: {id}
status: hypothesis
source: test
uses: 0
successes: 0
---
# {id}

Test.

```datalog
task({id}).
step({id}, {id}_s1).
action({id}_s1, boil).
object_of({id}_s1, water).
step({id}, {id}_s2).
action({id}_s2, {extra}).
object_of({id}_s2, cup).
before({id}_s1, {id}_s2).
```
"
        );
        b.kb.vault.put_procedure(sla_core::ProcedureKind::Procedure, sla_core::ProcedureFile::parse(&src, id));
    }
    let r = sla_agent::consolidate::run(&mut b, sla_core::Timestamp::now());
    assert!(r.shared_steps.iter().any(|(a, o, t)| a == "boil" && o == "water" && t.len() == 2), "{:?}", r.shared_steps);
    assert!(!r.shared_steps.iter().any(|(a, _, _)| a == "pour" || a == "stir"));
    drop(b);
    let _ = std::fs::remove_dir_all(&dir);
}

/// Consolidation migrates old `replace` facts and rules to `substitute`.
#[test]
fn consolidation_migrates_replace_to_substitute() {
    let dir = std::env::temp_dir().join(format!("sla_m18_migrate_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let a = Agent::open(&dir, "maksim").unwrap();
    let mut b = a.brain();
    let now = sla_core::Timestamp::now();
    b.kb.assert_fact(sla_core::Fact::new(sla_core::parse_ground("replace(saucepan, kettle)").unwrap(), "teacher"), now, "test");
    let meta = sla_core::RuleMeta { id: "replace_rule".into(), status: sla_core::RuleStatus::Confirmed, conf: 0.9, evidence: 0, counter: 0, module: "general".into(), source: "teacher".into() };
    b.kb.vault.put_rule_file(sla_core::RuleFile::new(&meta, "", &[sla_core::parse_rule("replace(A, minced_beef) :- is_a(A, lentil).").unwrap()]));
    assert_eq!(sla_agent::consolidate::migrate_substitutes(&mut b, now), 2);
    let open: Vec<String> = b.kb.all_facts().into_iter().filter(|f| f.is_open()).map(|f| f.atom.to_string()).collect();
    assert!(open.contains(&"substitute(kettle, saucepan)".to_string()), "{open:?}");
    assert!(open.contains(&"substitute(minced_beef, lentil)".to_string()), "{open:?}");
    assert!(!open.iter().any(|f| f.starts_with("replace(")), "{open:?}");
    assert!(!b.kb.vault.rules.contains_key("replace_rule"));
    drop(b);
    let _ = std::fs::remove_dir_all(&dir);
}
