#![allow(clippy::unit_arg)]
extern crate strum;
mod parser;
mod schema;
mod utils;

use scdlang::{prelude::*, Scdlang};
use schema::Coordinate;
use serde::Serialize;

pub mod prelude {
	pub use scdlang::external::*;
}

pub use option::Config;
pub mod option {
	use strum_macros::*;

	#[derive(AsRefStr)]
	#[strum(serialize_all = "lowercase")]
	pub enum Config {
		Mode,
	}

	#[derive(AsRefStr, EnumString)]
	#[strum(serialize_all = "kebab-case")]
	pub enum Mode {
		BlackboxState,
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

impl Machine<'_> {
	/// Create new StateMachine.
	/// Use this over `Machine::default()`❗
	pub fn new() -> Self {
		let (mut builder, schema) = (Scdlang::new(), Coordinate::default());
		builder.auto_clear_cache(false);
		Self { builder, schema }
	}
}

impl Drop for Machine<'_> {
	fn drop(&mut self) {
		self.flush_cache().expect("smcat: Deadlock");
	}
}

type DynError = Box<dyn std::error::Error>;

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
	#[ignore]
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
							// FIXME: 👇 should be tested using regex
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
