# Data-Driven Planar Areas (`gpt-6.2-sol`)

Single generic area rule driven by `sol_area_factor` facts and `@calc` — adding a shape takes one fact.

```karin
sol_area_factor(shape="rectangle", factor="1")
sol_area_factor(shape="triangle", factor="0.5")
sol_area_factor(shape="ellipse", factor="0.7853981633974483")

rule sol-run-planar-area
when: "area pack $shape width $w height $h", sol_area_factor(shape=$shape, factor=$factor), !dialogue_slot(), !calc_result(), !calc_error()
then: @calc(expr="$w * $h * $factor")
```
