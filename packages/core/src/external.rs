use std::fmt;

#[rustfmt::skip]
/// This trait was mean to be used outside the core
/// For example, to build a transpiler
pub trait Parser<'t>: fmt::Display {
	fn parse(&mut self, source: &str) -> Result<(), DynError>;
	fn insert_parse(&mut self, source: &str) -> Result<(), DynError>;

	fn try_parse(&self, source: &str) -> Result<Self, DynError> where Self: Sized;
	fn configure(&mut self) -> &mut dyn Builder<'t>;
}

pub trait Builder<'t> {
	fn with_err_path(&mut self, path: &'t str);
	fn with_err_line(&mut self, line: u16);
}

type DynError = Box<dyn std::error::Error>;
