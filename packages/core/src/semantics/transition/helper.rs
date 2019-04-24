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
	use ScdlError::*;

	pub fn state<'t>(current: &'t str, next: &'t str, kind: &'t StateType) -> (State<'t>, State<'t>) {
		(State { name: current, kind }, State { name: next, kind })
	}

	type Tuple<'target> = (Rule, &'target str);
	pub fn transition(pair: TokenPair<'_>) -> Result<Tuple, ScdlError> {
		let mut ops = None;
		let mut target = "";

		for span in pair.into_inner() {
			match span.as_rule() {
				Name::state => target = span.as_str(),
				Symbol::arrow::right | Symbol::arrow::left => ops = Some(span.as_rule()),
				_ => unreachable!(),
			}
		}

		Ok((ops.ok_or(MissingOperator)?, target))
	}

	pub fn trigger(pair: TokenPair<'_>) -> Result<&str, ScdlError> {
		let mut event = "";

		for span in pair.into_inner() {
			match span.as_rule() {
				Name::event => event = span.as_str(),
				Symbol::at => { /* reserved when Internal Event is implemented */ }
				_ => unreachable!(),
			}
		}

		Ok(event)
	}
}
