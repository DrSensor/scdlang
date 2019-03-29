use crate::{error::*, Rule, Scdlang};
use pest::{error::Error as PestError, iterators::Pairs, Parser};

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

type RuleError = PestError<Rule>;
