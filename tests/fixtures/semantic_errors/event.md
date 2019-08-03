---
title: Semantics Error/Warning on Transition with Event
references:
---

#### Event Transition
![diagram]()

##### 1. Have more than one transition with same trigger ✔
Transition with specific event must only occur once.
```scl,error
A -> B @ E
A -> C @ E
```
Which state should `A` transtition to when `E` is triggered?

##### 2. Event both **with** and **without** guard pointing to same state ✔
This expression is redundant.
```scl,error
A -> B @ E[valid]
A -> B @ E
```
Regardless `valid` is true or false, `A` will transition to `B` when `E` is triggered.
This should be rewritten as:
```scl
A -> B @ E
```

##### 3. Multiple guards on same event
This expression can cause unpredictable transition.
```scl,warning
A -> B @ E[valid]
A -> C @ E[exist]
```
Which state should `A` transtition to when `E` is triggered despite both `valid` and `exist` is true?
Formal verification should be used for extra precautions:
```scl
assume [guards <= 2] in A -> *

A -> B @ E[valid]
A -> C @ E[exist]
```
