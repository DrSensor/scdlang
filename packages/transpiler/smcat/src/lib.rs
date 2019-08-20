#![allow(clippy::unit_arg)]
mod schema;
mod utils;

use scdlang::{
	prelude::*,
	semantics::{Found, Kind},
	Scdlang,
};
use schema::*;
use serde::Serialize;
use std::{error, fmt, mem::ManuallyDrop};
use utils::*;
pub mod prelude {
	pub use scdlang::external::*;
}

pub mod option {
	pub const MODE: &str = "mode"; // -> normal,blackbox-state
	pub mod mode {
		pub mod blackbox {
			pub const STATE: &str = "blackbox-state";
		}
	}
}

#[derive(Default, Serialize)]
/** Transpiler Scdlang → State Machine Cat (JSON).

# Examples
```no_run
# use std::error::Error;
use scdlang_smcat::{prelude::*, Machine};

let mut parser = Machine::new();

parser.configure().with_err_path("test.scl");
parser.parse("A -> B")?;

println!("{}", parser.to_string());
# Ok::<(), Box<dyn Error>>(())
``` */
pub struct Machine<'a> {
	#[serde(skip)]
	builder: Scdlang<'a>, // TODO: refactor this as specialized builder

	#[serde(flatten)]
	schema: Coordinate, // TODO: replace with 👇 when https://github.com/serde-rs/serde/issues/1507 resolved
	                    // schema: mem::ManuallyDrop<StateChart>,
}

impl<'a> Parser<'a> for Machine<'a> {
	fn configure(&mut self) -> &mut dyn Builder<'a> {
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
		let get = |key| builder.get(key);

		for kind in builder.iter_from(source)? {
			match kind {
				Kind::Expression(expr) => {
					let (color, note) = match expr.semantic_check()? {
						Found::Error(ref message) if !builder.semantic_error => {
							(Some("red".to_string()), Some(message.split_to_vec('\n')))
						}
						_ => (None, None),
					};
					let (event, cond, action) = (
						expr.event().map(|e| e.into()),
						expr.guard().map(|e| e.into()),
						expr.action().map(|e| e.into()),
					);

					schema.states.merge(&{
						let (mut current, mut next) = (
							expr.current_state().into_type(Regular),
							expr.next_state().map(|s| s.into_type(Regular)),
						);
						if let/* mark error */Some(color) = &color {
							current.with_color(color);
							next = next.map(|mut s| s.with_color(color).clone())
						}
						use option::mode::*;
						match next {
							Some(next) => vec![current, next], // external transition
							None if get(option::MODE) == Some(blackbox::STATE) => vec![/*WARNING:wasted*/], // ignore anything inside state
							None => {
								if let (Some(event), Some(action)) = (event.as_ref(), action.as_ref()) {
									current.actions = Some(vec![ActionType {
										r#type: ActionTypeType::Activity,
										body: match cond.as_ref() {
											None => format!("{} / {}", event, action),
											Some(cond) => format!("{} [{}] / {}", event, cond, action),
										},
									}]);
								}
								vec![current] // internal transition
							}
						}
					});

					if let/* external transition */Some(next_state) = expr.next_state() {
						#[rustfmt::skip]
						let transition = Transition {
							from: expr.current_state().into(),
							to: next_state.into(),
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
							"note": ["duplicate transient-transition: A -> B,C"]
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
