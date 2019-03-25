use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Transition {
	pub on: HashMap<String, Value>,
	// 🤔☝️ how about convert it to struct of #[derive(Hash, Eq, PartialEq, Debug)]
	// see https://doc.rust-lang.org/nightly/std/collections/struct.HashMap.html#examples
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Machine {
	pub states: HashMap<String, Transition>,
}

impl Machine {
	pub fn new() -> Self {
		Machine {
			states: HashMap::default(),
		}
	}
}
