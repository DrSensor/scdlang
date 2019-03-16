/* `inner(rule(path::to), with(path::to),*)` not working
TODO: please join this conversation https://github.com/pest-parser/ast/issues/15
WARNING: order in the struct fields is matter❗
*/

mod utils;
use utils::*;

use from_pest::*;
use pest_ast::FromPest;
use scdlang_core::{self as scdlang, Rule};
use serde::Serialize;
use serde_json::Value;
use std::{collections::HashMap, error::Error, fmt};

#[derive(Debug, FromPest, Serialize)]
#[pest_ast(rule(Rule::transition))]
pub struct Transition {
	#[pest_ast(inner(with(span::into_string)))]
	#[serde(skip)]
	transition: String,

	#[pest_ast(inner(with(span::into_json)))]
	pub on: HashMap<String, Value>,
}

#[derive(Debug, Default, FromPest, Serialize)]
#[pest_ast(rule(Rule::DescriptionFile))]
pub struct Machine {
	#[pest_ast(inner(with(span::into_pair)))]
	pub states: HashMap<String, Transition>,

	#[serde(skip)]
	eoi: EOI,
}

#[derive(Debug, Default, FromPest)]
#[pest_ast(rule(Rule::EOI))]
struct EOI;

type MachineError = Box<dyn Error>;
impl Machine {
	pub fn new() -> Self {
		Machine {
			states: HashMap::default(),
			eoi: EOI,
		}
	}

	pub fn from(source: &str) -> Result<Machine, MachineError> {
		let mut parse_tree = scdlang::parse(&source)?;
		if pairs::is_expression(&parse_tree) {
			let line = &format!(r#"expression("{line}")"#, line = parse_tree.as_str());
			Ok(Machine::from_pest(&mut parse_tree).expect(line))
		} else {
			Ok(Machine::default())
		}
	}

	pub fn parse(&mut self, source: &str) -> Result<(), MachineError> {
		let ast = Self::from(source)?;
		self.states = ast.states;
		self.eoi = ast.eoi;
		Ok(())
	}

	pub fn insert_parse(&mut self, source: &str) -> Result<(), MachineError> {
		let ast = Self::from(source)?;
		self.states.extend(ast.states);
		Ok(())
	}

	pub fn to_string(&self) -> String {
		serde_json::to_string(&self).unwrap()
	}
}

impl fmt::Display for Machine {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match serde_json::to_string_pretty(&self) {
			Ok(json) => write!(f, "{}", json),
			Err(_) => Err(fmt::Error),
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn parse_to_json_string() {
		let machine = Machine::from("A->B").unwrap();
		let json = machine.to_string();
		assert_eq!(r#"{"states":{"A":{"on":{"":"B"}}}}"#, json);
	}

	#[test]
	fn pretty_print_json() {
		let machine = Machine::from("A->B").unwrap();
		let output = format!("{}", machine).replace(" ", "");
		let expected = r#"{
			"states": {
				"A": {
					"on": {
						"": "B"
					}
				}
			}
		}"#
		.replace("\t", "")
		.replace(" ", "");
		assert_eq!(output, expected);
	}
}
