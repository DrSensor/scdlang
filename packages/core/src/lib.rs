mod core;

pub mod error;
pub mod external;
pub mod semantics;
pub mod utils;

pub use crate::core::{parse, Scdlang};
pub use external::Parser as Transpiler;
pub use semantics::*;

pub mod prelude {
	pub use super::{external::*, utils::iterators::*};

	pub use pest::Parser as PestParser;
	pub use std::convert::*;
}

pub mod grammar {
	pub use super::core::Rule;

	#[allow(non_snake_case)]
	#[rustfmt::skip]
	pub mod Symbol {
		pub use super::Rule::{
			TransitionTo as to,
			TriggerAt as at
		};
	}

	#[allow(non_snake_case)]
	#[rustfmt::skip]
	pub mod Name {
		pub use super::Rule::{
			StateName as state,
			EventName as event
		};
	}
}

#[cfg(test)]
pub mod test {
	use super::*;
	use grammar::Rule;
	use pest::error::Error;

	pub fn expression(expression: &str) -> Result<&str, Error<Rule>> {
		Ok(crate::parse(expression)?.as_str())
	}

	pub fn correct_expressions(expr_list: &[&str]) -> Result<(), String> {
		for expression in expr_list {
			if let Err(expr) = test::expression(expression) {
				eprintln!("{}", expr.to_string()); // TODO: remove this after Rust test error reporting is better 😅
				return Err(expr.to_string());
			}
		}
		Ok(())
	}

	pub fn wrong_expressions(expr_list: &[&str]) -> Result<(), String> {
		for expression in expr_list {
			if let Ok(expr) = test::expression(expression) {
				return Err(String::from(expr));
			}
		}
		Ok(())
	}

	pub mod parse {
		use super::*;
		use crate::error::Error;
		use pest::iterators::{Pair, Pairs};
		pub type Result = std::result::Result<(), Error>;
		type Closure<P> = fn(P) -> Result;

		pub fn expression<'a>(text: &'a str, callback: Closure<Pair<'a, Rule>>) -> Result {
			let declaration = Scdlang::parse_from(text)?;
			for expression in declaration {
				if let Rule::expression = expression.as_rule() {
					callback(expression)?
				}
			}
			Ok(())
		}

		pub fn from(text: &'static str, callback: Closure<Pairs<'_, Rule>>) -> Result {
			let declaration = Scdlang::parse_from(text)?;
			callback(declaration)
		}
	}
}
