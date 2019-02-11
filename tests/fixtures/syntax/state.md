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
- symbol: $current_state `->` $next_state

```scl
A -> B
```
Read as: "*state **A*** transition to *state **B***"

```scl
B <- A
```
Read as: "*state **B*** transition from *state **A***"

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
=> B 
```
Read as: "*state **B*** transition to *state **B***"

#### compound/composite/nested
- keyword: *state*
- symbol: `{`$declarations$`}`
```scl
state P {...}
```
Read as: "inside *state **P***, ..."

#### parallel
- keyword: *parallel*
```scl
parallel state P {
  state P1 {...}
  state P2 {...}
}
```
Read as: "inside *state **P***, there is *state **P1*** and **P2** which run on *parallel*"

#### history shallow & deep
> default is shallow
- keyword: *history* or *cache*

```scl
history state P {...}
cache state P {...}
```
Read as: "inside *state **P*** which is *cache*able, ..."

```scl
A -> P[history]
```
Read as: "*state **A*** transition to *pervious* *state* of **P***"

```scl
P[cache] -> A
```
Read as: "cache the *current* *state* of **P** then transition to *state **A***"

##### shallow
- keyword: *shallow.*
```scl
shallow.history state P {...}
shallow.cache state P {...}
```
Read as: "inside *state **P*** which is *cache*able, ..."

##### deep
- keyword: *deep.*
- symbol: `*`
```scl
deep.history state P {...}
deep.cache state P {...}
*history state P {...}
*cache state P {...}
```
Read as: "inside *state **P*** and all *state* inside *state **P*** which is *cache*able, ..."

---