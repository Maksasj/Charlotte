# REPORT_DESIGN_1: The symbolic learning agent, first build and first lessons

*Report date: 2026-11-16. It covers everything from the first commit (`6d4aae5`) to the training app (`99c6fa4`). Repository: this one; the design is in `DESIGN.md`.*

This report gathers everything learned in the first build of the agent described in `DESIGN.md`: what was built, every measured result, what goes wrong in conversation and why, how the code holds up, how the work relates to existing research, and what to do next. Four parts were written by independent analysis agents, each working from the repository, the evaluation data and the transcripts. One part is the build log of the session itself. Every number comes from files in this repository: `eval/`, `vault/`, `data/` and the git history.

## Contents

1. Executive summary
2. Synthesis: what we know now
3. How the work went: timeline, decisions and lessons
4. Architecture and implementation review
5. Results and data analysis
6. Conversation failure analysis and recommendations
7. Related work and strategic options
8. Consolidated next steps

---

## 1. Executive summary

**What exists.** Every milestone in DESIGN.md's first run (1–10) is built, passes its acceptance test, and is committed. 68 automated tests pass. Of the optional milestone 11, only the LLM translator was built: Haiku turns free text into the formal intent language, while every reply stays symbolic.

Around the agent there is now:
- a command-line chat (`sla`) and a one-command evaluation with Haiku judges (`sla_eval`)
- pretraining tools (`sla_pretrain`): sentence lists, lessons, natural conversations and friend chats
- a training app (`sla_trainer`) with a React UI showing a live chat per teacher, token use, plan limits and answer times

The main vault holds about **2,171 facts** (374 of them closed), **207 rule files** and **about 620 entities**.

**What the numbers say.**

| Measure | Result |
|---|---|
| Knowledge tests: scripted teaching dialogues | **100%** (dev), **97.8%** (held-out) |
| Parser exact accuracy: fixed sentence sets | **100%** (dev), **98%** (held-out) |
| Conversation quality from Haiku judges, 1–10 | **flat at about 2.4–4.3**. Baseline 3.33 (3 judges); with the translator 4.25 → 3.88 (8 judges, change within noise); pretrained vault 2.38 → 3.88 after fixes |
| Real answers in the newest help-style training run | **0–1 of 68 replies** (the two analyses draw the line slightly differently); 44% not understood, 21% only acknowledged facts, 18% "I don't know" |
| Understanding as input gets longer | 97% (single sentences) → 86% (short lessons) → 81% (friend chats) → 56% (help requests) |
| Where an answer's time goes | **96% is the Haiku translation** (about 5.3 s); the symbolic agent takes a median of 48 ms |
| Learner on pretraining data | 73 hypothesis rules proposed, 1 confirmed, and that one has a garbled head |

**The diagnosis in one paragraph.** The symbolic core works, but it was built for, and tested on, short sentences about *facts*. The fixed tests use sentences of about 4 words, so they stayed perfect while real conversation stayed poor. Natural conversation, and helping above all, is about *doing things*: tasks, steps, what something needs, choosing among options, and following a conversation across turns. The agent has no way to represent any of that, so the translator either fails or scatters fragments into the vault ("Wheel removes from frame"), and the agent answers "I don't know what you do". Meanwhile the translator has quietly become the real parser, inventing predicates unchecked (278 distinct predicates, about 200 catch-all `property(_, quality, _)` values), and the agent rebuilds its whole model several times per message, which won't scale.

**The recommendation.** The next milestone should give the agent **procedural knowledge and dialogue state**: what a task needs, its steps and duration, task frames that ask for missing information, choosing among options, and a record of where the conversation is. It should start with **an honest metric**: every agent turn labelled by outcome, and real learning measured as *answering a repeated scenario without being taught*. Next comes a stricter, cheaper translator with richer reply generation, and then a curriculum for the teachers. Details are in sections 6, 7 and 8.

---

## 2. Synthesis: what we know now

These are the conclusions the four independent analyses agree on.

1. **The fixed tests measured the wrong thing.** Parser and knowledge tests stayed at 98–100% in every run while conversation quality stayed at about 3–4 out of 10. The tests use short sentences (about 4 words) that the pattern parser was designed for. A test set built from real conversation turns, labelled by outcome, is needed before any further tuning.
2. **The missing piece is representation, not intelligence.** Most failures happen *before* reasoning: there is nothing to translate "first take the wheel off, then…" into, no task or slot to fill from "I have pasta and 20 minutes", and no state that says "we are fixing a tyre, at step 2". The analyses converge on procedures (needs, steps, durations), task frames with slot filling, and dialogue state.
3. **The translator is both the biggest helper and the biggest risk.** It raised understanding of free text, but it now decides what counts as a fact, invents predicates freely, and accounts for 96% of answer time and 74% of the cost in the training run. It needs a closed set of predicates, grounding checks, few-shot examples from reviewed translations, working prompt caching, and lower thinking effort.
4. **Knowledge quality degrades without hygiene.** Notable problems:
   - 17% of facts are closed
   - 44 personal facts are still open
   - 24 of 40 sampled "proper names" are ordinary words
   - catch-all qualities and duplicated or synonym entities
   - the learner almost never confirms a rule

   The reviewer helps but can't fix a schema problem.
5. **Measurement needs more care than expected.** Three judges are too noisy; eight are the minimum. The pretrained-vault result (2.38) showed that side effects such as curiosity questions in chat can dominate the score. Every change should be compared with the previous build on the same judges and seeds.
6. **The engineering is sound but won't scale as is.** The Datalog engine, vault, proof traces and faithfulness checks are solid and well tested. But the agent fully re-evaluates the model several times per message, history questions mix facts that were never true together, the parallel training around the shared vault is fragile, and about 1,160 lines of trainer and pretraining code have no tests.
7. **Training throughput and limits are manageable.** Lean Claude calls (our own system prompt, no tools) cut the cost per call about 14 times. Translating outside the shared lock raised throughput from about 10 to 65 messages a minute. Plan limits can be read without spending tokens. The remaining lever is the translator: low effort roughly halves its time, and caching the spec saves about 4,400 tokens per call.

The detailed sections follow. Each stands on its own and keeps its own numbering.

---

---

## 3. How the work went: timeline, decisions and lessons

This part is the build log from the session itself: what was done in what order, why each decision was made, what blocked progress, and what the session taught about running this kind of project.

### Timeline

All times are local (Europe/Kyiv) on 2026-11-16. The commits for milestones 1–3 carry the same time because the code was written before a Rust toolchain existed and was committed once it compiled.

| Phase | Commits | What happened |
|---|---|---|
| **0. Blocked start** | – | DESIGN.md asks for Rust. The machine had no Rust and no MSVC build tools. Installing rustup (direct download, then winget) was refused by the session's permission policy. Milestones 1–3 were written without a compiler while waiting for the user to install Rust. |
| **1. Milestones 1–3** | `6d4aae5` `efe7d88` `82dd2c5` | After the user installed rustup (GNU toolchain, which brings its own linker), everything compiled on the first build. One test expectation was wrong, not the engine: with `is_a(penguin, bird)` the category `penguin` itself satisfies `flies(X)`. |
| **2. Milestones 4–10** | `cc1b389` … `e16d620` | Parser, reply generator, agent loop, pattern learner, corrections and demonstrations, curiosity drive, evaluation. Independent sub-agents wrote the test data (parser sentences, teaching dialogues) from specs alone, without seeing the code. |
| **3. Baseline** | `a5568fd` | First full evaluation: knowledge tests 100% (dev) and 97.8% (held-out); conversation quality 3.33/10 from 3 Haiku judges. Every milestone in DESIGN.md passes its acceptance test. |
| **4. Pretraining (cold start)** | `4c06c95` | Claude teachers seed the main vault: 10 topics of sentence lists, then 40 structured lessons, each followed by a Claude reviewer that rejects wrong or garbled facts. |
| **5. Conversation phrasing** | `2c9c3ca` | The user found that "what else do you know about birds?" failed. Added follow-ups, reactions, thanks and questions about the agent itself. |
| **6. LLM translator (M11, part)** | `a70311d` `75ac6fd` `4eb59b7` | At the user's request, Haiku translates free text into the formal intent language through the `Parser` trait. Replies stay fully symbolic. Policy: patterns first, the model for anything long, unclear or garbled. |
| **7. Measuring properly** | `40354cd` `e7f9705` | Moved to 8 judges because 3 were too noisy. Found that the pretrained vault hurt the score (2.38) because of off-topic curiosity questions and "you told me" about teacher facts. Fixed both. |
| **8. Friend chats** | `0ce89b2` … `f23b09d` | The user wanted real conversation, not lessons: Haiku plays a friend. Personal remarks are closed after each chat. Throughput rose from about 10 to 65 messages per minute once translation moved outside the shared lock. |
| **9. Training app** | `103f7fd` `7e26af1` `99c6fa4` | The user asked for a proper training app: `sla_trainer` (Rust backend) plus a React UI with a live chat per teacher, progress, token use, plan limits from `/usage`, and agent answer times. Training stays idle until the user presses Start. |

### Decisions and why

- **Package names `sla_*`.** A crate named `core` would clash with Rust's built-in `core` crate.
- **Few dependencies.** Milestones 1–10 use no external crates (timestamps via Hinnant's date algorithms, a tiny random generator for tests). `serde`/`serde_json` were added only for the trainer's HTTP API and for reading Claude's JSON output.
- **The GNU toolchain**, because no Microsoft C++ build tools were installed.
- **Independent test data.** Sub-agents wrote the parser sets, teaching dialogues and held-out sets from specifications only. Held-out failures are shown only as counts, so tuning never sees them.
- **Every reply stays symbolic.** The model translates input and acts as teacher, reviewer and judge; it never writes a reply. Each reply clause comes from an atom the knowledge base believes, and tests check this both structurally and by parsing the reply back.
- **Lean Claude calls.** Headless `claude -p` by default loads the whole Claude Code system prompt and tool definitions, about 25,500 cached tokens per call. A short system prompt of our own with no tools made a call about **14 times cheaper** ($0.00187 → $0.00013 at list price for a trivial call).
- **Plan limits instead of cost in the UI.** On a subscription, cost isn't billed; what matters is the share of the session and weekly limits. A headless `/usage` call reports them and uses no tokens.

### Blockers and incidents

| Incident | Effect | Resolution |
|---|---|---|
| No Rust, and installing it was refused by the permission policy | Milestones 1–3 written blind; dozens of repeated stop-hook turns while waiting | The user installed rustup |
| A stray `cat >` in a shell command waited on stdin | One command hung for 10 minutes | Stopped and rerun |
| The `/goal` stop hook kept blocking while waiting on the user | Many repeated "still blocked" turns | Product feedback drafted |
| Release binary locked by the running chat or trainer | Rebuilds failed | Build into a second target directory, or stop the process first |
| `sla_eval` became ambiguous after a second binary was added | The one-command evaluation broke | `default-run = "sla_eval"` |
| Translation ran inside the shared lock | 16 parallel conversations ran at the speed of 1 (about 10 messages/min) | `Agent::prepare` translates first, outside the lock (65 messages/min) |
| Each message was logged once per intent | Duplicated messages in the live feed | Log once per message |
| Usage limit reached during the friend-chat run | Only 417 of about 1,100 planned turns completed | Run treated as partial |
| Chat bubbles squashed in the training UI | Messages unreadable | Scrollable chat window with rows that never shrink |
| Starting training on the user's behalf after a redeploy was refused | The user presses Start | Correct: training spends the user's limits |

### Operational findings

- **Where an answer's time goes.** About 5.0 s per agent answer, of which about 4.9 s is the Haiku translation and only about 0.1 s (0.3 s at most) the agent's own reasoning, learning and vault update. The teacher takes about 5.3 s per message.
- **Thinking dominates translation time.** A translation produces 300–1,700 thinking tokens before a few lines of output. `--effort low` roughly halves the time (about 5–6 s instead of about 10 s at high effort). `MAX_THINKING_TOKENS=0` does not turn thinking off.
- **Caching only works in the system prompt.** With the language spec in the user message, nothing is cached (cache read 0). With the spec as the system prompt, the second call reads about 4,400 tokens from cache.
- **Throughput.** The agent's own update takes about 50 ms per message even on the full vault, so the shared lock isn't a bottleneck once translation happens outside it.
- **Judge noise.** Three judges move the average by a point per judge, so changes need at least 8 judges to be visible.

---

## 4. Architecture and implementation review

Scope: `DESIGN.md`, `PROGRESS.md`, `docs/parser_spec.md`, `docs/dialogue_format.md`, every source file of the seven crates (about 11,300 lines of Rust), `tools/live`, `crates/eval/ui/trainer.html`, the 27 commits from `6d4aae5` (Milestone 1) to `99c6fa4`, and the state of the main `vault/`. `cargo test --release` passes: **68 `#[test]` functions in 21 test binaries, 0 failures**. Several of these tests hold many cases each: the reasoner suite has 54 cases in one function, the NLG test 100 replies, and the parser test 200 + 100 sentences. Nothing that calls Claude was run for this review.

---

### 1. System overview

The design says the agent's reasoning, memory and learning are symbolic. Language goes in through a parser and comes out through a template generator. The code follows that shape. Since commit `a70311d`, though, an LLM translator in front of the parser handles every long or multi-sentence message.

```
                      user / teacher / judge message (text)
                                     |
                     Agent::respond  (crates/agent/src/lib.rs:232)
           pattern parser: split_sentences() at ". ! ?" and ", and" (lib.rs:1002)
           LLM parser: whole message at once
                                     |
             respond_one (lib.rs:266): reload vault if edited (mtime fingerprint),
             learn proper_name(x) from capitals, refresh name set
                                     |
          +--------------------------+------------------------------+
          | Parser trait (parser/src/lib.rs:155)                    |
          |  PatternParser: ELIZA-style tokens -> Intent + atoms    |
          |  LlmTranslator (parser/src/llm.rs):                     |
          |    >10 words or >1 sentence -> Claude CLI (Haiku)       |
          |    else pattern first; "suspicious" parse -> Claude     |
          |    output = canonical lines, parsed back + validated,   |
          |    cached in vault/llm_cache.tsv                        |
          +--------------------------+------------------------------+
                                     |  Vec<Intent>: tell / ask / recall / recall_more /
                                     |  why / correct / yes / no / social / about_you /
                                     v  greet / bye / how_are_you / unknown
                     Agent::handle (lib.rs:458), consumes `pending`
      +------------+---------------+--------------+------------------+-------------+
      | tell       | ask / why /   | correct      | yes / no         | social etc. |
      |            | recall        |              | (answer pending) | (canned     |
      v            v               v              v                  |  text)      |
  KnowledgeBase  Reasoner:      learner::        assert fact /       |             |
  assert_fact    evaluate_kb()  corrections::    confirm or reject   |             |
  (core/kb.rs)   semi-naive,    correct_derived  hypothesis rule     |             |
  - confirm      stratified     (refine rule)    (status change)     |             |
  - close p/-p   NAF, proofs    or retract fact                      |             |
  - functional   (reasoner/                                          |             |
    keys         lib.rs)                                             |             |
      |                                                              |             |
      v                                                              |             |
  Learner::update (learner/lib.rs:119): features -> hypothesis rules |             |
  (conf = e/(e+c+5), promote at 0.8, reject at c >= 2e)              |             |
      +-----------------------------+--------------------------------+-------------+
                                    v
                    Act (nlg/src/lib.rs:28) -> Realizer::realize -> Reply{text, intent, claims}
                    aggregation, pronouns, used-to tense, why-from-proof
                                    |
          Curiosity (agent/src/curiosity.rs): goals = contradictions (1.0),
          hypotheses (0.5-0.9), gaps (0.3-0.5); drive level; plan() -> one
          question + Pending slot; appended to the reply (cap 8/day) or
          proactive (cap 3/day) or asked to a Teacher
                                    |
          Vault save (dirty files only) + audit log line per change and per turn
          vault/facts/<entity>.md   vault/rules/<id>.md   vault/core/*.md (read-only)
          vault/log/<date>.md       vault/llm_cache.tsv
                                    |
                                    v
                                  reply
```

The knowledge model is flat Datalog. A `Value` is either an `Int` or a `Sym`. An `Atom` is a predicate plus terms, and there are no compound terms. A predicate starting with `-` is strong negation. A body literal `not p(..)` is negation as failure, and comparisons `= != < <= > >=` are allowed in bodies. Each `Fact` carries `@from`, `@until`, `@conf`, `@source`, `@uses` and `@last` (`core/src/term.rs:271-347`). A rule file has front matter with `id`, `status` (hypothesis, confirmed or rejected), `conf`, `evidence`, `counter`, `module` and `source` (`core/src/vault.rs:112-178`). The core ontology has 21 `core_predicate` declarations plus `functional(property, 2)` and `functional(located_in, 1)` (`crates/core/assets/core/ontology.md`). It also has 9 core rules: transitivity of `is_a`, `part_of` and `before`; symmetry of `same`; and inheritance of `has` and `property` with `not -has` / `not -property` exceptions (`assets/core/rules.md`).

Every answer is computed from scratch. `Agent::model()` (`agent/src/lib.rs:167`) evaluates all rules over all facts valid at "now". In history mode it evaluates over every fact ever stored. Nothing is materialized between messages.

---

### 2. Milestones versus DESIGN.md

| # | Design milestone | What was built | Acceptance test (file: what it checks) | Status | Deviations and why |
|---|---|---|---|---|---|
| 1 | Vault and data model; lossless round trip of 100 files | `core/src/{term,syntax,time,doc,vault,kb}.rs`. Markdown with fenced `datalog` blocks; unmodified lines keep their raw text; invalid lines are kept and reported; CRLF is detected; Hinnant civil-date time | `core/tests/roundtrip.rs`: generates 70 fact files and 30 rule files with an LCG, parses them, modifies some, writes them back, and asserts that only the intended lines differ. `doc.rs` and `vault.rs` unit tests cover lossless, CRLF, no trailing newline, and single-line change | Passing (`6d4aae5`) | Packages are prefixed `sla_` because `core` clashes with Rust's built-in crate. "Zero external crates" was the stated policy, later broken (serde/serde_json in `sla_parser` and `sla_eval`, `103f7fd`). `kb.rs` landed early because `lib.rs` referenced it |
| 2 | Datalog with semi-naive evaluation, stratified negation and proof traces; at least 50 cases | `reasoner/src/lib.rs` (815 lines): compiled rules, safety check, stratification, semi-naive delta rounds, first-derivation proofs, `contradictions()` | `reasoner/tests/suite.rs`: 54 table cases in `suite()` (including recursion/ancestor and negation), plus `proof_trace_ancestor`, `proof_shows_negation`, `unstratifiable_rejected`, `unsafe_rules_rejected`, `confidence_is_minimum`, `contradictions_detected`, `time_bounded_facts`, `rules_loaded_at_runtime_from_text`. `all_proofs_ok` checks that every derived fact has a proof | Passing (`efe7d88`) | "Time-bounded facts" means facts are filtered by validity before evaluation. Rules do not reason over time. Derived confidence comes from the first derivation found (PROGRESS: "revisit if it matters") |
| 3 | Time and confidence; "I don't drink coffee anymore" closes the old fact and history still finds it | `kb.rs::assert_fact`: `p` vs `-p` closes the old fact; a functional key closes the old value; confirmation sets `c + (1-c)/2`; derived confidence is the min of the inputs | `reasoner/tests/time_conf.rs`: `dont_drink_coffee_anymore`, `derived_confidence_and_time`, `core_rules_apply`. `kb.rs` unit tests: negation closes, functional value replaced, qualities not functional, repeat confirms, usage tracking | Passing (`82dd2c5`, tests and PROGRESS only) | `property(_, quality, _)` is hard-coded as many-valued (`kb.rs:192-194`) |
| 4 | Pattern parser for tell/ask/correct/why/yes-no; at least 90% on 200 sentences; `Parser` trait | `parser/src/{lib,words,lexicon}.rs` (about 1,350 lines) and `docs/parser_spec.md`. Also intents the design did not list: `recall`, `recall_more`, `social`, `about_you`, `greet`, `bye`, `how_are_you` | `parser/tests/dev_set.rs`: dev 200/200 exact; held-out 98/100 exact (100/100 intent). The held-out test prints only the score. The data was written by a sub-agent from the spec alone | Passing (`cc1b389`) | 100% on dev shows the patterns were tuned against the dev set. The held-out set is the honest number. Both sets are short template sentences. Real chat later reached about 56% understood (trainer snapshot: 38 of 68 turns) |
| 5 | Template NLG; grammatical; never states anything not in the KB | `nlg/src/lib.rs` (947 lines): `Act` enum, `Realizer`, aggregation, pronouns, used-to tense, why-from-proof, deterministic rotation of variants | `nlg/tests/replies.rs::hundred_replies`: 100 acts. `grammar_problems` is a heuristic checker (capital letter, final punctuation, no `_ ( ) :-`, ...). Every claim must be true in the model, and each claim's sentence must parse back to the same atom. `explanation_uses_proof` | Passing (`5037e70`) | "Grammatical" is checked by surface heuristics, not a grammar. `Act::Note` free text bypasses the claims mechanism (`nlg/src/lib.rs:858`) |
| 6 | CLI agent loop; 10 facts + 2 rules recalled after restart | `agent/src/{lib,main,router}.rs`; `sla` binary with `:facts :rules :log :reload :demo`, `--script`, `--llm` | `agent/tests/session.rs`: `teach_then_recall_after_restart`, `contradiction_and_history`, `hand_edits_are_reloaded`, `correction_refines_rule_in_conversation` | Passing (`a666d66`) | The module router only labels rule files. `modules_for` exists (`router.rs:26`) but nothing calls it, so questions are never routed |
| 7 | Pattern-counting learner; hypothesis after 5 mornings; confidence drops on a counterexample | `learner/src/lib.rs`: entity features, best single-feature body per head, `conf = e/(e+c+k)` with k=5, promote at 0.8, reject when `c >= 2e`, demote | `learner/tests/mornings.rs`: `five_mornings_then_counterexample`, `promotion_past_threshold` (20 cases), `rejected_when_counterexamples_dominate` | Passing (`1a6a0f4`) | Single-feature bodies only. PROGRESS notes that with no contrast the learner cannot tell `event`, `agent_of` and `during` apart, so the tests add evenings with tea |
| 8 | Corrections and demonstrations; "changes the right rules and nothing else" | `learner/src/corrections.rs`: `correct_derived` (condition `not feature(X)` or exception `not -head`; core rules read-only), `learn_from_demo` (before/after diff -> effect rules; repeated demos intersect preconditions) | `learner/tests/corrections.rs::correction_set` over `data/synthetic/corrections/cases.md` (10 cases with expected changed files, expected rule text, `also_false`, `keeps`, `stored`; everything else must still be believed); `demonstrations_become_rules` | Passing (`79c1d44`) | The design wanted corrections to come from Claude teacher sub-agents. The test uses a scripted case file, and live teachers arrived with M10 |
| 9 | Curiosity: gaps, contradictions, weak hypotheses; asks about a contradiction and a hypothesis | `agent/src/curiosity.rs`: scored goals, drive level with decay and reward, `plan()`, `Teacher` trait, `ScriptedTeacher`, daily caps, an asked-set restored from the log | `agent/tests/curiosity.rs`: `asks_about_contradiction_then_hypothesis`, `gaps_and_caps` | Passing (`d22ce55`) | "HTN/GOAP planner" is one `match` from goal to question (`curiosity.rs:174-183`). There is no decomposition and no multi-step plan |
| 10 | Synthetic dialogues; eval crate; `history.csv`; Haiku judges; one command | `eval/src/lib.rs`; 12 dev and 6 held-out teaching dialogues; `docs/dialogue_format.md`; `ClaudeTeacher`, `run_judge_with`, `history.csv`, `eval/reports/` | `eval/tests/one_command.rs`: `run_appends_a_row` (runs twice with `use_claude = false`, checks header + 2 rows, repeatable scores, `last_report.md`), `dialogue_format`, `checks` | Passing (`e16d620`); baseline row in `history.csv` | The Claude path (teachers, judges) has no automated test. Baseline conversation quality was 3.33/10, then 3.00, 4.25 and 3.88 across later rows. The live-teacher metric asked 0 questions, then 1 |
| 11 | Optional: ILP, LLM translator, forgetting | LLM translator (`a70311d`). Not in the design: Claude pretraining (`4c06c95`), natural and chat modes (`75ac6fd`, `6d26abd`), live monitor (`0ce89b2`), trainer app (`103f7fd` onward) | `llm.rs` unit tests: `canonical_round_trip`, `dev_set_expectations_round_trip`, `garbled_parses_are_suspicious`, `pattern_first_no_model_call`. Nothing tests the trainer or pretraining | Partial | No ILP. No forgetting: only `@uses` and `@last` are tracked. **PROGRESS.md was last updated in `a5568fd`.** It still says "Next: milestone 11", and it does not document the translator, pretraining, trainer or the new dependencies |

Design items that were never built, outside the milestone list:

- **Confirming important facts back to the user before storing them** (DESIGN, Core components 1). Facts are stored immediately.
- **Chunking** of solved reasoning chains into shortcut rules.
- **Merging duplicate rules, grouping items into concepts, pruning.**
- **Rollback from the audit log.** The log is append-only text and nothing replays it.
- **Routing questions to modules.**

---

### 3. Components in depth

#### 3.1 Core: data model, vault and knowledge base (`crates/core`)

- **Lossless round trip.** `Document` (`doc.rs`) stores segments: text kept verbatim, or a `Block` with `open` and `close` fence strings and `Line`s. A `Line::Item { item, raw: Some(text) }` renders back as `raw` until the item is modified. `update_items` sets `raw = None`, so only changed lines are re-rendered (`doc.rs:747-758`). Invalid lines become `Line::Invalid { raw, error }` and are written back unchanged. The newline style and the trailing newline are recorded at parse time. Front matter keeps its raw text until `set()` changes a value (`vault.rs:56-78`). After that the whole block is regenerated, so hand-written comments or ordering in front matter are lost.
- **Layout.** Each fact goes in the file named after its first argument (`entity_of`, `vault.rs:241`), so `-likes(anna, x)` lives in `anna.md`. Integer subjects go to `_numbers.md`. Rules get one file per id. The core files are copied into `vault/core` on first open and are read-only for the learner. The log is one Markdown line per change: `- time | action | subject | reason` (`vault.rs:431-443`).
- **`assert_fact`** (`kb.rs:168-237`):
  1. If the same open fact exists, confirm it: `conf = c + (1-c)/2`.
  2. Otherwise, close every open fact that conflicts with the new one. A fact conflicts if it is the complement (`p` vs `-p`), or if it has the same functional key and a different value. Only positive values compete for a key. Closing sets `@until = from`.
  3. Add the new fact.

  All conflicts are found by linear scans of `all_facts()`. Each call to `functional()` re-reads the core rule files.
- **Hand edits.** The agent fingerprints the mtime, size and path length of every vault file (`agent/lib.rs:65-81`). If the fingerprint changed, it reloads the whole vault before the next message.

#### 3.2 Reasoner (`crates/reasoner/src/lib.rs`)

- **Compile.** Variables become slots. The safety check requires every head, negated or compared variable to occur in a positive body atom (`compile`, lines 133-235). Filters (`not` and comparisons) are moved to just after the positive atom that binds their last variable.
- **Stratify** (lines 655-689). Iterative stratum assignment: a negated dependency forces `stratum(head) >= stratum(neg) + 1`. If the stratum count exceeds the number of IDB predicates plus one, the program is unstratifiable.
- **Semi-naive evaluation** (lines 699-747). For each stratum, round 0 fires every rule against the full model. Each later round fires a rule once for every positive body position whose predicate is in this stratum and in the last delta, using the delta relation at that position and the full relations elsewhere. New tuples become the next delta (`commit`). Relations are `Vec` + `HashSet` + a **first-argument-only index** (`Rel`, lines 240-269).
- **Proofs.** A `Derivation` is either `Given{conf, source}` or `Rule{rule index, premises, absent (NAF atoms checked), conf}`. Only the **first** derivation is kept (`fire`, lines 761-774). `proof()` rebuilds the tree recursively and can never loop, because a premise is always inserted before its conclusion. `Proof::render`, `leaves()` and `rules_used()` back the "why" answers and usage counting.
- **Confidence** = the minimum of the rule's confidence and the premises' confidences (line 771). Duplicate given facts keep the higher confidence.
- **Time** is applied outside the engine. `facts_for(facts, Some(t))` keeps facts valid at `t`. `None` means "every fact ever stored" (lines 780-789).

#### 3.3 Parser (`crates/parser`)

- **Pattern parser** (`lib.rs:194-356`), in order:
  1. Exact-phrase lexicons for social moves, then filler stripping.
  2. Greetings, which recurse on the rest of the message.
  3. Fixed phrases: yes, no, bye, "I don't know", "that's wrong".
  4. Correction leads ("no", "actually", ...) followed by a statement.
  5. Recall and recall-more leads, then why / how-do-you-know, then `if ... then`.
  6. Wh- and yes/no questions, then statements.

  A statement is split into subject and verb by `subject_end()` (lines 479-509). It uses an `AUX`/`VERBS` lexicon and falls back to "a word, then a word ending in -s". Verb phrases map to core predicates (`like/love/enjoy -> likes`, `have/own -> has`, `live in -> located_in`, ...). Any other verb becomes `lemma` or `lemma_prep` (`work_at`). Plural or "every" subjects with a verb other than have/be become rules over `is_a(A, category)`. With has, is_a, property, feels, located_in or part_of they stay facts about the category (line 737). Entities are lowercase, joined by `_`, singularized, and limited to 4 words (line 427).
- **`if/then`** uses a thread-local `IN_RULE` flag (lines 438-444, 927-943) so that pronouns become the variable `A` inside rules.
- **Context** is `{speaker, last_entity}`. He, she, it and they resolve to `last_entity`.

#### 3.4 LLM translator (`crates/parser/src/llm.rs`, `claude.rs`)

- **Routing** (`parse_many`, lines 228-252):
  - A message of more than 10 words or more than one sentence goes to the model as a whole.
  - Otherwise the pattern parser runs first.
  - If the pattern parse is not `Unknown` and not `suspicious()`, it is used.
  - If it is suspicious, the model's reading is used, with the pattern parse as fallback.
  - If it is `Unknown`, the message goes to the model.
- **`suspicious`** (lines 116-132) flags a parse whose entity has at least 4 `_` parts, or contains a glue word (`and`, `named`, `she`, `is`, ...). It also flags a predicate with 3 or more `_` parts.
- **Validation.** The model must return canonical lines from the spec. The full `parser_spec.md` is embedded in every prompt with `include_str!`. Each line goes through `from_canonical` (lines 17-85), which requires ground facts for `tell` and valid Datalog. Then `acceptable` (lines 88-105) checks:
  - predicate names are lowercase and at most 40 characters;
  - every fact has arguments;
  - every rule head contains a variable;
  - ask queries use only the variables `W` and `_`.

  Nothing checks predicates against the ontology. Rule safety is left to `add_rule`, which evaluates the program.
- **Caching.** The key is `speaker|last_entity|text`. The cache file `vault/llm_cache.tsv` is append-only, uses `\u{1f}` as separator, and holds 938 entries. Only **non-empty** results are cached (line 210).
- **`claude::call`** runs `claude -p --model M --output-format json --system-prompt <short> --tools "" --no-session-persistence --strict-mcp-config` with the prompt on stdin. It parses the usage fields and `total_cost_usd`, and records them by role in a global `Mutex<BTreeMap>` and in a thread-local map (`take_thread_usage`), which feeds the trainer's per-teacher accounting.

#### 3.5 Learner (`crates/learner`)

- **Features** (`lib.rs:78-99`). The learner evaluates the KB with core rules and non-learner confirmed rules; learned rules are excluded so they cannot support themselves. Every derived fact with at most 3 arguments becomes a feature of its first argument: `drink(X, coffee)` for entity `m1`.
- **Counting** (lines 102-116). Support counts entities that have the body and the head. A counterexample is an entity that has the body without the head. This is closed-world, except for `knows`, `wants` and `has`, where only an explicit `-head` counts (line 252).
- **Proposal** (lines 171-216). For each head feature with count >= 3 that is not in `NO_HEAD`, the learner tries every body feature with count >= 3 and a different predicate. A candidate needs support >= 3, *stated* support >= 3 (so it is not just a consequence of existing rules), and support / body-count >= 0.75. The best candidate by (ratio, specificity, constants) becomes `hyp_<pred>` with `status: hypothesis`, `source: learner`.
- **Re-scoring** runs on every `tell` that adds facts (`agent/lib.rs:686-688`). It applies `conf = e/(e+c+5)`, rounded to 3 decimals:
  - promote to confirmed at 0.8 or above (20 clean cases);
  - reject when `c >= 2e` and `c >= 3`;
  - demote a confirmed learned rule to hypothesis when `c > e`.

  Every change goes to the rule front matter and to the log.
- **Corrections** (`corrections.rs:57-152`):
  1. Evaluate and take the proof of the wrong atom. If it is a `Given` fact, the caller closes the stored fact.
  2. If the top rule is core, only store `-wrong`. This leaves a contradiction for curiosity to ask about.
  3. Otherwise compare the wrong entity's features with the features of the rule's other correct cases. Pick a distinguishing feature, preferring `is_a` and then single-argument features, and add `not feature(X)`. If none exists, add `not -head`.
  4. Always store `-wrong`, increment `counter`, and re-evaluate. If the program became invalid, revert.
- **Demonstrations** (`corrections.rs:164-222`):
  - The action's constants become the variables `A`, `B`, `C`, ....
  - Preconditions are the facts about participants present both before and after.
  - Each added fact becomes a rule head; each removed fact becomes a `-head`.
  - Heads whose variables are not bound by the body are skipped.
  - A repeated demonstration intersects the preconditions and adds 1 to evidence. Confidence uses the same formula.

#### 3.6 Curiosity (`crates/agent/src/curiosity.rs`)

- **Goals** (lines 75-94):
  - `ResolveContradiction` (score 1.0), from `Model::contradictions()` (both `p` and `-p` derived);
  - `ConfirmHypothesis` (0.5 + 0.4·min(evidence/5, 1));
  - `FillGap` (0.3 + 0.2·fraction).

  Gaps (lines 98-155) group members by category (`is_a`, `person`, plus "named things" = proper names that are subjects). For predicates in `GAP_PREDS = [likes, located_in, property:<name>, work_at]`, a category member lacking a predicate that more than half of at least 3 members have (and at least 2 holders) is a gap. Goals already asked are filtered out using `asked`, which is restored from the log.
- **Drive** (lines 158-170). `level = 0.7·level + 0.3·top_score + 0.05·min(#goals, 4)`, capped at 1. A reward of 0.3 (an answer to the agent's own question) or 0.05 (any new fact) lowers it. The agent asks only when `top >= 0.3` and `level >= 0.3`.
- **Plan** (lines 174-183). Maps a goal to `(Act::Ask*, Pending::*)`. In chat, goals are filtered to entities in the current turn's claims or `last_entity` (`agent/lib.rs:367-381`). Caps are counted by re-reading the log (`asked_today`, lines 387-394).

#### 3.7 NLG (`crates/nlg/src/lib.rs`)

- **Realization.** An `Act` is turned into a `Reply { text, intent, claims }`. `vp_terms` maps predicates to verb phrases. Core predicates have fixed templates. Unknown predicates are split on `_`: if the first word lemmatizes to a known verb it is used as the verb (`work_at` -> "works at"). Otherwise the predicate reads as a preposition, a noun relation ("Anna is an ancestor of Bob", using a `looks_like_noun` suffix heuristic), or a verb.
- **Morphology.** Third person and plurals are rule-based. `article` is a/an by first letter with a few exceptions. The speaker is "you", and `my_x` becomes "your x".
- **Aggregation** (line 405). Atoms are grouped by subject, then by predicate and middle arguments. Same subject and verb gives a joined object list. Repeated subjects become pronouns.
- **Why-answers** (`explain`, line 665). They walk the proof up to depth 3: "X because Y, and if A ... then ...". Given premises are grouped by source ("you told me that ...").
- **Faithfulness** is enforced by convention. Every asserted clause comes from an atom pushed into `claims`. The NLG test checks that each claim is in the model and that the claim parses back. Variation is deterministic rotation (`pick`, a `Cell` counter).

#### 3.8 Agent loop (`crates/agent/src/lib.rs`)

- **Dialogue state.** `ctx {speaker, last_entity}`, `last_answer` (the target of "why?" and "that's wrong"), one `pending` slot, and `last_recall` for "what else?" paging (8 items per page). LLM multi-intent results are de-duplicated, consecutive tells are merged, and noise is dropped when there is content.
- **Answering questions** (`ask`, lines 716-778):
  - A ground atom gets `Yes`, `No`, `WasTrue` or `Unknown`. `No` means the complement is in the model. `WasTrue` means the atom is in the history model.
  - A single atom with `W` gets the matches, falling back to history matches; `@history` returns only matches that are no longer true.
  - A conjunctive query returns the first premise of each answer.
- **Corrections** (lines 792-875). A stored wrong fact is closed and its negation stored. A derived one goes to `correct_derived`.
- **Usage** (`record_uses`) increments `@uses` and `@last` on the leaves of each proof used in an answer.

#### 3.9 Training and evaluation harness (`crates/eval`)

- **`run_dialogue`.** Gives each dialogue a fresh temporary vault and a fixed clock (+60 s per line). It checks the per-line expectations, then the expected facts (by model membership) and the expected rules. A stored rule satisfies an expected rule if the heads match and the expected body is a subset (`rule_satisfied`, `lib.rs:157`). An optional `ClaudeTeacher` phase follows.
- **Judges.** A Haiku judge role-plays a user for N turns, then scores 1-10. Judges run serially.
- **Pretraining** (`pretrain.rs`). Modes:
  - sentence lists per topic, then a reviewer marks each stored item OK or WRONG; WRONG facts are closed and WRONG rules rejected;
  - lesson-style conversations (`teacher_prompt`);
  - "natural" teacher conversations through the translator;
  - "chat" between friends, with speaker `friend` and personal facts closed afterwards.

  The parallel conversation modes run on one shared vault, in a `std::thread::scope` pool with a shared `Mutex<()>`.
- **Trainer** (`trainer.rs`). N teacher threads (clamped to 1-24) each pull scenario numbers from an `AtomicUsize`. Each opens its own `Agent` on the main vault with the LLM translator. Per turn:
  1. Call the teacher model.
  2. `agent.prepare(msg)` translates outside the lock.
  3. Under `agent_lock`: `reload_if_edited`, `respond`, and diff the facts and rules.

  At the end of a conversation, personal facts are closed and the reviewer runs. State goes to a `Mutex<TrainState>` that the UI polls. The trainer also tracks per-teacher usage through `take_thread_usage`, answer, translate and think times, and plan limits scraped from a headless `/usage` call (`fetch_limits`, lines 146-171).

---

### 4. Tooling built along the way

| Tool | What it is | Notes |
|---|---|---|
| `sla` (`agent/src/main.rs`) | REPL / `--script` runner with `:facts`, `:rules`, `:log` (last 20 entries), `:reload`, `:demo ACTION \| before: ... \| after: ...`, `--speaker`, `--source`, `--llm` / `--llm-model` | `--script` is how teachers and judges originally drove the agent. `:demo` is the only way to give a demonstration; nothing in natural language reaches it |
| `sla_eval` (`eval/src/main.rs`, `lib.rs`) | One command: parser dev/held-out accuracy, 18 teaching dialogues, an optional live-teacher phase, N Haiku judges over 8 topics, `--llm`, `--seed-vault` (judges start from a vault copy), appends to `eval/history.csv` and writes `eval/reports/<ts>.md` and `last_report.md` | Parser accuracy is always measured with the **pattern** parser, even with `--llm`. History rows so far: knowledge 100% dev / 97.8% held-out; conversation 3.33, 3.00, 4.25, 3.88 |
| `sla_pretrain` (`eval/src/bin/sla_pretrain.rs`) | Cold start: sentence lists per topic (`--per-topic`, 10 default topics) with a reviewer; `--conversations` (40 topics, lesson style), `--natural`, `--chat` (40 chat topics), `--parallel` (default 6), `--turns`, `--rounds` | Reports go to `eval/pretrain/`. The main vault now holds 678 entity files, 2,171 fact lines (1,797 open) and 210 rule files (128 confirmed, 42 hypothesis, 40 rejected) |
| `sla_trainer` (`eval/src/trainer.rs`, `bin/sla_trainer.rs`, `ui/trainer.html`) | Hand-written HTTP/1.1 server on `127.0.0.1:8770`: `GET /`, `GET /api/state`, `POST /api/start`, `POST /api/stop`. The React 18 + htm UI from unpkg polls state and shows live teacher chats, plan limits (`/usage`, refreshed every minute), and per-message translate/think times | 30 everyday "assistant" scenarios. One thread per connection, no auth or origin check. `eval/trainer_snapshot.json` (untracked) holds a run: 8 conversations, 68 turns, 38 understood, 44 facts, 6 rejected |
| `tools/live/live.py` + `live.html` | Python standard-library server (port 8765) that tails `vault/log/*.md` incrementally by byte offset and serves the feed, learning events and per-minute growth series | Read-only. Counts facts from `add`/`close` log events. `proper_name` facts are added without a log entry (`agent/lib.rs:183-191`), so its counts can drift from the vault |
| claude.ai dashboard | Mentioned in the brief | **Not in the repository.** It was likely published from the trainer state or snapshot, so it could not be reviewed as code |

---

### 5. Code-level weaknesses and technical debt

#### 5.1 Scaling: everything is recomputed and rescanned on every message

- **The whole model is re-evaluated several times per message.** `Agent::model()` (`agent/src/lib.rs:167-178`) runs a full fixpoint over all facts and rules on every call. Calls per message:
  - `ask` evaluates the current model, and then the history model once more at line 726 or line 744 (history mode, or when a wh-question finds nothing now);
  - `next_question_about` evaluates again for curiosity (line 406);
  - `tell -> Learner::update` evaluates again (`learner/lib.rs:91`);
  - `add_rule` evaluates (line 708);
  - `correct_derived` evaluates twice (`corrections.rs:59, 135`).

  A turn can therefore cost 3-5 full evaluations.
- **The reasoner has only a first-argument index** (`reasoner/lib.rs:240-269`). Joins on a bound second argument, such as `is_a(X, C), has(C, Y)` with `C` bound, scan the whole relation. Every model is built from scratch, so incremental maintenance is impossible.
- **`all_facts()` clones every fact in the vault.** There are 25 call sites across agent, learner and kb. Each `respond_one` calls it in `learn_names` (once per capitalized word, line 187), three times in `refresh_names` (lines 197-226, run twice per turn), in `facts_before` (line 359) and at the end of `respond_intent`. `assert_fact` scans it twice (`kb.rs:175, 212`). `functional()` re-reads core on every assert (`kb.rs:115-127`). The trainer then diffs `all_facts()` before and after each reply with `Vec::contains`, which is O(n²) (`trainer.rs:501-504`).
- **Learner proposal is O(F² · E)** over distinct features F and entities E (`learner/lib.rs:179-216`). It runs on every `tell` that adds a fact. With 1,800 open facts it is still fine, but it grows quadratically.
- **The audit log is re-read in full** whenever the daily question cap is checked (`asked_today`, `agent/lib.rs:387-394`) and on every `Agent::open` (lines 110-116). The single day file already has 6,074 lines (864 KB). `count_vault` in the trainer reads every fact file every 3 s (`trainer.rs:230-260`).
- **Reads cause writes.** `record_use` marks every fact file it touches as dirty (`kb.rs:275-288`). Answering a question therefore rewrites vault files and changes the fingerprint, and in the multi-agent trainer that forces every other agent to reload the whole vault on its next turn (`agent/lib.rs:149-158`).

#### 5.2 Concurrency (pretraining and trainer)

- **Shared-vault design.** Several `Agent` instances hold separate in-memory copies of the same vault. Consistency depends on (a) a global `Mutex<()>` around `respond`, and (b) an mtime/size/path-length fingerprint triggering a full reload. `save()` writes whole files from the in-memory copy, so correctness needs a reload inside the lock before every write. Most paths do this, but the invariant is enforced by convention only.
- **Model calls can happen inside the global lock.** The translator caches only non-empty results (`llm.rs:210-212`). If the call made by `prepare()` fails or returns nothing valid, the same message is translated again inside `respond()`, while the lock is held. That serializes every teacher behind a 5-20 s CLI call.
- **The translation cache file is written concurrently.** Each agent's `LlmTranslator` appends to `vault/llm_cache.tsv` from its own thread with no lock (`llm.rs:165-172`). Each agent also keeps a private in-memory cache, so parallel teachers never see each other's cache hits.
- **The reviewer can close facts other teachers taught.** `review_and_fix` closes facts with `close_matching(new_facts[n])` (`pretrain.rs:347`). If another conversation also taught that atom, its copy is closed too. The reviewer judges the NLG rendering of the atom, not the source sentence, so a fact with an awkward but correct rendering can be closed.
- **The trainer API is open to any local web page.** No auth, no Origin/CSRF check, and one thread per connection (`trainer.rs:602-667`). Any page open in the browser could `POST /api/start` and spend plan quota.
- **`IN_RULE` is a thread-local mutable flag** in the parser (`parser/lib.rs:438-444, 927-943`). It works because parsing is single-threaded per call, but it is hidden global state and breaks if `statement` ever recurses into `if_then`.

#### 5.3 Correctness bugs and brittle spots

- **The fallback model drops core rules.** When evaluation fails, `model()` logs the error and evaluates `facts_for(..)` with **no rules at all** (`agent/lib.rs:172-175`). The comment says "evaluate without learned rules". One bad hand-edited rule silently removes all inheritance and transitivity.
- **The correction revert path leaves partial state.** When a refinement makes the program invalid (`corrections.rs:135-145`), the rule text is restored, but:
  - the `-wrong` fact has already been stored (line 115);
  - `counter` has already been incremented (line 131);
  - the function returns `Change::Exception` although no exception literal was added.
- **History-mode reasoning is unsound.** `facts_for(.., None)` evaluates every fact ever stored together (`reasoner/lib.rs:780-789`, used at `agent/lib.rs:726, 744`). Rules can join premises that were never true at the same time, and `p` and `-p` from different times coexist. So "used to" answers can be wrong.
- **"Used to" statements skip contradiction handling.** They are written with `vault.add_fact` and `from = None` (`agent/lib.rs:655-665`), so a current fact is never compared against them.
- **Over-retraction on correction.** "No, Anna likes coffee" after the answer `likes(anna, tea)` retracts `likes(anna, tea)`. Any same-predicate, same-subject last answer is treated as exclusive, even for non-functional predicates (`agent/lib.rs:817-821`).
- **One-shot generalization from a single correction.** The condition chosen for a refinement is the first distinguishing feature after sorting by (`is_a` first, arity, name) (`corrections.rs:101`). One counterexample can permanently add `not is_a(X, penguin)` or an arbitrary incidental feature. The design wanted confidence to move gradually.
- **Pending state is fragile.** `handle()` takes `pending` at the start of every intent (`agent/lib.rs:459`). Any unrelated message, including a reaction, discards the open question. With several LLM intents, only the first one sees it.
- **The sentence count is fragile.** `split_sentences` rewrites ", and " and ", but " into sentence breaks globally (`agent/lib.rs:1004`). This breaks lists ("tea, and coffee"). The LLM routing counts `. ! ?` characters (`llm.rs:232`), which also counts decimals and abbreviations.
- **The live-teacher consistency check is lenient.** It only fails a learned fact whose complement is in the ground truth. The second disjunct at `eval/src/lib.rs:221` is a subset of the first. A fact that is invented but not contradicted counts as "consistent".
- **The plan-limit scraper depends on display text.** `fetch_limits` parses human-readable `/usage` output by looking for lines starting with "Current " and splitting on "%" and "resets " (`trainer.rs:159-166`). Any change in the CLI's wording breaks it silently.

#### 5.4 Hard-coded heuristics and where symbolic purity is compromised

- **The LLM is now the de facto parser.** The design calls the LLM optional, used "never as the reasoner". In practice every message over 10 words goes to Haiku (`llm.rs:233`), and the trainer and chat modes always enable it. The translator is not just translating. Its prompt asks it to decide what counts as a world statement, to drop "opinions, hypotheticals, meta talk", and to restate hedged generalities as plain facts ("'Jupiter is usually easy to spot' -> property(jupiter, quality, easy_to_spot)", `llm.rs:184-187`). The model is therefore the filter on what gets believed, and a 'usually' or 'most' claim is stored as an unqualified fact.
- **There is no ontology control on new predicates.** `acceptable()` checks only the predicate spelling (`llm.rs:88-105`). The main vault has **278 distinct predicates**: `grow_on`, `grow`, `grow_from`, `work_in`, `cover`, `bring`, and so on. `property(_, quality, _)` is a catch-all with about 200 values such as `easy_to_spot`, `soft_bodied` and `turning`. Learned rules use categories like `is_a(X, clothe)` (singularization artifact), `foods_from_plant` and `your_body`. There is also `is_a(here, first_fact)`. Nothing aligns synonyms or maps new predicates onto the core ontology, which the design calls "the most important design choice".
- **Lexicons and hand lists set behaviour in many places:**
  - parser: `VERBS`, `NOT_VERBS`, `BAD_IN_ENTITY`, `ADJ_LIKE_S`, emotions, colours, sizes (`parser/lexicon.rs`);
  - LLM: `GLUE` words (`llm.rs:110`);
  - NLG: `looks_like_noun` suffix list and hand list (`nlg/lib.rs:157`), and `article` exceptions (line 119);
  - agent: `FUNCTION_WORDS` and the capital-letter proper-name heuristic (`agent/lib.rs:935-988`; the vault contains `proper_name(add)`);
  - learner: `NO_HEAD`, `IGNORE` and `is_open_world` (`learner/lib.rs:35-37, 252`);
  - curiosity: `GAP_PREDS` (`curiosity.rs:46`);
  - `refresh_names`: source-label tests such as `starts_with("teacher")` (`agent/lib.rs:211`);
  - kb: the `quality` many-valued exception (`kb.rs:192-194`);
  - trainer and chat: the "personal" filter `x == "friend" || x.starts_with("my_")` (`trainer.rs:543`, `pretrain.rs:423`).
- **Module routing is duplicated, inconsistent and unused.** `agent/router.rs:5` and `learner/lib.rs:287` map predicates differently: the learner sends `is_a` to "general", the router to "things". `modules_for` has no callers.
- **The NLG depends on the parser crate** for `VERBS` and `lemma` (`nlg/lib.rs:9`), and the parser crate now depends on serde and spawns `claude` processes (`parser/src/claude.rs`). The core NL path is coupled to the Claude CLI.
- **Confidence barely affects behaviour.** Yes/no answers ignore `conf`; a 0.3 fact answers "Yes". Hypothesis rules never take part in answers (`model()` passes `include_hypotheses = false`). Confidence exists in the data but has almost no effect on what the agent says.
- **`Act::Note` bypasses the claims mechanism.** Social, "how are you" and about-you replies are free text with no claims (`nlg/lib.rs:858`). `about_me` also rewrites a recall reply by splitting at the first `:` (`agent/lib.rs:608`).

#### 5.5 Missing tests and stale documentation

- **No tests for the newest code.** `trainer.rs` (667 lines), `pretrain.rs` (492), `claude.rs` (114) and the trainer UI have none. The translator's model path (caching, validation of model output, the suspicious fallback order) has no mock-based test. The Claude branches of `sla_eval` (teacher phase, judges) are untested.
- **Acceptance tests are template-shaped.** Dev and held-out parser sets and the dialogues all use the short sentence patterns. The roughly 44% "not understood" rate in real trainer chats (snapshot: 38 of 68 turns understood) is not measured by any fixed test, and parser accuracy is never computed for `--llm`.
- **PROGRESS.md stops at `a5568fd`.** It does not describe the translator, pretraining, chat mode, the trainer, the dependency change, or the current vault size. It still says "Zero external crates" and "Next: milestone 11".
- **The repo contains mutable runtime state.** `vault/` (fact files, a 6,000-line log, the LLM cache) is committed and partly untracked (`git status` shows modified `vault/facts`, `vault/log` and hundreds of new files). This mixes code history with knowledge history, and a rollback from git is all-or-nothing.

---

### 6. What the architecture cannot currently represent or do

| Capability | Evidence |
|---|---|
| **Procedures, steps, recipes** | No sequence or plan structure. `before/2` exists in core, but the parser only emits it as `come_after(...)` style verbs, never as ordered steps. Demonstration rules are timeless implications (`go(A,B) -> -at(A,home)`) with no situation or state-transition semantics: once the action fact is stored, its effects hold forever. The trainer's own teacher prompt asks for "the steps in order" and "how long it takes" (`trainer.rs:280-281`). The snapshot shows the result: for the bicycle flat-tire scenario the agent replied "Okay, I'll remember that. Your bike has flat tire." and then "I'm not sure what you mean." / "Could you say that more simply?" when the teacher gave the steps |
| **Tasks and user goals** | `Goal` has only three epistemic variants: contradiction, hypothesis, gap (`curiosity.rs:22-26`). `wants(X, Y)` is just a stored fact, and no planner acts on it. All 30 trainer scenarios are help requests ("plan dinner", "make a checklist"). The agent can only store them ("Beach checklist has sunscreen, water, hat, sunglass, towel and snack.") or answer "Hello!" to "Can you make me a packing checklist?" |
| **Dialogue state beyond one slot** | State is `{speaker, last_entity}`, `last_answer`, `last_recall` and one `Pending`, which is cleared by the next message (`agent/lib.rs:36-62, 459`). There is no conversation history, no question under discussion, no tracking of what was already said to whom, and no multi-turn task frame. The 10-word routing also means the translator never sees earlier turns, only `last_entity` |
| **Numbers, quantities, units, arithmetic, aggregates** | `Value::Int` and comparisons exist (`term.rs:12, 196-227`), but the parser only turns a bare integer token into an entity (`parser/lib.rs:419`). There are no arithmetic built-ins, no `count/sum/min`, no units, no "how many/how long" intents. Durations and amounts become symbols such as `ten_minutes` or `property(x, quality, ...)` |
| **Nested or propositional attitudes** | `knows(X, F)`, `believes(X, F)` and `says(X, F)` are in the core ontology, but terms are flat: there are no compound terms and no reified facts, so `F` can only be a symbol. "Anna knows that Bob likes tea" cannot be represented. Sources are a string label on facts, not `says/2` facts |
| **n-ary and multi-entity relations** | Rules from the parser have one variable `A`. Learner features use one case variable `X` on the first argument and skip arity above 3 (`learner/lib.rs:93`). Corrections and gaps also assume the first argument is the entity. Relations between several entities, such as "Anna gave Bob a book at Christmas", need events, and the parser never creates event entities (`event/agent_of` are used only in tests) |
| **Time in reasoning** | Validity intervals exist only on stored facts. Derived facts have no interval, rules cannot mention time, history-mode reasoning mixes epochs (section 5.3), and tense is only "now" or "used to". Relative or absolute time in language ("yesterday", "in 2020", "every morning") is not parsed into `@from/@until` or `during` |
| **Graded or hedged generalization** | "Most", "usually" and "some" are stripped. A plural generic becomes a universal rule (`Cats like fish -> likes(A, fish) :- is_a(A, cat)`). Exceptions arrive only through `not -p` / `not is_a(X, c)` after a correction. Confidence is not used in answers |
| **Rule induction beyond one body literal** | No ILP. Bodies have one positive feature plus `not` conditions added by corrections. No multi-literal conjunctions, no relational paths, no recursion learned |
| **Concept formation, merging, forgetting** | No grouping of similar items, no merging of duplicate or synonymous predicates or rules, no pruning. Forgetting is not implemented: only `@uses` and `@last` are tracked |
| **Asking about anything but the three goal types** | Curiosity cannot ask for clarification of a message it did not understand. It cannot ask "what do you mean by X?" or ask to confirm an LLM translation before storing it. The design's "confirm important new facts back to the user" is absent |
| **A model of itself** | Self-descriptions are canned strings (`agent/lib.rs:583-646`). The agent has no facts about itself, its abilities, or past conversations |

**Bottom line.** The symbolic core does what milestones 1-10 asked: a lossless vault, a correct stratified semi-naive engine with proofs, time-closing contradictions, counting, correction and demonstration learning, and curiosity with caps. It is tested to the letter of those milestones. Two problems sit on top of it:

- **Performance.** Every message recomputes the whole model several times and rescans every fact.
- **Understanding.** The LLM front end produces facts with no ontology control.

Recent work moved toward assistant-style training: everyday help scenarios and procedural teaching. The data model cannot store procedures, tasks, quantities, or dialogue state beyond one slot.

---

## 5. Results and data analysis

All numbers below were computed with Python scripts over the files in `eval/`, `vault/` and `data/synthetic/` as they stand on 2026-11-16 (nothing in the repo was changed). Report timestamps are UTC; git commit times are local (+02:00), so a commit at 21:42 local corresponds to 19:42 in the report names. Where a figure could not be computed from the data, this is stated.

### 1. Evaluation timeline

**Test sets used by the fixed tests** (`data/synthetic/`): parser dev set 200 sentences, held-out 100 (intent mix dev: 92 tell, 48 ask, 16 correct, 14 why, 8 yes, 8 recall, 4 no, 4 unknown, 3 greet, 3 bye); mean sentence length 4.1 words (dev) and 4.0 (held-out). 12 dev and 6 held-out teaching dialogues (96 and 47 checks), and 10 correction cases.

**Every evaluation run** (`eval/history.csv`, `eval/reports_pretrained_vault/history.csv`, report files):

| Run (UTC) | Commit | Vault at start | Parser dev / held-out | Dialogue checks dev / held-out | Knowledge dev / held-out | Teacher Qs (consistent) | Judges | Conv. score mean | median | min-max | SD |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 19:06:24 | e16d620 | empty, pattern parser | 100% / 98% | 96/96 / 45/47 | 1.000 / 0.978 | 0 (0) | 3 | 3.33 | 3 | 2-5 | 1.53 |
| 19:08:04 | e16d620 (+ fix) | empty | same | same | same | 0 (0) | 0 | - | | | |
| 19:09:35 | e16d620 (+ fix) | empty | same | same | same | 1 (1) | 0 | - | | | |
| 19:36:48 | 75ac6fd | empty, LLM translator | same | same | same | 0 (0) | 3 | 3.00 | 3 | 2-4 | 1.00 |
| 19:42:56 | 40354cd | empty, LLM | same | same | same | 0 (0) | 8 | 4.25 | 4.5 | 2-6 | 1.75 |
| 19:42:56 | 40354cd | pretrained vault, LLM | same | same | same | 0 (0) | 8 | 2.38 | 2 | 2-4 | 0.74 |
| 19:49:58 | 40354cd | pretrained vault | same | same | same | 0 (0) | 0 (no judge scores) | - | | | |
| 19:51:19 | e7f9705 | empty, LLM | same | same | same | 0 (0) | 8 | 3.88 | 3.5 | 3-6 | 1.13 |
| 19:51:19 | e7f9705 | pretrained vault, LLM | same | same | same | 0 (0) | 8 | 3.88 | 3.5 | 2-6 | 1.64 |
| 20:20:44 | 6d26abd | pretrained vault | same | same | same | 0 (0) | 0 (no judge scores; not in any history.csv) | - | | | |

Observations:

- The fixed knowledge tests never moved: parser 200/200 dev and 98/100 held-out, dialogue checks 96/96 and 45/47 (the 2 held-out failures are both in "The chefs' kitchen"), knowledge score 100% / 97.8% in all 10 runs. They measure the short-sentence pattern path, which none of the later commits touched. They do not exercise the LLM translator.
- The live-teacher phase asked 1 question in 1 of 10 runs (19:09:35, 1 of 1 consistent with ground truth), and 0 in the other 9.
- Every judge conversation lasted 6 user turns (38 judges × 6 = 228 user turns). Judge messages average 10.8 words, compared with about 4 words in the parser test sets.
- Conversation score trend (mean of n judges): 3.33 (n=3), then 3.00 (n=3), then 4.25 (n=8), then 3.88 (n=8) with an empty vault. With the pretrained vault: 2.38, then 3.88 (n=8). All 38 judge scores fall between 2 and 6. Distribution: 2 ×12, 3 ×10, 4 ×6, 5 ×4, 6 ×6. Pooled mean: 3.82 for empty-vault runs (22 judges) and 3.13 for pretrained-vault runs (16 judges).
- Noise: the standard error of an 8-judge mean is 0.26 to 0.62. Run-to-run changes compared with a Welch t:
  - empty vault 4.25 to 3.88: −0.38 (t = −0.51, noise);
  - pretrained vault 2.38 to 3.88: +1.50 (t = 2.35);
  - empty vs pretrained at 40354cd: −1.88 (t = −2.79);
  - empty vs pretrained at e7f9705: 0.00.
  - The 3-judge runs (19:06, 19:36) are too small to compare (SE about 0.6 to 0.9).
- Per-topic means over the four 8-judge runs: "tools in a workshop" was worst at 2.50, and "your city, your work and your colleagues" was best at 4.50.

**What changed between runs** (git log, local times):

| Between | Commits | Change | Effect on mean score |
|---|---|---|---|
| 19:06 → 19:09 | a5568fd | teacher phase: no in-chat questions in scripted dialogues; "I don't know" treated as non-answer | teacher questions 0 → 1 |
| 19:09 → 19:36 | a71d546, 4c06c95, 2c9c3ca, a70311d, 75ac6fd | friendly chat replies, pretraining (teacher + reviewer), conversational phrasing, **Haiku LLM translator**, natural-conversation pretraining | 3.33 → 3.00 (3 judges each) |
| 19:36 → 19:42 | 4eb59b7, 40354cd | judge fixes (speaker's name, translator examples, no restating known facts); `--seed-vault`; 8 judge topics | 3.00 → 4.25 (empty); pretrained vault first measured at 2.38 |
| 19:42 → 19:51 | e7f9705 | on-topic curiosity in chat, honest "already knew", verbs not nouns | empty 4.25 → 3.88 (noise); pretrained 2.38 → 3.88 |
| 19:51 → 20:20 | 0ce89b2, 6d26abd | live monitor; chat-mode pretraining | no judge scores recorded |

### 2. Judge feedback analysis

All 38 judge reasons from the 6 judged runs were hand-tagged into recurring complaint types. A reason can carry several tags, so percentages are of 38 reasons. The split is 22 empty-vault judges and 16 pretrained-vault judges.

| Complaint type | Reasons | % of 38 | Empty vault (/22) | Pretrained (/16) | Example quotes |
|---|---|---|---|---|---|
| Ungrammatical / garbled reply wording | 21 | 55% | 15 | 6 | "Snow falls Winter", "Spring is an after of Winter"; "Cow is a say of moo" |
| Question not answered or answered wrongly | 18 | 47% | 8 | 10 | "fails the simple question 'Does Mom like apples?'"; "answered 'Ben is Anna's friend, right?' with the reversed 'Ben's friend is Anna'" |
| Facts dropped or meaning changed | 15 | 39% | 11 | 4 | "paraphrases 'lives in' as 'is in'"; "drops key facts like tennis, weekends, and Wednesdays" |
| Robotic, repetitive or mismatched stock phrases | 14 | 37% | 9 | 5 | "the repeated 'The rest I already knew' sounds robotic"; "'Glad I could help' answers a non-thanks" |
| Off-topic curiosity ("I noticed a pattern") questions | 11 | 29% | 0 | 11 | "fills most turns with off-topic 'I noticed a pattern' questions about clothes, plants, and vegetables" |
| False "already knew / you told me before" | 10 | 26% | 5 | 5 | "says 'I already knew that' to new facts"; "claims 'you told me before' for things never taught" |
| Plural, lemma or capitalization errors | 10 | 26% | 5 | 5 | "misspells 'wolf' as 'wolve' throughout"; "'Anna likes apple'", lowercase "lena", "berlin" |
| Clear input not understood ("say it more simply") | 7 | 18% | 5 | 2 | "answered two of the user's questions with 'I'm not sure what you mean' ... even though the question was clear" |
| States false or invented facts | 4 | 11% | 2 | 2 | "hallucinates 'eats rice and bread'"; "'Berlin used to be in germany'" |
| Speaker / identity confusion | 3 | 8% | 3 | 0 | "'Your name is you', 'You's name is you'" |
| Rule notation leaked / rule invented | 3 | 8% | 3 | 0 | "leaks rule notation ('if A is a wolve then A hunts in pack')" |

Supporting counts from the judge transcripts (228 agent replies):

- **Curiosity flood.** In the first pretrained-vault run (19:42, P1), **44 of 48** agent replies contained "I noticed a pattern. Is it true that ...". After commit e7f9705 (on-topic curiosity) this fell to 6 of 48. All 8 P1 judges named it. The empty-vault runs had 0 such replies, which accounts for that run's 2.38 score.
- **"Already knew" phrases** (already knew / knew that already / told me before), per run: 4, 4, 14, 2 (empty vault) and 22, 1 (pretrained). The e7f9705 "honest already knew" fix cut them from 14 → 2 and from 22 → 1.
- **Explicit not-understood replies** per run: 6, 0, 4, 3, 0, 2, so 15 of 228 (6.6%) overall. Few judge turns failed outright. Most judge complaints are about what the agent did with input it did understand.

### 3. Pretraining analysis per phase

Assumptions:
- The rejection rate is computed as rejected / (accepted facts + accepted rules + rejected).
- "Understood" is the agent's own count of turns in which at least one sentence was parsed.
- The parser used per phase is inferred from commit order. The LLM translator landed at a70311d (19:28 UTC), after the lesson run finished (report 19:28:04). The sentence-list and lesson phases therefore used the pattern parser, and the natural, chat and trainer phases used the Haiku translator.

| Phase (file) | Parser | Sessions | Turns / sentences | Understood | Facts + | Rules + | Rejected | Reject rate | Facts per turn | Teacher msg length |
|---|---|---|---|---|---|---|---|---|---|---|
| Sentence lists (19-18-06) | pattern | 10 topics | 279 sentences | 271 (97.1%) | 193 | 48 | 12 (9 facts, 3 rules) | 4.7% | 0.69 | single sentences |
| Structured lessons (19-28-04) | pattern | 40 | 560 (14 per session) | 483 (86.25%) | 370 | 112 | 57 | 10.6% | 0.66 | 7.5 words |
| Natural teacher (log, killed at 24/40) | LLM | 24 finished (+1 started) | 288 | 276 (95.8%) | 372 | 11 | 32 | 7.7% | 1.29 | not recorded |
| Friend-style chat (20-32-45) | LLM | 80 scheduled (40 topics × 2); **46 had 0 turns**, 2 had 1 turn, 34 had 10-14 | 417 | 339 (81.3%) | 577 | 33 | 49 | 7.4% | 1.38 | 45.1 words |
| Training app (trainer_snapshot) | LLM | 8 finished of 20 planned | 68 | 38 (55.9%) | 44 | 0 | 6 | 12.0% | 0.65 | 43.7 words |

Per-phase details:

- **Sentence lists.** All 8 not-understood sentences are subject-verb-object sentences with an unknown verb: "Hens lay eggs.", "Brooms sweep floors.", "Clocks show time.", "Nurses care for patients.", "Firefighters put out fires.", "Dentists check teeth.", "Bees pollinate flowers.", "Humans breathe air." Three rejections came from lemmatizing "leaves" to "leave" (2 facts and 1 rule).
- **Structured lessons.**
  - Per-session understood rate: median 93%, minimum 29% ("feelings in general": 4 of 14 turns understood).
  - 15 of 40 sessions added no rules.
  - Facts per session: mean 9.25, maximum 16.
- **Natural teacher.** 19 of 24 sessions understood 12 of 12 turns. The weakest were "pets and farm animals" (8 of 12) and "weather" (8 of 12). Rules nearly stopped: 11 in 24 sessions, against 112 in the 40 lessons.
- **Chat run.**
  - The 46 empty sessions have no recorded cause in the report.
  - Among the 34 non-empty sessions the median understood rate was 81%.
  - Facts per session averaged 7.21 over all 80 sessions (maximum 48), and 67 of 80 sessions added no rules.
  - Long chatty turns produced many facts per turn (1.38), but a large share were personal remarks that were later closed (see section 5).
- **Test runs on throwaway vault copies** (not part of the vault totals):
  - 19-14-21 (sentence list, 1 topic: 12/12 understood, +8 facts, +4 rules, 3 rules rejected)
  - 19-17-09 and 19-18-12 (lesson tests, 2 conversations each: 5/8 and 12/12 understood)
  - 19-32-57 and 19-35-53 (natural-style tests: 13/16 and 9/16 understood, 9 and 4 rejected)
  - 20-16-06 and 20-19-12 (chat-style tests: 4/12 and 12/14 understood)
- **Cost and throughput.**
  - None of the pretraining reports records cost or tokens, so cost cannot be computed for those phases. `vault/llm_cache.tsv` holds 937 non-empty translator cache entries.
  - Wall time from the audit log is approximate:
    - lesson run about 19:18:33 to 19:28:01 (about 9.5 min for 560 turns, roughly 59 turns/min, 6 in parallel, no LLM translation);
    - natural run about 19:37 to 20:19:48 (about 43 min for 288 turns, roughly 6.7 turns/min);
    - chat run 20:20:20 to 20:32:43 (about 12.4 min for 417 turns, roughly 34 turns/min).
  - Only the training app recorded usage (section 4).

**Reviewer rejections by category** (keyword-tagged and then checked by hand, lesson run 57 and chat run 49):

| Category | Lessons (57) | Chat (49) | Examples |
|---|---|---|---|
| Learner-induced rule from a spurious correlation | 22 (39%) | 0 | "if X is healthy then X grows in soil"; "if X is a fruit then X wants water" (7 fruit rules) |
| False or overgeneralized fact (situational state stored as a typical property) | 9 | 36 (73%) | "whale is fish"; "sea is flat", "basil is wilted", "ganymede is part of jupiter" |
| Negation / polarity error | 9 | 1 | "no bee makes honey", "no hat does not cover your feet", "muffin does not sweet" |
| Conversational or meta phrase stored as a fact | 7 | 1 | "let starts learning" (×2), "thanks that is clear", "weather's time is lately" |
| Garbled translation or lemma | 5 | 8 | "Africa is a both continent", "daughter is children"; "giraffe leaf eats leaf" (×4), "tonkotsu knows in fukuoka" |
| Stated rule overgeneralized | 4 | 2 | "if A is a patient then A goes to Hospital"; "if A is a person then A puts on hagelslag" |
| Reviewer false positive (the reviewer's own reason says the item is fine) | 1 | 1 | "people plays sport" ("the meaning ... is typical and true"); "sea is different every day" ("Correction: OK.") |

The vault log counts 117 facts closed by the reviewer and 34 rules rejected by the reviewer, 151 reviewer rejections in all. This is close to the 156 rejections reported by the non-test phase reports (12 + 57 + 32 + 49 + 6). The two sources were not reconciled item by item.

### 4. Training-app run (`eval/trainer_snapshot.json`)

Setup:
- Configuration: 1 teacher, 10 turns per conversation, 20 planned conversations, model haiku, review on.
- 8 conversations finished before the run was stopped: 7 conversations of 7-10 turns plus 1 conversation of 1 turn, for 68 turns in total.
- Wall time was 733.7 s, from 21:00:07 to 21:12:21 UTC.
- The scenarios are help requests: plan dinner, rain clothes, tea, picnic, balcony plants, flat tire, beach checklist, weather chat.
- 23 of 68 teacher messages say "A good answer would ...", which means the teacher demonstrated the expected answer.

**Classification of every agent reply** (rule-based, verified by hand on all 68 replies):

| Class | Replies | % |
|---|---|---|
| Not understood ("Sorry, I didn't understand that.", "I'm not sure what you mean.", "Could you say that more simply?") | 30 | 44.1% |
| Only acknowledged facts ("Noted. Rain causes wet.") | 14 | 20.6% |
| Understood but no answer ("I don't know what you wear yet.") | 12 | 17.6% |
| Small talk / social reply ("Glad I could help.", "Hello!", "What is wrong? I haven't said anything yet.") | 10 | 14.7% |
| Answer attempt (recites stored facts: "Here is what I know about chive: ...") | 2 | 2.9% |
| **Real answer to the request** (a recipe, list, steps or advice) | **0** | **0%** |

- The 38 replies that were not "not understood" match the snapshot's `understood = 38` exactly.
- Longest run of consecutive not-understood replies per conversation: 5, 4, 3, 5, 1, 2, 7, 0. In the beach-checklist conversation the last 7 replies in a row were not understood, while the teacher kept rewording the request.
- The 2 answer attempts did not answer what was asked: chive facts when asked for more plants, and snow facts when asked "will we get snow soon?".

**Timing** (68 agent replies, 68 teacher messages):

| Metric | Mean | Median | Min | Max | p90 | Sum |
|---|---|---|---|---|---|---|
| Agent answer time (ms) | 5,522 | 5,200 | 3,310 | 12,847 | 7,274 | 375,509 |
| of which LLM translation (ms) | 5,322 | 5,168 | 3,008 | 10,756 | 7,145 | 361,885 |
| of which agent "think" (ms) | 200 | 48 | 29 | 6,606 | 293 | 13,624 |
| Teacher message time (ms) | 4,936 | 4,824 | 2,766 | 7,446 | 6,370 | 335,618 |

- Translation is **96.4%** of the agent's answer time, and symbolic processing is 3.6%.
- Think time is above 200 ms in 17 replies and above 1 s in only one: 6,606 ms for "Noted. Beach checklist has sunscreen, water, hat, ...".
- Not-understood replies are the fastest (mean 4,437 ms, think 40 ms). Fact-storing replies are the slowest (mean 7,413 ms, think 721 ms).

**Token usage and cost per role** (as recorded in the snapshot):

| Role | Calls | Input tokens | Output tokens | Cache write | Cache read | Seconds | s per call | Cost (USD) |
|---|---|---|---|---|---|---|---|---|
| Translator | 69 | 138 | 43,956 | 327,778 | 4,802 | 368.2 | 5.34 | 0.0876 |
| Teacher | 68 | 136 | 23,241 | 86,425 | 7,714 | 335.7 | 4.94 | 0.0290 |
| Reviewer | 6 | 12 | 1,608 | 4,992 | 0 | 21.7 | 3.61 | 0.0018 |
| **Total** | 143 | 286 | 68,805 | 419,195 | 12,516 | 725.6 | | **0.1184** |

- The translator accounts for 64% of output tokens, 78% of cache-write tokens and 74% of cost. It averages 637 output tokens per call, against 342 for the teacher.
- 0 API errors.
- Plan usage at the end of the run: 24% of the session limit and 24% of the weekly all-model limit.

**Representative failing exchanges:**

1. Dinner (conversation 1, turn 1). The translator turned the question into a false fact about the user:
   - Teacher: "I've got pasta, garlic, olive oil, and cherry tomatoes, and only about 20 minutes. What can I make?"
   - Agent: "Okay, I'll remember that. You have pasta, garlic, olive oil and cherry tomato. You make tomato soup."
   - The next 5 replies were all not-understood, and the last was "Glad I could help."
2. Flat tire (conversation 6). The teacher supplied the procedure. The agent stored fragments of it and could not use them:
   - Agent: "Quick release lever holds wheel. Axle nut holds wheel. Wheel removes from frame." Later: "I don't know what you do yet." / "Nobody has told me what you do, so I don't know."
3. Tea (conversation 3). A correction loop produced a self-contradictory acknowledgment:
   - "Okay, I'll remember that. Water pours black tea bag." Then: "Thanks for the correction. I no longer believe that water pours black tea bag and water pours black tea bag."
   - The reviewer also rejected the stored negation "water does not pour black tea bag".

### 5. Vault and knowledge quality

**Composition** (current `vault/`):

- 678 entity files and 2,171 fact lines, of which 1,797 are open and 374 are closed (`@until`). This matches the snapshot's vault counters.
- All facts have `@conf 1`. 74 facts are strong negations.

Facts by source:

| Source | Lines | Open | Closed | Closed % |
|---|---|---|---|---|
| teacher:pretrain | 1,050 | 911 | 139 | 13.2% |
| teacher:chat | 751 | 568 | 183 | 24.4% |
| parser (`proper_name`) | 253 | 253 | 0 | 0% |
| teacher:train | 115 | 63 | 52 | 45.2% |
| user | 2 | 2 | 0 | 0% |

- Top predicates: `property` 450, `is_a` 389, `proper_name` 253, `has` 219, `located_in` 127, `part_of` 60, `likes` 52, `wants` 31.
- Personal facts: 202 lines have a personal subject (`friend` 172, `maksim` 19, `teacher` 11). 158 of them are closed, but **44 remain open**, for example `wants(friend, tea)`.
- Close reasons in the audit log (361 close events):

| Close reason | Count |
|---|---|
| "personal remark in a (training) chat" | 136 |
| reviewer rejection | 117 |
| contradiction (functional key or negation) | 99 |
| user said it is wrong | 9 |

- **`proper_name` facts are mostly noise.** In a random sample of 40, 24 are ordinary words: `tire`, `winter`, `sun`, `ships`, `apples`, `angry`, `here`, `now`, `seven`, `snacks_could`, and others. 16 are real names or capitalized adjectives (`london`, `nairobi`, `december`, `dutch`). The parser stores any capitalized word as a name, and teachers capitalize the first word of each sentence.

**Random sample of 40 non-parser fact lines** (seed 42), graded by hand:

| Grade | Count | Examples |
|---|---|---|
| Correct general knowledge | 27 (67.5%) | `has(fish, fin)`, `is_a(saturn, planet)`, `grow_on(cucumber, vine)`, `part_of(pilot, flight_crew)` |
| Trivial / context-only | 6 (15%) | `play(dog)`, `is_a(shady_corner, spot)`, `property(roasting, quality, easy_to_mess_up)` |
| Garbled or wrong | 5 (12.5%) | `is_a(americas, both_continent)`, `has(apple, branch)`, `causes(too_much_water, mushy)`, `-is_a(mother, child)`, `property(brazil, size, biggest)` |
| Personal | 2 (5%) | `wants(friend, tea)` (open), `eat(friend, pancake)` (closed) |

So about two thirds of the sample is usable world knowledge.

**Rules** (210 files):

| Status | teacher:pretrain | teacher:chat | teacher:train | learner | Total | Mean evidence | Mean conf |
|---|---|---|---|---|---|---|---|
| confirmed | 96 | 30 | 1 | 1 | 128 | 0.1 | 0.899 |
| hypothesis | 0 | 0 | 0 | 42 | 42 | 4.88 | 0.432 |
| rejected | 7 | 3 | 0 | 30 | 40 | 2.45 | 0.479 |

- **Learner.** It proposed 73 rules. 30 were rejected: 34 reviewer rejection events in the log, plus 7 rejections for counterexamples outweighing support. 42 are still hypotheses, and only 1 was ever confirmed, `live(X, thing) :- is_a(X, animal)` (evidence 13, confirmed when "user answered my question"). That rule's head is a garbled form of "is a living thing".
- **Hypothesis sample** (15 of 42, graded):
  - 9 plausible, e.g. `has(X, hair) :- is_a(X, mammal)`, `grow_in(X, soil) :- grow_from(X, seed)`;
  - 3 weak, e.g. `grow_in(X, garden) :- is_a(X, vegetable)`;
  - 3 wrong or garbled, e.g. `make(X, oxygen) :- is_a(X, foods_from_plant)`, `wants(X, water) :- property(X, quality, healthy)`, `live(X, thing) :- has(X, six_leg)`.
  - 28 of the 42 hypotheses use an `is_a` body, and most cluster on one plant/seed lesson (grow_in, grow_from, wants water/sunlight).
- **Confirmed (stated) rule sample** (15, graded): 10 correct (`give(A, milk) :- is_a(A, cow)`, `has(A, blade) :- is_a(A, knife)`) and 5 with garbled constants whose meaning can still be recovered (`make(A, food_hot) :- is_a(A, oven)`, `wants(A, air_to_live) :- is_a(A, whale)`, `property(A, quality, cold) :- property(A, quality, ice)`).
- **Evidence.** Mean evidence on confirmed rules is 0.1. Stated rules are confirmed at conf 0.9 on the teacher's word alone.

**Duplicates, synonyms and lemma errors:**

- **Plural duplicates.** 13 entity-file pairs exist in both singular and plural: apple/apples, chive/chives, lake/lakes, lion/lions, pear/pears, ship/ships, shirt/shirts, tree/trees, wave/waves, firefighter/firefighters, scissor/scissors, earth/earths, do/does. Across all fact arguments there are 21 such pairs, adding clothe/clothes, cracker/crackers, knive/knives, room/rooms, sandwich/sandwiches, snack/snacks, sunglass/sunglasses, math/maths.
- **Bad lemmas.**
  - `clothe` (6 uses, including the category in `is_a(hat, clothe)`), `knive` (2), `leave` for leaf (2), `buse` (1), `sunglass` (1).
  - Meta facts such as `property(knife, plural, knives)` and `property(plural_of_knife, quality, knives)`.
- **Compound junk constants.** `foods_from_plant` (9 occurrences), `both_continent`, `sunlight_to_grow`, `water_to_grow`, `my_bike` next to `bike`, `flat_tire` next to `tire`.
- **tire/tyre.** No `tyre` appears anywhere in the vault, so no tire/tyre split exists yet. `tire` itself carries a spurious `proper_name(tire)`.
- **No canonicalization.** No `same(X, Y)` synonym links were found. Nothing merges variants.

**Growth over time** (audit log `vault/log/2026-11-16.md`, 6,074 lines):

- Totals: 2,744 turn entries, 1,918 fact adds, 639 confirms, 361 closes, 137 stated rules, 73 proposed rules, 42 rule rejections, 144 rule-confidence updates, and 8 curiosity questions.
- 1 corrupted line (two writes interleaved, line 934) shows that concurrent writers can garble the log.

| 10-min bucket (UTC) | Fact adds | Closes | Rules added (stated + proposed) | Cumulative adds | Cumulative open (adds − closes) | Phase |
|---|---|---|---|---|---|---|
| 19:10 | 258 | 24 | 76 | 258 | 234 | sentence lists, lessons start |
| 19:20 | 312 | 34 | 84 | 570 | 512 | structured lessons |
| 19:30 | 38 | 12 | 2 | 608 | 538 | natural run start |
| 19:40 | 109 | 20 | 4 | 717 | 627 | natural |
| 19:50 | 117 | 24 | 1 | 834 | 720 | natural |
| 20:00 | 100 | 12 | 2 | 934 | 808 | natural |
| 20:10 | 118 | 9 | 5 | 1,052 | 917 | natural (killed 20:19) |
| 20:20 | 641 | 123 | 27 | 1,693 | 1,435 | chat run |
| 20:30 | 110 | 51 | 6 | 1,803 | 1,494 | chat run end |
| 20:50 | 71 | 32 | 3 | 1,874 | 1,533 | earlier training-app run(s) |
| 21:00 | 35 | 16 | 0 | 1,909 | 1,552 | snapshot training run |
| 21:10 | 9 | 4 | 0 | 1,918 | 1,557 | snapshot training run |

- Rule growth was front-loaded: 160 of 210 rule additions came in the first 20 minutes, during the pattern-parser phases.
- Fact growth peaked in the chat run, with 641 adds in 10 minutes, but the same interval also logged 123 closes (19% of its adds).
- The training-app runs added 115 facts, and 45% of them were already closed.

### 6. Key quantitative findings

- **0 of 68** replies in the help-style training run answered the request. 44.1% (30) were not understood, 20.6% only acknowledged facts, 17.6% said "I don't know", 14.7% were small talk, and 2.9% recited unrelated stored facts.
- **96.4%** of the agent's answer time (361.9 of 375.5 s) is LLM translation. Symbolic reasoning takes a median of 48 ms per reply against 5.2 s for translation.
- The translator is **74% of LLM cost** ($0.0876 of $0.1184) and 64% of output tokens in the training run.
- Conversation quality has not improved measurably. The 8-judge means are 4.25 and 3.88 (empty vault) and 2.38 and 3.88 (pretrained vault). All 38 scores lie between 2 and 6, and the empty-vault change of −0.38 is within noise (t = −0.51).
- The fixed knowledge tests are **saturated and flat** in all 10 runs: parser 100% / 98%, dialogue checks 96/96 and 45/47. They test 4-word sentences, while judges write 10.8-word messages and trainer teachers write 43.7-word messages.
- Starting from the pretrained vault **lowered** the score from 4.25 to 2.38 at the same commit (t = −2.79). The cause was a curiosity flood: 44 of 48 replies asked "I noticed a pattern" questions. The e7f9705 fix brought this down to 6 of 48 and the score back to 3.88.
- The most frequent judge complaints: garbled wording in **55%** of 38 reasons, unanswered or wrong answers in 47%, dropped or changed facts in 39%, and stock phrases in 37%.
- Understanding fell as teacher input grew longer: 97.1% for single sentences, 86.25% for lessons (7.5-word turns), 81.3% for friend-style chat (45-word turns), and 55.9% for help requests.
- Reviewer rejection rates were 4.7% (sentence lists), 10.6% (lessons), 7.7% (natural), 7.4% (chat) and 12.0% (training). In the lessons, **39%** of rejections were learner-induced spurious rules (22 of 57). In chat, 73% were situational or overgeneralized facts (36 of 49).
- The vault holds 2,171 fact lines (1,797 open), but **374 (17%)** are closed. About two thirds of sampled facts are correct (27 of 40). 44 personal facts about "friend", "maksim" and "teacher" remain open, and 24 of 40 sampled `proper_name` facts are ordinary words.
- The pattern learner is nearly unproductive: 73 rules proposed, **1 confirmed** (with a garbled head), 42 still hypotheses, 30 rejected. Stated rules (137) are confirmed with mean evidence 0.1.
- In the chat pretraining run, **46 of 80** scheduled conversations produced 0 turns, and the report records no cause. Lemma and plural problems leave at least 21 singular/plural constant pairs and bad lemmas (`clothe`, `knive`, `leave`, `buse`) in the vault.

---

## 6. Conversation failure analysis and recommendations

### 0. Evidence base and method

All numbers below come from the repo's own run artefacts (2026-11-16). Nothing was re-run.

| Sample | What it is | Size | How it was coded |
|---|---|---|---|
| **S1** `eval/trainer_snapshot.json` | Newest training run: Haiku teachers playing a user in help scenarios (dinner, flat tire, balcony plants, tea, rain, picnic, beach, weather) | 8 conversations, 68 agent turns | Every turn coded by hand, plus regex labels |
| **S2** `eval/pretrain/2026-11-16T20-32-45_conversations.md` | Friend-chat run (small talk) | 80 conversations, 417 turns | Regex labels on the agent reply (several labels per turn allowed) |
| **S3** `eval/pretrain/2026-11-16T19-28-04_conversations.md` | Structured lessons (short facts) | 40 conversations, 560 turns | Regex labels |
| **S4** `vault/log/2026-11-16.md` `turn` lines | Every message and reply of the day | 2,744 pairs (2,637 unique) | Regex labels |
| **S5** `eval/reports*/` | Haiku judges chatting and scoring | 27 judged conversations in 5 runs | Judges' scores and reasons read in full |
| **S6** `vault/llm_cache.tsv`, `vault/facts/` | Translator cache and the stored knowledge | 938 cache lines, 1,797 open facts | Counted with scripts |

Regex labels match the fixed NLG templates (for example `NotUnderstood` = "Sorry, I didn't understand that." / "I'm not sure what you mean." / "Could you say that more simply?"), so they are exact for template families. They are multi-label, so percentages in a sample can add up to more than 100%.

**Headline facts**

- **Help scenarios (S1): 0 of 8 conversations reached the user's goal.** Only **1 of 68 turns (1.5%)** gave the user any task content ("Here is what I know about chive: Chive is a herb, is easy, handles light and wants water."). 57 of 68 teacher messages contained a request ("can you / could you / what should I / how do I ..."); 4 got a reply that was not a failure template.
- The trainer's own **"understood" metric said 38/68 = 56%**. It counts any parse that is not `unknown:`, so it hides that almost nothing helpful came back.
- **Learning in S1:** +44 facts, **0 rules**, 6 rejected by the reviewer. The **dinner conversation stored 0 facts** although the teacher gave the full recipe 4 times.
- **Latency (S1):** 375.5 s total answer time, of which the **translator took 361.9 s (96%)**, about 5.3 s per turn. The agent's own reasoning took 13.6 s in total (about 0.2 s per turn).
- **Translator cost profile (S1):** 69 calls, 43,956 output tokens (**about 637 per call**, where the canonical lines need about 30–80), 327,778 cache-*write* tokens and only 4,802 cache-*read* tokens. The roughly 4.7k-token spec prompt is written to the cache on every call and almost never read back.
- **Translator result cache (S6):** 938 entries cover 937 distinct texts, so in practice natural messages **never hit the cache**. The key also includes `last_entity`, which makes hits even rarer.
- **Judges (S5):** 3.33 → 3.00 → 4.25 → 3.88 (vault without pretraining), and 2.38 → 3.88 (pretrained vault). Reasons mention garbled grammar or meaning in 7 of the 8 reasons of the last run.

---

### 1. Failure taxonomy

The counts come from S1 (hand-coded, 68 turns) unless another sample is named. S2/S3/S4 give the share of turns carrying the regex label.

| # | Failure mode | Frequency | Layer causing it |
|---|---|---|---|
| F1 | **Request dropped**: help or advice requests have no representation, so the reply covers only the greeting, the thanks or an echo | S1: 10/68 turns reply only "Hello!", "Glad I could help.", "Any time!", "You're welcome!" or "That's okay." to a message that contained a request. Translator: 6 of the 8 S1 opening messages map to `greet:` + `unknown:` or `greet:` only | Missing representation (no `request` intent), translator, dialogue management |
| F2 | **No procedural knowledge**: "how do I / what next / what should I wear" becomes a fact query `do(friend, W)` with nothing to find | S1: 11/68 "I don't know ..." turns, all for steps, advice or options. S4: 14% of turns | Missing representation (steps, needs, durations, recommendations) |
| F3 | **Not understood**: long, multi-sentence, meta or teaching messages fail translation | S1: **30/68 (44%)**. S3: 24%. S4: 9%. S1 teacher messages average **43.7 words**; 23/68 have more than 3 sentences | Translator, teacher behaviour |
| F4 | **Echo acknowledgement**: the user's situation is stored as permanent facts and read back to them | S1: 16/68 (24%). S2: **256/417 (61%)**. S3: 54%. S4: 46% | Dialogue management (no session state), NLG act choice |
| F5 | **Misplaced social reply**: "Thanks! ... could you ..." gets only "Glad I could help." | S1: 8/68 thanks-type replies, all to messages that also held a request or teaching | Dialogue management (the reply filter keeps `social: thanks` and drops `unknown:`), translator |
| F6 | **Self-deflection loop**: `about_you` fires too often and gets one stock template | S2: **240/417 (58%)**. S4: 10% | Translator (over-uses `about_you`), NLG (one template), no self-model |
| F7 | **Feedback read as a fact correction**: "Hmm, not quite" becomes `correct: none` | S1: 3/68. S3: 37/560 (7%). S4: 64 | Translator, dialogue management (feedback on the reply ≠ correction of a fact) |
| F8 | **Garbled semantics and roles**: predicate soup, swapped agent/patient, junk entities | S1: 6/44 new facts rejected by the reviewer (14%), plus at least 4 visibly garbled turns. Vault: **216 predicates, 132 used once (61%)**; 99 entities of 3+ words; 19 plural/singular duplicate pairs; 200 `property(_, quality, _)` catch-alls | Translator, knowledge quality |
| F9 | **Context errors**: wrong referent, facts from another conversation's persona | S1: 2 clear cases (stale `make(friend, tomato_soup)`, "long steeping's procedure"). Judges: 3/8 reasons in the last run | Dialogue state tracking |
| F10 | **Stiff or wrong surface form**: bare singulars, lowercased names, mangled plurals, "I already knew that" | S4: "already knew" 14%. Judges cite grammar in 7/8 reasons (last run) | NLG (no surface lexicon, no lists, no discourse planning) |
| F11 | **Unproductive teaching**: repeats, meta requests, role drift, apologies | S1: 17/68 teacher messages ask for formatting or "say it back"; 23/68 contain "a good answer would ..."; 33/68 open with "Sorry", "No worries", "Hmm" or "Okay, let ..."; one message repeated in substance 7 times | Teacher prompt and protocol |
| F12 | **Off-topic or garbled curiosity questions in chat** | Judges: 2/8 conversations in the last run were derailed by "I noticed a pattern. Is it true that if X is a foods from plant then X is healthy?" | Dialogue management (curiosity gating), NLG |

**Verbatim examples (two per mode)**

- **F1 Request dropped**
  - T: "Hey, I'm heading out in a bit and it looks like it's going to rain. What should I wear?" → A: "Hello!" (S1, rain)
  - T: "Thanks, that list is perfect! It's about four people, all of us, so could you tell me roughly how much food to get for that?" → A: "Any time!" (S1, picnic)
- **F2 No procedural knowledge**
  - T: "...Pump the tube up a little, then listen for a hiss or feel for air on your cheek. Once you find the hole, mark it with a pen. What do I do after that?" → A: "I don't know what you do." (S1, bike)
  - T: "So what exactly should I do, step by step? I'd like to know how long to steep it and how hot the water should be." → A: "I don't know what long steeping's procedure is. I don't know what long steeping's steep time is yet. Nobody has told me what long steeping's water temperature is, so I don't know." (S1, tea)
- **F3 Not understood**
  - T: "Sure. Boil the pasta for about 10 minutes. While it cooks, heat some olive oil and garlic in a pan. Add the tomatoes and wait until they get soft. Then mix everything with the drained pasta." → A: "Sorry, I didn't understand that." (S1, dinner)
  - T: "Okay, I'll try once more. Here are the five steps in order: 1. Boil the kettle. 2. Wait about a minute ... Could you say those five steps back to me?" → A: "Could you say that more simply?" (S1, tea)
- **F4 Echo acknowledgement**
  - T: "Hi! My bike has a flat tire and I have no idea what to do. Can you help me fix it?" → A: "Okay, I'll remember that. Your bike has flat tire. That's okay." (S1)
  - T: "Hi there! Could you help me make a good cup of tea? I've got a box of black tea bags..." → A: "Okay, I'll remember that. You have black tea bag." (S1)
- **F5 Misplaced social reply**
  - T: "Thanks! I usually just leave the bag in the mug until it's dark, and that's why mine always turns out bitter. What's the right way to do it?" → A: "Glad I could help." (S1, tea)
  - T: "Thanks, chives and parsley sounds good. Could you name two or three more that are easy?..." → A: "Glad I could help. Here is what I know about chive: ..." (S1, balcony)
- **F6 Self-deflection loop**
  - A: "I don't have likes of my own, but here is what I know about sea: Sea is a body of water, moves and is big." (S2; given 3 times in one conversation)
  - A: "I'm doing well, thanks. I've learned 865 facts so far, and I'm always happy to learn more." (given 3 times in 6 turns to "Do you like soup...?", pretrain 20-16-06)
- **F7 Feedback read as a correction**
  - T: "Hmm, that's not much help yet. A good answer would name a few easy plants..." → A: "What is wrong? I haven't said anything yet." (S1, balcony)
  - T: "Not quite, that came out jumbled. The order is: boil the kettle, ..." → A: "Thanks for the correction. I no longer believe that water pours black tea bag and water pours black tea bag." (S1, tea)
- **F8 Garbled semantics**
  - A: "I don't know what you a picnic bring of are. Got it. You plan picnic, want easy to set up item and do not need fridge. Picnic is a location of park and times weekend." (S1)
  - A: "...here is what I know about snow: Snow is a frozen water and melts faster glove cold and glove warm." (S1)
- **F9 Context errors**
  - T: "...I've got pasta, garlic, olive oil, and cherry tomatoes, and only about 20 minutes. What can I make?" → A: "...You make tomato soup." The translation was `ask: make(friend, W)`, and it matched `make(friend, tomato_soup)`, stored from an *earlier* friend-chat persona at 20:21:49.
  - A: "You have lunch together your colleague. Your colleague is your colleague." (S5, judge run 19-51-19)
- **F10 Stiff surface form**
  - A: "Noted. Beach checklist has sunscreen, water, hat, sunglass, towel and snack." (S1)
  - A: "Yes, apple used to be a fruit, but not anymore." (S5; the judge scored the conversation 2)
- **F11 Unproductive teaching**
  - T (7 times in a row in one conversation): "...A packing list is just one thing per line, each line starting with a dash. ... Can you write sunscreen, water, a hat, sunglasses, a towel, and snacks that way?" (S1, beach)
  - Role drift: T: "Here's the list, plainly: - A blanket to sit on ... Could you tell me how many people you're feeding so I know how much to get?" (S1, picnic: the teacher speaks as the assistant)
- **F12 Curiosity derailment**
  - A: "Got it. You have lunch together your colleague. ... I noticed a pattern. Is it true that if X is a foods from plant then X is healthy?" (S5)
  - A: "Okay, I'll remember that. Bird is an animal. I noticed a pattern. Is it true that if X is an animal then X lives thing?" (S3)

**Reading across the taxonomy.** F1–F2 are the root cause. The formal language (`docs/parser_spec.md`) has no way to say "the user wants help doing X", and the knowledge base has no way to say "doing X takes these steps". Everything else is the system degrading around that hole. The translator falls back to `unknown:`, or to `tell:` about the user's situation (F3, F4). Social fragments become the whole reply (F5). Teachers react to the failure by sending longer and more meta messages, which raises F3 further (F11). F8–F10 are quality problems that would remain even after the hole is filled.

---

### 2. Gap analysis against the dinner example, turn by turn

#### Turn 1. User: "Hi! Can you help me plan dinner tonight?" → Assistant: "Sure! What ingredients do you have, and how much time do you want to spend cooking?"

| Needed | Symbolic form (proposal) | Today |
|---|---|---|
| Translation | `greet:` + `request: plan(meal, dinner) @when tonight` | `greet:` + `unknown:` (compare cache line 785: "Can you help me figure out what to bring?" → `greet:unknown:`) |
| Task schema in the KB | `task(plan_meal)`, `slot(plan_meal, ingredients, required)`, `slot(plan_meal, time_budget, required)`, `slot(plan_meal, diet, optional)`, `slot_question(ingredients, "what ingredients do you have")`, `slot_question(time_budget, "how much time do you want to spend cooking")` | Nothing. No task, slot or request predicates exist |
| Dialogue state | Frame `{task: plan_meal, meal: dinner, when: tonight, filled: {}, missing: [ingredients, time_budget], status: collecting}` | Only `last_entity`, `pending` (one fact or one wh question) and `last_answer` |
| Policy | If the request has a known task and required slots are missing, acknowledge and ask for all of them in one turn | The curiosity drive can ask *one* wh question, and only about KB gaps |
| NLG | `AcceptTask` + `AskSlots([..])` → "Sure! {q1}, and {q2}?" | `AskWh` makes one question from an atom pattern, with no conjunction of questions and no "Sure!" lead |
| Result today | | "Hello!" |

#### Turn 2. User: "I have pasta, garlic, olive oil, and some cherry tomatoes. Maybe 20 minutes." → Assistant: the garlic tomato pasta recipe

| Needed | Symbolic form | Today |
|---|---|---|
| Translation as slot filling | `inform: available(pasta); available(garlic); available(olive_oil); available(cherry_tomato); time_budget(20, minute) @hedge` | `tell: has(friend, pasta); ...` stored **permanently** in the KB about the speaker, then echoed back |
| Session-scoped facts | Facts tagged `@session <id>` in the frame and not written to `vault/facts/`. Only facts the user marks as lasting get promoted | Everything goes to the vault (the trainer later closes facts with `friend`/`my_` arguments, which is a patch) |
| Recipe knowledge | `dish(garlic_tomato_pasta)`, `solves(garlic_tomato_pasta, plan_meal)`, `needs(garlic_tomato_pasta, pasta)` ... `needs_optional(_, salt)`, `staple(salt)`, `staple(pepper)`, `duration(garlic_tomato_pasta, 15, 20, minute)` | Fragments only: `cook_in(pasta, pot)`, and no dish entities linked to ingredients |
| Procedure knowledge | `step(gtp, 1, boil(pasta))`, `step(gtp, 2, saute(garlic, olive_oil))` with `step_duration(gtp, 2, 1, 1, minute)`, `step_prep(gtp, 2, sliced)`, `step_overlaps(gtp, 2, 1)` ("while it cooks"), `step(gtp, 3, add(cherry_tomato))` with `step_until(gtp, 3, soften(cherry_tomato))`, `step_duration(gtp, 3, 5, 7, minute)`, `step(gtp, 4, toss(pasta, splash(pasta_water)))`, `step(gtp, 5, season(salt, pepper))` | None. Durations and quantities can't be expressed (no numeric ranges or units in the spec) |
| Reasoning (option choice) | Candidates = dishes that solve the task. Feasible = every `needs` is `available` or a `staple`; `duration` max ≤ budget. Rank by fraction of available ingredients used, then by shorter time. Keep a proof trace ("because you have pasta, garlic, oil and tomatoes and it takes 15–20 minutes") | The reasoner has comparisons (`Cmp`) and stratified negation, which is enough for the feasibility test. It has no aggregates (count/max) for ranking, and no "choose one" step |
| NLG | `Recommend(dish)` + `Procedure(steps)` + `Duration(total)`: an imperative realizer ("Boil the pasta."), connectives from `step_overlaps` → "While it cooks, ...", from sequence → "then", `until` clauses, ranges "5–7 minutes", surface forms "cherry tomatoes" (plural kept), "a splash of pasta water" | The realizer only builds declarative clauses ("X verbs Y"). No imperative mood, no connectives except "and", no numbers, and plurals are lost at parse time |
| Faithfulness check | Each step clause is a `claim` that parses back to its `step(...)` atom | The existing claims-parse-back test would have to cover the new acts |
| Result today | | "Okay, I'll remember that. You have pasta, garlic, olive oil and cherry tomato. You make tomato soup." |

#### Turn 3. User: "Sounds great, thanks!" → Assistant: "You're welcome! Enjoy your dinner."

| Needed | Symbolic form | Today |
|---|---|---|
| Translation | `accept:` + `social: thanks` | `social: thanks` only (`accept` does not exist) |
| Dialogue state | Frame status `proposed` → `accepted` → `closed`. Log the outcome `task_outcome(session, plan_meal, accepted)` (a training signal: this recipe worked) | No task, so nothing to close |
| NLG | `Thanks` + `ClosingWish(task)` with `closing_wish(plan_meal, "Enjoy your {meal}")` → "Enjoy your dinner." | "You're welcome!" / "Glad I could help." / "Any time!" rotate with no context. This turn is the only one the system already gets half right |

**Summary of the gap.** Three representations are missing: request and task frames, procedural and recipe knowledge with numbers, and a session state. Two capabilities are missing: choosing among options, and imperative or discourse NLG. Translation, the Datalog engine with proof traces, the template realizer with claims checking, the learner and the reviewer all exist and can be reused.

---

### 3. Recommendations (prioritized)

Effort: S = days, M = 1–2 weeks, L = several weeks for one developer with Claude Code.

#### R1. Request, inform, accept and feedback dialogue acts plus task frames (P0, M)

- **Build:** Extend `parser_spec.md` and `from_canonical` with `request: <task>(<args>)`, `inform: <slot atoms>`, `accept:`, `reject:`, `feedback: unhelpful|wrong_format|good`, `meta: <what>`, `ask_how: <task>` and `ask_next:`. Add KB predicates `task/1`, `slot/3`, `slot_question/2`, `solves/2`. In the agent, add a `TaskFrame` that a `request` opens. Policy: if required slots are missing, ask for all of them; if the task is complete, recommend or proceed; if the task is unknown, say so honestly and ask to be taught ("I don't know how to plan dinner yet. What would a good plan include?").
- **Why:** F1 and F5 (10/68 dropped turns, 57/68 messages contain requests, 0/8 goals reached). F7: "Hmm, not quite" needs a target other than `correct: none`.
- **Metric:** *request handled rate* (requests answered with a task-appropriate act: slot question, recommendation, step, or honest "teach me"), from about 7% (4/57) to >80% even before any task knowledge. *Correction misfire count* to 0.
- **Risks:** Task inventory sprawl (keep a small set of generic schemas, see R2). The translator must choose the task name consistently: give it the registry of known task names in the prompt.

#### R2. Procedural and recommendation knowledge: needs, steps, durations, options (P0, L)

- **Build:** A small set of *generic* schemas rather than per-domain code:
  - **Procedure:** `step(T, N, Action)`, `step_duration(T, N, Min, Max, Unit)`, `step_until(T, N, Cond)`, `step_overlaps(T, N, M)`, `needs(T, Thing)`, `tool(T, Tool)`, `duration(T, Min, Max, Unit)`.
  - **Recommendation:** `recommended_for(Item, Situation)` with `because(...)`, for rain clothing or balcony plants.
  - **Checklist:** `pack_for(Trip, Item)`.
  - **Quantity:** `portion(Item, Amount, Unit, per_person)`.

  Add numbers and ranges as values (`Value::Num` if missing) and a unit lexicon. Steps are first-class atoms, so the reasoner can answer `ask_next` from `step(T, N+1, _)` given the frame's current step. Allow multi-line or multi-atom rule statements if needed.
- **Why:** F2 (11/68 "I don't know" turns, all procedural or advice). The dinner conversation stored 0 facts from 4 recipe tellings. The bike conversation stored "Wheel removes from frame." with no step order.
- **Metric:** *task completeness* (share of known tasks with needs + ordered steps + duration). Goal completion on second encounter (R8).
- **Risks:** Encoding vague conditions ("until they soften") as opaque atoms is fine for replay but not for reasoning. Accept that. Step granularity differs between teachers, so normalize through the R7 protocol.

#### R3. Dialogue state tracking with session-scoped facts (P0, M)

- **Build:** A per-conversation `DialogueState`: active task frame(s), filled and missing slots, current step index, the user's situation facts (stored with `@session`, never in `vault/facts/` unless promoted), last agent act, open question, and referents (a salience list rather than one `last_entity`). Reset per conversation. Pass a compact rendering of it to the translator (R6). Replace the trainer's after-the-fact closing of `friend`/`my_` facts with this.
- **Why:** F4 (echo in 24–61% of turns), F9 (the "You make tomato soup" leak across personas, "long steeping's procedure"). The translator gets only `last_entity` as context, so "What should I do next?" can't be resolved.
- **Metric:** *echo rate* (acknowledgements that repeat user situation facts), from 24% (S1) to <5%. *Cross-session leakage* (answers proved from another session's facts) to 0, checked from proof traces.
- **Risks:** Deciding what is lasting ("I'm vegetarian") versus situational ("I have pasta"). Default to session and promote on explicit or repeated statement. Memory of user preferences is a later feature.

#### R4. Option selection and ranking in the reasoner (P1, M)

- **Build:** A `choose` step: query candidates `solves(D, task)`, filter feasibility with Datalog (`missing(D, I) :- needs(D, I), not available(I), not staple(I).`, `feasible(D) :- solves(D, T), not has_missing(D), duration_max(D, M), M <= budget.`), then rank in Rust (count of used available ingredients, time, confidence, past acceptance from `task_outcome`). Return the best one with a proof, plus the runner-up for "anything else?". When nothing is feasible, return the closest option with its missing items ("You'd need eggs for that").
- **Why:** Turn 2 of the dinner example needs it. The balcony, rain, picnic and fridge-ingredient scenarios all are "choose among options under constraints".
- **Metric:** *recommendation acceptance* (user `accept:` after a proposal) and *feasibility errors* (recommendations needing an item the user lacks), judged per conversation.
- **Risks:** Aggregates break plain Datalog termination guarantees if added inside the engine, so keep ranking outside it. Ranking weights are hand-set.

#### R5. NLG for helpful replies (P1, M)

- **Build:**
  1. A **reply planner** that picks at most one main act per turn by priority: answer or recommend > ask a slot > honest "teach me" > acknowledge > social. A social fragment never replaces content.
  2. **Imperative and procedure realization** with connectives (then / while it cooks / until / after that), numbered lists or one item per line when there are more than 3 items, number ranges ("5–7 minutes"), "about".
  3. A **surface lexicon** stored with each entity (`surface(cherry_tomato, "cherry tomatoes", plural)`, `mass(olive_oil)`, `proper_name` capitalisation), so "sunglass", "potato", "berlin" stop.
  4. **Acknowledge only what matters:** never "I already knew that" in chat (S4: 14% of turns); acknowledge user situation as "Got it, 20 minutes and pasta, garlic, oil and tomatoes."
  5. **Contextual social replies:** `closing_wish(task)`, and "thanks" plus a request answers the request.
  6. Stable wording for the same failure (PROGRESS lists rotation as a judge complaint).
- **Why:** F5, F10, F12; judges cite surface errors in 7/8 reasons; "already knew" 14%.
- **Metric:** judge grammar flags per conversation; *repeated-reply rate* (identical agent texts within one conversation; S1 beach repeated 3 NotUnderstood templates 7 times); judge score.
- **Risks:** Every new template must keep the faithfulness check (claims parse back). Extend the checker to the new acts instead of bypassing it.

#### R6. Translator: context, few-shot from cache, validation and retry, caching, effort (P0, M)

- **Build:**
  - (a) Send the dialogue state (R3): last agent reply, active task and missing slots, and the registry of known predicates and tasks. "What do I do next?" then becomes `ask_next:`, and "20 minutes" becomes `inform: time_budget(20, minute)`.
  - (b) **Few-shot retrieval:** choose 5–10 reviewed `(message → canonical)` pairs from the cache by word overlap with the new message. This turns the cache, which never hits today (937 distinct texts in 938 lines), into a learning asset.
  - (c) **Validation with retry:** if output lines fail `from_canonical`, use unknown predicates, add a 3+-word entity, or produce `unknown:` for a message with more than 5 content words, call again once with the error list. Add a **grounding check**: every constant in a `tell`/`inform` must come from the message or the state, so the LLM cannot inject knowledge or answers.
  - (d) **Prompt caching:** move the spec into a stable system prompt (or use the API with `cache_control`). S1 shows 327,778 cache-write vs 4,802 cache-read tokens.
  - (e) **Low effort and no extended thinking** for translation, with output capped at about 200 tokens. Today it is about 637 output tokens per call.
  - (f) **Split teacher messages** into a "user part" and a "teaching part" (see R7) so each is translated alone.
  - (g) Key the result cache on text plus a state hash, not `last_entity`.
- **Why:** F3 (44% not understood in S1), F7, F8. Translation is 96% of latency.
- **Metric:** not-understood rate on S1-style scenarios 44% → <10%; translate time per turn 5.3 s → <2 s p50; translator cost per turn; validation-failure and retry rates; garbled-fact rejection rate (reviewer) 14% → <5%.
- **Risks:** Few-shot examples copy old errors, so use only reviewer-approved pairs. More context means longer prompts, which caching offsets. The grounding check may reject legitimate paraphrases (lemmatise before comparing).

#### R7. Teaching protocol for teachers (P0, S)

- **Build:** Rewrite `teacher_prompt` and add a protocol:
  1. **Two channels:** the teacher writes the *user turn* (in role, short) and, when the agent fails, a separate `[teach]` block that goes to the agent tagged `@source teacher` and is never treated as the user's turn.
  2. **Learnable chunks:** one chunk per `[teach]` block, in fixed sentence frames the spec supports ("To make garlic tomato pasta you need pasta, garlic, olive oil and cherry tomatoes." / "Step 2 of making garlic tomato pasta: fry sliced garlic in olive oil for 1 minute." / "Garlic tomato pasta takes 15 to 20 minutes." / "Garlic tomato pasta is a good dinner when you have little time.").
  3. **Show what was understood:** give the teacher the canonical lines and stored atoms from the last turn (the trainer already computes `facts_added`), so it can fix exactly the wrong atom instead of repeating the whole lesson.
  4. **Forbidden:** formatting requests ("one per line", "say it back"), apologies, playing the assistant, more than 3 sentences per user turn.
  5. **Retry rule:** after a teaching block, re-ask the *original* request once in the same words. If it still fails, move on and record the conversation as "taught, not learned".
- **Why:** F11. 17/68 teacher turns are meta or formatting requests that the symbolic system cannot satisfy by design. 33/68 apologise. The beach lesson repeated 7 times. The picnic teacher drifted into the assistant role. Long messages drive F3.
- **Metric:** *facts stored per teaching block*, *teaching blocks until first success*, the share of teaching blocks with reviewer rejections, and the not-understood rate on teaching blocks.
- **Risks:** Over-regular teaching makes the agent good at "teacher English" only. Keep the *user* turns natural and only the `[teach]` channel regular, and test with judges who never teach (R8).

#### R8. Curriculum and repetition to measure real learning (P0, M)

- **Build:** Each scenario becomes an item with a fixed goal checklist (for example dinner: proposes a feasible dish, gives steps in order, gives a time ≤ 20 min, closes politely). Then:
  - **Encounter 1:** teach as needed.
  - **Encounter 2:** a fresh session after other conversations, a paraphrased opening, and **no teaching allowed**. The teacher only plays the user.
  - **Transfer:** a variant (no garlic; 10 minutes; rice instead of pasta).
  - **Retention:** encounter 2 repeated after N days or N conversations.
  - Scenarios that fail encounter 2 go back into a spaced-repetition queue.
- **Why:** Today every conversation is a first encounter. Learning is reported as "+44 facts", which says nothing about whether the next user gets helped. The dinner conversation proves facts and help are decoupled (0 facts, 0 help).
- **Metric:** **solved-without-teaching rate on encounter 2**, the main metric. Also transfer rate and retention rate.
- **Risks:** Paraphrases written by the same model may be too close to the original. Use a different prompt or seed, and keep a held-out scenario set never used in teaching.

#### R9. Outcome classification instead of "understood" (P0, S)

- **Build:** Label every agent turn with one outcome: `helpful_answer`, `partial_answer`, `appropriate_clarification`, `honest_dont_know_with_teach_request`, `not_understood`, `misfire` (wrong act), `wrong_content`, `echo_only`. Use the same regex families as this analysis for template outcomes, plus a cheap Haiku judge with a rubric for content turns. Label each conversation `goal_reached` / `reached_after_teaching` / `not_reached`. Store the labels in the trainer state and in `Finished`.
- **Why:** "understood" = 56% while helpful = 1.5% (S1).
- **Metric:** this *is* the metric source for R1–R8 and section 4.
- **Risks:** Judge drift. Calibrate on 50 hand-labelled turns (S1 already is one) and re-check monthly.

#### R10. Knowledge hygiene (P1, M)

- **Build:**
  - (a) A **predicate registry** with synonyms (`same_pred(cope_with, tolerate)`, `handle` → `tolerate`). Map new predicates to existing ones at translation time (R6) and in the reviewer.
  - (b) **Entity normalization:** merge plural and singular duplicates (19 pairs found: `chive`/`chives`, `sandwich`/`sandwiches`, `knive`, `sunglass`), keep surface forms (R5), and reject or split entities of 3+ words (99 found, such as `good_for_eggs_and_frying` and `biggest_country_in_south_america`) into atoms.
  - (c) **Link fragments to tasks:** facts learned while a task frame is open get `context(fact_id, task)`, or are turned into step atoms. "Quick release lever holds wheel." should hang off `fix_flat_tire`.
  - (d) A **periodic hygiene pass:** duplicates, singleton predicates (132/216 = 61%), the `property(_, quality, _)` catch-all (200 facts), and orphan facts not reachable from any category or task.
  - (e) The reviewer also checks **situational versus general** knowledge ("person wants apple" was rejected only for overgeneralization).
- **Why:** F8 and F9. Predicate fragmentation is why "Mint copes with sun" cannot answer "how much sun does mint need?".
- **Metric:** new predicates per conversation (should fall over time), singleton-predicate ratio, duplicate-entity count, orphan-fact ratio, reviewer rejection rate.
- **Risks:** Wrong synonym merges silently change meaning. Merges should be proposals reviewed like rules, logged and reversible (the vault log already supports this).

#### R11. Small talk and self-model module (P1, S)

- **Build:** Self-model facts (`agent_cannot(taste)`, `agent_can(help_plan, cook_steps)`) and a small-talk policy: for an opinion or experience question, say one honest line about itself, then add *general knowledge plus a return question*, and never repeat the same template in one conversation. ("I can't taste food, but lots of people like thick soup on cold nights. What's your favourite?") Narrow `about_you` in the translator: "Do you think we'll get snow?" is a prediction question, not `about_you`.
- **Why:** F6 (58% of friend-chat turns; the same template up to 3 times per conversation).
- **Metric:** self-deflection rate; repeated-reply rate; friend-chat judge score.
- **Risks:** Low. The main risk is still sounding canned. Rotate by *content* (different facts) rather than by wording.

#### R12. Gate curiosity and replace "not understood" with partial understanding (P2, S)

- **Build:** In user-facing chat, curiosity may only ask about missing slots of the active task, or about the entity just discussed, and never about rule hypotheses ("if X is a foods from plant...": route those to teachers offline). When translation is partial, answer the understood part and ask one targeted clarification ("Do you want the steps for fixing the tire?") instead of "Could you say that more simply?".
- **Why:** F12 (2/8 judge conversations derailed); F3 replies give the user no way forward.
- **Metric:** off-topic question rate; *recoverable failure rate* (a failure turn followed by success within 2 turns).
- **Risks:** Fewer curiosity questions means slower gap discovery, which the offline `ask_teacher` covers.

#### R13. Training-loop plumbing (P2, S)

- **Build:** Run one translator call per turn in parallel with the teacher's next call, where possible. Record per-turn outcome labels, translator retries and canonical lines in the snapshot. Add a "show what agent understood" panel per turn (canonical lines → stored atoms → reply act).
- **Why:** Debugging this analysis needed cross-referencing three files (snapshot, log, cache) for each turn.
- **Metric:** time to diagnose a failure (qualitative); seconds per conversation.
- **Risks:** None significant.

**Suggested order.** R7 + R9 (cheap, they make measurement honest) → R1 + R3 + R6 (the turn-1 path) → R2 + R4 + R5 (the turn-2 path) → R8 (prove learning) → R10, R11, R12, R13.

---

### 4. Proposed metric suite for the training UI

Show each metric per run and as a trend over runs, split by **seen** and **held-out** scenarios.

| Group | Metric | Definition | Target (first milestone) |
|---|---|---|---|
| **Real learning** (headline) | Solved without teaching (encounter 2) | Share of encounter-2 conversations whose goal checklist is fully met with no `[teach]` blocks | ≥ 50% on seen scenarios |
| | Transfer rate | Same, for variant scenarios (different ingredients or constraints) | ≥ 30% |
| | Retention | Encounter-2 success after N later conversations or days, divided by immediate encounter-2 success | ≥ 0.8 |
| | Teaching cost | Mean `[teach]` blocks until the first successful answer in encounter 1 | falls over runs |
| **Turn outcomes** | Outcome mix | Stacked bar of R9 labels per run | helpful + clarification ≥ 60% |
| | Request handled rate | Requests answered with a task-appropriate act | ≥ 80% |
| | Wrong-content rate | Turns with false or infeasible content (judge + proof check) | ≤ 2% |
| | Honest-unknown ratio | honest "don't know, teach me" / (honest + wrong) | high; wrong answers are worse than "don't know" |
| **Conversation** | Goal completion | `goal_reached` + `reached_after_teaching` + `not_reached` per scenario | |
| | Turns to goal | Median turns | ≤ 4 for dinner-like tasks |
| | Judge score | Haiku judges, fixed scenario set, 1–10 | trend up; watch the spread |
| **Translator** | Coverage | Turns not translated to `unknown:` only | ≥ 90% |
| | Validation failure / retry rate | Share of calls rejected, share retried | |
| | Grounding violations | Constants not present in the message or state | 0 |
| | Latency p50/p95, cost per turn, cache-read share, few-shot hit share | | p50 < 2 s |
| **Knowledge hygiene** | New predicates per conversation; singleton-predicate ratio; duplicate entities; orphan-fact ratio; task completeness; reviewer rejection rate | see R10 | all falling except task completeness |
| **NLG** | Echo rate; repeated-reply rate; self-deflection rate; "already knew" rate; grammar flags per 100 turns (judge) | | echo < 5%, repeats < 5% |
| **Teacher quality** | Meta or formatting requests, apologies, role-drift turns, facts per teaching block | from R7 | meta = 0 |

Also show a **per-conversation drill-down** with three aligned rows per turn (message → canonical lines → stored atoms and act → reply), so a reviewer can see in one place *where* a turn failed: translator, missing representation, policy or NLG.

---

### 5. Risks to the core idea (symbolic replies only)

1. **Long-tail coverage.** Everyday help is open-domain. Each task needs schema instances (dishes, steps, items), and a user who asks something never taught gets "I don't know how to do that yet". *Mitigation:* a few generic schemas (procedure, recommendation, checklist, quantity, comparison), plus offline bulk teaching. Teachers author structured knowledge in reviewed batches (an allowed teacher role), so conversations mostly test and refine it. Make the honest "teach me" reply graceful rather than a failure.
2. **The translator becoming the hidden reasoner.** The more context and few-shot examples it gets, the easier it is for the LLM to slip answers into a translation (for example `tell: solves(garlic_tomato_pasta, ...)` for "What can I make?"). *Mitigation:* the grounding check in R6, translations limited to the user's own content, and audit logging of every translation that adds a constant that appears nowhere in the message.
3. **Adaptation and composition.** Real users change constraints ("no garlic", "only 10 minutes", "for 4 people"). Substitutions, scaling and timing need rules (`substitute(garlic, garlic_powder)`, portion arithmetic) and quickly approach planning. *Mitigation:* the HTN/GOAP planner already named in DESIGN.md, applied to step lists. Accept "closest option + what's missing" as an honest fallback.
4. **Naturalness ceiling of templates.** Even with R5, replies will feel assembled, especially in small talk, where good replies are mostly free-form and empathic. *Mitigation:* invest in a discourse planner and surface lexicon rather than more templates. Measure naturalness separately from helpfulness, and accept a lower naturalness score as the price of faithfulness.
5. **Vagueness and grounding.** "Until they soften", "a splash", "about a minute" have no truth conditions to reason over. *Mitigation:* store them as opaque condition atoms for replay. Reason only over the numeric parts.
6. **Wrong knowledge stored as truth.** S1 rejected 14% of new facts; S2/S3 rejected 49/577 and 57/370. Procedures make errors costlier, because one wrong step ruins the answer. *Mitigation:* procedures as units with confidence and status (hypothesis → confirmed after a second teacher or a successful encounter-2 use), reviewer checks per step, and `task_outcome` feedback.
7. **Evaluation subjectivity and self-reference.** Teachers, judges and paraphrasers are all Haiku, so the agent may learn "Haiku-English" and judges may grade kindly or noisily (scores move ±1.5 between runs with no code change). *Mitigation:* fixed checklists per scenario (R8), held-out scenarios, a small human-labelled calibration set, and a different model or prompt for judges than for teachers.
8. **Latency and cost of translation.** 96% of turn time is the LLM. A symbolic agent that waits 5 s per turn feels worse than its replies deserve. *Mitigation:* R6 (d, e, g); grow the pattern parser for the frequent dialogue acts (thanks, accept, "what next", short informs) so most turns never reach the model.

---

## 7. Related work and strategic options

*Scope note.* This section was written without web search. References are given from memory as author, year and venue where known; check exact years and titles before publishing. Each item says what it teaches *this* project: a symbolic agent whose replies must come from its own Datalog knowledge base, rules and grammar. An LLM may only translate input into the formal language, and LLMs act as teachers and judges during training.

Where the prototype stands: knowledge tests are near 100% (dev) and 97.8% (held-out), and the parser scores 100% and 98%. Conversation quality is about 3.3 to 4.3 out of 10. In help-style conversations, only about 15% of replies are real answers; 43% are "not understood", 18% are understood but unanswered, and 20% only acknowledge fragments. The diagnosis is that the system has **declarative** knowledge but no **procedural** knowledge, no **task or dialogue state**, and a **realizer without a microplanner**. Most of the related work below speaks to one of these three gaps.

---

### 1. Related work, by theme

#### 1.1 Semantic primes and core ontologies

- **Natural Semantic Metalanguage (NSM)** (Wierzbicka 1972, 1996; Goddard & Wierzbicka 2002, 2014). NSM proposes about 65 universal semantic primes (I, YOU, SOMEONE, SOMETHING, DO, HAPPEN, WANT, KNOW, BEFORE, AFTER, BECAUSE, IF, CAN, FOR SOME TIME, PART, KIND, ...). It explicates every other word through "reductive paraphrase" into primes. It also uses **semantic molecules**: an intermediate layer of non-prime concepts such as *hands*, *make*, *water* and *day*, which many explications reuse.
  *Lesson:* the project's ~20 core predicates are a reasonable logical reading of NSM, but they leave out exactly the primes that procedures need: DO (with a time order), CAN, FOR SOME TIME (duration), IF/BECAUSE as action conditions, and MOVE/TOUCH. NSM's molecule layer is the model for a **procedural core**: a small, fixed set of predicates (`step`, `needs`, `result_of`, `takes_time`) defined once by rules over the primes and then shared by every learned procedure. NSM was built to define meanings, not to support inference. Use it to decide what goes in the core, not as a reasoning formalism.
- **Cyc** (Lenat 1984–present; Lenat & Guha 1990). Cyc holds millions of hand-written assertions in CycL, a higher-order logic. Context is handled by **microtheories**, and Cyc has an extensive upper ontology.
  *Lesson:* (a) microtheories are what the design calls "domain modules". Cyc shows that context and module boundaries matter more than the size of the upper ontology. (b) Forty years of hand-encoding did not produce natural conversation, so knowledge alone is not dialogue competence. (c) Cyc's coverage problem is the reason the cold-start teacher needs to scale.
- **ConceptNet 5.5** (Speer, Chin & Havasi, AAAI 2017). ConceptNet is a crowd-sourced graph with about 34 relations. Several of them are directly procedural: `HasPrerequisite`, `HasSubevent`, `HasFirstSubevent`, `HasLastSubevent`, `UsedFor`, `CapableOf`, `MotivatedByGoal`, `Causes`, `AtLocation`.
  *Lesson:* this is ready-made, though noisy, seed data for `needs` and `step`. It could be imported as low-confidence `@source conceptnet` facts that teachers then confirm or correct. ConceptNet has no step order beyond first and last, and no durations.
- **ATOMIC / ATOMIC-2020** (Sap et al., AAAI 2019; Hwang et al., AAAI 2021). These are if-then commonsense graphs about events, with relations such as `xNeed` (what must happen first), `xIntent`, `xWant` (what the person wants next), `xEffect` and `oReact`.
  *Lesson:* `xNeed` and `xWant` are the **before and after slots of a dialogue turn**. They are the knowledge an agent needs to make a *helpful* follow-up ("you'll need a jack first") and a *social* follow-up ("hope the trip goes well"). Here small talk and assistance come from the same representation.
- **WordNet** (Miller 1995; Fellbaum 1998). WordNet provides synsets, hypernyms, meronyms and verb entailment and troponymy.
  *Lesson:* use it as a lexicon, not a knowledge base. It serves synonym normalization in the parser (`automobile` → `car`), lexical choice and variation in the realizer, and an `is_a` backbone for gap questions. Verb entailment (*snore* → *sleep*) yields sub-event rules.

#### 1.2 Datalog engines, incremental evaluation, truth maintenance

- **Soufflé** (Jordan, Scholz & Subotić, CAV 2016). Soufflé compiles Datalog to parallel C++. It supports stratified aggregates (`count`, `sum`, `min`) and records **provenance** as compact proof annotations: for each tuple it stores the rule and the height, and proof trees are rebuilt on demand (Zhao, Subotić & Scholz, TOPLAS 2020).
  *Lesson:* (a) store the minimal-height annotation instead of full traces, which keeps "why" answers cheap as the knowledge base grows. (b) Procedures need **stratified aggregation**: "how long does it take in total?" is a `sum` over step durations. The interpreter will need this, together with arithmetic built-ins that cannot create unbounded new values.
- **Incremental maintenance**: DRed, delete-and-rederive (Gupta, Mumick & Subrahmanian, SIGMOD 1993); counting algorithms; **differential dataflow** (McSherry et al., CIDR 2013) and **DDlog** (Ryzhyk & Budiu, Datalog 2.0 2019).
  *Lesson:* dialogue state changes every turn (current step, open question), so recomputing from scratch per message eventually becomes the bottleneck. DRed is the simplest fit for facts closed with `@until`. Differential dataflow is the principled solution but heavy for a zero-dependency crate. Treat it as a later optimization.
- **Existential rules / Datalog±** (Calì, Gottlob & Lukasiewicz 2009).
  *Lesson:* plain Datalog cannot invent new objects. A procedure instance ("this tire change, step 3") needs fresh identifiers. Create them outside the reasoner (the dialogue manager mints `task_17`) rather than adding existential rules.
- **Truth maintenance**: JTMS (Doyle, AIJ 1979), ATMS (de Kleer, AIJ 1986). **Belief revision**: AGM postulates (Alchourrón, Gärdenfors & Makinson, JSL 1985); epistemic entrenchment.
  *Lesson:* proof traces are already JTMS justifications. Retracting a fact must retract what depended on it, which the system gets by recomputation. An ATMS-style label ("true under hypothesis rules H1 and H2") would let the agent answer while *marking* reliance on unconfirmed rules, for example "probably, if ...". AGM entrenchment corresponds to confidence plus source. Corrections should give up the least entrenched belief, which is what the current "refine the top rule of the proof" heuristic approximates.

#### 1.3 Inductive logic programming and rule learning

- **FOIL** (Quinlan, MLJ 1990): greedy, information-gain-driven clause construction. **Progol / Aleph** (Muggleton 1995; Srinivasan): inverse entailment from a bottom clause. **Metagol** (Muggleton, Lin & Tamaddoni-Nezhad, MLJ 2015): meta-interpretive learning with metarules, which can **invent predicates** and learn recursion. **Popper** (Cropper & Morel, MLJ 2021): learning from failures, where an ASP solver generates hypotheses and failed hypotheses become constraints. Survey: Cropper & Dumančić, "Inductive logic programming at 30" (JAIR 2022).
  *Lesson:* the pattern-counting learner is a one-literal FOIL without negatives search. Popper is the natural next step because it learns multi-literal, recursive rules from few examples, and the curiosity drive and teachers can supply negative examples on demand. Metagol-style metarules suit procedures: a chain metarule `P(A,B) :- Q(A,C), R(C,B)` is the shape of "step then step".
- **AMIE / AMIE+** (Galárraga et al., WWW 2013; VLDBJ 2015). AMIE mines Horn rules from large open-world knowledge bases and introduced **PCA confidence** (the partial completeness assumption).
  *Lesson:* this directly addresses the open-world versus closed-world choice in PROGRESS.md (closed world for observed episodes, explicit negation only for `knows`, `wants`, `has`). PCA confidence is a published, principled rule for that case.
- **ILASP** (Law, Russo & Broda 2014–2020). ILASP learns answer set programs with defaults and exceptions.
  *Lesson:* corrections that add `not abnormal`-style exceptions are a special case of learning non-monotonic rules. ILASP's framing ("brave and cautious examples") helps when designing the correction test set.
- **Action-model learning**: ARMS (Yang, Wu & Jiang, AIJ 2007), LOCM (Cresswell, McCluskey & West 2009/2013), and Wang's OBSERVER (1995).
  *Lesson:* the `:demo before/after` mechanism is STRIPS action-model learning, intersecting preconditions over demonstrations, which is OBSERVER's method. This literature also shows how to learn from **partial** observations, which is what a teacher's narrated how-to provides.
- **Interactive task learning (ITL)**: Instructo-Soar (Huffman & Laird, JAIR 1995), Rosie (Mohan, Mininger, Kirk & Laird 2012; Laird et al., IEEE Intelligent Systems 2017), PLOW (Allen et al., AAAI 2007: learning procedures from one demonstration with narration).
  *Lesson:* this is the closest prior work to the project as a whole. Symbolic agents learned new tasks, goals and procedures **through natural dialogue with an instructor** and generalized from one or two examples. Rosie's lesson is that the learner must ask *structured* questions at the moment of failure ("What is the next step?", "When is the task done?"). That suits the curiosity drive.
- **Chunking and library learning**: Soar chunking (Laird, Rosenbloom & Newell, MLJ 1986), ACT-R production compilation (Anderson), DreamCoder (Ellis et al., PLDI 2021).
  *Lesson:* "save solved reasoning chains as shortcut rules" is chunking. DreamCoder shows that the value lies in *abstracting shared sub-structure* across many solved tasks ("loosen the bolts" appears in tire, wheel and shelf procedures), not in caching single chains.

#### 1.4 Scripts, frames, plans and procedural knowledge

- **Scripts** (Schank & Abelson, *Scripts, Plans, Goals and Understanding*, 1977; the SAM and PAM programs). A script is a stereotyped event sequence with **roles, props, entry conditions, scenes and results**; the restaurant script is the classic example. A **plan** in their sense is the goal-driven fallback used when no script fits.
  *Lesson:* this is the missing procedural layer. "Plan dinner" and "fix a flat tire" are scripts. Small talk is also scripted: greeting → how-are-you → topic → closing. Schank's split between script (known routine) and plan (derive from goals) maps onto the agent: answer from a stored procedure if one exists, otherwise compose one from `causes` and effect rules, otherwise ask.
- **Frames** (Minsky, "A Framework for Representing Knowledge", 1974). Frames are structures with slots, **default values** and attached procedures.
  *Lesson:* a task frame (`dinner_plan`: guests, date, cuisine, dietary limits, budget) is both knowledge and dialogue state. Defaults correspond to Datalog defaults with exceptions, which the system already has.
- **Script learning from text**: narrative event chains (Chambers & Jurafsky, ACL 2008); crowd-sourced script corpora DeScript (Wanzare et al., LREC 2016) and InScript (Modi et al., LREC 2016); **proScript** (Sakaguchi et al., Findings of EMNLP 2021), which uses partial-order scripts generated and evaluated with LMs.
  *Lesson:* real procedures are **partial orders**, not lists. Store `before(S1, S2)` edges, which is already a core predicate, rather than step numbers alone.
- **How-to corpora**: wikiHow datasets (Koupaee & Wang 2018; Zhang, Lyu & Callison-Burch, "Reasoning about Goals, Steps, and Temporal Ordering with WikiHow", EMNLP 2020); recipe corpora (Recipe1M, Marín et al. 2019).
  *Lesson:* wikiHow gives goal → steps → sub-steps with a clear structure, and it is a good *source for teachers*. Durations and tool lists are often implicit, so teachers have to make them explicit.
- **HTN planning**: SHOP / SHOP2 (Nau et al., JAIR 2003), with ordered task decomposition into methods with preconditions. **PDDL** (McDermott et al. 1998) and **PDDL 2.1** durative actions (Fox & Long, JAIR 2003). **GOAP** (Orkin, F.E.A.R., 2006). **HTN-MAKER** (Hogg, Muñoz-Avila & Kuter, AAAI 2008), which learns HTN methods from traces.
  *Lesson:* a help request is an HTN query: "how do I fix a flat?" is decomposing `fix(flat_tire)` and verbalizing the methods. "What do I need?" is the union of preconditions. "How long?" is the sum of durative actions. HTN methods fit naturally as Datalog facts plus a small decomposition interpreter, and HTN-MAKER shows they can be learned from demonstrations plus teacher annotations.
- **Collaborative plans**: SharedPlans (Grosz & Sidner 1990; Grosz & Kraus, AIJ 1996); **Collagen** (Rich & Sidner, UMUAI 1998).
  *Lesson:* assistance dialogue is *joint* plan execution. The agent tracks which steps the user has done and which they lack resources for, and its contributions are moves in the shared plan ("next, loosen the nuts"). Collagen built this on a symbolic plan recognizer, and it is the closest architectural template for "help as conversation".

#### 1.5 Dialogue: task-oriented systems and classic chatbots

- **GUS** (Bobrow et al., AIJ 1977) introduced **frame-driven dialogue**: slots, defaults and mixed initiative for travel booking. **VoiceXML's form interpretation algorithm** industrialized it.
  *Lesson:* a frame plus a "next unfilled slot" policy already produces coherent multi-turn help. It is the cheapest way to fix the "understood but no answer" 18%.
- **Dialogue state tracking**: the DSTC challenges (Williams et al., SIGDIAL 2013; Williams, Raux & Henderson, D&D 2016), MultiWOZ (Budzianowski et al., EMNLP 2018), Schema-Guided Dialogue (Rastogi et al., AAAI 2020).
  *Lesson:* state means slot values plus the active intent plus the requested slots. SGD's key idea is that **tasks are declared as data (schemas)**, so new tasks need no new code. In this project, schemas would be vault files the agent learns.
- **Information-state update** (Traum & Larsson 2003; TrindiKit; Larsson's IBiS, 2002), **Questions Under Discussion** (Ginzburg, *The Interactive Stance*, 2012), **grounding** (Clark & Schaefer 1989; Traum 1994), dialogue games and adjacency pairs (Levin & Moore 1977; Schegloff & Sacks 1973), and conversation as joint activity (Clark, *Using Language*, 1996).
  *Lesson:* the information state (QUD stack, agenda, shared commitments, grounding status) can be **Datalog facts in a session module**, and update rules can be **Datalog rules**. That makes the dialogue policy traceable and learnable with the same machinery as world knowledge. Grounding acts ("ok, got it — so you have a jack but no spare") replace the current fragment acknowledgments.
- **RavenClaw** (Bohus & Rudnicky, CSL 2009). RavenClaw separates a **task tree with an agenda** from domain-independent conversational strategies (repair, timeout, help).
  *Lesson:* it demonstrates that task-independent error handling and grounding can be written once and reused across tasks.
- **Classic chatbots**: ELIZA (Weizenbaum, CACM 1966), PARRY (Colby 1972), A.L.I.C.E./AIML (Wallace 1995–2009), and Mitsuku/Kuki (Worswick; multiple Loebner Prizes, hundreds of thousands of AIML categories). **Alexa Prize socialbots** (Ram et al. 2018; e.g. Gunrock, Chen et al. 2018) used hybrid modular topic handlers with templated NLG.
  *Why they plateau:* they map surface patterns to surface replies with no world model and little state, so coverage grows linearly with hand-written patterns while the input tail is unbounded. Replies repeat because variation is template rotation. They feel coherent locally but forget across turns. The Alexa Prize systems got further with **topic or task modules, dialogue-act planning and explicit state**, but still relied on large hand-written response banks.
  *Lesson:* the 20% "acknowledges fragments" bucket is ELIZA behaviour. Adding more patterns will not move it. State, procedures and a microplanner will.

#### 1.6 Natural language generation

- **Reiter & Dale pipeline** (*Building Natural Language Generation Systems*, 2000): (1) document planning (content determination, document structuring), (2) **microplanning** (lexicalization, referring-expression generation, **aggregation**), (3) surface realization. Survey: Gatt & Krahmer (JAIR 2018).
  *Lesson:* the current generator jumps from content to templates. The "robotic" feel mostly comes from the missing microplanner. It never merges facts into one clause, never chooses pronouns versus names consistently across turns, never uses discourse markers ("first", "then", "once that's done", "by the way"), and never elides what the user just said.
- **SimpleNLG** (Gatt & Reiter, ENLG 2009) is a realizer library: you give it a clause spec (subject, verb, object, tense, negation, modals, complements) and it handles morphology, agreement and ordering. It has ports to several languages.
  *Lesson:* a small Rust realizer with the same interface is the right scale for this project. It is roughly 1–2k lines and replaces templates with clause specs built from atoms.
- **Grammar-based realization**: FUF/SURGE (Elhadad & Robin 1996), KPML (Bateman 1997), OpenCCG (White 2006), and **Grammatical Framework (GF)** (Ranta 2004/2011). GF uses one abstract syntax with concrete grammars that are *both parsed and generated*.
  *Lesson:* GF-style reversibility fits the existing faithfulness check, which already parses every generated claim back to its atom. One grammar for both directions makes that guarantee structural instead of tested.
- **Referring expressions**: the incremental algorithm (Dale & Reiter, Cognitive Science 1995). **Discourse structure**: Rhetorical Structure Theory (Mann & Thompson 1988).
  *Lesson:* procedure explanations are RST sequences and elaborations, and "why" answers are RST cause and evidence relations. A tiny RST-based structurer gives multi-sentence replies their order and connectives.

#### 1.7 Semantic parsing with LLMs (LLM as translator)

- **Constrained decoding into formal languages**: Shin et al., "Constrained Language Models Yield Few-Shot Semantic Parsers" (EMNLP 2021), which uses canonical utterances and grammar-constrained decoding; PICARD (Scholak et al., EMNLP 2021); grammar-constrained decoding without fine-tuning (Geng et al., EMNLP 2023); Outlines (Willard & Louf 2023); GBNF grammars in llama.cpp; JSON-schema structured outputs in commercial APIs.
  *Lesson:* if the LLM can only emit strings in the agent's formal language over a **closed predicate vocabulary exported from the vault**, invalid output is impossible. That leaves two failure modes, wrong atom and wrong predicate choice, and both can be checked.
- **LLM-to-logic pipelines**: LINC (Olausson et al., EMNLP 2023), where the LLM translates to first-order logic and a prover reasons, with majority voting over translations; Logic-LM (Pan et al., Findings EMNLP 2023), which self-refines from solver error messages; SatLM (Ye et al., NeurIPS 2023).
  *Lesson:* this split is the project's constraint, and the literature reports that **translation errors dominate**. Two cheap safeguards follow. (a) Sample several translations and keep the consensus. (b) Verify by round trip: realize the parse with the agent's own NLG and check it against the input, either by asking the user ("So you have a jack but no spare?") or by having the translator re-parse it.
- *Risk specific to this project:* an open-vocabulary translator invents predicates (`own_vehicle`, `has_car`, `possesses`). A **lexicon canonicalization** step maps them to existing predicates via WordNet-like synonyms and core definitions, or queues a curiosity question. Without it the knowledge base fragments.

#### 1.8 Neuro-symbolic systems and LLM-as-teacher / distillation

- **Symbolic knowledge distillation** (West et al., NAACL 2022). GPT-3 generated ATOMIC-style commonsense, a trained critic filtered it, and the result (ATOMIC-10x) was larger and higher quality than human-authored data.
  *Lesson:* this is the cold-start plan at scale, and it shows that **a filter or critic is essential**. In this project the critic is a mix of teacher cross-checks, consistency with the knowledge base, and the curiosity drive's contradiction detection.
- **LLMs as world-model builders**: LLM+P (Liu et al. 2023), where the LLM writes a PDDL problem and a classical planner solves it; Guan et al. (NeurIPS 2023), where the LLM builds PDDL domain models that humans correct; Valmeekam et al. (NeurIPS 2023), which finds that LLMs plan poorly on their own but are useful as knowledge sources.
  *Lesson:* use teachers to *write* procedures and action models into the vault, after validation, and keep planning and answering symbolic. That is exactly the design's division of labour.
- **Never-ending learning**: NELL (Carlson et al. 2010; Mitchell et al., CACM 2018).
  *Lesson:* a knowledge base that keeps growing from noisy sources suffers **semantic drift** (errors compound through rules). NELL used coupled constraints and periodic human review. The equivalent here is promotion thresholds, held-out regression tests and an audit log, all of which are already designed.
- **Skill libraries**: Voyager (Wang et al. 2023), with an LLM-driven automatic curriculum and a stored, reusable skill library.
  *Lesson:* the curriculum-plus-library loop works, but Voyager's skills are LLM-written code. Here the "skills" must be symbolic procedures the agent can explain, so the library is the vault's procedure modules.
- **Cognitive architectures**: Soar and ACT-R.
  *Lesson:* ACT-R's base-level activation equation, B = ln Σ t_j^(−d), is a ready-made, well-studied formula for usage-based forgetting and retrieval priority using the `@uses` and `@last` fields already recorded.

#### 1.9 Curriculum, spaced repetition, active learning, curiosity

- **Curriculum learning** (Bengio et al., ICML 2009). **Automated curricula** driven by learning progress (Graves et al., ICML 2017). **Machine teaching** and teaching dimension (Goldman & Kearns 1995; Zhu 2015; Zhu et al. 2018).
  *Lesson:* a teacher that picks the *minimal* example set to pin down a rule (one positive, one near-miss negative; cf. Winston's near misses, 1970) is far more sample-efficient than random dialogues. The teacher protocol should be designed as machine teaching, not conversation sampling.
- **Exact learning with queries**: Angluin's L* and the minimally adequate teacher model (Inf. & Comp. 1987).
  *Lesson:* the teacher already answers membership queries ("Does Pingu fly?") and could answer equivalence queries ("Here is my rule. Is it right? If not, give a counterexample."). That formalizes what "confirmed" means and bounds how many questions learning needs.
- **Active learning** (Settles, survey 2009/2012): uncertainty sampling, query-by-committee, expected model change.
  *Lesson:* curiosity goals should be ranked by *expected change to answer-relevant rules*, not by gap counts.
- **Curiosity as learning progress**: Schmidhuber (1991, compression progress); Intelligent Adaptive Curiosity (Oudeyer, Kaplan & Hafner, IEEE TEC 2007); ICM (Pathak et al., ICML 2017) and its "noisy TV" failure mode.
  *Lesson:* the design already rewards learning progress, not novelty, which is correct. Learning progress should be measured **per topic or module** (as IAC does per region), so the agent moves away from topics it has mastered or cannot learn.
- **Spaced repetition**: Leitner boxes; SM-2 (Woźniak 1990); half-life regression (Settles & Meeder, ACL 2016); FSRS (2022–).
  *Lesson:* a symbolic store does not forget, so spaced repetition here is **re-verification**, not memorization. It schedules re-tests of hypothesis rules and old facts, which may have expired in the world, at intervals that grow with confidence. It also gives the measured "second encounter" test a principled schedule.

#### 1.10 Evaluation with LLM judges

- **LLM-as-a-judge**: Zheng et al. (NeurIPS 2023 Datasets & Benchmarks, MT-Bench and Chatbot Arena) found strong judges reach about 80% agreement with humans, with documented **position, verbosity and self-enhancement biases**. G-Eval (Liu et al., EMNLP 2023) adds rubric-based form filling. Smaller judges, such as Haiku-class models, are noisier and more lenient or harsh depending on prompt wording.
- **Dialogue-specific evaluation**: ACUTE-Eval (Li, Weston & Roller 2019) found **pairwise comparisons of whole conversations** more reliable than Likert scores. SSA, sensibleness and specificity average (Adiwardana et al., Meena, 2020), uses per-turn binary labels. FED and USR (Mehri & Eskenazi 2020) score fine-grained qualities.
  *Lesson:* three judges × six turns on a 1–10 scale cannot separate a 3.3 from a 4.3. The help-conversation breakdown (real answer, not understood, understood without answer, fragment acknowledgment) is already more informative than the 1–10 score. Formalize it as **per-turn binary labels** in the style of SSA, plus **pairwise A/B against the previous version** in the style of ACUTE. Use fixed scenario seeds, report confidence intervals, and calibrate judges on a few anchor transcripts with known scores. Keep the 1–10 score as a trend line only.

---

### 2. Strategic options for the next phase

Effort is given in *iterations*: one autonomous Claude Code run of the size used for milestones 1–10, roughly 3–4 milestones each.

#### Option A — Deepen the symbolic dialogue system: scripts/procedures + dialogue state

Add a **procedural core**: `task`, `step(Task, Step)`, `before(S1, S2)`, `needs(TaskOrStep, Thing)`, `takes_time(Step, Minutes)`, `result_of(Step, Fact)`, `method(Task, Subtask)`, all defined over the existing primes. Add a **session module** holding the information state: active task, current step, QUD stack, filled and missing frame slots, grounding status, last referents. Dialogue moves are selected by Datalog rules over that state. Small talk is handled as a set of social scripts (greeting, check-in, topic follow-up using `xWant`/`xNeed`-style knowledge), so assistance and small talk run on one mechanism.

- **Pros:** attacks the 18% "understood, no answer" and much of the 20% "fragments" directly. Keeps everything traceable ("why do I need a jack?" is answered from the proof). The dialogue policy itself becomes learnable by the existing learner and correction machinery. Closest to the proven ITL, Collagen and RavenClaw lineage.
- **Cons:** does nothing for the 43% "not understood" without parser work. Procedures must come from somewhere (teachers or imports). Partial-order steps, aggregates (sum of durations) and fresh task-instance IDs need interpreter extensions.
- **Effort:** 1–1.5 iterations.
- **Proves it:** on held-out help scenarios, real-answer rate rises from 15% to ≥ 50% with no rise in false claims, and the agent resumes a task correctly after a small-talk interruption.
- **Disproves it:** if real-answer rate stays below 30% even when inputs are hand-parsed (an oracle parser), then procedures plus state are not the bottleneck.

#### Option B — Grammar-constrained translator + microplanned NLG (SimpleNLG-like realizer)

Plug an LLM translator behind the existing `Parser` trait. Its output is constrained by a grammar (GBNF or JSON schema) generated from the vault's predicate lexicon, with consensus over several samples and canonicalization of new predicates. In parallel, replace templates with a pipeline of content plan → microplanner (aggregation, referring expressions, discourse markers, ellipsis, grounding acknowledgments) → a small Rust clause realizer. Optionally, move toward a GF-style reversible grammar so the parser and realizer share one grammar.

- **Pros:** the translator targets the largest bucket, 43% not understood. The microplanner targets the "robotic" judgment. It is consistent with the constraint, since the LLM only translates. The faithfulness test (claims parse back) carries over unchanged.
- **Cons:** a translator increases understanding but not answering. If the knowledge base has no procedures, better parsing turns "not understood" into "understood, no answer". It adds a runtime LLM dependency, latency and cost, and a new error mode (confidently wrong translations stored as facts). Realizer work has diminishing returns without better content.
- **Effort:** 1 iteration for the translator and microplanner. A reversible GF-style grammar would take another 1–2 iterations.
- **Proves it:** "not understood" ≤ 10% on held-out help and small-talk scenarios; translator atom-level accuracy ≥ 95% on a held-out set of free-form sentences with gold parses; pairwise judge preference ≥ 65% for the new NLG over the old one on identical content.
- **Disproves it:** if judges prefer the new NLG at chance level (≤ 55%) on identical content, wording was not the problem. If translator errors that slip past the round-trip check exceed 5%, the translator path is unsafe for writing to memory.

#### Option C — Curriculum-based teacher protocol with measured second-encounter learning

Turn teacher dialogues from scripted samples into a **machine-teaching protocol**. A curriculum graph runs core → everyday objects → procedures → social scripts. The agent fails, asks a structured question in the Rosie style ("What is the first step?", "What do I need?", "When is it done?"), and the teacher answers with minimal examples and near-miss negatives. Equivalence queries confirm rules. Spaced re-tests re-verify them. The key metric is **second-encounter learning**: a scenario the agent failed once is replayed with new wording after a delay and a restart.

- **Pros:** measures the claim that makes this project distinct, learning from conversation, instead of static knowledge. It fills the knowledge base with exactly what conversations need. It reuses curiosity, corrections and demonstrations. It is the right home for learning-progress curiosity per module.
- **Cons:** it only pays off once the representation (A) and understanding (B) can absorb what is taught, since teaching procedures to an agent with no procedure predicates fails. Teacher consistency is a risk: teachers must answer from a fixed ground truth, which is harder for open-ended how-to knowledge. It is also expensive in teacher calls.
- **Effort:** 1 iteration once A exists.
- **Proves it:** second-encounter success ≥ 80% on scenarios failed at first encounter, held-out wording, after a restart; false-fact rate ≤ 1% of stored facts; learning curves that keep rising across curriculum stages.
- **Disproves it:** if second-encounter success is under 50%, or if gains do not transfer to *related* unseen tasks (e.g. learned "change a bike tire", asked "patch an inner tube"), the learner is memorizing rather than generalizing. That would point to ILP or Popper (milestone 11) as the next investment.

#### Option D — Hybrid: an LLM paraphrases the symbolic content at output time

The symbolic system selects content and dialogue acts, and an LLM rewrites them into fluent text.

- **Pros:** the fastest path to high judge scores, probably +2 to +4 points within days. It is a useful **upper-bound ablation**: it tells you how much of the gap is wording versus content.
- **Cons and why it conflicts with the constraint:** the user requires that *all replies are produced by the symbolic system*. With a paraphraser, the reply text is produced by the LLM. That has three consequences. (1) Faithfulness becomes probabilistic: paraphrasers add hedges, facts and advice not in the knowledge base, which breaks DESIGN.md's rule that the agent "must only say what the agent actually believes". (2) "Why" answers no longer correspond word-for-word to proof traces. (3) Evaluation becomes misleading, because judges would score the LLM's fluency, not the agent's competence, and further progress on the symbolic side would become invisible. Re-parsing the paraphrase back to atoms reduces (1) but does not remove (3).
- **Compliant variant (D′):** use LLM paraphrases **offline, as teacher data**. Collect many realizations of the same content plan, induce templates, lexical choices and aggregation patterns from them, and keep those whose output parses back to the same atoms. At runtime only the symbolic realizer runs.
- **Effort:** D takes 0.25 iteration. D′ takes about 0.5 iteration, on top of B's realizer.
- **What it proves:** run D only in evaluation, as a ceiling. If D on top of the current content still scores ≤ 5/10, content and state (A) are the bottleneck, not wording.

#### Option E — Bulk procedural knowledge import via symbolic knowledge distillation

Translate ConceptNet procedural relations, ATOMIC `xNeed`/`xWant`/`xEffect`, and LLM-teacher-written wikiHow-style procedures into vault facts and rules offline. The translation goes through the constrained translator and is filtered by a critic made of cross-teacher agreement, knowledge base consistency and spot-checks. Everything enters at low confidence with a source tag, and promotion requires use and confirmation.

- **Pros:** fixes coverage quickly. Scenarios such as dinner and tires stop being blank. Follows the published ATOMIC-10x recipe.
- **Cons:** risks semantic drift (NELL), and a large unreviewed knowledge base erodes the "every fact is reviewable" property. It does not make the agent *learn from conversation*, which is the research point, so it should complement C rather than replace it.
- **Effort:** 0.5–1 iteration.
- **Proves it:** coverage of held-out help scenarios (at least one applicable procedure found) ≥ 90%, with the measured error rate of imported procedures ≤ 5% on a reviewed sample.
- **Disproves it:** if the import error rate is above 10%, or imported knowledge measurably degrades held-out knowledge tests.

#### Comparison

| Option | Targets | Satisfies constraint | Effort | Main risk |
|---|---|---|---|---|
| A Procedures + dialogue state | 18% no-answer, 20% fragments | yes | 1–1.5 | coverage of procedures |
| B Constrained translator + microplanner | 43% not understood, robotic wording | yes | 1 (+1–2 for GF) | wrong translations stored as facts |
| C Curriculum + second-encounter | learning from conversation (the thesis) | yes | 1 (after A) | teacher consistency |
| D LLM paraphrase at runtime | judge score only | **no** (D′ offline: yes) | 0.25 | hides real progress |
| E Bulk distillation import | coverage | yes | 0.5–1 | semantic drift |

**Recommendation:** do A first, because without a place to put procedures and state, nothing else can take effect. Then do B together with D′, then C. Use E inside A and C as the teachers' source material, not as a separate phase. Use D only as an evaluation ceiling.

---

### 3. Recommended roadmap: next three iterations

Same rules as DESIGN.md's build brief: work in order, do not start a milestone until the previous one passes, commit per milestone, run `cargo test` after every change, keep `PROGRESS.md` current. Milestone numbers continue from 11.

**Prerequisite for all three: evaluation v2.** This is a small step at the start of milestone 12.

- Fix 40 help scenarios (20 dev, 20 held-out) and 20 small-talk scenarios, each with a seed, a user goal and a ground-truth procedure or knowledge base.
- Haiku user-simulators play them for 12 turns.
- A judge labels **every agent turn** with one of {real answer, grounded acknowledgment, relevant question, understood-no-answer, not understood, fragment acknowledgment, false claim}.
- Report rates with 95% bootstrap confidence intervals. Add **pairwise A/B** against the previous commit's transcripts on the same seeds, with order randomized to control position bias. Keep the 1–10 score as a trend line.
- Add an **oracle-parser mode** where gold formal input is fed directly, so understanding and answering can be measured separately.
- Accept: one command produces all of the above and appends a row to `eval/history.csv`. Re-running the same commit twice gives category rates within ±5 percentage points.

#### Milestone 12 — Procedural knowledge and task-oriented dialogue state (Iteration 1)

1. **Procedural core.** Add to `vault/core/`: `task(T)`, `step(T, S)`, `before(S1, S2)` (reusing the core predicate), `needs(X, Thing)`, `takes_time(S, Minutes)`, `result_of(S, Fact)`, `method(T, Sub)`, `done_when(T, Fact)`, each with a plain-English, NSM-style definition note. Add stratified aggregates (`count`, `sum`, `min`, `max`) and non-generative arithmetic built-ins to the interpreter.
2. **Procedure vault format.** One file per task in `vault/procedures/`, with steps as a partial order, needs, durations and source. Hand-editable, lossless round-trip.
3. **Session information state.** A `session` module holds `active_task`, `at_step`, `qud(Q)` as a stack, frame slots `slot(Task, Name, Value)`, `grounded(Fact)` and `last_ref`. These are functional facts closed with `@until` by the existing mechanism. Dialogue moves (answer, give_next_step, ask_slot, ground, offer_help, social_followup, resume_task) are chosen by **Datalog rules over the state**, each move with a proof.
4. **Parser additions.** "how do I X", "what do I need for X", "how long does X take", "what's next / done / I did that", "I don't have X", "can I use Y instead", plus "plan / help me with X". The task name is resolved against `task/1` with synonyms.
5. **Social scripts.** Greeting, check-in, user-reported event → follow-up via `wants` or `needs` knowledge, and a closing script. All are stored as procedures with roles, not as code.
6. **Seed knowledge.** 30 everyday procedures (cooking, household, travel, repair, social planning), written by teachers from a fixed ground-truth file, imported at confidence 0.6, and reviewed by a second teacher.

*Accept:*
- Round-trip of 100 generated procedure files with no unintended diffs.
- A reasoner suite of at least 30 new cases: total duration by `sum`, union of needs including sub-tasks, "what's next" over a partial order, and the missing-needs set given `has` facts. Every answer prints its proof.
- On the 20 held-out help scenarios with the **oracle parser**: real-answer rate ≥ 60% and false claims = 0. With the normal parser: real-answer rate ≥ 40% (baseline 15%), understood-no-answer ≤ 10%.
- A scripted test interrupts a task with two small-talk turns, then "ok, what's next?". The agent resumes at the correct step.
- "Why do I need a jack?" is answered from a proof trace.
- All earlier tests still pass. Held-out knowledge score ≥ 97%.

#### Milestone 13 — Grammar-constrained translator and microplanned generation (Iteration 2)

1. **Constrained LLM translator** behind `Parser`. The grammar of the formal language is generated from the current vault lexicon (predicates, arities, known entities), with an escape hatch `new_pred(...)` routed to canonicalization. Take k = 3 samples and keep the consensus. Parse-time **round-trip check**: realize the parse with the agent's own NLG, then confirm it with the user for new important facts or re-parse it for self-consistency. Fall back to the pattern parser on disagreement.
2. **Lexicon canonicalization.** Map synonyms through a WordNet-like lexicon file in the vault. An unknown predicate becomes a curiosity goal ("Is 'own' the same as 'have'?") rather than a new predicate.
3. **Microplanner.** RST-lite document structuring (sequence, elaboration, cause, contrast); aggregation of same-subject and same-predicate atoms; referring expressions with the incremental algorithm; discourse markers; ellipsis of just-grounded content; an acknowledgment plus content combination in one turn; **deterministic failure wording** per input class (fixes the "rotating failure replies" complaint).
4. **Rust clause realizer**, SimpleNLG-like: tense, aspect, negation, modals, imperatives (needed for steps), questions, coordination and agreement. The faithfulness test (every claim parses back to its atom) stays mandatory.
5. **D′ offline variation mining.** A teacher paraphrases 300 content plans. Realization patterns are mined from them and kept only if their output parses back to the same atoms. Runtime stays symbolic.

*Accept:*
- On a new held-out set of 200 free-form sentences, written by a sub-agent that has not seen the code, with gold atoms: translator atom-level F1 ≥ 0.93 and stored-fact error rate after the round-trip check ≤ 2%.
- On held-out help and small-talk scenarios: not-understood ≤ 15% (baseline 43%) and fragment acknowledgment ≤ 5%.
- NLG test set of 200 content plans: 100% grammatical by the existing checks, 100% claims-in-KB, and distinct-2 ratio across a 12-turn conversation ≥ 0.6.
- Pairwise A/B on identical content (old NLG versus new, 3 judges, position randomized): new preferred in ≥ 65% of pairs.
- The D ceiling run, evaluation only, is recorded, and the gap between D and the symbolic system is reported.

#### Milestone 14 — Curriculum teacher protocol with measured second-encounter learning (Iteration 3)

1. **Curriculum graph** in `data/curriculum/`: stages (core concepts → objects and tools → procedures → social scripts → mixed help and small-talk), each with ground truth, prerequisites and held-out probe scenarios.
2. **Machine-teaching teacher.** It answers the agent's structured failure questions ("What is the first step of X?", "What do I need for X?", "When is X done?", "Is Y a kind of X?"), gives a minimal example and a near-miss negative, and answers **equivalence queries** ("My rule is: ... Right?") with a counterexample when wrong. Teachers answer only from ground truth, and a consistency checker rejects off-truth answers.
3. **Failure-triggered questions.** The curiosity drive turns understood-no-answer turns and planning dead ends into goals. Goals are ranked by expected answer-relevance and by **per-module learning progress**, as in IAC.
4. **Spaced re-verification.** Hypothesis rules and procedure facts get Leitner-style re-test intervals that grow with confidence. A failed re-test lowers confidence through the existing mechanism.
5. **Generalization.** Procedures with shared sub-steps are abstracted into reusable sub-tasks (DreamCoder-style chunking, kept simple: shared step sub-sequences across at least 3 procedures become a `method`). Optionally, a small Popper-style learner for multi-literal rules (milestone 11).

*Accept:*
- **Second-encounter test:** 30 held-out scenarios that fail at first encounter are replayed with new wording after a restart and at least one unrelated session in between. Real-answer rate on second encounter ≥ 80%.
- **Transfer test:** on 15 related-but-unseen tasks, real-answer or relevant-question rate ≥ 50%.
- False facts stored ≤ 1% (checked against ground truth). No held-out knowledge test regresses by more than 1 percentage point.
- The curiosity log shows learning progress per module, and the questions asked fall away on mastered modules.
- **Overall:** on the full held-out conversation suite, real-answer rate ≥ 60%, not understood ≤ 10%, and judge 1–10 mean ≥ 6.0 with the 95% confidence interval excluding the baseline's 4.3. A pairwise A/B against the milestone 12 build gives ≥ 70% preference.

---

### 4. Open research questions this prototype can answer

1. **How far can a fully symbolic reply path go?** With an LLM confined to input translation, what per-turn real-answer rate and judge score can everyday help and small-talk reach, and how large is the remaining gap to the D ceiling? Is the gap mostly wording or mostly content?
2. **Is a ~20-predicate prime-based core enough?** Does it need a fixed "molecule" layer (procedural core, social core)? Count how many learned predicates are definable from the core versus how many need new primitives, and track this as the knowledge base grows.
3. **Can the dialogue policy be learned like world knowledge?** If moves are Datalog rules over the information state, do teacher corrections ("you should have asked what tools I have") produce correct policy refinements with the same correction mechanism used for facts?
4. **How fast do symbolic agents learn from conversation?** Measure second-encounter learning versus the number of teacher turns, and compare structured failure-triggered questions (Rosie, L*) against free conversation. What is the empirical "teaching dimension" of an everyday procedure?
5. **Generalization versus memorization.** When does knowledge of one procedure transfer to related tasks, and is abstraction of shared sub-tasks or ILP (Popper) needed for it?
6. **Translator safety.** With grammar-constrained decoding, consensus sampling and round-trip confirmation, what fraction of stored facts are wrong, and which check catches which errors? Which predicate-canonicalization policy keeps the knowledge base from fragmenting?
7. **Semantic drift at scale.** With distilled imports plus conversational learning, how does the error rate of the knowledge base evolve over many sessions, and do confidence thresholds, spaced re-verification and contradiction-driven curiosity keep it bounded?
8. **Is small talk "just more scripts"?** Do social scripts plus `wants`/`needs` follow-ups on the same machinery as task procedures make small talk and assistance read as one skill to judges? Or does small talk need different mechanisms, such as topic models or persona?
9. **Evaluation validity.** How well do Haiku-class judges agree with each other and with a stronger judge on per-turn labels versus 1–10 scores? How many scenarios and turns are needed to detect a +1 point change reliably?
10. **Open world versus closed world in learning from dialogue.** Does PCA confidence (AMIE) or the current per-predicate choice produce fewer false rules from small, conversation-sized evidence?
11. **Cost of traceability.** As the knowledge base and session state grow, at what size does per-turn full recomputation need incremental maintenance (DRed or differential), and does compact provenance (Soufflé-style) keep "why" answers exact and fast?

---

## 8. Consolidated next steps

The recommendations from sections 4–7, merged and put in order. Each item says where its evidence is.

### Next milestone (12): procedural knowledge, dialogue state, honest metrics

1. **Evaluation v2 comes first** (sections 5.4, 6.4, 7.3). Label every agent turn as *not understood*, *understood but no answer*, *only acknowledged*, *small talk* or *real answer*, and show these in the training UI instead of "understood". Add the **second-encounter test**: repeat a taught scenario with teaching forbidden, and score whether the agent now helps. Compare every build with the previous one on the same scenarios and judges, with at least 8 judges.
2. **Procedures as facts** (sections 4.6, 6.2): what a task needs, its ordered steps, how long it takes, and its purpose, plus number ranges ("5–7 minutes"). Steps taught across several messages attach to the current task.
3. **Requests and task frames** (section 6.3): help, inform, accept and thank as dialogue acts. A help request opens a task frame, the agent asks for missing slots ("What ingredients do you have, and how much time?"), chooses among known options, and answers with the steps.
4. **Dialogue state per conversation** (sections 4.6, 6.3): the current task, step position, open questions, and the user's situation kept in the session rather than the vault. This stops echo replies and stops facts from one conversation leaking into another.
5. **Acceptance test: the user's dinner dialogue end to end**, after the recipe has been taught once, and a measurable drop in "not understood" on the training scenarios.

### Then (13): translator and generation

6. Restrict the translator to a closed set of predicates, plus a review queue for new ones. Add few-shot examples taken from reviewed cache entries, check that every atom is grounded in the message and retry once if not, put the spec in the cached system prompt, and use low thinking effort. Targets: under 10% not understood and under 2 s per turn (section 6.3).
7. Richer reply generation: a planning step before wording (what to say first, how to group it), natural acknowledgements, and steps with sequence words. Keep the faithfulness checks: every clause still comes from a believed atom (section 7.2).

### Then (14): curriculum and teacher protocol

8. Teachers teach in small fixed-frame chunks on a separate teaching channel and see what the agent stored. Formatting requests are banned. Scenarios repeat in a schedule, from simple to combined (section 6.3, 7.3).

### Engineering debt to pay along the way (section 4.5)

9. Incremental evaluation, or at least one model per message instead of 3–5. Index more than the first argument. Cap the learner's pattern search.
10. Sound time handling: give derived facts time intervals instead of mixing all of history.
11. Make parallel training safe: no model calls inside the global lock, a locked cache file, and the reviewer only touching facts its own conversation taught.
12. Tests for the trainer and pretraining code, and a current PROGRESS.md (it stops at the baseline).

### Housekeeping decisions for the user

- **Should `vault/` stay in git?** It was committed by accident during a chat and now holds training data, including personal facts from early chats.
- **The claude.ai trend dashboard** is a snapshot from before the chat training and the training app. Refresh it, or retire it in favour of the training UI.
- **Old pretraining tools:** `sla_pretrain` is superseded by `sla_trainer`. Keep it for scripted runs, or retire it.
- **Related-work references:** section 7 was written without web access; check its titles and years before quoting them outside this repository.
