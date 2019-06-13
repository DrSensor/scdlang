use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
	pub on: HashMap<String, Value>,
	// 🤔☝️ how about convert it to struct of #[derive(Hash, Eq, PartialEq, Debug)]
	// see https://doc.rust-lang.org/nightly/std/collections/struct.HashMap.html#examples
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StateChart {
	pub states: HashMap<String, Transition>,
}
