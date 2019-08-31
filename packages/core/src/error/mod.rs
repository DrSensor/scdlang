mod format;
mod util;

use crate::grammar::*;
use pest;

pub type PestError = pest::error::Error<Rule>;

#[derive(Debug)]
/// Parse-related error type.
// WARNING: 👇 adding lifetime annotation can cause lifetime refactoring hell 💢 (it will break Parser trait)
pub enum Error {
	/// Happen when there is syntax or semantics error
	Parse(Box<PestError>),

	/// Used internally on severity: WARNING
	WithId { id: u64, error: Box<PestError> },

	/// Can happen when accessing caches unsafely
	Deadlock,
}

pub(crate) struct ErrorMap {
	pub id: u64,
	pub message: String,
}

#[cfg(test)]
mod variant {
	#![allow(clippy::unit_arg)]
	use crate::{error::Error, test};
	use pest::error::ErrorVariant::*;
	type ParseResult = Result<(), Error>;

	#[test]
	fn syntax_error() -> ParseResult {
		test::parse::error(
			"A -> B <-
			A -> B,
			A -> B->
			A -> B PascalCase
			A -> B 'quoted'
			A -> B invalid name
		",
			|expression, error| {
				Ok(match error {
					CustomError { message } => match expression {
						"A -> B <-"
						| "A -> B,"
						| "A -> B->"
						| "A -> B PascalCase"
						| "A -> B 'quoted'"
						| "A -> B invalid name" => assert_eq!("expected @ or |>", message),
						_ => unreachable!("{}", expression),
					},
					ParsingError { .. } => unimplemented!("TODO: implement this if there is any case for that"),
				})
			},
		)
	}
}
