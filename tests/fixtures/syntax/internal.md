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
### action
- keyword: entry, exit
- symbol: `|>`, `<|`, `<|>`

#### on entry
```scl
state Alpha {
  entry |> something
}
```
or
```scl
state Alpha {
  |> something
}
```
or
```scl
Alpha <| something
```
or
```scl
something |> Alpha
```
Read as: "execute *action **something*** when transition to *state **Alpha***"

#### on exit
```scl
state Alpha { exit |> something }
```
or
```scl
state Alpha {
  <| something
}
```
or
```scl
Alpha |> something
```
or
```scl
something <| Alpha
```
Read as: "execute *action **something*** when transition from *state **Alpha***"

#### on entry and exit
```scl
state Alpha { entry,exit |> something }
```
or (discouraged)
```scl
state Alpha {
  <|> something
}
```
or (discouraged)
```scl
something <|> Alpha
```
or
```scl
Alpha <|> something
```
Read as: "when transition to/from *state **Alpha***, execute *action **something***"

#### action with expression
```scl
context VarX as x

state Alpha {
  entry |> --x
  exit |> ++x
}
```
Read as: "decrement `x` when entering *state **Alpha*** and increment `x` if exiting *state **Alpha***"

#### activity
```scl
state Beta { do |> beeping }
```
Read as: "perform *activity **beeping*** when on *state **Beta***"

#### internal transitions
> action on entry and exit will not be executed when those event triggered
```scl
state Beta { @ Click }
```
Read as: "*event **Click*** can occurred while in *state **Beta***"

##### with action
```scl
state Beta { @ Click |> something }
```
Read as: "execute *action **something*** when *event **Click*** occurred while in *state **Beta***"

##### with guard 🤔
```scl
state Beta { @ Click[x > 0 & {A}] }
```
Read as: "*event **Click*** can occurred while in *state **Beta*** only if in *state **Alpha***"

---