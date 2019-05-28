use super::{Rule, Scdlang};
use crate::{
	error::*,
	semantics::{analyze::SemanticCheck, *},
};
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
		// WARNING: ideally 👆 should be implemented using function generator but Rust not support it yet
	}
}

impl fmt::Display for Scdlang<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", &self)
	}
}

type RuleError = PestError<Rule>;
