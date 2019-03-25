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

	#[test]
	fn parse_to_json_string() -> Result<(), DynError> {
		let mut machine = Machine::new();
		machine.parse("A->B")?;

		let mut json = machine.to_string();
		json.retain(|c| c != ' ' && c != '\t' && c != '\n');

		Ok(assert_eq!(r#"{"states":{"A":{"on":{"":"B"}}}}"#, json))
	}
}
