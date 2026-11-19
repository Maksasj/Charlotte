# Sources of every number

| Macro | Value | Source |
|---|---|---|
| `\datasetTasks` | 98 | data/dataset3/*.md: file count |
| `\failExtra` | 6 | data/training/runs3/h3_r3_function/encounters.jsonl: first cause Extra (identity, wrong claim, missing step, need, order, else extra items or steps) |
| `\failIdentity` | 19 | data/training/runs3/h3_r3_function/encounters.jsonl: first cause Identity (identity, wrong claim, missing step, need, order, else extra items or steps) |
| `\failN` | 72 | data/training/runs3/h3_r3_function/encounters.jsonl: T and G second encounters not passed |
| `\failNeed` | 3 | data/training/runs3/h3_r3_function/encounters.jsonl: first cause Need (identity, wrong claim, missing step, need, order, else extra items or steps) |
| `\failOrder` | 0 | data/training/runs3/h3_r3_function/encounters.jsonl: first cause Order (identity, wrong claim, missing step, need, order, else extra items or steps) |
| `\failStep` | 32 | data/training/runs3/h3_r3_function/encounters.jsonl: first cause Step (identity, wrong claim, missing step, need, order, else extra items or steps) |
| `\failWrong` | 12 | data/training/runs3/h3_r3_function/encounters.jsonl: first cause Wrong (identity, wrong claim, missing step, need, order, else extra items or steps) |
| `\finalGapCases` | 232 | data/training/runs3/h3_r3_function/encounters.jsonl: variant encounters not passed |
| `\finalHonest` | 164 | data/training/runs3/h3_r3_function/encounters.jsonl: of those, answers with a gap line or a question |
| `\finalHonestRate` | 70.7 | data/training/runs3/h3_r3_function/encounters.jsonl: 100 * honest / cases |
| `\finalIdentity` | 75 | data/training/runs3/h3_r3_function/encounters.jsonl: T and G second encounters with the right task |
| `\finalIdentityN` | 94 | data/training/runs3/h3_r3_function/encounters.jsonl: T and G second encounters |
| `\finalN` | 432 | data/training/runs3/h3_r3_function/encounters.jsonl: encounters |
| `\finalWrong` | 77 | data/training/runs3/h3_r3_function/encounters.jsonl: encounters with a wrong claim |
| `\finalWrongRate` | 17.8 | data/training/runs3/h3_r3_function/encounters.jsonl: 100 * wrong / n |
| `\gtSteps` | 562 | data/curriculum3: step facts |
| `\gtTasks` | 105 | data/curriculum3: scenario files (incl. water_garden seed) |
| `\hThreeG` | 30 | data/heldout3/G: file count |
| `\hThreeS` | 60 | data/heldout3/S: file count |
| `\hThreeT` | 60 | data/heldout3/T: file count |
| `\haikuCalls` | 5085 | data/training/calls3/ledger.jsonl: line count (live calls) |
| `\pairAgentVsBThreeN` | 40 | data/training/runs3/h3_r3_function,b3_replay/encounters.jsonl: paired S_first encounters |
| `\pairAgentVsBThreeOnlyA` | 3 | data/training/runs3/h3_r3_function,b3_replay/encounters.jsonl: passed only in h3_r3_function |
| `\pairAgentVsBThreeOnlyB` | 0 | data/training/runs3/h3_r3_function,b3_replay/encounters.jsonl: passed only in b3_replay |
| `\pairAgentVsBThreeP` | 0.250 | data/training/runs3/h3_r3_function,b3_replay/encounters.jsonl: exact McNemar p (two-sided) |
| `\pairAgentVsBTwoN` | 60 | data/training/runs3/h3_r3_function,b2_h3/encounters.jsonl: paired T_second encounters |
| `\pairAgentVsBTwoOnlyA` | 7 | data/training/runs3/h3_r3_function,b2_h3/encounters.jsonl: passed only in h3_r3_function |
| `\pairAgentVsBTwoOnlyB` | 10 | data/training/runs3/h3_r3_function,b2_h3/encounters.jsonl: passed only in b2_h3 |
| `\pairAgentVsBTwoP` | 0.629 | data/training/runs3/h3_r3_function,b2_h3/encounters.jsonl: exact McNemar p (two-sided) |
| `\pairAgentVsBTwoSN` | 60 | data/training/runs3/h3_r3_function,b2_h3/encounters.jsonl: paired S_first encounters |
| `\pairAgentVsBTwoSOnlyA` | 4 | data/training/runs3/h3_r3_function,b2_h3/encounters.jsonl: passed only in h3_r3_function |
| `\pairAgentVsBTwoSOnlyB` | 9 | data/training/runs3/h3_r3_function,b2_h3/encounters.jsonl: passed only in b2_h3 |
| `\pairAgentVsBTwoSP` | 0.267 | data/training/runs3/h3_r3_function,b2_h3/encounters.jsonl: exact McNemar p (two-sided) |
| `\pairCOneVsCTwoN` | 60 | data/training/runs3/h3_r3_function,h3_r2_c2/encounters.jsonl: paired T_second encounters |
| `\pairCOneVsCTwoOnlyA` | 5 | data/training/runs3/h3_r3_function,h3_r2_c2/encounters.jsonl: passed only in h3_r3_function |
| `\pairCOneVsCTwoOnlyB` | 5 | data/training/runs3/h3_r3_function,h3_r2_c2/encounters.jsonl: passed only in h3_r2_c2 |
| `\pairCOneVsCTwoP` | 1.000 | data/training/runs3/h3_r3_function,h3_r2_c2/encounters.jsonl: exact McNemar p (two-sided) |
| `\pairFallbackVsNoneN` | 60 | data/training/runs3/h3_r3_function,h3_r3_nofallback/encounters.jsonl: paired T_second encounters |
| `\pairFallbackVsNoneOnlyA` | 4 | data/training/runs3/h3_r3_function,h3_r3_nofallback/encounters.jsonl: passed only in h3_r3_function |
| `\pairFallbackVsNoneOnlyB` | 2 | data/training/runs3/h3_r3_function,h3_r3_nofallback/encounters.jsonl: passed only in h3_r3_nofallback |
| `\pairFallbackVsNoneP` | 0.688 | data/training/runs3/h3_r3_function,h3_r3_nofallback/encounters.jsonl: exact McNemar p (two-sided) |
| `\pairFunctionVsBOneN` | 60 | data/training/runs3/h3_r3_function,h3_r3_b1/encounters.jsonl: paired S_first encounters |
| `\pairFunctionVsBOneOnlyA` | 2 | data/training/runs3/h3_r3_function,h3_r3_b1/encounters.jsonl: passed only in h3_r3_function |
| `\pairFunctionVsBOneOnlyB` | 0 | data/training/runs3/h3_r3_function,h3_r3_b1/encounters.jsonl: passed only in h3_r3_b1 |
| `\pairFunctionVsBOneP` | 0.500 | data/training/runs3/h3_r3_function,h3_r3_b1/encounters.jsonl: exact McNemar p (two-sided) |
| `\pairFunctionVsRulesN` | 60 | data/training/runs3/h3_r3_function,h3_r3_rules/encounters.jsonl: paired S_first encounters |
| `\pairFunctionVsRulesOnlyA` | 1 | data/training/runs3/h3_r3_function,h3_r3_rules/encounters.jsonl: passed only in h3_r3_function |
| `\pairFunctionVsRulesOnlyB` | 0 | data/training/runs3/h3_r3_function,h3_r3_rules/encounters.jsonl: passed only in h3_r3_rules |
| `\pairFunctionVsRulesP` | 1.000 | data/training/runs3/h3_r3_function,h3_r3_rules/encounters.jsonl: exact McNemar p (two-sided) |
| `\parserCoverage` | 10.9 | data/training/parser3/models/theta.json: 100 * coverage_at_95 |
| `\parserDevLines` | 101 | data/training/parser3/models/theta.json: dev_step_lines |
| `\runBThreeReplaySFirstHi` | 8.8 | data/training/runs3/b3_replay/summary.json: 100 * by_kind.S_first.ci95[1] |
| `\runBThreeReplaySFirstLo` | 0.0 | data/training/runs3/b3_replay/summary.json: 100 * by_kind.S_first.ci95[0] |
| `\runBThreeReplaySFirstN` | 40 | data/training/runs3/b3_replay/summary.json: by_kind.S_first.n |
| `\runBThreeReplaySFirstPass` | 0 | data/training/runs3/b3_replay/summary.json: by_kind.S_first.pass |
| `\runBThreeReplaySFirstRate` | 0.0 | data/training/runs3/b3_replay/summary.json: 100 * by_kind.S_first.rate |
| `\runBThreeReplaySFirstRecall` | 31.7 | data/training/runs3/b3_replay/summary.json: 100 * by_kind.S_first.mean_recall |
| `\runBThreeReplaySFirstWrong` | 15 | data/training/runs3/b3_replay/summary.json: by_kind.S_first.with_wrong_claim |
| `\runBTwoHThreeCalls` | 951 | data/training/runs3/b2_h3/summary.json: live_calls |
| `\runBTwoHThreeGSecondHi` | 46.2 | data/training/runs3/b2_h3/summary.json: 100 * by_kind.G_second.ci95[1] |
| `\runBTwoHThreeGSecondLo` | 16.8 | data/training/runs3/b2_h3/summary.json: 100 * by_kind.G_second.ci95[0] |
| `\runBTwoHThreeGSecondN` | 34 | data/training/runs3/b2_h3/summary.json: by_kind.G_second.n |
| `\runBTwoHThreeGSecondPass` | 10 | data/training/runs3/b2_h3/summary.json: by_kind.G_second.pass |
| `\runBTwoHThreeGSecondRate` | 29.4 | data/training/runs3/b2_h3/summary.json: 100 * by_kind.G_second.rate |
| `\runBTwoHThreeGSecondRecall` | 63.2 | data/training/runs3/b2_h3/summary.json: 100 * by_kind.G_second.mean_recall |
| `\runBTwoHThreeGSecondWrong` | 2 | data/training/runs3/b2_h3/summary.json: by_kind.G_second.with_wrong_claim |
| `\runBTwoHThreeGVariantHi` | 40.1 | data/training/runs3/b2_h3/summary.json: 100 * by_kind.G_variant.ci95[1] |
| `\runBTwoHThreeGVariantLo` | 18.6 | data/training/runs3/b2_h3/summary.json: 100 * by_kind.G_variant.ci95[0] |
| `\runBTwoHThreeGVariantN` | 64 | data/training/runs3/b2_h3/summary.json: by_kind.G_variant.n |
| `\runBTwoHThreeGVariantPass` | 18 | data/training/runs3/b2_h3/summary.json: by_kind.G_variant.pass |
| `\runBTwoHThreeGVariantRate` | 28.1 | data/training/runs3/b2_h3/summary.json: 100 * by_kind.G_variant.rate |
| `\runBTwoHThreeGVariantRecall` | 65.9 | data/training/runs3/b2_h3/summary.json: 100 * by_kind.G_variant.mean_recall |
| `\runBTwoHThreeGVariantWrong` | 9 | data/training/runs3/b2_h3/summary.json: by_kind.G_variant.with_wrong_claim |
| `\runBTwoHThreeSFirstHi` | 28.0 | data/training/runs3/b2_h3/summary.json: 100 * by_kind.S_first.ci95[1] |
| `\runBTwoHThreeSFirstLo` | 9.3 | data/training/runs3/b2_h3/summary.json: 100 * by_kind.S_first.ci95[0] |
| `\runBTwoHThreeSFirstN` | 60 | data/training/runs3/b2_h3/summary.json: by_kind.S_first.n |
| `\runBTwoHThreeSFirstPass` | 10 | data/training/runs3/b2_h3/summary.json: by_kind.S_first.pass |
| `\runBTwoHThreeSFirstRate` | 16.7 | data/training/runs3/b2_h3/summary.json: 100 * by_kind.S_first.rate |
| `\runBTwoHThreeSFirstRecall` | 66.3 | data/training/runs3/b2_h3/summary.json: 100 * by_kind.S_first.mean_recall |
| `\runBTwoHThreeSFirstWrong` | 18 | data/training/runs3/b2_h3/summary.json: by_kind.S_first.with_wrong_claim |
| `\runBTwoHThreeTSecondHi` | 42.5 | data/training/runs3/b2_h3/summary.json: 100 * by_kind.T_second.ci95[1] |
| `\runBTwoHThreeTSecondLo` | 19.9 | data/training/runs3/b2_h3/summary.json: 100 * by_kind.T_second.ci95[0] |
| `\runBTwoHThreeTSecondN` | 60 | data/training/runs3/b2_h3/summary.json: by_kind.T_second.n |
| `\runBTwoHThreeTSecondPass` | 18 | data/training/runs3/b2_h3/summary.json: by_kind.T_second.pass |
| `\runBTwoHThreeTSecondRate` | 30.0 | data/training/runs3/b2_h3/summary.json: 100 * by_kind.T_second.rate |
| `\runBTwoHThreeTSecondRecall` | 70.1 | data/training/runs3/b2_h3/summary.json: 100 * by_kind.T_second.mean_recall |
| `\runBTwoHThreeTSecondWrong` | 8 | data/training/runs3/b2_h3/summary.json: by_kind.T_second.with_wrong_claim |
| `\runBTwoHThreeTVariantHi` | 16.7 | data/training/runs3/b2_h3/summary.json: 100 * by_kind.T_variant.ci95[1] |
| `\runBTwoHThreeTVariantLo` | 5.8 | data/training/runs3/b2_h3/summary.json: 100 * by_kind.T_variant.ci95[0] |
| `\runBTwoHThreeTVariantN` | 120 | data/training/runs3/b2_h3/summary.json: by_kind.T_variant.n |
| `\runBTwoHThreeTVariantPass` | 12 | data/training/runs3/b2_h3/summary.json: by_kind.T_variant.pass |
| `\runBTwoHThreeTVariantRate` | 10.0 | data/training/runs3/b2_h3/summary.json: 100 * by_kind.T_variant.rate |
| `\runBTwoHThreeTVariantRecall` | 61.1 | data/training/runs3/b2_h3/summary.json: 100 * by_kind.T_variant.mean_recall |
| `\runBTwoHThreeTVariantWrong` | 19 | data/training/runs3/b2_h3/summary.json: by_kind.T_variant.with_wrong_claim |
| `\runCOneBothREightCalls` | 229 | data/training/runs3/c1_both_r8/summary.json: live_calls |
| `\runCOneBothREightFallback` | 497 | data/training/runs3/c1_both_r8/summary.json: ingest.fallback |
| `\runCOneBothREightLearned` | 38 | data/training/runs3/c1_both_r8/summary.json: ingest.learned |
| `\runCOneBothREightLines` | 535 | data/training/runs3/c1_both_r8/summary.json: ingest.lines |
| `\runCOneBothREightRetentionHi` | 53.8 | data/training/runs3/c1_both_r8/summary.json: 100 * by_kind.retention.ci95[1] |
| `\runCOneBothREightRetentionLo` | 31.5 | data/training/runs3/c1_both_r8/summary.json: 100 * by_kind.retention.ci95[0] |
| `\runCOneBothREightRetentionN` | 71 | data/training/runs3/c1_both_r8/summary.json: by_kind.retention.n |
| `\runCOneBothREightRetentionPass` | 30 | data/training/runs3/c1_both_r8/summary.json: by_kind.retention.pass |
| `\runCOneBothREightRetentionRate` | 42.3 | data/training/runs3/c1_both_r8/summary.json: 100 * by_kind.retention.rate |
| `\runCOneBothREightRetentionRecall` | 70.9 | data/training/runs3/c1_both_r8/summary.json: 100 * by_kind.retention.mean_recall |
| `\runCOneBothREightRetentionWrong` | 5 | data/training/runs3/c1_both_r8/summary.json: by_kind.retention.with_wrong_claim |
| `\runCOneBothREightSecondHi` | 62.0 | data/training/runs3/c1_both_r8/summary.json: 100 * by_kind.second.ci95[1] |
| `\runCOneBothREightSecondLo` | 39.3 | data/training/runs3/c1_both_r8/summary.json: 100 * by_kind.second.ci95[0] |
| `\runCOneBothREightSecondN` | 71 | data/training/runs3/c1_both_r8/summary.json: by_kind.second.n |
| `\runCOneBothREightSecondPass` | 36 | data/training/runs3/c1_both_r8/summary.json: by_kind.second.pass |
| `\runCOneBothREightSecondRate` | 50.7 | data/training/runs3/c1_both_r8/summary.json: 100 * by_kind.second.rate |
| `\runCOneBothREightSecondRecall` | 83.5 | data/training/runs3/c1_both_r8/summary.json: 100 * by_kind.second.mean_recall |
| `\runCOneBothREightSecondWrong` | 6 | data/training/runs3/c1_both_r8/summary.json: by_kind.second.with_wrong_claim |
| `\runCOneBothREightVariantHi` | 10.9 | data/training/runs3/c1_both_r8/summary.json: 100 * by_kind.variant.ci95[1] |
| `\runCOneBothREightVariantLo` | 0.4 | data/training/runs3/c1_both_r8/summary.json: 100 * by_kind.variant.ci95[0] |
| `\runCOneBothREightVariantN` | 48 | data/training/runs3/c1_both_r8/summary.json: by_kind.variant.n |
| `\runCOneBothREightVariantPass` | 1 | data/training/runs3/c1_both_r8/summary.json: by_kind.variant.pass |
| `\runCOneBothREightVariantRate` | 2.1 | data/training/runs3/c1_both_r8/summary.json: 100 * by_kind.variant.rate |
| `\runCOneBothREightVariantRecall` | 58.3 | data/training/runs3/c1_both_r8/summary.json: 100 * by_kind.variant.mean_recall |
| `\runCOneBothREightVariantRepeatHi` | 30.1 | data/training/runs3/c1_both_r8/summary.json: 100 * by_kind.variant_repeat.ci95[1] |
| `\runCOneBothREightVariantRepeatLo` | 8.9 | data/training/runs3/c1_both_r8/summary.json: 100 * by_kind.variant_repeat.ci95[0] |
| `\runCOneBothREightVariantRepeatN` | 47 | data/training/runs3/c1_both_r8/summary.json: by_kind.variant_repeat.n |
| `\runCOneBothREightVariantRepeatPass` | 8 | data/training/runs3/c1_both_r8/summary.json: by_kind.variant_repeat.pass |
| `\runCOneBothREightVariantRepeatRate` | 17.0 | data/training/runs3/c1_both_r8/summary.json: 100 * by_kind.variant_repeat.rate |
| `\runCOneBothREightVariantRepeatRecall` | 64.1 | data/training/runs3/c1_both_r8/summary.json: 100 * by_kind.variant_repeat.mean_recall |
| `\runCOneBothREightVariantRepeatWrong` | 21 | data/training/runs3/c1_both_r8/summary.json: by_kind.variant_repeat.with_wrong_claim |
| `\runCOneBothREightVariantWrong` | 11 | data/training/runs3/c1_both_r8/summary.json: by_kind.variant.with_wrong_claim |
| `\runCOneBothRFiveCalls` | 261 | data/training/runs3/c1_both_r5/summary.json: live_calls |
| `\runCOneBothRFiveFallback` | 364 | data/training/runs3/c1_both_r5/summary.json: ingest.fallback |
| `\runCOneBothRFiveLearned` | 171 | data/training/runs3/c1_both_r5/summary.json: ingest.learned |
| `\runCOneBothRFiveLines` | 535 | data/training/runs3/c1_both_r5/summary.json: ingest.lines |
| `\runCOneBothRFiveRetentionHi` | 43.9 | data/training/runs3/c1_both_r5/summary.json: 100 * by_kind.retention.ci95[1] |
| `\runCOneBothRFiveRetentionLo` | 22.7 | data/training/runs3/c1_both_r5/summary.json: 100 * by_kind.retention.ci95[0] |
| `\runCOneBothRFiveRetentionN` | 71 | data/training/runs3/c1_both_r5/summary.json: by_kind.retention.n |
| `\runCOneBothRFiveRetentionPass` | 23 | data/training/runs3/c1_both_r5/summary.json: by_kind.retention.pass |
| `\runCOneBothRFiveRetentionRate` | 32.4 | data/training/runs3/c1_both_r5/summary.json: 100 * by_kind.retention.rate |
| `\runCOneBothRFiveRetentionRecall` | 61.9 | data/training/runs3/c1_both_r5/summary.json: 100 * by_kind.retention.mean_recall |
| `\runCOneBothRFiveRetentionWrong` | 4 | data/training/runs3/c1_both_r5/summary.json: by_kind.retention.with_wrong_claim |
| `\runCOneBothRFiveSecondHi` | 52.5 | data/training/runs3/c1_both_r5/summary.json: 100 * by_kind.second.ci95[1] |
| `\runCOneBothRFiveSecondLo` | 30.2 | data/training/runs3/c1_both_r5/summary.json: 100 * by_kind.second.ci95[0] |
| `\runCOneBothRFiveSecondN` | 71 | data/training/runs3/c1_both_r5/summary.json: by_kind.second.n |
| `\runCOneBothRFiveSecondPass` | 29 | data/training/runs3/c1_both_r5/summary.json: by_kind.second.pass |
| `\runCOneBothRFiveSecondRate` | 40.8 | data/training/runs3/c1_both_r5/summary.json: 100 * by_kind.second.rate |
| `\runCOneBothRFiveSecondRecall` | 75.7 | data/training/runs3/c1_both_r5/summary.json: 100 * by_kind.second.mean_recall |
| `\runCOneBothRFiveSecondWrong` | 4 | data/training/runs3/c1_both_r5/summary.json: by_kind.second.with_wrong_claim |
| `\runCOneBothRFiveVariantHi` | 10.9 | data/training/runs3/c1_both_r5/summary.json: 100 * by_kind.variant.ci95[1] |
| `\runCOneBothRFiveVariantLo` | 0.4 | data/training/runs3/c1_both_r5/summary.json: 100 * by_kind.variant.ci95[0] |
| `\runCOneBothRFiveVariantN` | 48 | data/training/runs3/c1_both_r5/summary.json: by_kind.variant.n |
| `\runCOneBothRFiveVariantPass` | 1 | data/training/runs3/c1_both_r5/summary.json: by_kind.variant.pass |
| `\runCOneBothRFiveVariantRate` | 2.1 | data/training/runs3/c1_both_r5/summary.json: 100 * by_kind.variant.rate |
| `\runCOneBothRFiveVariantRecall` | 52.9 | data/training/runs3/c1_both_r5/summary.json: 100 * by_kind.variant.mean_recall |
| `\runCOneBothRFiveVariantRepeatHi` | 7.6 | data/training/runs3/c1_both_r5/summary.json: 100 * by_kind.variant_repeat.ci95[1] |
| `\runCOneBothRFiveVariantRepeatLo` | 0.0 | data/training/runs3/c1_both_r5/summary.json: 100 * by_kind.variant_repeat.ci95[0] |
| `\runCOneBothRFiveVariantRepeatN` | 47 | data/training/runs3/c1_both_r5/summary.json: by_kind.variant_repeat.n |
| `\runCOneBothRFiveVariantRepeatPass` | 0 | data/training/runs3/c1_both_r5/summary.json: by_kind.variant_repeat.pass |
| `\runCOneBothRFiveVariantRepeatRate` | 0.0 | data/training/runs3/c1_both_r5/summary.json: 100 * by_kind.variant_repeat.rate |
| `\runCOneBothRFiveVariantRepeatRecall` | 42.1 | data/training/runs3/c1_both_r5/summary.json: 100 * by_kind.variant_repeat.mean_recall |
| `\runCOneBothRFiveVariantRepeatWrong` | 27 | data/training/runs3/c1_both_r5/summary.json: by_kind.variant_repeat.with_wrong_claim |
| `\runCOneBothRFiveVariantWrong` | 20 | data/training/runs3/c1_both_r5/summary.json: by_kind.variant.with_wrong_claim |
| `\runCOneBothRFourCalls` | 520 | data/training/runs3/c1_both_r4/summary.json: live_calls |
| `\runCOneBothRFourFallback` | 364 | data/training/runs3/c1_both_r4/summary.json: ingest.fallback |
| `\runCOneBothRFourLearned` | 171 | data/training/runs3/c1_both_r4/summary.json: ingest.learned |
| `\runCOneBothRFourLines` | 535 | data/training/runs3/c1_both_r4/summary.json: ingest.lines |
| `\runCOneBothRFourRetentionHi` | 43.9 | data/training/runs3/c1_both_r4/summary.json: 100 * by_kind.retention.ci95[1] |
| `\runCOneBothRFourRetentionLo` | 22.7 | data/training/runs3/c1_both_r4/summary.json: 100 * by_kind.retention.ci95[0] |
| `\runCOneBothRFourRetentionN` | 71 | data/training/runs3/c1_both_r4/summary.json: by_kind.retention.n |
| `\runCOneBothRFourRetentionPass` | 23 | data/training/runs3/c1_both_r4/summary.json: by_kind.retention.pass |
| `\runCOneBothRFourRetentionRate` | 32.4 | data/training/runs3/c1_both_r4/summary.json: 100 * by_kind.retention.rate |
| `\runCOneBothRFourRetentionRecall` | 63.2 | data/training/runs3/c1_both_r4/summary.json: 100 * by_kind.retention.mean_recall |
| `\runCOneBothRFourRetentionWrong` | 5 | data/training/runs3/c1_both_r4/summary.json: by_kind.retention.with_wrong_claim |
| `\runCOneBothRFourSecondHi` | 52.5 | data/training/runs3/c1_both_r4/summary.json: 100 * by_kind.second.ci95[1] |
| `\runCOneBothRFourSecondLo` | 30.2 | data/training/runs3/c1_both_r4/summary.json: 100 * by_kind.second.ci95[0] |
| `\runCOneBothRFourSecondN` | 71 | data/training/runs3/c1_both_r4/summary.json: by_kind.second.n |
| `\runCOneBothRFourSecondPass` | 29 | data/training/runs3/c1_both_r4/summary.json: by_kind.second.pass |
| `\runCOneBothRFourSecondRate` | 40.8 | data/training/runs3/c1_both_r4/summary.json: 100 * by_kind.second.rate |
| `\runCOneBothRFourSecondRecall` | 75.8 | data/training/runs3/c1_both_r4/summary.json: 100 * by_kind.second.mean_recall |
| `\runCOneBothRFourSecondWrong` | 4 | data/training/runs3/c1_both_r4/summary.json: by_kind.second.with_wrong_claim |
| `\runCOneBothRFourVariantHi` | 7.4 | data/training/runs3/c1_both_r4/summary.json: 100 * by_kind.variant.ci95[1] |
| `\runCOneBothRFourVariantLo` | 0.0 | data/training/runs3/c1_both_r4/summary.json: 100 * by_kind.variant.ci95[0] |
| `\runCOneBothRFourVariantN` | 48 | data/training/runs3/c1_both_r4/summary.json: by_kind.variant.n |
| `\runCOneBothRFourVariantPass` | 0 | data/training/runs3/c1_both_r4/summary.json: by_kind.variant.pass |
| `\runCOneBothRFourVariantRate` | 0.0 | data/training/runs3/c1_both_r4/summary.json: 100 * by_kind.variant.rate |
| `\runCOneBothRFourVariantRecall` | 53.4 | data/training/runs3/c1_both_r4/summary.json: 100 * by_kind.variant.mean_recall |
| `\runCOneBothRFourVariantRepeatHi` | 7.4 | data/training/runs3/c1_both_r4/summary.json: 100 * by_kind.variant_repeat.ci95[1] |
| `\runCOneBothRFourVariantRepeatLo` | 0.0 | data/training/runs3/c1_both_r4/summary.json: 100 * by_kind.variant_repeat.ci95[0] |
| `\runCOneBothRFourVariantRepeatN` | 48 | data/training/runs3/c1_both_r4/summary.json: by_kind.variant_repeat.n |
| `\runCOneBothRFourVariantRepeatPass` | 0 | data/training/runs3/c1_both_r4/summary.json: by_kind.variant_repeat.pass |
| `\runCOneBothRFourVariantRepeatRate` | 0.0 | data/training/runs3/c1_both_r4/summary.json: 100 * by_kind.variant_repeat.rate |
| `\runCOneBothRFourVariantRepeatRecall` | 48.5 | data/training/runs3/c1_both_r4/summary.json: 100 * by_kind.variant_repeat.mean_recall |
| `\runCOneBothRFourVariantRepeatWrong` | 25 | data/training/runs3/c1_both_r4/summary.json: by_kind.variant_repeat.with_wrong_claim |
| `\runCOneBothRFourVariantWrong` | 24 | data/training/runs3/c1_both_r4/summary.json: by_kind.variant.with_wrong_claim |
| `\runCOneBothRSevenCalls` | 251 | data/training/runs3/c1_both_r7/summary.json: live_calls |
| `\runCOneBothRSevenFallback` | 364 | data/training/runs3/c1_both_r7/summary.json: ingest.fallback |
| `\runCOneBothRSevenLearned` | 171 | data/training/runs3/c1_both_r7/summary.json: ingest.learned |
| `\runCOneBothRSevenLines` | 535 | data/training/runs3/c1_both_r7/summary.json: ingest.lines |
| `\runCOneBothRSevenRetentionHi` | 51.1 | data/training/runs3/c1_both_r7/summary.json: 100 * by_kind.retention.ci95[1] |
| `\runCOneBothRSevenRetentionLo` | 28.9 | data/training/runs3/c1_both_r7/summary.json: 100 * by_kind.retention.ci95[0] |
| `\runCOneBothRSevenRetentionN` | 71 | data/training/runs3/c1_both_r7/summary.json: by_kind.retention.n |
| `\runCOneBothRSevenRetentionPass` | 28 | data/training/runs3/c1_both_r7/summary.json: by_kind.retention.pass |
| `\runCOneBothRSevenRetentionRate` | 39.4 | data/training/runs3/c1_both_r7/summary.json: 100 * by_kind.retention.rate |
| `\runCOneBothRSevenRetentionRecall` | 68.7 | data/training/runs3/c1_both_r7/summary.json: 100 * by_kind.retention.mean_recall |
| `\runCOneBothRSevenRetentionWrong` | 5 | data/training/runs3/c1_both_r7/summary.json: by_kind.retention.with_wrong_claim |
| `\runCOneBothRSevenSecondHi` | 63.3 | data/training/runs3/c1_both_r7/summary.json: 100 * by_kind.second.ci95[1] |
| `\runCOneBothRSevenSecondLo` | 40.7 | data/training/runs3/c1_both_r7/summary.json: 100 * by_kind.second.ci95[0] |
| `\runCOneBothRSevenSecondN` | 71 | data/training/runs3/c1_both_r7/summary.json: by_kind.second.n |
| `\runCOneBothRSevenSecondPass` | 37 | data/training/runs3/c1_both_r7/summary.json: by_kind.second.pass |
| `\runCOneBothRSevenSecondRate` | 52.1 | data/training/runs3/c1_both_r7/summary.json: 100 * by_kind.second.rate |
| `\runCOneBothRSevenSecondRecall` | 82.2 | data/training/runs3/c1_both_r7/summary.json: 100 * by_kind.second.mean_recall |
| `\runCOneBothRSevenSecondWrong` | 5 | data/training/runs3/c1_both_r7/summary.json: by_kind.second.with_wrong_claim |
| `\runCOneBothRSevenVariantHi` | 10.9 | data/training/runs3/c1_both_r7/summary.json: 100 * by_kind.variant.ci95[1] |
| `\runCOneBothRSevenVariantLo` | 0.4 | data/training/runs3/c1_both_r7/summary.json: 100 * by_kind.variant.ci95[0] |
| `\runCOneBothRSevenVariantN` | 48 | data/training/runs3/c1_both_r7/summary.json: by_kind.variant.n |
| `\runCOneBothRSevenVariantPass` | 1 | data/training/runs3/c1_both_r7/summary.json: by_kind.variant.pass |
| `\runCOneBothRSevenVariantRate` | 2.1 | data/training/runs3/c1_both_r7/summary.json: 100 * by_kind.variant.rate |
| `\runCOneBothRSevenVariantRecall` | 56.9 | data/training/runs3/c1_both_r7/summary.json: 100 * by_kind.variant.mean_recall |
| `\runCOneBothRSevenVariantRepeatHi` | 27.7 | data/training/runs3/c1_both_r7/summary.json: 100 * by_kind.variant_repeat.ci95[1] |
| `\runCOneBothRSevenVariantRepeatLo` | 7.4 | data/training/runs3/c1_both_r7/summary.json: 100 * by_kind.variant_repeat.ci95[0] |
| `\runCOneBothRSevenVariantRepeatN` | 47 | data/training/runs3/c1_both_r7/summary.json: by_kind.variant_repeat.n |
| `\runCOneBothRSevenVariantRepeatPass` | 7 | data/training/runs3/c1_both_r7/summary.json: by_kind.variant_repeat.pass |
| `\runCOneBothRSevenVariantRepeatRate` | 14.9 | data/training/runs3/c1_both_r7/summary.json: 100 * by_kind.variant_repeat.rate |
| `\runCOneBothRSevenVariantRepeatRecall` | 61.5 | data/training/runs3/c1_both_r7/summary.json: 100 * by_kind.variant_repeat.mean_recall |
| `\runCOneBothRSevenVariantRepeatWrong` | 20 | data/training/runs3/c1_both_r7/summary.json: by_kind.variant_repeat.with_wrong_claim |
| `\runCOneBothRSevenVariantWrong` | 12 | data/training/runs3/c1_both_r7/summary.json: by_kind.variant.with_wrong_claim |
| `\runCOneBothRSixCalls` | 128 | data/training/runs3/c1_both_r6/summary.json: live_calls |
| `\runCOneBothRSixFallback` | 364 | data/training/runs3/c1_both_r6/summary.json: ingest.fallback |
| `\runCOneBothRSixLearned` | 171 | data/training/runs3/c1_both_r6/summary.json: ingest.learned |
| `\runCOneBothRSixLines` | 535 | data/training/runs3/c1_both_r6/summary.json: ingest.lines |
| `\runCOneBothRSixRetentionHi` | 51.1 | data/training/runs3/c1_both_r6/summary.json: 100 * by_kind.retention.ci95[1] |
| `\runCOneBothRSixRetentionLo` | 28.9 | data/training/runs3/c1_both_r6/summary.json: 100 * by_kind.retention.ci95[0] |
| `\runCOneBothRSixRetentionN` | 71 | data/training/runs3/c1_both_r6/summary.json: by_kind.retention.n |
| `\runCOneBothRSixRetentionPass` | 28 | data/training/runs3/c1_both_r6/summary.json: by_kind.retention.pass |
| `\runCOneBothRSixRetentionRate` | 39.4 | data/training/runs3/c1_both_r6/summary.json: 100 * by_kind.retention.rate |
| `\runCOneBothRSixRetentionRecall` | 68.1 | data/training/runs3/c1_both_r6/summary.json: 100 * by_kind.retention.mean_recall |
| `\runCOneBothRSixRetentionWrong` | 5 | data/training/runs3/c1_both_r6/summary.json: by_kind.retention.with_wrong_claim |
| `\runCOneBothRSixSecondHi` | 63.3 | data/training/runs3/c1_both_r6/summary.json: 100 * by_kind.second.ci95[1] |
| `\runCOneBothRSixSecondLo` | 40.7 | data/training/runs3/c1_both_r6/summary.json: 100 * by_kind.second.ci95[0] |
| `\runCOneBothRSixSecondN` | 71 | data/training/runs3/c1_both_r6/summary.json: by_kind.second.n |
| `\runCOneBothRSixSecondPass` | 37 | data/training/runs3/c1_both_r6/summary.json: by_kind.second.pass |
| `\runCOneBothRSixSecondRate` | 52.1 | data/training/runs3/c1_both_r6/summary.json: 100 * by_kind.second.rate |
| `\runCOneBothRSixSecondRecall` | 82.2 | data/training/runs3/c1_both_r6/summary.json: 100 * by_kind.second.mean_recall |
| `\runCOneBothRSixSecondWrong` | 5 | data/training/runs3/c1_both_r6/summary.json: by_kind.second.with_wrong_claim |
| `\runCOneBothRSixVariantHi` | 7.4 | data/training/runs3/c1_both_r6/summary.json: 100 * by_kind.variant.ci95[1] |
| `\runCOneBothRSixVariantLo` | 0.0 | data/training/runs3/c1_both_r6/summary.json: 100 * by_kind.variant.ci95[0] |
| `\runCOneBothRSixVariantN` | 48 | data/training/runs3/c1_both_r6/summary.json: by_kind.variant.n |
| `\runCOneBothRSixVariantPass` | 0 | data/training/runs3/c1_both_r6/summary.json: by_kind.variant.pass |
| `\runCOneBothRSixVariantRate` | 0.0 | data/training/runs3/c1_both_r6/summary.json: 100 * by_kind.variant.rate |
| `\runCOneBothRSixVariantRecall` | 56.9 | data/training/runs3/c1_both_r6/summary.json: 100 * by_kind.variant.mean_recall |
| `\runCOneBothRSixVariantRepeatHi` | 7.4 | data/training/runs3/c1_both_r6/summary.json: 100 * by_kind.variant_repeat.ci95[1] |
| `\runCOneBothRSixVariantRepeatLo` | 0.0 | data/training/runs3/c1_both_r6/summary.json: 100 * by_kind.variant_repeat.ci95[0] |
| `\runCOneBothRSixVariantRepeatN` | 48 | data/training/runs3/c1_both_r6/summary.json: by_kind.variant_repeat.n |
| `\runCOneBothRSixVariantRepeatPass` | 0 | data/training/runs3/c1_both_r6/summary.json: by_kind.variant_repeat.pass |
| `\runCOneBothRSixVariantRepeatRate` | 0.0 | data/training/runs3/c1_both_r6/summary.json: 100 * by_kind.variant_repeat.rate |
| `\runCOneBothRSixVariantRepeatRecall` | 49.7 | data/training/runs3/c1_both_r6/summary.json: 100 * by_kind.variant_repeat.mean_recall |
| `\runCOneBothRSixVariantRepeatWrong` | 26 | data/training/runs3/c1_both_r6/summary.json: by_kind.variant_repeat.with_wrong_claim |
| `\runCOneBothRSixVariantWrong` | 20 | data/training/runs3/c1_both_r6/summary.json: by_kind.variant.with_wrong_claim |
| `\runCOneBothRTwoCalls` | 656 | data/training/runs3/c1_both_r2/summary.json: live_calls |
| `\runCOneBothRTwoFallback` | 16 | data/training/runs3/c1_both_r2/summary.json: ingest.fallback |
| `\runCOneBothRTwoLearned` | 519 | data/training/runs3/c1_both_r2/summary.json: ingest.learned |
| `\runCOneBothRTwoLines` | 535 | data/training/runs3/c1_both_r2/summary.json: ingest.lines |
| `\runCOneBothRTwoRetentionHi` | 30.4 | data/training/runs3/c1_both_r2/summary.json: 100 * by_kind.retention.ci95[1] |
| `\runCOneBothRTwoRetentionLo` | 12.1 | data/training/runs3/c1_both_r2/summary.json: 100 * by_kind.retention.ci95[0] |
| `\runCOneBothRTwoRetentionN` | 71 | data/training/runs3/c1_both_r2/summary.json: by_kind.retention.n |
| `\runCOneBothRTwoRetentionPass` | 14 | data/training/runs3/c1_both_r2/summary.json: by_kind.retention.pass |
| `\runCOneBothRTwoRetentionRate` | 19.7 | data/training/runs3/c1_both_r2/summary.json: 100 * by_kind.retention.rate |
| `\runCOneBothRTwoRetentionRecall` | 53.2 | data/training/runs3/c1_both_r2/summary.json: 100 * by_kind.retention.mean_recall |
| `\runCOneBothRTwoRetentionWrong` | 6 | data/training/runs3/c1_both_r2/summary.json: by_kind.retention.with_wrong_claim |
| `\runCOneBothRTwoSecondHi` | 38.1 | data/training/runs3/c1_both_r2/summary.json: 100 * by_kind.second.ci95[1] |
| `\runCOneBothRTwoSecondLo` | 17.9 | data/training/runs3/c1_both_r2/summary.json: 100 * by_kind.second.ci95[0] |
| `\runCOneBothRTwoSecondN` | 71 | data/training/runs3/c1_both_r2/summary.json: by_kind.second.n |
| `\runCOneBothRTwoSecondPass` | 19 | data/training/runs3/c1_both_r2/summary.json: by_kind.second.pass |
| `\runCOneBothRTwoSecondRate` | 26.8 | data/training/runs3/c1_both_r2/summary.json: 100 * by_kind.second.rate |
| `\runCOneBothRTwoSecondRecall` | 67.9 | data/training/runs3/c1_both_r2/summary.json: 100 * by_kind.second.mean_recall |
| `\runCOneBothRTwoSecondWrong` | 9 | data/training/runs3/c1_both_r2/summary.json: by_kind.second.with_wrong_claim |
| `\runCOneBothRTwoVariantHi` | 7.4 | data/training/runs3/c1_both_r2/summary.json: 100 * by_kind.variant.ci95[1] |
| `\runCOneBothRTwoVariantLo` | 0.0 | data/training/runs3/c1_both_r2/summary.json: 100 * by_kind.variant.ci95[0] |
| `\runCOneBothRTwoVariantN` | 48 | data/training/runs3/c1_both_r2/summary.json: by_kind.variant.n |
| `\runCOneBothRTwoVariantPass` | 0 | data/training/runs3/c1_both_r2/summary.json: by_kind.variant.pass |
| `\runCOneBothRTwoVariantRate` | 0.0 | data/training/runs3/c1_both_r2/summary.json: 100 * by_kind.variant.rate |
| `\runCOneBothRTwoVariantRecall` | 46.5 | data/training/runs3/c1_both_r2/summary.json: 100 * by_kind.variant.mean_recall |
| `\runCOneBothRTwoVariantRepeatHi` | 22.8 | data/training/runs3/c1_both_r2/summary.json: 100 * by_kind.variant_repeat.ci95[1] |
| `\runCOneBothRTwoVariantRepeatLo` | 0.0 | data/training/runs3/c1_both_r2/summary.json: 100 * by_kind.variant_repeat.ci95[0] |
| `\runCOneBothRTwoVariantRepeatN` | 13 | data/training/runs3/c1_both_r2/summary.json: by_kind.variant_repeat.n |
| `\runCOneBothRTwoVariantRepeatPass` | 0 | data/training/runs3/c1_both_r2/summary.json: by_kind.variant_repeat.pass |
| `\runCOneBothRTwoVariantRepeatRate` | 0.0 | data/training/runs3/c1_both_r2/summary.json: 100 * by_kind.variant_repeat.rate |
| `\runCOneBothRTwoVariantRepeatRecall` | 46.3 | data/training/runs3/c1_both_r2/summary.json: 100 * by_kind.variant_repeat.mean_recall |
| `\runCOneBothRTwoVariantRepeatWrong` | 7 | data/training/runs3/c1_both_r2/summary.json: by_kind.variant_repeat.with_wrong_claim |
| `\runCOneBothRTwoVariantWrong` | 19 | data/training/runs3/c1_both_r2/summary.json: by_kind.variant.with_wrong_claim |
| `\runCTwoBothROneCalls` | 226 | data/training/runs3/c2_both_r1/summary.json: live_calls |
| `\runCTwoBothROneFallback` | 460 | data/training/runs3/c2_both_r1/summary.json: ingest.fallback |
| `\runCTwoBothROneLearned` | 75 | data/training/runs3/c2_both_r1/summary.json: ingest.learned |
| `\runCTwoBothROneLines` | 535 | data/training/runs3/c2_both_r1/summary.json: ingest.lines |
| `\runCTwoBothROneRetentionHi` | 46.8 | data/training/runs3/c2_both_r1/summary.json: 100 * by_kind.retention.ci95[1] |
| `\runCTwoBothROneRetentionLo` | 25.1 | data/training/runs3/c2_both_r1/summary.json: 100 * by_kind.retention.ci95[0] |
| `\runCTwoBothROneRetentionN` | 71 | data/training/runs3/c2_both_r1/summary.json: by_kind.retention.n |
| `\runCTwoBothROneRetentionPass` | 25 | data/training/runs3/c2_both_r1/summary.json: by_kind.retention.pass |
| `\runCTwoBothROneRetentionRate` | 35.2 | data/training/runs3/c2_both_r1/summary.json: 100 * by_kind.retention.rate |
| `\runCTwoBothROneRetentionRecall` | 68.5 | data/training/runs3/c2_both_r1/summary.json: 100 * by_kind.retention.mean_recall |
| `\runCTwoBothROneRetentionWrong` | 7 | data/training/runs3/c2_both_r1/summary.json: by_kind.retention.with_wrong_claim |
| `\runCTwoBothROneSecondHi` | 52.5 | data/training/runs3/c2_both_r1/summary.json: 100 * by_kind.second.ci95[1] |
| `\runCTwoBothROneSecondLo` | 30.2 | data/training/runs3/c2_both_r1/summary.json: 100 * by_kind.second.ci95[0] |
| `\runCTwoBothROneSecondN` | 71 | data/training/runs3/c2_both_r1/summary.json: by_kind.second.n |
| `\runCTwoBothROneSecondPass` | 29 | data/training/runs3/c2_both_r1/summary.json: by_kind.second.pass |
| `\runCTwoBothROneSecondRate` | 40.8 | data/training/runs3/c2_both_r1/summary.json: 100 * by_kind.second.rate |
| `\runCTwoBothROneSecondRecall` | 80.1 | data/training/runs3/c2_both_r1/summary.json: 100 * by_kind.second.mean_recall |
| `\runCTwoBothROneSecondWrong` | 9 | data/training/runs3/c2_both_r1/summary.json: by_kind.second.with_wrong_claim |
| `\runCTwoBothROneVariantHi` | 10.9 | data/training/runs3/c2_both_r1/summary.json: 100 * by_kind.variant.ci95[1] |
| `\runCTwoBothROneVariantLo` | 0.4 | data/training/runs3/c2_both_r1/summary.json: 100 * by_kind.variant.ci95[0] |
| `\runCTwoBothROneVariantN` | 48 | data/training/runs3/c2_both_r1/summary.json: by_kind.variant.n |
| `\runCTwoBothROneVariantPass` | 1 | data/training/runs3/c2_both_r1/summary.json: by_kind.variant.pass |
| `\runCTwoBothROneVariantRate` | 2.1 | data/training/runs3/c2_both_r1/summary.json: 100 * by_kind.variant.rate |
| `\runCTwoBothROneVariantRecall` | 57.5 | data/training/runs3/c2_both_r1/summary.json: 100 * by_kind.variant.mean_recall |
| `\runCTwoBothROneVariantRepeatHi` | 25.2 | data/training/runs3/c2_both_r1/summary.json: 100 * by_kind.variant_repeat.ci95[1] |
| `\runCTwoBothROneVariantRepeatLo` | 6.0 | data/training/runs3/c2_both_r1/summary.json: 100 * by_kind.variant_repeat.ci95[0] |
| `\runCTwoBothROneVariantRepeatN` | 47 | data/training/runs3/c2_both_r1/summary.json: by_kind.variant_repeat.n |
| `\runCTwoBothROneVariantRepeatPass` | 6 | data/training/runs3/c2_both_r1/summary.json: by_kind.variant_repeat.pass |
| `\runCTwoBothROneVariantRepeatRate` | 12.8 | data/training/runs3/c2_both_r1/summary.json: 100 * by_kind.variant_repeat.rate |
| `\runCTwoBothROneVariantRepeatRecall` | 63.7 | data/training/runs3/c2_both_r1/summary.json: 100 * by_kind.variant_repeat.mean_recall |
| `\runCTwoBothROneVariantRepeatWrong` | 23 | data/training/runs3/c2_both_r1/summary.json: by_kind.variant_repeat.with_wrong_claim |
| `\runCTwoBothROneVariantWrong` | 12 | data/training/runs3/c2_both_r1/summary.json: by_kind.variant.with_wrong_claim |
| `\runDryDevCalls` | 0 | data/training/runs3/dry_dev/summary.json: live_calls |
| `\runDryDevFallback` | 0 | data/training/runs3/dry_dev/summary.json: ingest.fallback |
| `\runDryDevLearned` | 139 | data/training/runs3/dry_dev/summary.json: ingest.learned |
| `\runDryDevLines` | 139 | data/training/runs3/dry_dev/summary.json: ingest.lines |
| `\runDryDevRetentionHi` | 16.8 | data/training/runs3/dry_dev/summary.json: 100 * by_kind.retention.ci95[1] |
| `\runDryDevRetentionLo` | 0.0 | data/training/runs3/dry_dev/summary.json: 100 * by_kind.retention.ci95[0] |
| `\runDryDevRetentionN` | 19 | data/training/runs3/dry_dev/summary.json: by_kind.retention.n |
| `\runDryDevRetentionPass` | 0 | data/training/runs3/dry_dev/summary.json: by_kind.retention.pass |
| `\runDryDevRetentionRate` | 0.0 | data/training/runs3/dry_dev/summary.json: 100 * by_kind.retention.rate |
| `\runDryDevRetentionRecall` | 59.6 | data/training/runs3/dry_dev/summary.json: 100 * by_kind.retention.mean_recall |
| `\runDryDevRetentionWrong` | 2 | data/training/runs3/dry_dev/summary.json: by_kind.retention.with_wrong_claim |
| `\runDryDevSecondHi` | 16.8 | data/training/runs3/dry_dev/summary.json: 100 * by_kind.second.ci95[1] |
| `\runDryDevSecondLo` | 0.0 | data/training/runs3/dry_dev/summary.json: 100 * by_kind.second.ci95[0] |
| `\runDryDevSecondN` | 19 | data/training/runs3/dry_dev/summary.json: by_kind.second.n |
| `\runDryDevSecondPass` | 0 | data/training/runs3/dry_dev/summary.json: by_kind.second.pass |
| `\runDryDevSecondRate` | 0.0 | data/training/runs3/dry_dev/summary.json: 100 * by_kind.second.rate |
| `\runDryDevSecondRecall` | 61.5 | data/training/runs3/dry_dev/summary.json: 100 * by_kind.second.mean_recall |
| `\runDryDevSecondWrong` | 2 | data/training/runs3/dry_dev/summary.json: by_kind.second.with_wrong_claim |
| `\runDryDevTwoCalls` | 0 | data/training/runs3/dry_dev2/summary.json: live_calls |
| `\runDryDevTwoFallback` | 0 | data/training/runs3/dry_dev2/summary.json: ingest.fallback |
| `\runDryDevTwoLearned` | 139 | data/training/runs3/dry_dev2/summary.json: ingest.learned |
| `\runDryDevTwoLines` | 139 | data/training/runs3/dry_dev2/summary.json: ingest.lines |
| `\runDryDevTwoRetentionHi` | 48.8 | data/training/runs3/dry_dev2/summary.json: 100 * by_kind.retention.ci95[1] |
| `\runDryDevTwoRetentionLo` | 11.8 | data/training/runs3/dry_dev2/summary.json: 100 * by_kind.retention.ci95[0] |
| `\runDryDevTwoRetentionN` | 19 | data/training/runs3/dry_dev2/summary.json: by_kind.retention.n |
| `\runDryDevTwoRetentionPass` | 5 | data/training/runs3/dry_dev2/summary.json: by_kind.retention.pass |
| `\runDryDevTwoRetentionRate` | 26.3 | data/training/runs3/dry_dev2/summary.json: 100 * by_kind.retention.rate |
| `\runDryDevTwoRetentionRecall` | 67.6 | data/training/runs3/dry_dev2/summary.json: 100 * by_kind.retention.mean_recall |
| `\runDryDevTwoRetentionWrong` | 1 | data/training/runs3/dry_dev2/summary.json: by_kind.retention.with_wrong_claim |
| `\runDryDevTwoSecondHi` | 48.8 | data/training/runs3/dry_dev2/summary.json: 100 * by_kind.second.ci95[1] |
| `\runDryDevTwoSecondLo` | 11.8 | data/training/runs3/dry_dev2/summary.json: 100 * by_kind.second.ci95[0] |
| `\runDryDevTwoSecondN` | 19 | data/training/runs3/dry_dev2/summary.json: by_kind.second.n |
| `\runDryDevTwoSecondPass` | 5 | data/training/runs3/dry_dev2/summary.json: by_kind.second.pass |
| `\runDryDevTwoSecondRate` | 26.3 | data/training/runs3/dry_dev2/summary.json: 100 * by_kind.second.rate |
| `\runDryDevTwoSecondRecall` | 68.8 | data/training/runs3/dry_dev2/summary.json: 100 * by_kind.second.mean_recall |
| `\runDryDevTwoSecondWrong` | 1 | data/training/runs3/dry_dev2/summary.json: by_kind.second.with_wrong_claim |
| `\runDryDevTwoVariantHi` | 21.5 | data/training/runs3/dry_dev2/summary.json: 100 * by_kind.variant.ci95[1] |
| `\runDryDevTwoVariantLo` | 0.0 | data/training/runs3/dry_dev2/summary.json: 100 * by_kind.variant.ci95[0] |
| `\runDryDevTwoVariantN` | 14 | data/training/runs3/dry_dev2/summary.json: by_kind.variant.n |
| `\runDryDevTwoVariantPass` | 0 | data/training/runs3/dry_dev2/summary.json: by_kind.variant.pass |
| `\runDryDevTwoVariantRate` | 0.0 | data/training/runs3/dry_dev2/summary.json: 100 * by_kind.variant.rate |
| `\runDryDevTwoVariantRecall` | 53.1 | data/training/runs3/dry_dev2/summary.json: 100 * by_kind.variant.mean_recall |
| `\runDryDevTwoVariantWrong` | 8 | data/training/runs3/dry_dev2/summary.json: by_kind.variant.with_wrong_claim |
| `\runDryDevVariantHi` | 21.5 | data/training/runs3/dry_dev/summary.json: 100 * by_kind.variant.ci95[1] |
| `\runDryDevVariantLo` | 0.0 | data/training/runs3/dry_dev/summary.json: 100 * by_kind.variant.ci95[0] |
| `\runDryDevVariantN` | 14 | data/training/runs3/dry_dev/summary.json: by_kind.variant.n |
| `\runDryDevVariantPass` | 0 | data/training/runs3/dry_dev/summary.json: by_kind.variant.pass |
| `\runDryDevVariantRate` | 0.0 | data/training/runs3/dry_dev/summary.json: 100 * by_kind.variant.rate |
| `\runDryDevVariantRecall` | 50.0 | data/training/runs3/dry_dev/summary.json: 100 * by_kind.variant.mean_recall |
| `\runDryDevVariantWrong` | 9 | data/training/runs3/dry_dev/summary.json: by_kind.variant.with_wrong_claim |
| `\runHThreeROneCalls` | 1335 | data/training/runs3/h3_r1/summary.json: live_calls |
| `\runHThreeROneFallback` | 654 | data/training/runs3/h3_r1/summary.json: ingest.fallback |
| `\runHThreeROneGRetentionHi` | 40.0 | data/training/runs3/h3_r1/summary.json: 100 * by_kind.G_retention.ci95[1] |
| `\runHThreeROneGRetentionLo` | 12.4 | data/training/runs3/h3_r1/summary.json: 100 * by_kind.G_retention.ci95[0] |
| `\runHThreeROneGRetentionN` | 34 | data/training/runs3/h3_r1/summary.json: by_kind.G_retention.n |
| `\runHThreeROneGRetentionPass` | 8 | data/training/runs3/h3_r1/summary.json: by_kind.G_retention.pass |
| `\runHThreeROneGRetentionRate` | 23.5 | data/training/runs3/h3_r1/summary.json: 100 * by_kind.G_retention.rate |
| `\runHThreeROneGRetentionRecall` | 57.0 | data/training/runs3/h3_r1/summary.json: 100 * by_kind.G_retention.mean_recall |
| `\runHThreeROneGRetentionWrong` | 4 | data/training/runs3/h3_r1/summary.json: by_kind.G_retention.with_wrong_claim |
| `\runHThreeROneGSecondHi` | 36.8 | data/training/runs3/h3_r1/summary.json: 100 * by_kind.G_second.ci95[1] |
| `\runHThreeROneGSecondLo` | 10.3 | data/training/runs3/h3_r1/summary.json: 100 * by_kind.G_second.ci95[0] |
| `\runHThreeROneGSecondN` | 34 | data/training/runs3/h3_r1/summary.json: by_kind.G_second.n |
| `\runHThreeROneGSecondPass` | 7 | data/training/runs3/h3_r1/summary.json: by_kind.G_second.pass |
| `\runHThreeROneGSecondRate` | 20.6 | data/training/runs3/h3_r1/summary.json: 100 * by_kind.G_second.rate |
| `\runHThreeROneGSecondRecall` | 62.1 | data/training/runs3/h3_r1/summary.json: 100 * by_kind.G_second.mean_recall |
| `\runHThreeROneGSecondWrong` | 6 | data/training/runs3/h3_r1/summary.json: by_kind.G_second.with_wrong_claim |
| `\runHThreeROneGVariantHi` | 10.7 | data/training/runs3/h3_r1/summary.json: 100 * by_kind.G_variant.ci95[1] |
| `\runHThreeROneGVariantLo` | 0.9 | data/training/runs3/h3_r1/summary.json: 100 * by_kind.G_variant.ci95[0] |
| `\runHThreeROneGVariantN` | 64 | data/training/runs3/h3_r1/summary.json: by_kind.G_variant.n |
| `\runHThreeROneGVariantPass` | 2 | data/training/runs3/h3_r1/summary.json: by_kind.G_variant.pass |
| `\runHThreeROneGVariantRate` | 3.1 | data/training/runs3/h3_r1/summary.json: 100 * by_kind.G_variant.rate |
| `\runHThreeROneGVariantRecall` | 56.6 | data/training/runs3/h3_r1/summary.json: 100 * by_kind.G_variant.mean_recall |
| `\runHThreeROneGVariantWrong` | 22 | data/training/runs3/h3_r1/summary.json: by_kind.G_variant.with_wrong_claim |
| `\runHThreeROneLearned` | 49 | data/training/runs3/h3_r1/summary.json: ingest.learned |
| `\runHThreeROneLines` | 703 | data/training/runs3/h3_r1/summary.json: ingest.lines |
| `\runHThreeROneSFirstHi` | 18.1 | data/training/runs3/h3_r1/summary.json: 100 * by_kind.S_first.ci95[1] |
| `\runHThreeROneSFirstLo` | 3.6 | data/training/runs3/h3_r1/summary.json: 100 * by_kind.S_first.ci95[0] |
| `\runHThreeROneSFirstN` | 60 | data/training/runs3/h3_r1/summary.json: by_kind.S_first.n |
| `\runHThreeROneSFirstPass` | 5 | data/training/runs3/h3_r1/summary.json: by_kind.S_first.pass |
| `\runHThreeROneSFirstRate` | 8.3 | data/training/runs3/h3_r1/summary.json: 100 * by_kind.S_first.rate |
| `\runHThreeROneSFirstRecall` | 71.3 | data/training/runs3/h3_r1/summary.json: 100 * by_kind.S_first.mean_recall |
| `\runHThreeROneSFirstWrong` | 5 | data/training/runs3/h3_r1/summary.json: by_kind.S_first.with_wrong_claim |
| `\runHThreeROneStoredGN` | 34 | data/training/runs3/h3_r1/summary.json: stored_G.n |
| `\runHThreeROneStoredGPass` | 9 | data/training/runs3/h3_r1/summary.json: stored_G.pass |
| `\runHThreeROneStoredTN` | 60 | data/training/runs3/h3_r1/summary.json: stored_T.n |
| `\runHThreeROneStoredTPass` | 17 | data/training/runs3/h3_r1/summary.json: stored_T.pass |
| `\runHThreeROneTRetentionHi` | 28.0 | data/training/runs3/h3_r1/summary.json: 100 * by_kind.T_retention.ci95[1] |
| `\runHThreeROneTRetentionLo` | 9.3 | data/training/runs3/h3_r1/summary.json: 100 * by_kind.T_retention.ci95[0] |
| `\runHThreeROneTRetentionN` | 60 | data/training/runs3/h3_r1/summary.json: by_kind.T_retention.n |
| `\runHThreeROneTRetentionPass` | 10 | data/training/runs3/h3_r1/summary.json: by_kind.T_retention.pass |
| `\runHThreeROneTRetentionRate` | 16.7 | data/training/runs3/h3_r1/summary.json: 100 * by_kind.T_retention.rate |
| `\runHThreeROneTRetentionRecall` | 45.6 | data/training/runs3/h3_r1/summary.json: 100 * by_kind.T_retention.mean_recall |
| `\runHThreeROneTRetentionWrong` | 7 | data/training/runs3/h3_r1/summary.json: by_kind.T_retention.with_wrong_claim |
| `\runHThreeROneTSecondHi` | 37.2 | data/training/runs3/h3_r1/summary.json: 100 * by_kind.T_second.ci95[1] |
| `\runHThreeROneTSecondLo` | 15.8 | data/training/runs3/h3_r1/summary.json: 100 * by_kind.T_second.ci95[0] |
| `\runHThreeROneTSecondN` | 60 | data/training/runs3/h3_r1/summary.json: by_kind.T_second.n |
| `\runHThreeROneTSecondPass` | 15 | data/training/runs3/h3_r1/summary.json: by_kind.T_second.pass |
| `\runHThreeROneTSecondRate` | 25.0 | data/training/runs3/h3_r1/summary.json: 100 * by_kind.T_second.rate |
| `\runHThreeROneTSecondRecall` | 60.3 | data/training/runs3/h3_r1/summary.json: 100 * by_kind.T_second.mean_recall |
| `\runHThreeROneTSecondWrong` | 6 | data/training/runs3/h3_r1/summary.json: by_kind.T_second.with_wrong_claim |
| `\runHThreeROneTVariantHi` | 9.4 | data/training/runs3/h3_r1/summary.json: 100 * by_kind.T_variant.ci95[1] |
| `\runHThreeROneTVariantLo` | 1.8 | data/training/runs3/h3_r1/summary.json: 100 * by_kind.T_variant.ci95[0] |
| `\runHThreeROneTVariantN` | 120 | data/training/runs3/h3_r1/summary.json: by_kind.T_variant.n |
| `\runHThreeROneTVariantPass` | 5 | data/training/runs3/h3_r1/summary.json: by_kind.T_variant.pass |
| `\runHThreeROneTVariantRate` | 4.2 | data/training/runs3/h3_r1/summary.json: 100 * by_kind.T_variant.rate |
| `\runHThreeROneTVariantRecall` | 63.4 | data/training/runs3/h3_r1/summary.json: 100 * by_kind.T_variant.mean_recall |
| `\runHThreeROneTVariantWrong` | 27 | data/training/runs3/h3_r1/summary.json: by_kind.T_variant.with_wrong_claim |
| `\runHThreeRThreeBOneCalls` | 1 | data/training/runs3/h3_r3_b1/summary.json: live_calls |
| `\runHThreeRThreeBOneFallback` | 654 | data/training/runs3/h3_r3_b1/summary.json: ingest.fallback |
| `\runHThreeRThreeBOneGRetentionHi` | 40.0 | data/training/runs3/h3_r3_b1/summary.json: 100 * by_kind.G_retention.ci95[1] |
| `\runHThreeRThreeBOneGRetentionLo` | 12.4 | data/training/runs3/h3_r3_b1/summary.json: 100 * by_kind.G_retention.ci95[0] |
| `\runHThreeRThreeBOneGRetentionN` | 34 | data/training/runs3/h3_r3_b1/summary.json: by_kind.G_retention.n |
| `\runHThreeRThreeBOneGRetentionPass` | 8 | data/training/runs3/h3_r3_b1/summary.json: by_kind.G_retention.pass |
| `\runHThreeRThreeBOneGRetentionRate` | 23.5 | data/training/runs3/h3_r3_b1/summary.json: 100 * by_kind.G_retention.rate |
| `\runHThreeRThreeBOneGRetentionRecall` | 57.0 | data/training/runs3/h3_r3_b1/summary.json: 100 * by_kind.G_retention.mean_recall |
| `\runHThreeRThreeBOneGRetentionWrong` | 4 | data/training/runs3/h3_r3_b1/summary.json: by_kind.G_retention.with_wrong_claim |
| `\runHThreeRThreeBOneGSecondHi` | 36.8 | data/training/runs3/h3_r3_b1/summary.json: 100 * by_kind.G_second.ci95[1] |
| `\runHThreeRThreeBOneGSecondLo` | 10.3 | data/training/runs3/h3_r3_b1/summary.json: 100 * by_kind.G_second.ci95[0] |
| `\runHThreeRThreeBOneGSecondN` | 34 | data/training/runs3/h3_r3_b1/summary.json: by_kind.G_second.n |
| `\runHThreeRThreeBOneGSecondPass` | 7 | data/training/runs3/h3_r3_b1/summary.json: by_kind.G_second.pass |
| `\runHThreeRThreeBOneGSecondRate` | 20.6 | data/training/runs3/h3_r3_b1/summary.json: 100 * by_kind.G_second.rate |
| `\runHThreeRThreeBOneGSecondRecall` | 62.1 | data/training/runs3/h3_r3_b1/summary.json: 100 * by_kind.G_second.mean_recall |
| `\runHThreeRThreeBOneGSecondWrong` | 6 | data/training/runs3/h3_r3_b1/summary.json: by_kind.G_second.with_wrong_claim |
| `\runHThreeRThreeBOneGVariantHi` | 10.7 | data/training/runs3/h3_r3_b1/summary.json: 100 * by_kind.G_variant.ci95[1] |
| `\runHThreeRThreeBOneGVariantLo` | 0.9 | data/training/runs3/h3_r3_b1/summary.json: 100 * by_kind.G_variant.ci95[0] |
| `\runHThreeRThreeBOneGVariantN` | 64 | data/training/runs3/h3_r3_b1/summary.json: by_kind.G_variant.n |
| `\runHThreeRThreeBOneGVariantPass` | 2 | data/training/runs3/h3_r3_b1/summary.json: by_kind.G_variant.pass |
| `\runHThreeRThreeBOneGVariantRate` | 3.1 | data/training/runs3/h3_r3_b1/summary.json: 100 * by_kind.G_variant.rate |
| `\runHThreeRThreeBOneGVariantRecall` | 56.6 | data/training/runs3/h3_r3_b1/summary.json: 100 * by_kind.G_variant.mean_recall |
| `\runHThreeRThreeBOneGVariantWrong` | 36 | data/training/runs3/h3_r3_b1/summary.json: by_kind.G_variant.with_wrong_claim |
| `\runHThreeRThreeBOneLearned` | 49 | data/training/runs3/h3_r3_b1/summary.json: ingest.learned |
| `\runHThreeRThreeBOneLines` | 703 | data/training/runs3/h3_r3_b1/summary.json: ingest.lines |
| `\runHThreeRThreeBOneSFirstHi` | 13.7 | data/training/runs3/h3_r3_b1/summary.json: 100 * by_kind.S_first.ci95[1] |
| `\runHThreeRThreeBOneSFirstLo` | 1.7 | data/training/runs3/h3_r3_b1/summary.json: 100 * by_kind.S_first.ci95[0] |
| `\runHThreeRThreeBOneSFirstN` | 60 | data/training/runs3/h3_r3_b1/summary.json: by_kind.S_first.n |
| `\runHThreeRThreeBOneSFirstPass` | 3 | data/training/runs3/h3_r3_b1/summary.json: by_kind.S_first.pass |
| `\runHThreeRThreeBOneSFirstRate` | 5.0 | data/training/runs3/h3_r3_b1/summary.json: 100 * by_kind.S_first.rate |
| `\runHThreeRThreeBOneSFirstRecall` | 70.8 | data/training/runs3/h3_r3_b1/summary.json: 100 * by_kind.S_first.mean_recall |
| `\runHThreeRThreeBOneSFirstWrong` | 27 | data/training/runs3/h3_r3_b1/summary.json: by_kind.S_first.with_wrong_claim |
| `\runHThreeRThreeBOneStoredGN` | 34 | data/training/runs3/h3_r3_b1/summary.json: stored_G.n |
| `\runHThreeRThreeBOneStoredGPass` | 9 | data/training/runs3/h3_r3_b1/summary.json: stored_G.pass |
| `\runHThreeRThreeBOneStoredTN` | 60 | data/training/runs3/h3_r3_b1/summary.json: stored_T.n |
| `\runHThreeRThreeBOneStoredTPass` | 17 | data/training/runs3/h3_r3_b1/summary.json: stored_T.pass |
| `\runHThreeRThreeBOneTRetentionHi` | 28.0 | data/training/runs3/h3_r3_b1/summary.json: 100 * by_kind.T_retention.ci95[1] |
| `\runHThreeRThreeBOneTRetentionLo` | 9.3 | data/training/runs3/h3_r3_b1/summary.json: 100 * by_kind.T_retention.ci95[0] |
| `\runHThreeRThreeBOneTRetentionN` | 60 | data/training/runs3/h3_r3_b1/summary.json: by_kind.T_retention.n |
| `\runHThreeRThreeBOneTRetentionPass` | 10 | data/training/runs3/h3_r3_b1/summary.json: by_kind.T_retention.pass |
| `\runHThreeRThreeBOneTRetentionRate` | 16.7 | data/training/runs3/h3_r3_b1/summary.json: 100 * by_kind.T_retention.rate |
| `\runHThreeRThreeBOneTRetentionRecall` | 45.6 | data/training/runs3/h3_r3_b1/summary.json: 100 * by_kind.T_retention.mean_recall |
| `\runHThreeRThreeBOneTRetentionWrong` | 7 | data/training/runs3/h3_r3_b1/summary.json: by_kind.T_retention.with_wrong_claim |
| `\runHThreeRThreeBOneTSecondHi` | 37.2 | data/training/runs3/h3_r3_b1/summary.json: 100 * by_kind.T_second.ci95[1] |
| `\runHThreeRThreeBOneTSecondLo` | 15.8 | data/training/runs3/h3_r3_b1/summary.json: 100 * by_kind.T_second.ci95[0] |
| `\runHThreeRThreeBOneTSecondN` | 60 | data/training/runs3/h3_r3_b1/summary.json: by_kind.T_second.n |
| `\runHThreeRThreeBOneTSecondPass` | 15 | data/training/runs3/h3_r3_b1/summary.json: by_kind.T_second.pass |
| `\runHThreeRThreeBOneTSecondRate` | 25.0 | data/training/runs3/h3_r3_b1/summary.json: 100 * by_kind.T_second.rate |
| `\runHThreeRThreeBOneTSecondRecall` | 60.3 | data/training/runs3/h3_r3_b1/summary.json: 100 * by_kind.T_second.mean_recall |
| `\runHThreeRThreeBOneTSecondWrong` | 6 | data/training/runs3/h3_r3_b1/summary.json: by_kind.T_second.with_wrong_claim |
| `\runHThreeRThreeBOneTVariantHi` | 9.4 | data/training/runs3/h3_r3_b1/summary.json: 100 * by_kind.T_variant.ci95[1] |
| `\runHThreeRThreeBOneTVariantLo` | 1.8 | data/training/runs3/h3_r3_b1/summary.json: 100 * by_kind.T_variant.ci95[0] |
| `\runHThreeRThreeBOneTVariantN` | 120 | data/training/runs3/h3_r3_b1/summary.json: by_kind.T_variant.n |
| `\runHThreeRThreeBOneTVariantPass` | 5 | data/training/runs3/h3_r3_b1/summary.json: by_kind.T_variant.pass |
| `\runHThreeRThreeBOneTVariantRate` | 4.2 | data/training/runs3/h3_r3_b1/summary.json: 100 * by_kind.T_variant.rate |
| `\runHThreeRThreeBOneTVariantRecall` | 63.3 | data/training/runs3/h3_r3_b1/summary.json: 100 * by_kind.T_variant.mean_recall |
| `\runHThreeRThreeBOneTVariantWrong` | 61 | data/training/runs3/h3_r3_b1/summary.json: by_kind.T_variant.with_wrong_claim |
| `\runHThreeRThreeFunctionCalls` | 1 | data/training/runs3/h3_r3_function/summary.json: live_calls |
| `\runHThreeRThreeFunctionFallback` | 654 | data/training/runs3/h3_r3_function/summary.json: ingest.fallback |
| `\runHThreeRThreeFunctionGRetentionHi` | 40.0 | data/training/runs3/h3_r3_function/summary.json: 100 * by_kind.G_retention.ci95[1] |
| `\runHThreeRThreeFunctionGRetentionLo` | 12.4 | data/training/runs3/h3_r3_function/summary.json: 100 * by_kind.G_retention.ci95[0] |
| `\runHThreeRThreeFunctionGRetentionN` | 34 | data/training/runs3/h3_r3_function/summary.json: by_kind.G_retention.n |
| `\runHThreeRThreeFunctionGRetentionPass` | 8 | data/training/runs3/h3_r3_function/summary.json: by_kind.G_retention.pass |
| `\runHThreeRThreeFunctionGRetentionRate` | 23.5 | data/training/runs3/h3_r3_function/summary.json: 100 * by_kind.G_retention.rate |
| `\runHThreeRThreeFunctionGRetentionRecall` | 57.0 | data/training/runs3/h3_r3_function/summary.json: 100 * by_kind.G_retention.mean_recall |
| `\runHThreeRThreeFunctionGRetentionWrong` | 4 | data/training/runs3/h3_r3_function/summary.json: by_kind.G_retention.with_wrong_claim |
| `\runHThreeRThreeFunctionGSecondHi` | 36.8 | data/training/runs3/h3_r3_function/summary.json: 100 * by_kind.G_second.ci95[1] |
| `\runHThreeRThreeFunctionGSecondLo` | 10.3 | data/training/runs3/h3_r3_function/summary.json: 100 * by_kind.G_second.ci95[0] |
| `\runHThreeRThreeFunctionGSecondN` | 34 | data/training/runs3/h3_r3_function/summary.json: by_kind.G_second.n |
| `\runHThreeRThreeFunctionGSecondPass` | 7 | data/training/runs3/h3_r3_function/summary.json: by_kind.G_second.pass |
| `\runHThreeRThreeFunctionGSecondRate` | 20.6 | data/training/runs3/h3_r3_function/summary.json: 100 * by_kind.G_second.rate |
| `\runHThreeRThreeFunctionGSecondRecall` | 62.1 | data/training/runs3/h3_r3_function/summary.json: 100 * by_kind.G_second.mean_recall |
| `\runHThreeRThreeFunctionGSecondWrong` | 6 | data/training/runs3/h3_r3_function/summary.json: by_kind.G_second.with_wrong_claim |
| `\runHThreeRThreeFunctionGVariantHi` | 10.7 | data/training/runs3/h3_r3_function/summary.json: 100 * by_kind.G_variant.ci95[1] |
| `\runHThreeRThreeFunctionGVariantLo` | 0.9 | data/training/runs3/h3_r3_function/summary.json: 100 * by_kind.G_variant.ci95[0] |
| `\runHThreeRThreeFunctionGVariantN` | 64 | data/training/runs3/h3_r3_function/summary.json: by_kind.G_variant.n |
| `\runHThreeRThreeFunctionGVariantPass` | 2 | data/training/runs3/h3_r3_function/summary.json: by_kind.G_variant.pass |
| `\runHThreeRThreeFunctionGVariantRate` | 3.1 | data/training/runs3/h3_r3_function/summary.json: 100 * by_kind.G_variant.rate |
| `\runHThreeRThreeFunctionGVariantRecall` | 56.6 | data/training/runs3/h3_r3_function/summary.json: 100 * by_kind.G_variant.mean_recall |
| `\runHThreeRThreeFunctionGVariantWrong` | 22 | data/training/runs3/h3_r3_function/summary.json: by_kind.G_variant.with_wrong_claim |
| `\runHThreeRThreeFunctionLearned` | 49 | data/training/runs3/h3_r3_function/summary.json: ingest.learned |
| `\runHThreeRThreeFunctionLines` | 703 | data/training/runs3/h3_r3_function/summary.json: ingest.lines |
| `\runHThreeRThreeFunctionSFirstHi` | 18.1 | data/training/runs3/h3_r3_function/summary.json: 100 * by_kind.S_first.ci95[1] |
| `\runHThreeRThreeFunctionSFirstLo` | 3.6 | data/training/runs3/h3_r3_function/summary.json: 100 * by_kind.S_first.ci95[0] |
| `\runHThreeRThreeFunctionSFirstN` | 60 | data/training/runs3/h3_r3_function/summary.json: by_kind.S_first.n |
| `\runHThreeRThreeFunctionSFirstPass` | 5 | data/training/runs3/h3_r3_function/summary.json: by_kind.S_first.pass |
| `\runHThreeRThreeFunctionSFirstRate` | 8.3 | data/training/runs3/h3_r3_function/summary.json: 100 * by_kind.S_first.rate |
| `\runHThreeRThreeFunctionSFirstRecall` | 71.3 | data/training/runs3/h3_r3_function/summary.json: 100 * by_kind.S_first.mean_recall |
| `\runHThreeRThreeFunctionSFirstWrong` | 5 | data/training/runs3/h3_r3_function/summary.json: by_kind.S_first.with_wrong_claim |
| `\runHThreeRThreeFunctionStoredGN` | 34 | data/training/runs3/h3_r3_function/summary.json: stored_G.n |
| `\runHThreeRThreeFunctionStoredGPass` | 9 | data/training/runs3/h3_r3_function/summary.json: stored_G.pass |
| `\runHThreeRThreeFunctionStoredTN` | 60 | data/training/runs3/h3_r3_function/summary.json: stored_T.n |
| `\runHThreeRThreeFunctionStoredTPass` | 17 | data/training/runs3/h3_r3_function/summary.json: stored_T.pass |
| `\runHThreeRThreeFunctionTRetentionHi` | 28.0 | data/training/runs3/h3_r3_function/summary.json: 100 * by_kind.T_retention.ci95[1] |
| `\runHThreeRThreeFunctionTRetentionLo` | 9.3 | data/training/runs3/h3_r3_function/summary.json: 100 * by_kind.T_retention.ci95[0] |
| `\runHThreeRThreeFunctionTRetentionN` | 60 | data/training/runs3/h3_r3_function/summary.json: by_kind.T_retention.n |
| `\runHThreeRThreeFunctionTRetentionPass` | 10 | data/training/runs3/h3_r3_function/summary.json: by_kind.T_retention.pass |
| `\runHThreeRThreeFunctionTRetentionRate` | 16.7 | data/training/runs3/h3_r3_function/summary.json: 100 * by_kind.T_retention.rate |
| `\runHThreeRThreeFunctionTRetentionRecall` | 45.6 | data/training/runs3/h3_r3_function/summary.json: 100 * by_kind.T_retention.mean_recall |
| `\runHThreeRThreeFunctionTRetentionWrong` | 7 | data/training/runs3/h3_r3_function/summary.json: by_kind.T_retention.with_wrong_claim |
| `\runHThreeRThreeFunctionTSecondHi` | 37.2 | data/training/runs3/h3_r3_function/summary.json: 100 * by_kind.T_second.ci95[1] |
| `\runHThreeRThreeFunctionTSecondLo` | 15.8 | data/training/runs3/h3_r3_function/summary.json: 100 * by_kind.T_second.ci95[0] |
| `\runHThreeRThreeFunctionTSecondN` | 60 | data/training/runs3/h3_r3_function/summary.json: by_kind.T_second.n |
| `\runHThreeRThreeFunctionTSecondPass` | 15 | data/training/runs3/h3_r3_function/summary.json: by_kind.T_second.pass |
| `\runHThreeRThreeFunctionTSecondRate` | 25.0 | data/training/runs3/h3_r3_function/summary.json: 100 * by_kind.T_second.rate |
| `\runHThreeRThreeFunctionTSecondRecall` | 60.3 | data/training/runs3/h3_r3_function/summary.json: 100 * by_kind.T_second.mean_recall |
| `\runHThreeRThreeFunctionTSecondWrong` | 6 | data/training/runs3/h3_r3_function/summary.json: by_kind.T_second.with_wrong_claim |
| `\runHThreeRThreeFunctionTVariantHi` | 9.4 | data/training/runs3/h3_r3_function/summary.json: 100 * by_kind.T_variant.ci95[1] |
| `\runHThreeRThreeFunctionTVariantLo` | 1.8 | data/training/runs3/h3_r3_function/summary.json: 100 * by_kind.T_variant.ci95[0] |
| `\runHThreeRThreeFunctionTVariantN` | 120 | data/training/runs3/h3_r3_function/summary.json: by_kind.T_variant.n |
| `\runHThreeRThreeFunctionTVariantPass` | 5 | data/training/runs3/h3_r3_function/summary.json: by_kind.T_variant.pass |
| `\runHThreeRThreeFunctionTVariantRate` | 4.2 | data/training/runs3/h3_r3_function/summary.json: 100 * by_kind.T_variant.rate |
| `\runHThreeRThreeFunctionTVariantRecall` | 63.4 | data/training/runs3/h3_r3_function/summary.json: 100 * by_kind.T_variant.mean_recall |
| `\runHThreeRThreeFunctionTVariantWrong` | 27 | data/training/runs3/h3_r3_function/summary.json: by_kind.T_variant.with_wrong_claim |
| `\runHThreeRThreeNofallbackCalls` | 1 | data/training/runs3/h3_r3_nofallback/summary.json: live_calls |
| `\runHThreeRThreeNofallbackFallback` | 0 | data/training/runs3/h3_r3_nofallback/summary.json: ingest.fallback |
| `\runHThreeRThreeNofallbackGRetentionHi` | 33.5 | data/training/runs3/h3_r3_nofallback/summary.json: 100 * by_kind.G_retention.ci95[1] |
| `\runHThreeRThreeNofallbackGRetentionLo` | 8.3 | data/training/runs3/h3_r3_nofallback/summary.json: 100 * by_kind.G_retention.ci95[0] |
| `\runHThreeRThreeNofallbackGRetentionN` | 34 | data/training/runs3/h3_r3_nofallback/summary.json: by_kind.G_retention.n |
| `\runHThreeRThreeNofallbackGRetentionPass` | 6 | data/training/runs3/h3_r3_nofallback/summary.json: by_kind.G_retention.pass |
| `\runHThreeRThreeNofallbackGRetentionRate` | 17.6 | data/training/runs3/h3_r3_nofallback/summary.json: 100 * by_kind.G_retention.rate |
| `\runHThreeRThreeNofallbackGRetentionRecall` | 48.1 | data/training/runs3/h3_r3_nofallback/summary.json: 100 * by_kind.G_retention.mean_recall |
| `\runHThreeRThreeNofallbackGRetentionWrong` | 3 | data/training/runs3/h3_r3_nofallback/summary.json: by_kind.G_retention.with_wrong_claim |
| `\runHThreeRThreeNofallbackGSecondHi` | 26.6 | data/training/runs3/h3_r3_nofallback/summary.json: 100 * by_kind.G_second.ci95[1] |
| `\runHThreeRThreeNofallbackGSecondLo` | 4.7 | data/training/runs3/h3_r3_nofallback/summary.json: 100 * by_kind.G_second.ci95[0] |
| `\runHThreeRThreeNofallbackGSecondN` | 34 | data/training/runs3/h3_r3_nofallback/summary.json: by_kind.G_second.n |
| `\runHThreeRThreeNofallbackGSecondPass` | 4 | data/training/runs3/h3_r3_nofallback/summary.json: by_kind.G_second.pass |
| `\runHThreeRThreeNofallbackGSecondRate` | 11.8 | data/training/runs3/h3_r3_nofallback/summary.json: 100 * by_kind.G_second.rate |
| `\runHThreeRThreeNofallbackGSecondRecall` | 48.5 | data/training/runs3/h3_r3_nofallback/summary.json: 100 * by_kind.G_second.mean_recall |
| `\runHThreeRThreeNofallbackGSecondWrong` | 4 | data/training/runs3/h3_r3_nofallback/summary.json: by_kind.G_second.with_wrong_claim |
| `\runHThreeRThreeNofallbackGVariantHi` | 10.7 | data/training/runs3/h3_r3_nofallback/summary.json: 100 * by_kind.G_variant.ci95[1] |
| `\runHThreeRThreeNofallbackGVariantLo` | 0.9 | data/training/runs3/h3_r3_nofallback/summary.json: 100 * by_kind.G_variant.ci95[0] |
| `\runHThreeRThreeNofallbackGVariantN` | 64 | data/training/runs3/h3_r3_nofallback/summary.json: by_kind.G_variant.n |
| `\runHThreeRThreeNofallbackGVariantPass` | 2 | data/training/runs3/h3_r3_nofallback/summary.json: by_kind.G_variant.pass |
| `\runHThreeRThreeNofallbackGVariantRate` | 3.1 | data/training/runs3/h3_r3_nofallback/summary.json: 100 * by_kind.G_variant.rate |
| `\runHThreeRThreeNofallbackGVariantRecall` | 45.0 | data/training/runs3/h3_r3_nofallback/summary.json: 100 * by_kind.G_variant.mean_recall |
| `\runHThreeRThreeNofallbackGVariantWrong` | 31 | data/training/runs3/h3_r3_nofallback/summary.json: by_kind.G_variant.with_wrong_claim |
| `\runHThreeRThreeNofallbackLearned` | 703 | data/training/runs3/h3_r3_nofallback/summary.json: ingest.learned |
| `\runHThreeRThreeNofallbackLines` | 703 | data/training/runs3/h3_r3_nofallback/summary.json: ingest.lines |
| `\runHThreeRThreeNofallbackSFirstHi` | 13.7 | data/training/runs3/h3_r3_nofallback/summary.json: 100 * by_kind.S_first.ci95[1] |
| `\runHThreeRThreeNofallbackSFirstLo` | 1.7 | data/training/runs3/h3_r3_nofallback/summary.json: 100 * by_kind.S_first.ci95[0] |
| `\runHThreeRThreeNofallbackSFirstN` | 60 | data/training/runs3/h3_r3_nofallback/summary.json: by_kind.S_first.n |
| `\runHThreeRThreeNofallbackSFirstPass` | 3 | data/training/runs3/h3_r3_nofallback/summary.json: by_kind.S_first.pass |
| `\runHThreeRThreeNofallbackSFirstRate` | 5.0 | data/training/runs3/h3_r3_nofallback/summary.json: 100 * by_kind.S_first.rate |
| `\runHThreeRThreeNofallbackSFirstRecall` | 70.8 | data/training/runs3/h3_r3_nofallback/summary.json: 100 * by_kind.S_first.mean_recall |
| `\runHThreeRThreeNofallbackSFirstWrong` | 27 | data/training/runs3/h3_r3_nofallback/summary.json: by_kind.S_first.with_wrong_claim |
| `\runHThreeRThreeNofallbackStoredGN` | 34 | data/training/runs3/h3_r3_nofallback/summary.json: stored_G.n |
| `\runHThreeRThreeNofallbackStoredGPass` | 7 | data/training/runs3/h3_r3_nofallback/summary.json: stored_G.pass |
| `\runHThreeRThreeNofallbackStoredTN` | 60 | data/training/runs3/h3_r3_nofallback/summary.json: stored_T.n |
| `\runHThreeRThreeNofallbackStoredTPass` | 15 | data/training/runs3/h3_r3_nofallback/summary.json: stored_T.pass |
| `\runHThreeRThreeNofallbackTRetentionHi` | 24.2 | data/training/runs3/h3_r3_nofallback/summary.json: 100 * by_kind.T_retention.ci95[1] |
| `\runHThreeRThreeNofallbackTRetentionLo` | 6.9 | data/training/runs3/h3_r3_nofallback/summary.json: 100 * by_kind.T_retention.ci95[0] |
| `\runHThreeRThreeNofallbackTRetentionN` | 60 | data/training/runs3/h3_r3_nofallback/summary.json: by_kind.T_retention.n |
| `\runHThreeRThreeNofallbackTRetentionPass` | 8 | data/training/runs3/h3_r3_nofallback/summary.json: by_kind.T_retention.pass |
| `\runHThreeRThreeNofallbackTRetentionRate` | 13.3 | data/training/runs3/h3_r3_nofallback/summary.json: 100 * by_kind.T_retention.rate |
| `\runHThreeRThreeNofallbackTRetentionRecall` | 37.6 | data/training/runs3/h3_r3_nofallback/summary.json: 100 * by_kind.T_retention.mean_recall |
| `\runHThreeRThreeNofallbackTRetentionWrong` | 2 | data/training/runs3/h3_r3_nofallback/summary.json: by_kind.T_retention.with_wrong_claim |
| `\runHThreeRThreeNofallbackTSecondHi` | 33.6 | data/training/runs3/h3_r3_nofallback/summary.json: 100 * by_kind.T_second.ci95[1] |
| `\runHThreeRThreeNofallbackTSecondLo` | 13.1 | data/training/runs3/h3_r3_nofallback/summary.json: 100 * by_kind.T_second.ci95[0] |
| `\runHThreeRThreeNofallbackTSecondN` | 60 | data/training/runs3/h3_r3_nofallback/summary.json: by_kind.T_second.n |
| `\runHThreeRThreeNofallbackTSecondPass` | 13 | data/training/runs3/h3_r3_nofallback/summary.json: by_kind.T_second.pass |
| `\runHThreeRThreeNofallbackTSecondRate` | 21.7 | data/training/runs3/h3_r3_nofallback/summary.json: 100 * by_kind.T_second.rate |
| `\runHThreeRThreeNofallbackTSecondRecall` | 50.5 | data/training/runs3/h3_r3_nofallback/summary.json: 100 * by_kind.T_second.mean_recall |
| `\runHThreeRThreeNofallbackTSecondWrong` | 2 | data/training/runs3/h3_r3_nofallback/summary.json: by_kind.T_second.with_wrong_claim |
| `\runHThreeRThreeNofallbackTVariantHi` | 7.1 | data/training/runs3/h3_r3_nofallback/summary.json: 100 * by_kind.T_variant.ci95[1] |
| `\runHThreeRThreeNofallbackTVariantLo` | 0.9 | data/training/runs3/h3_r3_nofallback/summary.json: 100 * by_kind.T_variant.ci95[0] |
| `\runHThreeRThreeNofallbackTVariantN` | 120 | data/training/runs3/h3_r3_nofallback/summary.json: by_kind.T_variant.n |
| `\runHThreeRThreeNofallbackTVariantPass` | 3 | data/training/runs3/h3_r3_nofallback/summary.json: by_kind.T_variant.pass |
| `\runHThreeRThreeNofallbackTVariantRate` | 2.5 | data/training/runs3/h3_r3_nofallback/summary.json: 100 * by_kind.T_variant.rate |
| `\runHThreeRThreeNofallbackTVariantRecall` | 52.2 | data/training/runs3/h3_r3_nofallback/summary.json: 100 * by_kind.T_variant.mean_recall |
| `\runHThreeRThreeNofallbackTVariantWrong` | 50 | data/training/runs3/h3_r3_nofallback/summary.json: by_kind.T_variant.with_wrong_claim |
| `\runHThreeRThreeRulesCalls` | 1 | data/training/runs3/h3_r3_rules/summary.json: live_calls |
| `\runHThreeRThreeRulesFallback` | 654 | data/training/runs3/h3_r3_rules/summary.json: ingest.fallback |
| `\runHThreeRThreeRulesGRetentionHi` | 40.0 | data/training/runs3/h3_r3_rules/summary.json: 100 * by_kind.G_retention.ci95[1] |
| `\runHThreeRThreeRulesGRetentionLo` | 12.4 | data/training/runs3/h3_r3_rules/summary.json: 100 * by_kind.G_retention.ci95[0] |
| `\runHThreeRThreeRulesGRetentionN` | 34 | data/training/runs3/h3_r3_rules/summary.json: by_kind.G_retention.n |
| `\runHThreeRThreeRulesGRetentionPass` | 8 | data/training/runs3/h3_r3_rules/summary.json: by_kind.G_retention.pass |
| `\runHThreeRThreeRulesGRetentionRate` | 23.5 | data/training/runs3/h3_r3_rules/summary.json: 100 * by_kind.G_retention.rate |
| `\runHThreeRThreeRulesGRetentionRecall` | 57.0 | data/training/runs3/h3_r3_rules/summary.json: 100 * by_kind.G_retention.mean_recall |
| `\runHThreeRThreeRulesGRetentionWrong` | 4 | data/training/runs3/h3_r3_rules/summary.json: by_kind.G_retention.with_wrong_claim |
| `\runHThreeRThreeRulesGSecondHi` | 36.8 | data/training/runs3/h3_r3_rules/summary.json: 100 * by_kind.G_second.ci95[1] |
| `\runHThreeRThreeRulesGSecondLo` | 10.3 | data/training/runs3/h3_r3_rules/summary.json: 100 * by_kind.G_second.ci95[0] |
| `\runHThreeRThreeRulesGSecondN` | 34 | data/training/runs3/h3_r3_rules/summary.json: by_kind.G_second.n |
| `\runHThreeRThreeRulesGSecondPass` | 7 | data/training/runs3/h3_r3_rules/summary.json: by_kind.G_second.pass |
| `\runHThreeRThreeRulesGSecondRate` | 20.6 | data/training/runs3/h3_r3_rules/summary.json: 100 * by_kind.G_second.rate |
| `\runHThreeRThreeRulesGSecondRecall` | 62.1 | data/training/runs3/h3_r3_rules/summary.json: 100 * by_kind.G_second.mean_recall |
| `\runHThreeRThreeRulesGSecondWrong` | 6 | data/training/runs3/h3_r3_rules/summary.json: by_kind.G_second.with_wrong_claim |
| `\runHThreeRThreeRulesGVariantHi` | 10.7 | data/training/runs3/h3_r3_rules/summary.json: 100 * by_kind.G_variant.ci95[1] |
| `\runHThreeRThreeRulesGVariantLo` | 0.9 | data/training/runs3/h3_r3_rules/summary.json: 100 * by_kind.G_variant.ci95[0] |
| `\runHThreeRThreeRulesGVariantN` | 64 | data/training/runs3/h3_r3_rules/summary.json: by_kind.G_variant.n |
| `\runHThreeRThreeRulesGVariantPass` | 2 | data/training/runs3/h3_r3_rules/summary.json: by_kind.G_variant.pass |
| `\runHThreeRThreeRulesGVariantRate` | 3.1 | data/training/runs3/h3_r3_rules/summary.json: 100 * by_kind.G_variant.rate |
| `\runHThreeRThreeRulesGVariantRecall` | 56.6 | data/training/runs3/h3_r3_rules/summary.json: 100 * by_kind.G_variant.mean_recall |
| `\runHThreeRThreeRulesGVariantWrong` | 24 | data/training/runs3/h3_r3_rules/summary.json: by_kind.G_variant.with_wrong_claim |
| `\runHThreeRThreeRulesLearned` | 49 | data/training/runs3/h3_r3_rules/summary.json: ingest.learned |
| `\runHThreeRThreeRulesLines` | 703 | data/training/runs3/h3_r3_rules/summary.json: ingest.lines |
| `\runHThreeRThreeRulesSFirstHi` | 15.9 | data/training/runs3/h3_r3_rules/summary.json: 100 * by_kind.S_first.ci95[1] |
| `\runHThreeRThreeRulesSFirstLo` | 2.6 | data/training/runs3/h3_r3_rules/summary.json: 100 * by_kind.S_first.ci95[0] |
| `\runHThreeRThreeRulesSFirstN` | 60 | data/training/runs3/h3_r3_rules/summary.json: by_kind.S_first.n |
| `\runHThreeRThreeRulesSFirstPass` | 4 | data/training/runs3/h3_r3_rules/summary.json: by_kind.S_first.pass |
| `\runHThreeRThreeRulesSFirstRate` | 6.7 | data/training/runs3/h3_r3_rules/summary.json: 100 * by_kind.S_first.rate |
| `\runHThreeRThreeRulesSFirstRecall` | 70.8 | data/training/runs3/h3_r3_rules/summary.json: 100 * by_kind.S_first.mean_recall |
| `\runHThreeRThreeRulesSFirstWrong` | 7 | data/training/runs3/h3_r3_rules/summary.json: by_kind.S_first.with_wrong_claim |
| `\runHThreeRThreeRulesStoredGN` | 34 | data/training/runs3/h3_r3_rules/summary.json: stored_G.n |
| `\runHThreeRThreeRulesStoredGPass` | 9 | data/training/runs3/h3_r3_rules/summary.json: stored_G.pass |
| `\runHThreeRThreeRulesStoredTN` | 60 | data/training/runs3/h3_r3_rules/summary.json: stored_T.n |
| `\runHThreeRThreeRulesStoredTPass` | 17 | data/training/runs3/h3_r3_rules/summary.json: stored_T.pass |
| `\runHThreeRThreeRulesTRetentionHi` | 28.0 | data/training/runs3/h3_r3_rules/summary.json: 100 * by_kind.T_retention.ci95[1] |
| `\runHThreeRThreeRulesTRetentionLo` | 9.3 | data/training/runs3/h3_r3_rules/summary.json: 100 * by_kind.T_retention.ci95[0] |
| `\runHThreeRThreeRulesTRetentionN` | 60 | data/training/runs3/h3_r3_rules/summary.json: by_kind.T_retention.n |
| `\runHThreeRThreeRulesTRetentionPass` | 10 | data/training/runs3/h3_r3_rules/summary.json: by_kind.T_retention.pass |
| `\runHThreeRThreeRulesTRetentionRate` | 16.7 | data/training/runs3/h3_r3_rules/summary.json: 100 * by_kind.T_retention.rate |
| `\runHThreeRThreeRulesTRetentionRecall` | 45.6 | data/training/runs3/h3_r3_rules/summary.json: 100 * by_kind.T_retention.mean_recall |
| `\runHThreeRThreeRulesTRetentionWrong` | 7 | data/training/runs3/h3_r3_rules/summary.json: by_kind.T_retention.with_wrong_claim |
| `\runHThreeRThreeRulesTSecondHi` | 37.2 | data/training/runs3/h3_r3_rules/summary.json: 100 * by_kind.T_second.ci95[1] |
| `\runHThreeRThreeRulesTSecondLo` | 15.8 | data/training/runs3/h3_r3_rules/summary.json: 100 * by_kind.T_second.ci95[0] |
| `\runHThreeRThreeRulesTSecondN` | 60 | data/training/runs3/h3_r3_rules/summary.json: by_kind.T_second.n |
| `\runHThreeRThreeRulesTSecondPass` | 15 | data/training/runs3/h3_r3_rules/summary.json: by_kind.T_second.pass |
| `\runHThreeRThreeRulesTSecondRate` | 25.0 | data/training/runs3/h3_r3_rules/summary.json: 100 * by_kind.T_second.rate |
| `\runHThreeRThreeRulesTSecondRecall` | 60.3 | data/training/runs3/h3_r3_rules/summary.json: 100 * by_kind.T_second.mean_recall |
| `\runHThreeRThreeRulesTSecondWrong` | 6 | data/training/runs3/h3_r3_rules/summary.json: by_kind.T_second.with_wrong_claim |
| `\runHThreeRThreeRulesTVariantHi` | 9.4 | data/training/runs3/h3_r3_rules/summary.json: 100 * by_kind.T_variant.ci95[1] |
| `\runHThreeRThreeRulesTVariantLo` | 1.8 | data/training/runs3/h3_r3_rules/summary.json: 100 * by_kind.T_variant.ci95[0] |
| `\runHThreeRThreeRulesTVariantN` | 120 | data/training/runs3/h3_r3_rules/summary.json: by_kind.T_variant.n |
| `\runHThreeRThreeRulesTVariantPass` | 5 | data/training/runs3/h3_r3_rules/summary.json: by_kind.T_variant.pass |
| `\runHThreeRThreeRulesTVariantRate` | 4.2 | data/training/runs3/h3_r3_rules/summary.json: 100 * by_kind.T_variant.rate |
| `\runHThreeRThreeRulesTVariantRecall` | 63.3 | data/training/runs3/h3_r3_rules/summary.json: 100 * by_kind.T_variant.mean_recall |
| `\runHThreeRThreeRulesTVariantWrong` | 29 | data/training/runs3/h3_r3_rules/summary.json: by_kind.T_variant.with_wrong_claim |
| `\runHThreeRTwoCTwoCalls` | 806 | data/training/runs3/h3_r2_c2/summary.json: live_calls |
| `\runHThreeRTwoCTwoFallback` | 635 | data/training/runs3/h3_r2_c2/summary.json: ingest.fallback |
| `\runHThreeRTwoCTwoGRetentionHi` | 36.8 | data/training/runs3/h3_r2_c2/summary.json: 100 * by_kind.G_retention.ci95[1] |
| `\runHThreeRTwoCTwoGRetentionLo` | 10.3 | data/training/runs3/h3_r2_c2/summary.json: 100 * by_kind.G_retention.ci95[0] |
| `\runHThreeRTwoCTwoGRetentionN` | 34 | data/training/runs3/h3_r2_c2/summary.json: by_kind.G_retention.n |
| `\runHThreeRTwoCTwoGRetentionPass` | 7 | data/training/runs3/h3_r2_c2/summary.json: by_kind.G_retention.pass |
| `\runHThreeRTwoCTwoGRetentionRate` | 20.6 | data/training/runs3/h3_r2_c2/summary.json: 100 * by_kind.G_retention.rate |
| `\runHThreeRTwoCTwoGRetentionRecall` | 60.4 | data/training/runs3/h3_r2_c2/summary.json: 100 * by_kind.G_retention.mean_recall |
| `\runHThreeRTwoCTwoGRetentionWrong` | 3 | data/training/runs3/h3_r2_c2/summary.json: by_kind.G_retention.with_wrong_claim |
| `\runHThreeRTwoCTwoGSecondHi` | 30.1 | data/training/runs3/h3_r2_c2/summary.json: 100 * by_kind.G_second.ci95[1] |
| `\runHThreeRTwoCTwoGSecondLo` | 6.4 | data/training/runs3/h3_r2_c2/summary.json: 100 * by_kind.G_second.ci95[0] |
| `\runHThreeRTwoCTwoGSecondN` | 34 | data/training/runs3/h3_r2_c2/summary.json: by_kind.G_second.n |
| `\runHThreeRTwoCTwoGSecondPass` | 5 | data/training/runs3/h3_r2_c2/summary.json: by_kind.G_second.pass |
| `\runHThreeRTwoCTwoGSecondRate` | 14.7 | data/training/runs3/h3_r2_c2/summary.json: 100 * by_kind.G_second.rate |
| `\runHThreeRTwoCTwoGSecondRecall` | 63.4 | data/training/runs3/h3_r2_c2/summary.json: 100 * by_kind.G_second.mean_recall |
| `\runHThreeRTwoCTwoGSecondWrong` | 6 | data/training/runs3/h3_r2_c2/summary.json: by_kind.G_second.with_wrong_claim |
| `\runHThreeRTwoCTwoGVariantHi` | 5.7 | data/training/runs3/h3_r2_c2/summary.json: 100 * by_kind.G_variant.ci95[1] |
| `\runHThreeRTwoCTwoGVariantLo` | 0.0 | data/training/runs3/h3_r2_c2/summary.json: 100 * by_kind.G_variant.ci95[0] |
| `\runHThreeRTwoCTwoGVariantN` | 64 | data/training/runs3/h3_r2_c2/summary.json: by_kind.G_variant.n |
| `\runHThreeRTwoCTwoGVariantPass` | 0 | data/training/runs3/h3_r2_c2/summary.json: by_kind.G_variant.pass |
| `\runHThreeRTwoCTwoGVariantRate` | 0.0 | data/training/runs3/h3_r2_c2/summary.json: 100 * by_kind.G_variant.rate |
| `\runHThreeRTwoCTwoGVariantRecall` | 57.1 | data/training/runs3/h3_r2_c2/summary.json: 100 * by_kind.G_variant.mean_recall |
| `\runHThreeRTwoCTwoGVariantWrong` | 18 | data/training/runs3/h3_r2_c2/summary.json: by_kind.G_variant.with_wrong_claim |
| `\runHThreeRTwoCTwoLearned` | 68 | data/training/runs3/h3_r2_c2/summary.json: ingest.learned |
| `\runHThreeRTwoCTwoLines` | 703 | data/training/runs3/h3_r2_c2/summary.json: ingest.lines |
| `\runHThreeRTwoCTwoSFirstHi` | 20.1 | data/training/runs3/h3_r2_c2/summary.json: 100 * by_kind.S_first.ci95[1] |
| `\runHThreeRTwoCTwoSFirstLo` | 4.7 | data/training/runs3/h3_r2_c2/summary.json: 100 * by_kind.S_first.ci95[0] |
| `\runHThreeRTwoCTwoSFirstN` | 60 | data/training/runs3/h3_r2_c2/summary.json: by_kind.S_first.n |
| `\runHThreeRTwoCTwoSFirstPass` | 6 | data/training/runs3/h3_r2_c2/summary.json: by_kind.S_first.pass |
| `\runHThreeRTwoCTwoSFirstRate` | 10.0 | data/training/runs3/h3_r2_c2/summary.json: 100 * by_kind.S_first.rate |
| `\runHThreeRTwoCTwoSFirstRecall` | 70.5 | data/training/runs3/h3_r2_c2/summary.json: 100 * by_kind.S_first.mean_recall |
| `\runHThreeRTwoCTwoSFirstWrong` | 9 | data/training/runs3/h3_r2_c2/summary.json: by_kind.S_first.with_wrong_claim |
| `\runHThreeRTwoCTwoStoredGN` | 34 | data/training/runs3/h3_r2_c2/summary.json: stored_G.n |
| `\runHThreeRTwoCTwoStoredGPass` | 7 | data/training/runs3/h3_r2_c2/summary.json: stored_G.pass |
| `\runHThreeRTwoCTwoStoredTN` | 60 | data/training/runs3/h3_r2_c2/summary.json: stored_T.n |
| `\runHThreeRTwoCTwoStoredTPass` | 19 | data/training/runs3/h3_r2_c2/summary.json: stored_T.pass |
| `\runHThreeRTwoCTwoTRetentionHi` | 22.2 | data/training/runs3/h3_r2_c2/summary.json: 100 * by_kind.T_retention.ci95[1] |
| `\runHThreeRTwoCTwoTRetentionLo` | 5.8 | data/training/runs3/h3_r2_c2/summary.json: 100 * by_kind.T_retention.ci95[0] |
| `\runHThreeRTwoCTwoTRetentionN` | 60 | data/training/runs3/h3_r2_c2/summary.json: by_kind.T_retention.n |
| `\runHThreeRTwoCTwoTRetentionPass` | 7 | data/training/runs3/h3_r2_c2/summary.json: by_kind.T_retention.pass |
| `\runHThreeRTwoCTwoTRetentionRate` | 11.7 | data/training/runs3/h3_r2_c2/summary.json: 100 * by_kind.T_retention.rate |
| `\runHThreeRTwoCTwoTRetentionRecall` | 43.0 | data/training/runs3/h3_r2_c2/summary.json: 100 * by_kind.T_retention.mean_recall |
| `\runHThreeRTwoCTwoTRetentionWrong` | 3 | data/training/runs3/h3_r2_c2/summary.json: by_kind.T_retention.with_wrong_claim |
| `\runHThreeRTwoCTwoTSecondHi` | 37.2 | data/training/runs3/h3_r2_c2/summary.json: 100 * by_kind.T_second.ci95[1] |
| `\runHThreeRTwoCTwoTSecondLo` | 15.8 | data/training/runs3/h3_r2_c2/summary.json: 100 * by_kind.T_second.ci95[0] |
| `\runHThreeRTwoCTwoTSecondN` | 60 | data/training/runs3/h3_r2_c2/summary.json: by_kind.T_second.n |
| `\runHThreeRTwoCTwoTSecondPass` | 15 | data/training/runs3/h3_r2_c2/summary.json: by_kind.T_second.pass |
| `\runHThreeRTwoCTwoTSecondRate` | 25.0 | data/training/runs3/h3_r2_c2/summary.json: 100 * by_kind.T_second.rate |
| `\runHThreeRTwoCTwoTSecondRecall` | 57.0 | data/training/runs3/h3_r2_c2/summary.json: 100 * by_kind.T_second.mean_recall |
| `\runHThreeRTwoCTwoTSecondWrong` | 2 | data/training/runs3/h3_r2_c2/summary.json: by_kind.T_second.with_wrong_claim |
| `\runHThreeRTwoCTwoTVariantHi` | 10.5 | data/training/runs3/h3_r2_c2/summary.json: 100 * by_kind.T_variant.ci95[1] |
| `\runHThreeRTwoCTwoTVariantLo` | 2.3 | data/training/runs3/h3_r2_c2/summary.json: 100 * by_kind.T_variant.ci95[0] |
| `\runHThreeRTwoCTwoTVariantN` | 120 | data/training/runs3/h3_r2_c2/summary.json: by_kind.T_variant.n |
| `\runHThreeRTwoCTwoTVariantPass` | 6 | data/training/runs3/h3_r2_c2/summary.json: by_kind.T_variant.pass |
| `\runHThreeRTwoCTwoTVariantRate` | 5.0 | data/training/runs3/h3_r2_c2/summary.json: 100 * by_kind.T_variant.rate |
| `\runHThreeRTwoCTwoTVariantRecall` | 62.5 | data/training/runs3/h3_r2_c2/summary.json: 100 * by_kind.T_variant.mean_recall |
| `\runHThreeRTwoCTwoTVariantWrong` | 18 | data/training/runs3/h3_r2_c2/summary.json: by_kind.T_variant.with_wrong_claim |
| `\runSplitFourCalls` | 0 | data/training/runs3/split4/summary.json: live_calls |
| `\runSplitFourRRetentionHi` | 54.6 | data/training/runs3/split4/summary.json: 100 * by_kind.R_retention.ci95[1] |
| `\runSplitFourRRetentionLo` | 11.7 | data/training/runs3/split4/summary.json: 100 * by_kind.R_retention.ci95[0] |
| `\runSplitFourRRetentionN` | 14 | data/training/runs3/split4/summary.json: by_kind.R_retention.n |
| `\runSplitFourRRetentionPass` | 4 | data/training/runs3/split4/summary.json: by_kind.R_retention.pass |
| `\runSplitFourRRetentionRate` | 28.6 | data/training/runs3/split4/summary.json: 100 * by_kind.R_retention.rate |
| `\runSplitFourRRetentionRecall` | 70.9 | data/training/runs3/split4/summary.json: 100 * by_kind.R_retention.mean_recall |
| `\runSplitFourRRetentionWrong` | 3 | data/training/runs3/split4/summary.json: by_kind.R_retention.with_wrong_claim |
| `\runSplitFourRSecondHi` | 54.6 | data/training/runs3/split4/summary.json: 100 * by_kind.R_second.ci95[1] |
| `\runSplitFourRSecondLo` | 11.7 | data/training/runs3/split4/summary.json: 100 * by_kind.R_second.ci95[0] |
| `\runSplitFourRSecondN` | 14 | data/training/runs3/split4/summary.json: by_kind.R_second.n |
| `\runSplitFourRSecondPass` | 4 | data/training/runs3/split4/summary.json: by_kind.R_second.pass |
| `\runSplitFourRSecondRate` | 28.6 | data/training/runs3/split4/summary.json: 100 * by_kind.R_second.rate |
| `\runSplitFourRSecondRecall` | 72.8 | data/training/runs3/split4/summary.json: 100 * by_kind.R_second.mean_recall |
| `\runSplitFourRSecondWrong` | 2 | data/training/runs3/split4/summary.json: by_kind.R_second.with_wrong_claim |
| `\runSplitFourRVariantHi` | 24.2 | data/training/runs3/split4/summary.json: 100 * by_kind.R_variant.ci95[1] |
| `\runSplitFourRVariantLo` | 0.0 | data/training/runs3/split4/summary.json: 100 * by_kind.R_variant.ci95[0] |
| `\runSplitFourRVariantN` | 12 | data/training/runs3/split4/summary.json: by_kind.R_variant.n |
| `\runSplitFourRVariantPass` | 0 | data/training/runs3/split4/summary.json: by_kind.R_variant.pass |
| `\runSplitFourRVariantRate` | 0.0 | data/training/runs3/split4/summary.json: 100 * by_kind.R_variant.rate |
| `\runSplitFourRVariantRecall` | 57.4 | data/training/runs3/split4/summary.json: 100 * by_kind.R_variant.mean_recall |
| `\runSplitFourRVariantRepeatHi` | 44.8 | data/training/runs3/split4/summary.json: 100 * by_kind.R_variant_repeat.ci95[1] |
| `\runSplitFourRVariantRepeatLo` | 4.7 | data/training/runs3/split4/summary.json: 100 * by_kind.R_variant_repeat.ci95[0] |
| `\runSplitFourRVariantRepeatN` | 12 | data/training/runs3/split4/summary.json: by_kind.R_variant_repeat.n |
| `\runSplitFourRVariantRepeatPass` | 2 | data/training/runs3/split4/summary.json: by_kind.R_variant_repeat.pass |
| `\runSplitFourRVariantRepeatRate` | 16.7 | data/training/runs3/split4/summary.json: 100 * by_kind.R_variant_repeat.rate |
| `\runSplitFourRVariantRepeatRecall` | 71.8 | data/training/runs3/split4/summary.json: 100 * by_kind.R_variant_repeat.mean_recall |
| `\runSplitFourRVariantRepeatWrong` | 4 | data/training/runs3/split4/summary.json: by_kind.R_variant_repeat.with_wrong_claim |
| `\runSplitFourRVariantWrong` | 2 | data/training/runs3/split4/summary.json: by_kind.R_variant.with_wrong_claim |
| `\runSplitOneCalls` | 1 | data/training/runs3/split1/summary.json: live_calls |
| `\runSplitOneRRetentionHi` | 47.6 | data/training/runs3/split1/summary.json: 100 * by_kind.R_retention.ci95[1] |
| `\runSplitOneRRetentionLo` | 7.6 | data/training/runs3/split1/summary.json: 100 * by_kind.R_retention.ci95[0] |
| `\runSplitOneRRetentionN` | 14 | data/training/runs3/split1/summary.json: by_kind.R_retention.n |
| `\runSplitOneRRetentionPass` | 3 | data/training/runs3/split1/summary.json: by_kind.R_retention.pass |
| `\runSplitOneRRetentionRate` | 21.4 | data/training/runs3/split1/summary.json: 100 * by_kind.R_retention.rate |
| `\runSplitOneRRetentionRecall` | 56.3 | data/training/runs3/split1/summary.json: 100 * by_kind.R_retention.mean_recall |
| `\runSplitOneRRetentionWrong` | 2 | data/training/runs3/split1/summary.json: by_kind.R_retention.with_wrong_claim |
| `\runSplitOneRSecondHi` | 61.2 | data/training/runs3/split1/summary.json: 100 * by_kind.R_second.ci95[1] |
| `\runSplitOneRSecondLo` | 16.3 | data/training/runs3/split1/summary.json: 100 * by_kind.R_second.ci95[0] |
| `\runSplitOneRSecondN` | 14 | data/training/runs3/split1/summary.json: by_kind.R_second.n |
| `\runSplitOneRSecondPass` | 5 | data/training/runs3/split1/summary.json: by_kind.R_second.pass |
| `\runSplitOneRSecondRate` | 35.7 | data/training/runs3/split1/summary.json: 100 * by_kind.R_second.rate |
| `\runSplitOneRSecondRecall` | 76.0 | data/training/runs3/split1/summary.json: 100 * by_kind.R_second.mean_recall |
| `\runSplitOneRSecondWrong` | 1 | data/training/runs3/split1/summary.json: by_kind.R_second.with_wrong_claim |
| `\runSplitOneRVariantHi` | 27.8 | data/training/runs3/split1/summary.json: 100 * by_kind.R_variant.ci95[1] |
| `\runSplitOneRVariantLo` | 0.0 | data/training/runs3/split1/summary.json: 100 * by_kind.R_variant.ci95[0] |
| `\runSplitOneRVariantN` | 10 | data/training/runs3/split1/summary.json: by_kind.R_variant.n |
| `\runSplitOneRVariantPass` | 0 | data/training/runs3/split1/summary.json: by_kind.R_variant.pass |
| `\runSplitOneRVariantRate` | 0.0 | data/training/runs3/split1/summary.json: 100 * by_kind.R_variant.rate |
| `\runSplitOneRVariantRecall` | 62.9 | data/training/runs3/split1/summary.json: 100 * by_kind.R_variant.mean_recall |
| `\runSplitOneRVariantRepeatHi` | 40.4 | data/training/runs3/split1/summary.json: 100 * by_kind.R_variant_repeat.ci95[1] |
| `\runSplitOneRVariantRepeatLo` | 1.8 | data/training/runs3/split1/summary.json: 100 * by_kind.R_variant_repeat.ci95[0] |
| `\runSplitOneRVariantRepeatN` | 10 | data/training/runs3/split1/summary.json: by_kind.R_variant_repeat.n |
| `\runSplitOneRVariantRepeatPass` | 1 | data/training/runs3/split1/summary.json: by_kind.R_variant_repeat.pass |
| `\runSplitOneRVariantRepeatRate` | 10.0 | data/training/runs3/split1/summary.json: 100 * by_kind.R_variant_repeat.rate |
| `\runSplitOneRVariantRepeatRecall` | 65.5 | data/training/runs3/split1/summary.json: 100 * by_kind.R_variant_repeat.mean_recall |
| `\runSplitOneRVariantRepeatWrong` | 3 | data/training/runs3/split1/summary.json: by_kind.R_variant_repeat.with_wrong_claim |
| `\runSplitOneRVariantWrong` | 2 | data/training/runs3/split1/summary.json: by_kind.R_variant.with_wrong_claim |
| `\runSplitThreeCalls` | 0 | data/training/runs3/split3/summary.json: live_calls |
| `\runSplitThreeRRetentionHi` | 54.6 | data/training/runs3/split3/summary.json: 100 * by_kind.R_retention.ci95[1] |
| `\runSplitThreeRRetentionLo` | 11.7 | data/training/runs3/split3/summary.json: 100 * by_kind.R_retention.ci95[0] |
| `\runSplitThreeRRetentionN` | 14 | data/training/runs3/split3/summary.json: by_kind.R_retention.n |
| `\runSplitThreeRRetentionPass` | 4 | data/training/runs3/split3/summary.json: by_kind.R_retention.pass |
| `\runSplitThreeRRetentionRate` | 28.6 | data/training/runs3/split3/summary.json: 100 * by_kind.R_retention.rate |
| `\runSplitThreeRRetentionRecall` | 68.1 | data/training/runs3/split3/summary.json: 100 * by_kind.R_retention.mean_recall |
| `\runSplitThreeRRetentionWrong` | 3 | data/training/runs3/split3/summary.json: by_kind.R_retention.with_wrong_claim |
| `\runSplitThreeRSecondHi` | 54.6 | data/training/runs3/split3/summary.json: 100 * by_kind.R_second.ci95[1] |
| `\runSplitThreeRSecondLo` | 11.7 | data/training/runs3/split3/summary.json: 100 * by_kind.R_second.ci95[0] |
| `\runSplitThreeRSecondN` | 14 | data/training/runs3/split3/summary.json: by_kind.R_second.n |
| `\runSplitThreeRSecondPass` | 4 | data/training/runs3/split3/summary.json: by_kind.R_second.pass |
| `\runSplitThreeRSecondRate` | 28.6 | data/training/runs3/split3/summary.json: 100 * by_kind.R_second.rate |
| `\runSplitThreeRSecondRecall` | 78.4 | data/training/runs3/split3/summary.json: 100 * by_kind.R_second.mean_recall |
| `\runSplitThreeRSecondWrong` | 1 | data/training/runs3/split3/summary.json: by_kind.R_second.with_wrong_claim |
| `\runSplitThreeRVariantHi` | 24.2 | data/training/runs3/split3/summary.json: 100 * by_kind.R_variant.ci95[1] |
| `\runSplitThreeRVariantLo` | 0.0 | data/training/runs3/split3/summary.json: 100 * by_kind.R_variant.ci95[0] |
| `\runSplitThreeRVariantN` | 12 | data/training/runs3/split3/summary.json: by_kind.R_variant.n |
| `\runSplitThreeRVariantPass` | 0 | data/training/runs3/split3/summary.json: by_kind.R_variant.pass |
| `\runSplitThreeRVariantRate` | 0.0 | data/training/runs3/split3/summary.json: 100 * by_kind.R_variant.rate |
| `\runSplitThreeRVariantRecall` | 47.3 | data/training/runs3/split3/summary.json: 100 * by_kind.R_variant.mean_recall |
| `\runSplitThreeRVariantRepeatHi` | 35.4 | data/training/runs3/split3/summary.json: 100 * by_kind.R_variant_repeat.ci95[1] |
| `\runSplitThreeRVariantRepeatLo` | 1.5 | data/training/runs3/split3/summary.json: 100 * by_kind.R_variant_repeat.ci95[0] |
| `\runSplitThreeRVariantRepeatN` | 12 | data/training/runs3/split3/summary.json: by_kind.R_variant_repeat.n |
| `\runSplitThreeRVariantRepeatPass` | 1 | data/training/runs3/split3/summary.json: by_kind.R_variant_repeat.pass |
| `\runSplitThreeRVariantRepeatRate` | 8.3 | data/training/runs3/split3/summary.json: 100 * by_kind.R_variant_repeat.rate |
| `\runSplitThreeRVariantRepeatRecall` | 52.3 | data/training/runs3/split3/summary.json: 100 * by_kind.R_variant_repeat.mean_recall |
| `\runSplitThreeRVariantRepeatWrong` | 4 | data/training/runs3/split3/summary.json: by_kind.R_variant_repeat.with_wrong_claim |
| `\runSplitThreeRVariantWrong` | 4 | data/training/runs3/split3/summary.json: by_kind.R_variant.with_wrong_claim |
| `\runSplitTwoCalls` | 1 | data/training/runs3/split2/summary.json: live_calls |
| `\runSplitTwoRRetentionHi` | 61.2 | data/training/runs3/split2/summary.json: 100 * by_kind.R_retention.ci95[1] |
| `\runSplitTwoRRetentionLo` | 16.3 | data/training/runs3/split2/summary.json: 100 * by_kind.R_retention.ci95[0] |
| `\runSplitTwoRRetentionN` | 14 | data/training/runs3/split2/summary.json: by_kind.R_retention.n |
| `\runSplitTwoRRetentionPass` | 5 | data/training/runs3/split2/summary.json: by_kind.R_retention.pass |
| `\runSplitTwoRRetentionRate` | 35.7 | data/training/runs3/split2/summary.json: 100 * by_kind.R_retention.rate |
| `\runSplitTwoRRetentionRecall` | 67.0 | data/training/runs3/split2/summary.json: 100 * by_kind.R_retention.mean_recall |
| `\runSplitTwoRRetentionWrong` | 1 | data/training/runs3/split2/summary.json: by_kind.R_retention.with_wrong_claim |
| `\runSplitTwoRSecondHi` | 67.4 | data/training/runs3/split2/summary.json: 100 * by_kind.R_second.ci95[1] |
| `\runSplitTwoRSecondLo` | 21.4 | data/training/runs3/split2/summary.json: 100 * by_kind.R_second.ci95[0] |
| `\runSplitTwoRSecondN` | 14 | data/training/runs3/split2/summary.json: by_kind.R_second.n |
| `\runSplitTwoRSecondPass` | 6 | data/training/runs3/split2/summary.json: by_kind.R_second.pass |
| `\runSplitTwoRSecondRate` | 42.9 | data/training/runs3/split2/summary.json: 100 * by_kind.R_second.rate |
| `\runSplitTwoRSecondRecall` | 75.7 | data/training/runs3/split2/summary.json: 100 * by_kind.R_second.mean_recall |
| `\runSplitTwoRSecondWrong` | 2 | data/training/runs3/split2/summary.json: by_kind.R_second.with_wrong_claim |
| `\runSplitTwoRVariantHi` | 29.9 | data/training/runs3/split2/summary.json: 100 * by_kind.R_variant.ci95[1] |
| `\runSplitTwoRVariantLo` | 0.0 | data/training/runs3/split2/summary.json: 100 * by_kind.R_variant.ci95[0] |
| `\runSplitTwoRVariantN` | 9 | data/training/runs3/split2/summary.json: by_kind.R_variant.n |
| `\runSplitTwoRVariantPass` | 0 | data/training/runs3/split2/summary.json: by_kind.R_variant.pass |
| `\runSplitTwoRVariantRate` | 0.0 | data/training/runs3/split2/summary.json: 100 * by_kind.R_variant.rate |
| `\runSplitTwoRVariantRecall` | 59.9 | data/training/runs3/split2/summary.json: 100 * by_kind.R_variant.mean_recall |
| `\runSplitTwoRVariantRepeatHi` | 43.5 | data/training/runs3/split2/summary.json: 100 * by_kind.R_variant_repeat.ci95[1] |
| `\runSplitTwoRVariantRepeatLo` | 2.0 | data/training/runs3/split2/summary.json: 100 * by_kind.R_variant_repeat.ci95[0] |
| `\runSplitTwoRVariantRepeatN` | 9 | data/training/runs3/split2/summary.json: by_kind.R_variant_repeat.n |
| `\runSplitTwoRVariantRepeatPass` | 1 | data/training/runs3/split2/summary.json: by_kind.R_variant_repeat.pass |
| `\runSplitTwoRVariantRepeatRate` | 11.1 | data/training/runs3/split2/summary.json: 100 * by_kind.R_variant_repeat.rate |
| `\runSplitTwoRVariantRepeatRecall` | 63.1 | data/training/runs3/split2/summary.json: 100 * by_kind.R_variant_repeat.mean_recall |
| `\runSplitTwoRVariantRepeatWrong` | 4 | data/training/runs3/split2/summary.json: by_kind.R_variant_repeat.with_wrong_claim |
| `\runSplitTwoRVariantWrong` | 2 | data/training/runs3/split2/summary.json: by_kind.R_variant.with_wrong_claim |
| `\runSplitZeroCalls` | 1 | data/training/runs3/split0/summary.json: live_calls |
| `\runSplitZeroRRetentionHi` | 47.6 | data/training/runs3/split0/summary.json: 100 * by_kind.R_retention.ci95[1] |
| `\runSplitZeroRRetentionLo` | 7.6 | data/training/runs3/split0/summary.json: 100 * by_kind.R_retention.ci95[0] |
| `\runSplitZeroRRetentionN` | 14 | data/training/runs3/split0/summary.json: by_kind.R_retention.n |
| `\runSplitZeroRRetentionPass` | 3 | data/training/runs3/split0/summary.json: by_kind.R_retention.pass |
| `\runSplitZeroRRetentionRate` | 21.4 | data/training/runs3/split0/summary.json: 100 * by_kind.R_retention.rate |
| `\runSplitZeroRRetentionRecall` | 59.0 | data/training/runs3/split0/summary.json: 100 * by_kind.R_retention.mean_recall |
| `\runSplitZeroRRetentionWrong` | 2 | data/training/runs3/split0/summary.json: by_kind.R_retention.with_wrong_claim |
| `\runSplitZeroRSecondHi` | 54.6 | data/training/runs3/split0/summary.json: 100 * by_kind.R_second.ci95[1] |
| `\runSplitZeroRSecondLo` | 11.7 | data/training/runs3/split0/summary.json: 100 * by_kind.R_second.ci95[0] |
| `\runSplitZeroRSecondN` | 14 | data/training/runs3/split0/summary.json: by_kind.R_second.n |
| `\runSplitZeroRSecondPass` | 4 | data/training/runs3/split0/summary.json: by_kind.R_second.pass |
| `\runSplitZeroRSecondRate` | 28.6 | data/training/runs3/split0/summary.json: 100 * by_kind.R_second.rate |
| `\runSplitZeroRSecondRecall` | 71.2 | data/training/runs3/split0/summary.json: 100 * by_kind.R_second.mean_recall |
| `\runSplitZeroRSecondWrong` | 2 | data/training/runs3/split0/summary.json: by_kind.R_second.with_wrong_claim |
| `\runSplitZeroRVariantHi` | 27.8 | data/training/runs3/split0/summary.json: 100 * by_kind.R_variant.ci95[1] |
| `\runSplitZeroRVariantLo` | 0.0 | data/training/runs3/split0/summary.json: 100 * by_kind.R_variant.ci95[0] |
| `\runSplitZeroRVariantN` | 10 | data/training/runs3/split0/summary.json: by_kind.R_variant.n |
| `\runSplitZeroRVariantPass` | 0 | data/training/runs3/split0/summary.json: by_kind.R_variant.pass |
| `\runSplitZeroRVariantRate` | 0.0 | data/training/runs3/split0/summary.json: 100 * by_kind.R_variant.rate |
| `\runSplitZeroRVariantRecall` | 56.7 | data/training/runs3/split0/summary.json: 100 * by_kind.R_variant.mean_recall |
| `\runSplitZeroRVariantRepeatHi` | 27.8 | data/training/runs3/split0/summary.json: 100 * by_kind.R_variant_repeat.ci95[1] |
| `\runSplitZeroRVariantRepeatLo` | 0.0 | data/training/runs3/split0/summary.json: 100 * by_kind.R_variant_repeat.ci95[0] |
| `\runSplitZeroRVariantRepeatN` | 10 | data/training/runs3/split0/summary.json: by_kind.R_variant_repeat.n |
| `\runSplitZeroRVariantRepeatPass` | 0 | data/training/runs3/split0/summary.json: by_kind.R_variant_repeat.pass |
| `\runSplitZeroRVariantRepeatRate` | 0.0 | data/training/runs3/split0/summary.json: 100 * by_kind.R_variant_repeat.rate |
| `\runSplitZeroRVariantRepeatRecall` | 73.8 | data/training/runs3/split0/summary.json: 100 * by_kind.R_variant_repeat.mean_recall |
| `\runSplitZeroRVariantRepeatWrong` | 6 | data/training/runs3/split0/summary.json: by_kind.R_variant_repeat.with_wrong_claim |
| `\runSplitZeroRVariantWrong` | 3 | data/training/runs3/split0/summary.json: by_kind.R_variant.with_wrong_claim |
| `\splitRRetentionN` | 70 | data/training/runs3/split0-4/summary.json: sum of by_kind.R_retention.n |
| `\splitRRetentionPass` | 19 | data/training/runs3/split0-4/summary.json: sum of by_kind.R_retention.pass |
| `\splitRRetentionRate` | 27.1 | data/training/runs3/split0-4/summary.json: 100 * pass / n |
| `\splitRSecondN` | 70 | data/training/runs3/split0-4/summary.json: sum of by_kind.R_second.n |
| `\splitRSecondPass` | 23 | data/training/runs3/split0-4/summary.json: sum of by_kind.R_second.pass |
| `\splitRSecondRate` | 32.9 | data/training/runs3/split0-4/summary.json: 100 * pass / n |
| `\splitRVariantN` | 53 | data/training/runs3/split0-4/summary.json: sum of by_kind.R_variant.n |
| `\splitRVariantPass` | 0 | data/training/runs3/split0-4/summary.json: sum of by_kind.R_variant.pass |
| `\splitRVariantRate` | 0.0 | data/training/runs3/split0-4/summary.json: 100 * pass / n |
| `\splitRVariantRepeatN` | 53 | data/training/runs3/split0-4/summary.json: sum of by_kind.R_variant_repeat.n |
| `\splitRVariantRepeatPass` | 5 | data/training/runs3/split0-4/summary.json: sum of by_kind.R_variant_repeat.pass |
| `\splitRVariantRepeatRate` | 9.4 | data/training/runs3/split0-4/summary.json: 100 * pass / n |
| `\vOneHelpUnderstood` | 56 | data/paper/v1.json: understanding_help_pct |
| `\vOneKnowledgeHeldout` | 97.8 | data/paper/v1.json: knowledge_tests_heldout_pct |
| `\vOnePredicates` | 278 | data/paper/v1.json: distinct_predicates_invented |
| `\vOneRealAnswers` | 1 | data/paper/v1.json: real_answers |
| `\vOneRealAnswersOf` | 68 | data/paper/v1.json: real_answers_of |
| `\vTwoCleanRetention` | 5 | data/training/curriculum/2026-11-17.jsonl: line 2 (clean run 1) retention_passed |
| `\vTwoCleanRetentionOf` | 20 | data/training/curriculum/2026-11-17.jsonl: line 2 retention_total |
| `\vTwoCleanSecond` | 6 | data/training/curriculum/2026-11-17.jsonl: line 2 (clean run 1) repeat_passed |
| `\vTwoCleanSecondOf` | 20 | data/training/curriculum/2026-11-17.jsonl: line 2 repeat_total |
| `\vTwoCleanTransfer` | 0 | data/training/curriculum/2026-11-17.jsonl: line 2 (clean run 1) transfer_passed |
| `\vTwoCleanTransferOf` | 15 | data/training/curriculum/2026-11-17.jsonl: line 2 transfer_total |
| `\vTwoDevRetention` | 17 | data/training/curriculum/2026-11-18.jsonl: second-to-last line (final development run) retention_passed |
| `\vTwoDevSecond` | 18 | data/training/curriculum/2026-11-18.jsonl: second-to-last line (final development run) repeat_passed |
| `\vTwoDevTransfer` | 6 | data/training/curriculum/2026-11-18.jsonl: second-to-last line (final development run) transfer_passed |

## Inputs (SHA-256)

- `data/paper/v1.json` bcda56004e20e073eecc0a990ebd441d79951e1c2d7bb318faf95762f170fa71
- `data/training/calls3/ledger.jsonl` 0307f997bdeecb79d7cd9f98c9af4a209e49470afe8d357d9ec15398f43b3eb3
- `data/training/curriculum/2026-11-17.jsonl` 28107ca48596fea6e75e7bcb0e0eb7c889dd3db7263d10385cbae609382b6826
- `data/training/curriculum/2026-11-18.jsonl` 7b380fcadec0a5860aee69348204ce51dcffb949be3b186d949944c046d65196
- `data/training/parser3/models/theta.json` 9b54b565bdd4fdfa9342491170af6ba0c445569eb1ab50acd977d269ec002d27
- `data/training/runs3/b2_h3/encounters.jsonl` 5e7bfce1e00ed22501c2230db1f3f1bebf9404831e99019869bb338060dc37ae
- `data/training/runs3/b2_h3/summary.json` b9da60a85c00980944f4e8af32df3720d5f851dda275b93c207b6cdb58a1d614
- `data/training/runs3/b3_replay/encounters.jsonl` 39423de0dc8d4a5eebc8b168e45ef7efaf995dd876ab1c3843427d529272d312
- `data/training/runs3/b3_replay/summary.json` db15776013a96ac141589806b6df5b7e35a58338487c9cdf45718ae0edae82ae
- `data/training/runs3/c1_both_r2/summary.json` db1cfc9958276c4a3355f72bff6e59fa0817254f1622647fd5cf0998015af1b8
- `data/training/runs3/c1_both_r4/summary.json` 598ed16655340220b6695e5b0f4cef5106c3461dab62504cc775ef6f7cc31410
- `data/training/runs3/c1_both_r5/summary.json` 47c9be42228f7079dc1fb13f9b2eccf61830543a4aea783263b12d14e7b5ab76
- `data/training/runs3/c1_both_r6/summary.json` 000c26d32a29fcbce7c68eb4d111818de66efbb257782a6b149a64a1c61477c4
- `data/training/runs3/c1_both_r7/summary.json` 125fe64d212ce4ef203ffb7ed2def2c3ebc6717259793db231400999a62fef45
- `data/training/runs3/c1_both_r8/summary.json` 8bc761144466d2843213d445e230c8d7da2bdefaadda248b0e71d0f4bc7b1c95
- `data/training/runs3/c2_both_r1/summary.json` 0dbdd021c76c16b2cfee29a5d84f32fb6a472bba3068cc9ba2cb4216a9f9febc
- `data/training/runs3/dry_dev/summary.json` 96a281aefdb2beaec48d5fa09385683d2879747318b8832c644dec1e4fac183b
- `data/training/runs3/dry_dev2/summary.json` ef8ea3549f6b0a7aecd668f4190bbb394f61b5e3d7d2797ef72cc47f5a6da340
- `data/training/runs3/h3_r1/summary.json` 5f2f5e9c47ac2025589578ee6b6b58c3e7ad24b454f5934b3312e9ce76feca93
- `data/training/runs3/h3_r2_c2/encounters.jsonl` 0d742fc2dfe21391a246f60a0a6e711efb3705b8bbf7aa8d74f8ed07f7a39965
- `data/training/runs3/h3_r2_c2/summary.json` c0ec449cdc43f15d22cf48363bfeb85064637966b4456e079fe3c6a908f6f00b
- `data/training/runs3/h3_r3_b1/encounters.jsonl` 2d351e7e9e1ae2717a0dfc0166b05d100fc809c324c040f6f14a444e964e5479
- `data/training/runs3/h3_r3_b1/summary.json` 8ee2f319b113f753e59613845399ea1d1c1707cbacda537ee793ed9ae40c3433
- `data/training/runs3/h3_r3_function/encounters.jsonl` aad3cd4f2afa9217efbba18cd40e346ea4792759443e75d0ae35ec2f87bacefa
- `data/training/runs3/h3_r3_function/summary.json` e3a99ff286e6c68ddaa14970ceb3c07aa4a522d8f887036b0fccc5fde820ea7a
- `data/training/runs3/h3_r3_nofallback/encounters.jsonl` 0e0ab8a1cb6183c27d19dd5bc22a35e267de4368320a9fae241745e1c354d821
- `data/training/runs3/h3_r3_nofallback/summary.json` 532a1f37e2b4a9d9b20bf1d7f17dd0634b22b8b67598390f19c77ca9123618ea
- `data/training/runs3/h3_r3_rules/encounters.jsonl` 73d5946fba78855f8e87d23ae67677325fa1363c8a662d10c0d1c95e270d9cd2
- `data/training/runs3/h3_r3_rules/summary.json` 3ccb13c5300a5e5e5aa7d9ea103a60fc0024c0d86736bc1080ab8d9aea2ab579
- `data/training/runs3/split0/summary.json` 076411e19643691c2c2c39d9fb0bf036e6a49b1905b5b8a41639dc26017b0f9f
- `data/training/runs3/split1/summary.json` 423cc8f940eff55e08c0757b48b0684bf2f4cad412feea1afbec2177a6cd61e9
- `data/training/runs3/split2/summary.json` 6edce3b6e19a9880ca7982971fb20763c8624085d35f568e936524f7a0c1d8a6
- `data/training/runs3/split3/summary.json` 19289beda450e4193816e87929a6ce4fb6588e2e171650559ec814c602fe44af
- `data/training/runs3/split4/summary.json` c3e07bc610813182932a19ad8d43e8e1ffb2e35a680ea8bbe1b07f0d852b02a3
