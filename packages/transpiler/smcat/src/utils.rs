use super::{State, StateType};
use scdlang_core::utils::naming::Name;
use std::iter::FromIterator;

pub(super) trait IntoState: Into<String> {
	fn into_type(self, kind: StateType) -> State;
}

pub(super) trait MergeStates: FromIterator<State> {
	fn merge(&mut self, states: &[State]);
}

impl MergeStates for Vec<State> {
	fn merge(&mut self, states: &[State]) {
		for state in states {
			if !self.iter().any(|s| s.name == state.name) {
				self.push(state.to_owned());
			}
		}
	}
}

impl IntoState for Name<'_> {
	fn into_type(self, kind: StateType) -> State {
		State {
			name: self.into(),
			state_type: kind,
			..Default::default()
		}
	}
}
