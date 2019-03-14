mod grammar;

pub use grammar::*;
use pest::{error::Error, iterators::Pairs, Parser};

type RuleError = Error<Rule>;

pub fn parse(source: &str) -> Result<Pairs<Rule>, RuleError> {
	Scdlang::parse(Rule::DescriptionFile, source)
}

#[cfg(test)]
pub mod test {
	use super::*;

	pub fn expression(expression: &str) -> Result<&str, Error<Rule>> {
		Ok(Scdlang::parse(Rule::DescriptionFile, expression)?.as_str())
	}

	pub fn correct_expressions(expr_list: &[&str]) -> Result<(), String> {
		for expression in expr_list {
			if let Err(expr) = test::expression(expression) {
				println!("{}", expr.to_string()); // TODO: remove this after Rust test error reporting is better 😅
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
}
