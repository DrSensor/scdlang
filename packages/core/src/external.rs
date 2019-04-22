use crate::{cache, Scdlang};
use std::{error::Error, fmt};

#[rustfmt::skip]
/// This trait was mean to be used outside the core
/// For example, to build a transpiler
pub trait Parser<'t>: fmt::Display {
	fn parse(&mut self, source: &str) -> Result<(), BoxError>;
	fn insert_parse(&mut self, source: &str) -> Result<(), BoxError>;

	fn try_parse(source: &str, options: Scdlang<'t>) -> Result<Self, BoxError> where Self: Sized;
	fn configure(&mut self) -> &mut dyn Builder<'t>;

	fn flush_cache<'e>(&'t self) -> Result<(), DynError<'e>> {
		Ok(cache::clear()?.shrink()?)
	}

	fn clean_cache<'e>(&'t self) -> Result<(), DynError<'e>> {
		cache::clear()?;
		Ok(())
	}
}

pub trait Builder<'t> {
	fn with_err_path(&mut self, path: &'t str) -> &mut dyn Builder<'t>;
	fn with_err_line(&mut self, line: usize) -> &mut dyn Builder<'t>;
	fn with_err_semantic(&mut self, default: bool) -> &mut dyn Builder<'t>;

	/// Automatically clear cache when out of scope
	/// The cahce is used for analyzing semantics error
	/// This can be handy when parsing in streaming fashion
	/// default `true`
	fn auto_clear_cache(&mut self, default: bool) -> &mut dyn Builder<'t>;
}

type DynError<'t> = Box<dyn Error + 't>;
type BoxError = Box<dyn Error>;
