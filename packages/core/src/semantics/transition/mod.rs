mod analyze;
mod convert;
mod helper;
mod iter;

use crate::{
	semantics::{analyze::SemanticCheck, Expression, Found, Kind, Transition},
	utils::naming::Name,
	Error,
};

impl Expression for Transition<'_> {
	fn current_state(&self) -> Name {
		self.from.name.into()
	}

	fn next_state(&self) -> Option<Name> {
		self.to.as_ref().map(|state| state.name.into())
	}

	fn event(&self) -> Option<Name> {
		self.at.as_ref().and_then(|event| event.name).map(Into::into)
	}

	fn guard(&self) -> Option<Name> {
		self.at.as_ref().and_then(|event| event.guard).map(Into::into)
	}

	fn action(&self) -> Option<Name> {
		self.run.as_ref().map(|action| action.name.into())
	}

	fn semantic_check(&self) -> Result<Found, Error> {
		Ok(match self.check_error()? {
			Some(message) => Found::Error(message),
			None => Found::None,
		})
	}
}

impl<'t> From<Transition<'t>> for Kind<'t> {
	fn from(transition: Transition<'t>) -> Self {
		Kind::Expression(Box::new(transition))
	}
}

#[cfg(test)]
mod pair {
	#![allow(clippy::unit_arg)]
	use crate::{error::Error, semantics::Transition, test, Scdlang};

	pub type ParseResult = Result<(), Error>;

	#[test]
	fn normal_transition() -> ParseResult {
		test::parse::expression(
			r#"
			A <- D
			A -> D @ C
		"#,
			|expression| {
				Ok(match expression.as_str() {
					"A <- D" => {
						let state: Transition = expression.into();
						assert_eq!(state.from.name, "D");
						assert_eq!(state.to.map(|n| n.name), Some("A"));
						assert!(state.at.is_none());
					}
					"A -> D @ C" => {
						let state: Transition = expression.into();
						let event = state.at.expect("struct Event");
						assert_eq!(state.from.name, "A");
						assert_eq!(state.to.map(|n| n.name), Some("D"));
						assert_eq!(event.name, Some("C"));
					}
					_ => unreachable!("{}", expression.as_str()),
				})
			},
		)
	}

	#[test]
	fn toggle_transition() -> ParseResult {
		test::parse::expression(
			r#"
			B <-> E
			A <-> D @ C
		"#,
			|expression| {
				Ok(match expression.as_str() {
					"B <-> E" => {
						// contains state name B or E
						let (mut states, mut transitions) =
							(["B", "E"].iter(), Transition::from(expression.clone()).into_iter());
						assert!(states.any(|s| transitions.any(|t| t.from.name == *s || t.to.map(|n| n.name) == Some(*s))));

						let [state_b, state_e] = Transition::from(expression).into_iter().collect::<[Transition; 2]>();
						assert_eq!(Some(state_b.from.name), state_e.to.map(|n| n.name));
						assert_eq!(state_b.to.map(|n| n.name), Some(state_e.from.name));
						assert_eq!(state_b.at.is_none(), state_e.at.is_none());
					}
					"A <-> D @ C" => {
						// contains state name A or D
						let (mut states, mut transitions) =
							(["A", "D"].iter(), Transition::from(expression.clone()).into_iter());
						assert!(states.any(|s| transitions.any(|t| t.from.name == *s || t.to.map(|n| n.name) == Some(*s))));

						let [state_a, state_d] = Transition::from(expression).into_iter().collect::<[Transition; 2]>();
						let [event_a, event_d] = [state_a.at.expect("struct Event"), state_d.at.expect("struct Event")];
						assert_eq!(Some(state_a.from.name), state_d.to.map(|n| n.name));
						assert_eq!(state_a.to.map(|n| n.name), Some(state_d.from.name));
						assert!([event_a.name, event_d.name].iter().all(|e| *e == Some("C")));
					}
					_ => unreachable!("{}", expression.as_str()),
				})
			},
		)
	}

	#[test]
	/// in loop transition, order of expanded transition is matter to avoid caching wrong expression.
	fn loop_transition() -> ParseResult {
		test::parse::expression(
			r#"
			X ->> Z // should I make it as syntax error 🤔
			A ->> D @ C
			D <<- E @ B
			->> E @ C
		"#,
			|expression| {
				Ok(match expression.as_str() {
					"X ->> Z // should I make it as syntax error 🤔" => {
						// contains state name X or Z
						let (mut states, mut transitions) =
							(["X", "Z"].iter(), Transition::from(expression.clone()).into_iter());
						assert!(states.any(|s| transitions.any(|t| t.from.name == *s || t.to.map(|n| n.name) == Some(*s))));

						let [state_z, state_x] = Transition::from(expression).into_iter().collect::<[Transition; 2]>();
						assert_eq!(state_z.from.name, "Z");
						assert_eq!(Some(state_z.from.name), state_z.to.map(|n| n.name));
						assert_eq!(state_x.from.name, "X");
						assert_eq!(state_x.to.map(|n| n.name), Some(state_z.from.name));
						assert_eq!(state_z.at.is_none(), state_x.at.is_none());
					}
					"A ->> D @ C" => {
						// contains state name A or D
						let (mut states, mut transitions) =
							(["A", "D"].iter(), Transition::from(expression.clone()).into_iter());
						assert!(states.any(|s| transitions.any(|t| t.from.name == *s || t.to.map(|n| n.name) == Some(*s))));

						let [state_d, state_a] = Transition::from(expression).into_iter().collect::<[Transition; 2]>();
						let [event_d, event_a] = [state_d.at.expect("struct Event"), state_a.at.expect("struct Event")];
						assert_eq!(state_d.from.name, "D");
						assert_eq!(Some(state_d.from.name), state_d.to.map(|n| n.name));
						assert_eq!(state_a.from.name, "A");
						assert_eq!(state_a.to.map(|n| n.name), Some(state_d.from.name));
						assert!([event_d.name, event_a.name].iter().all(|e| *e == Some("C")));
					}
					"D <<- E @ B" => {
						// contains state name E or D
						let (mut states, mut transitions) =
							(["E", "D"].iter(), Transition::from(expression.clone()).into_iter());
						assert!(states.any(|s| transitions.any(|t| t.from.name == *s || t.to.map(|n| n.name) == Some(*s))));

						let [state_d, state_e] = Transition::from(expression).into_iter().collect::<[Transition; 2]>();
						let [event_d, event_e] = [state_d.at.expect("struct Event"), state_e.at.expect("struct Event")];
						assert_eq!(state_d.from.name, "D");
						assert_eq!(Some(state_d.from.name), state_e.to.as_ref().map(|n| n.name));
						assert_eq!(state_e.from.name, "E");
						assert_eq!(state_e.to.map(|n| n.name), Some(state_d.from.name));
						assert!([event_d.name, event_e.name].iter().all(|e| *e == Some("B")));
					}
					"->> E @ C" => Transition::from(expression).into_iter().for_each(|transition| {
						assert!([Some(transition.from.name), transition.to.map(|n| n.name)]
							.iter()
							.all(|e| *e == Some("E")));
						assert_eq!(transition.at.expect("struct Event").name, Some("C"));
					}),
					_ => unreachable!("{}", expression.as_str()),
				})
			},
		)
	}

	#[test]
	fn transient_loop_transition() -> ParseResult {
		test::parse::expression(
			r#"
			X >-> Z // should I make it as syntax error 🤔
			A >-> D @ C
			D <-< E @ B
		"#,
			|expression| {
				Ok(match expression.as_str() {
					"X >-> Z // should I make it as syntax error 🤔" => {
						// contains state name X or Z
						let (mut states, mut transitions) =
							(["X", "Z"].iter(), Transition::from(expression.clone()).into_iter());
						assert!(states.any(|s| transitions.any(|t| t.from.name == *s || t.to.map(|n| n.name) == Some(*s))));

						let [state_z, state_x] = Transition::from(expression).into_iter().collect::<[Transition; 2]>();
						assert_eq!(state_z.from.name, "Z");
						assert_eq!(Some(state_z.from.name), state_z.to.map(|n| n.name));
						assert_eq!(state_x.from.name, "X");
						assert_eq!(state_x.to.map(|n| n.name), Some(state_z.from.name));
						assert_eq!(state_z.at.is_none(), state_x.at.is_none());
					}
					"A >-> D @ C" => {
						// contains state name A or D
						let (mut states, mut transitions) =
							(["A", "D"].iter(), Transition::from(expression.clone()).into_iter());
						assert!(states.any(|s| transitions.any(|t| t.from.name == *s || t.to.map(|n| n.name) == Some(*s))));

						let [state_d, state_a] = Transition::from(expression).into_iter().collect::<[Transition; 2]>();
						assert_eq!(state_d.from.name, "D");
						assert_eq!(Some(state_d.from.name), state_d.to.map(|n| n.name));
						assert_eq!(state_d.at.expect("struct Event").name, Some("C"));
						assert_eq!(state_a.from.name, "A");
						assert_eq!(state_a.to.map(|n| n.name), Some(state_d.from.name));
						assert!(state_a.at.is_none());
					}
					"D <-< E @ B" => {
						// contains state name E or D
						let (mut states, mut transitions) =
							(["E", "D"].iter(), Transition::from(expression.clone()).into_iter());
						assert!(states.any(|s| transitions.any(|t| t.from.name == *s || t.to.map(|n| n.name) == Some(*s))));

						let [state_d, state_e] = Transition::from(expression).into_iter().collect::<[Transition; 2]>();
						assert_eq!(state_d.from.name, "D");
						assert_eq!(Some(state_d.from.name), state_e.to.as_ref().map(|n| n.name));
						assert_eq!(state_d.at.expect("struct Event").name, Some("B"));
						assert_eq!(state_e.from.name, "E");
						assert_eq!(state_e.to.map(|n| n.name), Some(state_d.from.name));
						assert!(state_e.at.is_none());
					}
					_ => unreachable!("{}", expression.as_str()),
				})
			},
		)
	}

	mod fix_issues {
		use super::*;
		use crate::semantics::analyze::SemanticAnalyze;
		use std::mem::ManuallyDrop;

		#[test]
		fn indirect_loop_transition() -> ParseResult {
			test::issue(20);
			test::parse::expression(
				r#"
				A ->> D @ Tok
				B ->> D @ Tok
				B -> D @ Tok
			"#,
				|expression| {
					let options = ManuallyDrop::new(Scdlang::default());
					Ok(match expression.as_str() {
						"A ->> D @ Tok" | "B -> D @ Tok" => Transition::analyze_from(expression, &options).map(|_| ())?,
						"B ->> D @ Tok" => {
							let error = Transition::analyze_from(expression, &options).err().expect("Error::Semantic");
							assert!(error.to_string().contains("B ->"), "multiple transition on state D");
						}
						_ => unreachable!("{}", expression.as_str()),
					})
				},
			)
		}
	}

	mod semantics_error {
		use super::*;
		use crate::semantics::analyze::SemanticAnalyze;
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
						"A -> B" => Transition::analyze_from(expression, &options).map(|_| ())?,
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
						"A -> B @ C" => Transition::analyze_from(expression, &options).map(|_| ())?,
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
							"A -> B" => Transition::analyze_from(expression, &options).map(|_| ())?,
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
							"A -> B @ C" => Transition::analyze_from(expression, &options).map(|_| ())?,
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
