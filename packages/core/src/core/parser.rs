use super::{Rule, Scdlang};
use crate::error::*;
use pest::{self, error::Error as PestError, iterators::Pairs};
use std::fmt;

impl<'g> Scdlang<'g> {
	pub fn parse(&self, source: &'g str) -> Result<Pairs<Rule>, Error> {
		inner(parse(&source).map_err(|e| {
			Error::Parse(
				(match self.path {
					Some(path) => e.with_path(path),
					None => e,
				})
				.into(),
			)
		})?)
	}

	pub fn parse_from(source: &str) -> Result<Pairs<Rule>, Error> {
		inner(parse(&source).map_err(|e| Error::Parse(e.into()))?)
	}
}

pub fn parse(source: &str) -> Result<Pairs<Rule>, RuleError> {
	<Scdlang as pest::Parser<Rule>>::parse(Rule::DescriptionFile, source)
}

fn inner(root_pairs: Pairs<Rule>) -> Result<Pairs<Rule>, Error> {
	Ok(root_pairs.peek().ok_or(Error::EmptyDeclaration)?.into_inner())
}

impl fmt::Display for Scdlang<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", &self)
	}
}

type RuleError = PestError<Rule>;
