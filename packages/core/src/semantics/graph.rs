//! Module that contain semantics graph of Scdlang which modeled as [DAG](https://en.wikipedia.org/wiki/Directed_acyclic_graph)
#![allow(dead_code)]
use std::fmt::{self, Display};

#[derive(Debug, Clone)]
/// SCXML equivalent:
/// ```scxml
/// <state id="from.name">
///     <transition target="to.name"/>
/// </state>
/// ```
pub struct Transition<'t> {
	pub from: State<'t>,
	pub to: Option<State<'t>>,
	pub at: Option<Event<'t>>,
	pub run: Option<Action<'t>>,
	pub kind: TransitionType<'t>, // 🤔 maybe I should hide it then implement kind() method
}

#[derive(Debug, Clone)]
/// ```scl
/// state A {
/// 	B -> C // Internal(A)
/// }
/// A -> D // External
/// A <-> F // Duplex
/// A ->> L // Loop
/// ```
#[allow(clippy::large_enum_variant)]
pub enum TransitionType<'t> {
	Inside {
		state: &'t State<'t>,
		kind: &'t TransitionType<'t>,
	},
	Internal,
	// FIXME: encapsulate 👇 as External variant
	Normal,
	Toggle,
	Loop {
		transient: bool,
	},
}

#[derive(Debug, Clone)]
/// SCXML equivalent:
/// ```scxml
/// <state id="name"/>
/// ```
pub struct State<'s> {
	pub name: &'s str,
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

#[derive(Debug, Default, Clone)]
/// SCXML equivalent:
/// ```scxml
/// <transition event="name"/>
/// ```
pub struct Event<'at> {
	// pub kind: &'s EventType, // 🤔 probably should not be a field but more like kind() method because the type can be deduce on the available field
	pub name: Option<&'at str>, // should be None when it only have a Guard or "it just an Internal Event"
	pub guard: Option<&'at str>,
}

impl Into<String> for &Event<'_> {
	fn into(self) -> String {
		self.name.unwrap_or(self.guard.unwrap_or("")).to_string()
	}
}

impl Display for Event<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{event}{guard}",
			event = self.name.unwrap_or(""),
			guard = match self.guard {
				Some(guard_name) => ["[", guard_name, "]"].concat(),
				None => String::new(),
			}
		)
	}
}

#[derive(Debug, Default, Clone)]
pub struct Action<'play> {
	pub name: &'play str,
}

impl Into<String> for &Action<'_> {
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
