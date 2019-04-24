use crate::*;
use pest::{
	error::{ErrorVariant::CustomError, InputLocation},
	Position, Span,
};
use std::{error, fmt, iter};

pub type PestError = pest::error::Error<grammar::Rule>;

#[derive(Debug)]
// WARNING: this enum doesn't support lifetime which will break Parser trait (lifetime refactoring hell)
pub enum Error {
	WrongRule(grammar::Rule),
	Parse(Box<PestError>),
	EmptyDeclaration,
	MissingOperator,
	Deadlock,
}

impl<'t> Scdlang<'t> {
	/// Used to beutifully format semantics error from span
	pub(crate) fn err_from_span(&self, span: Span, message: String) -> PestError {
		let source = span.as_str();
		let mut error = PestError::new_from_span(CustomError { message }, span);
		if let Some(offset) = self.line {
			error = error.with_offset(offset, source);
		}
		if let Some(path) = self.path {
			error = error.with_path(path);
		}
		error
	}
}

impl Offset for PestError {
	fn with_offset(self, offset: usize, source: &str) -> Self {
		match self.location {
			InputLocation::Span((start, end)) => PestError::new_from_span(
				self.variant,
				Span::new(
					&format!(
						"{offset}{src}",
						offset = iter::repeat('\n').take(offset).collect::<String>(),
						src = source
					),
					start + offset,
					end + offset,
				)
				.expect("Index (offset) must NOT out of bound"),
			),
			InputLocation::Pos(line) => PestError::new_from_pos(
				self.variant,
				Position::new(
					&format!(
						"{offset}{src}",
						offset = iter::repeat('\n').take(offset).collect::<String>(),
						src = source
					),
					line + offset,
				)
				.expect("Index (offset) must NOT out of bound"),
			),
		}
	}
}

impl From<PestError> for Error {
	fn from(err: PestError) -> Self {
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

//TODO: make PR on pest for defining offset so that param `source` can be omitted
pub(crate) trait Offset: error::Error {
	fn with_offset(self, offset: usize, source: &str) -> Self;
}
