---
title: Traffic Light
references:
  - https://xstate.js.org/docs/#finite-state-machines
  - https://hoverbear.org/2016/10/12/rust-state-machine-pattern/
  - https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=cb3b16f55f4d6ad25fc54f5058c7dacf
syntax:
  - scdlang
  - xstate
  - scxml
  - rust
---
![traffic light machine](https://imgur.com/rqqmkJh.png)
---
```scl
state light {
  initial green

  green -> yellow @ TIMER
  yellow -> red @ TIMER
  red -> green @ TIMER
}
```
or
```scl
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
export default {
  initial: 'green',
  states: {
    light: {
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
}
```
#### Example Usage
```js
import { Machine } from 'xstate'
import statecharts from './traffic-light'

const trafficLight = Machine(statecharts)

service = interpret(trafficLight)
  .onTransition(state => console.log(state))
  .start()

setInterval(() => trafficLight.send('TIMER'), 1000)
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
---
```rs
#[derive(Debug)]
enum Light {
  Green,
  Yellow,
  Red
}

enum Event {
  TIMER
}

#[derive(Debug)]
struct Machine { state: Light }

impl Machine {
  fn new() -> Self {
    Machine {
        state: Light::Green
    }
  }

  fn emit(&mut self, event: Event) {
    self.state = match event {
      Event::TIMER => match self.state {
        Light::Green => Light::Yellow,
        Light::Yellow => Light::Red,
        Light::Red => Light::Green,
      }
    }
  }
}
```
#### Example Usage
```rs
fn main() {
    let mut traffic_light = Machine::new();
    for _ in 0..6 {
        println!("{:?}", traffic_light.state);
        traffic_light.emit(Event::TIMER);
    }
}
```
---