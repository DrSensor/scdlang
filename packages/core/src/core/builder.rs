use crate::{cache, external::Builder};
use pest_derive::Parser;

#[derive(Parser, Default, Clone)] // 🤔 is it wise to derive from Copy&Clone ?
#[grammar = "grammar.pest"]
pub struct Scdlang<'g> {
	pub(super) source: &'g str,

	pub(crate) path: Option<&'g str>,
	pub(crate) line: Option<usize>,
	pub(super) clear_cache: Option<bool>,
}

impl<'g> Scdlang<'g> {
	pub fn new(source: &'g str) -> Self {
		Scdlang {
			source,
			..Default::default()
		}
	}
}

impl<'g> Builder<'g> for Scdlang<'g> {
	fn with_err_path(&mut self, path: &'g str) {
		self.path = Some(path);
	}

	fn with_err_line(&mut self, line: usize) {
		self.line = Some(line);
	}

	fn auto_clear_cache(&mut self, default: bool) {
		self.clear_cache = Some(default);
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
