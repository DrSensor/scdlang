---
title: Traffic Light
references:
  - https://xstate.js.org/docs/#finite-state-machines
syntax:
  - scdlang
  - xstate
  - scxml
---
![traffic light machine](https://imgur.com/rqqmkJh.png)
---
```scd
state light {
  initial green

  green -> yellow @ TIMER
  yellow -> red @ TIMER
  red -> green @ TIMER
}
```
or
```scd
state light {
  initial green
  
  @ TIMER {
    green -> yellow
    yellow -> red
    red -> green
  }
}
```
---
```js
{
  initial: 'green',
  states: {
    green: {
      on: {
        TIMER: 'yellow'
      }
    },
    yellow: {
      on: {
        TIMER: 'red'
      }
    },
    red: {
      on: {
        TIMER: 'green'
      }
    }
  }
}
```
---
```scxml
<scxml>
  <state id="light" initial="green">
    <state id="green">
      <transition event="TIMER" target="yellow"/>
    </state>
    <state id="yellow">
      <transition event="TIMER" target="red"/>
    </state>
    <state id="red">
      <transition event="TIMER" target="green"/>
    </state>
  </state>
</scxml>
```
