# Evaluation report 2026-11-16T19:51:19

commit `e7f9705`

## Knowledge and reasoning

- parser exact accuracy: dev 100.0%, held-out 98.0%
- teaching dialogues (dev): checks 96/96, knowledge score 100.0%
- teaching dialogues (held-out): checks 45/47, knowledge score 97.8%
- live teacher: 0 questions asked, 0 answers learned without contradicting the ground truth

### Dialogues: dev

- **Maksim's drinks**: checks 7/7, facts 3/3, rules 0/0
- **Birds and Pingu**: checks 9/9, facts 6/6, rules 1/1
- **Hospital staff**: checks 7/7, facts 6/6, rules 2/2
- **Rex the dog**: checks 7/7, facts 5/5, rules 1/1
- **Chess and guitar**: checks 9/9, facts 4/4, rules 1/1
- **Anna's family**: checks 7/7, facts 7/7, rules 1/1
- **Vehicles and fuel**: checks 9/9, facts 7/7, rules 0/0
- **Cities and moves**: checks 10/10, facts 7/7, rules 1/1
- **Workshop tools**: checks 8/8, facts 4/4, rules 1/1
- **Party and concert**: checks 7/7, facts 7/7, rules 1/1
- **Lena changes jobs**: checks 8/8, facts 6/6, rules 1/1
- **Fruit and vegetables**: checks 8/8, facts 6/6, rules 1/1

### Dialogues: held-out

- **Kofi switches sports**: checks 8/8, facts 5/5, rules 1/1
- **Mammals and Willy**: checks 8/8, facts 6/6, rules 1/1
- **Campus people**: checks 7/7, facts 7/7, rules 2/2
- **Priya's scooter**: checks 8/8, facts 4/4, rules 1/1
- **The chefs' kitchen**: checks 6/8, facts 7/7, rules 0/0
  - 2 failure(s) (details hidden for the held-out set)
- **Zara and Hugo**: checks 8/8, facts 7/7, rules 1/1

## Conversation quality

Average judge score: **3.88 / 10** from 8 judge(s).

### Judge: your family and what they like to eat (score 3)

It acknowledges facts, but it garbles them ("apple" for "apples", "Dad's likes is bread", lowercase "lena"), fails the simple question "Does Mom like apples?", and claims "I knew that already" about a fact the user had just stated.

> **User:** Hi! My family is Mom, Dad, and my little sister Lena. Mom likes apples.  
> **Agent:** Okay, I'll remember that. Your family is your family. Your family has Mom, Dad and your little sister lena.

> **User:** Mom likes apples. Dad likes bread.  
> **Agent:** Got it. Mom likes apple. Dad likes bread.

> **User:** Dad also likes bread, and Lena is my little sister.  
> **Agent:** Noted. Your sister is Lena.

> **User:** Dad likes bread. Does Mom like apples?  
> **Agent:** Sorry, I didn't understand that.

> **User:** Dad likes bread, and Lena is my little sister.  
> **Agent:** Got it. Dad's likes is bread.

> **User:** Dad likes bread, and Mom likes apples too. Does Dad like bread?  
> **Agent:** Right, I knew that already. Yes, Dad likes bread.

### Judge: pets and animals you know (score 4)

The bot handles the first few lines reasonably but garbles them: "Luna likes on my lap" and "Luna sits on your lap evening" drop key words and change meaning, and it then fails on a sentence it could have simplified rather than asking for simpler wording.

> **User:** Hi! I'm Maksim. I have a cat named Luna.  
> **Agent:** Okay, I'll remember that. Your cat is Luna.

> **User:** Luna is a cat.  
> **Agent:** Got it. Luna is a cat.

> **User:** Luna is a cat, and she is my pet.  
> **Agent:** Noted. Your pet is Luna.

> **User:** Luna likes to sit on my lap.  
> **Agent:** Okay, I'll remember that. Luna likes on my lap.

> **User:** Luna sits on my lap every evening.  
> **Agent:** Got it. Luna sits on your lap evening.

> **User:** Luna likes to sit in my lap, and she purrs when I pet her.  
> **Agent:** Could you say that more simply?

### Judge: your city, your work and your colleagues (score 6)

The bot's replies are grammatical and mostly relevant, but it drops key facts (the nurse job, "Tom is a doctor" becomes "works at Hospital"), misreads "Anna likes her job" as "Anna likes job", and tacks on incorrect statements like "Your colleague is in hospital."

> **User:** Hi! My name is Maksim. I work in Berlin, and my job is nurse.  
> **Agent:** Okay, I'll remember that. You work in Berlin. Your name is Maksim. Your job is nurse.

> **User:** Maksim: My colleagues at the hospital are very kind, and my friend Anna lives in Berlin too.  
> **Agent:** Got it. Your colleague is in hospital. Your colleague is kind. Your friend is Anna. Anna lives in Berlin.

> **User:** Maksim: My job is nurse, and I like my work at the hospital.  
> **Agent:** Noted. You like your work and work at Hospital.

> **User:** Maksim: My colleague Tom works in the hospital with me, and he is a doctor.  
> **Agent:** Okay, I'll remember that. Tom works at Hospital and is a doctor.

> **User:** Maksim: My job is nurse, and I work in Berlin.  
> **Agent:** Got it. You live in Berlin.

> **User:** Anna likes her job, but I work at the hospital with Tom.  
> **Agent:** Noted. Anna likes job. You have job.

### Judge: hobbies and sports of your friends (score 3)

The early replies are fine, but it dropped "on Sundays" in one turn, produced the garbled "Anna plays tennis and plays with Ben," and answered "Ben is Anna's friend, right?" with the reversed "Ben's friend is Anna" instead of a yes.

> **User:** Anna likes tea.  
> **Agent:** Okay, I'll remember that. Anna likes tea.

> **User:** Anna likes tennis.  
> **Agent:** Got it. Anna likes tennis.

> **User:** Anna plays tennis on Sundays.  
> **Agent:** Noted. Anna plays tennis on sundays.

> **User:** Anna's tennis game on Sundays is fun, so does Anna like to play tennis with friends?  
> **Agent:** Yes, Anna likes tennis.

> **User:** Anna plays tennis with her friend Ben on Sundays.  
> **Agent:** Got it. Anna plays tennis and plays with Ben.

> **User:** Ben is Anna's friend, right?  
> **Agent:** Noted. Ben's friend is Anna.

### Judge: tools in a workshop and what they are for (score 3)

The bot stays on topic and admits what it doesn't know, but it drops facts it was taught (the hammer/nails example), answers "Could you say it with simple words?" with a non-answer ("I think so too"), and produces ungrammatical lines like "Workshop tool has job."

> **User:** Anna likes tea.  
> **Agent:** Okay, I'll remember that. Anna likes tea.

> **User:** Tools in a workshop are used for specific jobs.  
> **Agent:** Got it. Tool is a workshop tool.

> **User:** Workshop tools are for jobs, like a hammer for nails.  
> **Agent:** Could you say that more simply?

> **User:** Could you say it with simple words?  
> **Agent:** I think so too. What else would you like to talk about?

> **User:** Workshop tools have a job, like a hammer hits nails.  
> **Agent:** Okay, I'll remember that. Workshop tool has job.

> **User:** What does a hammer hit?  
> **Agent:** I don't know what hammer hits yet.

### Judge: animals and nature (score 3)

The agent misspells "wolf" as "wolve" throughout, leaks rule notation ("if A is a wolve then A hunts in pack"), gives ungrammatical replies like "Wolve is in forest," and its final answer contradicts its earlier wording.

> **User:** Hi there! I'm Maksim, and I want to teach you a few things about animals and nature. Let's start: Bears are animals.  
> **Agent:** Okay, I'll remember that. Bear is an animal.

> **User:** Anna, I know. Bears are big animals.  
> **Agent:** Got it. Bear is a big animal.

> **User:** Wolves live in forests.  
> **Agent:** Noted. Wolve is in forest.

> **User:** Wolves hunt in packs.  
> **Agent:** I'll keep this as a rule: if A is a wolve then A hunts in pack.

> **User:** What do you know about wolves?  
> **Agent:** Here is what I know about wolve: Wolve is in Forest.

> **User:** Wolves are carnivores, so they eat meat.  
> **Agent:** Got it. Wolf is a carnivore and eats meat.

### Judge: food and cooking (score 4)

The bot handles simple facts reasonably, but "Carrot goods for eye" and "Right, I knew that already" are ungrammatical and wrong, and it misstates the user's statements ("You like Cooking", "You like cook") and ignores the repeated input's context.

> **User:** Maksim: Cooking is my favorite thing to do. Carrots are orange.  
> **Agent:** Okay, I'll remember that. You like Cooking. Carrot is orange.

> **User:** Carrots are orange, and I like to cook them with onions.  
> **Agent:** Got it. You like cook.

> **User:** Carrots are a root vegetable.  
> **Agent:** Noted. Carrot is a root vegetable.

> **User:** Carrots are good for your eyes.  
> **Agent:** Okay, I'll remember that. Carrot is good.

> **User:** Carrots are good for your eyes, and they also taste sweet when you roast them.  
> **Agent:** Got it. Carrot goods for eye.

> **User:** Carrots are good for your eyes, and they also taste sweet when you roast them.  
> **Agent:** Right, I knew that already.

### Judge: weather and the seasons (score 5)

The replies are grammatically clean and mostly sensible, but "Glad I could help" answers a non-thanks, "Weather whens sunny" and "Your weather is sunny" are broken, and the bot misreads "I'm telling you the weather is a sunny day" as the user being excited and wanting to go outside, so it loses track of who said what.

> **User:** Hi! I'm Maksim. Anna likes tea.  
> **Agent:** Okay, I'll remember that. Anna likes tea.

> **User:** Anna lives in Berlin.  
> **Agent:** Got it. Anna lives in Berlin.

> **User:** Anna has a job too, right? What is her job?  
> **Agent:** Nobody has told me what Anna's job is, so I don't know.

> **User:** Okay, thanks. Anna also has a dog named Rex.  
> **Agent:** Glad I could help. Okay, I'll remember that. Anna has Rex. Rex is a dog.

> **User:** Anna likes weather when it's sunny.  
> **Agent:** Got it. Anna likes weather. Weather whens sunny.

> **User:** Maksim here, and I'm telling you the weather is a sunny day, and sunny weather usually makes me want to go outside.  
> **Agent:** Noted. You are excited and want go outside. Your weather is sunny.

