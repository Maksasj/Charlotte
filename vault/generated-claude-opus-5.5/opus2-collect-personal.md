# Personal Lists via `@collect` (`claude-opus-5.5` round 2)

Lists all saved reminders, meetings, meetings on a specific day, projects, flashcards, and contacts using `@collect`.

```karin
rule opus2-list-reminders
when: "list all my reminders", !collected(tag="opus2_rem"), !collected_empty(tag="opus2_rem")
then: @collect(rel="reminder", field="task", tag="opus2_rem")

rule opus2-reply-reminders
when: collected(tag="opus2_rem", count=$n, list=$list)
then: => "You have $n reminder(s): $list."

rule opus2-reply-reminders-empty
when: collected_empty(tag="opus2_rem")
then: => "No reminders saved. Say \"remind me to ...\" to add one."

rule opus2-list-meetings
when: "list all my meetings", !collected(tag="opus2_meet"), !collected_empty(tag="opus2_meet")
then: @collect(rel="meeting", field="who", tag="opus2_meet")

rule opus2-list-meetings-day
when: "meetings on $day", !collected(tag="opus2_meet_day"), !collected_empty(tag="opus2_meet_day")
then:
  +opus2_ctx(kind="meet_day", day=$day)
  @collect(rel="meeting", field="who", tag="opus2_meet_day", day=$day)

rule opus2-reply-meetings
when: collected(tag="opus2_meet", count=$n, list=$list)
then: => "You have $n meeting(s), with: $list."

rule opus2-reply-meetings-day
when: collected(tag="opus2_meet_day", list=$list), opus2_ctx(kind="meet_day", day=$day)
then: => "On $day you meet: $list."

rule opus2-reply-meetings-empty
when: collected_empty(tag="opus2_meet")
then: => "No meetings yet. Say \"schedule a meeting\" to book one."

rule opus2-reply-meetings-day-empty
when: collected_empty(tag="opus2_meet_day"), opus2_ctx(kind="meet_day", day=$day)
then: => "Nothing scheduled on $day."

rule opus2-list-projects
when: "list my projects", !collected(tag="opus2_proj"), !collected_empty(tag="opus2_proj")
then: @collect(rel="rel", field="obj", tag="opus2_proj", subj="user", pred="works_on")

rule opus2-reply-projects
when: collected(tag="opus2_proj", list=$list)
then: => "Your projects: $list."

rule opus2-reply-projects-empty
when: collected_empty(tag="opus2_proj")
then: => "No projects yet. Say \"I am working on ...\"."

rule opus2-list-flashcards
when: "list my flashcards", !collected(tag="opus2_cards"), !collected_empty(tag="opus2_cards")
then: @collect(rel="flashcard", field="q", tag="opus2_cards", sep=" | ")

rule opus2-reply-flashcards
when: collected(tag="opus2_cards", count=$n, list=$list)
then: => "$n flashcard(s): $list"

rule opus2-reply-flashcards-empty
when: collected_empty(tag="opus2_cards")
then: => "No flashcards yet. Say \"add a flashcard\" to create one."

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
