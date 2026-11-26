# Calculator

Parses arithmetic requests (`"calculate 6 * 7"`), invokes the pure-Rust `@calc` tool, and formats the result or error.

```karin
rule run-calc-tool
when: intent(name="calc", expr=$expr), !calc_result(), !calc_error()
then: @calc(expr=$expr)

rule reply-calc-result
when: calc_result(expr=$expr, value=$val), !converting()
then: => "$expr = $val"

rule reply-calc-error
when: calc_error(expr=$expr, error=$err)
then: => "Couldn't evaluate \"$expr\": $err."
```
