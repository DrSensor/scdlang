use crate::{
	statechart::{State,WithInitial,TEvent}
};

#[derive(Debug)]
pub enum GlobalEvent {
	OkToGo
}
impl TEvent for GlobalEvent {
	type SharedEvent = Self;
}

#[derive(Debug)]
pub enum EventLight {
	Timer,
	Poweroff,
}
impl TEvent for EventLight {
	type SharedEvent = GlobalEvent;
}

#[derive(Debug)]
pub enum EventSwitch {
	Reset,
	Push,
}
impl TEvent for EventSwitch {
	type SharedEvent = GlobalEvent;
}

#[allow(non_snake_case)]
pub mod Event {
	pub use crate::{
		EventLight::Poweroff, EventLight::Timer, EventSwitch::Push, EventSwitch::Reset,
		GlobalEvent::OkToGo,
	};
}

#[derive(Debug)]
pub enum Light {
	Green,
	Yellow,
	Red,
	Final(),
}

#[derive(Debug)]
pub enum Switch {
	On,
	Off,
}

impl State for Light {
	type Event = EventLight;
	fn transition<'s>(&self, event: &'s Self::Event) -> Option<Self> {
		match event {
			EventLight::Timer => match self {
				Light::Green => Some(Light::Yellow),
				Light::Yellow => Some(Light::Red),
				Light::Red => Some(Light::Green),
				_ => None,
			},
			EventLight::Poweroff => Some(match self {
				_ => Light::Final(),
			}),
		}
	}

    fn is_final(&self) -> bool {
		if let Light::Final() = self {
			true
		} else {
			false
		}
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

	fn is_final(&self) -> bool {
		false
	}
}
