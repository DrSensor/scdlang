mod graph;
mod kind;
mod transition;

pub use graph::*;
pub use kind::*;

pub(super) mod analyze {
	use crate::{error::Error, grammar::Rule, Scdlang};
	use pest::iterators::Pair;
	use std::convert::TryFrom;

	pub type TokenPair<'i> = Pair<'i, Rule>;

	pub trait SemanticCheck<'c>: TryFrom<TokenPair<'c>, Error = Error> {
		fn analyze_from(pair: TokenPair<'c>, options: &'c Scdlang) -> Result<Self, Error>;
		fn into_kind(self) -> super::Kind<'c>;
	}
}
