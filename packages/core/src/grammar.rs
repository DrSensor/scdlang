use crate::error::*;
use pest::{error::Error as PestError, iterators::Pairs, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Scdlang;
type RuleError = PestError<Rule>;

impl Scdlang {
	pub fn parse_from(source: &str) -> Result<Pairs<Rule>, Error> {
		Ok(parse(&source)
			.map_err(|e| Error::Parse(e.into()))?
			.peek()
			.ok_or(Error::EmptyDeclaration)?
			.into_inner())
	}
}

pub fn parse(source: &str) -> Result<Pairs<Rule>, RuleError> {
	Scdlang::parse(Rule::DescriptionFile, source)
}

#[cfg(test)]
mod test {
	pub use crate::test;
	pub type Yes = Result<(), String>;

	#[test]
	fn transition_to() -> Yes {
		test::correct_expressions(&[r#"A->B"#, r#"Alpha-->B"#, r#"A--->Beta"#, r#"AlphaGo->BetaRust"#])
	}

	mod should_fail_when {
		use super::*;

		#[test]
		fn use_wrong_symbol() -> Yes {
			// From https://github.com/tonsky/FiraCode 😋
			test::wrong_expressions(&[
				// #region transition_to
				r#"A->>B"#, r#"A>->B"#, r#"A>-B"#, r#"A>>-B"#, r#"A~>B"#, r#"A~~>B"#,
				// #endregion
			])
		}
	}
}
