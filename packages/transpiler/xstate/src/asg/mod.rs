#![allow(clippy::unit_arg)]
mod schema;

use scdlang_core as scdlang;
pub use schema::*;

use scdlang_core::{grammar::Rule, prelude::*, Scdlang};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{error, fmt};
use voca_rs::case::{camel_case, shouty_snake_case};

#[derive(Default, Serialize, Deserialize)]
pub struct Machine<'a> {
	#[serde(flatten)]
	schema: StateChart,

	#[serde(skip)]
	builder: Scdlang<'a>,
}

impl<'a> Parser<'a> for Machine<'a> {
	fn configure(&mut self) -> &mut Builder<'a> {
		&mut self.builder
	}

	fn parse(&mut self, source: &str) -> Result<(), DynError> {
		let ast = self.try_parse(source)?;
		Ok(self.schema.states = ast.schema.states)
	}

	fn insert_parse(&mut self, source: &str) -> Result<(), DynError> {
		let ast = self.try_parse(source)?;
		Ok(for (current_state, transition) in ast.schema.states {
			self.schema
				.states
				.entry(current_state)
				.and_modify(|t| t.on.extend(transition.on.clone()))
				.or_insert(transition);
		})
	}

	fn try_parse(&self, source: &str) -> Result<Self, DynError> {
		let parse_tree = self.builder.parse(source)?;
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

		Ok(Machine {
			schema,
			builder: self.builder.clone(),
		})
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
		let mut machine = Machine::default();
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
		let mut machine = Machine::default();
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
}
