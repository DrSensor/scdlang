#![allow(clippy::unit_arg)]
mod schema;

use scdlang_core as scdlang;
pub use schema::*;

use crate::Parser;
use scdlang_core::{prelude::*, Rule, Scdlang};
use serde_json::json;
use std::{error, fmt};

impl Parser for Machine {
	fn parse(&mut self, source: &str) -> Result<(), DynError> {
		let ast = Self::try_parse(source)?;
		Ok(self.states = ast.states)
	}

	fn insert_parse(&mut self, source: &str) -> Result<(), DynError> {
		let ast = Self::try_parse(source)?;
		Ok(for (current_state, transition) in ast.states {
			self.states
				.entry(current_state)
				.and_modify(|t| t.on.extend(transition.on.clone()))
				.or_insert(transition);
		})
	}

	fn try_parse(source: &str) -> Result<Self, DynError> {
		// TODO: remove .unwrap() after scdlang::Error implement std::error:Error
		let parse_tree = Scdlang::parse_from(source)?;
		let mut machine = Machine::new();

		for pair in parse_tree {
			if let Rule::expression = pair.as_rule() {
				let transition: scdlang::Transition = pair.try_into()?;

				let event_name = transition.at.map(|e| e.name).unwrap_or("");
				let current_state = transition.from.name.to_string();
				let next_state = transition.to.name;

				machine
					.states
					.entry(current_state)
					.and_modify(|t| {
						t.on.entry(event_name.to_string()).or_insert_with(|| json!(next_state));
					})
					.or_insert(Transition {
						// TODO: waiting for https://github.com/rust-lang/rfcs/issues/542
						on: [(event_name.to_string(), json!(next_state))].iter().cloned().collect(),
					});
			}
		}

		Ok(machine)
	}
}

impl fmt::Display for Machine {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", serde_json::to_string_pretty(&self).map_err(|_| fmt::Error)?)
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
		machine.parse("A -> B")?;

		Ok(assert_json_eq!(
			json!({
				"states": {
					"A": {
						"on": {
							"": "B"
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
			"A -> B @ C
			A -> D @ E",
		)?;

		Ok(assert_json_eq!(
			json!({
				"states": {
					"A": {
						"on": {
							"C":"B",
							"E":"D"
						}
					}
				}
			}),
			json!(machine)
		))
	}
}
