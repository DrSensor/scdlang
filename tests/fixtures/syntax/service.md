---
title: Invoking services (or other state machine)
references:
  - https://xstate.js.org/docs/guides/communication.html#scxml
  - https://www.w3.org/TR/scxml/#invoke
syntax:
  - scdlang
---
![type state picture]()
---
### service
- keyword: service

```scl
service fetchData {
  @ done {
    A -> B
    C -> B
    D -> A
  }

  @ error {
    A -> C
    C -> D
    D -> B
  }
}
```

```scl
autoforward service fetchData {
  @ done {
    A -> B
  }

  @ error {
    A -> C
  }
}
```
or
```scl
service[forward] fetchData {
  @ done {
    A -> B
  }

  @ error {
    A -> C
  }
}
```

###### 2nd form

```scl
A -> B @ done(fetchData[autoforward])
```
or
```scl
A -> B @ done(forward fetchData) // 🤔
```

```scl
A -> B @ error(fetchData)
```

###### 3rd form

```scl
autoforward service fetchData {
  A -> B @ done
  A -> C @ error
}
```

##### Send an event to other services

```scl
B |> Send(C -> otherMachine)
```

```scl
B |> Send(C -> service1,service2,serviceN)
```

##### Send an event to other services and myself

```scl
B |> Send(<-C -> otherMachine)
```

```scl
B |> Send(<-C -> service1,service2,serviceN)
```