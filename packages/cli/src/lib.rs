#[path = "arg.rs"]
pub mod arg;
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

		#[allow(dead_code)]
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
				"smcat" => "perl",
				"scxml" | "xmi" => "xml",
				"ascii" | "boxart" => "txt",
				_ => lang,
			})
			.build()
			.unwrap() // because it only throw error if field not been initialized
	};
}

pub mod iter {
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

	pub fn into_legacy_dot(input: &str) -> String {
		use regex::Regex;
		let re = Regex::new(r#"( style=["']?\w+["']?)|( penwidth=["']?\d+.\d["']?)"#).expect("valid regex");
		re.replace_all(input, "").to_string()
	}
}

pub mod spawn {
	use super::format;
	use std::{
		io::{self, Read, Write},
		process::{Child, Command, Stdio},
	};

	pub trait ShortProcess {
		fn output_from(self, input: String) -> io::Result<String>;
	}

	pub fn smcat(fmt: &str) -> io::Result<impl ShortProcess> {
		Ok(Process(spawn(
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
		)?))
	}

	pub fn graph_easy(fmt: &str) -> io::Result<impl ShortProcess> {
		Ok(Process(spawn("graph-easy", &["--as", fmt])?))
	}

	impl ShortProcess for Process {
		fn output_from(mut self, input: String) -> io::Result<String> {
			let mut output = String::new();

			write!(self.0.stdin.as_mut().expect("process not exit"), "{}", input)?;
			self.0.wait()?;
			self.0.stdout.as_mut().expect("process to exit").read_to_string(&mut output)?;

			Ok(output)
		}
	}

	pub trait ActiveProcess {
		fn output_from(&mut self, input: String) -> io::Result<String>;
	}

	// INSERT ActiveProcess cli that always keepalive here (if any)

	impl ActiveProcess for Process {
		fn output_from(&mut self, input: String) -> io::Result<String> {
			let mut output = String::new();

			write!(self.0.stdin.as_mut().expect("process not exit"), "{}", input)?;
			self.0.wait()?;
			self.0.stdout.as_mut().expect("process to exit").read_to_string(&mut output)?;

			Ok(output)
		}
	}

	struct Process(Child);
	fn spawn(cmd: &str, args: &[&str]) -> io::Result<Child> {
		Command::new(cmd)
			.args(args)
			.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.spawn()
	}
}
