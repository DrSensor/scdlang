---
title: Invoking services (or other state machine)
references:
syntax:
  - scdlang
---
![type state picture]()
---
#### Initial state declaration
- keyword: initial
- type: state

```scxml
initial -> A
```

#### Final state declaration
- keyword: final
- type: action

```scxml
B -> final
```

#### Tiggering an event
> https://www.w3.org/TR/scxml/#send
- keyword: Send
- type: action

```scl
A -> B |> Send(C)
```

```scl
Send(C) |> A
```
or
```scl
(C ~>) |> A
```
or
```scl
state A { @entry |> Send(C) }
```

<details><summary>self transition loop</summary>

```scl
->> B @ C |> Send(C)
B -> C @ G
```
Read as: "Loop transition to *state **B*** as long as *event **G*** is not triggered"
</details>

#### Delayed transition
- keyword: after
- type: action

```scl
// loop every 10 seconds
A -> B @ C |> Send(C, after=10s) // it follow XML data type for Duration
```

<details><summary>shortcut</summary>

> references: https://xstate.js.org/docs/guides/delays.html#behind-the-scenes

```scl
A -> B @ after(0.5s)
```
is a shortcut of
```scl
A <| Send(UUID, after=0.5s)
A -> B @ UUID
```
or
```scl
state A { @entry |> Send(UUID, after=0.5s) }
A -> B @ UUID
```
</details>

##### Canceling delayed transition
- keyword: Cancel
- type: action

```scl
B @ G |> Cancel // internal transition
```
Read as: "Cancel all delayed transition from *state **B*** if *event **G*** is triggered"

```scl
B @ G |> Cancel(C)
```
Read as: "Cancel all delayed transition to *state **C*** from *state **B*** if *event **G*** is triggered"

##### Raise an event
> https://www.w3.org/TR/scxml/#raise

This will prevent self transition loop though the entry/exit actions might still be executed 🤔 (maybe)
```scl
A -> B @ C |> Send(C, instantly)
```
or
```scl
A -> B @ C |> Send(|C|)
```
or
```scl
A -> B @ C |> Raise(C)
```

#### Assigning data
- keyword: Assign
- type: action

```scl
let VarX as x

A -> B @ C |> Assign(x=5)
```