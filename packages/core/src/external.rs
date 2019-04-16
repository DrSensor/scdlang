use crate::{cache, Scdlang};
use std::fmt;

#[rustfmt::skip]
/// This trait was mean to be used outside the core
/// For example, to build a transpiler
pub trait Parser<'t>: fmt::Display {
	fn parse(&mut self, source: &str) -> Result<(), DynError>;
	fn insert_parse(&mut self, source: &str) -> Result<(), DynError>;

	fn try_parse(source: &str, options: Scdlang<'t>) -> Result<Self, DynError> where Self: Sized;
	fn configure(&mut self) -> &mut dyn Builder<'t>;

	fn clear_cache(&self) -> Result<(), DynError> {
		Ok(cache::drop()?)
	}
}

pub trait Builder<'t> {
	fn with_err_path(&mut self, path: &'t str);
	fn with_err_line(&mut self, line: usize);

	/// Automatically clear cache when out of scope
	/// The cahce is used for analyzing semantics error
	/// This can be handy when parsing in streaming fashion
	/// default `true`
	fn auto_clear_cache(&mut self, default: bool);
}

type DynError = Box<dyn std::error::Error>;
