# Personal Assistant Memory & Aliases (`claude-opus-5.5`)

Stores goals, projects, and contacts directly in the knowledge graph (`prop`/`rel`/`is_a`) and adds natural phrasing aliases for canonical intents.

```karin
rule mem-set-goal
when: "my goal is $goal"
then:
  -prop(entity="user", attr="goal")
  ++prop(entity="user", attr="goal", value=$goal)
  => "Goal saved: $goal. I'll keep it in mind."

rule mem-recall-goal
when: "what is my goal", prop(entity="user", attr="goal", value=$goal)
then: => "Your goal is: $goal."

rule mem-recall-goal-unknown
when: "what is my goal", !prop(entity="user", attr="goal")
then: => "You haven't set a goal yet. Say \"my goal is ...\"."

rule mem-working-on
when: "I am working on $project"
then:
  ++rel(subj="user", pred="works_on", obj=$project)
  => "Noted - you're working on $project."

rule mem-recall-projects
when: "what am I working on", rel(subj="user", pred="works_on", obj=$p)
then: => "Most recently you told me you're working on $p."

rule mem-recall-projects-empty
when: "what am I working on", !rel(subj="user", pred="works_on")
then: => "No projects yet. Say \"I am working on ...\"."

rule mem-save-contact
when: "save contact $name email $email"
then:
  ++is_a(child=$name, parent="contact")
  -prop(entity=$name, attr="email")
  ++prop(entity=$name, attr="email", value=$email)
  => "Saved contact $name ($email)."

rule alias-calc-how-much
when: "how much is $expr", !intent(name="calc")
then: +intent(name="calc", expr=$expr)

rule alias-define-tell-me
when: "tell me about $topic", !intent(name="define")
then: +intent(name="define", topic=$topic)

rule alias-weather-raining
when: "is it raining in $city", !intent(name="weather")
then: +intent(name="weather", city=$city)

rule alias-convert-into
when: "convert $value $from into $to", !intent(name="convert")
then: +intent(name="convert", value=$value, from=$from, to=$to)

rule alias-cancel-never-mind
when: dialogue_slot(goal=$goal, slot=$slot), "never mind"
then:
  -dialogue_slot(goal=$goal, slot=$slot)
  -dialogue_goal(goal=$goal)
  -dialogue_filled(goal=$goal)
  -dialogue_quiz
  => "No problem, dropped it."

rule social-how-are-you
when: "how are you", persona_name(value=$me)
then: => "Running smoothly - every fact in place. How can I help?"
```
