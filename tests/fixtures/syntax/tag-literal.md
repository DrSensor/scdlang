---
title: Tagged template literal
references:
  - [JavaScript tagged template literal](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Template_literals#Tagged_templates)
status: **nice to have**
syntax:
  - scdlang
  - julia
  - javascript
---
![type state picture]()
---
### Normal literal
```scl
InState |> `() => callAction()`
```
This will automatically translate to the native target. For example if targeting `xstate`, it will output:
```js
export default {
  states: {
    inState: {
      onExit: () => callAction()
    }
  }
}
```

#### Markdown style
> Note that new line is necessary 👇
````scl
CurrentState -> NextState @ Event |>
```
function go() {
  // some long code
}
```
````
This will automatically translate to the native target. For example if targeting `kingly`, it will output:
```js
const [CS0, E0, NS0] = Array.from(Array(3), () => Symbol())
export default {
  events: [E0],
  transitions: [
    {
      from: CS0,
      event: E0,
      to: NS0,
      action: function go() {
        // some long code
      }
    }
  ]
}
```

### Tagged literal
Some language (e.g Julia) and runtime (e.g GraalVM) have the ability to execute other programming language. To accomodate this, there will be a need for tagged literal similiar to JavaScript tagged templates.
```scl
let someList
InState |> python`$someList = [i for i in range(10)]`
```
This will automatically translate to the native target. For example if targeting `xstate` and specify `graal` as the runtime, it will output:
```js
export default {
  states: {
    inState: {
      onExit: assign({
        someList: Polyglot.eval(
          'python', '[i for i in range(10)]'
        )
      })
    }
  }
}
```

#### Markdown style
> This is where this style really shine✨
````scl
CurrentState -> NextState @ Event |>
```py
# long python code
```
````
This will automatically translate to the native target. For example if targeting `kingly` with `graal` as a runtime, it will output:
```js
const [CS0, E0, NS0] = Array.from(Array(3), () => Symbol())
export default {
  events: [E0],
  transitions: [
    {
      from: CS0,
      event: E0,
      to: NS0,
      action: () => {
        Polyglot.eval('py', `
          # long python code
        `)
      }
    }
  ]
}
```