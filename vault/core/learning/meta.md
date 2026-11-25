# Self-Improvement & Meta-Tools

## meta-generalize-command
when: "generalize", !meta_result()
then: @generalize_episodes()

## meta-chunk-command
when: "chunk last", !meta_result()
then: @chunk_last_chain()

## reply-meta-result
when: meta_result(summary=?summary)
then: => "?summary"

# Seed Episodes

"greet Alice" => "Warm greetings, Alice! Glad you're here."
"greet Bob" => "Warm greetings, Bob! Glad you're here."
"praise Rust" => "Rust is genuinely impressive — clean, fast, and reliable."
"praise Prolog" => "Prolog is genuinely impressive — clean, fast, and reliable."
