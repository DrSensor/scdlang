---
title: Declaring State
references:
  - https://www.uml-diagrams.org/state-machine-diagrams.html
  - https://github.com/sverweij/state-machine-cat
  - https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=cb3b16f55f4d6ad25fc54f5058c7dacf
syntax:
  - scdlang
  - rust
---
![type state picture]()
---
#### atomic/simple
- keyword: *state*
- symbol: `{`$declarations$`}`
```scl
state P {...}
```
Read as: "*state **P*** have ..."

> declaration are optional

##### transition
- symbol: $current_state `->` $next_state

```scl
A -> B
```
Read as: "*state **A*** transition to *state **B***" ✔

```scl
B <- A
```
Read as: "*state **B*** transition from *state **A***" ✔

```scl
A,C,D -> B
```
Read as: "*state **A**, **C**, and **D*** transition to *state **B***"

```scl
B <- A,C,D
```
Read as: "*state **B*** transition from either *state **A**, **C**, or **D***"

```scl
B -> B
```
or
```scl
->> B
```
Read as: "*state **B*** transition to *state **B***"

```scl
A -> B
B -> B
```
or
```scl
A -> B
->> B
```
or
```scl
A ->> B
```
Read as: "*state **A*** transition to *state **B***" then loop transition to itself

#### compound/composite/nested
- keyword: *compund*|*composite*
- symbol: `$keyword state {`$declarations$`}`
```scl
compound state P {...}
```
Read as: "inside *compound state **P***, ..."

##### ~~type~~statecasting
This will force state B to be a compound state when transitioning from A
```scl
state B { ... }

A -> [B]
```
Read as: "*state **A*** transition to *compound state **B***"

##### transition to nested state
```scl
state B {
  C -> A
}

A -> B[C]
```
or
```scl
compund state B {
  C -> A
}

A -> B[C]
```
Read as: "*state **A*** transition to *compound state **B***"

#### parallel
- keyword: *parallel*
```scl
parallel state P {
  state P1 {...}
  state P2 {...}
}
```
Read as: "inside *state **P***, there is *state **P1*** and **P2** which run on *parallel*"

##### ~~type~~statecasting
This will force state B to be a parallel state when transitioning from A
```scl
state B { ... }

A ->| B,C
```
Read as: "*state **A*** fork to *state **B*** and *state **C***"

```scl
state B { ... }

B,C |-> A
```
Read as: "*state **B*** and *state **C*** join to *state **A***"

<details>
<summary>examples</summary>

```scl
A ->| B,D,E
parallel { // 👈 is this really neccessary 🤔
  {
    B -> L1 @ C
    L1 -> L2
    L2 -> L
  }
  D -> G
  E -> I
}
L,G |-> A
A,I |-> Z
```
</details>

> **no transitions should exist between parallel state nodes**

#### history shallow & deep
> default is shallow
- keyword: *history* or *cache*

```scl
history state P {...}
cache state P {...}
```
Read as: "inside *state **P*** which is *cache*able, ..."

##### ~~type~~statecasting
```scl
state P { ... }

A -> P[history]
```
Read as: "*state **A*** transition to *pervious* *state* of **P***"

```scl
state P { ... }

P[cache] -> A
```
Read as: "cache the *current* *state* of **P** then transition to *state **A***"

##### shallow
- keyword: *shallow.*
```scl
history[shallow] state P {...}
state[shallow.cache] B {...}
```
or
```scl
history state P {...}
state[cache] B {...}
```
Read as: "inside *state **P*** which is *cache*able, ..."

##### deep
- keyword: *deep.*
- symbol: `*`
```scl
history[deep] state P {...}
state[deep.cache] B {...}
```
or
```scl
history[*] state P {...}
```
Read as: "inside *state **P*** and all *state* inside *state **P*** which is *cache*able, ..."

---