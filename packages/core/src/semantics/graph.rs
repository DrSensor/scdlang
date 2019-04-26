use crate::utils::naming::Name;

#[derive(Debug)]
/// SCXML equivalent:
/// ```scxml
/// <state id="from.name">
///     <transition target="to.name"/>
/// </state>
/// ```
pub(crate) struct Transition<'t> {
	pub from: State<'t>,
	pub to: State<'t>,
	pub at: Option<Event<'t>>,
	pub kind: TransitionType<'t>, // 🤔 maybe I should hide it then implement kind() method
}

#[derive(Debug)]
/// ```scl
/// state A {
/// 	B -> C // Internal(A)
/// }
/// A -> D // External
/// ```
pub enum TransitionType<'t> {
	Internal(&'t State<'t>),
	External, // 🤔 should I implement Default trait?
}

#[derive(Debug)]
/// SCXML equivalent:
/// ```scxml
/// <state id="name"/>
/// ```
pub struct State<'s> {
	pub name: Name<'s>,
	pub kind: &'s StateType, // 🤔 should I hide it then implement kind() method?
}

#[derive(Debug)]
/// See https://statecharts.github.io/glossary/state.html
pub enum StateType {
	Atomic,
}

impl Into<String> for &State<'_> {
	fn into(self) -> String {
		self.name.to_string()
	}
}

#[derive(Debug, Clone)]
/// SCXML equivalent:
/// ```scxml
/// <transition event="name"/>
/// ```
pub struct Event<'s> {
	// pub kind: &'s EventType, // 🤔 probably should not be a field but more like kind() method because the type can be deduce on the available field
	pub name: Name<'s>, // TODO: should be None when it only have a Guard or it just an Internal Event
}

impl Into<String> for &Event<'_> {
	fn into(self) -> String {
		self.name.to_string()
	}
}

// 👇 still not sure 🤔
// #[derive(Debug)]
// /// see https://statecharts.github.io/glossary/internal-event.html
// pub enum EventType {
// 	External,
// }
// ☝️ maybe I need it if kind() method is exposed 🤔
