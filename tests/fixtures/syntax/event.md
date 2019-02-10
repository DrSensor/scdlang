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

##### shortcut
```scl
@ E {
  A -> B
  B -> C
  C -> D
}
```
is a shortcut for
```
A -> B @ E
B -> C @ E
C -> D @ E
```

<details>
<summary>can be combined with guards or actions</summary>

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
@ E {
  A -> B @ [x > 0] |> activate
  C -> D @ [isOk] |> lampOn
}
```
</details>

#### guard
- symbol: `[`$guardsName|$expression`]`

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

##### with guards
```scl
A -> B @ C[D] |> f
```
Read as: "*state **A*** transition to *state **B*** at *event **C*** only if *condition **D*** is true will execute *action **f***"

---