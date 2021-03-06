/*! Collection of Trait for extending core functionality.
Useful for creating transpiler, codegen, or even compiler.

# Examples

1. Implement trait [`Parser`] on struct with [`Scdlang`] field (or any type that implement [`Builder`]).
```no_run
#[derive(Default)]
pub struct Machine<'a> {
	builder: Scdlang<'a>, // or any type that implmenet trait `Builder`
	schema: std::any::Any,
}

impl Machine<'_> {
	pub fn new() -> Self {
		let mut builder = Scdlang::new();
		// insert any pre-configuration here. For example:
		// builder.auto_clear_cache(false)
		Self { builder, ..Default::default() }
	}
}

impl<'a> Parser<'a> for Machine<'a> {
	fn configure(&mut self) -> &mut Builder<'a> {
		&mut self.builder
	}

	fn parse(&mut self, source: &str) -> Result<(), DynError> {
		self.clean_cache()?;
		unimplemented!();
	}

	fn insert_parse(&mut self, source: &str) -> Result<(), Box<dyn error::Error>> {
		unimplemented!();
	}

	fn try_parse(source: &str, builder: Scdlang<'a>) -> Result<Self, Box<dyn error::Error>> {
		unimplemented!();
	}
}
```

2. Then it can be used like this:
```ignore
let parser: Box<dyn Parser> = Box::new(match args {
	Some(text) => module_a::Machine::try_parse(text)?,
	None => module_b::Machine::new(),
});

parser.configure()
	.auto_clear_cache(false)
	.with_err_path("my_fsm.scl");

parser.parse("Off -> On")?;
parser.insert_parse("Off <- On @ Shutdown")?;

parser.configure().with_err_semantic(false);
parser.parse("Off -> On @ Power")?;
```

[`Parser`]: trait.Parser.html
[`Builder`]: trait.Builder.html
[`Scdlang`]: ../struct.Scdlang.html */
use crate::{cache, Scdlang};
use std::{error::Error, fmt};

#[rustfmt::skip]
/** A Trait which external parser must implement.

This trait was mean to be used outside the core.
For example, to build a transpiler. */
pub trait Parser<'t>: fmt::Display {
	/// Parse `source` then replace the results.
	fn parse(&mut self, source: &str) -> Result<(), BoxError>;
	/// Parse `source` then insert/append the results.
	fn insert_parse(&mut self, source: &str) -> Result<(), BoxError>;

	/// Parse `source` while instantiate the Parser.
	fn try_parse(source: &str, options: Scdlang<'t>) -> Result<Self, BoxError> where Self: Sized;
	/// Configure the parser.
	fn configure(&mut self) -> &mut dyn Builder<'t>;

	/// Completely clear the caches which also deallocate the memory.
	fn flush_cache<'e>(&'t self) -> Result<(), DynError<'e>> {
		Ok(cache::clear()?.shrink()?)
	}

	/// Clear the caches while still retain the allocated memory.
	fn clean_cache<'e>(&'t self) -> Result<(), DynError<'e>> {
		cache::clear()?;
		Ok(())
	}
}

/// A Trait to configure the `Parser`.
/// This is a config builder for [`Scdlang`](../struct.Scdlang.html) core parser.
pub trait Builder<'t> {
	/** Automatically clear cache when out of scope.
	 * `default` - set `false` to disable it. (default: `true`)

	The cache is used for analyzing semantics error.
	This can be handy when parsing in streaming fashion. */
	fn auto_clear_cache(&mut self, default: bool) -> &mut dyn Builder<'t>;

	/// Enable semantics error. (default: `true`).
	fn with_err_semantic(&mut self, default: bool) -> &mut dyn Builder<'t>;
	/// Set path that going to be printed in the error essages.
	fn with_err_path(&mut self, path: &'t str) -> &mut dyn Builder<'t>;
	/// Set the line_of_code offset of the error essages.
	fn with_err_line(&mut self, line: usize) -> &mut dyn Builder<'t>;
}

type DynError<'t> = Box<dyn Error + 't>;
type BoxError = Box<dyn Error>;
