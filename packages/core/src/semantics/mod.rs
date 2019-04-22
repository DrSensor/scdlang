mod graph;
mod kind;
mod transition;

pub use graph::*;
pub use kind::*;

pub(super) mod analyze {
	use crate::{error::Error, grammar::Rule, Scdlang};
	use pest::{iterators::Pair, Span};
	use std::convert::{TryFrom, TryInto};

	pub type TokenPair<'i> = Pair<'i, Rule>;

	pub trait SemanticCheck<'c>: TryFrom<TokenPair<'c>, Error = Error> {
		fn analyze_from(pair: TokenPair<'c>, options: &'c Scdlang) -> Result<Self, Error> {
			let span = pair.as_span();
			let sc = pair.try_into()?;
			Self::analyze_error(&sc, span, options)?;
			// reserved for another analysis!
			Ok(sc)
		}
		fn analyze_error(&self, span: Span<'c>, options: &'c Scdlang) -> Result<(), Error>;
		fn into_kind(self) -> super::Kind<'c>;
	}
}
