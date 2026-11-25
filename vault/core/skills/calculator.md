# Calculator

Parses arithmetic requests (`"calculate 6 * 7"`), invokes the pure-Rust `@calc` tool, and formats the result or error.

```karin
rule parse-calculate
when: "calculate $expr", !want_calc()
then: +want_calc(expr=$expr)

rule run-calc-tool
when: want_calc(expr=$expr), !calc_result(), !calc_error()
then: @calc(expr=$expr)

rule reply-calc-result
when: calc_result(expr=$expr, value=$val)
then: => "$expr = $val"

rule reply-calc-error
when: calc_error(expr=$expr, error=$err)
then: => "Couldn't evaluate \"$expr\": $err."
```
