# Data-Driven Unit Conversions

Each unit is declared once relative to its dimension's base unit. A single generic rule composes any compatible pair through `@calc`, giving N² conversions from N facts.

## Unit Table

```karin
unit_base(unit="mm", dim="length", to_base="0.001")
unit_base(unit="cm", dim="length", to_base="0.01")
unit_base(unit="m", dim="length", to_base="1")
unit_base(unit="km", dim="length", to_base="1000")
unit_base(unit="in", dim="length", to_base="0.0254")
unit_base(unit="ft", dim="length", to_base="0.3048")
unit_base(unit="mi", dim="length", to_base="1609.344")

unit_base(unit="mg", dim="mass", to_base="0.001")
unit_base(unit="g", dim="mass", to_base="1")
unit_base(unit="kg", dim="mass", to_base="1000")
unit_base(unit="lb", dim="mass", to_base="453.592")
unit_base(unit="oz", dim="mass", to_base="28.3495")

unit_base(unit="s", dim="time", to_base="1")
unit_base(unit="min", dim="time", to_base="60")
unit_base(unit="h", dim="time", to_base="3600")
unit_base(unit="day", dim="time", to_base="86400")
```

## Generic Conversion Rules

```karin
rule convert-units
when: intent(name="convert", value=$v, from=$from, to=$to), unit_base(unit=$from, dim=$dim, to_base=$f1), unit_base(unit=$to, dim=$dim, to_base=$f2), !converting()
then:
  +converting(value=$v, from=$from, to=$to)
  @calc(expr="$v * $f1 / $f2")

rule reply-conversion
when: converting(value=$v, from=$from, to=$to), calc_result(value=$res)
then: => "$v $from = $res $to"

rule reply-conversion-dim-mismatch
when: intent(name="convert", from=$from, to=$to), unit_base(unit=$from, dim=$d1), unit_base(unit=$to, dim=$d2), $d1 != $d2
then: => "Cannot convert $from ($d1) to $to ($d2) — incompatible dimensions."
```
