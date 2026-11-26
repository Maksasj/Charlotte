# Knowledge Graph Group Queries (`claude-opus-5.5` round 2)

Lists all members of a category, similar siblings, places in a region, creations, parts, and known attributes via `@collect`.

```karin
rule opus2-list-category
when: "list every $parent", !collected(tag="opus2_members"), !collected_empty(tag="opus2_members")
then:
  +opus2_ctx(kind="members", key=$parent)
  @collect(rel="is_a", field="child", tag="opus2_members", parent=$parent)

rule opus2-reply-category
when: collected(tag="opus2_members", count=$n, list=$list), opus2_ctx(kind="members", key=$p)
then: => "I know $n $p(s): $list."

rule opus2-reply-category-empty
when: collected_empty(tag="opus2_members"), opus2_ctx(kind="members", key=$p)
then: => "I don't know any $p yet. Teach me with \"X is a $p\"."

rule opus2-siblings
when: "things similar to $a", is_a(child=$a, parent=$p), !collected(tag="opus2_sib"), !collected_empty(tag="opus2_sib")
then:
  +opus2_ctx(kind="sib", key=$p, of=$a)
  @collect(rel="is_a", field="child", tag="opus2_sib", parent=$p)

rule opus2-reply-siblings
when: collected(tag="opus2_sib", list=$list), opus2_ctx(kind="sib", key=$p, of=$a)
then: => "$a is a $p, like: $list."

rule opus2-siblings-unknown
when: "things similar to $a", !is_a(child=$a)
then: => "I don't know what kind of thing $a is. Teach me with \"$a is a ...\"."

rule opus2-list-region
when: "list places within $region", !collected(tag="opus2_region"), !collected_empty(tag="opus2_region")
then:
  +opus2_ctx(kind="region", key=$region)
  @collect(rel="rel", field="subj", tag="opus2_region", pred="located_in", obj=$region)

rule opus2-reply-region
when: collected(tag="opus2_region", list=$list), opus2_ctx(kind="region", key=$r)
then: => "Places directly within $r: $list."

rule opus2-reply-region-empty
when: collected_empty(tag="opus2_region"), opus2_ctx(kind="region", key=$r)
then: => "I don't know any places within $r yet."

rule opus2-list-creations
when: "list everything created by $obj", !collected(tag="opus2_made"), !collected_empty(tag="opus2_made")
then:
  +opus2_ctx(kind="made", key=$obj)
  @collect(rel="rel", field="subj", tag="opus2_made", pred="created_by", obj=$obj)

rule opus2-reply-creations
when: collected(tag="opus2_made", list=$list), opus2_ctx(kind="made", key=$o)
then: => "$o created: $list."

rule opus2-reply-creations-empty
when: collected_empty(tag="opus2_made"), opus2_ctx(kind="made", key=$o)
then: => "I don't know anything created by $o."

rule opus2-list-parts
when: "list the parts of $whole", !collected(tag="opus2_parts"), !collected_empty(tag="opus2_parts")
then:
  +opus2_ctx(kind="parts", key=$whole)
  @collect(rel="rel", field="subj", tag="opus2_parts", pred="part_of", obj=$whole)

rule opus2-reply-parts
when: collected(tag="opus2_parts", list=$list), opus2_ctx(kind="parts", key=$w)
then: => "Parts of $w: $list."

rule opus2-reply-parts-empty
when: collected_empty(tag="opus2_parts"), opus2_ctx(kind="parts", key=$w)
then: => "I don't know the parts of $w yet."

rule opus2-list-attrs
when: "list facts about $e", !collected(tag="opus2_attrs"), !collected_empty(tag="opus2_attrs")
then:
  +opus2_ctx(kind="attrs", key=$e)
  @collect(rel="prop", field="attr", tag="opus2_attrs", entity=$e)

rule opus2-reply-attrs
when: collected(tag="opus2_attrs", list=$list), opus2_ctx(kind="attrs", key=$e)
then: => "I know these attributes of $e: $list. Ask \"what is the ATTR of $e\"."

rule opus2-reply-attrs-empty
when: collected_empty(tag="opus2_attrs"), opus2_ctx(kind="attrs", key=$e)
then: => "I don't have any attributes for $e yet."
```
