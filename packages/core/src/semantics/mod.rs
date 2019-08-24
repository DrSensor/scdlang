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
	use crate::{cache, error::*, grammar::Rule, Scdlang};
	use pest::{iterators::Pair, Span};

	pub type TokenPair<'i> = Pair<'i, Rule>;

	// TODO: report visibility of pub(crate) inside pub(super) as bugs
	//		 it's a workaround for ErrorMap
	pub(crate) trait SemanticCheck: Expression {
		fn check_error(&self) -> Result<Option<String>, Error>;
		fn check_warning(&self) -> Result<Option<ErrorMap>, Error> {
			Ok(None)
		}
	}

	/// A Trait that must be implmented for doing semantics checking.
	pub trait SemanticAnalyze<'c>: From<TokenPair<'c>> {
		fn analyze_warning(&self, _span: Span<'c>, _options: &'c Scdlang) -> Result<(), Error> {
			Ok(())
		}
		// span is not borrowed because PestError::new_from_span(..) is consumable
		fn analyze_error(&self, span: Span<'c>, options: &'c Scdlang) -> Result<(), Error>;

		/// Perform full semantics analysis from pest::iterators::Pair.
		fn analyze_from(pair: TokenPair<'c>, options: &'c Scdlang) -> Result<Self, Error> {
			let this = pair.clone().into();
			// WARNING: there is possibility that one expression can contain both error and warning because of sugar syntax (<->, ->>, >->)
			Self::analyze_error(&this, pair.as_span(), options)?;
			if let Err(Error::WithId { id, error }) = Self::analyze_warning(&this, pair.as_span(), options) {
				cache::write::warning()?
					.entry(id)
					.and_modify(|e| *e = error.to_string())
					.or_insert_with(|| error.to_string());
			}
			// reserved for another analysis! 💪
			Ok(this)
		}

		fn into_kinds(self) -> Vec<super::Kind<'c>>;
	}
}
