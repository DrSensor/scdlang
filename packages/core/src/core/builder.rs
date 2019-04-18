use crate::external::Builder;
use pest_derive::Parser;

#[derive(Parser, Default, Clone)] // 🤔 is it wise to derive from Copy&Clone ?
#[grammar = "grammar.pest"]
pub struct Scdlang<'g> {
	pub(crate) path: Option<&'g str>,
	pub(crate) line: Option<usize>,
	pub(super) clear_cache: Option<bool>,
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
