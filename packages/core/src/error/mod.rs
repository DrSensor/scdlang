mod format;
mod util;

use crate::grammar::*;
use pest;

pub type PestError = pest::error::Error<Rule>;

#[derive(Debug)]
// WARNING: this enum doesn't support lifetime which will break Parser trait (lifetime refactoring hell)
pub enum Error {
	WrongRule(Rule),
	Parse(Box<PestError>),
	EmptyDeclaration,
	MissingOperator,
	Deadlock,
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
						| "A -> B invalid name" => assert_eq!("expected @", message),
						_ => unreachable!("{}", expression),
					},
					ParsingError { .. } => unimplemented!("TODO: implement this if there is any case for that"),
				})
			},
		)
	}
}
