use crate::{utils::naming::Name, Error};
use std::{any::Any, fmt::Debug};

#[derive(Debug)]
/// An enum returned by [Scdlang.iter_from](../struct.Scdlang.html#method.iter_from)
/// to access semantics type/kind
pub enum Kind<'g> {
	/// See trait [`Expression`](trait.Expression.html)
	Expression(Box<dyn Expression + 'g>),
	/// See trait [`Declaration`](trait.Declaration.html)
	Declaration(Box<dyn Declaration + 'g>),
	/// See trait [`Statement`](trait.Statement.html)
	Statement(Box<dyn Statement + 'g>),
}

/** An Enum returned by [Expression.semantic_check](trait.Expression.html#tymethod.semantic_check)
to detect incorrect semantics */
pub enum Found {
	/// Threw when detect errornous expressions or statements
	Error(String),
	/// Threw when detect expressions or statements that can cause ambiguity
	Warning(String),
	/// All is well 😉
	None,
}

#[rustfmt::skip]
/** Everything that can change state

Example:
```scl
A -> B
``` */
pub trait Expression: Debug {
	fn current_state(&self) -> Name;
	fn next_state(&self) -> Name;
	fn event(&self) -> Option<Name>;
	fn guard(&self) -> Option<Name>;
	fn action(&self) -> Option<Name>;
	fn semantic_check(&self) -> Result<Found, Error>;
}

/** [UNIMPLEMENTED] Mostly everything that use curly braces.

Example:
```scl
state A {}
```
🤔 I wonder if curly braces that can expand into multiple transition is included */
pub trait Declaration: Debug {
	/// e.g: `@entry |> doSomething`
	fn statements(&self) -> Option<Vec<&dyn Statement>>;

	/// e.g: `history state`
	fn properties(&self) -> Option<&dyn Any>;

	fn expressions(&self) -> Option<Vec<&dyn Expression>>;
}

/** [UNIMPLEMENTED] Everything that don't change state (no transition)

Example:
```scl
A |> doSomething
```
or just a shorthand for writing a declaration in one line */
pub trait Statement: Debug {
	fn state(&self) -> Option<Name>;
	fn action(&self) -> Option<&Any /*👈TBD*/>;
	fn event(&self) -> Option<Name>;
}
