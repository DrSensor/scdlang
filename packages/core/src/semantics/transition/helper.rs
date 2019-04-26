pub(super) mod prelude {
	pub use crate::{
		error::{Error as ScdlError, PestError},
		grammar::*,
		semantics::analyze::TokenPair,
		utils::naming,
		Scdlang,
	};
	pub use pest::{error::ErrorVariant, iterators::Pair, Span};
	pub use std::convert::TryInto;
}

pub(super) mod get {
	use super::{get, prelude::*};
	use crate::semantics::*;

	pub fn state<'t>(current: naming::Name<'t>, next: naming::Name<'t>, kind: &'t StateType) -> (State<'t>, State<'t>) {
		(State { name: current, kind }, State { name: next, kind })
	}

	type Tuple<'target> = (Rule, naming::Name<'target>);
	pub fn transition(pair: TokenPair) -> Tuple {
		let mut ops = Rule::EOI;
		let mut target = naming::Name::Unquoted("");

		for span in pair.into_inner() {
			match span.as_rule() {
				Name::state => target = get::name(span),
				Symbol::arrow::right | Symbol::arrow::left => ops = span.as_rule(),
				_ => unreachable!("Rule::{:?}", span.as_rule()),
			}
		}

		(ops, target)
	}

	pub fn trigger(pair: TokenPair) -> naming::Name {
		let mut event = naming::Name::Unquoted("");

		for span in pair.into_inner() {
			match span.as_rule() {
				Name::event => event = get::name(span),
				Symbol::at => { /* reserved when Internal Event is implemented */ }
				_ => unreachable!("Rule::{:?}", span.as_rule()),
			}
		}

		event
	}

	pub fn name(pair: TokenPair) -> naming::Name {
		use naming::Name::{self, *};
		let mut name = Name::Unquoted("");

		for span in pair.into_inner() {
			match span.as_rule() {
				Rule::PASCAL_CASE => name = Unquoted(span.as_str()),
				Rule::QUOTED => name = Quoted(span.as_str().trim_matches(|c| c == '\'' || c == '"' || c == '`')),
				_ => unreachable!("Rule::{:?}", span.as_rule()),
			}
		}

		name
	}
}
