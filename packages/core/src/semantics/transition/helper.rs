pub(super) mod prelude {
	pub use crate::{
		error::{Error as ScdlError, PestError},
		grammar::*,
		semantics::analyze::TokenPair,
		Scdlang,
	};
	pub use pest::{error::ErrorVariant, iterators::Pair, Span};
	pub use std::{convert::TryInto, fmt::Write, sync::MutexGuard};
}

pub(super) mod get {
	use super::prelude::*;
	use crate::semantics::*;

	pub fn state<'t>(current: &'t str, next: Option<&'t str>, kind: &'t StateType) -> (State<'t>, Option<State<'t>>) {
		(State { name: current, kind }, next.map(|name| State { name, kind }))
	}

	type Tuple<'target> = (Rule, &'target str);
	pub fn transition(pair: TokenPair) -> Tuple {
		let mut ops = Rule::EOI;
		let mut target = "";

		for span in pair.into_inner() {
			match span.as_rule() {
				Name::state => target = span.as_str(),
				Symbol::arrow::both
				| Symbol::arrow::right
				| Symbol::arrow::left
				| Symbol::double_arrow::right
				| Symbol::double_arrow::left
				| Symbol::tail_arrow::right
				| Symbol::tail_arrow::left => ops = span.as_rule(),
				_ => unreachable!("Rule::{:?}", span.as_rule()),
			}
		}

		(ops, target)
	}

	pub fn arrowless_transition(pair: TokenPair) -> (&str, Event, Action) {
		use super::get;
		let (mut state, mut event, mut action) = ("", Event::default(), Action::default());

		for span in pair.into_inner() {
			match span.as_rule() {
				Rule::StateName => state = span.as_str(),
				Rule::trigger => event = get::trigger(span),
				Rule::action => action = Action { name: get::action(span) },
				_ => unreachable!("Rule::{:?}", span.as_rule()),
			}
		}

		(state, event, action)
	}

	pub fn action(pair: TokenPair) -> &str {
		let mut action = "";

		for span in pair.into_inner() {
			match span.as_rule() {
				Name::action => action = span.as_str(),
				Symbol::triangle::right => { /* reserved when exit/entry is implemented */ }
				_ => unreachable!("Rule::{:?}", span.as_rule()),
			}
		}

		action
	}

	pub fn trigger(pair: TokenPair) -> Event {
		let (mut event, mut guard) = (None, None);

		for span in pair.into_inner() {
			match span.as_rule() {
				Name::event => event = Some(span.as_str()),
				Name::guard => guard = Some(span.as_str()),
				Symbol::at => { /* reserved when Internal Event is implemented */ }
				_ => unreachable!("Rule::{:?}", span.as_rule()),
			}
		}

		Event { name: event, guard }
	}
}

/// analyze.rs helpers for transforming key for caches
pub(super) mod transform_key {
	use crate::semantics::*;

	// WARNING: not performant because of using concatenated String as a key which cause filtering
	impl From<&Event<'_>> for String {
		fn from(event: &Event<'_>) -> Self {
			format!("{}?{}", event.name.unwrap_or(""), event.guard.unwrap_or(""))
		}
	}

	impl<'i> EventKey<'i> for &'i Option<String> {}
	pub trait EventKey<'i>: Into<Option<&'i String>> {
		fn has_trigger(self) -> bool {
			self.into().filter(|e| is_empty(e.rsplit('?'))).is_some()
		}
		fn has_guard(self) -> bool {
			self.into().filter(|e| is_empty(e.split('?'))).is_some()
		}
		fn get_guard(self) -> Option<&'i str> {
			self.into().and_then(|e| none_empty(e.split('?')))
		}
		fn get_trigger(self) -> Option<&'i str> {
			self.into().and_then(|e| none_empty(e.rsplit('?')))
		}
		fn guards_with_same_trigger(self, trigger: Option<&'i str>) -> Option<&'i str> {
			self.into()
				.filter(|e| none_empty(e.rsplit('?')) == trigger)
				.and_then(|e| none_empty(e.split('?')))
		}
		fn triggers_with_same_guard(self, guard: Option<&'i str>) -> Option<&'i str> {
			self.into()
				.filter(|e| none_empty(e.split('?')) == guard)
				.and_then(|e| none_empty(e.rsplit('?')))
		}
		fn as_expression(self) -> String {
			self.into().map(String::as_str).as_expression()
		}
	}

	impl<'o> Trigger<'o> for &'o Option<&'o str> {}
	pub trait Trigger<'o>: Into<Option<&'o &'o str>> {
		fn as_expression(self) -> String {
			self.into()
				.map(|s| {
					format!(
						" @ {trigger}{guard}",
						trigger = none_empty(s.rsplit('?')).unwrap_or_default(),
						guard = none_empty(s.split('?'))
							.filter(|_| s.contains('?'))
							.map(|g| format!("[{}]", g))
							.unwrap_or_default(),
					)
				})
				.unwrap_or_default()
		}
		fn as_key(self, guard: &str) -> Option<String> {
			Some(format!("{}?{}", self.into().unwrap_or(&""), guard))
		}
	}

	fn is_empty<'a>(split: impl Iterator<Item = &'a str>) -> bool {
		none_empty(split).is_some()
	}

	fn none_empty<'a>(split: impl Iterator<Item = &'a str>) -> Option<&'a str> {
		split.last().filter(|s| !s.is_empty())
	}
}
