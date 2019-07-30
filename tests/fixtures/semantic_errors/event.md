---
title: Semantics Error on Transition with Event
references:
---

#### Event Transition
![diagram]()

##### 1. Have more than one transition with same trigger ✔
Transition with specific event must only occur once.
```scl,error
A -> B @ D
A -> C @ D
```
Which state should `A` transtition to when event `D` is triggered?

##### 2. Event both **with** and **without** guard pointing to same state ✔
This expression is redundant.
```scl,error
A -> B @ D[valid]
A -> B @ D
```
Regardless `valid` is true or false, `A` will transition to `B` when `D` is triggered.
This should be rewritten as:
```scl
A -> B @ D
```
