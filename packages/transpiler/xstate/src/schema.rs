use serde::Serialize;
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize)]
pub struct TransitionObject {
	pub target: Option<String>,
	pub actions: Option<String>, // TODO: should be Option<Vec<String>> in the future
	pub cond: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
#[serde(untagged)]
pub enum Transition {
	Target(Option<String>),
	Object(TransitionObject),
	ListObject(Vec<TransitionObject>),
}

type Event = String;

// #[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct State {
	// #[serde(flatten)]
	// pub child: Option<StateChart>,
	pub on: HashMap<Event, Transition>,
	// 🤔☝️ how about convert it to struct of #[derive(Hash, Eq, PartialEq, Debug)]
	// see https://doc.rust-lang.org/nightly/std/collections/struct.HashMap.html#examples
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct StateChart {
	pub states: HashMap<String, State>,
}
