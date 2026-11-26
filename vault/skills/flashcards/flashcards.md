# Flashcard Study & Quiz (`claude-opus-5.5`)

Save flashcards via the shared workflow engine and quiz yourself with automatic answer checking via `dialogue_quiz` + `need_slot`.

```karin
goal_need(goal="flashcard", slot="answer", prompt="And the answer?")
goal_need(goal="flashcard", slot="question", prompt="What's the question?")

rule wf-flashcard-start
when: "add a flashcard", !dialogue_goal()
then: +dialogue_goal(goal="flashcard")

rule wf-flashcard-finish
when: dialogue_goal(goal="flashcard"), dialogue_filled(goal="flashcard", slot="question", value=$q), dialogue_filled(goal="flashcard", slot="answer", value=$a)
then:
  -dialogue_goal(goal="flashcard")
  -dialogue_filled(goal="flashcard")
  ++flashcard(q=$q, a=$a)
  => "Flashcard saved: \"$q\"."

rule quiz-start
when: "quiz me", flashcard(q=$q, a=$a), !dialogue_quiz(), !dialogue_goal()
then:
  +dialogue_quiz(q=$q, a=$a)
  +need_slot(goal="quiz", slot="answer", prompt=$q)

rule quiz-start-empty
when: "quiz me", !flashcard()
then: => "No flashcards yet. Say \"add a flashcard\" first."

rule quiz-cancel
when: dialogue_quiz(), dialogue_slot(goal="quiz", slot="answer"), "cancel"
then:
  -dialogue_quiz
  -dialogue_slot(goal="quiz", slot="answer")
  => "Okay, quiz cancelled."

rule quiz-correct
when: slot_value(goal="quiz", slot="answer", value=$v), dialogue_quiz(a=$v)
then:
  -dialogue_quiz
  -slot_value(goal="quiz")
  => "Correct! 🎉"

rule quiz-wrong
when: slot_value(goal="quiz", slot="answer", value=$v), dialogue_quiz(a=$a), $v != $a
then:
  -dialogue_quiz
  -slot_value(goal="quiz")
  => "Not quite - the answer is $a."
```


## Listing flashcards

```karin
rule opus2-list-flashcards
when: "list my flashcards", !collected(tag="opus2_cards"), !collected_empty(tag="opus2_cards")
then: @collect(rel="flashcard", field="q", tag="opus2_cards", sep=" | ")

rule opus2-reply-flashcards
when: collected(tag="opus2_cards", count=$n, list=$list)
then: => "$n flashcard(s): $list"

rule opus2-reply-flashcards-empty
when: collected_empty(tag="opus2_cards")
then: => "No flashcards yet. Say \"add a flashcard\" to create one."
```
