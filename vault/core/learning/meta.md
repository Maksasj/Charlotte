# Self-Improvement & Meta-Tools

Commands that invoke Karin's own learning tools: anti-unification over stored episodes (`@generalize_episodes`) and chain chunking (`@chunk_last_chain`).

## Meta-Tool Rules

```karin
rule meta-generalize-command
when: "generalize", !meta_result()
then: @generalize_episodes()

rule meta-chunk-command
when: "chunk last", !meta_result()
then: @chunk_last_chain()

rule reply-meta-result
when: meta_result(summary=?summary)
then: => "?summary"
```

## Seed Episodes

Concrete examples used to test and bootstrap anti-unification.

```karin
"greet Alice" => "Warm greetings, Alice! Glad you're here."
"greet Bob" => "Warm greetings, Bob! Glad you're here."
"praise Rust" => "Rust is genuinely impressive — clean, fast, and reliable."
"praise Prolog" => "Prolog is genuinely impressive — clean, fast, and reliable."
```
