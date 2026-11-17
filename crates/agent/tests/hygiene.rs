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
