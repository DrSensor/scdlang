#![allow(clippy::unit_arg)]
extern crate strum;
mod parser;
mod schema;
mod typescript;

use scdlang::{prelude::*, Scdlang};
use schema::StateChart;
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
		Output,
		ExportName,
	}

	#[derive(AsRefStr, EnumString)]
	#[strum(serialize_all = "lowercase")]
	pub enum Output {
		JSON,
		TypeScript,
		JavaScript,
	}
}

#[derive(Default, Serialize)]
/** Transpiler Scdlang → XState.

# Examples
```no_run
# use std::error::Error;
use scdlang_xstate::{prelude::*, Machine};

let mut parser = Machine::new();

parser.configure().with_err_path("test.scl");
parser.parse("A -> B")?;

println!("{}", parser.to_string());
# Ok::<(), Box<dyn Error>>(())
``` */
pub struct Machine<'a> {
	#[serde(skip)]
	builder: Scdlang<'a>, // TODO:refactor this as specialized builder

	#[serde(flatten)]
	schema: StateChart, // TODO: replace with 👇 when https://github.com/serde-rs/serde/issues/1507 resolved
	                    // schema: mem::ManuallyDrop<StateChart>,
}

impl Machine<'_> {
	/* Create new StateMachine in default mode

	##### custom config
	* "output": "json" | "typescript" (default: "json") */
	pub fn new() -> Self {
		use option::*;
		let (mut builder, schema) = (Scdlang::new(), StateChart::default());
		builder.auto_clear_cache(false);
		builder.set(&Config::Output, &Output::JSON);
		Self { builder, schema }
	}
}

impl Drop for Machine<'_> {
	fn drop(&mut self) {
		self.flush_cache().expect("xstate: Deadlock");
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
				"states": {
					"alphaGo": {
						"on": {
							"": "betaRust"
						}
					}
				}
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
				"states": {
					"a": {
						"on": {
							"CARLIE_CAPLIN": "b",
							"ENHANCED_ERLANG": "d"
						}
					},
					"b": {
						"on": {
							"CARLIE_CAPLIN": "a"
						}
					}
				}
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
				"states": {
					"a": {
						"on": {
							"": "b"
						}
					}
				}
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
				"states": {
					"a": {
						"on": {
							"": "c"
						}
					}
				}
			}),
			json!(machine)
		)
	}
}
