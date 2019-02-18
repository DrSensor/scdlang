# `scrap` (**s**tate**c**harts **r**h**ap**sody)
> **Work In Progress**

Features:
- [x] output as [xstate configuration][xstate-format]

## Usage
Given this simple declaration:
```scl
A1 -> B @ C
A2 <- B @ D
```
then
```console
$ scrap simple.scl --format xstate
{
  states: {
    A1: {
      on: {
        C: "B"
      }
    },
    A2: {
      on: {
        D: "B"
      }
    }
  }
}
```

[xstate-format]: https://xstate.js.org/docs/guides/machines.html#configuration