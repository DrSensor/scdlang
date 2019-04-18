mod helper;

use crate::{cache, Scdlang};
use helper::{get, prelude::*};

pub use std::convert::TryFrom;
impl<'t> TryFrom<(&Scdlang<'t>, TokenPair<'t>)> for Transition<'t> {
	type Error = ScdlError;
	fn try_from((options, pair): (&Scdlang<'t>, TokenPair<'t>)) -> Result<Self, Self::Error> {
		use ScdlError::*;
		let rule = pair.as_rule();
		let span = pair.as_span();

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

			// analyze semantics error
			let mut t = cache::transition()?;
			let (current, target) = (current_state.name.to_string(), next_state.name.to_string());
			if let Some(trigger) = event.clone() {
				if let Some(prev_target) = t.insert(
					cache::Transition::new(current.clone(), trigger.name.to_string()),
					target.clone(),
				) {
					let message = format!(
						"duplicate transition: {} -> {},{} @ {}",
						current, target, prev_target, trigger.name
					);
					return Err(options.err_from_span(span, message).into());
				}
			} else if let Some(prev_target) = t.insert(current.clone().into(), target.clone()) {
				let message = format!("duplicate transient transition: {} -> {},{}", current, target, prev_target);
				return Err(options.err_from_span(span, message).into());
			}

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

	mod semantics_error {
		use super::*;
		use crate::prelude::*;

		#[test]
		fn duplicate_transient_transition() -> ParseResult {
			test::parse::expression(
				r#"
				A -> B
				A -> D
			"#,
				|expression| {
					Ok(match expression.as_str() {
						"A -> B" => Transition::try_from(expression).ok().map_or((), |_| ()),
						"A -> D" => {
							let error = Transition::try_from(expression).err().expect("Error::Semantic");
							assert!(error.to_string().contains("A ->"), "multiple transient transition on state A");
						}
						_ => unreachable!(),
					})
				},
			)
		}

		#[test]
		fn duplicate_transition_with_same_trigger() -> ParseResult {
			test::parse::expression(
				r#"
				A -> B @ C
				A -> D @ C
			"#,
				|expression| {
					Ok(match expression.as_str() {
						"A -> B @ C" => Transition::try_from(expression).ok().map_or((), |_| ()),
						"A -> D @ C" => {
							let error = Transition::try_from(expression).err().expect("Error::Semantic");
							for message in &["A ->", "@ C"] {
								assert!(error.to_string().contains(message), "multiple transition on state A");
							}
						}
						_ => unreachable!(),
					})
				},
			)
		}

		mod ambigous_transition {
			use super::*;

			#[test]
			#[ignore]
			fn transient_transition_before_trigger() -> ParseResult {
				test::parse::expression(
					r#"
					A -> B
					A -> B @ C
				"#,
					|expression| {
						Ok(match expression.as_str() {
							"A -> B" => Transition::try_from(expression).ok().map_or((), |_| ()),
							"A -> B @ C" => {
								let error = Transition::try_from(expression).err().expect("Error::Semantic");
								assert!(error.to_string().contains("A ->"), "multiple transition on state A");
							}
							_ => unreachable!(),
						})
					},
				)
			}

			#[test]
			#[ignore]
			fn transient_transition_after_trigger() -> ParseResult {
				test::parse::expression(
					r#"
					A -> B @ C
					A -> B
				"#,
					|expression| {
						Ok(match expression.as_str() {
							"A -> B @ C" => Transition::try_from(expression).ok().map_or((), |_| ()),
							"A -> B" => {
								let error = Transition::try_from(expression).err().expect("Error::Semantic");
								assert!(error.to_string().contains("A ->"), "multiple transition on state A");
							}
							_ => unreachable!(),
						})
					},
				)
			}
		}
	}
}
