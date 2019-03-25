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
		Ok(self.states.extend(ast.states))
	}

	fn try_parse(source: &str) -> Result<Self, DynError> {
		// TODO: remove .unwrap() after scdlang::Error implement std::error:Error
		let parse_tree = Scdlang::parse_from(source)?;
		let mut machine = Machine::new();

		for pair in parse_tree {
			if let Rule::expression = pair.as_rule() {
				let transition: scdlang::Transition = pair.try_into()?;
				machine.states.insert(
					transition.from.name.to_string(),
					Transition {
						// TODO: waiting for https://github.com/rust-lang/rfcs/issues/542
						on: [("".to_string(), json!(transition.to.name))].iter().cloned().collect(),
					},
				);
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

	#[test]
	fn parse_to_json_string() -> Result<(), DynError> {
		let mut machine = Machine::new();
		machine.parse("A->B")?;

		let mut json = machine.to_string();
		json.retain(|c| c != ' ' && c != '\t' && c != '\n');

		Ok(assert_eq!(r#"{"states":{"A":{"on":{"":"B"}}}}"#, json))
	}
}
