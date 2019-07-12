---
title: File separation
references:
status: **half-baked**
---
![type state picture]()
---
### Import with no side effect
```scl
state {*} = import`./relative/path/explicit/extension.scl`
```
Read as: "use all statements in the `extension.scl`, no transition expression is included"

#### Partial import
```scl
state {
  State0
  CompoundState1 => { NestedCoumpundState },
  CompoundState2 => {
    NestedCoumpundState as C2,
    NState
  },
} = import`./relative/path/explicit/extension.scl`
```

### Side effect import
```scl
import`./relative/path/explicit/extension.scl`
```
Read as: "import all transitions on the `extension.scl`, including the nested transition"

### Import as a service (a.k.a external State Machine)
```scl
service extMachine {
  import`./relative/path/explicit/extension.scl`
}
```
Read as: "import all transition as a *service* with a name **extMachine**"

```scl
service extMachine {
  state { Compound, * } = import`./extension.scl`
  use Compound
}
```
Read as: "import all statements in the `extension.scl` while also use all transitions in **Compound** state  as a *service* with a name **extMachine**""

### Import as a specific state
```scl
state Extend = import`./relative/path/explicit/extension.scl`
```
or
```scl
compound state Extend = import`./relative/path/explicit/extension.scl`
```
Read as: "import all transition as a *compound state* with a name **Extend**"

```scl
parallel state Extend = import`./relative/path/explicit/extension.scl`
```
Read as: "import all transition as a *parallel state* with a name **Extend**"

### Import and execute script as an action (also applied to activity)
```scl
Alpha |> import`./script.js`
```
or
```scl
import`./script.js` <| Alpha
```
or
```scl
action runThing = import`./script.js`

Alpha |> runThing
```
or
```scl
state Alpha {
  @exit |> import`./script.js`
}
```
Read as: "import and execute ***./script.js*** when transition from *state **Alpha***"