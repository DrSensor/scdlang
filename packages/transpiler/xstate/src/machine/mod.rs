#![allow(clippy::unit_arg)]
mod schema;
use schema::*;

use scdlang::{prelude::*, semantics::Kind, Scdlang};
use serde::Serialize;
use std::{error, fmt, mem::ManuallyDrop};
use voca_rs::case::{camel_case, shouty_snake_case};

#[derive(Default, Serialize)]
/** Transpiler Scdlang → XState.

# Examples
```no_run
let xstate = Machine::new();

xstate.configure().with_err_path("test.scl");
parser.parse("A -> B")?;

println!("{}", parser.to_string());
``` */
pub struct Machine<'a> {
	#[serde(skip)]
	builder: Scdlang<'a>,

	#[serde(flatten)]
	schema: StateChart, // TODO: replace with 👇 when https://github.com/serde-rs/serde/issues/1507 resolved
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
		let ast = ManuallyDrop::new(Self::try_parse(source, self.builder.to_owned())?);
		for (current_state, transition) in ast.schema.states.to_owned(/*FIXME: expensive clone*/) {
			self.schema
				.states
				.entry(current_state)
				.and_modify(|t| t.on.extend(transition.on.clone()))
				.or_insert(transition);
		}
		Ok(())
	}

	fn try_parse(source: &str, builder: Scdlang<'a>) -> Result<Self, DynError> {
		let mut schema = StateChart::default();

		for kind in builder.iter_from(source)? {
			match kind {
				Kind::Expression(expr) => {
					let current_state = expr.current_state().map(camel_case);
					let next_state = expr.next_state().map(camel_case);
					let action = expr.action().map(|e| e.map(camel_case));
					let guard = expr.guard().map(|e| e.map(camel_case));
					let event_name = expr.event().map(|e| e.map(shouty_snake_case)).unwrap_or_default();

					let transition = if guard.is_none() && action.is_none() {
						Transition::Target(next_state)
					} else {
						Transition::Object {
							target: if next_state.is_empty() { None } else { Some(next_state) },
							actions: action,
							cond: guard,
						}
					};

					schema
						.states
						.entry(current_state)
						.and_modify(|t| {
							t.on.entry(event_name.to_string()).or_insert_with(|| transition.clone());
						})
						.or_insert(State {
							// TODO: waiting for map macros https://github.com/rust-lang/rfcs/issues/542
							on: [(event_name.to_string(), transition)].iter().cloned().collect(),
						});
				}
				_ => unimplemented!("TODO: implement the rest on the next update"),
			}
		}

		Ok(Machine { schema, builder })
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
			schema: StateChart::default(),
		}
	}
}

impl Drop for Machine<'_> {
	fn drop(&mut self) {
		self.flush_cache().expect("xstate: Deadlock");
	}
}

impl fmt::Display for Machine<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", serde_json::to_string_pretty(&self.schema).map_err(|_| fmt::Error)?)
	}
}

type DynError = Box<dyn error::Error>;

#[cfg(test)]
mod test {
	use super::*;
	use assert_json_diff::assert_json_eq;
	use serde_json::json;

	#[test]
	fn transient_transition() -> Result<(), DynError> {
		let mut machine = Machine::new();
		machine.parse("AlphaGo -> BetaRust")?;

		Ok(assert_json_eq!(
			json!({
				"states": {
					"alphaGo": {
						"on": {
							"": "betaRust"
						}
					}
				}
			}),
			json!(machine)
		))
	}

	#[test]
	fn eventful_transition() -> Result<(), DynError> {
		let mut machine = Machine::new();
		machine.parse(
			"A -> B @ CarlieCaplin
			A <- B @ CarlieCaplin
			A -> D @ EnhancedErlang",
		)?;

		Ok(assert_json_eq!(
			json!({
				"states": {
					"a": {
						"on": {
							"CARLIE_CAPLIN": "b",
							"ENHANCED_ERLANG": "d"
						}
					},
					"b": {
						"on": {
							"CARLIE_CAPLIN": "a"
						}
					}
				}
			}),
			json!(machine)
		))
	}

	#[test]
	fn no_clear_cache() {
		let mut machine = Machine::new();
		machine.parse("A -> B").expect("Nothing happened");
		machine.insert_parse("A -> C").expect_err("Duplicate transition");

		assert_json_eq!(
			json!({
				"states": {
					"a": {
						"on": {
							"": "b"
						}
					}
				}
			}),
			json!(machine)
		)
	}

	#[test]
	fn clear_cache() {
		let mut machine = Machine::new();
		machine.insert_parse("A -> B").expect("Nothing happened");
		machine.parse("A -> C").expect("Clear cache and replace schema");

		assert_json_eq!(
			json!({
				"states": {
					"a": {
						"on": {
							"": "c"
						}
					}
				}
			}),
			json!(machine)
		)
	}
}
