# Synthetic teaching dialogue format

A teaching dialogue is a Markdown file in `data/synthetic/teaching/dev/` or
`data/synthetic/teaching/held_out/`. A teacher (playing the user, who is
**maksim**: "I" = maksim) teaches the agent and checks its answers. Every
check is exact, so the evaluation is repeatable.

```markdown
# Dialogue: Anna's breakfast
domain: food

## script
- Anna likes tea.
- Anna likes toast.
- Does Anna like tea? => yes
- Does Anna like coffee? => unknown
- What does Anna like? => contains: tea; toast
- If someone likes tea then they want tea.
- Does Anna want tea? => yes
- Why? => contains: because
- Anna doesn't like toast anymore.
- Does Anna like toast? => no
- Did Anna like toast? => contains: used to

## expect facts
- likes(anna, tea)
- -likes(anna, toast)

## expect rules
- wants(A, tea) :- likes(A, tea).

## ground truth
- likes(anna, tea)
- is_a(anna, person)
```

## Script lines

Each `- ` line is one message to the agent, sent in order. A line with
` => ` is a message plus a check on the agent's reply:

| Check | Passes when the reply... |
|---|---|
| `yes` | starts with "Yes" or "That's right" |
| `no` | starts with "No" |
| `unknown` | contains "don't know", "not sure" or "no idea" |
| `contains: a; b` | contains every listed text (case-insensitive) |
| `not: a; b` | contains none of the listed texts (case-insensitive) |

Lines without ` => ` are statements; their replies are not checked.

## Sections after the script

- `## expect facts`: Datalog atoms that must be true at the end (stored or
  derived). Negated atoms start with `-`.
- `## expect rules`: rules the agent must have stored, written exactly as the
  parser produces them (variable `A`, see `docs/parser_spec.md`).
- `## ground truth` (optional): the full world the teacher knows, as atoms.
  A live teacher answers the agent's curiosity questions only from these.

## What the agent understands

Only the sentence patterns in `docs/parser_spec.md`. Replies follow these forms:
- yes/no questions: "Yes, Anna likes tea." / "No, Anna does not like tea." /
  "I don't know whether ..." (also "I'm not sure whether", "I have no idea whether");
  past: "Yes, Anna used to like toast, but not anymore."
- wh-questions: the facts, e.g. "Anna likes tea and toast." or "I don't know what ...".
- why: "... because you told me." / "... because <reasons>, and if A ... then ...".
- statements: "Okay, I'll remember that. ..." / "Got it. ..." / "Noted. ...";
  a contradicted old fact: "So ... used to ... before, but not anymore.";
  rules: "I'll keep this as a rule: if A ... then A ...".
- corrections ("No, X." / "Actually, X." / "That's wrong."):
  "Thanks for the correction. ..." and, for a wrong conclusion from a rule,
  "I changed my rule to: ...".
- The agent may append one curiosity question to a reply ("I have conflicting
  information: ...", "I noticed a pattern. Is it true that ...?", "What is ...?").
  The next message answers it if it is "yes"/"no"/a statement.
