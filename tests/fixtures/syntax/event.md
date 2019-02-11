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
// or
B <- A @ C
```
Read as: "*state **A*** transition to *state **B*** at *event **C***"

> Useful for defining Toggle event
```scl
A <-> B @ C
```
Read as: "*state **A*** transition to/from *state **B*** at *event **C***"

```scl
A -> B @ C,D,E
```
Read as: "*state **A*** transition to *state **B*** at *event **C**, **D**, or **E***"

##### 2nd form
```scl
A -> B @ E
B -> C @ E
C -> D @ E
```
can be written as:
```scl
@ E {
  A -> B
  B -> C
  C -> D
}
```
Read as: "at *event **E***:
<br>- *state **A*** can transition to *state **B***
<br>- *state **B*** can transition to *state **C***
<br>- *state **C*** can transition to *state **D***"

<details>
<summary>Also, can be combined with guards and/or actions.</summary>

```scl
@ E[isOk] |> activate,lampOn,etc {
  A -> B
  C -> D
}
```

```scl
@ E[x > 0] {
  A -> B |> activate
  C -> D |> lampOn
}
```

```scl
@ [x > 0] {
  A -> B |> activate
  A -> B @ E |> lampOn
}
```

```scl
|> activate {
  A -> B @ E
  C -> D @ F
  J -> K
}
```
</details>

#### guard
- symbol: `[`$guardNames$|$expression$`]`

##### use $guardsName
```scl
A -> B @ C[isD]
```
Read as: "*state **A*** transition to *state **B*** at *event **C*** only if *condition **isD*** is true"

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
- symbol: `|>`
```scl
A -> B @ C |> f
```
Read as: "*state **A*** transition to *state **B*** at *event **C*** will execute *action **f***"

```scl
|> f {
  J -> K
  A -> B @ E
}
```
Read as: "execute *action **f***
<br>on *state **J*** transition to *state **K***
<br>or
<br>on *state **A*** transition to *state **B*** at *event **E***"

##### with guards
```scl
A -> B @ C[D] |> f
```
Read as: "*state **A*** transition to *state **B*** at *event **C*** only if *condition **D*** is true will execute *action **f***"

```scl
@ [D] {
  => J |> g
  A -> B @ C |> f
}
```
Read as: "only if *condition **D*** is true:
<br> - *state **A*** can transition to *state **B*** at *event **C*** which will execute *action **f***
<br> - *state **J*** can self transition which will execute *action **g***"

---