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
