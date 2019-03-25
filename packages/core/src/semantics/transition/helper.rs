pub mod prelude {
	pub use crate::{error::Error as ScdlError, grammar::Rule, semantics::*};
	pub use pest::iterators::Pair;
	pub type TokenPair<'i> = Pair<'i, Rule>;
}

pub mod get {
	use super::prelude::*;
	use Rule::*;
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
				StateName => target = ident,
				TransitionTo => ops = Some(span.as_rule()),
				_ => unreachable!(),
			}
		}

		Ok((ops.ok_or(MissingOperator)?, target))
	}
}
