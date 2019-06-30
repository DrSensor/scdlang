pub(super) mod prelude {
	pub use crate::{
		error::{Error as ScdlError, PestError},
		grammar::*,
		semantics::analyze::TokenPair,
		Scdlang,
	};
	pub use pest::{error::ErrorVariant, iterators::Pair, Span};
	pub use std::convert::TryInto;
}

pub(super) mod get {
	use super::prelude::*;
	use crate::semantics::*;

	pub fn state<'t>(current: &'t str, next: &'t str, kind: &'t StateType) -> (State<'t>, State<'t>) {
		(State { name: current, kind }, State { name: next, kind })
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

	pub fn trigger(pair: TokenPair) -> &str {
		let mut event = "";

		for span in pair.into_inner() {
			match span.as_rule() {
				Name::event => event = span.as_str(),
				Symbol::at => { /* reserved when Internal Event is implemented */ }
				_ => unreachable!("Rule::{:?}", span.as_rule()),
			}
		}

		event
	}
}
