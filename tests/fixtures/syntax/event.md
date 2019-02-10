---
title: Declaring internal action, activity, and event
references:
syntax:
  - scdlang
  - rust
---
![type state picture]()
---
#### event
- symbol: `@`

```scl
A -> B @ C
```
Read as: "*state **A*** transition to *state **B*** at *event **C***"

```scl
A -> B @ C,D,E
```
Read as: "*state **A*** transition to *state **B*** at *event **C**, **D**, or **E***"

#### guard
- symbol: `[`$guardsName|$expression`]`

##### use $guardsName
```scl
A -> B @ C[D]
```
Read as: "*state **A*** transition to *state **B*** at *event **C*** only if *condition **D*** is true"

##### use $expression
```scl
context VarX as x
context VarY as y

A -> B @ C[x > y]
```
Read as: "*state **A*** transition to *state **B*** at *event **C*** only if `x > y`"

##### use boolean operator
- symbol: `|`,`&`,`!`,`^`
```scl
A -> B @ C[D|!E]
```
or
```scl
context VarX as x
context VarY as y

A -> B @ C[x > y & y > 0]
```

##### use "in state" guards
```scl
context VarX as x

state A {
  E -> G @ F
  G -> E @ F
}
A -> B @ C[{G}]
```
Read as: "*state **A*** transition to *state **B*** at *event **C*** only if **A** is in *state **G***"


#### action
- symbol: `/`$actionaName`(`_$arguments`)`
```scl
A -> B @ C |> f
```
Read as: "*state **A*** transition to *state **B*** at *event **C*** will execute *action **f()***"

##### with guards
```scl
A -> B @ C[D] |> f
```
Read as: "*state **A*** transition to *state **B*** at *event **C*** only if *condition **D*** is true will execute *action **f()***"

##### with $arguments
```scl
context VarX as x

A -> B @ C |> f
```
Read as: "*state **A*** transition to *state **B*** at *event **C*** will execute *action **f(x)***"

---