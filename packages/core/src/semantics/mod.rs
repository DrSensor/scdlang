/*! A module for acessing semantics type.

See [`Scdlang`](../struct.Scdlang.html) for example. */
mod graph;
mod kind;
mod transition;

pub(crate) use graph::*;
pub use kind::*;

pub(super) mod analyze {
	// WARNING: move this on separate file when it became more complex
	use super::*;
	use crate::{error::Error, grammar::Rule, Scdlang};
	use pest::{iterators::Pair, Span};

	pub type TokenPair<'i> = Pair<'i, Rule>;

	pub trait SemanticCheck: Expression {
		fn check_error(&self) -> Result<Option<String>, Error>;
	}

	/// A Trait that must be implmented for doing semantics checking.
	pub trait SemanticAnalyze<'c>: From<TokenPair<'c>> {
		fn analyze_error(&self, span: Span<'c>, options: &'c Scdlang) -> Result<(), Error>;

		/// Perform full semantics analysis from pest::iterators::Pair.
		fn analyze_from(pair: TokenPair<'c>, options: &'c Scdlang) -> Result<Self, Error> {
			let span = pair.as_span();
			let sc = pair.into();
			Self::analyze_error(&sc, span, options)?;
			// reserved for another analysis! 💪
			Ok(sc)
		}

		fn into_kinds(self) -> Vec<super::Kind<'c>>;
	}
}
