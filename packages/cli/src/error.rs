use crate::{print::*, prompt};
use atty::Stream;
use clap::ArgMatches;
use colored::*;
use prettyprint::PrettyPrint;
use scdlang_core as scdlang;
use std::*;

#[derive(Debug)]
pub enum Error<'s> {
	Count {
		count: usize,
		topic: &'s str,
	},
	StreamParse(&'s str),
	WrongFormat {
		target: String,
		format: String,
		possible_formats: &'s [&'s str],
	},
}

pub trait Report: fmt::Debug + fmt::Display + Sized {
	fn report_and_exit(&self, default_exit_code: Option<i32>, args: Option<ArgMatches>);
	fn report(&self) {
		self.report_and_exit(None, None)
	}
}

impl Report for Box<dyn error::Error> {
	fn report_and_exit(&self, default_exit_code: Option<i32>, arguments: Option<ArgMatches>) {
		let print = PRINTER("haskell").change(Mode::Error);
		let args: ArgMatches = arguments.unwrap_or_default();

		if let Some(err) = self.downcast_ref::<io::Error>() {
			use io::ErrorKind::*;
			match err.kind() {
				NotFound | PermissionDenied | AlreadyExists => prompting(&format!(
					"{} for {}",
					remove_os_error(err.to_string()).replace(" or directory", ""),
					args.value_of("FILE").unwrap_or("<FILE>").yellow(),
				)),
				_ if err.raw_os_error() == Some(21) => prompting(&format!(
					"{} {}",
					args.value_of("FILE").unwrap_or("<FILE>").yellow(),
					remove_os_error(err.to_string()).to_lowercase(),
				)),
				_ => prompting(&err.to_string()),
			};
			if let Some(exit_code) = default_exit_code {
				process::exit(err.raw_os_error().unwrap_or(exit_code))
			}
		} else if let Some(err) = self.downcast_ref::<scdlang::Error>() {
			use scdlang::Error::*;
			match err {
				Deadlock => prompting(&err.to_string()),
				_ => print.prompt(&err.to_string(), "can't parse"),
			}
		} else {
			prompting(&self.to_string())
		}

		if let Some(exit_code) = default_exit_code {
			process::exit(exit_code)
		}
	}
}

impl Report for Error<'_> {
	fn report_and_exit(&self, default_exit_code: Option<i32>, _: Option<ArgMatches>) {
		let print = PRINTER("haskell").change(Mode::Error);
		match self {
			Error::StreamParse(err) => print.prompt(&err.to_string(), "can't parse"),
			_ => prompting(&self.to_string()),
		};
		if let Some(exit_code) = default_exit_code {
			process::exit(exit_code)
		}
	}
}

impl error::Error for Error<'_> {}
impl fmt::Display for Error<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use Error::*;
		match self {
			StreamParse(err) => write!(f, "{}", err),
			Count { count, topic } => write!(f, "Found {} error on {}", count, topic),
			WrongFormat {
				target,
				format,
				possible_formats,
			} => write!(
				f,
				"Argument {target} doesn't support {format}\nTry: {values}",
				target = format!("--format {}", target).red().bold(),
				format = format!("--as {}", format).red().bold(),
				values = format!("--format {} --as <{}>", target, possible_formats.join("|")).green(),
			),
		}
	}
}

// ============= 👇HELPER👇 =============

fn error(message: &str) -> String {
	if atty::is(Stream::Stderr) {
		format!("{} {}", prompt::ERROR.red().bold(), message.magenta())
	} else {
		format!("{} {}", prompt::ERROR, message)
	}
}

impl Prompt for PrettyPrint {
	fn print(&self, message: &str, title: &str) {
		self.string_with_header(message, &error(title)).unwrap()
	}
}

trait Prompt {
	fn prompt(&self, message: &str, title: &str) {
		if atty::is(Stream::Stderr) {
			self.print(message, title)
		} else {
			prompting_with_header(title, message)
		}
	}
	fn print(&self, message: &str, header: &str);
}

fn prompting_with_header(header: &str, message: &str) {
	eprintln!("{}\n---\n{}\n---", error(header), message)
}

fn prompting(message: &str) {
	eprintln!("{}", error(message))
}

fn remove_os_error(message: String) -> String {
	let no_oserr = message.replace("os error ", "");
	let arr_msg: Vec<&str> = no_oserr.split(' ').collect();
	arr_msg[..arr_msg.len() - 1].join(" ")
}
