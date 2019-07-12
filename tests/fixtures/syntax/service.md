---
title: Invoking services (or other state machine)
references:
  - https://xstate.js.org/docs/guides/communication.html#scxml
  - https://www.w3.org/TR/scxml/#invoke
status: **half-baked**
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
  }

  @ error {
    A -> C
    D -> D
  }
}
```
---
```scl
autoforward service fetchData {
  A -> B @ done
  A -> C @ error
}
```
or
```scl
service[forward] fetchData {
  A -> B @ done
  A -> C @ error
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
---
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
B |> Send(C ~> otherMachine)
```
or
```scl
B |> (C ~> otherMachine)
```
Read as: ""

---
```scl
B |> Send(C ~> service1,service2,serviceN)
```
or
```scl
B |> (C ~> service1,service2,serviceN)
```
Read as: ""

---
```scl
A -> B @ D |> Send(C ~> service1,service2,serviceN)
```
or
```scl
A -> B @ D |> (C ~> service1,service2,serviceN)
```
Read as: ""

---
```scl
A -> B @ D |> doSomething, Send(C ~> service1,service2,serviceN)
```
or
```scl
A -> B @ D |> doSomething, (C ~> service1,service2,serviceN)
```
Read as: ""

##### Send an event to other services and myself

```scl
B |> Send(<~ C ~> otherMachine)
```
or
```scl
B |> (<~ C ~> otherMachine)
```
---
```scl
B |> Send(<~ C ~> service1,service2,serviceN)
```
or
```scl
B |> (<~ C ~> service1,service2,serviceN)
```
---
```scl
B |> Send(D), Send(C~> service1,service2,serviceN)
```
or
```scl
B |> (<~ D), (C ~> service1,service2,serviceN)
```
or
```scl
B |> Send(<~D | C~> service1,service2,serviceN)
```
or
```scl
B |> (<~D | C~> service1,service2,serviceN)
```
