#![allow(clippy::unit_arg)]
mod schema;
mod utils;

use scdlang_core::{self as scdlang, prelude::*, Scdlang};
pub use schema::*;

use from_pest::FromPest;
use serde::{Deserialize, Serialize};
use std::{error, fmt};
use utils::pairs;

#[derive(Default, Serialize, Deserialize)]
pub struct Machine<'a> {
	#[serde(flatten)]
	schema: StateChart,

	#[serde(skip)]
	builder: Scdlang<'a>,
}

/** Finally, found the downside of AST as a direct structure 😏
 * Because it doesn't have a process of breaking down the semantics into specific structure,
 * it's too much hasle for adding simple process like converting state name from PascalCase into camelCase
 */
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
		Ok(self.schema.states.extend(ast.schema.states))
	}

	fn try_parse(&self, source: &str) -> Result<Self, DynError> {
		let mut parse_tree = scdlang::parse(&source)?;

		let schema = if pairs::is_expression(&parse_tree) {
			let line = &format!(r#"expression("{line}")"#, line = parse_tree.as_str());
			StateChart::from_pest(&mut parse_tree).expect(line)
		} else {
			StateChart::default()
		};

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
	use serde_json::json;

	#[test]
	fn transient_transition() -> Result<(), DynError> {
		let mut machine = Machine::default();
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
		let mut machine = Machine::default();
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
