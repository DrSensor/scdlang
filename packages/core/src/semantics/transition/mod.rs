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
		let mut event = None;

		if let Rule::expression = rule {
			// determine the lhs, rhs, and operators
			for span in pair.into_inner() {
				let ident = span.as_str();
				match span.as_rule() {
					Name::state => lhs = ident,
					Rule::transition => {
						// TODO: waiting for https://github.com/rust-lang/rfcs/pull/2649 (Destructuring without `let`)
						let (operators, target) = get::transition(span)?;
						rhs = target;
						ops = operators;
					}
					Rule::trigger => {
						event = Some(Event {
							name: get::trigger(span)?,
						})
					}
					_ => unreachable!(),
				}
			}

			// determine the current, next, and type of the State
			let (current_state, next_state) = match ops {
				Symbol::to => get::state(lhs, rhs, &StateType::Atomic),
				_ => unreachable!(),
			};

			// register into Transition graph
			Ok(Transition {
				from: current_state,
				to: next_state,
				at: event,
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
			A -> D @ C
        "#,
			|expression| {
				Ok(match expression.as_str() {
					"A -> B" => {
						let state: Transition = expression.try_into()?;
						assert_eq!(state.from.name, "A");
						assert_eq!(state.to.name, "B");
						assert!(state.at.is_none());
					}
					"A -> D @ C" => {
						let state: Transition = expression.try_into()?;
						assert_eq!(state.from.name, "A");
						assert_eq!(state.to.name, "D");
						let event = state.at.expect("struct Event");
						assert_eq!(event.name, "C");
					}
					_ => unreachable!(),
				})
			},
		)
	}
}
