# Evaluation report 2026-11-16T19:36:48

commit `75ac6fd`

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

Average judge score: **3.00 / 10** from 3 judge(s).

### Judge: your family and what they like to eat (score 4)

The chatbot's replies are mostly grammatical-ish but incoherent: it says "You are a name of you," "Your name is you," and "Your family likes eat..." and it mangles facts ("Anna is a sister" instead of "Anna is my sister"), and it hallucinates "eats rice and bread" from a user statement it didn't say.

> **User:** Hi! My name is Maksim. I want to teach you about my family and what they like to eat.  
> **Agent:** Okay, I'll remember that. You are a name of you. Hi there! Noted. Your name is you.

> **User:** Hi! My name is Maksim.  
> **Agent:** Hello! I already knew that. Your name is you.

> **User:** Okay, my family likes to eat a lot of different foods, and I'd like to tell you about them.  
> **Agent:** Got it. Your family likes eat a lot of different food.

> **User:** Okay, my family likes to eat rice and bread. Anna is my sister.  
> **Agent:** Noted. Your family likes rice and bread and eats rice and bread. Okay, I'll remember that. Anna is a sister.

> **User:** Anna likes rice.  
> **Agent:** Got it. Anna likes rice.

> **User:** Anna likes bread too.  
> **Agent:** Noted. Anna likes bread.

### Judge: pets and animals you know (score 3)

Its replies are mostly grammatical only in the first few turns, then garble the question ("whether Rex is a what kind of animal is"), drop articles and capitalization ("golden retriever", "rex"), and repeat old facts unprompted.

> **User:** Anna has a dog named Rex.  
> **Agent:** Okay, I'll remember that. Anna has Rex. Rex is a dog.

> **User:** Anna likes tea.  
> **Agent:** Got it. Anna likes tea.

> **User:** Anna's dog Rex is a pet, and dogs are animals.  
> **Agent:** Noted. Rex is a pet. Dog is an animal. I already knew that. Anna has Rex. Rex is a dog.

> **User:** Do you know what kind of animal Rex is?  
> **Agent:** I don't know whether Rex is a what kind of animal is.

> **User:** Anna's dog Rex is a golden retriever, right?  
> **Agent:** I'm not sure whether Anna's dog rex is golden retriever right.

> **User:** Anna's dog Rex is a golden retriever.  
> **Agent:** Noted. Anna's dog rex is golden retriever.

### Judge: your city, your work and your colleagues (score 2)

The bot misnamed the user ("Your name is you", "You's name is you"), garbled pronouns ("works with me at bank"), repeated itself, and said "I already knew that" in ways that don't match what was taught.

> **User:** Hi! I'm Maksim. I want to teach you a few things about my city, my work and my colleagues.  
> **Agent:** Hello! Got it. Your name is you.

> **User:** Your name is Maksim, and I work in a city called Berlin.  
> **Agent:** Noted. You's name is you. You work in Berlin. Berlin is a city.

> **User:** My colleague is Anna.  
> **Agent:** Okay, I'll remember that. Your colleague is Anna.

> **User:** Anna works with me at the bank.  
> **Agent:** Got it. Anna works with me at bank.

> **User:** Anna is my colleague, and she works with me at the bank in Berlin.  
> **Agent:** Noted. Anna is a colleague. I already knew that. Anna is a colleague. Okay, I'll remember that. Anna works with you and works at bank. Bank is in Berlin. I already knew that. Anna is a colleague.

> **User:** Anna is a colleague, and she works at the bank with me, so please remember that she works in Berlin too.  
> **Agent:** Got it. You work at bank. Anna works in Berlin. I already knew that. Anna works at bank and is a colleague.

