# Calculator

## p16: parse-calculate
- status: active
- kind: language
- trust: 0.92
- support: 5
- provenance: seed

### when
- text: calculate ?expr
- not_fact: want_calc()

### then
- assert[working]: want_calc(expr=?expr)

## p17: run-calc-tool
- status: active
- kind: reasoning
- trust: 0.92
- support: 5
- provenance: seed

### when
- fact: want_calc(expr=?expr)
- not_fact: calc_result()
- not_fact: calc_error()

### then
- tool: calc(expr=?expr)

## p18: reply-calc-result
- status: active
- kind: reply
- trust: 0.92
- support: 5
- provenance: seed

### when
- fact: calc_result(expr=?expr, value=?val)

### then
- reply: ?expr = ?val

## p19: reply-calc-error
- status: active
- kind: reply
- trust: 0.9
- support: 3
- provenance: seed

### when
- fact: calc_error(error=?err, expr=?expr)

### then
- reply: Couldn't evaluate "?expr": ?err.
