use serde::Serialize;
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Transition {
	Target(Option<String>),
	Object {
		target: Option<String>,
		actions: Option<String>, // TODO: should be Option<Vec<String>> in the future
		cond: Option<String>,
	},
}

type Event = String;
#[derive(Debug, Clone, Serialize)]
pub struct State {
	pub on: HashMap<Event, Transition>,
	// 🤔☝️ how about convert it to struct of #[derive(Hash, Eq, PartialEq, Debug)]
	// see https://doc.rust-lang.org/nightly/std/collections/struct.HashMap.html#examples
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct StateChart {
	pub states: HashMap<String, State>,
}
