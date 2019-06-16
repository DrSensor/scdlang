#[path = "cli.rs"]
pub mod cli;
#[path = "commands/mod.rs"]
pub mod commands;
#[path = "error.rs"]
pub mod error;

pub mod prompt {
	use rustyline::config::{self, *};

	pub const REPL: &str = "»";
	pub const ERROR: &str = "ERROR:";

	// TODO: PR are welcome 😆
	pub const CONFIG: fn() -> config::Config = || {
		config::Builder::new()
			.history_ignore_dups(true)
			.history_ignore_space(true)
			.auto_add_history(true)
			.completion_type(CompletionType::List)
			.build()
	};
}

pub mod print {
	use prettyprint::{PagingMode, PrettyPrint, PrettyPrinter};

	pub enum Mode {
		REPL,
		Debug,
		Error,
		MultiLine,
		UseHeader,
		Plain,
	}

	pub trait PrinterChange {
		fn change(&self, mode: Mode) -> Self;
		fn change_language(&self, lang: &str) -> Self;
		fn change_theme(&self, lang: &str) -> Self;
	}

	impl PrinterChange for PrettyPrint {
		fn change(&self, mode: Mode) -> Self {
			(match mode {
				Mode::Plain => self.configure().grid(false).header(false).line_numbers(false).build(),
				Mode::UseHeader => self.configure().grid(true).header(true).build(),
				Mode::MultiLine => self.configure().grid(true).build(),
				Mode::REPL => self.configure().line_numbers(true).grid(true).build(),
				Mode::Debug => self.configure().line_numbers(true).grid(true).header(true).build(),
				Mode::Error => self
					.configure()
					.grid(true)
					.header(true)
					.theme("Sublime Snazzy")
					.paging_mode(PagingMode::Error)
					.build(),
			})
			.unwrap() // because it only throw error if field not been initialized
		}
		fn change_language(&self, lang: &str) -> Self {
			self.configure().language(lang).build().unwrap()
		}
		fn change_theme(&self, theme: &str) -> Self {
			self.configure().theme(theme).build().unwrap()
		}
	}

	pub const PRINTER: fn(&str) -> PrettyPrint = |lang| {
		let mut printer = PrettyPrinter::default();
		printer // Default Mode::Plain 👇
			.header(false)
			.grid(false)
			.line_numbers(false)
			.paging_mode(PagingMode::Never) // to support Alpine linux
			.theme("TwoDark")
			.language(lang)
			.build()
			.unwrap() // because it only throw error if field not been initialized
	};
}
