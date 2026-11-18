//! The Rust cache key equals tools/design3/haiku.py's, so both share one archive.

#[test]
fn sha256_known_vectors() {
    assert_eq!(teach3::sha256::hex(b""), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    assert_eq!(teach3::sha256::hex(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    let long = vec![b'a'; 1000];
    assert_eq!(teach3::sha256::hex(&long).len(), 64);
}

#[test]
fn key_matches_the_python_archive() {
    // the phase A smoke call made by haiku.py is in the archive; its key must be reproduced
    let path = kb3::repo_root().join("data/training/calls3/cache.jsonl");
    let text = std::fs::read_to_string(path).unwrap();
    let mut n = 0;
    for line in text.lines().take(25) {
        let v: serde_json::Value = serde_json::from_str(line).unwrap();
        let k = teach3::key(v["model"].as_str().unwrap(), v["system"].as_str().unwrap(), v["prompt"].as_str().unwrap());
        assert_eq!(k, v["key"].as_str().unwrap());
        n += 1;
    }
    assert!(n >= 2);
}

#[test]
fn facts_in_keeps_only_valid_schema_facts() {
    let a = "step(t, t_s1).\naction(t_s1, heat).\nHere you go:\nfoo(t).\n- uses(t_s1, pan).";
    let f = teach3::facts_in(a);
    assert_eq!(f.len(), 3);
}
