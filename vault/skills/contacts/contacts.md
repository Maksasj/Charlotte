# Personal Assistant Memory & Aliases (`claude-opus-5.5`)

Stores contacts directly in the knowledge graph (goals and projects moved to the entities pack, per speaker) (`prop`/`rel`/`is_a`) and adds natural phrasing aliases for canonical intents.

```karin
rule mem-save-contact
when: "save contact $name email $email"
then:
  ++is_a(child=$name, parent="contact")
  -prop(entity=$name, attr="email")
  ++prop(entity=$name, attr="email", value=$email)
  => "Saved contact $name ($email)."

```


## Listing contacts

```karin
rule opus2-list-contacts
when: "list my contacts", !collected(tag="opus2_contacts"), !collected_empty(tag="opus2_contacts")
then: @collect(rel="is_a", field="child", tag="opus2_contacts", parent="contact")

rule opus2-reply-contacts
when: collected(tag="opus2_contacts", list=$list)
then: => "Your contacts: $list."

rule opus2-reply-contacts-empty
when: collected_empty(tag="opus2_contacts")
then: => "No contacts yet. Say \"save contact NAME email EMAIL\"."
```
