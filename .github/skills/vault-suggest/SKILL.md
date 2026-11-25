---
name: vault-suggest
description: Read-only analysis of Karin's vault/core to propose and verify ready-to-paste Compact Rule DSL rules and facts without modifying any vault files. Use when asked to suggest, design, audit, or verify new vault rules and facts.
---

# Vault Suggest (Read-Only)

Analyze Karin's modular symbolic vault (`vault/core/`) and propose high-leverage, ready-to-paste Compact Rule DSL rules and facts — **strictly read-only**.

## Strict Read-Only Rule

- **NEVER** edit, create, delete, or overwrite any file in `vault/` or `software/`.
- Only read files and run read-only checks. Output proposed rules and facts directly in the response, ready for the user to paste.

## Step 1: Read the Current Brain & Engine Capabilities

1. List and read all `*.md` modules under `vault/core/` (`persona/`, `language/`, `dialogue/`, `knowledge/`, `skills/`, `learning/`).
2. Confirm supported DSL syntax and tools by checking:
   - `software/crates/karin-core/src/vault/syntax.rs` (supported conditions and actions)
   - `software/crates/karin-core/src/tools/mod.rs` (available `@tool` names and argument keys)
   - `software/crates/karin-core/src/engine/mod.rs` (turn retention rules, step limit `MAX_STEPS_PER_TURN`, scoring bonuses)

## Step 2: Design High-Leverage Suggestions

Prioritize **reusable, compositional additions** over one-off Q&A:

- Plug into existing canonical layers whenever possible:
  - `intent(name=..., ...)` in `vault/core/language/intents.md`
  - `need_slot` / `dialogue_slot` / `slot_value` in `vault/core/dialogue/frames.md`
  - `goal_need` / `dialogue_goal` / `dialogue_filled` in `vault/core/dialogue/workflows.md`
  - `is_a`, `prop`, `rel`, `inverse` in `vault/core/knowledge/`
  - `unit_base` + `@calc` in `vault/core/skills/units.md`
- Respect any user focus area (e.g., UX, knowledge depth, workflows, phrasing coverage, domain pack).

## Step 3: Verify Every Proposed Rule & Fact (Read-Only Checklist)

Before outputting any snippet, verify every rule and fact against the current vault and engine:

1. **DSL Syntax**
   - Inside a ` ```karin ` fenced block (all prose outside fences is ignored).
   - Variables use `$var` (never `?var`).
   - Facts: `rel(k="v", ...)` (long-term by default in vault files).
   - Rules: `rule <unique-kebab-name>`, `when: <comma-separated conditions>`, `then:` (inline single action or indented multi-line actions).
   - Valid conditions only: `"template"`, `~"phrase"`, `rel(k=v)`, `!rel(k=v)`, `$a != $b`.
   - Valid actions only: `+rel(...)` (working), `++rel(...)` (long-term), `-rel` / `-rel(...)` (retract), `@tool(...)`, `=> "..."` (reply).
   - All referenced `@tool(...)` names and argument keys exist in `karin-core/src/tools/`.

2. **Loop & Step-Limit Safety**
   - **No unguarded long-term-only rules** (e.g., never fire on long-term `is_a`/`rel` facts without a working query/intent gate, or they will run on every unrelated turn).
   - Every intermediate rule has a negative guard (`!intent(...)`, `!need_slot(...)`, `!slot_value(...)`, `!visited_*(...)`) preventing re-firing.
   - Multi-hop / multi-step turns stay well under `MAX_STEPS_PER_TURN` (12).

3. **Collision & Priority Check**
   - Rule names do not duplicate any existing `rule <name>` in `vault/core/`.
   - Text templates do not shadow or collide with existing patterns (watch out for broad templates like `"what is $x"`, `"$a is $b"`, `"$x in $y"`).
   - Remember `=> "..."` (`Action::Reply`) gets a `+2.5` score bonus — guard fallback reply rules (e.g., `!is_a(child=$cur)`) so they cannot preempt multi-hop or slot-capture rules.
   - Remember `unify.rs` matches facts newest-first (`facts.iter().rev()`) — check ordering when declaring multiple `goal_need` facts for one workflow.

4. **Cross-Turn State Check**
   - Only `Scope::LongTerm` (`++`) and `dialogue_*` working facts survive across turns.
   - Multi-turn workflows clean up `-dialogue_goal(...)` and `-dialogue_filled(...)` on completion (and rely on `dialogue-cancel-slot` on `"cancel"`).

## Step 4: Output Format

Keep output concise, scannable, and immediately copy-pasteable:

For each proposed addition:
1. **Target file** (e.g., `vault/core/knowledge/relations.md`) + 1-line rationale.
2. **Ready-to-paste ` ```karin ` block** containing only valid facts/rules.
3. **Verification:** Short pass/fail checklist (syntax, unique names, collisions, loop safety, tool/state fit) + 1–2 example chat turns showing the expected trace.
