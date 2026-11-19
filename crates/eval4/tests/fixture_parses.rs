//! Milestone 31 acceptance: every fixture message parses, and the planted sort errors (and only
//! those) are rejected when taught. Delivery only; nothing is scored.

#[test]
fn fixture_messages_parse_and_only_planted_sort_errors_are_rejected() {
    let dir = eval4::root().join("data/spike4");
    if !dir.join("bench/questions.jsonl").exists() {
        return;
    }
    let fx = eval4::Fixture::load(&dir).unwrap();
    for l in &fx.teach {
        agent4::msg::parse(&l.text).unwrap_or_else(|e| panic!("{}:{} {e}", l.file, l.no));
    }
    for q in &fx.questions {
        agent4::msg::parse(&q.query).unwrap_or_else(|e| panic!("{} {e}", q.id));
    }
    let d = eval4::deliver(&fx, &eval4::Opts { learn: false, naming: true, prefix: 1.0, min_support: 3, ..Default::default() });
    let mut reasons: std::collections::BTreeMap<String, usize> = Default::default();
    for (_, r) in &d.rejects {
        let key: String = r.split(" is ").nth(1).map(|x| x.to_string()).unwrap_or(r.clone());
        *reasons.entry(key).or_insert(0) += 1;
    }
    eprintln!("{:#?}", d.rejects.iter().take(12).collect::<Vec<_>>());
    eprintln!("{reasons:#?}");
    assert_eq!(d.agent.stats.rejected, 2, "planted sort errors are 2; rejected {}", d.agent.stats.rejected);
}
