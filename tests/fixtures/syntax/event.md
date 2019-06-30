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
Read as: "*state **A*** transition to *state **B*** at *event **C***" ✔

> Useful for defining Toggle event
```scl
A <-> B @ C
```
Read as: "*state **A*** transition to/from *state **B*** at *event **C***" ✔

```scl
A -> B @ C,D,E
```
Read as: "*state **A*** transition to *state **B*** at *event **C**, **D**, or **E***"

```scl
A -> A @ C
B -> B @ C
```
or
```scl
A ->> B @ C
```
or
```scl
B <<- A @ C
```
Read as: "*state **A*** loop into *state **B*** at *event **C** ✔

```scl
A -> A
B -> B @ C
```
or
```scl
A >-> B @ C
```
or
```scl
B <-< A @ C
```
Read as: "*state **A*** transition to *state **B*** then loop at *event **C** ✔

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

##### 3rd form
```scl
A -> B @ C[isG],D[isT],F
A -> B @ E |> run
```
can be written as:
```scl
A -> B @ C[isG]
|@ D[isT]
|@ E |> run
|@ F
```
Read as: "*state **A*** transition to *state **B*** at *event **C***
<br>- or at *event **D*** only if *condition **isT*** is true"
<br>- or at *event **E*** and will execute *action **run***

<details>
<summary>Also, the guards/actions can be expressed beforehand</summary>

```scl
A -> B @ C[isT]
A -> B @ D[isT & isE]
A -> B @ [isT] |> run
```
can be written as:
```scl
A -> B @ [isT & *]
|      @ C
|      @ D[isE]
|      |> run
```
---
```scl
A -> B @ C |> reset
A -> B @ D[isT] |> reset
A -> B @ F |> reset,run
```
can be written as:
```scl
A -> B |> reset,*
|      @ C
|      @ D[isE]
|      @ F |> run
```
---
```scl
A -> B @ C[isT] |> reset
A -> B @ D[isE | isT] |> reset
A -> B @ F[isT] |> run,reset
```
can be written as:
```scl
A -> B @ [* | isT] |> *,reset
|      @ C[isT]
|      @ D[isE | isT]
|      @ F[isT] |> run,reset
```
---
```scl
A -> B @ C[isT]
A -> B @ C[isE] |> run
A -> B @ C |> reset
```
can be written as:
```scl
A -> B @ C[*] |> *
|      @ [isT]
|      @ [isE] |> run
|      |> reset
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
let VarX as x
let VarY as y

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
let VarX as x
let VarY as y

A -> B @ C[x > y & y > 0]
```

##### use "in state" guards
```scl
let VarX as x

state A {
  E -> G @ F
  G -> E @ F
}
A -> B @ C[<G>] //TODO: consider to use `in` keyword 🤔
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
  A -> J @ D |> g
  A ->> B @ C[isEmergency] |> f
}
```
Read as: "only if *condition **D*** is true:
<br> - *state **A*** can transition to *state **B*** at *event **C*** which will execute *action **f***
<br> - *state **J*** can self transition which will execute *action **g***"

---