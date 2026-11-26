# Shopping List Workflow (`gpt-6.2-sol`)

Multi-turn `goal_need` workflow to capture an item and quantity, plus `@collect` to list all saved items.

```karin
goal_need(goal="sol_shopping_add", slot="sol_shopping_quantity", prompt="How much should I add?")
goal_need(goal="sol_shopping_add", slot="sol_shopping_item", prompt="What should I put on your shopping list?")

rule sol-start-shopping-entry
when: "start shopping entry", !dialogue_goal(), !dialogue_slot()
then: +dialogue_goal(goal="sol_shopping_add")

rule sol-finish-shopping-entry
when: dialogue_goal(goal="sol_shopping_add"), dialogue_filled(goal="sol_shopping_add", slot="sol_shopping_item", value=$item), dialogue_filled(goal="sol_shopping_add", slot="sol_shopping_quantity", value=$quantity)
then:
  -dialogue_goal(goal="sol_shopping_add")
  -dialogue_filled(goal="sol_shopping_add")
  ++sol_shopping_item(value="$item ($quantity)")
  => "Added to your shopping list: $item ($quantity)."

rule sol-collect-shopping-list
when: "show my shopping list", !dialogue_slot(), !collected(tag="sol_shopping"), !collected_empty(tag="sol_shopping")
then: @collect(rel="sol_shopping_item", field="value", tag="sol_shopping")

rule sol-reply-shopping-list
when: collected(tag="sol_shopping", list=$list, count=$count)
then: => "Shopping list ($count): $list"

rule sol-reply-shopping-list-empty
when: collected_empty(tag="sol_shopping")
then: => "Your shopping list is empty. Say \"start shopping entry\" to add something."
```
