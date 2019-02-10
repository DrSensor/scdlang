---
title: Declaring internal action, activity, and event
references:
- [xstate internal transition](https://xstate.js.org/docs/guides/internal.html)
syntax:
  - scdlang
  - rust
---
![type state picture]()
---
#### action on entry
```scl
... -> Alpha|>something
```
or (preferred)
```
state Alpha {
  entry |> something
}
```
Read as: "execute *action **something*** when transition to *state **Beta***"

#### action on exit
```scl
Alpha|>something -> ...
```
or (preferred)
```
state Alpha {
  exit |> something
}
```
Read as: "execute *action **something*** when transition from *state **Alpha***"

#### action with expression
```scl
context VarX as x

Alpha|>--x -> ...
... -> Alpha|>++x
```
or (preferred)
```
state Alpha {
  entry |> --x
  exit |> ++x
}
```
Read as: "decrement `x` when entering *state **Alpha*** and increment `x` if exiting *state **Alpha***"

#### activity
```scl
Alpha -> Beta[<beeping>]
```
or
```scl
state Beta {
  do |> beeping
}
```
Read as: "perform *activity **beeping*** when on *state **Beta***"

#### internal transitions
> action on entry and exit will not be executed when those event triggered
```scl
state Beta {
  @ Click
}
```
Read as: "*event **Click*** can occurred while in *state **Beta***"

##### with action
```scl
state Beta {
  @ Click |> something
}
```
Read as: "execute *action **something*** when *event **Click*** occurred while in *state **Beta***"

##### with guard 🤔
```scl
state Beta {
  @ Click[{Alpha}]
}
```
Read as: "*event **Click*** can occurred while in *state **Beta*** only if in *state **Alpha***"

---