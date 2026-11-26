# Proposal

## The Idea

Build a general assistant **without neural networks**.

Old symbolic assistants and expert systems are clear and easy to control, but they are stuck with whatever someone typed into them. The idea here is to make a symbolic system that **keeps learning on its own**: picking up new facts, new templates, and new structure over time, and getting better through use.

## How It Works

Instead of hard-coding separate systems for chat, tools, or workflows, the agent runs on one flexible core that keeps track of its active context-what is going on right now, the facts it knows so far, and the history of messages or tool runs.

From that context, it:
- **Turns inputs** into structured facts, whether they come from a person, a document, or a tool.
- **Matches and combines patterns**-reusable ways of responding and acting-to decide what to do next and carry it out.
- **Acts through replies and tools** in the same way, with every result feeding back into what it knows.

Natural conversation, an AI persona, tool use, and multi-step work are all just different expressions of the same system.

## How It Improves

- **Being taught:** A person using it can teach it new facts, rules, or replies directly.
- **Mining and induction:** It looks through past experience, texts, and conversations to pull out new facts, templates, and recurring patterns-and combines simple patterns into more capable ones.
- **Meta-tools:** Its own learning and mining steps are tools it can run itself, so noticing a gap and improving itself works the same way as solving any other problem.
