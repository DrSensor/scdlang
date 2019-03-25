#[derive(Debug)]
/// SCXML equivalent:
/// ```scxml
/// <state id="from.name">
///     <transition target="to.name"/>
/// </state>
/// ```
pub struct Transition<'t> {
	pub from: State<'t>,
	pub to: State<'t>,
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
	pub name: &'s str,
	pub kind: &'s StateType, // 🤔 should I hide it then implement kind() method?
}

#[derive(Debug)]
/// See https://statecharts.github.io/glossary/state.html
pub enum StateType {
	Atomic,
}
