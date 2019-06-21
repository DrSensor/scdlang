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
			.language(match lang {
				"smcat" => "dot",
				"scxml" | "xmi" => "xml",
				"ascii" | "boxart" => "txt",
				_ => lang,
			})
			.build()
			.unwrap() // because it only throw error if field not been initialized
	};
}

pub mod prelude {
	use std::{iter::FromIterator, ops::Deref};
	pub trait Merge<T: PartialEq + Clone>: FromIterator<T> + Deref<Target = [T]> {
		fn merge_value(&mut self, item: T);
		fn merge_from_slice(&mut self, items: &[T]) {
			for item in items {
				self.merge_value(item.to_owned())
			}
		}
		fn merge(&mut self, items: Self) {
			self.merge_from_slice(&items)
		}
	}

	impl<T> Merge<T> for Vec<T>
	where
		T: PartialEq + Clone,
	{
		fn merge_value(&mut self, item: T) {
			if !self.iter().any(|v| *v == item) {
				self.push(item);
			}
		}
	}
}

pub mod format {
	pub const XSTATE: [&str; 1] = ["json" /*, typescript*/];
	pub const SMCAT: &str = "json";

	pub mod ext {
		pub const SMCAT: [&str; 7] = ["svg", "dot", "smcat", "json", "html", "scxml", "xmi"];
		pub const GRAPH_EASY: [&str; 10] = ["ascii", "boxart", "bmp", "gif", "jpg", "pdf", "png", "ps", "ps2", "tif"];
	}
}

pub mod exec {
	use super::format;
	use regex::Regex;
	use std::{
		io::{self, Write},
		process::{Child, Command, Stdio},
		str::from_utf8,
	};

	pub fn smcat(fmt: &str, input: String) -> io::Result<String> {
		let mut command = spawn(
			"smcat",
			&[
				"--input-type",
				"json",
				"--output-type",
				if format::ext::GRAPH_EASY.iter().any(|f| f == &fmt) {
					"dot"
				} else {
					fmt
				},
			],
		)?;
		write!(command.stdin.as_mut().unwrap(), "{}", input)?;
		Ok(from_utf8(&command.wait_with_output()?.stdout).unwrap().to_string())
	}

	pub fn graph_easy(fmt: &str, input: String) -> io::Result<String> {
		let mut command = spawn("graph-easy", &["--as", fmt])?;
		let re = Regex::new(r#"( style=["']?\w+["']?)|( penwidth=["']?\d+.\d["']?)"#).unwrap();

		write!(command.stdin.as_mut().unwrap(), "{}", re.replace_all(&input, ""))?;
		Ok(from_utf8(&command.wait_with_output()?.stdout).unwrap().to_string())
	}

	fn spawn(cmd: &str, args: &[&str]) -> io::Result<Child> {
		Command::new(cmd)
			.args(args)
			.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.spawn()
	}
}
