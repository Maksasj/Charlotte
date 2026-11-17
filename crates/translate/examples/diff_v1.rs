//! Differential check: the v1 reading (split at . ! ? and ", and", then the
//! pattern parser) against the 2.0 translator without a model, over the dev
//! parser set and the dev teaching dialogues. Prints every message whose acts
//! differ. Run: cargo run -p sla_translate --example diff_v1

use sla_parser::{parse, Context, Intent};
use sla_translate::{TContext, Translator};

fn v1_split(text: &str) -> Vec<String> {
    let text = text.replace(", and ", ". ").replace(", but ", ". ").replace("; ", ". ");
    sla_translate::segment::sentences(&text)
}

fn main() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..");
    let mut msgs: Vec<String> = Vec::new();
    for l in std::fs::read_to_string(root.join("data/synthetic/parser/dev.md")).unwrap().lines() {
        if let Some((s, _)) = l.strip_prefix("- ").and_then(|x| x.rsplit_once(" => ")) {
            msgs.push(s.to_string());
        }
    }
    for e in std::fs::read_dir(root.join("data/synthetic/teaching/dev")).unwrap().flatten() {
        for l in std::fs::read_to_string(e.path()).unwrap().lines() {
            if let Some(m) = l.strip_prefix("> ") {
                msgs.push(m.split(" => ").next().unwrap_or(m).trim().to_string());
            }
        }
    }
    let mut t = Translator::new(sla_lexicon::Registry::builtin());
    let ctx = TContext { speaker: "maksim".into(), ..Default::default() };
    let mut diffs = 0;
    for m in &msgs {
        let mut last = None;
        let mut old: Vec<String> = Vec::new();
        for s in v1_split(m) {
            let i = parse(&s, &Context { speaker: "maksim".into(), last_entity: last.clone() });
            if let Intent::Tell { facts, .. } = &i {
                if let Some(sla_core::Term::Val(sla_core::Value::Sym(e))) = facts.first().and_then(|a| a.args.first()) {
                    last = Some(e.clone());
                }
            }
            old.push(i.canonical());
        }
        let tr = t.translate(m, &ctx);
        let new: Vec<String> = tr.parts.iter().flat_map(|p| if p.ok { p.acts.iter().map(|a| a.canonical()).collect::<Vec<_>>() } else { vec![format!("FAILED: {}", p.problems.join("; "))] }).collect();
        let norm = |v: &[String]| v.iter().map(|s| s.replace("quality", "Q")).collect::<Vec<_>>();
        if norm(&old) != norm(&new) {
            diffs += 1;
            println!("{m}\n   v1:  {}\n   2.0: {}", old.join(" | "), new.join(" | "));
        }
    }
    println!("{diffs} of {} messages differ", msgs.len());
}
