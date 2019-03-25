use crate::*;
use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
	WrongRule(grammar::Rule),
	Parse(Box<pest::error::Error<grammar::Rule>>),
	EmptyDeclaration,
	MissingOperator,
}

impl error::Error for Error {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		Some(self)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:#?}", self) // TODO: make it fluent and verbose 😅
	}
}
