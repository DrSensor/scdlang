/*! A module for acessing semantics type.

See [`Scdlang`] for example.
[`Scdlang`]: ../struct.Scdlang.html */
mod graph;
mod kind;
mod transition;

pub(crate) use graph::*;
pub use kind::*;

pub(super) mod analyze {
	// WARNING: move this on separate file when it became more complex
	use crate::{error::Error, grammar::Rule, Scdlang};
	use pest::{iterators::Pair, Span};
	use std::convert::{TryFrom, TryInto};

	pub type TokenPair<'i> = Pair<'i, Rule>;

	/// A Trait that must be implmented for doing semactics checking.
	pub trait SemanticCheck<'c>: TryFrom<TokenPair<'c>, Error = Error> {
		fn analyze_error(&self, span: Span<'c>, options: &'c Scdlang) -> Result<(), Error>;

		/// Perform full semantics analysis from pest::iterators::Pair.
		fn analyze_from(pair: TokenPair<'c>, options: &'c Scdlang) -> Result<Self, Error> {
			let span = pair.as_span();
			let sc = pair.try_into()?;
			Self::analyze_error(&sc, span, options)?;
			// reserved for another analysis! 💪
			Ok(sc)
		}

		fn into_kind(self) -> super::Kind<'c>;
	}
}
