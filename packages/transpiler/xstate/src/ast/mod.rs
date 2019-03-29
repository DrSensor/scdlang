#![allow(clippy::unit_arg)]
mod schema;
mod utils;

use scdlang_core as scdlang;
pub use schema::*;

use crate::Parser;
use from_pest::FromPest;
use std::{error, fmt};
use utils::pairs;

impl Parser for Machine {
	fn parse(&mut self, source: &str) -> Result<(), DynError> {
		let ast = Self::try_parse(source)?;
		Ok(self.states = ast.states)
	}

	fn insert_parse(&mut self, source: &str) -> Result<(), DynError> {
		let ast = Self::try_parse(source)?;
		Ok(self.states.extend(ast.states))
	}

	fn try_parse(source: &str) -> Result<Self, DynError> {
		let mut parse_tree = scdlang::parse(&source)?;

		if pairs::is_expression(&parse_tree) {
			let line = &format!(r#"expression("{line}")"#, line = parse_tree.as_str());
			Ok(Machine::from_pest(&mut parse_tree).expect(line))
		} else {
			Ok(Machine::default())
		}
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
	use serde_json::json;

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
	#[ignore] // 🤔 seems it's difficult to support this, maybe I should drop AST parser 😕
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
