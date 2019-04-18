use crate::*;
use pest::{error::ErrorVariant, Span};
use std::{error, fmt, iter};

pub type ParseError = pest::error::Error<grammar::Rule>;

#[derive(Debug)]
// WARNING: this enum doesn't support lifetime which will break Parser trait (lifetime refactoring hell)
pub enum Error {
	WrongRule(grammar::Rule),
	Parse(Box<ParseError>),
	EmptyDeclaration,
	MissingOperator,
	Deadlock,
}

impl Error {
	pub fn from_span(span: Span, message: String) -> ParseError {
		ParseError::new_from_span(ErrorVariant::CustomError { message }, span)
	}
}

impl<'t> Scdlang<'t> {
	pub(crate) fn err_from_span(&self, span: Span, message: String) -> ParseError {
		use pest::error::InputLocation;
		let mut error = ParseError::new_from_span(ErrorVariant::CustomError { message }, span.clone());
		if let Some(offset) = self.line {
			//TODO: make PR on pest to add `fn with_line(self, offset: usize) -> Error<R>`
			if let InputLocation::Span((start, end)) = error.location {
				error = ParseError::new_from_span(
					error.variant,
					Span::new(
						&format!(
							"{offset}{src}",
							offset = iter::repeat('\n').take(offset).collect::<String>(),
							src = span.as_str()
						),
						start + offset,
						end + offset,
					)
					.unwrap(),
				);
			}
		}
		if let Some(path) = self.path {
			error = error.with_path(path);
		}
		error
	}
}

impl From<ParseError> for Error {
	fn from(err: ParseError) -> Self {
		Error::Parse(err.into())
	}
}

impl error::Error for Error {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		Some(self)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Error::Parse(err) => write!(f, "{}", err),
			_ => write!(f, "{:#?}", self), // TODO: make it fluent and verbose 😅
		}
	}
}
