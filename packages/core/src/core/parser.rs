use super::{Rule, Scdlang};
use crate::{
	error::*,
	semantics::{analyze::SemanticCheck, *},
};
use pest::{self, error::Error as PestError, iterators::Pairs};
use std::{fmt, *};

/// Wrapper for pest::Parser::parse(...)
pub fn parse(source: &str) -> Result<Pairs<Rule>, RuleError> {
	<Scdlang as pest::Parser<Rule>>::parse(Rule::DescriptionFile, source)
}

impl<'g> Scdlang<'g> {
	pub fn parse(&self, source: &'g str) -> Result<Pairs<Rule>, Error> {
		inner(parse(&source).map_err(|e| {
			let mut error = e;
			if let Some(offset) = self.line {
				error = error.with_offset(offset, source);
			}
			if let Some(path) = self.path {
				error = error.with_path(path);
			}
			Error::Parse(error.into())
		})?)
	}

	pub fn parse_from(source: &str) -> Result<Pairs<Rule>, Error> {
		inner(parse(&source).map_err(|e| Error::Parse(e.into()))?)
	}

	pub fn iter_from(&self, source: &'g str) -> Result<Vec<Kind<'_>>, Error> {
		use convert::TryFrom;
		let pairs = self.parse(source)?;
		pairs
			.filter(|pair| if let Rule::EOI = pair.as_rule() { false } else { true })
			.map(|pair| {
				Ok(match pair.as_rule() {
					Rule::expression if self.semantic_error => Transition::analyze_from(pair, &self)?.into_kind(),
					Rule::expression => Transition::try_from(pair)?.into_kind(),
					_ => unreachable!("Rule::{:?}", pair.as_rule()),
				})
			})
			.collect()
	}
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
