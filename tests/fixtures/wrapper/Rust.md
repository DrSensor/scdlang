---
title: Transient and Compound State
references:
  - https://www.uml-diagrams.org/state-machine-diagrams.html
  - https://github.com/sverweij/state-machine-cat
  - https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=cb3b16f55f4d6ad25fc54f5058c7dacf
syntax:
  - scdlang
  - rust
---
# Simple/Atomic
```rs
enum State {…}

enum Event {…}

struct Machine {
    state: State
}
```
```rs
impl Machine {
    fn from(initial: State) -> Self {
        Self { state: initial }
    }

    fn send(&mut self, event: Event) {…}
}
```
## If number of `Event` < `State`
```diff
impl Machine {
    fn from(initial: State) -> Self {
        Self { state: initial }
    }

    fn send(&mut self, event: Event) {
+       self.state = match event {
+           Event::… => match self.state {
+               State::… => State::…,
+           }
        }
    }
}
```
## If number of `State` < `Event`
```diff
impl Machine {
    fn from(initial: State) -> Self {
        Self { state: initial }
    }

    fn send(&mut self, event: Event) {
+       self.state = match self.state {
+           State::… => match event {
+               Event::… => State::…,
+           }
        }
    }
}
```
## If `initial` declared
```diff
impl Machine {
+   fn new() -> Self {
+       Self { state: State::… /*initial state*/ }
+   }

-   fn from(initial: State) -> Self {
-       Self { state: initial }
-   }

    fn send(&mut self, event: Event) {…}
}
```
<br/>

# Parallel
```rs
trait State: Sized {
    fn transition(&self, event: Event) -> Option<Self>;
}
```
```rs
enum Event {…}

enum __StateName {…}
impl State for __StateName {…}
```
```rs
struct Machine<T: State> {
    state: T
}

impl<T: State> Machine<T> {
    fn from(initial: T) -> Self {
        Self { state: initial }
    }

    fn send(&mut self, event: Event) {
        self.state = self.state.transition(event);
    }
}
```
## If `initial` declared
```diff
trait State: Sized {
+   fn initial() -> Self;
    fn transition<'s>(&self, event: Event) -> Option<&'s Self>;
}
```
```diff
enum Event {…}

enum __StateName {…}
impl State for __StateName {
+   fn initial<'s>() -> &'s Self {
+       &__StateName::… // initial state
+   }

    fn transition<'s>(&self, event: Event) -> Option<&'s Self>;
}
```
```diff
struct Machine<'s, T: State> {…}

impl<T: State> Machine<T> {
+   fn new() -> Self {
+       Machine { state: T::initial() }
+   }

-   fn from(initial: State) -> Self {
-       Self { state: initial }
-   }

    fn send(&mut self, event: Event) {…}
}
```
## With state listener
TODO: test this
```diff
struct Machine<T: State> {
    state: T,
+   callback: Option<fn(&T)>
}

impl<T: State> Machine<T> {
    fn from(initial: T) -> Self {
        Self {
            state: initial,
+           callback: None
        }
    }

    fn send(&mut self, event: Event) {
        self.state = T::transition(&self.state, event);
+       if let Some(cb) = self.callback {
+           cb(&self.state)
+       }
    }

+   fn on_transition(&mut self, callback: fn(&T)) {
+       self.callback = Some(callback)
+   }
}
```
## If number of `Event` < `State`
```diff
enum Event {…}

enum __StateName {…}
impl State for __StateName {
    fn transition(&self, event: Event) -> Self {
+       match event {
+           Event::… => match self {
+               __StateName::… => __StateName::…,
+           }
+       }
    }
}
```
## If number of `State` < `Event`
```diff
enum Event {…}

enum __StateName {…}
impl State for __StateName {
    fn transition(&self, event: Event) -> Self {
+       match self {
+           __StateName::… => match event {
+               Event::… => __StateName::…,
+           }
+       }
    }
}
```