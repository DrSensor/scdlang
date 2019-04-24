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
			let ident = span.as_str();
			match span.as_rule() {
				Name::state => target = ident,
				Symbol::to => ops = Some(span.as_rule()),
				_ => unreachable!(),
			}
		}

		Ok((ops.ok_or(MissingOperator)?, target))
	}

	pub fn trigger(pair: TokenPair<'_>) -> Result<&str, ScdlError> {
		let mut event = "";

		for span in pair.into_inner() {
			let ident = span.as_str();
			match span.as_rule() {
				Name::event => event = ident,
				Symbol::at => { /* reserved when Internal Event is implemented */ }
				_ => unreachable!(),
			}
		}

		Ok(event)
	}
}
