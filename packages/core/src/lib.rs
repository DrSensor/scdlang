mod core;

pub(crate) mod cache;

mod error;
pub mod external;
pub mod semantics;
pub mod utils;

pub use crate::core::{parse, Scdlang};
pub use error::Error;
pub use external::Parser as Transpiler;

/// A prelude providing convenient access to commonly-used features of scdlang core parser.
pub mod prelude {
	pub use super::{external::*, utils::naming::*};
	pub use pest::Parser as PestParser;
}

/** A helper module for aliasing several generated [`Rule`] which alias of [`pest::RuleType`]

# Examples
```ignore
use scdlang::{parse, grammar::*};

let pair = parse("A -> B")?;
match pair.as_rule() {
	Name::state => print!("state {}", pair.as_str()),
	Symbol::arrow::right | Symbol::arrow::left => print!(" {} ", pair.as_str()),
	_ => unreachable!()
}
```

[`Rule`]: grammar/enum.Rule.html
[`pest::RuleType`]: https://docs.rs/pest/latest/pest/trait.RuleType.html */
pub mod grammar {
	pub use super::core::Rule;

	#[allow(non_snake_case)]
	#[rustfmt::skip]
	/** Enum variants that represent a symbol (e.g `->`, `@`) 
	
	Each of them follow [unicode name](http://xahlee.info/comp/unicode_index.html) */
	pub mod Symbol {
		pub use super::Rule::TriggerAt as at;

		pub mod arrow {
			pub use crate::core::Rule::{
				TransitionTo as right,
				TransitionFrom as left,
				TransitionToggle as both,
			};
		}

		pub mod double_arrow {
			pub use crate::core::Rule::{
				LoopTo as right,
				LoopFrom as left,
			};
		}

		pub mod tail_arrow {
			pub use crate::core::Rule::{
				TransientLoopTo as right,
				TransientLoopFrom as left,
			};
		}

		pub mod triangle {
			pub use crate::core::Rule::{
				PlayNext as right,
			};
		}
	}

	#[allow(non_snake_case)]
	#[rustfmt::skip]
	/// Enum variants that represent a name (e.g `state`, `event`)
	pub mod Name {
		pub use super::Rule::{
			StateName as state,
			EventName as event,
			actionName as action,
			guardName as guard,
		};
	}
}

#[cfg(test)]
pub mod test {
	use super::*;
	use grammar::Rule;
	use pest::error::Error;

	const BASE_ISSUES: &str = "https://github.com/DrSensor/scdlang";
	pub fn issue(id: isize) {
		println!("{}/issues/{}", BASE_ISSUES, id)
	}

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
		use crate::{cache, error::Error, prelude::*};
		use pest::{
			error::ErrorVariant,
			iterators::{Pair, Pairs},
		};
		pub type Result = std::result::Result<(), Error>;

		pub fn expression<'a>(text: &'a str, callback: impl Fn(Pair<'a, Rule>) -> Result) -> Result {
			let declaration = Scdlang::parse_from(text)?;
			for expression in declaration {
				if let Rule::expression = expression.as_rule() {
					callback(expression)?
				}
			}
			cache::clear()?.shrink()
		}

		pub fn from<'a>(text: &'a str, callback: impl FnOnce(Pairs<'a, Rule>) -> Result) -> Result {
			let declaration = Scdlang::parse_from(text)?;
			callback(declaration)?;
			cache::clear()?.shrink()
		}

		pub fn error(text: &str, callback: impl Fn(&str, ErrorVariant<Rule>) -> Result) -> Result {
			let mut parser = Scdlang::new();
			parser.auto_clear_cache(false);
			for line in text.trim().lines() {
				if let Error::Parse(error) = parser.iter_from(line).expect_err(&format!("No Error for {}", line)) {
					callback(line.trim(), error.variant)?;
					cache::clear()?.shrink()?;
				} else {
					panic!("No Error for {}", line);
				}
			}
			Ok(())
		}
	}
}
