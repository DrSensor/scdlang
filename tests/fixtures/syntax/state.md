---
title: Transient and Compound State
references:
  - https://www.uml-diagrams.org/state-machine-diagrams.html
  - https://github.com/sverweij/state-machine-cat
syntax:
  - scdlang
---
![type state picture]()
---
#### atomic/simple
```scd
A -> B
```
#### compound/composite/nested
```scd
state P {

}
```
#### parallel
```scd
parallel state P {
  state P1 {
    
  }
  state P2 {
    
  }
}
```
