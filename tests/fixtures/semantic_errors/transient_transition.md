---
title: Semantics Error/Warning on Transient Transition
references:
---

#### Transient Transition
![diagram]()

##### 1. Have more than one ✔
Transient transition must only occur once.
```scl,error
A -> B
A -> C
```
Which state should `A` immediately transtition to?

##### 2. Accompany with triggered event ✔
The state machine interpreter will confused if there is 2 things can be happened at the same time.
This will likely to cause a paradox.
```scl,error
A -> B
A -> C @ D
```
State `C` will never occur because `A` will transition to `B` immediately.
It still cause confusion even if guard is added to event `D`.
```scl,error
A -> B
A -> C @ D [isAllowed]
```
Even `isAllowed` is true, the program that implement this state machine will likely don't have enough time to trigger event `D`.
However, guarded event with no trigger is allowed because guard is precomputed (hence it written in camelCase).
```scl,warning
A -> B
A -> C @ [isAllowed]
```
`A` will transition to `C` if `isAllowed` else it will transition to `B`.
<!--TODO:-->Even so, formal verification should be used for extra precautions:
```scl
assume [auto + transient] in A -> *

A -> B
A -> C @ [isAllowed]
```

##### 3. Have multiple guards (auto transition)
This expression can cause unpredictable transition.
```scl,warning
A -> B @ [valid]
A -> C @ [exist]
```
Which state should `A` transtition to when `valid` and `exist` is true?
Formal verification should be used for extra precautions:
```scl
assume [guards <= 2] in A -> *

A -> B @ [valid]
A -> C @ [exist]
```
