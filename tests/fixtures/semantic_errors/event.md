---
title: Semantics Error on Transition with Event
references:
---

#### Event Transition
![diagram]()

##### 1. Have more than one transition with same trigger
Transition with specific event must only occur once.
```scl,error
A -> B @ D
A -> C @ D
```
Which state should `A` transtition to when event `D` is triggered?