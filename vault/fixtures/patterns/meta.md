# Meta-Tools

## p27: meta-generalize-command
- status: active
- kind: reasoning
- trust: 0.95
- support: 3
- provenance: seed:meta

### when
- text: generalize
- not_fact: meta_result()

### then
- tool: generalize_episodes()

## p28: meta-chunk-command
- status: active
- kind: reasoning
- trust: 0.95
- support: 3
- provenance: seed:meta

### when
- text: chunk last
- not_fact: meta_result()

### then
- tool: chunk_last_chain()

## p29: reply-meta-result
- status: active
- kind: reply
- trust: 0.95
- support: 3
- provenance: seed:meta

### when
- fact: meta_result(summary=?summary)

### then
- reply: ?summary
