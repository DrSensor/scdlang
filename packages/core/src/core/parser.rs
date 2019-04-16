use super::{Rule, Scdlang};
use crate::{cache, error::*};
use pest::{self, error::Error as PestError, iterators::Pairs, Position};
use std::{fmt, iter};

impl<'g> Scdlang<'g> {
	pub fn parse(&self, source: &'g str) -> Result<Pairs<Rule>, Error> {
		use pest::error::InputLocation::*;
		inner(parse(&source).map_err(|e| {
			let mut error = e;
			if let Some(offset) = self.line {
				//TODO: make PR on pest to add `fn with_line(self, offset: usize) -> Error<R>`
				if let Pos(line) = error.location {
					error = PestError::new_from_pos(
						error.variant,
						Position::new(
							&format!(
								"{offset}{src}",
								offset = iter::repeat('\n').take(offset).collect::<String>(),
								src = source
							),
							line + offset,
						)
						.unwrap(),
					);
				}
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
}

impl<'g> Drop for Scdlang<'g> {
	fn drop(&mut self) {
		let clear_cache = || cache::drop().expect("Deadlock");
		match self.clear_cache {
			None => clear_cache(), // default behaviour
			Some(auto_clear) if auto_clear => clear_cache(),
			_ => { /* don't clear cache */ }
		}
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
