#![allow(clippy::unit_arg)]
mod schema;
use schema::*;

use scdlang_core::{prelude::*, semantics::Kind, Scdlang};
use serde::Serialize;
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
		let mut ast = ManuallyDrop::new(Self::try_parse(source, self.builder.to_owned())?);
		self.schema.states.append(&mut ast.schema.states);
		match (&mut self.schema.transitions, &mut ast.schema.transitions) {
			(Some(origin), Some(parsed)) => origin.extend_from_slice(parsed),
			(None, _) => self.schema.transitions = ast.schema.transitions.to_owned(),
			_ => {}
		}
		Ok(())
	}

	fn try_parse(source: &str, builder: Scdlang<'a>) -> Result<Self, DynError> {
		let mut schema = Coordinate::default();

		for kind in builder.iter_from(source)? {
			match kind {
				Kind::Expression(expr) => {
					schema.states.append(&mut vec![
						regular_state(expr.current_state().into()),
						regular_state(expr.next_state().into()),
					]);
					let transition = Transition {
						from: expr.current_state().into(),
						to: expr.next_state().into(),
						event: expr.event().map(|e| e.into()),
						label: expr.event().map(|e| e.into()),
						..Default::default()
					};
					match &mut schema.transitions {
						Some(transitions) => transitions.push(transition),
						None => schema.transitions = Some(vec![transition]),
					};
				}
				_ => unimplemented!("TODO: implement the rest on the next update"),
			}
		}

		Ok(Machine { schema, builder })
	}
}

fn regular_state(name: String) -> State {
	State {
		name,
		..Default::default()
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
