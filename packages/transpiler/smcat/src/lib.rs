#![allow(clippy::unit_arg)]
mod schema;
use schema::*;

use scdlang_core::{prelude::*, semantics::Kind, Scdlang};
use serde::Serialize;
use serde_json::json;
use std::{error, fmt, mem::ManuallyDrop};

#[derive(Default, Serialize)]
/** Transpiler Scdlang → State Machine Cat (JSON).

# Examples
```no_run
let smcat = Machine::new();

smcat.configure().with_err_path("test.scl");
parser.parse("A -> B")?;

println!("{}", parser.to_string());
``` */
pub struct Machine<'a> {
	#[serde(skip)]
	builder: Scdlang<'a>,

	#[serde(flatten)]
	schema: Coordinate, // TODO: replace with 👇 when https://github.com/serde-rs/serde/issues/1507 resolved
	                    // schema: mem::ManuallyDrop<StateChart>,
}

impl<'a> Parser<'a> for Machine<'a> {
	fn configure(&mut self) -> &mut Builder<'a> {
		&mut self.builder
	}

	fn parse(&mut self, source: &str) -> Result<(), DynError> {
		self.clean_cache()?;
		let ast = ManuallyDrop::new(Self::try_parse(source, self.builder.to_owned())?);
		Ok(self.schema = ast.schema.to_owned()) // FIXME: expensive clone
	}

	fn insert_parse(&mut self, source: &str) -> Result<(), DynError> {
		unimplemented!("TODO: Parse then insert into [smcat] Machine.schema")
	}

	fn try_parse(source: &str, builder: Scdlang<'a>) -> Result<Self, DynError> {
		unimplemented!("TODO: Parse into [smcat] Machine.schema")
	}
}

impl Machine<'_> {
	/// Create new StateMachine.
	/// Use this over `Machine::default()`❗
	pub fn new() -> Self {
		let mut builder = Scdlang::new();
		builder.auto_clear_cache(false);
		Self {
			builder,
			schema: Coordinate::default(),
		}
	}
}

impl Drop for Machine<'_> {
	fn drop(&mut self) {
		self.flush_cache().expect("smcat: Deadlock");
	}
}

impl fmt::Display for Machine<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", serde_json::to_string_pretty(&self.schema).map_err(|_| fmt::Error)?)
	}
}

type DynError = Box<dyn error::Error>;

#[cfg(test)]
mod tests {}
