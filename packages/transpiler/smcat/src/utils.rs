use super::{State, StateType};
use scdlang::utils::naming::Name;
use std::iter::FromIterator;

pub trait SplitToVec<T> {
	fn split_to_vec(self, separator: char) -> Vec<T>;
}

impl SplitToVec<String> for &str {
	fn split_to_vec(self, separator: char) -> Vec<String> {
		self.split(separator).map(String::from).collect()
	}
}

pub(super) trait MetaData {
	fn with_color(&mut self, color: &str) -> &mut Self;
	fn with_notes(&mut self, notes: &str) -> &mut Self;
}

impl MetaData for State {
	fn with_color(&mut self, color: &str) -> &mut Self {
		self.color = Some(color.to_string());
		self
	}

	fn with_notes(&mut self, notes: &str) -> &mut Self {
		self.note = Some(notes.split_to_vec('\n'));
		self
	}
}

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
			} else if state.color.is_some() {
				let pos = self
					.iter()
					.position(|s| s.name == state.name)
					.expect("any(|s| s.name == state.name)");
				self[pos].color = state.color.clone();
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
