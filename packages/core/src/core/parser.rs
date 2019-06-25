use super::{Rule, Scdlang};
use crate::{
	error::*,
	semantics::{analyze::SemanticCheck, *},
};
use either::Either;
use pest::{self, error::Error as PestError, iterators::Pairs};
use std::{fmt, *};

/** Wrapper for [`pest::Parser::parse`](https://docs.rs/pest/latest/pest/trait.Parser.html#tymethod.parse)

# Examples
```
let token_pairs = parse("A -> B")?;
println("{:#?}", token_pairs);
``` */
pub fn parse(source: &str) -> Result<Pairs<Rule>, RuleError> {
	<Scdlang as pest::Parser<Rule>>::parse(Rule::DescriptionFile, source)
}

impl<'g> Scdlang<'g> {
	/// Parse from `source` and also reformat the error messages.
	pub fn parse(&self, source: &'g str) -> Result<Pairs<Rule>, Error> {
		parse(source).map_err(|e| Error::Parse(self.reformat_error(source, e).into()))
	}

	/// Parse from `source` but don't modify or fix the error messages.
	pub fn parse_from(source: &str) -> Result<Pairs<Rule>, Error> {
		parse(source).map_err(|e| Error::Parse(e.into()))
	}

	/// Parse from `source` then iterate.
	/// This is the preferred methods for implementing transpiler, codegen, or compiler.
	pub fn iter_from(&self, source: &'g str) -> Result<Vec<Kind>, Error> {
		let pairs = self.parse(source)?;
		pairs
			.filter(|pair| if let Rule::EOI = pair.as_rule() { false } else { true })
			.map(|pair| {
				Ok(match pair.as_rule() {
					// Rule::expression => Transition::from(pair).into_kinds(),
					Rule::expression => Transition::analyze_from(pair, &self)?.into_kinds(),
					_ => unreachable!("Rule::{:?}", pair.as_rule()),
				})
			})
			// see https://paulkernfeld.com/2018/11/03/flatten-nested-iterator-result.html
			// and https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=5d59f87affa104d758d9c58655d4aed9
			.flat_map(|result| match result {
				Err(e) => Either::Left(iter::once(Err(e))),
				Ok(kinds) => Either::Right(kinds.into_iter().map(Ok)),
			})
			.collect()
		// WARNING: ideally 👆 should be implemented using function generator but Rust not support it yet. Maybe I should use closure arg 🤔
	}
}

impl fmt::Display for Scdlang<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", &self)
	}
}

type RuleError = PestError<Rule>;
