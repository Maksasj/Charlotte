# Meta-Tools

## meta-generalize-command
when: "generalize", !meta_result()
then: @generalize_episodes()

## meta-chunk-command
when: "chunk last", !meta_result()
then: @chunk_last_chain()

## reply-meta-result
when: meta_result(summary=?summary)
then: => "?summary"
