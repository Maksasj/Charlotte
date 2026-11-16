# Evaluation report 2026-11-16T19:42:56

commit `40354cd`

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

Average judge score: **4.25 / 10** from 8 judge(s).

### Judge: your family and what they like to eat (score 5)

The replies are mostly on topic and consistent, but they have awkward grammar ("Anna likes apple", "whether your dad's favorite fruit is bananas right") and the repeated "The rest I already knew" sounds robotic.

> **User:** Hi! Today I want to tell you about my family and what they like to eat. My mom is Anna.  
> **Agent:** Hello! Got it. Your mom is Anna.

> **User:** Anna likes apples.  
> **Agent:** Noted. Anna likes apple.

> **User:** Anna likes apples, and so does my dad, but he also likes bananas.  
> **Agent:** Okay, I'll remember that. Your dad likes apple and banana. The rest I already knew.

> **User:** Dad's name is Mike, and he likes apples too.  
> **Agent:** Got it. Your dad's name is Mike. The rest I already knew.

> **User:** Dad's job is teacher.  
> **Agent:** Noted. Dad's job is teacher.

> **User:** Your dad's favorite fruit is bananas, right?  
> **Agent:** I don't know whether your dad's favorite fruit is bananas right.

### Judge: pets and animals you know (score 6)

The replies are grammatical and mostly relevant, but they are robotic and inconsistent (a formal rule restatement, a stray "Okay." at the end, and a curt "I know, you told me before").

> **User:** Hi! I want to teach you a few things about pets. Dogs are animals.  
> **Agent:** Hello! Got it. Dog is an animal.

> **User:** Dogs are good pets for a family.  
> **Agent:** Noted. Dog is a pet.

> **User:** Dogs can bark at night.  
> **Agent:** I'll keep this as a rule: if A is a dog then A barks at night.

> **User:** Cats are pets too.  
> **Agent:** Okay, I'll remember that. Cat is a pet.

> **User:** Rex is a dog, and dogs bark at night, so Rex barks at night.  
> **Agent:** Got it. Rex is a dog and barks at night. Okay.

> **User:** Rex is a dog.  
> **Agent:** I know, you told me before.

### Judge: your city, your work and your colleagues (score 6)

The bot handled the basic facts and the simple answer correctly, but its "Anna is a colleague of maksim" lowercased the user's name, repeated itself awkwardly, and it answered two of the user's questions with "I'm not sure what you mean" and "Could you say that more simply?" even though the question was clear.

> **User:** Hi! I'm Maksim. I live in Berlin and I work as a nurse at a hospital.  
> **Agent:** Hello! Got it. You are in Berlin, are a nurse and work at hospital.

> **User:** Hmm, I'm not sure what you want me to output. The prompt sets up a role-play where I write Maksim's next chat message to a learning chatbot, and the chatbot's reply so far ("Got it. You are in Berlin, are a nurse and work at hospital.") suggests it's the bot's turn to learn facts. That part is simple enough to do.  
> **Agent:** Could you say that more simply?

> **User:** Sorry, I'll say it simpler: I am a nurse. I work at a hospital in Berlin.  
> **Agent:** Okay, I'll remember that. Hospital is in Berlin. The rest I already knew.

> **User:** Got it. Do you know any nurses who work with me at the hospital?  
> **Agent:** I'm not sure what you mean.

> **User:** Do you know any nurses who work with me at the hospital?  
> **Agent:** Could you say that more simply?

> **User:** Anna is my colleague.  
> **Agent:** Okay, I'll remember that. Anna is a colleague of maksim. Got it. Your colleague is Anna.

### Judge: hobbies and sports of your friends (score 3)

The replies are often ungrammatical or inaccurate ("Anna likes play," "plays at club"), they drop key facts like tennis, weekends, and Wednesdays, and they wrongly claim "the rest I already knew" when new information was given.

> **User:** Maksim: Hi. My friend Anna likes tennis.  
> **Agent:** Hello! Got it. Your friend is Anna. Anna likes tennis.

> **User:** Anna plays tennis every Saturday.  
> **Agent:** Noted. Anna plays tennis saturday.

> **User:** Anna plays tennis every Saturday, and she also likes it a lot.  
> **Agent:** Okay, I'll remember that. Anna plays tennis. The rest I already knew.

> **User:** Anna plays tennis with me on Saturdays.  
> **Agent:** Got it. Anna plays with you. The rest I already knew.

> **User:** Anna also plays tennis with Maksim, and she likes to play with him on weekends.  
> **Agent:** Noted. Anna likes play. The rest I already knew.

> **User:** Anna plays tennis at the club near my house, and she trains there on Wednesdays too.  
> **Agent:** Okay, I'll remember that. Anna plays at club and trains at club. The rest I already knew.

### Judge: tools in a workshop and what they are for (score 2)

The bot misreads simple sentences, gives garbled facts ("Screwdriver is a turn of screw"), claims it was never told the screwdriver's use right after being told, and adds random greetings.

> **User:** Hi! I'm Maksim. I want to teach you about tools in a workshop. A hammer is a tool.  
> **Agent:** Sorry, I didn't understand that.

> **User:** Maksim here, just a quick note for you: a hammer is a tool that you use to hit nails.  
> **Agent:** Got it. Hammer is a tool.

> **User:** Hammer is a tool. A hammer is used to hit nails.  
> **Agent:** I know, you told me before.

> **User:** Maksim here, a hammer is used to hit nails, and a screwdriver is used to turn screws.  
> **Agent:** Hello, nice to talk to you. Okay, I'll remember that. Hammer hits nail. Screwdriver is a turn of screw.

> **User:** Maksim here, so there's a hammer and a screwdriver. Can you tell me what a screwdriver is used for?  
> **Agent:** Hi there! Nobody has told me what screwdriver's use is, so I don't know.

> **User:** Maksim here, so what do you know about screwdrivers now?  
> **Agent:** Hello! About screwdriver: Screwdriver is a turn of screw.

### Judge: animals and nature (score 4)

The bot's replies are grammatical and mostly track the input, but they garble facts ("Cow is a say of moo", "Anna is in Berlin" for "lives in") and it misreads "Hi" as a taught fact, so it's inconsistent and unnatural.

> **User:** Hi! I'm Maksim. Animals are my favorite topic, so let's talk about them. A cat is an animal.  
> **Agent:** Okay, I'll remember that. Your greeting is hi. Got it. Animal is a topic.

> **User:** Rex is a dog, and dogs are animals. What is a cat?  
> **Agent:** Noted. Rex is a dog. Dog is an animal. I don't know what cat is.

> **User:** Anna likes tea.  
> **Agent:** Got it. Anna likes tea.

> **User:** Anna lives in Berlin.  
> **Agent:** Noted. Anna is in Berlin.

> **User:** Let's keep going with animals: a cow says moo.  
> **Agent:** Okay, I'll remember that. Cow is a say of moo.

> **User:** Cool, and what do you know about cows?  
> **Agent:** About cow: Cow is a say of moo.

### Judge: food and cooking (score 6)

The replies are grammatical and mostly track the facts taught, but they repeat "The rest I already knew" awkwardly, invent a rule ("if A is a banana then A grows on herb") from a loosely stated fact, and the final summary wrongly says the banana "grows on herb and plant" while omitting that herbs are plants.

> **User:** Hi, I'm Maksim, and I want to teach you some facts about food.  
> **Agent:** Hello!

> **User:** Hi! I'd like to teach you that bananas are fruits.  
> **Agent:** Got it. Banana is a fruit.

> **User:** Bananas are fruits, and they grow on plants called herbs.  
> **Agent:** Noted. Banana grows on plant. The rest I already knew. Okay, I'll remember that. Banana grows on herb. The rest I already knew.

> **User:** Bananas grow on herbs, and herbs are not trees, so that's a useful fact too.  
> **Agent:** I'll keep this as a rule: if A is a banana then A grows on herb. Got it. Herb is not a tree.

> **User:** Herbs are plants, but bananas grow on a plant called a herb, not on a tree.  
> **Agent:** Noted. Herb is a plant. Banana does not grow on tree. The rest I already knew.

> **User:** Ah, so bananas grow on herbs, which are not trees, and herbs are plants. Got it. What do you know about bananas now?  
> **Agent:** I know, you told me before. Here is what I know about banana: Banana is a fruit, does not grow on tree and grows on herb and plant.

### Judge: weather and the seasons (score 2)

Most replies are ungrammatical or nonsensical ("Snow falls Winter", "Spring is an after of Winter"), and the last answer ("I know, you told me before.") ignores the question about the current season.

> **User:** Winter is cold, and the days are short. Snow falls in winter.  
> **Agent:** Okay, I'll remember that. Winter is cold. Day is short. Got it. Snow falls Winter.

> **User:** Do you know what season comes after winter?  
> **Agent:** I have no idea whether season is a what follows of Winter.

> **User:** Which season comes after winter?  
> **Agent:** I don't know what is a season of comes after winter.

> **User:** Winter is cold. Spring comes after winter.  
> **Agent:** Got it. Winter's temperature is cold. Spring is an after of Winter.

> **User:** Winter is cold, but spring comes after winter, so what do you know about spring?  
> **Agent:** Noted. Spring comes after Winter. The rest I already knew. Here is what I know about Spring: Spring is an after of Winter and comes after Winter.

> **User:** Winter is cold, but spring comes after winter, so what season is it right now for you?  
> **Agent:** I know, you told me before.

