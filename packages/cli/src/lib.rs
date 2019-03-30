#![allow(clippy::type_complexity)]

pub mod prompt {
	pub const REPL: &str = ">";
	pub const ERROR: &str = "ERROR:";
}

pub mod print {
	use prettyprint::{PrettyPrint, PrettyPrinter};

	pub enum Mode {
		REPL,
		Default,
	}

	// TODO: PR are welcome 😆
	pub const PRINTER: fn(&str, Mode) -> PrettyPrint = |lang, mode| {
		let mut printer = PrettyPrinter::default();
		printer // Default 👇
			.header(false)
			.grid(false)
			.line_numbers(false)
			.theme("TwoDark")
			.language(lang);
		(match mode /*👆*/ {
			Mode::Default => printer.build(),
			Mode::REPL => printer.line_numbers(true).build(),
		})
		.unwrap() // because it only throw error if field not been initialized
	};
}

pub mod typedef {
	pub mod tuple {
		pub type Printer<Parser> = (prettyprint::PrettyPrint, Box<Parser>);
	}
}
