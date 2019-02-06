/** TODO: Support Global Event
 * problems:
 * 1. How to do it while retaining type safety 🤔
 */
use core::fmt::Debug;

pub trait State: Sized + Debug {
    type Event;
    fn transition<'s>(&self, event: &'s Self::Event) -> Option<Self>;
    fn is_final(&self) -> bool;
}

pub trait WithInitial: State {
    fn initial() -> Self;
}

// pub trait TEvent: Debug {}

// #region Declaration
// #[derive(Debug)]
// pub enum GlobalEvent {
//     OkToGo
// }
// impl TEvent for GlobalEvent {}

#[derive(Debug)]
pub enum EventLight {
    Timer,
    Poweroff,
}
// impl TEvent for EventLight {}

#[derive(Debug)]
pub enum EventSwitch {
    Reset,
    Push,
}
// impl TEvent for EventSwitch {}

#[allow(non_snake_case)]
pub mod Event {
    pub use crate::statechart::{
        // GlobalEvent::OkToGo as OkToGo,
        EventLight::Timer as Timer,
        EventLight::Poweroff as Poweroff,
        EventSwitch::Push as Push,
        EventSwitch::Reset as Reset
    };
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
    type Event = EventLight;
    fn transition<'s>(&self, event: &'s Self::Event) -> Option<Self> {
        match event {
            EventLight::Timer => match self {
                Light::Green => Some(Light::Yellow),
                Light::Yellow => Some(Light::Red),
                Light::Red => Some(Light::Green),
                _ => None
            },
            EventLight::Poweroff => Some(match self {
                _ => Light::Final()
            })
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
    type Event = EventSwitch;
    fn transition<'s>(&self, event: &'s Self::Event) -> Option<Self> {
        match event {
            EventSwitch::Push => Some(match self {
                Switch::Off => Switch::On,
                Switch::On => Switch::Off,
            }),
            EventSwitch::Reset => Some(Self::initial()),
        }
    }

    fn is_final(&self) -> bool { false }
}

pub struct Machine<T: State> {
    pub state: T,
    finish: Option<fn(&T, &T::Event)>,
    transition: Option<fn(&T, &T, T::Event)>, // fn(&current_state, &previous_state)
}

impl<T: WithInitial> Machine<T> {
    pub fn new() -> Self {
        Self {
            state: T::initial(),
            finish: None,
            transition: None,
        }
    }
}

impl<T: State> Machine<T> {
    pub fn from(initial: T) -> Self {
        Self {
            state: initial,
            finish: None,
            transition: None,
        }
    }

    pub fn on_final(&mut self, callback: fn(&T, &T::Event)) {
        self.finish = Some(callback);
    }

    pub fn on_transition(&mut self, callback: fn(&T, &T, T::Event)) {
        self.transition = Some(callback);
    }

    pub fn send(&mut self, event: T::Event) {
        if let Some(_state) = self.state.transition(&event) {
            if _state.is_final() {
                if let Some(stale) = self.finish {
                    stale(&self.state, &event);
                }
            } else {
                if let Some(transition) = self.transition {
                    transition(&_state, &self.state, event);
                }
            }
            self.state = _state;
        }
    }
}