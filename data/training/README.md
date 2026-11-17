# Training data archive

Every real conversation the project generates is kept, never deleted, so it
can be reused later (learned input patterns, judge calibration, regression
sets, teacher-quality checks).

| Folder | What | Written by |
|---|---|---|
| `trainer/<date>.jsonl` | every finished training conversation: scenario, full transcript with outcome labels, dialogue acts and moves, facts and rules stored, reviewer rejections | `sla_trainer` (runs on mock models are synthetic and not kept) |
| `judges/<date>.jsonl` | every judge chat with its score and reason | `sla_eval` (full evaluation) |

Training data from before this archive existed, all kept in the repository:

| Where | What |
|---|---|
| `eval/trainer_snapshot.json` | the v1 training-app run: 8 help-scenario conversations, 68 agent turns |
| `eval/pretrain/` | v1 pretraining: structured lessons and friend chats (`*_conversations.md`) |
| `eval/reports/`, `eval/reports_pretrained_vault/` | v1 judge chats with scores |
| `eval/outcomes/` | per-turn outcome labels of every outcome run |
| `eval/label_cache.jsonl` | Haiku judge labels |
| `data/regression/` | the 296 v1 real turns and hand labels |
| `research/vault_before_reset_2026-11-16.zip` | the whole v1 vault, its log of every turn and the v1 translation cache |

Conversations held in the CLI are traced to `sessions/` beside the vault
(kept on disk, not in git, since they can hold personal facts).
