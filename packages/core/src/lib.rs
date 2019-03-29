pub mod error;
mod grammar;
mod parser;
pub mod semantics;
pub mod utils;

pub use grammar::*;
pub use parser::parse;
pub use semantics::*;

pub mod prelude {
	use super::*;

	pub use pest::Parser;
	pub use std::convert::*;
	pub use utils::iterators::*;
}

#[cfg(test)]
pub mod test {
	use super::*;
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
