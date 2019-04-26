mod analyze;
mod convert;
mod helper;

use crate::semantics::{Event, Expression, State, Transition};

impl Expression for Transition<'_> {
	fn current_state(&self) -> &State {
		&self.from
	}

	fn next_state(&self) -> &State {
		&self.to
	}

	fn event(&self) -> Option<&Event> {
		self.at.as_ref()
	}
}

#[cfg(test)]
mod pair {
	#![allow(clippy::unit_arg)]
	use crate::{error::Error, semantics::Transition, test, Scdlang};
	use std::convert::TryInto;

	pub type ParseResult = Result<(), Error>;

	#[test]
	fn into_transition() -> ParseResult {
		test::parse::expression(
			r#"
            A <- D
			A -> D @ C
        "#,
			|expression| {
				Ok(match expression.as_str() {
					"A <- D" => {
						let state: Transition = expression.try_into()?;
						assert_eq!(&state.from.name, "D");
						assert_eq!(&state.to.name, "A");
						assert!(state.at.is_none());
					}
					"A -> D @ C" => {
						let state: Transition = expression.try_into()?;
						assert_eq!(&state.from.name, "A");
						assert_eq!(&state.to.name, "D");
						let event = state.at.expect("struct Event");
						assert_eq!(&event.name, "C");
					}
					_ => unreachable!("{}", expression.as_str()),
				})
			},
		)
	}

	mod semantics_error {
		use super::*;
		use crate::semantics::analyze::SemanticCheck;
		use std::mem::ManuallyDrop;

		#[test]
		fn duplicate_transient_transition() -> ParseResult {
			test::parse::expression(
				r#"
				A -> B
				D <- A
			"#,
				|expression| {
					let options = ManuallyDrop::new(Scdlang::default());
					Ok(match expression.as_str() {
						"A -> B" => Transition::analyze_from(expression, &options).ok().map_or((), |_| ()),
						"D <- A" => {
							let error = Transition::analyze_from(expression, &options).err().expect("Error::Semantic");
							assert!(error.to_string().contains("A ->"), "multiple transient transition on state A");
						}
						_ => unreachable!("{}", expression.as_str()),
					})
				},
			)
		}

		#[test]
		fn duplicate_transition_with_same_trigger() -> ParseResult {
			test::parse::expression(
				r#"
				A -> B @ C
				D <- A @ C
			"#,
				|expression| {
					let options = ManuallyDrop::new(Scdlang::default());
					Ok(match expression.as_str() {
						"A -> B @ C" => Transition::analyze_from(expression, &options).ok().map_or((), |_| ()),
						"D <- A @ C" => {
							let error = Transition::analyze_from(expression, &options).err().expect("Error::Semantic");
							for message in &["A ->", "@ C"] {
								assert!(error.to_string().contains(message), "multiple transition on state A");
							}
						}
						_ => unreachable!("{}", expression.as_str()),
					})
				},
			)
		}

		mod ambigous_transition {
			use super::*;

			#[test]
			fn transient_transition_before_trigger() -> ParseResult {
				test::parse::expression(
					r#"
					A -> B
					A -> B @ C
				"#,
					|expression| {
						let options = ManuallyDrop::new(Scdlang::default());
						Ok(match expression.as_str() {
							"A -> B" => Transition::analyze_from(expression, &options).ok().map_or((), |_| ()),
							"A -> B @ C" => {
								let error = Transition::analyze_from(expression, &options).err().expect("Error::Semantic");
								assert!(error.to_string().contains("A ->"), "multiple transition on state A");
							}
							_ => unreachable!("{}", expression.as_str()),
						})
					},
				)
			}

			#[test]
			fn transient_transition_after_trigger() -> ParseResult {
				test::parse::expression(
					r#"
					A -> B @ C
					A -> B
				"#,
					|expression| {
						let options = ManuallyDrop::new(Scdlang::default());
						Ok(match expression.as_str() {
							"A -> B @ C" => Transition::analyze_from(expression, &options).ok().map_or((), |_| ()),
							"A -> B" => {
								let error = Transition::analyze_from(expression, &options).err().expect("Error::Semantic");
								assert!(error.to_string().contains("A ->"), "multiple transition on state A");
							}
							_ => unreachable!("{}", expression.as_str()),
						})
					},
				)
			}
		}
	}
}
