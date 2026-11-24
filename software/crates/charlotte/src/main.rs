//! The `charlotte` binary: chat in a terminal, serve the HTTP API, learn from a corpus, check the
//! vault, and produce simulated conversations.

mod http;

use charlotte_engine::learn::{Corpus, LearnOptions};
use charlotte_engine::sim::{simulate, Paraphrases, SimOptions};
use charlotte_engine::{default_layers, Engine, Verdict};
use charlotte_vault::Vault;
use std::io::{BufRead, Write};

const USAGE: &str = "charlotte [--vault DIR] <command>      (the vault defaults to ./vault, then ../vault)

  chat                          talk to the agent; :trace shows the last trace, :close <verdict>, :quit
  ask [--trace] \"text\" ...      hold one conversation with the given messages and print the replies
  serve [--port 7878] [--web DIR]
  learn FILE [--review]         learn from a corpus (JSON lines); --review leaves everything as candidates
  promote ID | reject ID        trust a candidate, or discard it
  teach \"message\" \"reply\"      teach a reply to a message, as a candidate chat (in `chat`: :teach <reply> after a message)
  check [--fix]                 list problems in the vault; --fix rewrites notes in the engine's own format
  grow FILE [--intent ID]       find new wordings for the social intents in unlabelled chats (candidates, by reply)
  tasks FILE [--promote] [--eval N]  learn task intents, slots and their ask order from labelled task dialogues (SGD format)
  connectors FILE               find the words that join two requests in one message (a lexicon candidate)
  vet FILE                      check phrase candidates against the replies in a chat file (cross-source)
  regress FILE                  run the regression set (rewordings and real held-out exchanges) and print the score;
                                exits 1 on a regression
  simulate [--n 80] [--per-shape] [--seed 1] [--paraphrases FILE] [--out FILE]
                                write conversations held by a simulated teacher over the vault;
                                --per-shape gives --n of each shape (rule check, procedure, lookup)";

struct Args(Vec<String>);
impl Args {
    fn flag(&mut self, name: &str) -> bool {
        match self.0.iter().position(|a| a == name) {
            Some(i) => {
                self.0.remove(i);
                true
            }
            None => false,
        }
    }
    fn opt(&mut self, name: &str) -> Option<String> {
        let i = self.0.iter().position(|a| a == name)?;
        if i + 1 >= self.0.len() {
            return None;
        }
        self.0.remove(i);
        Some(self.0.remove(i))
    }
}

fn main() {
    let mut args = Args(std::env::args().skip(1).collect());
    // Defaults work from the repository root and from software/.
    let vault = args.opt("--vault").unwrap_or_else(|| first_existing(&["vault", "../vault"]));
    if args.0.is_empty() {
        println!("{USAGE}");
        return;
    }
    let cmd = args.0.remove(0);
    if let Err(e) = run(&cmd, &vault, args) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

/// The first of these paths that exists, else the first.
fn first_existing(paths: &[&str]) -> String {
    paths.iter().find(|p| std::path::Path::new(p).exists()).unwrap_or(&paths[0]).to_string()
}

fn run(cmd: &str, vault: &str, mut args: Args) -> Result<(), String> {
    match cmd {
        "chat" => {
            let engine = Engine::open(vault)?;
            let mut conv = engine.conversation();
            let mut last = None;
            let stdin = std::io::stdin();
            print!("> ");
            std::io::stdout().flush().ok();
            for line in stdin.lock().lines() {
                let line = line.map_err(|e| e.to_string())?;
                let line = line.trim();
                match line {
                    "" => {}
                    ":quit" | ":q" => break,
                    l if l.starts_with(":teach") => match conv.teach(l[6..].trim()) {
                        Ok(item) => println!("  taught as candidate {} - `charlotte promote {}` to trust it", item.id, item.id),
                        Err(e) => println!("  {e}"),
                    },
                    ":trace" => println!("{}", last.as_ref().map(|t| serde_json::to_string_pretty(t).unwrap()).unwrap_or_else(|| "no reply yet".into())),
                    l if l.starts_with(":close") => {
                        let verdict = match l.split_whitespace().nth(1) {
                            Some("partial") => Verdict::Partial,
                            Some("failed") => Verdict::Failed,
                            _ => Verdict::Resolved,
                        };
                        for c in conv.close(verdict)? {
                            println!("  {}: trust {:.2} -> {:.2}", c.id, c.before, c.after);
                        }
                        conv = engine.conversation();
                    }
                    l => {
                        let reply = conv.say(l);
                        println!("{}", reply.text);
                        last = Some(reply.trace);
                    }
                }
                print!("> ");
                std::io::stdout().flush().ok();
            }
            Ok(())
        }
        "ask" => {
            let trace = args.flag("--trace");
            let engine = Engine::open(vault)?;
            let mut conv = engine.conversation();
            for m in &args.0 {
                let reply = conv.say(m);
                println!("user:  {m}\nagent: {}", reply.text);
                if trace {
                    for r in &reply.trace.requests {
                        let acts: Vec<String> = r.steps.iter().map(|s| format!("{}{}", s.act.op.name(), s.act.arg.as_ref().map(|a| format!("({a})")).unwrap_or_default())).collect();
                        let rule = r.steps.last().and_then(|s| s.rule.as_ref()).map(|x| format!("rule {} [{}]", x.index, x.when.join(", "))).unwrap_or_else(|| "no rule".into());
                        println!(
                            "       intent {} · facts {} · {} · pattern {} · atom {} · {} · {}",
                            r.intent.as_deref().unwrap_or("?"),
                            serde_json::to_string(&r.facts).unwrap(),
                            r.kind.name(),
                            r.pattern.as_deref().unwrap_or("-"),
                            r.atom.as_deref().unwrap_or("-"),
                            rule,
                            acts.join(" > ")
                        );
                    }
                    for n in &reply.trace.notes {
                        println!("       note: {n}");
                    }
                }
            }
            Ok(())
        }
        "serve" => {
            let port: u16 = args.opt("--port").and_then(|p| p.parse().ok()).unwrap_or(7878);
            let web = args.opt("--web").unwrap_or_else(|| first_existing(&["software/web/dist", "web/dist"]));
            http::serve(Engine::open(vault)?, vault, port, &web)
        }
        "learn" => {
            let review = args.flag("--review");
            let file = args.0.first().ok_or("learn needs a corpus file")?;
            let engine = Engine::open(vault)?;
            let corpus = Corpus::open(file)?;
            let report = engine.learn(&corpus, &LearnOptions { auto_promote: !review, ..Default::default() }, &|_| {})?;
            println!("{report}");
            Ok(())
        }
        "promote" | "reject" => {
            let id = args.0.first().ok_or("give the candidate's id")?;
            let engine = Engine::open(vault)?;
            if cmd == "promote" {
                let item = engine.promote(id)?;
                println!("{} is now trusted ({})", item.id, item.path);
            } else {
                engine.reject(id)?;
                println!("{id} discarded");
            }
            Ok(())
        }
        "teach" => {
            let (message, reply) = match (args.0.first(), args.0.get(1)) {
                (Some(m), Some(r)) => (m.clone(), r.clone()),
                _ => return Err("teach \"message\" \"reply\"".into()),
            };
            let engine = Engine::open(vault)?;
            let mut conv = engine.conversation();
            conv.say(&message);
            let item = conv.teach(&reply)?;
            println!("taught as candidate {} - `charlotte promote {}` to trust it", item.id, item.id);
            Ok(())
        }
        "check" => {
            let v = Vault::open(vault)?;
            if args.flag("--fix") {
                println!("{} notes rewritten", v.normalize()?);
            }
            let problems = v.lint();
            for p in &problems {
                println!("{p}");
            }
            let mut kinds = std::collections::BTreeMap::new();
            for n in v.notes.values() {
                *kinds.entry(format!("{} {}", n.status(), n.kind())).or_insert(0) += 1;
            }
            println!("{} notes: {}", v.notes.len(), kinds.iter().map(|(k, n)| format!("{n} {k}")).collect::<Vec<_>>().join(", "));
            if problems.is_empty() {
                println!("no problems");
                Ok(())
            } else {
                Err(format!("{} problems", problems.len()))
            }
        }
        "grow" | "vet" => {
            use charlotte_engine::grow::{self, GrowOptions};
            let only = args.opt("--intent");
            let file = args.0.first().ok_or("give a chat corpus file")?;
            let corpus = charlotte_engine::Corpus::open(file)?;
            let ex = grow::exchanges(&corpus);
            let opts = GrowOptions::default();
            let mut v = Vault::open(vault)?;
            println!("{} exchanges from {file}", ex.len());
            if cmd == "grow" {
                for g in grow::grow(&v, &ex, &opts, only.as_deref()) {
                    match &g.skipped {
                        Some(why) => println!("{:<10} skipped: {why}", g.intent),
                        None => {
                            println!("{:<10} {} seed messages, {:.0}% drew a reply like: {}  (base rate {:.1}%)", g.intent, g.seed_messages, g.coherence * 100.0, g.signature.join(", "), g.base_rate * 100.0);
                            for x in &g.grown {
                                println!("  + {:<28} {:>5} msgs, {:.0}% / held out {:.0}% of {}   e.g. {}", x.phrase, x.support, x.precision * 100.0, x.held_precision * 100.0, x.held_n, x.examples.first().cloned().unwrap_or_default());
                            }
                        }
                    }
                    if let Some(n) = grow::candidate_note(&g, file) {
                        v.put(n)?;
                    }
                }
            } else {
                for (id, r) in grow::vet_candidates(&mut v, &ex, &opts, file) {
                    match r {
                        Err(e) => println!("{id}: not checked ({e})"),
                        Ok(list) => {
                            println!("{id}");
                            for x in list {
                                println!("  {:<28} {:>5} msgs, {:.0}% drew the reply (lift {:.1})", x.phrase, x.messages, x.precision * 100.0, x.lift);
                            }
                        }
                    }
                }
            }
            Ok(())
        }
        "types" => {
            use charlotte_engine::exchange_types as et;
            let opts = et::TypeOptions { user_only: args.flag("--user-only"), ..Default::default() };
            let file = args.0.first().ok_or("give a chat corpus file")?;
            let corpus = charlotte_engine::Corpus::open(file)?;
            let ex = charlotte_engine::grow::exchanges(&corpus);
            let mut v = Vault::open(vault)?;
            println!("{} exchanges from {file}", ex.len());
            for t in et::induce(&v, &ex, &opts) {
                let tag = match (&t.known, &t.skipped) {
                    (Some(k), _) => format!("known: {k}"),
                    (_, Some(why)) => format!("skipped: {why}"),
                    _ => "NEW".into(),
                };
                println!("{:<18} {:>6} exchanges, {:.0}% explained, replies {:.0}% in top 3   [{tag}]", t.id, t.exchanges, t.coverage * 100.0, t.reply_share * 100.0);
                println!("    replies like: {}", t.reply_words.join(", "));
                for s in t.skeletons.iter().take(4) {
                    println!("    + {:<24} {:>5} msgs, {:.0}% / held out {:.0}% of {}", s.phrase, s.support, s.precision * 100.0, s.held_precision * 100.0, s.held_n);
                }
                if let Some(e) = &t.echo {
                    println!("    echo after \"{}\": {} values ({}), {:.0}% said back", e.after, e.values, e.examples.join(", "), e.share * 100.0);
                }
                println!("    reply: {}", t.reply);
                for n in et::candidate_notes(&t, file) {
                    v.put(n)?;
                }
            }
            Ok(())
        }
        "numbers" => {
            use charlotte_engine::numwords;
            let only = args.opt("--slot");
            let file = args.0.first().ok_or("give a labelled corpus file")?;
            let corpus = charlotte_engine::Corpus::open(file)?;
            let mut v = Vault::open(vault)?;
            for r in numwords::induce(&v, &corpus, &Default::default(), only.as_deref()) {
                match &r.skipped {
                    Some(why) => println!("{:<10} skipped: {why}", r.slot),
                    None => {
                        println!("{:<10} {} labelled turns", r.slot, r.labelled);
                        for p in &r.patterns {
                            println!("  + {:<22} {:>5} numbers, {:.0}% / held out {:.0}% of {}   e.g. {}", p.text, p.support, p.precision * 100.0, p.held_precision * 100.0, p.held_n, p.example);
                        }
                    }
                }
                if let Some(n) = numwords::candidate_note(&r, file) {
                    v.put(n)?;
                }
            }
            Ok(())
        }
        "tasks" => {
            use charlotte_engine::tasks;
            let file = args.0.first().ok_or("give a labelled dialogue file")?;
            let text = std::fs::read_to_string(file).map_err(|e| format!("{file}: {e}"))?;
            let dialogues = tasks::parse(&text);
            let found = tasks::induce(&dialogues, &Default::default());
            let mut v = Vault::open(vault)?;
            let notes = tasks::candidate_notes(&found, file, &v);
            for t in &found {
                println!("  {:<24} {:>5} dialogues  {} wordings  asks: {}", t.intent, t.dialogues, t.phrases.len(), t.slots.iter().map(|s| s.name.as_str()).collect::<Vec<_>>().join(", "));
            }
            let ids: Vec<String> = notes.iter().map(|n| n.id.clone()).collect();
            for n in notes {
                v.put(n)?;
            }
            println!("{} candidates written", ids.len());
            let promote = args.flag("--promote");
            let eval = args.opt("--eval").and_then(|x| x.parse().ok());
            if promote || eval.is_some() {
                let engine = Engine::open(vault)?;
                if promote {
                    for id in &ids {
                        engine.promote(id)?;
                    }
                    println!("promoted {}", ids.len());
                }
                if let Some(n) = eval {
                    let s = tasks::evaluate(&engine, &dialogues, &found, n);
                    let pct = |a: usize, b: usize| if b == 0 { 0.0 } else { 100.0 * a as f64 / b as f64 };
                    println!("held-out dialogues played: {}", s.dialogues);
                    println!("  intent read from the first request: {:.0}%", pct(s.intent_read, s.dialogues));
                    println!("  hand-over reached:                  {:.0}%", pct(s.handoff, s.dialogues));
                    println!("  asked exactly what the agent asked: {:.0}%", pct(s.asked_right, s.dialogues));
                    println!("  slots asked that the agent asked:   {:.0}%", pct(s.slot_precision.0, s.slot_precision.1));
                    println!("  slots the agent asked that we did:  {:.0}%", pct(s.slot_recall.0, s.slot_recall.1));
                }
            }
            Ok(())
        }
        "connectors" => {
            use charlotte_engine::connectors;
            let file = args.0.first().ok_or("give a chat file")?;
            let corpus = charlotte_engine::Corpus::open(file)?;
            let mut v = Vault::open(vault)?;
            let found = connectors::induce(&v, &corpus, &Default::default());
            if found.is_empty() {
                println!("no new connectors");
            }
            for c in &found {
                println!("  + {:<22} between requests in {:>4} messages, {:.0}% / held out {:.0}% of {}   e.g. {}", c.phrase, c.support, c.precision * 100.0, c.held_precision * 100.0, c.held_n, c.example);
            }
            if let Some(n) = connectors::candidate_note(&found, file) {
                v.put(n)?;
            }
            Ok(())
        }
        "regress" => {
            let file = args.0.first().ok_or("regress needs a case file")?;
            let text = std::fs::read_to_string(file).map_err(|e| format!("{file}: {e}"))?;
            let out = charlotte_engine::regress::run(vault, &charlotte_engine::regress::parse(&text)?)?;
            println!("{out}");
            if out.regressions() > 0 {
                return Err(format!("{} regressions", out.regressions()));
            }
            Ok(())
        }
        "simulate" => {
            let n = args.opt("--n").and_then(|x| x.parse().ok()).unwrap_or(80);
            let seed = args.opt("--seed").and_then(|x| x.parse().ok()).unwrap_or(1);
            let per_shape = args.flag("--per-shape");
            let para: Paraphrases = match args.opt("--paraphrases") {
                Some(f) => serde_yaml_from(&f)?,
                None => Paraphrases::default(),
            };
            let out = args.opt("--out");
            let v = Vault::open(vault)?;
            let convs = simulate(&v, &default_layers(&v), &para, &SimOptions { n, per_shape, seed, corrections: 0.3, calls: args.flag("--calls") });
            let text: String = convs.iter().map(|c| serde_json::to_string(c).unwrap() + "\n").collect();
            match out {
                Some(f) => {
                    if let Some(dir) = std::path::Path::new(&f).parent() {
                        std::fs::create_dir_all(dir).ok();
                    }
                    std::fs::write(&f, text).map_err(|e| e.to_string())?;
                    println!("{} conversations written to {f}", convs.len());
                }
                None => print!("{text}"),
            }
            Ok(())
        }
        _ => Err(format!("unknown command '{cmd}'\n\n{USAGE}")),
    }
}

fn serde_yaml_from(file: &str) -> Result<Paraphrases, String> {
    let text = std::fs::read_to_string(file).map_err(|e| format!("{file}: {e}"))?;
    charlotte_engine::sim::paraphrases_from_yaml(&text).map_err(|e| format!("{file}: {e}"))
}
