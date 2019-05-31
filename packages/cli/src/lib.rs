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
		Default,
	}

	// TODO: PR are welcome 😆
	pub const PRINTER: fn(&str, Mode) -> PrettyPrint = |lang, mode| {
		let mut printer = PrettyPrinter::default();
		printer // Default 👇
			.header(false)
			.grid(false)
			.line_numbers(false)
			.paging_mode(PagingMode::Never) // to support Alpine linux
			.theme("TwoDark")
			.language(lang);
		(match mode /*👆*/ {
			Mode::Default => printer.build(),
			Mode::UseHeader => printer.grid(true).header(true).build(),
			Mode::MultiLine => printer.grid(true).build(),
			Mode::Error => printer.grid(true).header(true).theme("Sublime Snazzy").paging_mode(PagingMode::Error).build(),
			Mode::REPL => printer.line_numbers(true).grid(true).build(),
			Mode::Debug => printer.line_numbers(true).grid(true).header(true).build(),
		})
		.unwrap() // because it only throw error if field not been initialized
	};
}

pub mod typedef {
	pub mod tuple {
		pub type Printer<Parser> = (prettyprint::PrettyPrint, Box<Parser>);
	}
}
