---
kind: slot
type: number
ask: How many days ago was that?
patterns:
- text: '{n} days ago'
- text: '{n} day ago'
- text: '{n} weeks ago'
  scale: 7
- text: '{n} week ago'
  scale: 7
- text: '{n} months ago'
  scale: 30
- text: '{n} month ago'
  scale: 30
- text: yesterday
  value: 1
- text: today
  value: 0
- text: last week
  value: 7
---
