mod helper;

use helper::{get, prelude::*};

pub use std::convert::TryFrom;
impl<'t> TryFrom<TokenPair<'t>> for Transition<'t> {
	type Error = ScdlError;
	fn try_from(pair: TokenPair<'t>) -> Result<Self, Self::Error> {
		use ScdlError::*;
		let rule = pair.as_rule();

		let mut lhs = "";
		let mut ops = Rule::EOI;
		let mut rhs = "";

		if let Rule::expression = rule {
			// determine the lhs, rhs, and operators
			for span in pair.into_inner() {
				let ident = span.as_str();
				match span.as_rule() {
					Rule::StateName => lhs = ident,
					Rule::transition => {
						// TODO: waiting for https://github.com/rust-lang/rfcs/pull/2649 (Destructuring without `let`)
						let (operators, target) = get::transition(span)?;
						rhs = target;
						ops = operators;
					}
					_ => unreachable!(),
				}
			}

			// determine the current, next, and type of the State
			let (current_state, next_state) = match ops {
				Rule::TransitionTo => get::state(lhs, rhs, &StateType::Atomic),
				_ => unreachable!(),
			};

			// register into Transition graph
			Ok(Transition {
				from: current_state,
				to: next_state,
				kind: TransitionType::External,
			})
		} else {
			Err(WrongRule(rule))
		}
	}
}

#[cfg(test)]
mod pair {
	#![allow(clippy::unit_arg)]
	use crate::{error::Error, semantics::Transition, test};
	use std::convert::TryInto;

	pub type ParseResult = Result<(), Error>;

	#[test]
	fn into_transition() -> ParseResult {
		test::parse::expression(
			r#"
            A -> B
        "#,
			|expression| {
				Ok(match expression.as_str() {
					"A -> B" => {
						let state: Transition = expression.try_into()?;
						assert_eq!(state.from.name, "A");
						assert_eq!(state.to.name, "B");
					}
					_ => unreachable!(),
				})
			},
		)
	}
}
