# Calculator

## parse-calculate
when: "calculate ?expr", !want_calc()
then: +want_calc(expr=?expr)

## run-calc-tool
when: want_calc(expr=?expr), !calc_result(), !calc_error()
then: @calc(expr=?expr)

## reply-calc-result
when: calc_result(expr=?expr, value=?val)
then: => "?expr = ?val"

## reply-calc-error
when: calc_error(expr=?expr, error=?err)
then: => "Couldn't evaluate \"?expr\": ?err."
