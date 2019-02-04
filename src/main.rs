mod statechart {
    use core::fmt::Debug;

    pub trait State: Sized + Debug {
        fn transition<'s>(&self, event: &'s Event) -> Result<Self, &'s Event>;
        fn is_final(&self) -> bool;
    }

    pub trait WithInitial: State {
        fn initial() -> Self;
    }

    // #region Declaration
    #[derive(Debug)]
    pub enum Event {
        Reset,
        Timer,
        Push,
    }

    #[derive(Debug)]
    pub enum Light {
        Green,
        Yellow,
        Red,
        Final()
    }

    #[derive(Debug)]
    pub enum Switch {
        On,
        Off,
    }
    // #endregion

    impl State for Light {
        fn transition<'s>(&self, event: &'s Event) -> Result<Self, &'s Event> {
            match event {
                Event::Timer => Ok(match self {
                    Light::Green => Light::Yellow,
                    Light::Yellow => Light::Red,
                    Light::Red => Light::Final(),
                    Light::Final() => Light::Final(),
                }),
                _ => Err(event),
            }
        }

        fn is_final(&self) -> bool {
            if let Light::Final() = self { true }
            else { false }
        }
    }

    impl WithInitial for Switch {
        fn initial() -> Self {
            Switch::Off
        }
    }

    impl State for Switch {
        fn transition<'s>(&self, event: &'s Event) -> Result<Self, &'s Event> {
            match event {
                Event::Push => Ok(match self {
                    Switch::Off => Switch::On,
                    Switch::On => Switch::Off,
                }),
                Event::Reset => Ok(Self::initial()),
                _ => Err(event),
            }
        }

        fn is_final(&self) -> bool { false }
    }

    pub struct Machine<T: State> {
        pub state: T,
        error: Option<fn(&Event, &T)>,
        stale: Option<fn(&Event)>,
        transition: Option<fn(&T, &T, Event)>, // fn(&current_state, &previous_state)
    }

    impl<T: WithInitial> Machine<T> {
        pub fn new() -> Self {
            Self {
                state: T::initial(),
                stale: None,
                error: None,
                transition: None,
            }
        }
    }

    impl<T: State> Machine<T> {
        pub fn from(initial: T) -> Self {
            Self {
                state: initial,
                stale: None,
                error: None,
                transition: None,
            }
        }

        pub fn on_final(&mut self, callback: fn(&Event)) {
            self.stale = Some(callback);
        }

        pub fn on_transition(&mut self, callback: fn(&T, &T, Event)) {
            self.transition = Some(callback);
        }

        pub fn error_handler(&mut self, callback: fn(&Event, &T)) {
            self.error = Some(callback);
        }

        pub fn send(&mut self, event: Event) {
            match self.state.transition(&event) {
                Ok(_state) => if let Some(transition) = self.transition {
                    if _state.is_final() {
                        if let Some(stale) = self.stale { stale(&event) }
                    } else {
                        transition(&_state, &self.state, event);
                        self.state = _state;
                    }
                },
                Err(_event) => if let Some(error) = self.error {
                    error(_event, &self.state);
                }
            }
        }
    }
}

use statechart::*;

fn main() {
    // let mut traffic_light = Machine::<Light>::new();
    let mut lamp = Machine::<Switch>::new();
    let mut traffic_light = Machine::from(Light::Red);

    traffic_light.on_transition(|state, previous, event| println!("{:?} -> {:?} @ {:?}", previous, state, event));
    lamp.on_transition(|state, previous, event| println!("{:?} -> {:?} @ {:?}", previous, state, event));
    traffic_light.on_final(|event| println!("final @ {:?}", event));

    traffic_light.error_handler(|event, state| println!("error: @ {:?} {{{:?} -> <unknown>}}", event, state));
    lamp.error_handler(|event, state| println!("error: @ {:?} {{{:?} -> <unknown>}}", event, state));

    // loop {
    for x in 0..17 {
        println!("{}. {:?} {:?}", x, lamp.state, traffic_light.state);
        traffic_light.send(Event::Timer);

        if x % 4 == 0 {
            lamp.send(Event::Push)
        } else {
            // This must produce compile error
            traffic_light.send(Event::Push);
            lamp.send(Event::Timer)
        }
    }
    traffic_light.send(Event::Reset);
    lamp.send(Event::Reset);
    // }
}
