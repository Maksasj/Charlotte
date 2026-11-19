# Sources of every number

| Macro | Value | Source |
|---|---|---|
| `\datasetTasks` | 9 | data/dataset3/*.md: file count |
| `\gtSteps` | 562 | data/curriculum3: step facts |
| `\gtTasks` | 105 | data/curriculum3: scenario files (incl. water_garden seed) |
| `\hThreeG` | 30 | data/heldout3/G: file count |
| `\hThreeS` | 60 | data/heldout3/S: file count |
| `\hThreeT` | 60 | data/heldout3/T: file count |
| `\haikuCalls` | 230 | data/training/calls3/ledger.jsonl: line count (live calls) |
| `\parserCoverage` | 33.7 | data/training/parser3/models/theta.json: 100 * coverage_at_95 |
| `\parserDevLines` | 101 | data/training/parser3/models/theta.json: dev_step_lines |
| `\runDryDevCalls` | 0 | data/training/runs3/dry_dev2/summary.json: live_calls |
| `\runDryDevFallback` | 0 | data/training/runs3/dry_dev2/summary.json: ingest.fallback |
| `\runDryDevLearned` | 139 | data/training/runs3/dry_dev2/summary.json: ingest.learned |
| `\runDryDevLines` | 139 | data/training/runs3/dry_dev2/summary.json: ingest.lines |
| `\runDryDevRetentionHi` | 48.8 | data/training/runs3/dry_dev2/summary.json: 100 * by_kind.retention.ci95[1] |
| `\runDryDevRetentionLo` | 11.8 | data/training/runs3/dry_dev2/summary.json: 100 * by_kind.retention.ci95[0] |
| `\runDryDevRetentionN` | 19 | data/training/runs3/dry_dev2/summary.json: by_kind.retention.n |
| `\runDryDevRetentionPass` | 5 | data/training/runs3/dry_dev2/summary.json: by_kind.retention.pass |
| `\runDryDevRetentionRate` | 26.3 | data/training/runs3/dry_dev2/summary.json: 100 * by_kind.retention.rate |
| `\runDryDevRetentionRecall` | 67.6 | data/training/runs3/dry_dev2/summary.json: 100 * by_kind.retention.mean_recall |
| `\runDryDevRetentionWrong` | 1 | data/training/runs3/dry_dev2/summary.json: by_kind.retention.with_wrong_claim |
| `\runDryDevSecondHi` | 48.8 | data/training/runs3/dry_dev2/summary.json: 100 * by_kind.second.ci95[1] |
| `\runDryDevSecondLo` | 11.8 | data/training/runs3/dry_dev2/summary.json: 100 * by_kind.second.ci95[0] |
| `\runDryDevSecondN` | 19 | data/training/runs3/dry_dev2/summary.json: by_kind.second.n |
| `\runDryDevSecondPass` | 5 | data/training/runs3/dry_dev2/summary.json: by_kind.second.pass |
| `\runDryDevSecondRate` | 26.3 | data/training/runs3/dry_dev2/summary.json: 100 * by_kind.second.rate |
| `\runDryDevSecondRecall` | 68.8 | data/training/runs3/dry_dev2/summary.json: 100 * by_kind.second.mean_recall |
| `\runDryDevSecondWrong` | 1 | data/training/runs3/dry_dev2/summary.json: by_kind.second.with_wrong_claim |
| `\runDryDevVariantHi` | 21.5 | data/training/runs3/dry_dev2/summary.json: 100 * by_kind.variant.ci95[1] |
| `\runDryDevVariantLo` | 0.0 | data/training/runs3/dry_dev2/summary.json: 100 * by_kind.variant.ci95[0] |
| `\runDryDevVariantN` | 14 | data/training/runs3/dry_dev2/summary.json: by_kind.variant.n |
| `\runDryDevVariantPass` | 0 | data/training/runs3/dry_dev2/summary.json: by_kind.variant.pass |
| `\runDryDevVariantRate` | 0.0 | data/training/runs3/dry_dev2/summary.json: 100 * by_kind.variant.rate |
| `\runDryDevVariantRecall` | 53.1 | data/training/runs3/dry_dev2/summary.json: 100 * by_kind.variant.mean_recall |
| `\runDryDevVariantWrong` | 8 | data/training/runs3/dry_dev2/summary.json: by_kind.variant.with_wrong_claim |
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
- `data/training/calls3/ledger.jsonl` adc1d511fbbf27fd99cc9c079279621950e19304d2017496d4d1c02ed3067979
- `data/training/curriculum/2026-11-17.jsonl` 28107ca48596fea6e75e7bcb0e0eb7c889dd3db7263d10385cbae609382b6826
- `data/training/curriculum/2026-11-18.jsonl` 7b380fcadec0a5860aee69348204ce51dcffb949be3b186d949944c046d65196
- `data/training/parser3/models/theta.json` 552164a82a87b79a54fb5b12b25a3535f5396a767f596c03a1f5546fcd5b7f49
- `data/training/runs3/dry_dev/summary.json` 96a281aefdb2beaec48d5fa09385683d2879747318b8832c644dec1e4fac183b
- `data/training/runs3/dry_dev2/summary.json` ef8ea3549f6b0a7aecd668f4190bbb394f61b5e3d7d2797ef72cc47f5a6da340
