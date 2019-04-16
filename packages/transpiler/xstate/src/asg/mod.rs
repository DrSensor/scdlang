#![allow(clippy::unit_arg)]
mod schema;

use scdlang_core as scdlang;
pub use schema::*;

use scdlang_core::{grammar::Rule, prelude::*, Scdlang};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{error, fmt, mem::ManuallyDrop};
use voca_rs::case::{camel_case, shouty_snake_case};

#[derive(Default, Serialize, Deserialize)]
pub struct Machine<'a> {
	#[serde(skip)]
	builder: Scdlang<'a>,

	#[serde(flatten)]
	schema: StateChart, // TODO: replace with 👇 when https://github.com/serde-rs/serde/issues/1507 resolved
	                    // schema: mem::ManuallyDrop<StateChart>,
}

impl<'a> Parser<'a> for Machine<'a> {
	fn configure(&mut self) -> &mut Builder<'a> {
		&mut self.builder
	}

	fn parse(&mut self, source: &str) -> Result<(), DynError> {
		self.clear_cache()?;
		let ast = ManuallyDrop::new(Self::try_parse(source, self.builder.to_owned())?);
		Ok(self.schema = ast.schema.to_owned()) // FIXME: expensive clone
	}

	fn insert_parse(&mut self, source: &str) -> Result<(), DynError> {
		let ast = ManuallyDrop::new(Self::try_parse(source, self.builder.to_owned())?);
		Ok(
			for (current_state, transition) in ast.schema.states.to_owned(/*FIXME: expensive clone*/) {
				self.schema
					.states
					.entry(current_state)
					.and_modify(|t| t.on.extend(transition.on.clone()))
					.or_insert(transition);
			},
		)
	}

	fn try_parse(source: &str, builder: Scdlang<'a>) -> Result<Self, DynError> {
		let parse_tree = builder.parse(source)?;
		let mut schema = StateChart::default();

		for pair in parse_tree {
			if let Rule::expression = pair.as_rule() {
				let transition: scdlang::Transition = pair.try_into()?;

				let event_name = shouty_snake_case(transition.at.map(|e| e.name).unwrap_or(""));
				let current_state = camel_case(transition.from.name);
				let next_state = camel_case(transition.to.name);

				schema
					.states
					.entry(current_state)
					.and_modify(|t| {
						t.on.entry(event_name.to_string()).or_insert_with(|| json!(next_state));
					})
					.or_insert(Transition {
						// TODO: waiting for map macros https://github.com/rust-lang/rfcs/issues/542
						on: [(event_name.to_string(), json!(next_state))].iter().cloned().collect(),
					});
			}
		}

		Ok(Machine { schema, builder })
	}
}

impl<'a> Machine<'a> {
	pub fn new() -> Self {
		let mut machine = Self::default();
		machine.builder.auto_clear_cache(false);
		machine
	}
}

impl Drop for Machine<'_> {
	fn drop(&mut self) {
		self.clear_cache().expect("xstate: Deadlock");
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
