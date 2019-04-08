use crate::external::Builder;
use pest_derive::Parser;

#[derive(Parser, Default, Copy, Clone)] // 🤔 is it wise to derive from Copy&Clone ?
#[grammar = "grammar.pest"]
pub struct Scdlang<'g> {
	pub(super) path: Option<&'g str>,
	pub(super) line: Option<u16>,
}

impl<'g> Builder<'g> for Scdlang<'g> {
	fn with_err_path(&mut self, path: &'g str) {
		self.path = Some(path);
	}

	fn with_err_line(&mut self, line: u16) {
		self.line = Some(line);
	}
}
