//! H3 is sealed (Design 3.0 section 6.1): every file matches the committed manifest, and no
//! file was added or removed.

#[test]
fn heldout_matches_its_manifest() {
    let root = kb3::repo_root().join("data/heldout3");
    let manifest = std::fs::read_to_string(root.join("MANIFEST.sha256")).unwrap();
    let mut listed = std::collections::BTreeSet::new();
    for line in manifest.lines() {
        let (hash, path) = line.split_once("  ").unwrap();
        let bytes = std::fs::read(root.join(path)).unwrap_or_else(|_| panic!("{path} missing"));
        assert_eq!(teach3::sha256::hex(&bytes), hash, "{path} changed after sealing");
        listed.insert(path.to_string());
    }
    for tier in ["G", "S", "T"] {
        for e in std::fs::read_dir(root.join(tier)).unwrap().flatten() {
            let p = format!("{tier}/{}", e.file_name().to_string_lossy());
            assert!(listed.contains(&p), "{p} added after sealing");
        }
    }
    assert_eq!(listed.len(), 150);
}
