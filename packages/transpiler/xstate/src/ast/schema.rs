/* `inner(rule(path::to), with(path::to),*)` not working
TODO: please join this conversation https://github.com/pest-parser/ast/issues/15
WARNING: order in the struct fields is matter❗
*/

use super::utils::*;
use pest_ast::FromPest;
use scdlang_core::Rule;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, FromPest, Serialize, Deserialize)]
#[pest_ast(rule(Rule::transition))]
pub struct Transition {
	#[pest_ast(inner(with(span::into_string)))]
	#[serde(skip)]
	transition: String, // order 👇 matter

	#[pest_ast(inner(with(span::into_json)))]
	pub on: HashMap<String, Value>,
	// 🤔☝️ how about convert it to struct of #[derive(Hash, Eq, PartialEq, Debug)]
	// see https://doc.rust-lang.org/nightly/std/collections/struct.HashMap.html#examples
}

#[derive(Debug, Default, FromPest, Serialize, Deserialize)]
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

impl Machine {
	pub fn new() -> Self {
		Machine {
			states: HashMap::default(),
			eoi: EOI,
		}
	}
}
