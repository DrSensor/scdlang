#![allow(clippy::unit_arg)]
mod schema;
mod utils;
pub use scdlang::Transpiler;

use scdlang::{
	prelude::*,
	semantics::{Found, Kind},
	Scdlang,
};
use schema::*;
use serde::Serialize;
use std::{error, fmt, mem::ManuallyDrop};
use utils::*;

#[derive(Default, Serialize)]
/** Transpiler Scdlang → State Machine Cat (JSON).

# Examples
```no_run
let smcat = Machine::new();

smcat.configure().with_err_path("test.scl");
parser.parse("A -> B")?;

println!("{}", parser.to_string());
``` */
pub struct Machine<'a> {
	#[serde(skip)]
	builder: Scdlang<'a>,

	#[serde(flatten)]
	schema: Coordinate, // TODO: replace with 👇 when https://github.com/serde-rs/serde/issues/1507 resolved
	                    // schema: mem::ManuallyDrop<StateChart>,
}

impl<'a> Parser<'a> for Machine<'a> {
	fn configure(&mut self) -> &mut Builder<'a> {
		&mut self.builder
	}

	fn parse(&mut self, source: &str) -> Result<(), DynError> {
		self.clean_cache()?;
		let ast = ManuallyDrop::new(Self::try_parse(source, self.builder.to_owned())?);
		Ok(self.schema = ast.schema.to_owned()) // FIXME: expensive clone
	}

	fn insert_parse(&mut self, source: &str) -> Result<(), DynError> {
		let mut ast = ManuallyDrop::new(Self::try_parse(source, self.builder.to_owned())?);
		self.schema.states.merge(&ast.schema.states);
		match (&mut self.schema.transitions, &mut ast.schema.transitions) {
			(Some(origin), Some(parsed)) => origin.extend_from_slice(parsed),
			(None, _) => self.schema.transitions = ast.schema.transitions.to_owned(),
			_ => {}
		}
		Ok(())
	}

	#[allow(clippy::match_bool)]
	fn try_parse(source: &str, builder: Scdlang<'a>) -> Result<Self, DynError> {
		use StateType::*;
		let mut schema = Coordinate::default();

		for kind in builder.iter_from(source)? {
			match kind {
				Kind::Expression(expr) => {
					let (color, note) = match builder.semantic_error {
						false => match expr.semantic_check()? {
							Found::Error(message) => (Some("red".to_string()), Some(message.split_to_vec('\n'))),
							_ => (None, None),
						},
						true => (None, None),
					};
					schema.states.merge(&{
						let mut states = [expr.current_state().into_type(Regular), expr.next_state().into_type(Regular)];
						if let Some(color) = &color {
							states.iter_mut().for_each(|s| {
								s.with_color(color);
							});
						}
						states
					});
					let (event, cond, action) = (
						expr.event().map(|e| e.into()),
						expr.guard().map(|e| e.into()),
						expr.action().map(|e| e.into()),
					);
					#[rustfmt::skip]
					let transition = Transition {
						from: expr.current_state().into(),
						to: expr.next_state().into(),
						label: if event.is_some() || cond.is_some() || action.is_some() {
							let action_cond = action.is_some() || cond.is_some();
							let (event, cond, action) = (event.clone(), cond.clone(), action.clone());
							Some(format!( // add spacing on each token
								"{on}{is}{run}",
								on = event.map(|event| format!("{}{spc}", event, spc = if action_cond { " " } else { "" },))
									.unwrap_or_default(),
								is = cond.map(|guard| format!("[{}]{spc}", guard, spc = if action.is_some() { " " } else { "" },))
									.unwrap_or_default(),
								run = action.map(|act| format!("/ {}", act)).unwrap_or_default()
							))
						} else { None }, event, cond, action, color, note
					};
					match &mut schema.transitions {
						Some(transitions) => transitions.push(transition),
						None => schema.transitions = Some(vec![transition]),
					};
				}
				_ => unimplemented!("TODO: implement the rest on the next update"),
			}
		}

		Ok(Machine { schema, builder })
	}
}

impl Machine<'_> {
	/// Create new StateMachine.
	/// Use this over `Machine::default()`❗
	pub fn new() -> Self {
		let mut builder = Scdlang::new();
		builder.auto_clear_cache(false);
		Self {
			builder,
			schema: Coordinate::default(),
		}
	}
}

impl Drop for Machine<'_> {
	fn drop(&mut self) {
		self.flush_cache().expect("smcat: Deadlock");
	}
}

impl fmt::Display for Machine<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", serde_json::to_string_pretty(&self.schema).map_err(|_| fmt::Error)?)
	}
}

type DynError = Box<dyn error::Error>;

#[cfg(test)]
mod test {
	use super::*;
	use assert_json_diff::assert_json_eq;
	use serde_json::json;

	#[test]
	fn transient_transition() -> Result<(), DynError> {
		let mut machine = Machine::new();
		machine.parse("AlphaGo -> BetaRust")?;

		Ok(assert_json_eq!(
			json!({
				"states": [{
						"name": "AlphaGo",
						"type": "regular"
					}, {
						"name": "BetaRust",
						"type": "regular"
				}],
				"transitions": [{
					"from": "AlphaGo",
					"to": "BetaRust"
				}]
			}),
			json!(machine)
		))
	}

	#[test]
	fn eventful_transition() -> Result<(), DynError> {
		let mut machine = Machine::new();
		machine.parse(
			"A -> B @ CarlieCaplin
			A <- B @ CarlieCaplin
			A -> D @ EnhancedErlang",
		)?;

		Ok(assert_json_eq!(
			json!({
				"states": [{
						"name": "A",
						"type": "regular"
					}, {
						"name": "B",
						"type": "regular"
					}, {
						"name": "D",
						"type": "regular"
				}],
				"transitions": [{
						"from": "A",
						"to": "B",
						"event": "CarlieCaplin",
						"label": "CarlieCaplin"
					}, {
						"from": "B",
						"to": "A",
						"event": "CarlieCaplin",
						"label": "CarlieCaplin"
					}, {
						"from": "A",
						"to": "D",
						"event": "EnhancedErlang",
						"label": "EnhancedErlang"
				}]
			}),
			json!(machine)
		))
	}

	#[test]
	fn no_clear_cache() {
		let mut machine = Machine::new();
		machine.parse("A -> B").expect("Nothing happened");
		machine.insert_parse("A -> C").expect_err("Duplicate transition");

		assert_json_eq!(
			json!({
				"states": [{
						"name": "A",
						"type": "regular"
					}, {
						"name": "B",
						"type": "regular"
				}],
				"transitions": [{
					"from": "A",
					"to": "B"
				}]
			}),
			json!(machine)
		)
	}

	#[test]
	fn clear_cache() {
		let mut machine = Machine::new();
		machine.insert_parse("A -> B").expect("Nothing happened");
		machine.parse("A -> C").expect("Clear cache and replace schema");

		assert_json_eq!(
			json!({
				"states": [{
						"name": "A",
						"type": "regular"
					}, {
						"name": "C",
						"type": "regular"
				}],
				"transitions": [{
					"from": "A",
					"to": "C"
				}]
			}),
			json!(machine)
		)
	}

	#[test]
	fn disable_semantic_error() -> Result<(), DynError> {
		let mut machine = Machine::new();
		machine.configure().with_err_semantic(false);
		let fixture = " A -> C
						A -> B";

		let assert = |machine: &Machine| {
			assert_json_eq!(
				json!({
					"states": [{
							"name": "A",
							"type": "regular",
							"color": "red"
						}, {
							"name": "C",
							"type": "regular"
						}, {
							"name": "B",
							"type": "regular",
							"color": "red"
					}],
					"transitions": [{
							"from": "A",
							"to": "C"
						}, {
							"from": "A",
							"to": "B",
							"color": "red",
							"note": ["duplicate transient transition: A -> B,C"]
					}]
				}),
				json!(machine)
			)
		};

		for expression in fixture.split('\n') {
			machine.insert_parse(expression)?;
		}
		assert(&machine);

		machine.parse(fixture)?;
		Ok(assert(&machine))
	}

	#[test]
	/// Fix #23
	fn consecutive_insert_parse() -> Result<(), DynError> {
		let mut machine = Machine::new();
		machine.insert_parse("A -> B @ C")?;
		machine.insert_parse("A -> D @ Cs")?;

		Ok(assert_json_eq!(
			json!({
				"states": [{
						"name": "A",
						"type": "regular"
					}, {
						"name": "B",
						"type": "regular"
					}, {
						"name": "D",
						"type": "regular"
				}],
				"transitions": [{
						"event": "C",
						"from": "A",
						"label": "C",
						"to": "B"
					}, {
						"event": "Cs",
						"from": "A",
						"label": "Cs",
						"to": "D"
				}]
			}),
			json!(machine)
		))
	}
}
