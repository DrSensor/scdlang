use crate::{cache, error::Error, external::Builder};
use pest_derive::Parser;

#[derive(Parser, Default, Clone)] // 🤔 is it wise to derive from Copy&Clone ?
#[grammar = "grammar.pest"]
pub struct Scdlang<'g> {
	pub(crate) path: Option<&'g str>,
	pub(crate) line: Option<usize>,

	pub(super) clear_cache: bool,    //-|in case for program that need to disable…|
	pub(super) semantic_error: bool, //-|…then enable semantic error at runtime|
}

impl Scdlang<'_> {
	/// This method is prefered when instantiating than using`::default()`
	pub fn new() -> Self {
		Self {
			clear_cache: true,
			semantic_error: true,
			..Default::default()
		}
	}

	/// Manually drop this object which can return Err(Deadlock), useful when in multi-thread process
	/// Call this if you want to avoid panic! by Drop when out of scope cause
	pub fn finish(mut self) -> Result<Self, Error> {
		self.clear_cache = false;
		clear_cache()?;
		Ok(self)
	}
}

impl<'g> Builder<'g> for Scdlang<'g> {
	fn with_err_path(&mut self, path: &'g str) -> &mut dyn Builder<'g> {
		self.path = Some(path);
		self
	}

	fn with_err_line(&mut self, line: usize) -> &mut dyn Builder<'g> {
		self.line = Some(line);
		self
	}

	fn with_err_semantic(&mut self, default: bool) -> &mut dyn Builder<'g> {
		self.semantic_error = default;
		self
	}

	fn auto_clear_cache(&mut self, default: bool) -> &mut dyn Builder<'g> {
		self.clear_cache = default;
		self
	}
}

impl<'g> Drop for Scdlang<'g> {
	fn drop(&mut self) {
		if self.clear_cache {
			clear_cache().expect("Deadlock")
		}
	}
}

#[inline]
fn clear_cache() -> Result<(), Error> {
	cache::clear()?.shrink()
}
