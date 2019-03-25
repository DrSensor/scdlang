// TODO: move this file to core/src when there is additional transpiler
use std::{error, fmt};

#[rustfmt::skip]
pub trait Parser: fmt::Display {
	fn parse(&mut self, source: &str) -> Result<(), DynError>;
	fn insert_parse(&mut self, source: &str) -> Result<(), DynError>;

	fn try_parse(source: &str) -> Result<Self, DynError> where Self: Sized;
}

type DynError = Box<dyn error::Error>;
