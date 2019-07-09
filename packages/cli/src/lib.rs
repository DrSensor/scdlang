#[path = "arg.rs"]
pub mod arg;
#[path = "cli.rs"]
pub mod cli;
#[path = "commands/mod.rs"]
pub mod commands;
#[path = "error.rs"]
pub mod error;

use error::Error as ScrapError;
use std::any::Any;

impl<T> Downcast for T {}
pub trait Downcast {
	// TODO: find a way to convert Box<dyn Any> into Box<dyn Error>
	// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=0b19ec3df257a583d4e560cbfb51b808
	fn downcast<T: Any>(self) -> Result<T, ScrapError<'static>>
	where
		Self: Sized + 'static,
	{
		let this: Box<dyn Any> = Box::new(self);
		this.downcast::<T>().map(|t| *t).map_err(|_| ScrapError::Downcast)
	}
}

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
	use std::{hash::Hash, iter::FromIterator, ops::Deref};

	impl Compare for &str {}
	impl<T: PartialEq + Clone> Merge<T> for Vec<T> {
		fn merge_value(&mut self, item: T) {
			if !self.iter().any(|v| *v == item) {
				self.push(item);
			}
		}
	}

	#[rustfmt::skip]
	pub fn merge<T: Clone + Hash + Eq>(slices: &[&[T]]) -> Vec<T> {
		use std::collections::HashSet;
		slices.iter().cloned()
			.flatten().cloned()
			.collect::<HashSet<_>>() // https://www.rosettacode.org/wiki/Remove_duplicate_elements#Rust
			.drain().collect()
	}

	pub trait Compare {
		fn one_of(&self, /*TODO:replace*/ collection: &[Self] /*with impl IntoIterator*/) -> bool
		where
			Self: Sized + PartialEq,
		{
			collection.iter().any(|item| item == self)
		}
	}

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
}

// TODO: Hacktoberfest
pub mod format {
	pub const XSTATE: [&str; 1] = ["json" /*, typescript*/];
	pub const SMCAT: &str = "json";
	#[rustfmt::skip]
	pub const BLOB: [&str; 13] = ["bmp", "gd", "gd2", "gif", "jpg", "jpeg", "jpe", "png", "svgz", "tif", "tiff", "vmlz", "webmp"];

	#[rustfmt::skip]
	pub mod ext {
		pub const SMCAT: [&str; 7] = ["svg", "dot", "smcat", "json", "html", "scxml", "xmi"];
		pub const DOT: [&str; 32] = ["bmp", "canon", "dot", "gv", "xdot", "eps", "fig", "gd", "gd2", "gif", "jpg", "jpeg", "jpe", "json", "json0", "dot_json", "xdot_json", "pic", "plain", "plain-ext", "png", "ps", "ps2", "svg", "svgz", "tif", "tiff", "tk", "vml", "vmlz", "vrml", "wbmp"];
		pub const GRAPH_EASY: [&str; 13] = ["ascii", "boxart", "svg", "dot", "txt", "bmp", "gif", "jpg", "pdf", "png", "ps", "ps2", "tif"];
	}

	pub fn into_legacy_dot(input: &str) -> String {
		use regex::Regex;
		let re = Regex::new(re::OLD_TO_NEW_DOT).expect("valid regex");
		re.replace_all(input, "").replace("note", "box")
	}

	mod re {
		pub const OLD_TO_NEW_DOT: &str = r#"( style=["']\w+["'])|( penwidth=["']?\d+.\d["']?)"#;
	}
}

// TODO: make a blog about "Categorizing process using trait system"
pub mod spawn {
	use std::{
		io::{self, Read, Write},
		process::*,
	};

	pub trait ShortProcess {
		type Input;
		type Output; //TODO:[String] wait associated type defaults to be stable
		fn output_from(self, input: Self::Input) -> io::Result<Self::Output>;
	}

	pub fn smcat(fmt: &str) -> io::Result<impl ShortProcess<Input = String, Output = String>> {
		let (input, output) = (None, None) as (Option<Stdio>, Option<Stdio>);
		Process::new("smcat", format!("-I json -d left-right -T {}", fmt)).spawn(input, output)
	}

	pub fn graph_easy(fmt: &str) -> io::Result<impl ShortProcess<Input = String, Output = String>> {
		let (input, output): (Option<Stdio>, Option<Stdio>) = (None, None);
		Process::new("graph-easy", format!("--as {}", fmt)).spawn(input, output)
	}

	pub fn dot(fmt: &str) -> impl ShortProcess<Input = (String, Child), Output = Vec<u8>> {
		Process::new("dot", format!("-T{}", fmt))
	}

	impl ShortProcess for Process<'_> {
		type Input = (String, Child);
		type Output = Vec<u8>;
		fn output_from(self, input: Self::Input) -> io::Result<Self::Output> {
			let mut output = Vec::new();
			let (input, mut child) = input;

			write!(child.stdin.as_mut().expect("process not exit"), "{}", input)?;
			child = self.spawn(child.stdout.take(), None as Option<Stdio>)?;
			child.wait()?;
			child.stdout.as_mut().expect("process to exit").read_to_end(&mut output)?;

			Ok(output)
		}
	}

	impl ShortProcess for Child {
		type Input = String;
		type Output = String;
		fn output_from(mut self, input: Self::Input) -> io::Result<Self::Output> {
			let mut output = String::new();

			write!(self.stdin.as_mut().expect("process not exit"), "{}", input)?;
			self.wait()?;
			self.stdout.as_mut().expect("process to exit").read_to_string(&mut output)?;

			Ok(output)
		}
	}

	pub trait ActiveProcess {
		type Input;
		fn output_from(&mut self, input: Self::Input) -> io::Result<String>;
	}

	// INSERT ActiveProcess cli that always keepalive here (if any)

	impl ActiveProcess for Process<'_> {
		type Input = (String, Child);
		fn output_from(&mut self, input: Self::Input) -> io::Result<String> {
			let mut output = String::new();
			let (input, mut child) = input;

			write!(child.stdin.as_mut().expect("process not exit"), "{}", input)?;
			let mut child = self.spawn(child.stdout.take(), None as Option<Stdio>)?;
			loop {
				if let Some(stdout) = child.stdout.as_mut() {
					stdout.read_to_string(&mut output)?;
					break;
				}
			}

			Ok(output)
		}
	}

	impl ActiveProcess for Child {
		type Input = String;
		fn output_from(&mut self, input: Self::Input) -> io::Result<String> {
			let mut output = String::new();

			write!(self.stdin.as_mut().expect("process not exit"), "{}", input)?;
			loop {
				if let Some(stdout) = self.stdout.as_mut() {
					stdout.read_to_string(&mut output)?;
					break;
				}
			}
			Ok(output)
		}
	}

	pub struct Process<'p> {
		cmd: &'p str,
		args: String,
	}

	impl<'p> Process<'p> {
		fn new(cmd: &'p str, args: String) -> Self {
			Process { cmd, args }
		}

		fn spawn(&self, input: Option<impl Into<Stdio>>, output: Option<impl Into<Stdio>>) -> io::Result<Child> {
			Command::new(self.cmd)
				.args(self.args.split_whitespace())
				.stdin(input.map_or(Stdio::piped(), Into::into))
				.stdout(output.map_or(Stdio::piped(), Into::into))
				.spawn()
		}
	}
}
