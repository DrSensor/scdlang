//! Generated from JSON schema of [smcat-ast]
//! using https://quicktype.io with some fixes
//! [smcat-ast]: https://github.com/sverweij/state-machine-cat/blob/develop/src/parse/smcat-ast.schema.json
#![allow(dead_code)]
use serde::Serialize;
use serde_with::skip_serializing_none;

// TODO: replace Vec with HashSet

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize)]
pub struct State {
	#[serde(rename = "actions")]
	pub actions: Option<Vec<ActionType>>,

	#[serde(rename = "active")]
	pub active: Option<bool>,

	#[serde(rename = "color")]
	pub color: Option<String>,

	#[serde(rename = "isComposite")]
	pub is_composite: Option<bool>,

	#[serde(rename = "label")]
	pub label: Option<String>,

	#[serde(rename = "name")]
	pub name: String,

	#[serde(rename = "note")]
	pub note: Option<Vec<String>>,

	#[serde(rename = "statemachine")]
	pub statemachine: Option<Coordinate>,

	#[serde(rename = "type")]
	pub state_type: StateType,

	#[serde(rename = "typeExplicitlySet")]
	pub type_explicitly_set: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize)]
pub struct Coordinate {
	#[serde(rename = "states")]
	pub states: Vec<State>,

	#[serde(rename = "transitions")]
	pub transitions: Option<Vec<Transition>>,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Serialize)]
pub struct ActionType {
	#[serde(rename = "body")]
	pub body: String,

	#[serde(rename = "type")]
	pub r#type: ActionTypeType,
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize)]
pub struct Transition {
	#[serde(rename = "action")]
	pub action: Option<String>,

	#[serde(rename = "color")]
	pub color: Option<String>,

	#[serde(rename = "cond")]
	pub cond: Option<String>,

	#[serde(rename = "event")]
	pub event: Option<String>,

	#[serde(rename = "from")]
	pub from: String,

	#[serde(rename = "label")]
	pub label: Option<String>,

	#[serde(rename = "note")]
	pub note: Option<Vec<String>>,

	#[serde(rename = "to")]
	pub to: String,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Serialize)]
pub enum ActionTypeType {
	#[serde(rename = "activity")]
	Activity,

	#[serde(rename = "entry")]
	Entry,

	#[serde(rename = "exit")]
	Exit,
}

impl Default for StateType {
	fn default() -> Self {
		StateType::Regular
	}
}

#[derive(Debug, Clone, Serialize)]
pub enum StateType {
	#[serde(rename = "choice")]
	Choice,

	#[serde(rename = "deephistory")]
	Deephistory,

	#[serde(rename = "final")]
	Final,

	#[serde(rename = "fork")]
	Fork,

	#[serde(rename = "forkjoin")]
	Forkjoin,

	#[serde(rename = "history")]
	History,

	#[serde(rename = "initial")]
	Initial,

	#[serde(rename = "join")]
	Join,

	#[serde(rename = "junction")]
	Junction,

	#[serde(rename = "parallel")]
	Parallel,

	#[serde(rename = "regular")]
	Regular,

	#[serde(rename = "terminate")]
	Terminate,
}
