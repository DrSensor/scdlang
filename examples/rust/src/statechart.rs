/** TODO: Support Global Event
 * problems:
 * 1. How to do it while retaining type safety 🤔
 */
use core::fmt::Debug;

pub trait State: Sized + Debug {
	type Event: TEvent;
	//            how to make 👇 also support global event
	fn transition<'s>(&self, event: &'s Self::Event) -> Option<Self>;
	fn is_final(&self) -> bool;
}

pub trait WithInitial: State {
	fn initial() -> Self;
}

pub trait TEvent: Debug {
	type SharedEvent: TEvent;
}

// #region Declaration
#[derive(Debug)]
pub enum GlobalEvent {
	OkToGo
}
impl TEvent for GlobalEvent {
	type SharedEvent = Self;
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

	//        let's focus on 👇 to make it support global event
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
