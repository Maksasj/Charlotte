---
kind: slot
type: id
ask: Which order, ticket or reference number is it?
patterns:
- text: order {id}
  sets:
    object: order
- text: ticket {id}
  sets:
    object: ticket
- text: subscription {id}
  sets:
    object: subscription
- text: account {id}
  sets:
    object: account
- text: expense {id}
  sets:
    object: expense
- text: request {id}
  sets:
    object: request
- text: device {id}
  sets:
    object: device
- text: reference {id}
- text: number {id}
---
