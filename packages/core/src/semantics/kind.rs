use crate::utils::naming::Name;
use std::{any::Any, fmt::Debug};

#[derive(Debug)]
pub enum Kind<'g> {
	Expression(Box<dyn Expression + 'g>),
	Declaration(Box<dyn Declaration + 'g>),
	Statement(Box<dyn Statement + 'g>),
}

#[rustfmt::skip]
/// Everything that can change state
/// Example:
/// ```scl
/// A -> B
/// ```
pub trait Expression: Debug {
	fn current_state(&self) -> Name;
	fn next_state(&self) -> Name;
	fn event(&self) -> Option<Name>;
	fn action(&self) -> Option<&Any/*👈TBD*/> {
		unimplemented!("TBD")
	}
}

/// Mostly everything that use curly braces
/// Example:
/// ```scl
/// state A {}
/// ```
/// 🤔 I wonder if curly braces that can expand into multiple transition is included
pub trait Declaration: Debug {
	/// e.g: `@entry |> doSomething`
	fn statements(&self) -> Option<&dyn Statement>;

	/// e.g: `history state`
	fn properties(&self) -> Option<&dyn Any>;

	fn expressions(&self) -> Option<&dyn Expression>;
}

/// Everything that don't change state (no transition)
/// Example:
/// ```scl
/// A |> doSomething
/// ```
/// or just a shorthand for writing a declaration in one line
pub trait Statement: Debug {
	fn state(&self) -> Option<Name>;
	fn action(&self) -> Option<&Any /*👈TBD*/>;
	fn event(&self) -> Option<Name>;
}
