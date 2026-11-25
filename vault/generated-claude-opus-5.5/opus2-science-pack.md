# Science Knowledge Pack (`claude-opus-5.5` round 2)

Solar system, chemical elements, and cell biology facts that compose with the existing knowledge graph, containment, and `@collect` queries.

```karin
is_a(child="Mercury", parent="planet")
is_a(child="Venus", parent="planet")
is_a(child="Earth", parent="planet")
is_a(child="Mars", parent="planet")
is_a(child="Jupiter", parent="planet")
is_a(child="Saturn", parent="planet")
is_a(child="planet", parent="celestial body")
is_a(child="Sun", parent="star")
is_a(child="star", parent="celestial body")
rel(subj="Mercury", pred="orbits", obj="Sun")
rel(subj="Venus", pred="orbits", obj="Sun")
rel(subj="Earth", pred="orbits", obj="Sun")
rel(subj="Mars", pred="orbits", obj="Sun")
rel(subj="Moon", pred="orbits", obj="Earth")
prop(entity="Mars", attr="color", value="red")
prop(entity="Jupiter", attr="size", value="largest planet in the Solar System")
is_a(child="Oxygen", parent="chemical element")
is_a(child="Hydrogen", parent="chemical element")
is_a(child="Carbon", parent="chemical element")
prop(entity="Oxygen", attr="symbol", value="O")
prop(entity="Oxygen", attr="atomic number", value="8")
prop(entity="Hydrogen", attr="symbol", value="H")
prop(entity="Hydrogen", attr="atomic number", value="1")
prop(entity="Carbon", attr="symbol", value="C")
prop(entity="Carbon", attr="atomic number", value="6")
rel(subj="nucleus", pred="part_of", obj="cell")
rel(subj="mitochondrion", pred="part_of", obj="cell")
rel(subj="membrane", pred="part_of", obj="cell")
rel(subj="cell", pred="part_of", obj="tissue")
rel(subj="tissue", pred="part_of", obj="organ")

rule opus2-what-orbits
when: "what orbits $x", !collected(tag="opus2_orbit"), !collected_empty(tag="opus2_orbit")
then:
  +opus2_ctx(kind="orbit", key=$x)
  @collect(rel="rel", field="subj", tag="opus2_orbit", pred="orbits", obj=$x)

rule opus2-reply-orbits
when: collected(tag="opus2_orbit", list=$list), opus2_ctx(kind="orbit", key=$x)
then: => "Things orbiting $x: $list."

rule opus2-reply-orbits-empty
when: collected_empty(tag="opus2_orbit"), opus2_ctx(kind="orbit", key=$x)
then: => "I don't know anything that orbits $x."
```
