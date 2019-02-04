---
title: Transient and Compound State
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
```scl
A -> B
```
#### compound/composite/nested
```scl
state P {

}
```
#### parallel
```scl
parallel state P {
  state P1 {
    
  }
  state P2 {
    
  }
}
```
---
#### atomic/simple
```rs
enum State {
  A,
  B
}
```
#### compound/composite/nested
```rs
enum P {...}
```
#### parallel
```rs
trait State {...}

enum P1 {...}
impl State for P1 {...}

enum P2 {...}
impl State for P2 {...}
```
---