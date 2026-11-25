# Everyday Formulas & Temperature (`claude-opus-5.5`)

Adds percentage, tip, discount, bill split, travel time/speed, and affine temperature conversions on top of `@calc` and canonical `intent(name="convert")`.

```karin
temp_formula(from="c", to="f", mul="1.8", add="32")
temp_formula(from="f", to="c", mul="0.5555555556", add="-17.7777777778")
temp_formula(from="c", to="k", mul="1", add="273.15")
temp_formula(from="k", to="c", mul="1", add="-273.15")

rule formula-temp-convert
when: intent(name="convert", value=$v, from=$from, to=$to), temp_formula(from=$from, to=$to, mul=$m, add=$a), !converting()
then:
  +converting(value=$v, from=$from, to=$to)
  @calc(expr="$v * $m + $a")

rule formula-percent-of
when: "what is $p percent of $n", !formula_calc()
then:
  +formula_calc(kind="percent", p=$p, n=$n)
  @calc(expr="$p * $n / 100")

rule formula-tip
when: "tip $p percent on $amount", !formula_calc()
then:
  +formula_calc(kind="tip", p=$p, n=$amount)
  @calc(expr="$amount + $amount * $p / 100")

rule formula-discount
when: "discount $p percent off $amount", !formula_calc()
then:
  +formula_calc(kind="discount", p=$p, n=$amount)
  @calc(expr="$amount - $amount * $p / 100")

rule formula-split
when: "split $amount between $people", !formula_calc()
then:
  +formula_calc(kind="split", p=$people, n=$amount)
  @calc(expr="$amount / $people")

rule formula-travel-time
when: "travel time for $d km at $s kmh", !formula_calc()
then:
  +formula_calc(kind="travel_time", p=$s, n=$d)
  @calc(expr="$d / $s")

rule formula-speed
when: "speed for $d km in $t hours", !formula_calc()
then:
  +formula_calc(kind="speed", p=$t, n=$d)
  @calc(expr="$d / $t")

rule reply-formula-percent
when: formula_calc(kind="percent", p=$p, n=$n), calc_result(value=$r)
then: => "$p% of $n is $r."

rule reply-formula-tip
when: formula_calc(kind="tip", p=$p, n=$n), calc_result(value=$r)
then: => "With a $p% tip, $n comes to $r in total."

rule reply-formula-discount
when: formula_calc(kind="discount", p=$p, n=$n), calc_result(value=$r)
then: => "$n minus $p% is $r."

rule reply-formula-split
when: formula_calc(kind="split", p=$p, n=$n), calc_result(value=$r)
then: => "$n split between $p is $r each."

rule reply-formula-travel-time
when: formula_calc(kind="travel_time", p=$s, n=$d), calc_result(value=$r)
then: => "$d km at $s km/h takes $r hours."

rule reply-formula-speed
when: formula_calc(kind="speed", p=$t, n=$d), calc_result(value=$r)
then: => "$d km in $t hours is an average of $r km/h."
```
